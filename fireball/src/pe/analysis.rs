//! PE analysis utilities: entropy, anomaly detection, wide-string scanning,
//! export-forwarder resolution, and relocation-driven code pointer discovery.

use crate::{core::Section, prelude::*};
use std::collections::HashSet;

// PE section characteristic flags.
const IMAGE_SCN_MEM_READ: u32 = 0x4000_0000;
const IMAGE_SCN_MEM_WRITE: u32 = 0x8000_0000;
const IMAGE_SCN_MEM_EXECUTE: u32 = 0x2000_0000;

/// Minimum character count for a UTF-16LE string to be reported.
const MIN_WIDE_STRING_CHARS: usize = 4;

// ---------------------------------------------------------------------------
// 1. Shannon entropy per section
// ---------------------------------------------------------------------------

/// Compute the Shannon entropy of a byte slice.
///
/// Returns a value between 0.0 (all identical bytes) and 8.0 (maximally
/// random / packed / encrypted data).
pub fn shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut counts = [0u64; 256];
    for &b in data {
        counts[b as usize] += 1;
    }

    let len = data.len() as f64;
    let mut entropy = 0.0f64;
    for &count in &counts {
        if count == 0 {
            continue;
        }
        let p = count as f64 / len;
        entropy -= p * p.log2();
    }
    entropy
}

/// Result of per-section entropy analysis.
#[derive(Debug, Clone)]
pub struct SectionEntropy {
    pub name: String,
    pub entropy: f64,
    /// True when entropy exceeds 7.0 (likely packed or encrypted).
    pub likely_packed: bool,
}

/// Compute Shannon entropy for every section in the binary.
///
/// Each section's raw bytes are extracted from `binary` using the section's
/// `file_offset` and `size_of_file` fields.
pub fn section_entropies(binary: &[u8], sections: &[Section]) -> Vec<SectionEntropy> {
    sections
        .iter()
        .map(|sec| {
            let start = sec.file_offset as usize;
            let end = (start + sec.size_of_file as usize).min(binary.len());
            let slice = if start < binary.len() {
                &binary[start..end]
            } else {
                &[]
            };
            let entropy = shannon_entropy(slice);
            let likely_packed = entropy > 7.0;
            if likely_packed {
                warn!(
                    "Section '{}' has high entropy ({:.3}), likely packed/encrypted",
                    sec.name, entropy
                );
            }
            SectionEntropy {
                name: sec.name.clone(),
                entropy,
                likely_packed,
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// 2. RWX anomaly detection
// ---------------------------------------------------------------------------

/// A section that has simultaneous Read+Write+Execute permissions.
#[derive(Debug, Clone)]
pub struct RwxAnomaly {
    pub name: String,
    pub characteristics: u32,
    pub virtual_address: u64,
    pub virtual_size: u64,
}

/// Detect sections with simultaneous Read+Write+Execute characteristics.
///
/// Such sections are unusual in legitimate binaries and may indicate
/// self-modifying code, packing, or exploitation payloads.
pub fn detect_rwx_sections(sections: &[Section]) -> Vec<RwxAnomaly> {
    let rwx_mask = IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE | IMAGE_SCN_MEM_EXECUTE;

    sections
        .iter()
        .filter(|sec| (sec.characteristics & rwx_mask) == rwx_mask)
        .map(|sec| {
            warn!(
                "Section '{}' at VA {:#X} is RWX (characteristics {:#010X})",
                sec.name, sec.virtual_address, sec.characteristics
            );
            RwxAnomaly {
                name: sec.name.clone(),
                characteristics: sec.characteristics,
                virtual_address: sec.virtual_address,
                virtual_size: sec.virtual_size,
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// 3. Wide-string (UTF-16LE) identification
// ---------------------------------------------------------------------------

/// A decoded UTF-16LE string found at a given offset in a byte slice.
#[derive(Debug, Clone)]
pub struct WideString {
    /// Byte offset within the scanned slice where the string begins.
    pub offset: usize,
    /// The decoded string content.
    pub value: String,
}

/// Scan `data` for UTF-16LE encoded strings of at least
/// [`MIN_WIDE_STRING_CHARS`] printable characters.
///
/// Returns `(offset, decoded_string)` pairs for every match found.
pub fn scan_wide_strings(data: &[u8]) -> Vec<WideString> {
    let mut results = Vec::new();
    let mut i = 0;

    while i + 1 < data.len() {
        let start = i;
        let mut chars: Vec<u16> = Vec::new();

        // Greedily collect valid printable UTF-16LE code units.
        while i + 1 < data.len() {
            let code_unit = u16::from_le_bytes([data[i], data[i + 1]]);
            // Accept printable ASCII range (0x20..=0x7E) and common extended
            // characters, but reject surrogates and control chars.
            if (0x20..=0x7E).contains(&code_unit)
                || code_unit == 0x09 // tab
                || code_unit == 0x0A // LF
                || code_unit == 0x0D
            // CR
            {
                chars.push(code_unit);
                i += 2;
            } else {
                break;
            }
        }

        if chars.len() >= MIN_WIDE_STRING_CHARS {
            let value = String::from_utf16_lossy(&chars);
            results.push(WideString {
                offset: start,
                value,
            });
        }

        // Advance past the non-matching code unit (or by 1 byte to handle
        // misaligned strings).
        if i == start {
            i += 1;
        }
    }

    debug!("Wide-string scan found {} strings", results.len());
    results
}

// ---------------------------------------------------------------------------
// 4. Export-forwarder resolution
// ---------------------------------------------------------------------------

/// A forwarded PE export entry.
#[derive(Debug, Clone)]
pub struct ForwardedExport {
    /// The name of the export in this binary.
    pub name: String,
    /// The DLL the export forwards to.
    pub forward_dll: String,
    /// The symbol name (or ordinal string) in the target DLL.
    pub forward_name: String,
}

/// Identify forwarded exports from a goblin-parsed PE.
///
/// Forwarded exports are entries in the export table that reference a symbol
/// in another DLL rather than pointing to code within this binary.  We detect
/// them via the `reexport` field on `goblin::pe::export::Export` and extract
/// the forwarding target from its `Debug` representation.
pub fn resolve_forwarded_exports(pe: &goblin::pe::PE<'_>) -> Vec<ForwardedExport> {
    let mut results = Vec::new();

    for export in &pe.exports {
        if export.reexport.is_none() {
            continue;
        }
        let reexport = export.reexport.as_ref().unwrap();

        let export_name = export
            .name
            .map(|n| n.to_string())
            .unwrap_or_else(|| format!("rva_{:#x}", export.rva));

        // goblin's Reexport stores the forwarding string (e.g. "NTDLL.RtlAllocateHeap").
        // Its Debug output includes the DLL and target name. We parse the
        // "DLL.Symbol" pair from the debug representation.
        let debug_str = format!("{:?}", reexport);

        // Extract the forwarding target. The Debug output typically looks like:
        //   Reexport { dll: "NTDLL", name: "RtlAllocateHeap" }
        // or similar. We look for quoted strings as a robust extraction.
        let quoted_strings: Vec<&str> = debug_str
            .split('"')
            .enumerate()
            .filter_map(|(i, s)| if i % 2 == 1 { Some(s) } else { None })
            .collect();

        let (forward_dll, forward_name) = match quoted_strings.len() {
            0 => {
                // Fallback: try to parse as "DLL.Name" from any content
                ("unknown".to_string(), debug_str.clone())
            }
            1 => {
                // Single quoted string, might be "DLL.Name" format
                let s = quoted_strings[0];
                if let Some(dot_pos) = s.find('.') {
                    (s[..dot_pos].to_string(), s[dot_pos + 1..].to_string())
                } else {
                    ("unknown".to_string(), s.to_string())
                }
            }
            _ => {
                // Two or more quoted strings: first is DLL, second is name
                (quoted_strings[0].to_string(), quoted_strings[1].to_string())
            }
        };

        debug!(
            "Export '{}' forwards to {}!{}",
            export_name, forward_dll, forward_name
        );

        results.push(ForwardedExport {
            name: export_name,
            forward_dll,
            forward_name,
        });
    }

    if !results.is_empty() {
        info!("Found {} forwarded export(s) in PE", results.len());
    }

    results
}

// ---------------------------------------------------------------------------
// 5. Relocation-driven code pointer scan
// ---------------------------------------------------------------------------

/// A relocation entry that falls within an executable section.
#[derive(Debug, Clone)]
pub struct CodeRelocation {
    /// The virtual address of the relocation entry.
    pub address: u64,
    /// Name of the executable section containing this relocation.
    pub section_name: String,
}

/// Identify relocation entries that point into executable sections.
///
/// These are likely code pointers (vtable entries, function pointers, indirect
/// call targets, etc.) and are valuable for improving disassembly coverage.
pub fn find_code_relocations(
    relocation_addresses: &HashSet<u64>,
    sections: &[Section],
) -> Vec<CodeRelocation> {
    // Pre-filter to only executable sections for the inner loop.
    let exec_sections: Vec<&Section> = sections.iter().filter(|s| s.is_executable()).collect();

    let mut results: Vec<CodeRelocation> = relocation_addresses
        .iter()
        .filter_map(|&addr| {
            exec_sections.iter().find_map(|sec| {
                let sec_start = sec.virtual_address;
                let sec_end = sec_start + sec.virtual_size;
                if addr >= sec_start && addr < sec_end {
                    Some(CodeRelocation {
                        address: addr,
                        section_name: sec.name.clone(),
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    results.sort_by_key(|r| r.address);

    debug!(
        "Found {} relocation(s) within executable sections (out of {} total)",
        results.len(),
        relocation_addresses.len()
    );

    results
}

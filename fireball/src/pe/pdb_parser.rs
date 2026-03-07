//! PDB/CodeView symbol parsing for PE binaries.
//!
//! Attempts to locate and parse a PDB file referenced by the PE's debug
//! directory, then merges discovered symbols into the decompiler's
//! [`PreDefinedOffsets`] table.

use crate::{
    core::{Address, PreDefinedOffset, PreDefinedOffsets, Sections},
    prelude::*,
};
use pdb::FallibleIterator;
use std::path::Path;

/// A single symbol extracted from a PDB file.
pub struct PdbSymbol {
    /// Demangled (or raw) symbol name.
    pub name: String,
    /// Relative virtual address inside the image.
    pub rva: u32,
    /// Size of the symbol in bytes (0 when unknown).
    pub size: u32,
}

/// Collection of symbols loaded from a PDB file.
pub struct PdbInfo {
    pub symbols: Vec<PdbSymbol>,
}

// ---------------------------------------------------------------------------
// PDB path extraction from PE binary
// ---------------------------------------------------------------------------

/// Extracts the PDB file path embedded in the PE's CodeView PDB70 debug
/// directory entry.  Searches the raw binary for the `RSDS` signature and
/// reads the null-terminated UTF-8 path that follows the 24-byte header
/// (signature + GUID + age).
fn extract_pdb_path_from_binary(binary: &[u8]) -> Option<String> {
    // The CodeView PDB70 record layout:
    //   offset 0:  u32  signature  "RSDS" (0x53445352)
    //   offset 4:  [u8; 16] GUID
    //   offset 20: u32  age
    //   offset 24: null-terminated UTF-8 file name
    const RSDS: &[u8; 4] = b"RSDS";
    const HEADER_LEN: usize = 24;

    let pos = binary.windows(4).position(|window| window == RSDS)?;

    if pos + HEADER_LEN >= binary.len() {
        return None;
    }

    let name_start = pos + HEADER_LEN;
    let name_end = binary[name_start..]
        .iter()
        .position(|&b| b == 0)
        .map(|n| name_start + n)
        .unwrap_or(binary.len());

    let raw = &binary[name_start..name_end];
    String::from_utf8(raw.to_vec()).ok()
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Attempts to load PDB symbols for the given PE binary.
///
/// The function is entirely opportunistic: if the PDB file cannot be found or
/// parsed, it returns `None` without logging errors.
pub fn try_load_pdb(pe_path: &str, binary: &[u8]) -> Option<PdbInfo> {
    let pdb_name = extract_pdb_path_from_binary(binary)?;

    // Resolve the PDB path relative to the PE file's directory.
    let pe_dir = Path::new(pe_path).parent()?;
    let pdb_filename = Path::new(&pdb_name).file_name()?;
    let pdb_path = pe_dir.join(pdb_filename);

    if !pdb_path.exists() {
        return None;
    }

    let file = std::fs::File::open(&pdb_path).ok()?;
    let mut pdb = pdb::PDB::open(file).ok()?;

    let mut symbols = Vec::new();

    // -- Global symbols --
    collect_global_symbols(&mut pdb, &mut symbols);

    // -- Module (per-compiland) symbols --
    collect_module_symbols(&mut pdb, &mut symbols);

    debug!(
        "Loaded {} symbols from PDB '{}'",
        symbols.len(),
        pdb_path.display()
    );

    Some(PdbInfo { symbols })
}

/// Inserts PDB symbols into the [`PreDefinedOffsets`] table.
pub fn merge_pdb_symbols(
    pdb_info: &PdbInfo,
    defined: &PreDefinedOffsets,
    sections: &Sections,
    image_base: u64,
) {
    let mut count = 0u64;
    for sym in &pdb_info.symbols {
        if sym.rva == 0 {
            continue;
        }
        let va = image_base + sym.rva as u64;
        defined.insert(PreDefinedOffset {
            name: sym.name.clone(),
            address: Address::from_virtual_address(sections, va),
        });
        count += 1;
    }
    debug!("Merged {} PDB symbols into predefined offsets", count);
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn demangle_name(raw: &str) -> String {
    // Try C++ demangling first.
    if let Ok(sym) = cpp_demangle::Symbol::new(raw) {
        if let Ok(demangled) = sym.demangle() {
            return demangled;
        }
    }
    // Try Rust demangling as fallback.
    let demangled = rustc_demangle::demangle(raw).to_string();
    if demangled != raw {
        return demangled;
    }
    raw.to_string()
}

fn collect_global_symbols(pdb: &mut pdb::PDB<std::fs::File>, out: &mut Vec<PdbSymbol>) {
    let symbol_table = match pdb.global_symbols() {
        Ok(t) => t,
        Err(_) => return,
    };

    let address_map = match pdb.address_map() {
        Ok(m) => m,
        Err(_) => return,
    };

    let mut iter = symbol_table.iter();
    while let Ok(Some(symbol)) = iter.next() {
        let parsed: Result<pdb::SymbolData<'_>, _> = symbol.parse();
        if let Ok(pdb::SymbolData::Public(data)) = parsed {
            let rva = match data.offset.to_rva(&address_map) {
                Some(r) => r.0,
                None => continue,
            };
            out.push(PdbSymbol {
                name: demangle_name(&data.name.to_string()),
                rva,
                size: 0,
            });
        }
    }
}

fn collect_module_symbols(pdb: &mut pdb::PDB<std::fs::File>, out: &mut Vec<PdbSymbol>) {
    let dbi = match pdb.debug_information() {
        Ok(d) => d,
        Err(_) => return,
    };

    let address_map = match pdb.address_map() {
        Ok(m) => m,
        Err(_) => return,
    };

    let mut modules = match dbi.modules() {
        Ok(m) => m,
        Err(_) => return,
    };

    while let Ok(Some(module)) = modules.next() {
        let info = match pdb.module_info(&module) {
            Ok(Some(info)) => info,
            _ => continue,
        };

        let mut syms = match info.symbols() {
            Ok(s) => s,
            Err(_) => continue,
        };

        while let Ok(Some(symbol)) = syms.next() {
            let parsed: Result<pdb::SymbolData<'_>, _> = symbol.parse();
            match parsed {
                Ok(pdb::SymbolData::Procedure(proc)) => {
                    let rva = match proc.offset.to_rva(&address_map) {
                        Some(r) => r.0,
                        None => continue,
                    };
                    out.push(PdbSymbol {
                        name: demangle_name(&proc.name.to_string()),
                        rva,
                        size: proc.len,
                    });
                }
                Ok(pdb::SymbolData::Data(data)) => {
                    let rva = match data.offset.to_rva(&address_map) {
                        Some(r) => r.0,
                        None => continue,
                    };
                    out.push(PdbSymbol {
                        name: demangle_name(&data.name.to_string()),
                        rva,
                        size: 0,
                    });
                }
                _ => {}
            }
        }
    }
}

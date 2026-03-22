//! Module containing the implementation of the MachO struct

use super::MachO;
use crate::{
    arch,
    core::{Address, Blocks, PreDefinedOffset, PreDefinedOffsets, Relations, Sections},
    prelude::*,
};
use iceball::MachineArchitecture;
use std::sync::atomic::Ordering;

impl MachO {
    pub fn from_path(path: &str) -> Result<MachO, FireballError> {
        let binary = std::fs::read(path)?;
        MachO::new(Some(path.to_owned()), binary)
    }

    pub fn from_binary(binary: Vec<u8>) -> Result<MachO, FireballError> {
        MachO::new(None, binary)
    }

    /// Creates a MachO struct from binary data.
    pub(crate) fn new(path: Option<String>, binary: Vec<u8>) -> Result<Self, FireballError> {
        // Extract architecture and entry point from the Mach-O header.
        // We parse inside a block so the borrow of `binary` is released
        // before we move `binary` into the struct.
        let (cputype, entry_addr, symbols_info) = {
            let mach = goblin::mach::Mach::parse(&binary)
                .map_err(|e| FireballError::MachOParsingFailed(e.to_string()))?;

            match mach {
                goblin::mach::Mach::Binary(ref macho) => extract_macho_info(macho),
                goblin::mach::Mach::Fat(ref multi) => {
                    // For fat binaries, extract the first architecture's slice
                    // and recursively parse it.
                    let arches = multi.arches().map_err(|e| {
                        FireballError::MachOParsingFailed(format!("Failed to read fat arches: {e}"))
                    })?;
                    if let Some(arch_entry) = arches.first() {
                        let offset = arch_entry.offset as usize;
                        let size = arch_entry.size as usize;
                        if offset + size <= binary.len() {
                            let slice = binary[offset..offset + size].to_vec();
                            return MachO::new(path, slice);
                        }
                    }
                    return Err(FireballError::MachOParsingFailed(
                        "Fat binary contains no usable Mach-O architectures".to_string(),
                    ));
                }
            }
        };

        let architecture = arch::from_mach_cputype(cputype);

        // Build section information for the entire binary
        let sections = Sections::new();
        sections.build_all(&binary);

        // Create Capstone engine
        let capstone = arch::build_capstone(architecture)?;

        // Generate predefined binary offset information from symbol info
        let defined = {
            let defined = PreDefinedOffsets::new();

            for (name, addr) in &symbols_info {
                defined.insert(PreDefinedOffset {
                    name: name.clone(),
                    address: Address::from_virtual_address(&sections, *addr),
                });
            }

            defined
        };

        let relations = Relations::new();
        Ok(MachO {
            entry: Address::from_virtual_address(&sections, entry_addr),
            path,
            binary,
            architecture,
            capstone,
            defined,
            sections,
            relations: relations.clone(),
            blocks: Blocks::new(relations),
            cancel_token: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        })
    }

    pub fn entry(&self) -> &Address {
        &self.entry
    }

    pub(crate) fn architecture(&self) -> MachineArchitecture {
        self.architecture
    }

    pub fn cancel_analysis(&self) {
        self.cancel_token.store(true, Ordering::Relaxed);
    }

    pub fn reset_analysis_cancellation(&self) {
        self.cancel_token.store(false, Ordering::Relaxed);
    }
}

/// Extract cpu type, entry address, and symbol info from a parsed MachO.
/// Returns owned data so the borrow on the binary can be released.
fn extract_macho_info(macho: &goblin::mach::MachO) -> (u32, u64, Vec<(String, u64)>) {
    let cputype = macho.header.cputype as u32;
    let entry_addr = macho.entry;

    let mut symbols_info = Vec::new();
    if let Some(symbols) = macho.symbols.as_ref() {
        for symbol_result in symbols.iter() {
            let Ok((name, nlist)) = symbol_result else {
                continue;
            };
            if nlist.n_value == 0 || name.is_empty() {
                continue;
            }
            // Skip debug stab entries
            if nlist.n_type & 0xe0 != 0 {
                continue;
            }

            let demangled = demangle_symbol(name);
            symbols_info.push((demangled, nlist.n_value));
        }
    }

    (cputype, entry_addr, symbols_info)
}

/// Demangle a symbol name (C++ or Rust), returning the original if
/// demangling fails. Strips leading underscore common in Mach-O symbols.
fn demangle_symbol(name: &str) -> String {
    // Mach-O prepends an underscore to C symbols
    let stripped = name.strip_prefix('_').unwrap_or(name);

    if let Ok(sym) = cpp_demangle::Symbol::new(name) {
        if let Ok(demangled) = sym.demangle() {
            return demangled;
        }
    }
    if stripped != name {
        if let Ok(sym) = cpp_demangle::Symbol::new(stripped) {
            if let Ok(demangled) = sym.demangle() {
                return demangled;
            }
        }
    }

    let demangled = rustc_demangle::demangle(name);
    let demangled_str = demangled.to_string();
    if demangled_str != name {
        return demangled_str;
    }

    stripped.to_string()
}

//! Module containing the implementation of the PE struct

use super::{Pe, pdb_parser};
use crate::{
    core::{Address, Blocks, PreDefinedOffset, PreDefinedOffsets, Relations, Sections},
    prelude::*,
};
use capstone::prelude::BuildsCapstone;
use std::sync::atomic::Ordering;

impl Pe {
    pub fn from_path(path: &str) -> Result<Pe, FireballError> {
        let binary = std::fs::read(path)?;
        Pe::new(Some(path.to_owned()), binary)
    }

    pub fn from_binary(binary: Vec<u8>) -> Result<Pe, FireballError> {
        Pe::new(None, binary)
    }

    /// Creates a PE struct from binary data.
    pub(crate) fn new(path: Option<String>, binary: Vec<u8>) -> Result<Self, FireballError> {
        // 1. Build section information
        // 2. Create Capstone object
        // 3. Generate predefined binary offset information

        // Common objects used throughout
        let gl = goblin::pe::PE::parse(&binary)?;

        // Build section information for the entire binary
        let sections = Sections::new();
        sections.build_all(&binary);

        // Create Capstone object
        let capstone = {
            // Check if it's 86x64 based on the binary
            let is_86 = !gl.is_64;

            // Create Capstone object
            let capstone = capstone::Capstone::new()
                .x86()
                .mode(if is_86 {
                    capstone::arch::x86::ArchMode::Mode32
                } else {
                    capstone::arch::x86::ArchMode::Mode64
                })
                .build()?;

            Box::pin(capstone)
        };

        // Generate predefined binary offset information
        let defined = {
            let defined = PreDefinedOffsets::new();

            let imports = gl.imports;
            let exports = gl.exports;

            for import in imports {
                let name = format!("{}::{}", import.dll, import.name);
                let offset = import.offset as u64;

                defined.insert(PreDefinedOffset {
                    name,
                    address: Address::from_virtual_address(&sections, offset),
                });
            }

            for export in exports {
                let Some(offset_raw) = export.offset else {
                    warn!(
                        "Skipping malformed export without offset: {}",
                        export.name.unwrap_or("<unnamed>")
                    );
                    continue;
                };

                let name = if let Some(name) = export.name {
                    // Try C++ demangling for exported symbols
                    if let Ok(sym) = cpp_demangle::Symbol::new(name) {
                        sym.demangle().unwrap_or_else(|_| name.to_string())
                    } else {
                        // Try Rust demangling as fallback
                        let demangled = rustc_demangle::demangle(name);
                        let demangled_str = demangled.to_string();
                        if demangled_str != name {
                            demangled_str
                        } else {
                            name.to_string()
                        }
                    }
                } else {
                    format!("0x{:x}", offset_raw)
                };
                let offset = offset_raw as u64;

                defined.insert(PreDefinedOffset {
                    name,
                    address: Address::from_virtual_address(&sections, offset),
                });
            }

            // Load PDB symbols if a PDB file is available next to the PE.
            if let Some(ref pe_path) = path {
                if let Some(pdb_info) = pdb_parser::try_load_pdb(pe_path, &binary) {
                    let image_base = gl
                        .header
                        .optional_header
                        .map(|opt| opt.windows_fields.image_base)
                        .unwrap_or(0);
                    pdb_parser::merge_pdb_symbols(&pdb_info, &defined, &sections, image_base);
                }
            }

            defined
        };

        // L125: Parse base relocation table to identify pointer-holding addresses.
        let relocation_addresses = {
            let mut relocs = std::collections::HashSet::new();
            if let Some(ref debug_data) = gl.debug_data {
                // goblin doesn't expose relocations directly on the PE struct,
                // so we parse the base relocation directory manually.
                let _ = debug_data; // placeholder — actual reloc parsing below
            }
            // goblin >= 0.7 exposes relocations via header.optional_header
            if let Some(opt) = gl.header.optional_header {
                if let Some(reloc_table) = opt.data_directories.get_base_relocation_table() {
                    let reloc_rva = reloc_table.virtual_address as usize;
                    let reloc_size = reloc_table.size as usize;
                    if reloc_rva > 0 && reloc_size > 0 {
                        // Walk the base relocation blocks
                        let image_base = opt.windows_fields.image_base;
                        if let Some(offset) = goblin::pe::utils::find_offset(
                            reloc_rva,
                            &gl.sections,
                            0,
                            &Default::default(),
                        ) {
                            let mut pos = offset;
                            let end = offset + reloc_size;
                            while pos + 8 <= end && pos + 8 <= binary.len() {
                                let page_rva = u32::from_le_bytes([
                                    binary[pos],
                                    binary[pos + 1],
                                    binary[pos + 2],
                                    binary[pos + 3],
                                ]);
                                let block_size = u32::from_le_bytes([
                                    binary[pos + 4],
                                    binary[pos + 5],
                                    binary[pos + 6],
                                    binary[pos + 7],
                                ]) as usize;
                                if block_size < 8 || pos + block_size > end {
                                    break;
                                }
                                let entry_count = (block_size - 8) / 2;
                                for i in 0..entry_count {
                                    let entry_offset = pos + 8 + i * 2;
                                    if entry_offset + 2 > binary.len() {
                                        break;
                                    }
                                    let entry = u16::from_le_bytes([
                                        binary[entry_offset],
                                        binary[entry_offset + 1],
                                    ]);
                                    let reloc_type = entry >> 12;
                                    let reloc_off = (entry & 0x0FFF) as u32;
                                    // Type 3 = HIGHLOW (32-bit), Type 10 = DIR64 (64-bit)
                                    if reloc_type == 3 || reloc_type == 10 {
                                        let va = image_base + (page_rva + reloc_off) as u64;
                                        relocs.insert(va);
                                    }
                                }
                                pos += block_size;
                            }
                        }
                        debug!(
                            "Parsed {} relocation entries from base relocation table",
                            relocs.len()
                        );
                    }
                }
            }
            relocs
        };

        let relations = Relations::new();
        Ok(Pe {
            entry: Address::from_virtual_address(&sections, gl.entry as u64),
            path,
            binary,
            capstone,
            defined,
            sections,
            relations: relations.clone(),
            blocks: Blocks::new(relations),
            cancel_token: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            relocation_addresses: std::sync::Arc::new(relocation_addresses),
        })
    }

    pub fn entry(&self) -> &Address {
        &self.entry
    }

    pub fn cancel_analysis(&self) {
        self.cancel_token.store(true, Ordering::Relaxed);
    }

    pub fn reset_analysis_cancellation(&self) {
        self.cancel_token.store(false, Ordering::Relaxed);
    }
}

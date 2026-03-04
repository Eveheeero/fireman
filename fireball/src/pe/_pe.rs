//! Module containing the implementation of the PE struct

use super::Pe;
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
                        name.to_string()
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

            defined
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

//! Module containing the implementation of the PE struct

use super::Pe;
use crate::{
    core::{Address, Blocks, PreDefinedOffset, PreDefinedOffsets, Relations, Sections},
    prelude::IoError,
};
use capstone::prelude::BuildsCapstone;

impl Pe {
    pub fn from_path(path: &str) -> Result<Pe, IoError> {
        let binary = std::fs::read(path)?;
        Ok(Pe::new(Some(path.to_owned()), binary))
    }

    pub fn from_binary(binary: Vec<u8>) -> Result<Pe, IoError> {
        Ok(Pe::new(None, binary))
    }

    /// Creates a PE struct from binary data.
    pub(crate) fn new(path: Option<String>, binary: Vec<u8>) -> Self {
        // 1. Detect architecture
        // 2. Build section information
        // 3. Create Capstone object
        // 4. Generate predefined binary offset information

        // Detect architecture from binary
        let architecture =
            crate::arch::architecture::ArchitectureDetector::detect_from_bytes(&binary);

        // Common objects used throughout
        let gl = goblin::pe::PE::parse(&binary).unwrap();

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
                .build()
                .unwrap();

            Box::pin(capstone)
        };

        // Generate predefined binary offset information
        let defined = {
            let defined = PreDefinedOffsets::new();

            let imports = gl.imports;
            let exports = gl.exports;

            for import in imports {
                let name = import.name.to_string();
                let offset = import.offset as u64;

                defined.insert(PreDefinedOffset {
                    name,
                    address: Address::from_virtual_address(&sections, offset),
                });
            }

            for export in exports {
                let name = if let Some(name) = export.name {
                    name.to_string()
                } else {
                    format!("0x{:x}", export.offset.unwrap())
                };
                let offset = export.offset.unwrap() as u64;

                defined.insert(PreDefinedOffset {
                    name,
                    address: Address::from_virtual_address(&sections, offset),
                });
            }

            defined
        };

        let relations = Relations::new();
        Pe {
            entry: Address::from_virtual_address(&sections, gl.entry as u64),
            path,
            binary,
            capstone,
            defined,
            sections,
            relations: relations.clone(),
            blocks: Blocks::new(relations),
            architecture,
        }
    }

    pub fn entry(&self) -> &Address {
        &self.entry
    }
}

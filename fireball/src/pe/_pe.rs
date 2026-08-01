//! Module containing the implementation of the PE struct

use super::Pe;
use crate::{
    BinaryKind, arch,
    core::{Address, Blocks, PreDefinedOffset, PreDefinedOffsets, Relations, Sections},
    prelude::*,
};
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

        let architecture = arch::from_pe_machine(gl.header.coff_header.machine, gl.is_64);

        // IMAGE_FILE_DLL = 0x2000
        let kind = if gl.header.coff_header.characteristics & 0x2000 != 0 {
            BinaryKind::SharedLibrary
        } else {
            BinaryKind::Executable
        };

        // Build section information for the entire binary
        let sections = Sections::new();
        sections.build_all(&binary);

        // Create Capstone object
        let capstone = arch::build_capstone(architecture)?;

        let image_base = gl
            .header
            .optional_header
            .map(|opt| opt.windows_fields.image_base)
            .unwrap_or(0);

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

            defined
        };

        let relations = Relations::new();
        Ok(Pe {
            kind,
            entry: Address::from_virtual_address(&sections, gl.entry as u64),
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

    pub fn kind(&self) -> BinaryKind {
        self.kind
    }

    pub fn entry(&self) -> &Address {
        &self.entry
    }

    pub(crate) fn architecture(&self) -> iceball::MachineArchitecture {
        self.architecture
    }

    pub fn cancel_analysis(&self) {
        self.cancel_token.store(true, Ordering::Relaxed);
    }

    pub fn reset_analysis_cancellation(&self) {
        self.cancel_token.store(false, Ordering::Relaxed);
    }
}

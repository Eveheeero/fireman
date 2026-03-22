//! Module containing the implementation of the Elf struct

use super::Elf;
use crate::{
    BinaryKind, arch,
    core::{Address, Blocks, PreDefinedOffset, PreDefinedOffsets, Relations, Sections},
    prelude::*,
};
use iceball::MachineArchitecture;
use std::sync::atomic::Ordering;

impl Elf {
    pub fn from_path(path: &str) -> Result<Elf, FireballError> {
        let binary = std::fs::read(path)?;
        Elf::new(Some(path.to_owned()), binary)
    }

    pub fn from_binary(binary: Vec<u8>) -> Result<Elf, FireballError> {
        Elf::new(None, binary)
    }

    /// Creates an Elf struct from binary data.
    pub(crate) fn new(path: Option<String>, binary: Vec<u8>) -> Result<Self, FireballError> {
        let gl = goblin::elf::Elf::parse(&binary)
            .map_err(|e| FireballError::ElfParsingFailed(e.to_string()))?;

        let is_64 = gl.is_64;
        let architecture = arch::from_elf_machine(gl.header.e_machine, is_64);

        // goblin::elf::header: ET_EXEC=2, ET_DYN=3, ET_REL=1
        let kind = match gl.header.e_type {
            goblin::elf::header::ET_DYN => BinaryKind::SharedLibrary,
            goblin::elf::header::ET_REL => BinaryKind::ObjectFile,
            _ => BinaryKind::Executable,
        };

        // Build section information for the entire binary
        let sections = Sections::new();
        sections.build_all(&binary);

        // Create Capstone engine
        let capstone = arch::build_capstone(architecture)?;

        // Generate predefined binary offset information from symbol tables
        let defined = {
            let defined = PreDefinedOffsets::new();

            // Static symbol table
            for sym in gl.syms.iter() {
                if sym.st_value == 0 {
                    continue;
                }
                let name = if let Some(name) = gl.strtab.get_at(sym.st_name) {
                    if name.is_empty() {
                        continue;
                    }
                    demangle_symbol(name)
                } else {
                    continue;
                };

                defined.insert(PreDefinedOffset {
                    name,
                    address: Address::from_virtual_address(&sections, sym.st_value),
                });
            }

            // Dynamic symbol table
            for sym in gl.dynsyms.iter() {
                if sym.st_value == 0 {
                    continue;
                }
                let name = if let Some(name) = gl.dynstrtab.get_at(sym.st_name) {
                    if name.is_empty() {
                        continue;
                    }
                    demangle_symbol(name)
                } else {
                    continue;
                };

                defined.insert(PreDefinedOffset {
                    name,
                    address: Address::from_virtual_address(&sections, sym.st_value),
                });
            }

            // Load DWARF symbols if debug sections are present
            if let Some(dwarf_info) = crate::pe::dwarf_parser::try_load_dwarf(&binary) {
                // ELF addresses in DWARF are already virtual addresses (no image base)
                crate::pe::dwarf_parser::merge_dwarf_symbols(&dwarf_info, &defined, &sections, 0);
            }

            defined
        };

        let relations = Relations::new();
        Ok(Elf {
            kind,
            entry: Address::from_virtual_address(&sections, gl.entry),
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

/// Demangle a symbol name (C++ or Rust), returning the original if
/// demangling fails.
fn demangle_symbol(name: &str) -> String {
    if let Ok(sym) = cpp_demangle::Symbol::new(name) {
        if let Ok(demangled) = sym.demangle() {
            return demangled;
        }
    }
    let demangled = rustc_demangle::demangle(name);
    let demangled_str = demangled.to_string();
    if demangled_str != name {
        demangled_str
    } else {
        name.to_string()
    }
}

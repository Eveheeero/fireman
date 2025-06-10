//! ELF (Executable and Linkable Format) support for Linux/Unix binaries
//!
//! This module implements parsing and analysis of ELF files commonly used on
//! Linux, BSD, and other Unix-like operating systems.

use crate::core::{Section, Sections};
use crate::utils::error::DecompileError;
use std::collections::BTreeMap;
use std::path::Path;

pub mod fire;
pub mod header;
pub mod parser;
pub mod section;
pub mod symbol;

/// ELF file representation
#[derive(Debug)]
pub struct Elf {
    /// File header
    pub header: header::ElfHeader,

    /// Program headers (segments)
    pub program_headers: Vec<header::ProgramHeader>,

    /// Section headers
    pub section_headers: Vec<section::SectionHeader>,

    /// Symbol tables
    pub symbols: symbol::SymbolTables,

    /// String tables
    pub string_tables: BTreeMap<usize, Vec<u8>>,

    /// Raw file data
    pub data: Vec<u8>,
}

impl Elf {
    /// Load an ELF file from disk
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, DecompileError> {
        let data = std::fs::read(path)?;
        Self::from_bytes(data)
    }

    /// Parse ELF from raw bytes
    pub fn from_bytes(data: Vec<u8>) -> Result<Self, DecompileError> {
        parser::parse_elf(data)
    }

    /// Create an ELF from binary data (for compatibility with Fireball interface)
    pub fn from_binary(data: Vec<u8>) -> Result<Self, crate::prelude::FireballError> {
        Self::from_bytes(data).map_err(|_e| crate::prelude::FireballError::Unknown)
    }

    /// Get the entry point address
    pub fn entry_point(&self) -> u64 {
        self.header.entry
    }

    /// Get the target architecture
    pub fn architecture(&self) -> Architecture {
        match self.header.machine {
            0x03 => Architecture::X86,
            0x3E => Architecture::X86_64,
            0xB7 => Architecture::Arm64,
            0x28 => Architecture::Arm32,
            _ => Architecture::Unknown,
        }
    }

    /// Check if this is a 64-bit ELF
    pub fn is_64bit(&self) -> bool {
        self.header.class == header::ElfClass::Elf64
    }

    /// Convert to generic Sections format
    pub fn to_sections(&self) -> Result<Sections, DecompileError> {
        let sections = Sections::default();

        // Convert each loaded segment to a Section
        for phdr in &self.program_headers {
            if phdr.p_type == header::PT_LOAD {
                let section = Section {
                    id: 0, // Will be set by add_section
                    name: format!("segment_{:x}", phdr.p_vaddr),
                    real_name: None,
                    virtual_address: phdr.p_vaddr,
                    virtual_size: phdr.p_memsz,
                    file_offset: phdr.p_offset as u64,
                    size_of_file: phdr.p_filesz,
                };

                sections.add_section(section);
            }
        }

        Ok(sections)
    }

    /// Convert segment flags to section characteristics
    fn segment_flags_to_characteristics(&self, flags: u32) -> u32 {
        let mut characteristics = 0;

        if flags & header::PF_R != 0 {
            characteristics |= 0x40000000; // IMAGE_SCN_MEM_READ
        }
        if flags & header::PF_W != 0 {
            characteristics |= 0x80000000; // IMAGE_SCN_MEM_WRITE
        }
        if flags & header::PF_X != 0 {
            characteristics |= 0x20000000; // IMAGE_SCN_MEM_EXECUTE
        }

        characteristics
    }

    /// Get imported functions
    pub fn imports(&self) -> Vec<Import> {
        self.symbols.get_imports()
    }

    /// Get exported functions
    pub fn exports(&self) -> Vec<Export> {
        self.symbols.get_exports()
    }
}

/// Architecture types found in ELF files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86,
    X86_64,
    Arm32,
    Arm64,
    Mips,
    PowerPC,
    Risc,
    Unknown,
}

/// Imported function information
#[derive(Debug, Clone)]
pub struct Import {
    pub name: String,
    pub library: Option<String>,
    pub address: u64,
}

/// Exported function information
#[derive(Debug, Clone)]
pub struct Export {
    pub name: String,
    pub address: u64,
    pub size: Option<u64>,
}

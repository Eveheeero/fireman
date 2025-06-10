//! Mach-O (Mach Object) format support for macOS/iOS binaries
//!
//! This module implements parsing and analysis of Mach-O files used on
//! Apple platforms including macOS, iOS, tvOS, and watchOS.

use crate::core::{Section, Sections};
use crate::utils::error::DecompileError;
use std::path::Path;

pub mod header;
pub mod parser;
pub mod segment;
pub mod symbol;

/// Mach-O file representation
#[derive(Debug)]
pub struct MachO {
    /// File header
    pub header: header::MachHeader,

    /// Load commands
    pub load_commands: Vec<LoadCommand>,

    /// Segments
    pub segments: Vec<segment::Segment>,

    /// Symbol table
    pub symbols: symbol::SymbolTable,

    /// String table
    pub string_table: Vec<u8>,

    /// Raw file data
    pub data: Vec<u8>,
}

impl MachO {
    /// Load a Mach-O file from disk
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, DecompileError> {
        let data = std::fs::read(path)?;
        Self::from_bytes(data)
    }

    /// Parse Mach-O from raw bytes
    pub fn from_bytes(data: Vec<u8>) -> Result<Self, DecompileError> {
        parser::parse_macho(data)
    }

    /// Get the entry point address
    pub fn entry_point(&self) -> Option<u64> {
        // In Mach-O, entry point is specified in LC_MAIN or LC_UNIXTHREAD
        for cmd in &self.load_commands {
            match cmd {
                LoadCommand::Main { entry_offset, .. } => {
                    return Some(*entry_offset);
                }
                LoadCommand::UnixThread { entry_point, .. } => {
                    return Some(*entry_point);
                }
                _ => {}
            }
        }
        None
    }

    /// Get the target architecture
    pub fn architecture(&self) -> Architecture {
        match (self.header.cpu_type, self.header.cpu_subtype) {
            (0x7, _) => Architecture::X86,
            (0x1000007, _) => Architecture::X86_64,
            (0xC, _) => Architecture::Arm32,
            (0x100000C, _) => Architecture::Arm64,
            _ => Architecture::Unknown,
        }
    }

    /// Check if this is a 64-bit Mach-O
    pub fn is_64bit(&self) -> bool {
        self.header.magic == header::MH_MAGIC_64 || self.header.magic == header::MH_CIGAM_64
    }

    /// Convert to generic Sections format
    pub fn to_sections(&self) -> Result<Sections, DecompileError> {
        let sections = Sections::default();

        // Convert each segment's sections
        for segment in &self.segments {
            for section in &segment.sections {
                let generic_section = Section {
                    id: 0, // Will be set by add_section
                    name: section.name.clone(),
                    real_name: Some(section.segname.clone()),
                    virtual_address: section.addr,
                    virtual_size: section.size,
                    file_offset: section.offset as u64,
                    size_of_file: section.size,
                };

                sections.add_section(generic_section);
            }
        }

        Ok(sections)
    }

    /// Convert section flags to PE-style characteristics
    fn section_flags_to_characteristics(&self, flags: u32) -> u32 {
        let mut characteristics = 0;

        // Always readable in Mach-O
        characteristics |= 0x40000000; // IMAGE_SCN_MEM_READ

        if flags & segment::S_ATTR_SOME_INSTRUCTIONS != 0 {
            characteristics |= 0x20000000; // IMAGE_SCN_MEM_EXECUTE
        }

        // Check segment permissions for write access
        // This would need to check the parent segment's maxprot field

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

/// Load command types
#[derive(Debug, Clone)]
pub enum LoadCommand {
    Segment {
        name: String,
        vmaddr: u64,
        vmsize: u64,
        fileoff: u64,
        filesize: u64,
    },
    SymbolTable {
        symoff: u32,
        nsyms: u32,
        stroff: u32,
        strsize: u32,
    },
    DynamicSymbolTable {
        // Dynamic symbol table info
    },
    Main {
        entry_offset: u64,
        stack_size: u64,
    },
    UnixThread {
        entry_point: u64,
    },
    LoadDylib {
        name: String,
        timestamp: u32,
        current_version: u32,
        compatibility_version: u32,
    },
}

/// Architecture types found in Mach-O files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86,
    X86_64,
    Arm32,
    Arm64,
    PowerPC,
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
    pub flags: u32,
    pub size: Option<u64>,
}

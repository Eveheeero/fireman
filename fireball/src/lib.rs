#![allow(dead_code)]

pub mod abstract_syntax_tree;
pub mod arch;
pub mod core;
pub mod elf;
pub mod ir;
pub mod pe;
pub mod prelude;
#[cfg(test)]
pub mod tests;
pub mod utils;

pub use crate::{abstract_syntax_tree::pattern_matching, core::Fire};
use crate::{core::FireRaw, prelude::*};

/// Enum storing parsers for all supported binary formats
#[derive(Debug)]
pub enum Fireball {
    /// Parser for PE files
    Pe(pe::Pe),
    /// Parser for ELF files
    Elf(elf::Elf),
}

impl Fireball {
    /// Creates a decompiler object from a file path.
    /// Detects the binary format automatically.
    pub fn from_path(path: &str) -> Result<Self, FireballError> {
        info!("Initializing decompiler object with file path {}", path);
        let binary = std::fs::read(path)?;
        Self::from_path_and_binary(Some(path.to_owned()), binary)
    }

    /// Creates a decompiler object from raw binary data.
    /// Detects the binary format automatically.
    pub fn from_binary(binary: Vec<u8>) -> Result<Self, FireballError> {
        Self::from_path_and_binary(None, binary)
    }

    fn from_path_and_binary(path: Option<String>, binary: Vec<u8>) -> Result<Self, FireballError> {
        // Detect format from magic bytes, then delegate to format-specific
        // constructors. The binary is parsed once here for detection and
        // once more inside Pe::new / Elf::new for full header extraction.
        // The path variant uses `new()` directly to avoid a redundant
        // file read.
        match goblin::Object::parse(&binary)? {
            goblin::Object::PE(_) => Ok(Fireball::Pe(pe::Pe::new(path, binary)?)),
            goblin::Object::Elf(_) => Ok(Fireball::Elf(elf::Elf::new(path, binary)?)),
            _ => Err(FireballError::UnsupportedFormat),
        }
    }

    pub fn cancel_analysis(&self) {
        match self {
            Self::Pe(pe) => pe.cancel_analysis(),
            Self::Elf(elf) => elf.cancel_analysis(),
        }
    }

    pub fn reset_analysis_cancellation(&self) {
        match self {
            Self::Pe(pe) => pe.reset_analysis_cancellation(),
            Self::Elf(elf) => elf.reset_analysis_cancellation(),
        }
    }
}

impl Fire for Fireball {
    fn get_path(&self) -> Option<String> {
        match self {
            Self::Pe(pe) => pe.get_path(),
            Self::Elf(elf) => elf.get_path(),
        }
    }

    fn get_binary(&self) -> &Vec<u8> {
        match self {
            Self::Pe(pe) => pe.get_binary(),
            Self::Elf(elf) => elf.get_binary(),
        }
    }

    fn decompile_all(&self) -> Result<String, prelude::DecompileError> {
        match self {
            Self::Pe(pe) => pe.decompile_all(),
            Self::Elf(elf) => elf.decompile_all(),
        }
    }

    fn decompile_from_entry(&self) -> Result<String, prelude::DecompileError> {
        match self {
            Self::Pe(pe) => pe.decompile_from_entry(),
            Self::Elf(elf) => elf.decompile_from_entry(),
        }
    }

    fn decompile_from_file_offset(&self, address: u64) -> Result<String, prelude::DecompileError> {
        match self {
            Self::Pe(pe) => pe.decompile_from_file_offset(address),
            Self::Elf(elf) => elf.decompile_from_file_offset(address),
        }
    }

    fn decompile_from_virtual_address(
        &self,
        address: u64,
    ) -> Result<String, prelude::DecompileError> {
        match self {
            Self::Pe(pe) => pe.decompile_from_virtual_address(address),
            Self::Elf(elf) => elf.decompile_from_virtual_address(address),
        }
    }
}

impl FireRaw for Fireball {
    fn analyze_all(&self) -> Result<Vec<std::sync::Arc<core::Block>>, prelude::DecompileError> {
        match self {
            Self::Pe(pe) => pe.analyze_all(),
            Self::Elf(elf) => elf.analyze_all(),
        }
    }

    fn analyze_from_entry(&self) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        match self {
            Self::Pe(pe) => pe.analyze_from_entry(),
            Self::Elf(elf) => elf.analyze_from_entry(),
        }
    }

    fn analyze_from_file_offset(
        &self,
        address: u64,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        match self {
            Self::Pe(pe) => pe.analyze_from_file_offset(address),
            Self::Elf(elf) => elf.analyze_from_file_offset(address),
        }
    }

    fn analyze_from_virtual_address(
        &self,
        address: u64,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        match self {
            Self::Pe(pe) => pe.analyze_from_virtual_address(address),
            Self::Elf(elf) => elf.analyze_from_virtual_address(address),
        }
    }

    fn analyze_block(
        &self,
        address: &core::Address,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        match self {
            Self::Pe(pe) => pe.analyze_block(address),
            Self::Elf(elf) => elf.analyze_block(address),
        }
    }

    fn get_sections(&self) -> std::sync::Arc<core::Sections> {
        match self {
            Self::Pe(pe) => pe.get_sections(),
            Self::Elf(elf) => elf.get_sections(),
        }
    }

    fn get_defined(&self) -> std::sync::Arc<core::PreDefinedOffsets> {
        match self {
            Self::Pe(pe) => pe.get_defined(),
            Self::Elf(elf) => elf.get_defined(),
        }
    }

    fn get_blocks(&self) -> std::sync::Arc<core::Blocks> {
        match self {
            Self::Pe(pe) => pe.get_blocks(),
            Self::Elf(elf) => elf.get_blocks(),
        }
    }

    fn get_relations(&self) -> std::sync::Arc<core::Relations> {
        match self {
            Self::Pe(pe) => pe.get_relations(),
            Self::Elf(elf) => elf.get_relations(),
        }
    }
}

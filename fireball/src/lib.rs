#![allow(dead_code)]

pub mod abstract_syntax_tree;
pub mod arch;
pub mod core;
pub mod elf;
pub mod ir;
pub mod macho;
pub mod pe;
pub mod prelude;
#[cfg(test)]
pub mod tests;
pub mod utils;

pub use crate::{
    abstract_syntax_tree::pattern_matching, core::Fire,
    utils::error::decompile_error::DecompileError,
};
use crate::{core::FireRaw, prelude::*};

/// Classification of the loaded binary file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryKind {
    /// Standard executable (PE exe, ELF ET_EXEC, Mach-O MH_EXECUTE)
    Executable,
    /// Shared library (DLL, .so ET_DYN, .dylib MH_DYLIB)
    SharedLibrary,
    /// Relocatable object file (.o ET_REL, .obj COFF)
    ObjectFile,
}

/// Enum storing parsers for all supported binary formats
#[derive(Debug)]
pub enum Fireball {
    /// Parser for PE files
    Pe(pe::Pe),
    /// Parser for ELF files
    Elf(elf::Elf),
    /// Parser for Mach-O files
    MachO(macho::MachO),
}

/// Dispatches a method call to the inner format-specific parser.
macro_rules! dispatch {
    ($self:expr, $method:ident $(, $arg:expr)*) => {
        match $self {
            Fireball::Pe(inner) => inner.$method($($arg),*),
            Fireball::Elf(inner) => inner.$method($($arg),*),
            Fireball::MachO(inner) => inner.$method($($arg),*),
        }
    };
}

impl Fireball {
    /// Returns the classification of the loaded binary (executable, shared
    /// library, or object file).
    pub fn kind(&self) -> BinaryKind {
        dispatch!(self, kind)
    }

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
        // once more inside the format-specific constructor for full header
        // extraction. The path variant uses `new()` directly to avoid a
        // redundant file read.
        match goblin::Object::parse(&binary)? {
            goblin::Object::PE(_) => Ok(Fireball::Pe(pe::Pe::new(path, binary)?)),
            goblin::Object::Elf(_) => Ok(Fireball::Elf(elf::Elf::new(path, binary)?)),
            goblin::Object::Mach(_) => Ok(Fireball::MachO(macho::MachO::new(path, binary)?)),
            _ => Err(FireballError::UnsupportedFormat),
        }
    }

    pub fn cancel_analysis(&self) {
        dispatch!(self, cancel_analysis);
    }

    pub fn reset_analysis_cancellation(&self) {
        dispatch!(self, reset_analysis_cancellation);
    }
}

impl Fire for Fireball {
    fn get_path(&self) -> Option<String> {
        dispatch!(self, get_path)
    }

    fn get_binary(&self) -> &Vec<u8> {
        dispatch!(self, get_binary)
    }

    fn decompile_all(&self) -> Result<String, prelude::DecompileError> {
        dispatch!(self, decompile_all)
    }

    fn decompile_from_entry(&self) -> Result<String, prelude::DecompileError> {
        dispatch!(self, decompile_from_entry)
    }

    fn decompile_from_file_offset(&self, address: u64) -> Result<String, prelude::DecompileError> {
        dispatch!(self, decompile_from_file_offset, address)
    }

    fn decompile_from_virtual_address(
        &self,
        address: u64,
    ) -> Result<String, prelude::DecompileError> {
        dispatch!(self, decompile_from_virtual_address, address)
    }
}

impl FireRaw for Fireball {
    fn analyze_all(&self) -> Result<Vec<std::sync::Arc<core::Block>>, prelude::DecompileError> {
        dispatch!(self, analyze_all)
    }

    fn analyze_from_entry(&self) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        dispatch!(self, analyze_from_entry)
    }

    fn analyze_from_file_offset(
        &self,
        address: u64,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        dispatch!(self, analyze_from_file_offset, address)
    }

    fn analyze_from_virtual_address(
        &self,
        address: u64,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        dispatch!(self, analyze_from_virtual_address, address)
    }

    fn analyze_block(
        &self,
        address: &core::Address,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        dispatch!(self, analyze_block, address)
    }

    fn get_sections(&self) -> std::sync::Arc<core::Sections> {
        dispatch!(self, get_sections)
    }

    fn get_defined(&self) -> std::sync::Arc<core::PreDefinedOffsets> {
        dispatch!(self, get_defined)
    }

    fn get_blocks(&self) -> std::sync::Arc<core::Blocks> {
        dispatch!(self, get_blocks)
    }

    fn get_relations(&self) -> std::sync::Arc<core::Relations> {
        dispatch!(self, get_relations)
    }
}

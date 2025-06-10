#![allow(dead_code)]

pub mod arch;
mod binary;
pub mod core;
pub mod elf;
pub mod ir;
pub mod macho;
pub mod pe;
pub mod prelude;
pub mod simulation;
#[cfg(test)]
pub mod tests;
pub mod utils;

pub use crate::core::Fire;
use crate::{core::FireRaw, prelude::*};

/// Enum storing parsers for all supported types
#[derive(Debug)]
pub enum Fireball {
    /// Parser for PE files (Windows)
    Pe(pe::Pe),
    /// Parser for ELF files (Linux/Unix)
    Elf(elf::Elf),
    /// Parser for Mach-O files (macOS/iOS)
    MachO(macho::MachO),
}

impl Fireball {
    /// Creates a decompiler object from a file path.
    pub fn from_path(path: &str) -> Result<Self, FireballError> {
        info!("Initializing decompiler object with file path {}", path);
        let binary = std::fs::read(path)?;
        Self::from_binary(binary)
    }

    /// Creates a decompiler object from binary data, auto-detecting format
    pub fn from_binary(binary: Vec<u8>) -> Result<Self, FireballError> {
        // Detect binary format from magic bytes
        if binary.len() < 4 {
            return Err(FireballError::InvalidBinary("File too small".to_string()));
        }

        match &binary[0..4] {
            // PE format: MZ header
            [0x4D, 0x5A, _, _] => Ok(Fireball::Pe(pe::Pe::from_binary(binary)?)),

            // ELF format: 0x7F ELF
            [0x7F, 0x45, 0x4C, 0x46] => {
                // TODO: Implement ELF parser
                Err(FireballError::Unimplemented("ELF format not yet implemented".to_string()))
            }

            // Mach-O format (little-endian)
            [0xFE, 0xED, 0xFA, 0xCE] | // 32-bit
            [0xFE, 0xED, 0xFA, 0xCF] | // 64-bit
            [0xCE, 0xFA, 0xED, 0xFE] | // 32-bit big-endian
            [0xCF, 0xFA, 0xED, 0xFE] => { // 64-bit big-endian
                // TODO: Implement Mach-O parser
                Err(FireballError::Unimplemented("Mach-O format not yet implemented".to_string()))
            }

            _ => Err(FireballError::InvalidBinary("Unknown binary format".to_string())),
        }
    }

    /// Returns the decompiler object.
    pub fn get_object(&self) -> &dyn FireRaw {
        match self {
            Self::Pe(pe) => pe,
            Self::Elf(elf) => panic!("ELF FireRaw not implemented"),
            Self::MachO(macho) => panic!("MachO FireRaw not implemented"),
        }
    }
}

impl Fire for Fireball {
    fn get_path(&self) -> Option<String> {
        self.get_object().get_path()
    }

    fn get_binary(&self) -> &Vec<u8> {
        self.get_object().get_binary()
    }

    fn decompile_all(&self) -> Result<String, prelude::DecompileError> {
        self.get_object().decompile_all()
    }

    fn decompile_from_entry(&self) -> Result<String, prelude::DecompileError> {
        self.get_object().decompile_from_entry()
    }

    fn decompile_from_file_offset(&self, address: u64) -> Result<String, prelude::DecompileError> {
        self.get_object().decompile_from_file_offset(address)
    }

    fn decompile_from_virtual_address(
        &self,
        address: u64,
    ) -> Result<String, prelude::DecompileError> {
        self.get_object().decompile_from_virtual_address(address)
    }
}
impl FireRaw for Fireball {
    fn analyze_all(&self) -> Result<Vec<std::sync::Arc<core::Block>>, prelude::DecompileError> {
        self.get_object().analyze_all()
    }

    fn analyze_from_entry(&self) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        self.get_object().analyze_from_entry()
    }

    fn analyze_from_file_offset(
        &self,
        address: u64,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        self.get_object().analyze_from_file_offset(address)
    }

    fn analyze_from_virtual_address(
        &self,
        address: u64,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        self.get_object().analyze_from_virtual_address(address)
    }

    fn analyze_block(
        &self,
        address: &core::Address,
    ) -> Result<std::sync::Arc<core::Block>, prelude::DecompileError> {
        self.get_object().analyze_block(address)
    }

    fn get_sections(&self) -> std::sync::Arc<core::Sections> {
        self.get_object().get_sections()
    }

    fn get_defined(&self) -> std::sync::Arc<core::PreDefinedOffsets> {
        self.get_object().get_defined()
    }

    fn get_blocks(&self) -> std::sync::Arc<core::Blocks> {
        self.get_object().get_blocks()
    }

    fn get_relations(&self) -> std::sync::Arc<core::Relations> {
        self.get_object().get_relations()
    }
}

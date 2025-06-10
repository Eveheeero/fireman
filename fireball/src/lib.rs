#![allow(dead_code)]

pub mod arch;
pub mod core;
pub mod ir;
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
    /// Parser for PE files
    Pe(pe::Pe),
}

impl Fireball {
    /// Creates a decompiler object from a file path.
    pub fn from_path(path: &str) -> Result<Self, FireballError> {
        info!("Initializing decompiler object with file path {}", path);
        Ok(Fireball::Pe(pe::Pe::from_path(path)?))
    }
    pub fn from_binary(binary: Vec<u8>) -> Result<Self, FireballError> {
        Ok(Fireball::Pe(pe::Pe::from_binary(binary)?))
    }

    /// Returns the decompiler object.
    pub fn get_object(&self) -> &impl FireRaw {
        match self {
            Self::Pe(pe) => pe,
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

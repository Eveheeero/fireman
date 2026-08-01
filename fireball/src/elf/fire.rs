//! Modules that implement the `Fire` trait for the `Elf` struct.

mod analyze_all;
mod analyze_block;
mod analyze_from_entry;
mod analyze_from_file_offset;
mod analyze_from_virtual_address;

use super::Elf;
use crate::{
    core::{Address, Block, Blocks, Fire, FireRaw, PreDefinedOffsets, Relations, Sections},
    prelude::DecompileError,
};
use std::sync::Arc;

impl Fire for Elf {
    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_binary(&self) -> &Vec<u8> {
        &self.binary
    }

    fn decompile_all(&self) -> Result<String, DecompileError> {
        let blocks = self.analyze_all()?;
        Ok(
            crate::ir::analyze::generate_ast_with_pre_defined_symbols(blocks, self.get_defined())?
                .optimize(None)?
                .print(None),
        )
    }

    fn decompile_from_entry(&self) -> Result<String, DecompileError> {
        let block = self.analyze_from_entry()?;
        Ok(
            crate::ir::analyze::generate_ast_with_pre_defined_symbols([block], self.get_defined())?
                .optimize(None)?
                .print(None),
        )
    }

    fn decompile_from_file_offset(&self, address: u64) -> Result<String, DecompileError> {
        let block = self.analyze_from_file_offset(address)?;
        Ok(
            crate::ir::analyze::generate_ast_with_pre_defined_symbols([block], self.get_defined())?
                .optimize(None)?
                .print(None),
        )
    }

    fn decompile_from_virtual_address(&self, address: u64) -> Result<String, DecompileError> {
        let block = self.analyze_from_virtual_address(address)?;
        Ok(
            crate::ir::analyze::generate_ast_with_pre_defined_symbols([block], self.get_defined())?
                .optimize(None)?
                .print(None),
        )
    }
}

impl FireRaw for Elf {
    fn analyze_all(&self) -> Result<Vec<Arc<Block>>, DecompileError> {
        self._analyze_all()
    }

    fn analyze_from_entry(&self) -> Result<Arc<Block>, DecompileError> {
        self._analyze_from_entry()
    }

    fn analyze_from_file_offset(&self, address: u64) -> Result<Arc<Block>, DecompileError> {
        self._analyze_from_file_offset(address)
    }

    fn analyze_from_virtual_address(&self, address: u64) -> Result<Arc<Block>, DecompileError> {
        self._analyze_from_virtual_address(address)
    }

    fn analyze_block(&self, address: &Address) -> Result<Arc<Block>, DecompileError> {
        self._analyze_block(address)
    }

    fn get_sections(&self) -> Arc<Sections> {
        self.sections.clone()
    }

    fn get_defined(&self) -> Arc<PreDefinedOffsets> {
        self.defined.clone()
    }

    fn get_blocks(&self) -> Arc<Blocks> {
        self.blocks.clone()
    }

    fn get_relations(&self) -> Arc<Relations> {
        self.relations.clone()
    }
}

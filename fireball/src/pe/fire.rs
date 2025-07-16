//! Modules that implement the `Fire` trait for the `Pe` struct.

mod analyze_all;
mod analyze_block;
mod analyze_from_entry;
mod analyze_from_file_offset;
mod analyze_from_virtual_address;

use super::Pe;
use crate::{
    core::{Address, Block, Blocks, Fire, FireRaw, PreDefinedOffsets, Relations, Sections},
    prelude::DecompileError,
};
use std::sync::Arc;

impl Fire for Pe {
    fn get_path(&self) -> Option<String> {
        self.path.clone()
    }

    fn get_binary(&self) -> &Vec<u8> {
        &self.binary
    }

    fn decompile_all(&self) -> Result<String, DecompileError> {
        Ok(crate::ir::analyze::generate_ast(self.analyze_all()?)?.to_c_code(None))
    }

    fn decompile_from_entry(&self) -> Result<String, DecompileError> {
        Ok(crate::ir::analyze::generate_ast([self.analyze_from_entry()?])?.to_c_code(None))
    }

    fn decompile_from_file_offset(&self, address: u64) -> Result<String, DecompileError> {
        Ok(
            crate::ir::analyze::generate_ast([self.analyze_from_file_offset(address)?])?
                .to_c_code(None),
        )
    }

    fn decompile_from_virtual_address(&self, address: u64) -> Result<String, DecompileError> {
        Ok(
            crate::ir::analyze::generate_ast([self.analyze_from_virtual_address(address)?])?
                .to_c_code(None),
        )
    }
}
impl FireRaw for Pe {
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

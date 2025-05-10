//! 코어 트레이트에 대한 구현이 담겨있는 모듈

mod decom_all;
mod decom_block;
mod decom_from_entry;
mod decom_from_file_offset;
mod decom_from_virtual_address;

use super::Pe;
use crate::{
    core::{Address, Block, Blocks, Fire, PreDefinedOffsets, Relations, Sections},
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

    fn decom_all(&self) -> Result<Vec<Arc<Block>>, DecompileError> {
        self._decom_all()
    }

    fn decom_from_entry(&self) -> Result<Arc<Block>, DecompileError> {
        self._decom_from_entry()
    }

    fn decom_from_file_offset(&self, address: u64) -> Result<Arc<Block>, DecompileError> {
        self._decom_from_file_offset(address)
    }

    fn decom_from_virtual_address(&self, address: u64) -> Result<Arc<Block>, DecompileError> {
        self._decom_from_virtual_address(address)
    }

    fn decom_block(&self, address: &Address) -> Result<Arc<Block>, DecompileError> {
        self._decom_block(address)
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

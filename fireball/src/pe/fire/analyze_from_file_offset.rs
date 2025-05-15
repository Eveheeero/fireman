use super::Pe;
use crate::{
    core::{Address, Block, Fire},
    prelude::DecompileError,
};
use std::sync::Arc;

impl Pe {
    pub(super) fn _analyze_from_file_offset(
        &self,
        address: u64,
    ) -> Result<Arc<Block>, DecompileError> {
        let address = Address::from_file_offset(&self.sections, address);
        self.analyze_block(&address)
    }
}

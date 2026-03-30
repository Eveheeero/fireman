use super::Elf;
use crate::{
    core::{Address, Block, FireRaw},
    prelude::DecompileError,
};
use std::sync::Arc;

impl Elf {
    pub(super) fn _analyze_from_file_offset(
        &self,
        address: u64,
    ) -> Result<Arc<Block>, DecompileError> {
        let address = Address::from_file_offset(&self.sections, address);
        self.analyze_block(&address)
    }
}

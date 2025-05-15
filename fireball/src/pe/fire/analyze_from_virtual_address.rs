use super::Pe;
use crate::{
    core::{Address, Block, FireRaw},
    prelude::DecompileError,
};
use std::sync::Arc;

impl Pe {
    pub(super) fn _analyze_from_virtual_address(
        &self,
        address: u64,
    ) -> Result<Arc<Block>, DecompileError> {
        let address = Address::from_virtual_address(&self.sections, address);
        self.analyze_block(&address)
    }
}

use super::Pe;
use crate::{
    core::{Address, Fire},
    prelude::DecompileError,
};

impl Pe {
    pub(super) fn _decom_from_file_offset(&self, address: u64) -> Result<(), DecompileError> {
        let address = Address::from_file_offset(&self.sections, address);
        self.decom_block(&address)
    }
}

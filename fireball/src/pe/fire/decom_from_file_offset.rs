use super::PE;
use crate::{core::Address, prelude::DecompileError};

impl PE {
    pub(super) fn _decom_from_file_offset(&self, address: u64) -> Result<(), DecompileError> {
        let _address = Address::from_file_offset(&self.sections, address);
        todo!();
    }
}

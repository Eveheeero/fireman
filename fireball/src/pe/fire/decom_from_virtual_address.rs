use super::PE;
use crate::{core::Address, prelude::DecompileError};

impl PE {
    pub(super) fn _decom_from_virtual_address(&self, address: u64) -> Result<(), DecompileError> {
        let address = Address::from_virtual_address(&self.sections, address);
        self._decom_function(address)
    }
}

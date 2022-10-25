use super::PE;
use crate::core::Address;

impl PE {
    pub(super) fn _decom_from_virtual_address(
        &self,
        address: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _address = Address::from_virtual_address(&self.sections, address);
        todo!();
    }
}

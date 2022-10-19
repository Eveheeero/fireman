use super::PE;
use crate::core::Address;

impl PE {
    pub(super) fn _decom_from_file_offset(
        &self,
        address: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _address = Address::from_file_offset(&self.binary, address as usize);
        todo!();
    }
}

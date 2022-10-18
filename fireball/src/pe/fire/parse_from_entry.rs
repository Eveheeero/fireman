use super::PE;
use crate::core::Address;

impl PE {
    pub(super) fn _decom_from_entry(&self) -> Result<(), Box<dyn std::error::Error>> {
        let gl = goblin::pe::PE::parse(&self.binary)?;
        let entry = Address::from_file_offset(&self.binary, gl.entry);

        let insts = self.parse_assem_range(entry, 0x100).unwrap();

        dbg!(insts);

        Ok(())
    }
}

use super::PE;
use crate::core::{Address, Fire};

impl PE {
    pub(super) fn _decom_from_entry(&self) -> Result<(), Box<dyn std::error::Error>> {
        let gl = goblin::pe::PE::parse(&self.binary)?;

        let entry = Address::from_virtual_address(&self.sections, gl.entry as u64).unwrap();
        let block = self.parse_block(entry);

        dbg!(block);

        Ok(())
    }
}

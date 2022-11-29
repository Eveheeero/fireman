use super::PE;
use crate::{
    core::{Address, Fire},
    prelude::DecompileError,
};

impl PE {
    pub(super) fn _decom_from_entry(&self) -> Result<(), DecompileError> {
        let gl = goblin::pe::PE::parse(&self.binary)?;

        let entry = Address::from_virtual_address(&self.sections, gl.entry as u64).unwrap();

        let mut now = entry;
        loop {
            let block = self.parse_block(now);
            let connected_to = match block.get_connected_to().first() {
                Some(connected_to) => connected_to.clone(),
                None => break,
            };
            now = connected_to.to().clone();
        }

        Ok(())
    }
}

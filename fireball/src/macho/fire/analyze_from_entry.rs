use super::MachO;
use crate::{
    core::{Block, FireRaw},
    prelude::DecompileError,
};
use std::sync::Arc;

impl MachO {
    pub(super) fn _analyze_from_entry(&self) -> Result<Arc<Block>, DecompileError> {
        if self.entry.get_virtual_address() == 0 {
            return Err(DecompileError::NoEntryPoint);
        }
        self.analyze_block(&self.entry)
    }
}

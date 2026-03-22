use super::Elf;
use crate::{
    core::{Block, FireRaw},
    prelude::DecompileError,
};
use std::sync::Arc;

impl Elf {
    pub(super) fn _analyze_from_entry(&self) -> Result<Arc<Block>, DecompileError> {
        self.analyze_block(&self.entry)
    }
}

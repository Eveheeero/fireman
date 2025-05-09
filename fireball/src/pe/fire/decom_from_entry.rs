use super::Pe;
use crate::{
    core::{Block, Fire},
    prelude::DecompileError,
};
use std::sync::Arc;

impl Pe {
    pub(super) fn _decom_from_entry(&self) -> Result<Arc<Block>, DecompileError> {
        self.decom_block(&self.entry)
    }
}

use super::Pe;
use crate::{
    core::{Block, FireRaw},
    prelude::DecompileError,
};
use std::sync::Arc;

impl Pe {
    pub(crate) fn _analyze_from_entry(&self) -> Result<Arc<Block>, DecompileError> {
        self.analyze_block(&self.entry)
    }
}

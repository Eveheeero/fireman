use super::Pe;
use crate::{core::Block, prelude::DecompileError};
use std::sync::Arc;

impl Pe {
    pub(super) fn _decom_all(&self) -> Result<Vec<Arc<Block>>, DecompileError> {
        todo!();
    }
}

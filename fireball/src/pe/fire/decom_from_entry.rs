use super::PE;
use crate::{core::Fire, prelude::DecompileError};

impl PE {
    pub(super) fn _decom_from_entry(&self) -> Result<(), DecompileError> {
        self.decom_block(&self.entry)
    }
}

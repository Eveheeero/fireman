//! Module defining `PreDefinedOffsets`, a container for `PreDefinedOffset` entries

use super::PreDefinedOffset;
use std::sync::{Arc, RwLock, RwLockReadGuard};

/// Struct managing pre-defined address information inside the binary
pub struct PreDefinedOffsets {
    data: RwLock<Vec<PreDefinedOffset>>,
}

impl PreDefinedOffsets {
    /// Creates a container for storing pre-defined offsets.
    ///
    /// ### Returns
    /// - `Arc<Self>` - container managing pre-defined offset information
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }

    /// Inserts a pre-defined offset into the container.
    ///
    /// ### Arguments
    /// - `data: PreDefinedOffset` - pre-defined address information
    pub(crate) fn insert(&self, data: PreDefinedOffset) {
        self.data.write().unwrap().push(data);
    }

    /// Returns a read guard for the internal data.
    ///
    /// ### Returns
    /// - `RwLockReadGuard<Vec<PreDefinedOffset>>` - read guard for internal data
    pub fn get_reader(&self) -> RwLockReadGuard<Vec<PreDefinedOffset>> {
        self.data.read().unwrap()
    }
}

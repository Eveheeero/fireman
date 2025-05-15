//! Modules that contain the `Relations` struct and its methods.

use super::Relation;
use std::sync::{Arc, RwLockReadGuard};

/// Struct to manage relations between blocks
#[derive(Debug)]
pub struct Relations {
    data: std::sync::RwLock<Vec<Relation>>,
}

impl Relations {
    /// Creates a container for managing relations between blocks.
    ///
    /// ### Returns
    /// - `Arc<Self>`: container
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            data: Default::default(),
        })
    }

    pub(crate) fn add_relation(&self, relation: Relation) {
        self.data.write().unwrap().push(relation);
    }
    pub fn get_relations(&self) -> RwLockReadGuard<Vec<Relation>> {
        self.data.read().unwrap()
    }
}

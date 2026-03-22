//! Module for standalone PDB (Program Database) debug info file analysis.
//!
//! Standalone PDB files contain no executable code — only type information,
//! symbol tables, and source line mappings. The [`StandalonePdb`] struct
//! implements the [`Fire`] and [`FireRaw`] traits so it can participate in
//! the unified [`Fireball`] dispatch, producing a C-like type/symbol dump
//! from `decompile_all()`.

mod fire;

use crate::core::{Blocks, PreDefinedOffsets, Relations, Sections};
use std::sync::Arc;

/// A parsed standalone PDB file.
pub struct StandalonePdb {
    /// Original file path, if loaded from disk.
    path: Option<String>,
    /// Raw PDB file bytes.
    binary: Vec<u8>,
    /// Symbols extracted from the PDB.
    defined: Arc<PreDefinedOffsets>,
    /// Empty sections (PDB has no loadable sections).
    sections: Arc<Sections>,
    /// Empty blocks (PDB has no code).
    blocks: Arc<Blocks>,
    /// Empty relations.
    relations: Arc<Relations>,
    /// C-like text dump of types and symbols, built at construction time.
    dump: String,
}

impl std::fmt::Debug for StandalonePdb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StandalonePdb")
            .field("path", &self.path)
            .finish()
    }
}

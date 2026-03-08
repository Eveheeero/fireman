//! Module containing structures for PE files

mod _pe;
pub mod analysis;
pub mod api_prototypes;
mod asm;
mod block;
pub mod cfi_parser;
pub mod dwarf_parser;
mod fire;
mod fmt;
pub mod linker_map;
pub mod pdb_parser;
pub mod rtti;

use self::{
    analysis::{CodeRelocation, ForwardedExport, RwxAnomaly, SectionEntropy, WideString},
    cfi_parser::UnwindFunctionInfo,
    rtti::RttiEntry,
};
use crate::core::{Address, Blocks, PreDefinedOffsets, Relations, Sections};
use std::{
    pin::Pin,
    sync::{Arc, atomic::AtomicBool},
};

pub struct Pe {
    /// Entry address
    entry: Address,
    /// File path
    path: Option<String>,
    /// Binary data
    binary: Vec<u8>,
    /// Capstone engine
    capstone: Pin<Box<capstone::Capstone>>,

    /// Predefined offsets within the file
    defined: Arc<PreDefinedOffsets>,
    /// Section information data
    sections: Arc<Sections>,
    /// Block information data
    blocks: Arc<Blocks>,
    /// Block relation information data
    relations: Arc<Relations>,
    /// Cooperative cancellation flag for long-running analysis
    cancel_token: Arc<AtomicBool>,
    /// L125: Relocation entries — addresses that hold pointers (not integers).
    /// Populated from the PE base relocation table.
    relocation_addresses: Arc<std::collections::HashSet<u64>>,
    /// Entropy metrics for each section, populated during PE load.
    section_entropies: Vec<SectionEntropy>,
    /// Read-write-execute section anomalies detected during PE load.
    rwx_anomalies: Vec<RwxAnomaly>,
    /// UTF-16LE strings found in the raw binary during PE load.
    wide_strings: Vec<WideString>,
    /// Forwarded exports resolved from the export table during PE load.
    forwarded_exports: Vec<ForwardedExport>,
    /// Relocations that point into executable sections.
    code_relocations: Vec<CodeRelocation>,
    /// Win64 unwind records parsed from `.pdata` / `.xdata`.
    unwind_functions: Vec<UnwindFunctionInfo>,
    /// MSVC RTTI-backed vtable/typeinfo records parsed from read-only data.
    rtti_entries: Vec<RttiEntry>,
}

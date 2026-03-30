//! Module containing structures for Mach-O files

mod _macho;
mod asm;
mod block;
mod fire;
mod fmt;

use crate::{
    BinaryKind,
    core::{Address, Blocks, PreDefinedOffsets, Relations, Sections},
};
use iceball::MachineArchitecture;
use std::{
    pin::Pin,
    sync::{Arc, atomic::AtomicBool},
};

pub struct MachO {
    /// Classification of the binary (executable vs dylib vs object).
    kind: BinaryKind,
    /// Entry address
    entry: Address,
    /// File path
    path: Option<String>,
    /// Binary data
    binary: Vec<u8>,
    /// Detected instruction-set architecture for parsing and IR lowering.
    architecture: MachineArchitecture,
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
}

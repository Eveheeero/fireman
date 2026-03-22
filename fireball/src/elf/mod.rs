//! Module containing structures for ELF files

mod _elf;
mod asm;
mod block;
mod fire;
mod fmt;

use crate::core::{Address, Blocks, PreDefinedOffsets, Relations, Sections};
use iceball::MachineArchitecture;
use std::{
    pin::Pin,
    sync::{Arc, atomic::AtomicBool},
};

pub struct Elf {
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

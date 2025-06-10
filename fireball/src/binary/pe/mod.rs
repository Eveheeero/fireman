//! Module containing structures for PE files

mod _pe;
mod asm;
mod block;
mod fire;
mod fmt;

use crate::arch::architecture::ArchitectureInfo;
use crate::core::{Address, Blocks, PreDefinedOffsets, Relations, Sections};
use std::{pin::Pin, sync::Arc};

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
    /// Detected architecture information
    architecture: ArchitectureInfo,
}

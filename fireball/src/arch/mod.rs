//! Module containing implementations for multiple architectures

pub mod arm32;
pub mod arm64;
pub mod x86;
pub mod x86_64;

use crate::core::Instruction;
use crate::ir::statements::IrStatement;

/// Trait for architecture-specific operations
pub trait Architecture {
    /// Architecture name
    fn name(&self) -> &'static str;

    /// Pointer size in bits
    fn pointer_size(&self) -> u8;

    /// Whether this is a little-endian architecture
    fn is_little_endian(&self) -> bool;

    /// Convert instruction to IR statements
    fn instruction_to_ir(&self, instruction: &Instruction) -> Option<&'static [IrStatement]>;
}

/// Supported architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchType {
    X86_64,
    X86,
    Arm64,
    Arm32,
}

impl ArchType {
    /// Detect architecture from binary data
    pub fn detect(data: &[u8]) -> Option<Self> {
        // TODO: Implement architecture detection from binary headers
        // For now, default to x86_64
        Some(ArchType::X86_64)
    }

    /// Get pointer size for this architecture
    pub fn pointer_size(&self) -> u8 {
        match self {
            ArchType::X86_64 | ArchType::Arm64 => 64,
            ArchType::X86 | ArchType::Arm32 => 32,
        }
    }
}

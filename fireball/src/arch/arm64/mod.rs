//! ARM64 (AArch64) architecture support for Fireman decompiler
//!
//! This module implements ARM64 instruction decoding and IR generation.

pub mod instruction_analyze;
pub mod lifter;
pub mod register;

use crate::core::Instruction;
use crate::ir::statements::IrStatement;

/// ARM64-specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arm64Error {
    /// Unsupported instruction
    UnsupportedInstruction(String),
    /// Invalid instruction encoding
    InvalidEncoding(u32),
    /// Unimplemented feature
    Unimplemented(String),
}

impl std::fmt::Display for Arm64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arm64Error::UnsupportedInstruction(name) => {
                write!(f, "Unsupported ARM64 instruction: {}", name)
            }
            Arm64Error::InvalidEncoding(encoding) => {
                write!(f, "Invalid ARM64 instruction encoding: 0x{:08x}", encoding)
            }
            Arm64Error::Unimplemented(feature) => {
                write!(f, "Unimplemented ARM64 feature: {}", feature)
            }
        }
    }
}

impl std::error::Error for Arm64Error {}

/// ARM64 architecture information
#[derive(Debug, Clone)]
pub struct Arm64Info {
    /// Whether this is 64-bit mode (always true for ARM64)
    pub is_64bit: bool,
    /// Endianness (little-endian by default)
    pub is_little_endian: bool,
    /// Whether to use Thumb mode (not applicable to ARM64)
    pub thumb_mode: bool,
}

impl Default for Arm64Info {
    fn default() -> Self {
        Self {
            is_64bit: true,
            is_little_endian: true,
            thumb_mode: false,
        }
    }
}

/// Convert ARM64 instruction to IR statements
pub fn arm64_to_ir(instruction: &Instruction) -> Result<Vec<IrStatement>, Arm64Error> {
    instruction_analyze::create_ir_statement(instruction)
        .ok_or_else(|| Arm64Error::UnsupportedInstruction("Unknown instruction".to_string()))
        .map(|statements| statements.to_vec())
}

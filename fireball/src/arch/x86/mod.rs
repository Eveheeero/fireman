//! x86 (32-bit) architecture support for Fireman decompiler
//!
//! This module implements x86 32-bit instruction decoding and IR generation.
//! It shares many instructions with x86_64 but operates in 32-bit mode.

pub mod instruction_analyze;
pub mod lifter;
pub mod register;

use crate::core::Instruction;
use crate::ir::statements::IrStatement;

/// x86 (32-bit) specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum X86Error {
    /// Unsupported instruction
    UnsupportedInstruction(String),
    /// Invalid instruction encoding
    InvalidEncoding(Vec<u8>),
    /// Unimplemented feature
    Unimplemented(String),
    /// 64-bit instruction in 32-bit mode
    Invalid64BitInstruction,
}

impl std::fmt::Display for X86Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X86Error::UnsupportedInstruction(name) => {
                write!(f, "Unsupported x86 instruction: {}", name)
            }
            X86Error::InvalidEncoding(bytes) => {
                write!(f, "Invalid x86 instruction encoding: {:?}", bytes)
            }
            X86Error::Unimplemented(feature) => {
                write!(f, "Unimplemented x86 feature: {}", feature)
            }
            X86Error::Invalid64BitInstruction => {
                write!(f, "64-bit instruction not valid in 32-bit mode")
            }
        }
    }
}

impl std::error::Error for X86Error {}

/// x86 (32-bit) architecture information
#[derive(Debug, Clone, Default)]
pub struct X86Info {
    /// Always false for 32-bit x86
    pub is_64bit: bool,
    /// CPU features (MMX, SSE, etc.)
    pub features: X86Features,
}

/// x86 CPU features for 32-bit mode
#[derive(Debug, Clone, Default)]
pub struct X86Features {
    pub mmx: bool,
    pub sse: bool,
    pub sse2: bool,
    pub sse3: bool,
    pub ssse3: bool,
    pub sse4_1: bool,
    pub sse4_2: bool,
    pub avx: bool,
    pub avx2: bool,
    pub fpu: bool,
}

/// Convert x86 (32-bit) instruction to IR statements
pub fn x86_to_ir(instruction: &Instruction) -> Result<Vec<IrStatement>, X86Error> {
    instruction_analyze::create_ir_statement(instruction)
        .ok_or_else(|| X86Error::UnsupportedInstruction("Unknown instruction".to_string()))
        .map(|statements| statements.to_vec())
}

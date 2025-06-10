//! Unified x86/x86_64 architecture support
//!
//! This module provides a unified implementation for both 32-bit (x86) and
//! 64-bit (x86_64) x86 architectures, maximizing code reuse while handling
//! architecture-specific differences.

use crate::{
    arch::architecture::{ArchType, ArchitectureInfo},
    core::{Address, Instruction as CoreInstruction},
    ir::{low_ir, statements::IrStatement},
    prelude::*,
};

pub mod common;
pub mod instruction_analyze;
pub mod register;

pub use register::X86Register;

/// Processor mode for x86 family
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorMode {
    /// 16-bit real mode (legacy, not actively supported)
    Real,
    /// 32-bit protected mode (x86)
    Protected32,
    /// 64-bit long mode (x86_64)
    Long64,
}

impl From<ArchType> for ProcessorMode {
    fn from(arch: ArchType) -> Self {
        match arch {
            ArchType::X86 => ProcessorMode::Protected32,
            ArchType::X86_64 => ProcessorMode::Long64,
            _ => panic!("Invalid architecture for x86 processor mode"),
        }
    }
}

/// Unified x86 analyzer that handles both 32-bit and 64-bit modes
pub struct X86UnifiedAnalyzer {
    mode: ProcessorMode,
    arch_info: ArchitectureInfo,
}

impl X86UnifiedAnalyzer {
    /// Create a new analyzer for the specified architecture
    pub fn new(arch_info: ArchitectureInfo) -> Result<Self, DecompileError> {
        match arch_info.arch_type {
            ArchType::X86 | ArchType::X86_64 => Ok(Self {
                mode: arch_info.arch_type.into(),
                arch_info,
            }),
            _ => Err(DecompileError::InvalidArchitecture(
                "X86 analyzer requires x86 or x86_64 architecture".to_string(),
            )),
        }
    }

    /// Get the current processor mode
    pub fn mode(&self) -> ProcessorMode {
        self.mode
    }

    /// Get the pointer size in bytes
    pub fn pointer_size(&self) -> u8 {
        match self.mode {
            ProcessorMode::Protected32 => 4,
            ProcessorMode::Long64 => 8,
            ProcessorMode::Real => 2,
        }
    }

    /// Check if this is 64-bit mode
    pub fn is_64bit(&self) -> bool {
        matches!(self.mode, ProcessorMode::Long64)
    }

    /// Get default operand size in bytes
    pub fn default_operand_size(&self) -> u8 {
        match self.mode {
            ProcessorMode::Protected32 => 4,
            ProcessorMode::Long64 => 4, // Note: default is still 32-bit in long mode
            ProcessorMode::Real => 2,
        }
    }

    /// Create IR statements from an instruction
    pub fn instruction_to_ir(
        &self,
        instruction: &CoreInstruction,
    ) -> Result<Vec<IrStatement>, DecompileError> {
        instruction_analyze::create_ir_statements(self, instruction)
    }

    /// Get register size based on mode and register type
    pub fn get_register_size(&self, reg: &X86Register) -> u8 {
        use X86Register::*;

        match reg {
            // 8-bit registers
            AL | CL | DL | BL | AH | CH | DH | BH | SIL | DIL | SPL | BPL => 1,

            // 8-bit extended registers (64-bit mode only)
            R8B | R9B | R10B | R11B | R12B | R13B | R14B | R15B => {
                if self.is_64bit() {
                    1
                } else {
                    0
                }
            }

            // 16-bit registers
            AX | CX | DX | BX | SP | BP | SI | DI => 2,

            // 16-bit extended registers (64-bit mode only)
            R8W | R9W | R10W | R11W | R12W | R13W | R14W | R15W => {
                if self.is_64bit() {
                    2
                } else {
                    0
                }
            }

            // 32-bit registers
            EAX | ECX | EDX | EBX | ESP | EBP | ESI | EDI => 4,

            // 32-bit extended registers (64-bit mode only)
            R8D | R9D | R10D | R11D | R12D | R13D | R14D | R15D => {
                if self.is_64bit() {
                    4
                } else {
                    0
                }
            }

            // 64-bit registers (64-bit mode only)
            RAX | RCX | RDX | RBX | RSP | RBP | RSI | RDI | R8 | R9 | R10 | R11 | R12 | R13
            | R14 | R15 | RIP => {
                if self.is_64bit() {
                    8
                } else {
                    0
                }
            }

            // Segment registers
            CS | DS | ES | FS | GS | SS => 2,

            // Flags register
            EFLAGS => 4,
            RFLAGS => {
                if self.is_64bit() {
                    8
                } else {
                    4
                }
            }

            // Control registers
            CR0 | CR2 | CR3 | CR4 | CR8 => self.pointer_size(),

            // Debug registers
            DR0 | DR1 | DR2 | DR3 | DR6 | DR7 => self.pointer_size(),

            // Vector registers (size depends on context)
            XMM(_) => 16,
            YMM(_) => 32,
            ZMM(_) => 64,

            ST(_) => 10, // x87 FPU registers are 80-bit
        }
    }

    /// Convert a 32-bit register to its 64-bit equivalent (if applicable)
    pub fn extend_register_64(&self, reg: X86Register) -> Option<X86Register> {
        if !self.is_64bit() {
            return None;
        }

        use X86Register::*;
        match reg {
            EAX => Some(RAX),
            ECX => Some(RCX),
            EDX => Some(RDX),
            EBX => Some(RBX),
            ESP => Some(RSP),
            EBP => Some(RBP),
            ESI => Some(RSI),
            EDI => Some(RDI),
            _ => None,
        }
    }

    /// Handle zero-extension in 64-bit mode
    /// In x64, 32-bit operations zero-extend to 64 bits
    pub fn handle_zero_extension(&self, statements: Vec<IrStatement>) -> Vec<IrStatement> {
        if !self.is_64bit() {
            return statements;
        }

        // TODO: Implement zero-extension logic for 32-bit operations in 64-bit mode
        // For now, return as-is
        statements
    }
}

/// Common utilities for x86 family
pub mod utils {
    use super::*;

    /// Check if an instruction uses REX prefix (64-bit only)
    pub fn has_rex_prefix(bytes: &[u8]) -> bool {
        bytes.first().map(|&b| (b & 0xF0) == 0x40).unwrap_or(false)
    }

    /// Check if REX.W is set (64-bit operand size)
    pub fn has_rex_w(bytes: &[u8]) -> bool {
        bytes.first().map(|&b| b & 0x48 == 0x48).unwrap_or(false)
    }

    /// Get effective operand size
    pub fn get_operand_size(mode: ProcessorMode, inst: &CoreInstruction) -> u8 {
        // TODO: Check prefixes, REX.W, etc.
        match mode {
            ProcessorMode::Protected32 => 4,
            ProcessorMode::Long64 => {
                // Check for REX.W prefix
                if let Some(bytes) = &inst.inner().bytes {
                    if has_rex_w(bytes) {
                        return 8;
                    }
                }
                4 // Default to 32-bit in 64-bit mode
            }
            ProcessorMode::Real => 2,
        }
    }

    /// Check if address size override prefix is present
    pub fn has_address_size_override(bytes: &[u8]) -> bool {
        bytes.iter().any(|&b| b == 0x67)
    }

    /// Check if operand size override prefix is present
    pub fn has_operand_size_override(bytes: &[u8]) -> bool {
        bytes.iter().any(|&b| b == 0x66)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_mode_from_arch() {
        assert_eq!(
            ProcessorMode::from(ArchType::X86),
            ProcessorMode::Protected32
        );
        assert_eq!(ProcessorMode::from(ArchType::X86_64), ProcessorMode::Long64);
    }

    #[test]
    fn test_pointer_size() {
        let arch_32 = ArchitectureInfo {
            arch_type: ArchType::X86,
            pointer_size: 32,
            endianness: crate::arch::architecture::Endianness::Little,
            register_count: 8,
            instruction_alignment: 1,
        };

        let analyzer_32 = X86UnifiedAnalyzer::new(arch_32).unwrap();
        assert_eq!(analyzer_32.pointer_size(), 4);
        assert!(!analyzer_32.is_64bit());

        let arch_64 = ArchitectureInfo {
            arch_type: ArchType::X86_64,
            pointer_size: 64,
            endianness: crate::arch::architecture::Endianness::Little,
            register_count: 16,
            instruction_alignment: 1,
        };

        let analyzer_64 = X86UnifiedAnalyzer::new(arch_64).unwrap();
        assert_eq!(analyzer_64.pointer_size(), 8);
        assert!(analyzer_64.is_64bit());
    }
}

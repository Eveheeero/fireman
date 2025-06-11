//! x87 FPU stack management for proper instruction semantics
//!
//! The x87 FPU uses a stack-based architecture with 8 registers (ST0-ST7).
//! The TOP field in the status word indicates which physical register is
//! currently the top of stack (ST0).

use crate::ir::Register;
use crate::ir::data::IrData;
use crate::utils::Aos;

/// FPU status word bits
pub struct FpuStatusWord;

impl FpuStatusWord {
    /// TOP field mask (bits 11-13)
    pub const TOP_MASK: u16 = 0x3800;
    /// TOP field shift
    pub const TOP_SHIFT: u16 = 11;

    /// Condition code C0 (bit 8)
    pub const C0: u16 = 1 << 8;
    /// Condition code C1 (bit 9)
    pub const C1: u16 = 1 << 9;
    /// Condition code C2 (bit 10)
    pub const C2: u16 = 1 << 10;
    /// Condition code C3 (bit 14)
    pub const C3: u16 = 1 << 14;

    /// Stack fault (bit 6)
    pub const SF: u16 = 1 << 6;
    /// Exception summary (bit 7)
    pub const ES: u16 = 1 << 7;
    /// Busy (bit 15)
    pub const B: u16 = 1 << 15;
}

/// Get the FPU status word register
#[inline]
pub fn fpu_status_word() -> Aos<IrData> {
    IrData::Register(Register::new("fpsw", 0..16)).into()
}

/// Get the FPU control word register
#[inline]
pub fn fpu_control_word() -> Aos<IrData> {
    IrData::Register(Register::new("fpcw", 0..16)).into()
}

/// Get FPU tag word register (tracks which stack slots are valid)
#[inline]
pub fn fpu_tag_word() -> Aos<IrData> {
    IrData::Register(Register::new("fptw", 0..16)).into()
}

/// Helper to get ST(i) register
/// This is a simplified version that doesn't handle TOP yet
#[inline]
pub fn fpu_st(index: u8) -> Aos<IrData> {
    // TODO: This should compute the actual physical register based on TOP
    // For now, we use a simplified naming scheme
    assert!(index < 8, "FPU stack index must be 0-7");
    // Use static names for the FPU stack registers
    let name = match index {
        0 => "st0",
        1 => "st1",
        2 => "st2",
        3 => "st3",
        4 => "st4",
        5 => "st5",
        6 => "st6",
        7 => "st7",
        _ => unreachable!("Index already checked"),
    };
    IrData::Register(Register::new(name, 0..80)).into()
}

/// FPU comparison results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FpuCompareResult {
    Less,
    Equal,
    Greater,
    Unordered, // For NaN comparisons
}

// The complex operations (push, pop, etc.) need to be implemented
// within the instruction_analyze module where we have access to the
// shortcuts and helper functions.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fpu_registers() {
        // Test stack register creation
        for i in 0..8 {
            let _reg = fpu_st(i);
            // Just verify it doesn't panic
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_stack_register() {
        fpu_st(8); // Should panic
    }
}

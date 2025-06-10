//! ARM32 (ARMv7) architecture support

use crate::utils::error::FireballError;

pub mod decoder;
pub mod instruction_analyze;
pub mod lifter;
pub mod register;

pub use register::str_to_arm32_register;

/// ARM32-specific errors
#[derive(Debug, Clone)]
pub enum Arm32Error {
    /// Unsupported instruction
    UnsupportedInstruction(String),
    /// Invalid instruction encoding
    InvalidEncoding(u32),
    /// Unimplemented feature
    Unimplemented(String),
}

impl From<Arm32Error> for FireballError {
    fn from(err: Arm32Error) -> Self {
        match err {
            Arm32Error::UnsupportedInstruction(msg) => {
                FireballError::InvalidBinary(format!("ARM32: Unsupported instruction - {}", msg))
            }
            Arm32Error::InvalidEncoding(encoding) => {
                FireballError::InvalidBinary(format!("ARM32: Invalid encoding 0x{:08x}", encoding))
            }
            Arm32Error::Unimplemented(feature) => {
                FireballError::Unimplemented(format!("ARM32: {}", feature))
            }
        }
    }
}

impl From<iceball::DisassembleError> for Arm32Error {
    fn from(err: iceball::DisassembleError) -> Self {
        match err {
            iceball::DisassembleError::Unknown => {
                Arm32Error::UnsupportedInstruction("Unknown error".to_string())
            }
            iceball::DisassembleError::UnknownStatement => {
                Arm32Error::UnsupportedInstruction("Unknown statement".to_string())
            }
            iceball::DisassembleError::UnknownRegister => {
                Arm32Error::UnsupportedInstruction("Unknown register".to_string())
            }
        }
    }
}

/// ARM32 instruction condition codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    EQ = 0x0, // Equal
    NE = 0x1, // Not equal
    CS = 0x2, // Carry set / unsigned higher or same
    CC = 0x3, // Carry clear / unsigned lower
    MI = 0x4, // Minus / negative
    PL = 0x5, // Plus / positive or zero
    VS = 0x6, // Overflow
    VC = 0x7, // No overflow
    HI = 0x8, // Unsigned higher
    LS = 0x9, // Unsigned lower or same
    GE = 0xA, // Signed greater than or equal
    LT = 0xB, // Signed less than
    GT = 0xC, // Signed greater than
    LE = 0xD, // Signed less than or equal
    AL = 0xE, // Always (unconditional)
    NV = 0xF, // Never (ARMv1-v3 only, now used for other purposes)
}

/// ARM32 instruction modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionSet {
    /// ARM mode (32-bit instructions)
    ARM,
    /// Thumb mode (16-bit instructions)
    Thumb,
    /// Thumb-2 mode (mix of 16/32-bit instructions)
    Thumb2,
}

/// ARM32 processor mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorMode {
    User = 0x10,
    FIQ = 0x11,
    IRQ = 0x12,
    Supervisor = 0x13,
    Abort = 0x17,
    Undefined = 0x1B,
    System = 0x1F,
}

/// ARM32 architecture information
pub struct Arm32Info {
    /// Current instruction set mode
    pub mode: InstructionSet,
    /// Processor mode
    pub proc_mode: ProcessorMode,
    /// Architecture version (e.g., 7 for ARMv7)
    pub version: u8,
    /// CPU features
    pub features: Arm32Features,
}

/// ARM32 CPU features
pub struct Arm32Features {
    /// Thumb-2 support
    pub thumb2: bool,
    /// NEON SIMD support
    pub neon: bool,
    /// VFPv3 floating-point support
    pub vfpv3: bool,
    /// VFPv4 floating-point support
    pub vfpv4: bool,
    /// Integer divide instructions
    pub idiv: bool,
    /// Security extensions (TrustZone)
    pub trustzone: bool,
    /// Multiprocessing extensions
    pub mp_ext: bool,
}

impl Default for Arm32Features {
    fn default() -> Self {
        Self {
            thumb2: true, // ARMv7 typically has Thumb-2
            neon: false,
            vfpv3: false,
            vfpv4: false,
            idiv: false,
            trustzone: false,
            mp_ext: false,
        }
    }
}

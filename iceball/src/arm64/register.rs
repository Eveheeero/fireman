//! ARM64 (AArch64) register definitions

use crate::DisassembleError;

/// ARM64 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Arm64Register {
    // 64-bit general purpose registers (X0-X30)
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,

    // Zero register (64-bit)
    XZR,

    // Stack pointer (64-bit)
    SP,

    // 32-bit general purpose registers (W0-W30)
    W0,
    W1,
    W2,
    W3,
    W4,
    W5,
    W6,
    W7,
    W8,
    W9,
    W10,
    W11,
    W12,
    W13,
    W14,
    W15,
    W16,
    W17,
    W18,
    W19,
    W20,
    W21,
    W22,
    W23,
    W24,
    W25,
    W26,
    W27,
    W28,
    W29,
    W30,

    // Zero register (32-bit)
    WZR,

    // Stack pointer (32-bit)
    WSP,

    // SIMD/FP registers (128-bit vectors)
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31,

    // System registers
    PC,   // Program counter
    NZCV, // Condition flags
    FPCR, // Floating-point control register
    FPSR, // Floating-point status register
}

impl Arm64Register {
    /// Parse a register from string
    pub fn parse(s: &str) -> Result<Self, DisassembleError> {
        let s = s.trim().to_lowercase();

        match s.as_str() {
            // 64-bit GPRs
            "x0" => Ok(Self::X0),
            "x1" => Ok(Self::X1),
            "x2" => Ok(Self::X2),
            "x3" => Ok(Self::X3),
            "x4" => Ok(Self::X4),
            "x5" => Ok(Self::X5),
            "x6" => Ok(Self::X6),
            "x7" => Ok(Self::X7),
            "x8" => Ok(Self::X8),
            "x9" => Ok(Self::X9),
            "x10" => Ok(Self::X10),
            "x11" => Ok(Self::X11),
            "x12" => Ok(Self::X12),
            "x13" => Ok(Self::X13),
            "x14" => Ok(Self::X14),
            "x15" => Ok(Self::X15),
            "x16" => Ok(Self::X16),
            "x17" => Ok(Self::X17),
            "x18" => Ok(Self::X18),
            "x19" => Ok(Self::X19),
            "x20" => Ok(Self::X20),
            "x21" => Ok(Self::X21),
            "x22" => Ok(Self::X22),
            "x23" => Ok(Self::X23),
            "x24" => Ok(Self::X24),
            "x25" => Ok(Self::X25),
            "x26" => Ok(Self::X26),
            "x27" => Ok(Self::X27),
            "x28" => Ok(Self::X28),
            "x29" | "fp" => Ok(Self::X29), // Frame pointer
            "x30" | "lr" => Ok(Self::X30), // Link register
            "xzr" => Ok(Self::XZR),
            "sp" => Ok(Self::SP),

            // 32-bit GPRs
            "w0" => Ok(Self::W0),
            "w1" => Ok(Self::W1),
            "w2" => Ok(Self::W2),
            "w3" => Ok(Self::W3),
            "w4" => Ok(Self::W4),
            "w5" => Ok(Self::W5),
            "w6" => Ok(Self::W6),
            "w7" => Ok(Self::W7),
            "w8" => Ok(Self::W8),
            "w9" => Ok(Self::W9),
            "w10" => Ok(Self::W10),
            "w11" => Ok(Self::W11),
            "w12" => Ok(Self::W12),
            "w13" => Ok(Self::W13),
            "w14" => Ok(Self::W14),
            "w15" => Ok(Self::W15),
            "w16" => Ok(Self::W16),
            "w17" => Ok(Self::W17),
            "w18" => Ok(Self::W18),
            "w19" => Ok(Self::W19),
            "w20" => Ok(Self::W20),
            "w21" => Ok(Self::W21),
            "w22" => Ok(Self::W22),
            "w23" => Ok(Self::W23),
            "w24" => Ok(Self::W24),
            "w25" => Ok(Self::W25),
            "w26" => Ok(Self::W26),
            "w27" => Ok(Self::W27),
            "w28" => Ok(Self::W28),
            "w29" => Ok(Self::W29),
            "w30" => Ok(Self::W30),
            "wzr" => Ok(Self::WZR),
            "wsp" => Ok(Self::WSP),

            // Vector registers
            "v0" => Ok(Self::V0),
            "v1" => Ok(Self::V1),
            "v2" => Ok(Self::V2),
            "v3" => Ok(Self::V3),
            "v4" => Ok(Self::V4),
            "v5" => Ok(Self::V5),
            "v6" => Ok(Self::V6),
            "v7" => Ok(Self::V7),
            "v8" => Ok(Self::V8),
            "v9" => Ok(Self::V9),
            "v10" => Ok(Self::V10),
            "v11" => Ok(Self::V11),
            "v12" => Ok(Self::V12),
            "v13" => Ok(Self::V13),
            "v14" => Ok(Self::V14),
            "v15" => Ok(Self::V15),
            "v16" => Ok(Self::V16),
            "v17" => Ok(Self::V17),
            "v18" => Ok(Self::V18),
            "v19" => Ok(Self::V19),
            "v20" => Ok(Self::V20),
            "v21" => Ok(Self::V21),
            "v22" => Ok(Self::V22),
            "v23" => Ok(Self::V23),
            "v24" => Ok(Self::V24),
            "v25" => Ok(Self::V25),
            "v26" => Ok(Self::V26),
            "v27" => Ok(Self::V27),
            "v28" => Ok(Self::V28),
            "v29" => Ok(Self::V29),
            "v30" => Ok(Self::V30),
            "v31" => Ok(Self::V31),

            // System registers
            "pc" => Ok(Self::PC),
            "nzcv" => Ok(Self::NZCV),
            "fpcr" => Ok(Self::FPCR),
            "fpsr" => Ok(Self::FPSR),

            _ => Err(DisassembleError::UnknownRegister),
        }
    }

    /// Get the size of the register in bytes
    pub fn size(&self) -> usize {
        match self {
            // 64-bit registers
            Self::X0
            | Self::X1
            | Self::X2
            | Self::X3
            | Self::X4
            | Self::X5
            | Self::X6
            | Self::X7
            | Self::X8
            | Self::X9
            | Self::X10
            | Self::X11
            | Self::X12
            | Self::X13
            | Self::X14
            | Self::X15
            | Self::X16
            | Self::X17
            | Self::X18
            | Self::X19
            | Self::X20
            | Self::X21
            | Self::X22
            | Self::X23
            | Self::X24
            | Self::X25
            | Self::X26
            | Self::X27
            | Self::X28
            | Self::X29
            | Self::X30
            | Self::XZR
            | Self::SP => 8,

            // 32-bit registers
            Self::W0
            | Self::W1
            | Self::W2
            | Self::W3
            | Self::W4
            | Self::W5
            | Self::W6
            | Self::W7
            | Self::W8
            | Self::W9
            | Self::W10
            | Self::W11
            | Self::W12
            | Self::W13
            | Self::W14
            | Self::W15
            | Self::W16
            | Self::W17
            | Self::W18
            | Self::W19
            | Self::W20
            | Self::W21
            | Self::W22
            | Self::W23
            | Self::W24
            | Self::W25
            | Self::W26
            | Self::W27
            | Self::W28
            | Self::W29
            | Self::W30
            | Self::WZR
            | Self::WSP => 4,

            // 128-bit vector registers
            Self::V0
            | Self::V1
            | Self::V2
            | Self::V3
            | Self::V4
            | Self::V5
            | Self::V6
            | Self::V7
            | Self::V8
            | Self::V9
            | Self::V10
            | Self::V11
            | Self::V12
            | Self::V13
            | Self::V14
            | Self::V15
            | Self::V16
            | Self::V17
            | Self::V18
            | Self::V19
            | Self::V20
            | Self::V21
            | Self::V22
            | Self::V23
            | Self::V24
            | Self::V25
            | Self::V26
            | Self::V27
            | Self::V28
            | Self::V29
            | Self::V30
            | Self::V31 => 16,

            // System registers
            Self::PC => 8,
            Self::NZCV | Self::FPCR | Self::FPSR => 4,
        }
    }

    /// Check if this is a general-purpose register
    pub fn is_gpr(&self) -> bool {
        matches!(
            self,
            Self::X0
                | Self::X1
                | Self::X2
                | Self::X3
                | Self::X4
                | Self::X5
                | Self::X6
                | Self::X7
                | Self::X8
                | Self::X9
                | Self::X10
                | Self::X11
                | Self::X12
                | Self::X13
                | Self::X14
                | Self::X15
                | Self::X16
                | Self::X17
                | Self::X18
                | Self::X19
                | Self::X20
                | Self::X21
                | Self::X22
                | Self::X23
                | Self::X24
                | Self::X25
                | Self::X26
                | Self::X27
                | Self::X28
                | Self::X29
                | Self::X30
                | Self::XZR
                | Self::SP
                | Self::W0
                | Self::W1
                | Self::W2
                | Self::W3
                | Self::W4
                | Self::W5
                | Self::W6
                | Self::W7
                | Self::W8
                | Self::W9
                | Self::W10
                | Self::W11
                | Self::W12
                | Self::W13
                | Self::W14
                | Self::W15
                | Self::W16
                | Self::W17
                | Self::W18
                | Self::W19
                | Self::W20
                | Self::W21
                | Self::W22
                | Self::W23
                | Self::W24
                | Self::W25
                | Self::W26
                | Self::W27
                | Self::W28
                | Self::W29
                | Self::W30
                | Self::WZR
                | Self::WSP
        )
    }

    /// Check if this is a vector register
    pub fn is_vector(&self) -> bool {
        matches!(
            self,
            Self::V0
                | Self::V1
                | Self::V2
                | Self::V3
                | Self::V4
                | Self::V5
                | Self::V6
                | Self::V7
                | Self::V8
                | Self::V9
                | Self::V10
                | Self::V11
                | Self::V12
                | Self::V13
                | Self::V14
                | Self::V15
                | Self::V16
                | Self::V17
                | Self::V18
                | Self::V19
                | Self::V20
                | Self::V21
                | Self::V22
                | Self::V23
                | Self::V24
                | Self::V25
                | Self::V26
                | Self::V27
                | Self::V28
                | Self::V29
                | Self::V30
                | Self::V31
        )
    }

    /// Get the 64-bit version of a register (if applicable)
    pub fn to_64bit(&self) -> Option<Self> {
        match self {
            // Already 64-bit
            Self::X0
            | Self::X1
            | Self::X2
            | Self::X3
            | Self::X4
            | Self::X5
            | Self::X6
            | Self::X7
            | Self::X8
            | Self::X9
            | Self::X10
            | Self::X11
            | Self::X12
            | Self::X13
            | Self::X14
            | Self::X15
            | Self::X16
            | Self::X17
            | Self::X18
            | Self::X19
            | Self::X20
            | Self::X21
            | Self::X22
            | Self::X23
            | Self::X24
            | Self::X25
            | Self::X26
            | Self::X27
            | Self::X28
            | Self::X29
            | Self::X30
            | Self::XZR
            | Self::SP => Some(*self),

            // 32-bit to 64-bit mapping
            Self::W0 => Some(Self::X0),
            Self::W1 => Some(Self::X1),
            Self::W2 => Some(Self::X2),
            Self::W3 => Some(Self::X3),
            Self::W4 => Some(Self::X4),
            Self::W5 => Some(Self::X5),
            Self::W6 => Some(Self::X6),
            Self::W7 => Some(Self::X7),
            Self::W8 => Some(Self::X8),
            Self::W9 => Some(Self::X9),
            Self::W10 => Some(Self::X10),
            Self::W11 => Some(Self::X11),
            Self::W12 => Some(Self::X12),
            Self::W13 => Some(Self::X13),
            Self::W14 => Some(Self::X14),
            Self::W15 => Some(Self::X15),
            Self::W16 => Some(Self::X16),
            Self::W17 => Some(Self::X17),
            Self::W18 => Some(Self::X18),
            Self::W19 => Some(Self::X19),
            Self::W20 => Some(Self::X20),
            Self::W21 => Some(Self::X21),
            Self::W22 => Some(Self::X22),
            Self::W23 => Some(Self::X23),
            Self::W24 => Some(Self::X24),
            Self::W25 => Some(Self::X25),
            Self::W26 => Some(Self::X26),
            Self::W27 => Some(Self::X27),
            Self::W28 => Some(Self::X28),
            Self::W29 => Some(Self::X29),
            Self::W30 => Some(Self::X30),
            Self::WZR => Some(Self::XZR),
            Self::WSP => Some(Self::SP),

            // Vector and system registers don't have 64-bit equivalents
            _ => None,
        }
    }
}

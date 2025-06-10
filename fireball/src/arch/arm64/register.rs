//! ARM64 register definitions and utilities

use crate::ir::data::IrData;
use crate::utils::Aos;
use std::sync::LazyLock;

/// ARM64 general-purpose registers (64-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Arm64Register {
    // General-purpose registers (64-bit)
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
    X30, // X30 is LR

    // Stack pointer
    SP,

    // Zero register
    XZR,

    // Program counter
    PC,

    // 32-bit views of registers (W0-W30)
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

    // 32-bit zero register
    WZR,

    // SIMD/FP registers (128-bit)
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
    NZCV, // Condition flags
    FPCR, // Floating-point control register
    FPSR, // Floating-point status register
}

impl Arm64Register {
    /// Get the bit size of this register
    pub fn bit_size(&self) -> u8 {
        use Arm64Register::*;
        match self {
            // 64-bit registers
            X0 | X1 | X2 | X3 | X4 | X5 | X6 | X7 | X8 | X9 | X10 | X11 | X12 | X13 | X14 | X15
            | X16 | X17 | X18 | X19 | X20 | X21 | X22 | X23 | X24 | X25 | X26 | X27 | X28 | X29
            | X30 | SP | XZR | PC => 64,

            // 32-bit registers
            W0 | W1 | W2 | W3 | W4 | W5 | W6 | W7 | W8 | W9 | W10 | W11 | W12 | W13 | W14 | W15
            | W16 | W17 | W18 | W19 | W20 | W21 | W22 | W23 | W24 | W25 | W26 | W27 | W28 | W29
            | W30 | WZR => 32,

            // 128-bit SIMD registers
            V0 | V1 | V2 | V3 | V4 | V5 | V6 | V7 | V8 | V9 | V10 | V11 | V12 | V13 | V14 | V15
            | V16 | V17 | V18 | V19 | V20 | V21 | V22 | V23 | V24 | V25 | V26 | V27 | V28 | V29
            | V30 | V31 => 128,

            // Special registers
            NZCV => 32,
            FPCR | FPSR => 32,
        }
    }

    /// Get the name of this register
    pub fn name(&self) -> &'static str {
        use Arm64Register::*;
        match self {
            X0 => "x0",
            X1 => "x1",
            X2 => "x2",
            X3 => "x3",
            X4 => "x4",
            X5 => "x5",
            X6 => "x6",
            X7 => "x7",
            X8 => "x8",
            X9 => "x9",
            X10 => "x10",
            X11 => "x11",
            X12 => "x12",
            X13 => "x13",
            X14 => "x14",
            X15 => "x15",
            X16 => "x16",
            X17 => "x17",
            X18 => "x18",
            X19 => "x19",
            X20 => "x20",
            X21 => "x21",
            X22 => "x22",
            X23 => "x23",
            X24 => "x24",
            X25 => "x25",
            X26 => "x26",
            X27 => "x27",
            X28 => "x28",
            X29 => "x29",
            X30 => "x30",
            SP => "sp",
            XZR => "xzr",
            PC => "pc",

            W0 => "w0",
            W1 => "w1",
            W2 => "w2",
            W3 => "w3",
            W4 => "w4",
            W5 => "w5",
            W6 => "w6",
            W7 => "w7",
            W8 => "w8",
            W9 => "w9",
            W10 => "w10",
            W11 => "w11",
            W12 => "w12",
            W13 => "w13",
            W14 => "w14",
            W15 => "w15",
            W16 => "w16",
            W17 => "w17",
            W18 => "w18",
            W19 => "w19",
            W20 => "w20",
            W21 => "w21",
            W22 => "w22",
            W23 => "w23",
            W24 => "w24",
            W25 => "w25",
            W26 => "w26",
            W27 => "w27",
            W28 => "w28",
            W29 => "w29",
            W30 => "w30",
            WZR => "wzr",

            V0 => "v0",
            V1 => "v1",
            V2 => "v2",
            V3 => "v3",
            V4 => "v4",
            V5 => "v5",
            V6 => "v6",
            V7 => "v7",
            V8 => "v8",
            V9 => "v9",
            V10 => "v10",
            V11 => "v11",
            V12 => "v12",
            V13 => "v13",
            V14 => "v14",
            V15 => "v15",
            V16 => "v16",
            V17 => "v17",
            V18 => "v18",
            V19 => "v19",
            V20 => "v20",
            V21 => "v21",
            V22 => "v22",
            V23 => "v23",
            V24 => "v24",
            V25 => "v25",
            V26 => "v26",
            V27 => "v27",
            V28 => "v28",
            V29 => "v29",
            V30 => "v30",
            V31 => "v31",

            NZCV => "nzcv",
            FPCR => "fpcr",
            FPSR => "fpsr",
        }
    }

    /// Check if this is a general-purpose register
    pub fn is_gpr(&self) -> bool {
        use Arm64Register::*;
        matches!(
            self,
            X0 | X1
                | X2
                | X3
                | X4
                | X5
                | X6
                | X7
                | X8
                | X9
                | X10
                | X11
                | X12
                | X13
                | X14
                | X15
                | X16
                | X17
                | X18
                | X19
                | X20
                | X21
                | X22
                | X23
                | X24
                | X25
                | X26
                | X27
                | X28
                | X29
                | X30
                | SP
                | XZR
                | PC
                | W0
                | W1
                | W2
                | W3
                | W4
                | W5
                | W6
                | W7
                | W8
                | W9
                | W10
                | W11
                | W12
                | W13
                | W14
                | W15
                | W16
                | W17
                | W18
                | W19
                | W20
                | W21
                | W22
                | W23
                | W24
                | W25
                | W26
                | W27
                | W28
                | W29
                | W30
                | WZR
        )
    }

    /// Check if this is a SIMD/FP register
    pub fn is_simd(&self) -> bool {
        use Arm64Register::*;
        matches!(
            self,
            V0 | V1
                | V2
                | V3
                | V4
                | V5
                | V6
                | V7
                | V8
                | V9
                | V10
                | V11
                | V12
                | V13
                | V14
                | V15
                | V16
                | V17
                | V18
                | V19
                | V20
                | V21
                | V22
                | V23
                | V24
                | V25
                | V26
                | V27
                | V28
                | V29
                | V30
                | V31
        )
    }
}

// Static register references for IR generation
macro_rules! define_register {
    ($name:ident, $reg:expr) => {
        pub static $name: LazyLock<Aos<IrData>> = LazyLock::new(|| {
            Aos::new_static(IrData::Register(crate::ir::Register::new(
                $reg.name(),
                0..64,
            )))
        });
    };
}

// Define commonly used registers
define_register!(x0, Arm64Register::X0);
define_register!(x1, Arm64Register::X1);
define_register!(x2, Arm64Register::X2);
define_register!(x3, Arm64Register::X3);
define_register!(x4, Arm64Register::X4);
define_register!(x5, Arm64Register::X5);
define_register!(x6, Arm64Register::X6);
define_register!(x7, Arm64Register::X7);
define_register!(x8, Arm64Register::X8);
define_register!(x29, Arm64Register::X29); // Frame pointer
define_register!(x30, Arm64Register::X30); // Link register
define_register!(sp, Arm64Register::SP);
define_register!(pc, Arm64Register::PC);
define_register!(xzr, Arm64Register::XZR);

// Condition flags
define_register!(nzcv, Arm64Register::NZCV);

/// ARM64 condition codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    EQ = 0b0000, // Equal
    NE = 0b0001, // Not equal
    CS = 0b0010, // Carry set (HS)
    CC = 0b0011, // Carry clear (LO)
    MI = 0b0100, // Minus/negative
    PL = 0b0101, // Plus/positive
    VS = 0b0110, // Overflow set
    VC = 0b0111, // Overflow clear
    HI = 0b1000, // Unsigned higher
    LS = 0b1001, // Unsigned lower or same
    GE = 0b1010, // Signed greater or equal
    LT = 0b1011, // Signed less than
    GT = 0b1100, // Signed greater than
    LE = 0b1101, // Signed less or equal
    AL = 0b1110, // Always
    NV = 0b1111, // Never (reserved)
}

impl Condition {
    /// Get the condition name
    pub fn name(&self) -> &'static str {
        match self {
            Condition::EQ => "eq",
            Condition::NE => "ne",
            Condition::CS => "cs",
            Condition::CC => "cc",
            Condition::MI => "mi",
            Condition::PL => "pl",
            Condition::VS => "vs",
            Condition::VC => "vc",
            Condition::HI => "hi",
            Condition::LS => "ls",
            Condition::GE => "ge",
            Condition::LT => "lt",
            Condition::GT => "gt",
            Condition::LE => "le",
            Condition::AL => "al",
            Condition::NV => "nv",
        }
    }
}

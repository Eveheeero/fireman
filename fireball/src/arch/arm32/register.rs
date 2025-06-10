//! ARM32 register definitions

use crate::ir::Register;
use crate::ir::data::{IrData, IrIntrinsic};
use crate::utils::Aos;
use std::fmt;
use std::sync::LazyLock;

/// ARM32 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Arm32Register {
    // Core registers (R0-R15)
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13, // SP (Stack Pointer)
    R14, // LR (Link Register)
    R15, // PC (Program Counter)

    // Special names for registers
    SP, // Alias for R13
    LR, // Alias for R14
    PC, // Alias for R15

    // Current Program Status Register
    CPSR,

    // Saved Program Status Registers (banked registers)
    SpsrFiq,
    SpsrIrq,
    SpsrSvc,
    SpsrAbt,
    SpsrUnd,

    // Banked registers for different modes
    // FIQ mode has its own R8-R14
    R8Fiq,
    R9Fiq,
    R10Fiq,
    R11Fiq,
    R12Fiq,
    R13Fiq, // SP_FIQ
    R14Fiq, // LR_FIQ

    // Other modes have their own SP and LR
    R13Irq, // SP_IRQ
    R14Irq, // LR_IRQ
    R13Svc, // SP_SVC
    R14Svc, // LR_SVC
    R13Abt, // SP_ABT
    R14Abt, // LR_ABT
    R13Und, // SP_UND
    R14Und, // LR_UND

    // VFP/NEON single-precision registers (S0-S31)
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
    S13,
    S14,
    S15,
    S16,
    S17,
    S18,
    S19,
    S20,
    S21,
    S22,
    S23,
    S24,
    S25,
    S26,
    S27,
    S28,
    S29,
    S30,
    S31,

    // VFP/NEON double-precision registers (D0-D31)
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
    D26,
    D27,
    D28,
    D29,
    D30,
    D31,

    // NEON quad-word registers (Q0-Q15)
    Q0,
    Q1,
    Q2,
    Q3,
    Q4,
    Q5,
    Q6,
    Q7,
    Q8,
    Q9,
    Q10,
    Q11,
    Q12,
    Q13,
    Q14,
    Q15,

    // VFP status registers
    FPSCR, // Floating-Point Status and Control Register
    FPEXC, // Floating-Point Exception Register
    FPSID, // Floating-Point System ID Register
}

impl Arm32Register {
    /// Get register size in bytes
    pub fn size(&self) -> usize {
        match self {
            // Core registers are 32-bit
            Self::R0
            | Self::R1
            | Self::R2
            | Self::R3
            | Self::R4
            | Self::R5
            | Self::R6
            | Self::R7
            | Self::R8
            | Self::R9
            | Self::R10
            | Self::R11
            | Self::R12
            | Self::R13
            | Self::R14
            | Self::R15
            | Self::SP
            | Self::LR
            | Self::PC
            | Self::CPSR
            | Self::SpsrFiq
            | Self::SpsrIrq
            | Self::SpsrSvc
            | Self::SpsrAbt
            | Self::SpsrUnd
            | Self::R8Fiq
            | Self::R9Fiq
            | Self::R10Fiq
            | Self::R11Fiq
            | Self::R12Fiq
            | Self::R13Fiq
            | Self::R14Fiq
            | Self::R13Irq
            | Self::R14Irq
            | Self::R13Svc
            | Self::R14Svc
            | Self::R13Abt
            | Self::R14Abt
            | Self::R13Und
            | Self::R14Und => 4,

            // Single-precision float registers (S0-S31)
            Self::S0
            | Self::S1
            | Self::S2
            | Self::S3
            | Self::S4
            | Self::S5
            | Self::S6
            | Self::S7
            | Self::S8
            | Self::S9
            | Self::S10
            | Self::S11
            | Self::S12
            | Self::S13
            | Self::S14
            | Self::S15
            | Self::S16
            | Self::S17
            | Self::S18
            | Self::S19
            | Self::S20
            | Self::S21
            | Self::S22
            | Self::S23
            | Self::S24
            | Self::S25
            | Self::S26
            | Self::S27
            | Self::S28
            | Self::S29
            | Self::S30
            | Self::S31 => 4,

            // Double-precision float registers (D0-D31)
            Self::D0
            | Self::D1
            | Self::D2
            | Self::D3
            | Self::D4
            | Self::D5
            | Self::D6
            | Self::D7
            | Self::D8
            | Self::D9
            | Self::D10
            | Self::D11
            | Self::D12
            | Self::D13
            | Self::D14
            | Self::D15
            | Self::D16
            | Self::D17
            | Self::D18
            | Self::D19
            | Self::D20
            | Self::D21
            | Self::D22
            | Self::D23
            | Self::D24
            | Self::D25
            | Self::D26
            | Self::D27
            | Self::D28
            | Self::D29
            | Self::D30
            | Self::D31 => 8,

            // Quad-word NEON registers (Q0-Q15)
            Self::Q0
            | Self::Q1
            | Self::Q2
            | Self::Q3
            | Self::Q4
            | Self::Q5
            | Self::Q6
            | Self::Q7
            | Self::Q8
            | Self::Q9
            | Self::Q10
            | Self::Q11
            | Self::Q12
            | Self::Q13
            | Self::Q14
            | Self::Q15 => 16,

            // VFP status registers
            Self::FPSCR | Self::FPEXC | Self::FPSID => 4,
        }
    }

    /// Check if this is a general-purpose register
    pub fn is_gpr(&self) -> bool {
        matches!(
            self,
            Self::R0
                | Self::R1
                | Self::R2
                | Self::R3
                | Self::R4
                | Self::R5
                | Self::R6
                | Self::R7
                | Self::R8
                | Self::R9
                | Self::R10
                | Self::R11
                | Self::R12
                | Self::R13
                | Self::R14
                | Self::R15
                | Self::SP
                | Self::LR
                | Self::PC
        )
    }

    /// Check if this is a floating-point/SIMD register
    pub fn is_fp(&self) -> bool {
        matches!(
            self,
            // Single-precision float registers (S0-S31)
            Self::S0 | Self::S1 | Self::S2 | Self::S3 | Self::S4 | Self::S5 | Self::S6 | Self::S7 |
            Self::S8 | Self::S9 | Self::S10 | Self::S11 | Self::S12 | Self::S13 | Self::S14 | Self::S15 |
            Self::S16 | Self::S17 | Self::S18 | Self::S19 | Self::S20 | Self::S21 | Self::S22 | Self::S23 |
            Self::S24 | Self::S25 | Self::S26 | Self::S27 | Self::S28 | Self::S29 | Self::S30 | Self::S31 |

            // Double-precision float registers (D0-D31)
            Self::D0 | Self::D1 | Self::D2 | Self::D3 | Self::D4 | Self::D5 | Self::D6 | Self::D7 |
            Self::D8 | Self::D9 | Self::D10 | Self::D11 | Self::D12 | Self::D13 | Self::D14 | Self::D15 |
            Self::D16 | Self::D17 | Self::D18 | Self::D19 | Self::D20 | Self::D21 | Self::D22 | Self::D23 |
            Self::D24 | Self::D25 | Self::D26 | Self::D27 | Self::D28 | Self::D29 | Self::D30 | Self::D31 |

            // Quad-word NEON registers (Q0-Q15)
            Self::Q0 | Self::Q1 | Self::Q2 | Self::Q3 | Self::Q4 | Self::Q5 | Self::Q6 | Self::Q7 |
            Self::Q8 | Self::Q9 | Self::Q10 | Self::Q11 | Self::Q12 | Self::Q13 | Self::Q14 | Self::Q15
        )
    }

    /// Check if this is a status register
    pub fn is_status(&self) -> bool {
        matches!(
            self,
            Self::CPSR
                | Self::SpsrFiq
                | Self::SpsrIrq
                | Self::SpsrSvc
                | Self::SpsrAbt
                | Self::SpsrUnd
                | Self::FPSCR
                | Self::FPEXC
                | Self::FPSID
        )
    }

    /// Get the core register number (0-15) if applicable
    pub fn core_number(&self) -> Option<u8> {
        match self {
            Self::R0 => Some(0),
            Self::R1 => Some(1),
            Self::R2 => Some(2),
            Self::R3 => Some(3),
            Self::R4 => Some(4),
            Self::R5 => Some(5),
            Self::R6 => Some(6),
            Self::R7 => Some(7),
            Self::R8 => Some(8),
            Self::R9 => Some(9),
            Self::R10 => Some(10),
            Self::R11 => Some(11),
            Self::R12 => Some(12),
            Self::R13 | Self::SP => Some(13),
            Self::R14 | Self::LR => Some(14),
            Self::R15 | Self::PC => Some(15),
            _ => None,
        }
    }
}

impl fmt::Display for Arm32Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Core registers
            Self::R0 => write!(f, "r0"),
            Self::R1 => write!(f, "r1"),
            Self::R2 => write!(f, "r2"),
            Self::R3 => write!(f, "r3"),
            Self::R4 => write!(f, "r4"),
            Self::R5 => write!(f, "r5"),
            Self::R6 => write!(f, "r6"),
            Self::R7 => write!(f, "r7"),
            Self::R8 => write!(f, "r8"),
            Self::R9 => write!(f, "r9"),
            Self::R10 => write!(f, "r10"),
            Self::R11 => write!(f, "r11"),
            Self::R12 => write!(f, "r12"),
            Self::R13 => write!(f, "r13"),
            Self::R14 => write!(f, "r14"),
            Self::R15 => write!(f, "r15"),

            // Special names
            Self::SP => write!(f, "sp"),
            Self::LR => write!(f, "lr"),
            Self::PC => write!(f, "pc"),

            // Status registers
            Self::CPSR => write!(f, "cpsr"),
            Self::SpsrFiq => write!(f, "spsr_fiq"),
            Self::SpsrIrq => write!(f, "spsr_irq"),
            Self::SpsrSvc => write!(f, "spsr_svc"),
            Self::SpsrAbt => write!(f, "spsr_abt"),
            Self::SpsrUnd => write!(f, "spsr_und"),

            // Banked registers
            Self::R8Fiq => write!(f, "r8_fiq"),
            Self::R9Fiq => write!(f, "r9_fiq"),
            Self::R10Fiq => write!(f, "r10_fiq"),
            Self::R11Fiq => write!(f, "r11_fiq"),
            Self::R12Fiq => write!(f, "r12_fiq"),
            Self::R13Fiq => write!(f, "r13_fiq"),
            Self::R14Fiq => write!(f, "r14_fiq"),
            Self::R13Irq => write!(f, "r13_irq"),
            Self::R14Irq => write!(f, "r14_irq"),
            Self::R13Svc => write!(f, "r13_svc"),
            Self::R14Svc => write!(f, "r14_svc"),
            Self::R13Abt => write!(f, "r13_abt"),
            Self::R14Abt => write!(f, "r14_abt"),
            Self::R13Und => write!(f, "r13_und"),
            Self::R14Und => write!(f, "r14_und"),

            // VFP/NEON registers - Single precision
            Self::S0 => write!(f, "s0"),
            Self::S1 => write!(f, "s1"),
            Self::S2 => write!(f, "s2"),
            Self::S3 => write!(f, "s3"),
            Self::S4 => write!(f, "s4"),
            Self::S5 => write!(f, "s5"),
            Self::S6 => write!(f, "s6"),
            Self::S7 => write!(f, "s7"),
            Self::S8 => write!(f, "s8"),
            Self::S9 => write!(f, "s9"),
            Self::S10 => write!(f, "s10"),
            Self::S11 => write!(f, "s11"),
            Self::S12 => write!(f, "s12"),
            Self::S13 => write!(f, "s13"),
            Self::S14 => write!(f, "s14"),
            Self::S15 => write!(f, "s15"),
            Self::S16 => write!(f, "s16"),
            Self::S17 => write!(f, "s17"),
            Self::S18 => write!(f, "s18"),
            Self::S19 => write!(f, "s19"),
            Self::S20 => write!(f, "s20"),
            Self::S21 => write!(f, "s21"),
            Self::S22 => write!(f, "s22"),
            Self::S23 => write!(f, "s23"),
            Self::S24 => write!(f, "s24"),
            Self::S25 => write!(f, "s25"),
            Self::S26 => write!(f, "s26"),
            Self::S27 => write!(f, "s27"),
            Self::S28 => write!(f, "s28"),
            Self::S29 => write!(f, "s29"),
            Self::S30 => write!(f, "s30"),
            Self::S31 => write!(f, "s31"),

            // VFP/NEON registers - Double precision
            Self::D0 => write!(f, "d0"),
            Self::D1 => write!(f, "d1"),
            Self::D2 => write!(f, "d2"),
            Self::D3 => write!(f, "d3"),
            Self::D4 => write!(f, "d4"),
            Self::D5 => write!(f, "d5"),
            Self::D6 => write!(f, "d6"),
            Self::D7 => write!(f, "d7"),
            Self::D8 => write!(f, "d8"),
            Self::D9 => write!(f, "d9"),
            Self::D10 => write!(f, "d10"),
            Self::D11 => write!(f, "d11"),
            Self::D12 => write!(f, "d12"),
            Self::D13 => write!(f, "d13"),
            Self::D14 => write!(f, "d14"),
            Self::D15 => write!(f, "d15"),
            Self::D16 => write!(f, "d16"),
            Self::D17 => write!(f, "d17"),
            Self::D18 => write!(f, "d18"),
            Self::D19 => write!(f, "d19"),
            Self::D20 => write!(f, "d20"),
            Self::D21 => write!(f, "d21"),
            Self::D22 => write!(f, "d22"),
            Self::D23 => write!(f, "d23"),
            Self::D24 => write!(f, "d24"),
            Self::D25 => write!(f, "d25"),
            Self::D26 => write!(f, "d26"),
            Self::D27 => write!(f, "d27"),
            Self::D28 => write!(f, "d28"),
            Self::D29 => write!(f, "d29"),
            Self::D30 => write!(f, "d30"),
            Self::D31 => write!(f, "d31"),

            // VFP/NEON registers - Quad word
            Self::Q0 => write!(f, "q0"),
            Self::Q1 => write!(f, "q1"),
            Self::Q2 => write!(f, "q2"),
            Self::Q3 => write!(f, "q3"),
            Self::Q4 => write!(f, "q4"),
            Self::Q5 => write!(f, "q5"),
            Self::Q6 => write!(f, "q6"),
            Self::Q7 => write!(f, "q7"),
            Self::Q8 => write!(f, "q8"),
            Self::Q9 => write!(f, "q9"),
            Self::Q10 => write!(f, "q10"),
            Self::Q11 => write!(f, "q11"),
            Self::Q12 => write!(f, "q12"),
            Self::Q13 => write!(f, "q13"),
            Self::Q14 => write!(f, "q14"),
            Self::Q15 => write!(f, "q15"),

            // VFP status registers
            Self::FPSCR => write!(f, "fpscr"),
            Self::FPEXC => write!(f, "fpexc"),
            Self::FPSID => write!(f, "fpsid"),
        }
    }
}

/// ARM32 condition flags in CPSR/SPSR
#[derive(Debug, Clone, Copy)]
pub struct ConditionFlags {
    /// Negative flag
    pub n: bool,
    /// Zero flag
    pub z: bool,
    /// Carry flag
    pub c: bool,
    /// Overflow flag
    pub v: bool,
}

impl ConditionFlags {
    /// Create new condition flags
    pub fn new(n: bool, z: bool, c: bool, v: bool) -> Self {
        Self { n, z, c, v }
    }

    /// Check if condition is satisfied
    pub fn check_condition(&self, cond: super::Condition) -> bool {
        use super::Condition;

        match cond {
            Condition::EQ => self.z,
            Condition::NE => !self.z,
            Condition::CS => self.c,
            Condition::CC => !self.c,
            Condition::MI => self.n,
            Condition::PL => !self.n,
            Condition::VS => self.v,
            Condition::VC => !self.v,
            Condition::HI => self.c && !self.z,
            Condition::LS => !self.c || self.z,
            Condition::GE => self.n == self.v,
            Condition::LT => self.n != self.v,
            Condition::GT => !self.z && (self.n == self.v),
            Condition::LE => self.z || (self.n != self.v),
            Condition::AL => true,
            Condition::NV => false,
        }
    }
}

// Static register definitions
// ARM32 has 16 general-purpose registers (r0-r15), each 32 bits
// We allocate them sequentially in the virtual machine's bit array
static R0: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r0", 0..32)).into());

static R1: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r1", 32..64)).into());

static R2: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r2", 64..96)).into());

static R3: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r3", 96..128)).into());

static R4: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r4", 128..160)).into());

static R5: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r5", 160..192)).into());

static R6: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r6", 192..224)).into());

static R7: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r7", 224..256)).into());

static R8: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r8", 256..288)).into());

static R9: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r9", 288..320)).into());

static R10: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r10", 320..352)).into());

static R11: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r11", 352..384)).into());

static R12: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("r12", 384..416)).into());

// r13 = sp (stack pointer)
static SP: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("sp", 416..448)).into());

// r14 = lr (link register)
static LR: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("lr", 448..480)).into());

// r15 = pc (program counter)
static PC: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("pc", 480..512)).into());

// CPSR (Current Program Status Register)
static CPSR: LazyLock<Aos<IrData>> =
    LazyLock::new(|| IrData::Register(Register::new("cpsr", 512..544)).into());

/// Convert string to ARM32 register as IrData
pub fn str_to_arm32_register(s: &str) -> Aos<IrData> {
    match s.to_lowercase().as_str() {
        // Core registers
        "r0" => R0.clone(),
        "r1" => R1.clone(),
        "r2" => R2.clone(),
        "r3" => R3.clone(),
        "r4" => R4.clone(),
        "r5" => R5.clone(),
        "r6" => R6.clone(),
        "r7" => R7.clone(),
        "r8" => R8.clone(),
        "r9" => R9.clone(),
        "r10" => R10.clone(),
        "r11" => R11.clone(),
        "r12" => R12.clone(),
        "r13" | "sp" => SP.clone(),
        "r14" | "lr" => LR.clone(),
        "r15" | "pc" => PC.clone(),

        // Status registers
        "cpsr" => CPSR.clone(),

        // Unknown register
        _ => IrData::Intrinsic(IrIntrinsic::Unknown).into(),
    }
}

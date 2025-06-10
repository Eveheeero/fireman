//! ARM32 register definitions

use crate::{DisassembleError, RegisterInner};

/// ARM32 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Arm32Register {
    // General purpose registers
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
    R13, // Stack Pointer
    R14, // Link Register
    R15, // Program Counter

    // Aliases
    SP, // Stack Pointer (R13)
    LR, // Link Register (R14)
    PC, // Program Counter (R15)

    // Status register
    CPSR,
    SPSR,

    // VFP/NEON registers
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
}

impl RegisterInner for Arm32Register {}

impl Arm32Register {
    /// Parse register from string
    pub fn parse(s: impl AsRef<str>) -> Result<Self, DisassembleError> {
        let s = s.as_ref().to_lowercase();

        match s.as_str() {
            // General purpose registers
            "r0" => Ok(Self::R0),
            "r1" => Ok(Self::R1),
            "r2" => Ok(Self::R2),
            "r3" => Ok(Self::R3),
            "r4" => Ok(Self::R4),
            "r5" => Ok(Self::R5),
            "r6" => Ok(Self::R6),
            "r7" => Ok(Self::R7),
            "r8" => Ok(Self::R8),
            "r9" => Ok(Self::R9),
            "r10" => Ok(Self::R10),
            "r11" | "fp" => Ok(Self::R11), // Frame pointer
            "r12" | "ip" => Ok(Self::R12), // Intra-procedure scratch
            "r13" | "sp" => Ok(Self::SP),
            "r14" | "lr" => Ok(Self::LR),
            "r15" | "pc" => Ok(Self::PC),

            // Status registers
            "cpsr" => Ok(Self::CPSR),
            "spsr" => Ok(Self::SPSR),

            // VFP single precision
            "s0" => Ok(Self::S0),
            "s1" => Ok(Self::S1),
            "s2" => Ok(Self::S2),
            "s3" => Ok(Self::S3),
            "s4" => Ok(Self::S4),
            "s5" => Ok(Self::S5),
            "s6" => Ok(Self::S6),
            "s7" => Ok(Self::S7),
            "s8" => Ok(Self::S8),
            "s9" => Ok(Self::S9),
            "s10" => Ok(Self::S10),
            "s11" => Ok(Self::S11),
            "s12" => Ok(Self::S12),
            "s13" => Ok(Self::S13),
            "s14" => Ok(Self::S14),
            "s15" => Ok(Self::S15),
            "s16" => Ok(Self::S16),
            "s17" => Ok(Self::S17),
            "s18" => Ok(Self::S18),
            "s19" => Ok(Self::S19),
            "s20" => Ok(Self::S20),
            "s21" => Ok(Self::S21),
            "s22" => Ok(Self::S22),
            "s23" => Ok(Self::S23),
            "s24" => Ok(Self::S24),
            "s25" => Ok(Self::S25),
            "s26" => Ok(Self::S26),
            "s27" => Ok(Self::S27),
            "s28" => Ok(Self::S28),
            "s29" => Ok(Self::S29),
            "s30" => Ok(Self::S30),
            "s31" => Ok(Self::S31),

            // VFP double precision
            "d0" => Ok(Self::D0),
            "d1" => Ok(Self::D1),
            "d2" => Ok(Self::D2),
            "d3" => Ok(Self::D3),
            "d4" => Ok(Self::D4),
            "d5" => Ok(Self::D5),
            "d6" => Ok(Self::D6),
            "d7" => Ok(Self::D7),
            "d8" => Ok(Self::D8),
            "d9" => Ok(Self::D9),
            "d10" => Ok(Self::D10),
            "d11" => Ok(Self::D11),
            "d12" => Ok(Self::D12),
            "d13" => Ok(Self::D13),
            "d14" => Ok(Self::D14),
            "d15" => Ok(Self::D15),
            "d16" => Ok(Self::D16),
            "d17" => Ok(Self::D17),
            "d18" => Ok(Self::D18),
            "d19" => Ok(Self::D19),
            "d20" => Ok(Self::D20),
            "d21" => Ok(Self::D21),
            "d22" => Ok(Self::D22),
            "d23" => Ok(Self::D23),
            "d24" => Ok(Self::D24),
            "d25" => Ok(Self::D25),
            "d26" => Ok(Self::D26),
            "d27" => Ok(Self::D27),
            "d28" => Ok(Self::D28),
            "d29" => Ok(Self::D29),
            "d30" => Ok(Self::D30),
            "d31" => Ok(Self::D31),

            // NEON quadword
            "q0" => Ok(Self::Q0),
            "q1" => Ok(Self::Q1),
            "q2" => Ok(Self::Q2),
            "q3" => Ok(Self::Q3),
            "q4" => Ok(Self::Q4),
            "q5" => Ok(Self::Q5),
            "q6" => Ok(Self::Q6),
            "q7" => Ok(Self::Q7),
            "q8" => Ok(Self::Q8),
            "q9" => Ok(Self::Q9),
            "q10" => Ok(Self::Q10),
            "q11" => Ok(Self::Q11),
            "q12" => Ok(Self::Q12),
            "q13" => Ok(Self::Q13),
            "q14" => Ok(Self::Q14),
            "q15" => Ok(Self::Q15),

            _ => Err(DisassembleError::UnknownRegister),
        }
    }

    /// Get register size in bits
    pub fn size(&self) -> u32 {
        match self {
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
            | Self::SPSR => 32,

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
            | Self::S31 => 32,

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
            | Self::D31 => 64,

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
            | Self::Q15 => 128,
        }
    }

    /// Check if this is a general purpose register
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

    /// Check if this is a floating point register
    pub fn is_fp(&self) -> bool {
        matches!(
            self,
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
                | Self::S31
                | Self::D0
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
                | Self::D31
                | Self::Q0
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
                | Self::Q15
        )
    }
}

impl std::fmt::Display for Arm32Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // General purpose
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

            // Aliases
            Self::SP => write!(f, "sp"),
            Self::LR => write!(f, "lr"),
            Self::PC => write!(f, "pc"),

            // Status
            Self::CPSR => write!(f, "cpsr"),
            Self::SPSR => write!(f, "spsr"),

            // VFP/NEON
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
        }
    }
}

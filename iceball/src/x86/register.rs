//! x86 (32-bit) register definitions

use crate::RegisterInner;

/// x86 (32-bit) registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum X86Register {
    // 32-bit general purpose registers
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,

    // 16-bit general purpose registers
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    SP,
    BP,

    // 8-bit registers
    AL,
    AH,
    BL,
    BH,
    CL,
    CH,
    DL,
    DH,

    // Instruction pointer
    EIP,

    // Flags
    EFLAGS,

    // Segment registers
    CS,
    DS,
    ES,
    FS,
    GS,
    SS,

    // Control registers
    CR0,
    CR1,
    CR2,
    CR3,
    CR4,

    // Debug registers
    DR0,
    DR1,
    DR2,
    DR3,
    DR4,
    DR5,
    DR6,
    DR7,

    // x87 FPU registers
    ST0,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    ST7,

    // MMX registers (64-bit)
    MM0,
    MM1,
    MM2,
    MM3,
    MM4,
    MM5,
    MM6,
    MM7,

    // SSE registers (128-bit)
    XMM0,
    XMM1,
    XMM2,
    XMM3,
    XMM4,
    XMM5,
    XMM6,
    XMM7,
}

impl RegisterInner for X86Register {}

impl X86Register {
    /// Returns the bit width of the register
    pub fn width(&self) -> u8 {
        use X86Register::*;
        match self {
            // 32-bit registers
            EAX | EBX | ECX | EDX | ESI | EDI | ESP | EBP | EIP | EFLAGS => 32,

            // 16-bit registers
            AX | BX | CX | DX | SI | DI | SP | BP | CS | DS | ES | FS | GS | SS => 16,

            // 8-bit registers
            AL | AH | BL | BH | CL | CH | DL | DH => 8,

            // Control registers (32-bit in 32-bit mode)
            CR0 | CR1 | CR2 | CR3 | CR4 => 32,

            // Debug registers (32-bit in 32-bit mode)
            DR0 | DR1 | DR2 | DR3 | DR4 | DR5 | DR6 | DR7 => 32,

            // FPU registers (80-bit)
            ST0 | ST1 | ST2 | ST3 | ST4 | ST5 | ST6 | ST7 => 80,

            // MMX registers (64-bit)
            MM0 | MM1 | MM2 | MM3 | MM4 | MM5 | MM6 | MM7 => 64,

            // SSE registers (128-bit)
            XMM0 | XMM1 | XMM2 | XMM3 | XMM4 | XMM5 | XMM6 | XMM7 => 128,
        }
    }

    /// Returns true if this is a general-purpose register
    pub fn is_gpr(&self) -> bool {
        use X86Register::*;
        matches!(
            self,
            EAX | EBX
                | ECX
                | EDX
                | ESI
                | EDI
                | ESP
                | EBP
                | AX
                | BX
                | CX
                | DX
                | SI
                | DI
                | SP
                | BP
                | AL
                | AH
                | BL
                | BH
                | CL
                | CH
                | DL
                | DH
        )
    }

    /// Returns true if this is a segment register
    pub fn is_segment(&self) -> bool {
        use X86Register::*;
        matches!(self, CS | DS | ES | FS | GS | SS)
    }

    /// Returns true if this is an FPU register
    pub fn is_fpu(&self) -> bool {
        use X86Register::*;
        matches!(self, ST0 | ST1 | ST2 | ST3 | ST4 | ST5 | ST6 | ST7)
    }

    /// Returns true if this is an SSE register
    pub fn is_sse(&self) -> bool {
        use X86Register::*;
        matches!(self, XMM0 | XMM1 | XMM2 | XMM3 | XMM4 | XMM5 | XMM6 | XMM7)
    }

    /// Returns true if this is an MMX register
    pub fn is_mmx(&self) -> bool {
        use X86Register::*;
        matches!(self, MM0 | MM1 | MM2 | MM3 | MM4 | MM5 | MM6 | MM7)
    }
}

impl std::fmt::Display for X86Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use X86Register::*;
        let name = match self {
            // General purpose
            EAX => "eax",
            EBX => "ebx",
            ECX => "ecx",
            EDX => "edx",
            ESI => "esi",
            EDI => "edi",
            ESP => "esp",
            EBP => "ebp",
            AX => "ax",
            BX => "bx",
            CX => "cx",
            DX => "dx",
            SI => "si",
            DI => "di",
            SP => "sp",
            BP => "bp",
            AL => "al",
            AH => "ah",
            BL => "bl",
            BH => "bh",
            CL => "cl",
            CH => "ch",
            DL => "dl",
            DH => "dh",

            // Special
            EIP => "eip",
            EFLAGS => "eflags",

            // Segment
            CS => "cs",
            DS => "ds",
            ES => "es",
            FS => "fs",
            GS => "gs",
            SS => "ss",

            // Control
            CR0 => "cr0",
            CR1 => "cr1",
            CR2 => "cr2",
            CR3 => "cr3",
            CR4 => "cr4",

            // Debug
            DR0 => "dr0",
            DR1 => "dr1",
            DR2 => "dr2",
            DR3 => "dr3",
            DR4 => "dr4",
            DR5 => "dr5",
            DR6 => "dr6",
            DR7 => "dr7",

            // FPU
            ST0 => "st0",
            ST1 => "st1",
            ST2 => "st2",
            ST3 => "st3",
            ST4 => "st4",
            ST5 => "st5",
            ST6 => "st6",
            ST7 => "st7",

            // MMX
            MM0 => "mm0",
            MM1 => "mm1",
            MM2 => "mm2",
            MM3 => "mm3",
            MM4 => "mm4",
            MM5 => "mm5",
            MM6 => "mm6",
            MM7 => "mm7",

            // SSE
            XMM0 => "xmm0",
            XMM1 => "xmm1",
            XMM2 => "xmm2",
            XMM3 => "xmm3",
            XMM4 => "xmm4",
            XMM5 => "xmm5",
            XMM6 => "xmm6",
            XMM7 => "xmm7",
        };
        write!(f, "{}", name)
    }
}

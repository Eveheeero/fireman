//! x86 (32-bit) register definitions and utilities

use crate::ir::data::IrData;
use crate::utils::Aos;
use std::sync::LazyLock;

/// x86 32-bit registers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum X86Register {
    // General-purpose registers (32-bit)
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,

    // 16-bit registers
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

    // Segment registers
    CS,
    DS,
    ES,
    FS,
    GS,
    SS,

    // Control registers
    CR0,
    CR2,
    CR3,
    CR4,

    // Debug registers
    DR0,
    DR1,
    DR2,
    DR3,
    DR6,
    DR7,

    // FPU registers
    ST0,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    ST7,

    // MMX registers (alias FPU)
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

    // Special registers
    EIP,    // Instruction pointer
    EFLAGS, // Flags register
}

impl X86Register {
    /// Get the bit size of this register
    pub fn bit_size(&self) -> u8 {
        use X86Register::*;
        match self {
            // 32-bit registers
            EAX | EBX | ECX | EDX | ESI | EDI | ESP | EBP => 32,
            EIP | EFLAGS => 32,
            CR0 | CR2 | CR3 | CR4 => 32,
            DR0 | DR1 | DR2 | DR3 | DR6 | DR7 => 32,

            // 16-bit registers
            AX | BX | CX | DX | SI | DI | SP | BP => 16,
            CS | DS | ES | FS | GS | SS => 16,

            // 8-bit registers
            AL | AH | BL | BH | CL | CH | DL | DH => 8,

            // 80-bit FPU registers
            ST0 | ST1 | ST2 | ST3 | ST4 | ST5 | ST6 | ST7 => 80,

            // 64-bit MMX registers
            MM0 | MM1 | MM2 | MM3 | MM4 | MM5 | MM6 | MM7 => 64,

            // 128-bit SSE registers
            XMM0 | XMM1 | XMM2 | XMM3 | XMM4 | XMM5 | XMM6 | XMM7 => 128,
        }
    }

    /// Get the name of this register
    pub fn name(&self) -> &'static str {
        use X86Register::*;
        match self {
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

            CS => "cs",
            DS => "ds",
            ES => "es",
            FS => "fs",
            GS => "gs",
            SS => "ss",

            CR0 => "cr0",
            CR2 => "cr2",
            CR3 => "cr3",
            CR4 => "cr4",
            DR0 => "dr0",
            DR1 => "dr1",
            DR2 => "dr2",
            DR3 => "dr3",
            DR6 => "dr6",
            DR7 => "dr7",

            ST0 => "st0",
            ST1 => "st1",
            ST2 => "st2",
            ST3 => "st3",
            ST4 => "st4",
            ST5 => "st5",
            ST6 => "st6",
            ST7 => "st7",

            MM0 => "mm0",
            MM1 => "mm1",
            MM2 => "mm2",
            MM3 => "mm3",
            MM4 => "mm4",
            MM5 => "mm5",
            MM6 => "mm6",
            MM7 => "mm7",

            XMM0 => "xmm0",
            XMM1 => "xmm1",
            XMM2 => "xmm2",
            XMM3 => "xmm3",
            XMM4 => "xmm4",
            XMM5 => "xmm5",
            XMM6 => "xmm6",
            XMM7 => "xmm7",

            EIP => "eip",
            EFLAGS => "eflags",
        }
    }

    /// Check if this is a general-purpose register
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

    /// Check if this is an FPU register
    pub fn is_fpu(&self) -> bool {
        use X86Register::*;
        matches!(self, ST0 | ST1 | ST2 | ST3 | ST4 | ST5 | ST6 | ST7)
    }

    /// Check if this is an SSE register
    pub fn is_sse(&self) -> bool {
        use X86Register::*;
        matches!(self, XMM0 | XMM1 | XMM2 | XMM3 | XMM4 | XMM5 | XMM6 | XMM7)
    }
}

// Static register references for IR generation
macro_rules! define_register {
    ($name:ident, $reg:expr) => {
        pub static $name: LazyLock<Aos<IrData>> = LazyLock::new(|| {
            Aos::new_static(IrData::Register(crate::ir::Register::new(
                $reg.name(),
                0..32,
            )))
        });
    };
}

// Define commonly used registers
define_register!(EAX, X86Register::EAX);
define_register!(EBX, X86Register::EBX);
define_register!(ECX, X86Register::ECX);
define_register!(EDX, X86Register::EDX);
define_register!(ESI, X86Register::ESI);
define_register!(EDI, X86Register::EDI);
define_register!(ESP, X86Register::ESP);
define_register!(EBP, X86Register::EBP);
define_register!(EIP, X86Register::EIP);
define_register!(EFLAGS, X86Register::EFLAGS);

// 16-bit registers
define_register!(AX, X86Register::AX);
define_register!(BX, X86Register::BX);
define_register!(CX, X86Register::CX);
define_register!(DX, X86Register::DX);

// 8-bit registers
define_register!(AL, X86Register::AL);
define_register!(AH, X86Register::AH);
define_register!(BL, X86Register::BL);
define_register!(BH, X86Register::BH);
define_register!(CL, X86Register::CL);
define_register!(CH, X86Register::CH);
define_register!(DL, X86Register::DL);
define_register!(DH, X86Register::DH);

/// x86 condition codes (same as x86_64)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    O,   // Overflow
    NO,  // No overflow
    B,   // Below (CF=1)
    NB,  // Not below (CF=0)
    E,   // Equal (ZF=1)
    NE,  // Not equal (ZF=0)
    BE,  // Below or equal
    NBE, // Not below or equal
    S,   // Sign (SF=1)
    NS,  // No sign (SF=0)
    P,   // Parity (PF=1)
    NP,  // No parity (PF=0)
    L,   // Less (SF!=OF)
    NL,  // Not less (SF=OF)
    LE,  // Less or equal
    NLE, // Not less or equal
}

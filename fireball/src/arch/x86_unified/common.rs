//! Common definitions and utilities shared between x86 and x86_64

use crate::prelude::*;

/// x86 condition codes (shared between 32 and 64-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86Condition {
    /// Overflow (OF=1)
    Overflow,
    /// No overflow (OF=0)
    NoOverflow,
    /// Below/Carry (CF=1)
    Below,
    /// Above or equal/No carry (CF=0)
    AboveEqual,
    /// Equal/Zero (ZF=1)
    Equal,
    /// Not equal/Not zero (ZF=0)
    NotEqual,
    /// Below or equal (CF=1 or ZF=1)
    BelowEqual,
    /// Above (CF=0 and ZF=0)
    Above,
    /// Sign (SF=1)
    Sign,
    /// Not sign (SF=0)
    NotSign,
    /// Parity/Parity even (PF=1)
    Parity,
    /// Not parity/Parity odd (PF=0)
    NotParity,
    /// Less (SF≠OF)
    Less,
    /// Greater or equal (SF=OF)
    GreaterEqual,
    /// Less or equal (ZF=1 or SF≠OF)
    LessEqual,
    /// Greater (ZF=0 and SF=OF)
    Greater,
}

impl X86Condition {
    /// Parse condition from instruction suffix
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        match suffix {
            "o" => Some(Self::Overflow),
            "no" => Some(Self::NoOverflow),
            "b" | "c" | "nae" => Some(Self::Below),
            "ae" | "nb" | "nc" => Some(Self::AboveEqual),
            "e" | "z" => Some(Self::Equal),
            "ne" | "nz" => Some(Self::NotEqual),
            "be" | "na" => Some(Self::BelowEqual),
            "a" | "nbe" => Some(Self::Above),
            "s" => Some(Self::Sign),
            "ns" => Some(Self::NotSign),
            "p" | "pe" => Some(Self::Parity),
            "np" | "po" => Some(Self::NotParity),
            "l" | "nge" => Some(Self::Less),
            "ge" | "nl" => Some(Self::GreaterEqual),
            "le" | "ng" => Some(Self::LessEqual),
            "g" | "nle" => Some(Self::Greater),
            _ => None,
        }
    }

    /// Get the inverted condition
    pub fn invert(self) -> Self {
        match self {
            Self::Overflow => Self::NoOverflow,
            Self::NoOverflow => Self::Overflow,
            Self::Below => Self::AboveEqual,
            Self::AboveEqual => Self::Below,
            Self::Equal => Self::NotEqual,
            Self::NotEqual => Self::Equal,
            Self::BelowEqual => Self::Above,
            Self::Above => Self::BelowEqual,
            Self::Sign => Self::NotSign,
            Self::NotSign => Self::Sign,
            Self::Parity => Self::NotParity,
            Self::NotParity => Self::Parity,
            Self::Less => Self::GreaterEqual,
            Self::GreaterEqual => Self::Less,
            Self::LessEqual => Self::Greater,
            Self::Greater => Self::LessEqual,
        }
    }
}

/// Common x86 instruction categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum X86InstructionCategory {
    /// Data movement (MOV, MOVZX, MOVSX, etc.)
    DataMovement,
    /// Arithmetic (ADD, SUB, MUL, DIV, etc.)
    Arithmetic,
    /// Logical (AND, OR, XOR, NOT, etc.)
    Logical,
    /// Shift and rotate (SHL, SHR, ROL, ROR, etc.)
    ShiftRotate,
    /// Bit manipulation (BT, BTS, BTR, BTC, etc.)
    BitManipulation,
    /// Control transfer (JMP, CALL, RET, Jcc)
    ControlTransfer,
    /// String operations (MOVS, STOS, CMPS, etc.)
    String,
    /// Flag control (STC, CLC, CMC, etc.)
    FlagControl,
    /// Stack operations (PUSH, POP)
    Stack,
    /// Comparison (CMP, TEST)
    Comparison,
    /// Conversion (CBW, CWD, CDQ, etc.)
    Conversion,
    /// System (CPUID, RDTSC, etc.)
    System,
    /// SIMD (SSE, AVX, etc.)
    SIMD,
    /// x87 FPU
    FPU,
}

/// Determine instruction category from mnemonic
pub fn categorize_instruction(mnemonic: &str) -> X86InstructionCategory {
    // Remove condition suffix for jumps
    let base = if mnemonic.starts_with('j') && mnemonic.len() > 1 {
        if X86Condition::from_suffix(&mnemonic[1..]).is_some() {
            "jcc"
        } else {
            mnemonic
        }
    } else if mnemonic.starts_with("cmov") {
        "cmov"
    } else if mnemonic.starts_with("set") {
        "setcc"
    } else {
        mnemonic
    };

    match base {
        // Data movement
        "mov" | "movzx" | "movsx" | "movsxd" | "movabs" | "lea" | "xchg" | "bswap" => {
            X86InstructionCategory::DataMovement
        }

        // Arithmetic
        "add" | "sub" | "mul" | "imul" | "div" | "idiv" | "inc" | "dec" | "neg" | "adc" | "sbb" => {
            X86InstructionCategory::Arithmetic
        }

        // Logical
        "and" | "or" | "xor" | "not" | "test" => X86InstructionCategory::Logical,

        // Shift and rotate
        "shl" | "shr" | "sal" | "sar" | "rol" | "ror" | "rcl" | "rcr" | "shld" | "shrd" => {
            X86InstructionCategory::ShiftRotate
        }

        // Bit manipulation
        "bt" | "bts" | "btr" | "btc" | "bsf" | "bsr" | "popcnt" | "lzcnt" | "tzcnt" => {
            X86InstructionCategory::BitManipulation
        }

        // Control transfer
        "jmp" | "jcc" | "call" | "ret" | "loop" | "loope" | "loopne" | "jcxz" | "jecxz"
        | "jrcxz" => X86InstructionCategory::ControlTransfer,

        // String operations
        "movs" | "movsb" | "movsw" | "movsd" | "movsq" | "stos" | "stosb" | "stosw" | "stosd"
        | "stosq" | "lods" | "lodsb" | "lodsw" | "lodsd" | "lodsq" | "cmps" | "cmpsb" | "cmpsw"
        | "cmpsd" | "cmpsq" | "scas" | "scasb" | "scasw" | "scasd" | "scasq" => {
            X86InstructionCategory::String
        }

        // Flag control
        "stc" | "clc" | "cmc" | "std" | "cld" | "sti" | "cli" | "sahf" | "lahf" => {
            X86InstructionCategory::FlagControl
        }

        // Stack operations
        "push" | "pop" | "pushf" | "popf" | "pushfd" | "popfd" | "pushfq" | "popfq" | "pusha"
        | "popa" | "pushad" | "popad" | "enter" | "leave" => X86InstructionCategory::Stack,

        // Comparison
        "cmp" | "cmov" | "setcc" => X86InstructionCategory::Comparison,

        // Conversion
        "cbw" | "cwd" | "cdq" | "cqo" | "cwde" | "cdqe" => X86InstructionCategory::Conversion,

        // System
        "cpuid" | "rdtsc" | "rdtscp" | "rdmsr" | "wrmsr" | "rdpmc" => {
            X86InstructionCategory::System
        }

        // SIMD (simplified - there are many more)
        inst if inst.starts_with("movap")
            || inst.starts_with("movup")
            || inst.starts_with("add") && inst.contains('p')
            || inst.starts_with("sub") && inst.contains('p')
            || inst.starts_with("mul") && inst.contains('p')
            || inst.starts_with("xmm")
            || inst.starts_with("ymm")
            || inst.starts_with("zmm") =>
        {
            X86InstructionCategory::SIMD
        }

        // FPU
        inst if inst.starts_with('f') && !inst.starts_with("fence") => X86InstructionCategory::FPU,

        // Default
        _ => X86InstructionCategory::DataMovement,
    }
}

/// Common operand types for x86
#[derive(Debug, Clone, PartialEq)]
pub enum X86Operand {
    /// Register operand
    Register(crate::arch::x86_unified::register::X86Register),
    /// Immediate value
    Immediate(i64),
    /// Memory operand
    Memory {
        base: Option<crate::arch::x86_unified::register::X86Register>,
        index: Option<crate::arch::x86_unified::register::X86Register>,
        scale: u8,
        displacement: i64,
        segment: Option<crate::arch::x86_unified::register::X86Register>,
    },
    /// Relative offset (for jumps/calls)
    Relative(i64),
}

/// Common flags affected by instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X86Flags {
    pub carry: bool,
    pub parity: bool,
    pub auxiliary: bool,
    pub zero: bool,
    pub sign: bool,
    pub overflow: bool,
    pub direction: bool,
    pub interrupt: bool,
    pub trap: bool,
}

impl Default for X86Flags {
    fn default() -> Self {
        Self {
            carry: false,
            parity: false,
            auxiliary: false,
            zero: false,
            sign: false,
            overflow: false,
            direction: false,
            interrupt: true, // Usually enabled
            trap: false,
        }
    }
}

/// Get flags affected by an instruction
pub fn get_affected_flags(mnemonic: &str) -> X86Flags {
    match categorize_instruction(mnemonic) {
        X86InstructionCategory::Arithmetic => X86Flags {
            carry: true,
            parity: true,
            auxiliary: true,
            zero: true,
            sign: true,
            overflow: true,
            ..Default::default()
        },
        X86InstructionCategory::Logical => X86Flags {
            carry: true, // Cleared
            parity: true,
            zero: true,
            sign: true,
            overflow: true, // Cleared
            ..Default::default()
        },
        X86InstructionCategory::Comparison => X86Flags {
            carry: true,
            parity: true,
            auxiliary: true,
            zero: true,
            sign: true,
            overflow: true,
            ..Default::default()
        },
        _ => Default::default(),
    }
}

/// Shared instruction suffix parsing
pub fn parse_instruction_suffix(mnemonic: &str) -> (&str, Option<&str>) {
    // Common suffixes: b (byte), w (word), d (dword), q (qword)
    if let Some(base) = mnemonic.strip_suffix('b') {
        (base, Some("b"))
    } else if let Some(base) = mnemonic.strip_suffix('w') {
        (base, Some("w"))
    } else if let Some(base) = mnemonic.strip_suffix('d') {
        (base, Some("d"))
    } else if let Some(base) = mnemonic.strip_suffix('q') {
        (base, Some("q"))
    } else {
        (mnemonic, None)
    }
}

/// Get operand size from suffix
pub fn suffix_to_size(suffix: Option<&str>) -> Option<u8> {
    match suffix {
        Some("b") => Some(1),
        Some("w") => Some(2),
        Some("d") | Some("l") => Some(4),
        Some("q") => Some(8),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_parsing() {
        assert_eq!(X86Condition::from_suffix("e"), Some(X86Condition::Equal));
        assert_eq!(
            X86Condition::from_suffix("ne"),
            Some(X86Condition::NotEqual)
        );
        assert_eq!(X86Condition::from_suffix("l"), Some(X86Condition::Less));
        assert_eq!(
            X86Condition::from_suffix("ge"),
            Some(X86Condition::GreaterEqual)
        );
    }

    #[test]
    fn test_condition_inversion() {
        assert_eq!(X86Condition::Equal.invert(), X86Condition::NotEqual);
        assert_eq!(X86Condition::Less.invert(), X86Condition::GreaterEqual);
        assert_eq!(X86Condition::Below.invert(), X86Condition::AboveEqual);
    }

    #[test]
    fn test_instruction_categorization() {
        assert_eq!(
            categorize_instruction("mov"),
            X86InstructionCategory::DataMovement
        );
        assert_eq!(
            categorize_instruction("add"),
            X86InstructionCategory::Arithmetic
        );
        assert_eq!(
            categorize_instruction("jmp"),
            X86InstructionCategory::ControlTransfer
        );
        assert_eq!(
            categorize_instruction("je"),
            X86InstructionCategory::ControlTransfer
        );
        assert_eq!(
            categorize_instruction("push"),
            X86InstructionCategory::Stack
        );
    }

    #[test]
    fn test_suffix_parsing() {
        assert_eq!(parse_instruction_suffix("movb"), ("mov", Some("b")));
        assert_eq!(parse_instruction_suffix("addq"), ("add", Some("q")));
        assert_eq!(parse_instruction_suffix("jmp"), ("jmp", None));
    }
}

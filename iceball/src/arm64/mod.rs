//! ARM64 (AArch64) disassembly support

pub mod register;
pub mod statement;

use crate::{Argument, DisassembleError};
use std::fmt;

/// Parse ARM64 instruction argument
pub fn parse_argument(op: impl AsRef<str>) -> Result<Argument, DisassembleError> {
    let op = op.as_ref().trim();

    // Check for register
    if let Ok(reg) = Arm64Register::parse(op) {
        return Ok(Argument::Register(crate::Register::Arm64(reg)));
    }

    // Check for immediate value
    if let Some(imm_str) = op.strip_prefix('#') {
        if let Ok(value) = parse_immediate(imm_str) {
            return Ok(Argument::Constant(value));
        }
    }

    // Check for memory addressing
    if op.starts_with('[') && op.ends_with(']') {
        let inner = &op[1..op.len() - 1];
        return parse_memory_operand(inner);
    }

    // Try parsing as plain constant (without #)
    if let Ok(value) = parse_immediate(op) {
        return Ok(Argument::Constant(value));
    }

    Err(DisassembleError::Unknown)
}

/// Parse immediate value (hex or decimal)
fn parse_immediate(s: &str) -> Result<u64, DisassembleError> {
    let s = s.trim();

    // Handle hex values
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).map_err(|_| DisassembleError::Unknown)
    } else {
        // Try decimal
        s.parse().map_err(|_| DisassembleError::Unknown)
    }
}

/// Parse memory operand
fn parse_memory_operand(inner: &str) -> Result<Argument, DisassembleError> {
    // ARM64 memory addressing can be complex:
    // [Xn]
    // [Xn, #imm]
    // [Xn, #imm]!  (pre-indexed)
    // [Xn], #imm   (post-indexed)
    // [Xn, Xm]
    // [Xn, Xm, LSL #imm]
    // etc.

    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

    if parts.is_empty() {
        return Err(DisassembleError::Unknown);
    }

    let mut index: Option<crate::Register> = None;
    let mut scale: u8 = 1;
    let mut displacement: i64 = 0;

    // Parse base register
    let base_reg = Arm64Register::parse(parts[0])?;
    let base = Some(crate::Register::Arm64(base_reg));

    // Parse additional parts
    if parts.len() > 1 {
        let part = parts[1];

        // Check for immediate offset
        if let Some(imm_str) = part.strip_prefix('#') {
            if let Ok(value) = parse_immediate(imm_str) {
                displacement = value as i64;
            }
        }
        // Check for register with optional shift
        else if let Ok(index_reg) =
            Arm64Register::parse(part.split_whitespace().next().unwrap_or(part))
        {
            index = Some(crate::Register::Arm64(index_reg));

            // Check for shift operations (LSL, etc.)
            if parts.len() > 2 {
                let shift_part = parts[2];
                if shift_part.starts_with("LSL") || shift_part.starts_with("lsl") {
                    if let Some(shift_str) = shift_part.split_whitespace().nth(1) {
                        if let Some(shift_val) = shift_str.strip_prefix('#') {
                            if let Ok(shift) = shift_val.parse::<u8>() {
                                scale = 1 << shift;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(Argument::Memory(crate::Memory {
        base,
        index,
        scale,
        displacement,
        size: None,
    }))
}

pub use register::Arm64Register;
pub use statement::Arm64Statement;

impl fmt::Display for Arm64Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // 64-bit general purpose registers
            Arm64Register::X0 => write!(f, "x0"),
            Arm64Register::X1 => write!(f, "x1"),
            Arm64Register::X2 => write!(f, "x2"),
            Arm64Register::X3 => write!(f, "x3"),
            Arm64Register::X4 => write!(f, "x4"),
            Arm64Register::X5 => write!(f, "x5"),
            Arm64Register::X6 => write!(f, "x6"),
            Arm64Register::X7 => write!(f, "x7"),
            Arm64Register::X8 => write!(f, "x8"),
            Arm64Register::X9 => write!(f, "x9"),
            Arm64Register::X10 => write!(f, "x10"),
            Arm64Register::X11 => write!(f, "x11"),
            Arm64Register::X12 => write!(f, "x12"),
            Arm64Register::X13 => write!(f, "x13"),
            Arm64Register::X14 => write!(f, "x14"),
            Arm64Register::X15 => write!(f, "x15"),
            Arm64Register::X16 => write!(f, "x16"),
            Arm64Register::X17 => write!(f, "x17"),
            Arm64Register::X18 => write!(f, "x18"),
            Arm64Register::X19 => write!(f, "x19"),
            Arm64Register::X20 => write!(f, "x20"),
            Arm64Register::X21 => write!(f, "x21"),
            Arm64Register::X22 => write!(f, "x22"),
            Arm64Register::X23 => write!(f, "x23"),
            Arm64Register::X24 => write!(f, "x24"),
            Arm64Register::X25 => write!(f, "x25"),
            Arm64Register::X26 => write!(f, "x26"),
            Arm64Register::X27 => write!(f, "x27"),
            Arm64Register::X28 => write!(f, "x28"),
            Arm64Register::X29 => write!(f, "x29"),
            Arm64Register::X30 => write!(f, "x30"),
            Arm64Register::XZR => write!(f, "xzr"),
            Arm64Register::SP => write!(f, "sp"),

            // 32-bit general purpose registers
            Arm64Register::W0 => write!(f, "w0"),
            Arm64Register::W1 => write!(f, "w1"),
            Arm64Register::W2 => write!(f, "w2"),
            Arm64Register::W3 => write!(f, "w3"),
            Arm64Register::W4 => write!(f, "w4"),
            Arm64Register::W5 => write!(f, "w5"),
            Arm64Register::W6 => write!(f, "w6"),
            Arm64Register::W7 => write!(f, "w7"),
            Arm64Register::W8 => write!(f, "w8"),
            Arm64Register::W9 => write!(f, "w9"),
            Arm64Register::W10 => write!(f, "w10"),
            Arm64Register::W11 => write!(f, "w11"),
            Arm64Register::W12 => write!(f, "w12"),
            Arm64Register::W13 => write!(f, "w13"),
            Arm64Register::W14 => write!(f, "w14"),
            Arm64Register::W15 => write!(f, "w15"),
            Arm64Register::W16 => write!(f, "w16"),
            Arm64Register::W17 => write!(f, "w17"),
            Arm64Register::W18 => write!(f, "w18"),
            Arm64Register::W19 => write!(f, "w19"),
            Arm64Register::W20 => write!(f, "w20"),
            Arm64Register::W21 => write!(f, "w21"),
            Arm64Register::W22 => write!(f, "w22"),
            Arm64Register::W23 => write!(f, "w23"),
            Arm64Register::W24 => write!(f, "w24"),
            Arm64Register::W25 => write!(f, "w25"),
            Arm64Register::W26 => write!(f, "w26"),
            Arm64Register::W27 => write!(f, "w27"),
            Arm64Register::W28 => write!(f, "w28"),
            Arm64Register::W29 => write!(f, "w29"),
            Arm64Register::W30 => write!(f, "w30"),
            Arm64Register::WZR => write!(f, "wzr"),
            Arm64Register::WSP => write!(f, "wsp"),

            // Vector registers (128-bit)
            Arm64Register::V0 => write!(f, "v0"),
            Arm64Register::V1 => write!(f, "v1"),
            Arm64Register::V2 => write!(f, "v2"),
            Arm64Register::V3 => write!(f, "v3"),
            Arm64Register::V4 => write!(f, "v4"),
            Arm64Register::V5 => write!(f, "v5"),
            Arm64Register::V6 => write!(f, "v6"),
            Arm64Register::V7 => write!(f, "v7"),
            Arm64Register::V8 => write!(f, "v8"),
            Arm64Register::V9 => write!(f, "v9"),
            Arm64Register::V10 => write!(f, "v10"),
            Arm64Register::V11 => write!(f, "v11"),
            Arm64Register::V12 => write!(f, "v12"),
            Arm64Register::V13 => write!(f, "v13"),
            Arm64Register::V14 => write!(f, "v14"),
            Arm64Register::V15 => write!(f, "v15"),
            Arm64Register::V16 => write!(f, "v16"),
            Arm64Register::V17 => write!(f, "v17"),
            Arm64Register::V18 => write!(f, "v18"),
            Arm64Register::V19 => write!(f, "v19"),
            Arm64Register::V20 => write!(f, "v20"),
            Arm64Register::V21 => write!(f, "v21"),
            Arm64Register::V22 => write!(f, "v22"),
            Arm64Register::V23 => write!(f, "v23"),
            Arm64Register::V24 => write!(f, "v24"),
            Arm64Register::V25 => write!(f, "v25"),
            Arm64Register::V26 => write!(f, "v26"),
            Arm64Register::V27 => write!(f, "v27"),
            Arm64Register::V28 => write!(f, "v28"),
            Arm64Register::V29 => write!(f, "v29"),
            Arm64Register::V30 => write!(f, "v30"),
            Arm64Register::V31 => write!(f, "v31"),

            // System registers
            Arm64Register::PC => write!(f, "pc"),
            Arm64Register::NZCV => write!(f, "nzcv"),
            Arm64Register::FPCR => write!(f, "fpcr"),
            Arm64Register::FPSR => write!(f, "fpsr"),
        }
    }
}

impl fmt::Display for Arm64Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

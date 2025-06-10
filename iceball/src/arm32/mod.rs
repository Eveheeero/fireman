//! ARM32 (ARMv7) disassembler module

pub mod register;
pub mod statement;

use crate::{Argument, DisassembleError, Memory};

pub use register::Arm32Register;
pub use statement::Arm32Statement;

/// Parse ARM32 instruction argument
pub fn parse_argument(op: impl AsRef<str>) -> Result<Argument, DisassembleError> {
    let op = op.as_ref().trim();

    // Check for register
    if let Ok(reg) = Arm32Register::parse(op) {
        return Ok(Argument::Register(crate::Register::Arm32(reg)));
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

    // Try parsing as immediate without '#' prefix (some assemblers omit it)
    if let Ok(value) = parse_immediate(op) {
        return Ok(Argument::Constant(value));
    }

    Err(DisassembleError::Unknown)
}

/// Parse immediate value (with or without prefix)
fn parse_immediate(s: &str) -> Result<u64, DisassembleError> {
    let s = s.trim();

    // Handle hex values
    if let Some(hex_str) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(hex_str, 16).map_err(|_| DisassembleError::Unknown)
    }
    // Handle decimal values
    else {
        s.parse::<u64>().map_err(|_| DisassembleError::Unknown)
    }
}

/// Parse memory operand like [r0], [r1, #4], [r2, r3, lsl #2]
fn parse_memory_operand(inner: &str) -> Result<Argument, DisassembleError> {
    let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

    if parts.is_empty() {
        return Err(DisassembleError::Unknown);
    }

    let mut index: Option<crate::Register> = None;
    let mut scale: u8 = 1;
    let mut displacement: i64 = 0;

    // First part should be a register
    let base = if let Ok(base_reg) = Arm32Register::parse(parts[0]) {
        Some(crate::Register::Arm32(base_reg))
    } else {
        return Err(DisassembleError::Unknown);
    };

    // Parse additional parts (offset, index register with shift)
    for part in parts.iter().skip(1) {
        // Check for immediate offset
        if let Some(imm_str) = part.strip_prefix('#') {
            if let Ok(offset) = parse_immediate(imm_str) {
                displacement = offset as i64;
            } else if let Some(neg_str) = imm_str.strip_prefix('-') {
                if let Ok(offset) = parse_immediate(neg_str) {
                    displacement = -(offset as i64);
                }
            }
        }
        // Check for register with optional shift
        else if let Ok(index_reg) =
            Arm32Register::parse(part.split_whitespace().next().unwrap_or(part))
        {
            index = Some(crate::Register::Arm32(index_reg));

            // Check for shift operations (lsl, lsr, asr, ror)
            let tokens: Vec<&str> = part.split_whitespace().collect();
            if tokens.len() >= 3 && tokens[1] == "lsl" {
                if let Some(shift_str) = tokens[2].strip_prefix('#') {
                    if let Ok(shift) = shift_str.parse::<u8>() {
                        scale = 1 << shift; // Convert left shift to scale
                    }
                }
            }
            // TODO: Handle other shift operations
        }
    }

    Ok(Argument::Memory(Memory {
        base,
        index,
        scale,
        displacement,
        size: None,
    }))
}

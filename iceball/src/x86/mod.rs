pub mod register;
pub mod statement;

use crate::{Argument, DisassembleError, Memory, Register};

/// Parses an x86 (32-bit) memory operand from arguments
pub fn parse_memory(args: &[Argument]) -> Option<Memory> {
    match args.first()? {
        Argument::Memory(mem) => Some(mem.clone()),
        _ => None,
    }
}

/// Creates a memory operand from register + displacement
pub fn create_memory_from_register(reg: Register, displacement: i64) -> Memory {
    Memory {
        base: Some(reg),
        index: None,
        scale: 1,
        displacement,
        size: None,
    }
}

/// Creates a memory operand from base + index*scale + displacement
pub fn create_memory_sib(
    base: Option<Register>,
    index: Option<Register>,
    scale: u8,
    displacement: i64,
) -> Memory {
    Memory {
        base,
        index,
        scale,
        displacement,
        size: None,
    }
}

/// Parse an x86 operand string into an Argument
pub fn parse_argument(op: impl AsRef<str>) -> Result<Argument, DisassembleError> {
    let op = op.as_ref();

    // Try to parse as register
    if let Ok(reg) = parse_register(op) {
        return Ok(Argument::Register(Register::X86(reg)));
    }

    // Try to parse as immediate
    if let Some(imm) = parse_immediate(op) {
        return Ok(Argument::Constant(imm));
    }

    // Try to parse as memory operand
    if let Some(mem) = parse_memory_operand(op) {
        return Ok(Argument::Memory(mem));
    }

    Err(DisassembleError::Unknown)
}

/// Parse a register name
fn parse_register(name: &str) -> Result<register::X86Register, DisassembleError> {
    use register::X86Register::*;

    let reg = match name.to_lowercase().as_str() {
        // 32-bit registers
        "eax" => EAX,
        "ebx" => EBX,
        "ecx" => ECX,
        "edx" => EDX,
        "esi" => ESI,
        "edi" => EDI,
        "esp" => ESP,
        "ebp" => EBP,

        // 16-bit registers
        "ax" => AX,
        "bx" => BX,
        "cx" => CX,
        "dx" => DX,
        "si" => SI,
        "di" => DI,
        "sp" => SP,
        "bp" => BP,

        // 8-bit registers
        "al" => AL,
        "ah" => AH,
        "bl" => BL,
        "bh" => BH,
        "cl" => CL,
        "ch" => CH,
        "dl" => DL,
        "dh" => DH,

        // Special registers
        "eip" => EIP,
        "eflags" => EFLAGS,

        // Segment registers
        "cs" => CS,
        "ds" => DS,
        "es" => ES,
        "fs" => FS,
        "gs" => GS,
        "ss" => SS,

        // FPU registers
        "st0" | "st(0)" => ST0,
        "st1" | "st(1)" => ST1,
        "st2" | "st(2)" => ST2,
        "st3" | "st(3)" => ST3,
        "st4" | "st(4)" => ST4,
        "st5" | "st(5)" => ST5,
        "st6" | "st(6)" => ST6,
        "st7" | "st(7)" => ST7,

        // MMX registers
        "mm0" => MM0,
        "mm1" => MM1,
        "mm2" => MM2,
        "mm3" => MM3,
        "mm4" => MM4,
        "mm5" => MM5,
        "mm6" => MM6,
        "mm7" => MM7,

        // SSE registers
        "xmm0" => XMM0,
        "xmm1" => XMM1,
        "xmm2" => XMM2,
        "xmm3" => XMM3,
        "xmm4" => XMM4,
        "xmm5" => XMM5,
        "xmm6" => XMM6,
        "xmm7" => XMM7,

        _ => return Err(DisassembleError::UnknownRegister),
    };

    Ok(reg)
}

/// Parse an immediate value
fn parse_immediate(op: &str) -> Option<u64> {
    // Handle hex, decimal, binary formats
    if op.starts_with("0x") || op.starts_with("0X") {
        // Hex
        u64::from_str_radix(&op[2..], 16).ok()
    } else if op.starts_with("0b") || op.starts_with("0B") {
        // Binary
        u64::from_str_radix(&op[2..], 2).ok()
    } else {
        // Try decimal
        op.parse::<u64>().ok()
    }
}

/// Parse a memory operand like [ebp+4] or dword ptr [eax+ebx*2+8]
fn parse_memory_operand(_op: &str) -> Option<Memory> {
    // For now, return None - full memory operand parsing is complex
    // This would need to handle various formats like:
    // [eax], [eax+4], [eax+ebx], [eax+ebx*2], [eax+ebx*2+4], etc.
    None
}

//! Architecture-specific register mapping
//!
//! This module provides unified register mapping across different architectures,
//! converting architecture-specific registers to the IR register format.

use crate::ir::register::Register;
use std::collections::BTreeMap;

/// Trait for architecture-specific register mapping
pub trait RegisterMapper {
    /// Get the IR register representation for an architecture-specific register
    fn to_ir_register(&self, reg_name: &str) -> Option<Register>;

    /// Get the register size in bits
    fn get_register_size(&self, reg_name: &str) -> Option<usize>;

    /// Get the calling convention registers for this architecture
    fn get_calling_convention_registers(&self) -> CallingConventionRegisters;

    /// Check if a register is a stack pointer
    fn is_stack_pointer(&self, reg_name: &str) -> bool;

    /// Check if a register is a frame pointer
    fn is_frame_pointer(&self, reg_name: &str) -> bool;

    /// Check if a register is an instruction pointer
    fn is_instruction_pointer(&self, reg_name: &str) -> bool;
}

/// Calling convention register information
#[derive(Debug, Clone)]
pub struct CallingConventionRegisters {
    /// Registers used for function arguments (in order)
    pub argument_registers: Vec<&'static str>,
    /// Register used for return value
    pub return_register: &'static str,
    /// Caller-saved registers (volatile)
    pub caller_saved: Vec<&'static str>,
    /// Callee-saved registers (non-volatile)
    pub callee_saved: Vec<&'static str>,
}

/// x86 (32-bit) register mapper
pub struct X86RegisterMapper {
    register_map: BTreeMap<&'static str, Register>,
}

impl X86RegisterMapper {
    pub fn new() -> Self {
        let mut register_map = BTreeMap::new();

        // 32-bit general purpose registers
        register_map.insert("eax", Register::new("eax", 0..32));
        register_map.insert("ebx", Register::new("ebx", 0..32));
        register_map.insert("ecx", Register::new("ecx", 0..32));
        register_map.insert("edx", Register::new("edx", 0..32));
        register_map.insert("esi", Register::new("esi", 0..32));
        register_map.insert("edi", Register::new("edi", 0..32));
        register_map.insert("esp", Register::new("esp", 0..32));
        register_map.insert("ebp", Register::new("ebp", 0..32));

        // 16-bit registers
        register_map.insert("ax", Register::new("ax", 0..16));
        register_map.insert("bx", Register::new("bx", 0..16));
        register_map.insert("cx", Register::new("cx", 0..16));
        register_map.insert("dx", Register::new("dx", 0..16));
        register_map.insert("si", Register::new("si", 0..16));
        register_map.insert("di", Register::new("di", 0..16));
        register_map.insert("sp", Register::new("sp", 0..16));
        register_map.insert("bp", Register::new("bp", 0..16));

        // 8-bit registers
        register_map.insert("al", Register::new("al", 0..8));
        register_map.insert("ah", Register::new("ah", 8..16));
        register_map.insert("bl", Register::new("bl", 0..8));
        register_map.insert("bh", Register::new("bh", 8..16));
        register_map.insert("cl", Register::new("cl", 0..8));
        register_map.insert("ch", Register::new("ch", 8..16));
        register_map.insert("dl", Register::new("dl", 0..8));
        register_map.insert("dh", Register::new("dh", 8..16));

        Self { register_map }
    }
}

impl RegisterMapper for X86RegisterMapper {
    fn to_ir_register(&self, reg_name: &str) -> Option<Register> {
        self.register_map
            .get(reg_name.to_lowercase().as_str())
            .copied()
    }

    fn get_register_size(&self, reg_name: &str) -> Option<usize> {
        self.to_ir_register(reg_name).map(|reg| reg.bit_len())
    }

    fn get_calling_convention_registers(&self) -> CallingConventionRegisters {
        // cdecl calling convention (most common on x86)
        CallingConventionRegisters {
            argument_registers: vec![], // Arguments passed on stack in cdecl
            return_register: "eax",
            caller_saved: vec!["eax", "ecx", "edx"],
            callee_saved: vec!["ebx", "esi", "edi", "ebp"],
        }
    }

    fn is_stack_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "esp"
    }

    fn is_frame_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "ebp"
    }

    fn is_instruction_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "eip"
    }
}

/// x86_64 (64-bit) register mapper
pub struct X86_64RegisterMapper {
    register_map: BTreeMap<&'static str, Register>,
}

impl X86_64RegisterMapper {
    pub fn new() -> Self {
        let mut register_map = BTreeMap::new();

        // 64-bit general purpose registers
        register_map.insert("rax", Register::new("rax", 0..64));
        register_map.insert("rbx", Register::new("rbx", 0..64));
        register_map.insert("rcx", Register::new("rcx", 0..64));
        register_map.insert("rdx", Register::new("rdx", 0..64));
        register_map.insert("rsi", Register::new("rsi", 0..64));
        register_map.insert("rdi", Register::new("rdi", 0..64));
        register_map.insert("rsp", Register::new("rsp", 0..64));
        register_map.insert("rbp", Register::new("rbp", 0..64));
        register_map.insert("r8", Register::new("r8", 0..64));
        register_map.insert("r9", Register::new("r9", 0..64));
        register_map.insert("r10", Register::new("r10", 0..64));
        register_map.insert("r11", Register::new("r11", 0..64));
        register_map.insert("r12", Register::new("r12", 0..64));
        register_map.insert("r13", Register::new("r13", 0..64));
        register_map.insert("r14", Register::new("r14", 0..64));
        register_map.insert("r15", Register::new("r15", 0..64));

        // 32-bit sub-registers
        register_map.insert("eax", Register::new("eax", 0..32));
        register_map.insert("ebx", Register::new("ebx", 0..32));
        register_map.insert("ecx", Register::new("ecx", 0..32));
        register_map.insert("edx", Register::new("edx", 0..32));
        register_map.insert("esi", Register::new("esi", 0..32));
        register_map.insert("edi", Register::new("edi", 0..32));
        register_map.insert("esp", Register::new("esp", 0..32));
        register_map.insert("ebp", Register::new("ebp", 0..32));
        register_map.insert("r8d", Register::new("r8d", 0..32));
        register_map.insert("r9d", Register::new("r9d", 0..32));
        register_map.insert("r10d", Register::new("r10d", 0..32));
        register_map.insert("r11d", Register::new("r11d", 0..32));
        register_map.insert("r12d", Register::new("r12d", 0..32));
        register_map.insert("r13d", Register::new("r13d", 0..32));
        register_map.insert("r14d", Register::new("r14d", 0..32));
        register_map.insert("r15d", Register::new("r15d", 0..32));

        // 16-bit sub-registers
        register_map.insert("ax", Register::new("ax", 0..16));
        register_map.insert("bx", Register::new("bx", 0..16));
        register_map.insert("cx", Register::new("cx", 0..16));
        register_map.insert("dx", Register::new("dx", 0..16));
        register_map.insert("si", Register::new("si", 0..16));
        register_map.insert("di", Register::new("di", 0..16));
        register_map.insert("sp", Register::new("sp", 0..16));
        register_map.insert("bp", Register::new("bp", 0..16));
        register_map.insert("r8w", Register::new("r8w", 0..16));
        register_map.insert("r9w", Register::new("r9w", 0..16));
        register_map.insert("r10w", Register::new("r10w", 0..16));
        register_map.insert("r11w", Register::new("r11w", 0..16));
        register_map.insert("r12w", Register::new("r12w", 0..16));
        register_map.insert("r13w", Register::new("r13w", 0..16));
        register_map.insert("r14w", Register::new("r14w", 0..16));
        register_map.insert("r15w", Register::new("r15w", 0..16));

        // 8-bit sub-registers
        register_map.insert("al", Register::new("al", 0..8));
        register_map.insert("ah", Register::new("ah", 8..16));
        register_map.insert("bl", Register::new("bl", 0..8));
        register_map.insert("bh", Register::new("bh", 8..16));
        register_map.insert("cl", Register::new("cl", 0..8));
        register_map.insert("ch", Register::new("ch", 8..16));
        register_map.insert("dl", Register::new("dl", 0..8));
        register_map.insert("dh", Register::new("dh", 8..16));
        register_map.insert("sil", Register::new("sil", 0..8));
        register_map.insert("dil", Register::new("dil", 0..8));
        register_map.insert("spl", Register::new("spl", 0..8));
        register_map.insert("bpl", Register::new("bpl", 0..8));
        register_map.insert("r8b", Register::new("r8b", 0..8));
        register_map.insert("r9b", Register::new("r9b", 0..8));
        register_map.insert("r10b", Register::new("r10b", 0..8));
        register_map.insert("r11b", Register::new("r11b", 0..8));
        register_map.insert("r12b", Register::new("r12b", 0..8));
        register_map.insert("r13b", Register::new("r13b", 0..8));
        register_map.insert("r14b", Register::new("r14b", 0..8));
        register_map.insert("r15b", Register::new("r15b", 0..8));

        Self { register_map }
    }
}

impl RegisterMapper for X86_64RegisterMapper {
    fn to_ir_register(&self, reg_name: &str) -> Option<Register> {
        self.register_map
            .get(reg_name.to_lowercase().as_str())
            .copied()
    }

    fn get_register_size(&self, reg_name: &str) -> Option<usize> {
        self.to_ir_register(reg_name).map(|reg| reg.bit_len())
    }

    fn get_calling_convention_registers(&self) -> CallingConventionRegisters {
        // System V AMD64 ABI (Linux)
        CallingConventionRegisters {
            argument_registers: vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"],
            return_register: "rax",
            caller_saved: vec!["rax", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11"],
            callee_saved: vec!["rbx", "rbp", "r12", "r13", "r14", "r15"],
        }
    }

    fn is_stack_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "rsp"
    }

    fn is_frame_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "rbp"
    }

    fn is_instruction_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "rip"
    }
}

/// ARM32 register mapper
pub struct Arm32RegisterMapper {
    register_map: BTreeMap<&'static str, Register>,
}

impl Arm32RegisterMapper {
    pub fn new() -> Self {
        let mut register_map = BTreeMap::new();

        // Core registers
        register_map.insert("r0", Register::new("r0", 0..32));
        register_map.insert("r1", Register::new("r1", 0..32));
        register_map.insert("r2", Register::new("r2", 0..32));
        register_map.insert("r3", Register::new("r3", 0..32));
        register_map.insert("r4", Register::new("r4", 0..32));
        register_map.insert("r5", Register::new("r5", 0..32));
        register_map.insert("r6", Register::new("r6", 0..32));
        register_map.insert("r7", Register::new("r7", 0..32));
        register_map.insert("r8", Register::new("r8", 0..32));
        register_map.insert("r9", Register::new("r9", 0..32));
        register_map.insert("r10", Register::new("r10", 0..32));
        register_map.insert("r11", Register::new("r11", 0..32));
        register_map.insert("r12", Register::new("r12", 0..32));
        register_map.insert("r13", Register::new("r13", 0..32));
        register_map.insert("r14", Register::new("r14", 0..32));
        register_map.insert("r15", Register::new("r15", 0..32));

        // Aliases
        register_map.insert("sp", Register::new("sp", 0..32)); // R13
        register_map.insert("lr", Register::new("lr", 0..32)); // R14
        register_map.insert("pc", Register::new("pc", 0..32)); // R15

        Self { register_map }
    }
}

impl RegisterMapper for Arm32RegisterMapper {
    fn to_ir_register(&self, reg_name: &str) -> Option<Register> {
        self.register_map
            .get(reg_name.to_lowercase().as_str())
            .copied()
    }

    fn get_register_size(&self, _reg_name: &str) -> Option<usize> {
        Some(32) // All ARM32 registers are 32-bit
    }

    fn get_calling_convention_registers(&self) -> CallingConventionRegisters {
        // ARM AAPCS (ARM Architecture Procedure Call Standard)
        CallingConventionRegisters {
            argument_registers: vec!["r0", "r1", "r2", "r3"],
            return_register: "r0",
            caller_saved: vec!["r0", "r1", "r2", "r3", "r12"],
            callee_saved: vec!["r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11"],
        }
    }

    fn is_stack_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "sp" || reg_name.to_lowercase() == "r13"
    }

    fn is_frame_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "r11" // Common frame pointer on ARM
    }

    fn is_instruction_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "pc" || reg_name.to_lowercase() == "r15"
    }
}

/// ARM64 register mapper
pub struct Arm64RegisterMapper {
    register_map: BTreeMap<&'static str, Register>,
}

impl Arm64RegisterMapper {
    pub fn new() -> Self {
        let mut register_map = BTreeMap::new();

        // 64-bit general purpose registers
        register_map.insert("x0", Register::new("x0", 0..64));
        register_map.insert("x1", Register::new("x1", 0..64));
        register_map.insert("x2", Register::new("x2", 0..64));
        register_map.insert("x3", Register::new("x3", 0..64));
        register_map.insert("x4", Register::new("x4", 0..64));
        register_map.insert("x5", Register::new("x5", 0..64));
        register_map.insert("x6", Register::new("x6", 0..64));
        register_map.insert("x7", Register::new("x7", 0..64));
        register_map.insert("x8", Register::new("x8", 0..64));
        register_map.insert("x9", Register::new("x9", 0..64));
        register_map.insert("x10", Register::new("x10", 0..64));
        register_map.insert("x11", Register::new("x11", 0..64));
        register_map.insert("x12", Register::new("x12", 0..64));
        register_map.insert("x13", Register::new("x13", 0..64));
        register_map.insert("x14", Register::new("x14", 0..64));
        register_map.insert("x15", Register::new("x15", 0..64));
        register_map.insert("x16", Register::new("x16", 0..64));
        register_map.insert("x17", Register::new("x17", 0..64));
        register_map.insert("x18", Register::new("x18", 0..64));
        register_map.insert("x19", Register::new("x19", 0..64));
        register_map.insert("x20", Register::new("x20", 0..64));
        register_map.insert("x21", Register::new("x21", 0..64));
        register_map.insert("x22", Register::new("x22", 0..64));
        register_map.insert("x23", Register::new("x23", 0..64));
        register_map.insert("x24", Register::new("x24", 0..64));
        register_map.insert("x25", Register::new("x25", 0..64));
        register_map.insert("x26", Register::new("x26", 0..64));
        register_map.insert("x27", Register::new("x27", 0..64));
        register_map.insert("x28", Register::new("x28", 0..64));
        register_map.insert("x29", Register::new("x29", 0..64));
        register_map.insert("x30", Register::new("x30", 0..64));

        // 32-bit sub-registers
        register_map.insert("w0", Register::new("w0", 0..32));
        register_map.insert("w1", Register::new("w1", 0..32));
        register_map.insert("w2", Register::new("w2", 0..32));
        register_map.insert("w3", Register::new("w3", 0..32));
        register_map.insert("w4", Register::new("w4", 0..32));
        register_map.insert("w5", Register::new("w5", 0..32));
        register_map.insert("w6", Register::new("w6", 0..32));
        register_map.insert("w7", Register::new("w7", 0..32));
        register_map.insert("w8", Register::new("w8", 0..32));
        register_map.insert("w9", Register::new("w9", 0..32));
        register_map.insert("w10", Register::new("w10", 0..32));
        register_map.insert("w11", Register::new("w11", 0..32));
        register_map.insert("w12", Register::new("w12", 0..32));
        register_map.insert("w13", Register::new("w13", 0..32));
        register_map.insert("w14", Register::new("w14", 0..32));
        register_map.insert("w15", Register::new("w15", 0..32));
        register_map.insert("w16", Register::new("w16", 0..32));
        register_map.insert("w17", Register::new("w17", 0..32));
        register_map.insert("w18", Register::new("w18", 0..32));
        register_map.insert("w19", Register::new("w19", 0..32));
        register_map.insert("w20", Register::new("w20", 0..32));
        register_map.insert("w21", Register::new("w21", 0..32));
        register_map.insert("w22", Register::new("w22", 0..32));
        register_map.insert("w23", Register::new("w23", 0..32));
        register_map.insert("w24", Register::new("w24", 0..32));
        register_map.insert("w25", Register::new("w25", 0..32));
        register_map.insert("w26", Register::new("w26", 0..32));
        register_map.insert("w27", Register::new("w27", 0..32));
        register_map.insert("w28", Register::new("w28", 0..32));
        register_map.insert("w29", Register::new("w29", 0..32));
        register_map.insert("w30", Register::new("w30", 0..32));

        // Special registers
        register_map.insert("sp", Register::new("sp", 0..64));
        register_map.insert("lr", Register::new("lr", 0..64)); // X30
        register_map.insert("fp", Register::new("fp", 0..64)); // X29

        Self { register_map }
    }
}

impl RegisterMapper for Arm64RegisterMapper {
    fn to_ir_register(&self, reg_name: &str) -> Option<Register> {
        self.register_map
            .get(reg_name.to_lowercase().as_str())
            .copied()
    }

    fn get_register_size(&self, reg_name: &str) -> Option<usize> {
        self.to_ir_register(reg_name).map(|reg| reg.bit_len())
    }

    fn get_calling_convention_registers(&self) -> CallingConventionRegisters {
        // ARM64 AAPCS64
        CallingConventionRegisters {
            argument_registers: vec!["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"],
            return_register: "x0",
            caller_saved: vec![
                "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11", "x12",
                "x13", "x14", "x15", "x16", "x17", "x18",
            ],
            callee_saved: vec![
                "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28", "x29", "x30",
            ],
        }
    }

    fn is_stack_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "sp"
    }

    fn is_frame_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "fp" || reg_name.to_lowercase() == "x29"
    }

    fn is_instruction_pointer(&self, reg_name: &str) -> bool {
        reg_name.to_lowercase() == "pc" // Program counter is not directly accessible but can be referenced
    }
}

/// Get a register mapper for a specific architecture
pub fn get_register_mapper(arch: crate::arch::architecture::ArchType) -> Box<dyn RegisterMapper> {
    match arch {
        crate::arch::architecture::ArchType::X86 => Box::new(X86RegisterMapper::new()),
        crate::arch::architecture::ArchType::X86_64 => Box::new(X86_64RegisterMapper::new()),
        crate::arch::architecture::ArchType::Arm32 => Box::new(Arm32RegisterMapper::new()),
        crate::arch::architecture::ArchType::Arm64 => Box::new(Arm64RegisterMapper::new()),
        _ => Box::new(X86_64RegisterMapper::new()), // Default fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x86_register_mapping() {
        let mapper = X86RegisterMapper::new();

        // Test general purpose registers
        assert_eq!(mapper.get_register_size("eax"), Some(32));
        assert_eq!(mapper.get_register_size("ax"), Some(16));
        assert_eq!(mapper.get_register_size("al"), Some(8));
        assert_eq!(mapper.get_register_size("ah"), Some(8));

        // Test special registers
        assert!(mapper.is_stack_pointer("esp"));
        assert!(mapper.is_frame_pointer("ebp"));
        assert!(!mapper.is_stack_pointer("eax"));

        // Test calling convention
        let cc = mapper.get_calling_convention_registers();
        assert_eq!(cc.return_register, "eax");
        assert!(cc.argument_registers.is_empty()); // cdecl uses stack
    }

    #[test]
    fn test_x86_64_register_mapping() {
        let mapper = X86_64RegisterMapper::new();

        // Test 64-bit registers
        assert_eq!(mapper.get_register_size("rax"), Some(64));
        assert_eq!(mapper.get_register_size("r8"), Some(64));

        // Test sub-registers
        assert_eq!(mapper.get_register_size("eax"), Some(32));
        assert_eq!(mapper.get_register_size("r8d"), Some(32));
        assert_eq!(mapper.get_register_size("ax"), Some(16));
        assert_eq!(mapper.get_register_size("al"), Some(8));

        // Test special registers
        assert!(mapper.is_stack_pointer("rsp"));
        assert!(mapper.is_frame_pointer("rbp"));

        // Test calling convention
        let cc = mapper.get_calling_convention_registers();
        assert_eq!(cc.argument_registers.len(), 6);
        assert_eq!(cc.argument_registers[0], "rdi");
        assert_eq!(cc.return_register, "rax");
    }

    #[test]
    fn test_arm32_register_mapping() {
        let mapper = Arm32RegisterMapper::new();

        // All ARM32 registers are 32-bit
        assert_eq!(mapper.get_register_size("r0"), Some(32));
        assert_eq!(mapper.get_register_size("r15"), Some(32));
        assert_eq!(mapper.get_register_size("sp"), Some(32));

        // Test special registers
        assert!(mapper.is_stack_pointer("sp"));
        assert!(mapper.is_stack_pointer("r13"));
        assert!(mapper.is_instruction_pointer("pc"));
        assert!(mapper.is_instruction_pointer("r15"));

        // Test calling convention
        let cc = mapper.get_calling_convention_registers();
        assert_eq!(cc.argument_registers.len(), 4);
        assert_eq!(cc.argument_registers[0], "r0");
    }

    #[test]
    fn test_arm64_register_mapping() {
        let mapper = Arm64RegisterMapper::new();

        // Test 64-bit registers
        assert_eq!(mapper.get_register_size("x0"), Some(64));
        assert_eq!(mapper.get_register_size("x30"), Some(64));

        // Test 32-bit sub-registers
        assert_eq!(mapper.get_register_size("w0"), Some(32));
        assert_eq!(mapper.get_register_size("w30"), Some(32));

        // Test special registers
        assert!(mapper.is_stack_pointer("sp"));
        assert!(mapper.is_frame_pointer("fp"));
        assert!(mapper.is_frame_pointer("x29"));

        // Test calling convention
        let cc = mapper.get_calling_convention_registers();
        assert_eq!(cc.argument_registers.len(), 8);
        assert_eq!(cc.argument_registers[0], "x0");
        assert_eq!(cc.return_register, "x0");
    }
}

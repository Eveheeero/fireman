//! CPU state representation for simulation
//!
//! This module provides structures to represent CPU registers and flags
//! during simulation. It supports x86_64 architecture initially.

use crate::simulation::{FpuState, SimulationResult};
use std::collections::BTreeMap;

/// Represents the state of CPU registers and flags
#[derive(Debug, Clone)]
pub struct CpuState {
    /// General purpose registers
    registers: BTreeMap<String, u64>,
    /// CPU flags
    flags: BTreeMap<String, bool>,
    /// Instruction pointer
    pub rip: u64,
    /// Stack pointer
    pub rsp: u64,
    /// FPU state (x87)
    pub fpu: FpuState,
}

impl CpuState {
    /// Create a new CPU state with default values
    pub fn new() -> Self {
        let mut state = Self {
            registers: BTreeMap::new(),
            flags: BTreeMap::new(),
            rip: 0,
            rsp: 0x7fff_ffff_ffff_0000, // Default stack pointer
            fpu: FpuState::new(),
        };

        // Initialize x86_64 registers
        state.init_x64_registers();
        state.init_x64_flags();

        state
    }

    /// Initialize x86_64 general purpose registers
    fn init_x64_registers(&mut self) {
        // 64-bit registers
        let regs_64 = [
            "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp", "r8", "r9", "r10", "r11",
            "r12", "r13", "r14", "r15",
        ];

        for reg in &regs_64 {
            self.registers.insert(reg.to_string(), 0);
        }

        // Special case: rsp is already set
        self.registers.insert("rsp".to_string(), self.rsp);

        // Segment registers
        let seg_regs = ["cs", "ds", "es", "fs", "gs", "ss"];
        for reg in &seg_regs {
            self.registers.insert(reg.to_string(), 0);
        }
    }

    /// Initialize x86_64 flags
    fn init_x64_flags(&mut self) {
        let flags = ["cf", "pf", "af", "zf", "sf", "tf", "if", "df", "of"];

        for flag in &flags {
            self.flags.insert(flag.to_string(), false);
        }
    }

    /// Get register value
    pub fn get_register(&self, name: &str) -> SimulationResult<u64> {
        // Handle special registers
        match name {
            "rip" => return Ok(self.rip),
            "rsp" => return Ok(self.rsp),
            _ => {}
        }

        // Handle FPU registers (st0-st7)
        if name.starts_with("st") && name.len() == 3 {
            if let Ok(index) = name[2..].parse::<u8>() {
                if index <= 7 {
                    // Convert f64 to u64 bits for compatibility
                    let value = self.fpu.get(index)?;
                    return Ok(value.to_bits());
                }
            }
        }

        // Handle sub-registers (e.g., eax, ax, al for rax)
        let (full_reg, mask, shift) = self.resolve_register_name(name)?;

        let value =
            self.registers.get(full_reg).copied().ok_or_else(|| {
                crate::simulation::SimulationError::UnknownRegister(name.to_string())
            })?;

        Ok((value >> shift) & mask)
    }

    /// Set register value
    pub fn set_register(&mut self, name: &str, value: u64) -> SimulationResult<()> {
        // Handle special registers
        match name {
            "rip" => {
                self.rip = value;
                return Ok(());
            }
            "rsp" => {
                self.rsp = value;
                self.registers.insert("rsp".to_string(), value);
                return Ok(());
            }
            _ => {}
        }

        // Handle FPU registers (st0-st7)
        if name.starts_with("st") && name.len() == 3 {
            if let Ok(index) = name[2..].parse::<u8>() {
                if index <= 7 {
                    // Convert u64 bits to f64
                    let float_value = f64::from_bits(value);
                    self.fpu.set(index, float_value)?;
                    return Ok(());
                }
            }
        }

        // Handle sub-registers
        let (full_reg, mask, shift) = self.resolve_register_name(name)?;

        let current =
            self.registers.get(full_reg).copied().ok_or_else(|| {
                crate::simulation::SimulationError::UnknownRegister(name.to_string())
            })?;

        let new_value = (current & !(mask << shift)) | ((value & mask) << shift);
        self.registers.insert(full_reg.to_string(), new_value);

        Ok(())
    }

    /// Get flag value
    pub fn get_flag(&self, name: &str) -> SimulationResult<bool> {
        self.flags.get(name).copied().ok_or_else(|| {
            crate::simulation::SimulationError::UnknownRegister(format!("flag {}", name))
        })
    }

    /// Set flag value
    pub fn set_flag(&mut self, name: &str, value: bool) -> SimulationResult<()> {
        if self.flags.contains_key(name) {
            self.flags.insert(name.to_string(), value);
            Ok(())
        } else {
            Err(crate::simulation::SimulationError::UnknownRegister(
                format!("flag {}", name),
            ))
        }
    }

    /// Resolve register name to full register name, mask, and shift
    fn resolve_register_name(&self, name: &str) -> SimulationResult<(&'static str, u64, u32)> {
        match name {
            // 64-bit registers
            "rax" => Ok(("rax", 0xffff_ffff_ffff_ffff, 0)),
            "rbx" => Ok(("rbx", 0xffff_ffff_ffff_ffff, 0)),
            "rcx" => Ok(("rcx", 0xffff_ffff_ffff_ffff, 0)),
            "rdx" => Ok(("rdx", 0xffff_ffff_ffff_ffff, 0)),
            "rsi" => Ok(("rsi", 0xffff_ffff_ffff_ffff, 0)),
            "rdi" => Ok(("rdi", 0xffff_ffff_ffff_ffff, 0)),
            "rbp" => Ok(("rbp", 0xffff_ffff_ffff_ffff, 0)),
            "rsp" => Ok(("rsp", 0xffff_ffff_ffff_ffff, 0)),
            "r8" => Ok(("r8", 0xffff_ffff_ffff_ffff, 0)),
            "r9" => Ok(("r9", 0xffff_ffff_ffff_ffff, 0)),
            "r10" => Ok(("r10", 0xffff_ffff_ffff_ffff, 0)),
            "r11" => Ok(("r11", 0xffff_ffff_ffff_ffff, 0)),
            "r12" => Ok(("r12", 0xffff_ffff_ffff_ffff, 0)),
            "r13" => Ok(("r13", 0xffff_ffff_ffff_ffff, 0)),
            "r14" => Ok(("r14", 0xffff_ffff_ffff_ffff, 0)),
            "r15" => Ok(("r15", 0xffff_ffff_ffff_ffff, 0)),

            // 32-bit registers
            "eax" => Ok(("rax", 0xffff_ffff, 0)),
            "ebx" => Ok(("rbx", 0xffff_ffff, 0)),
            "ecx" => Ok(("rcx", 0xffff_ffff, 0)),
            "edx" => Ok(("rdx", 0xffff_ffff, 0)),
            "esi" => Ok(("rsi", 0xffff_ffff, 0)),
            "edi" => Ok(("rdi", 0xffff_ffff, 0)),
            "ebp" => Ok(("rbp", 0xffff_ffff, 0)),
            "esp" => Ok(("rsp", 0xffff_ffff, 0)),
            "r8d" => Ok(("r8", 0xffff_ffff, 0)),
            "r9d" => Ok(("r9", 0xffff_ffff, 0)),
            "r10d" => Ok(("r10", 0xffff_ffff, 0)),
            "r11d" => Ok(("r11", 0xffff_ffff, 0)),
            "r12d" => Ok(("r12", 0xffff_ffff, 0)),
            "r13d" => Ok(("r13", 0xffff_ffff, 0)),
            "r14d" => Ok(("r14", 0xffff_ffff, 0)),
            "r15d" => Ok(("r15", 0xffff_ffff, 0)),

            // 16-bit registers
            "ax" => Ok(("rax", 0xffff, 0)),
            "bx" => Ok(("rbx", 0xffff, 0)),
            "cx" => Ok(("rcx", 0xffff, 0)),
            "dx" => Ok(("rdx", 0xffff, 0)),
            "si" => Ok(("rsi", 0xffff, 0)),
            "di" => Ok(("rdi", 0xffff, 0)),
            "bp" => Ok(("rbp", 0xffff, 0)),
            "sp" => Ok(("rsp", 0xffff, 0)),
            "r8w" => Ok(("r8", 0xffff, 0)),
            "r9w" => Ok(("r9", 0xffff, 0)),
            "r10w" => Ok(("r10", 0xffff, 0)),
            "r11w" => Ok(("r11", 0xffff, 0)),
            "r12w" => Ok(("r12", 0xffff, 0)),
            "r13w" => Ok(("r13", 0xffff, 0)),
            "r14w" => Ok(("r14", 0xffff, 0)),
            "r15w" => Ok(("r15", 0xffff, 0)),

            // 8-bit low registers
            "al" => Ok(("rax", 0xff, 0)),
            "bl" => Ok(("rbx", 0xff, 0)),
            "cl" => Ok(("rcx", 0xff, 0)),
            "dl" => Ok(("rdx", 0xff, 0)),
            "sil" => Ok(("rsi", 0xff, 0)),
            "dil" => Ok(("rdi", 0xff, 0)),
            "bpl" => Ok(("rbp", 0xff, 0)),
            "spl" => Ok(("rsp", 0xff, 0)),
            "r8b" => Ok(("r8", 0xff, 0)),
            "r9b" => Ok(("r9", 0xff, 0)),
            "r10b" => Ok(("r10", 0xff, 0)),
            "r11b" => Ok(("r11", 0xff, 0)),
            "r12b" => Ok(("r12", 0xff, 0)),
            "r13b" => Ok(("r13", 0xff, 0)),
            "r14b" => Ok(("r14", 0xff, 0)),
            "r15b" => Ok(("r15", 0xff, 0)),

            // 8-bit high registers
            "ah" => Ok(("rax", 0xff, 8)),
            "bh" => Ok(("rbx", 0xff, 8)),
            "ch" => Ok(("rcx", 0xff, 8)),
            "dh" => Ok(("rdx", 0xff, 8)),

            _ => Err(crate::simulation::SimulationError::UnknownRegister(
                name.to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_operations() {
        let mut cpu = CpuState::new();

        // Test setting and getting 64-bit register
        cpu.set_register("rax", 0x1234567890abcdef).unwrap();
        assert_eq!(cpu.get_register("rax").unwrap(), 0x1234567890abcdef);

        // Test sub-registers
        assert_eq!(cpu.get_register("eax").unwrap(), 0x90abcdef);
        assert_eq!(cpu.get_register("ax").unwrap(), 0xcdef);
        assert_eq!(cpu.get_register("al").unwrap(), 0xef);
        assert_eq!(cpu.get_register("ah").unwrap(), 0xcd);

        // Test setting sub-register
        cpu.set_register("al", 0x42).unwrap();
        assert_eq!(cpu.get_register("rax").unwrap(), 0x1234567890abcd42);
    }

    #[test]
    fn test_flag_operations() {
        let mut cpu = CpuState::new();

        // Test setting and getting flags
        cpu.set_flag("zf", true).unwrap();
        assert!(cpu.get_flag("zf").unwrap());
        assert!(!cpu.get_flag("cf").unwrap());

        cpu.set_flag("cf", true).unwrap();
        assert!(cpu.get_flag("cf").unwrap());
    }
}

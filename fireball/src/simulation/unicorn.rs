//! Unicorn Engine integration for simulation
//!
//! This module provides a Unicorn-based implementation of the simulation
//! framework, replacing the custom CPU and memory emulation with the
//! battle-tested Unicorn Engine.

use std::collections::BTreeMap;
use unicorn_engine::unicorn_const::{Arch, Mode, Permission, SECOND_SCALE};
use unicorn_engine::{Context as UnicornContext, RegisterARM, RegisterARM64, RegisterX86, Unicorn};

use super::{SimulationError, SimulationResult};
use crate::arch::architecture::{ArchType, ArchitectureInfo, Endianness};
use crate::ir::Register;

/// Unicorn-based simulation engine
pub struct UnicornSimulator<'a> {
    /// Unicorn engine instance
    engine: Unicorn<'a, ()>,
    /// Architecture information
    architecture: ArchitectureInfo,
    /// Memory regions mapped
    memory_regions: BTreeMap<u64, (usize, Permission)>,
    /// Saved contexts for snapshot/restore
    contexts: BTreeMap<String, UnicornContext>,
}

impl<'a> UnicornSimulator<'a> {
    /// Create a new Unicorn simulator
    pub fn new(architecture: ArchitectureInfo) -> SimulationResult<Self> {
        let (arch, mode) = Self::get_unicorn_arch_mode(&architecture)?;
        let engine = Unicorn::new(arch, mode).map_err(|e| {
            SimulationError::InvalidOperation(format!("Failed to create Unicorn: {:?}", e))
        })?;

        Ok(Self {
            engine,
            architecture,
            memory_regions: BTreeMap::new(),
            contexts: BTreeMap::new(),
        })
    }

    /// Convert our architecture to Unicorn arch/mode
    fn get_unicorn_arch_mode(arch_info: &ArchitectureInfo) -> SimulationResult<(Arch, Mode)> {
        match arch_info.arch_type {
            ArchType::X86 => Ok((Arch::X86, Mode::MODE_32)),
            ArchType::X86_64 => Ok((Arch::X86, Mode::MODE_64)),
            ArchType::Arm32 => Ok((
                Arch::ARM,
                if arch_info.endianness == Endianness::Little {
                    Mode::LITTLE_ENDIAN
                } else {
                    Mode::BIG_ENDIAN
                },
            )),
            ArchType::Arm64 => Ok((
                Arch::ARM64,
                if arch_info.endianness == Endianness::Little {
                    Mode::LITTLE_ENDIAN
                } else {
                    Mode::BIG_ENDIAN
                },
            )),
            _ => Err(SimulationError::UnsupportedFeature(format!(
                "Architecture {:?} not supported",
                arch_info.arch_type
            ))),
        }
    }

    /// Map memory with given permissions
    pub fn map_memory(
        &mut self,
        address: u64,
        size: usize,
        perms: Permission,
    ) -> SimulationResult<()> {
        self.engine
            .mem_map(address, size, perms)
            .map_err(|_e| SimulationError::MemoryAccessViolation { address, size })?;

        self.memory_regions.insert(address, (size, perms));
        Ok(())
    }

    /// Write to memory
    pub fn write_memory(&mut self, address: u64, data: &[u8]) -> SimulationResult<()> {
        self.engine
            .mem_write(address, data)
            .map_err(|_| SimulationError::MemoryAccessViolation {
                address,
                size: data.len(),
            })
    }

    /// Read from memory
    pub fn read_memory(&mut self, address: u64, size: usize) -> SimulationResult<Vec<u8>> {
        let mut data = vec![0u8; size];
        self.engine
            .mem_read(address, &mut data)
            .map_err(|_| SimulationError::MemoryAccessViolation { address, size })?;
        Ok(data)
    }

    /// Read a register value
    pub fn read_register(&self, register: &Register) -> SimulationResult<u64> {
        match self.architecture.arch_type {
            ArchType::X86 | ArchType::X86_64 => {
                let uc_reg = self.map_x86_register(register)?;
                self.engine
                    .reg_read(uc_reg)
                    .map_err(|_| SimulationError::UnknownRegister(register.name().to_string()))
            }
            ArchType::Arm32 => {
                let uc_reg = self.map_arm_register(register)?;
                self.engine
                    .reg_read(uc_reg)
                    .map_err(|_| SimulationError::UnknownRegister(register.name().to_string()))
            }
            ArchType::Arm64 => {
                let uc_reg = self.map_arm64_register(register)?;
                self.engine
                    .reg_read(uc_reg)
                    .map_err(|_| SimulationError::UnknownRegister(register.name().to_string()))
            }
            _ => Err(SimulationError::UnsupportedFeature(format!(
                "Architecture {:?} not supported",
                self.architecture.arch_type
            ))),
        }
    }

    /// Write a register value
    pub fn write_register(&mut self, register: &Register, value: u64) -> SimulationResult<()> {
        match self.architecture.arch_type {
            ArchType::X86 | ArchType::X86_64 => {
                let uc_reg = self.map_x86_register(register)?;
                self.engine
                    .reg_write(uc_reg, value)
                    .map_err(|_| SimulationError::UnknownRegister(register.name().to_string()))
            }
            ArchType::Arm32 => {
                let uc_reg = self.map_arm_register(register)?;
                self.engine
                    .reg_write(uc_reg, value)
                    .map_err(|_| SimulationError::UnknownRegister(register.name().to_string()))
            }
            ArchType::Arm64 => {
                let uc_reg = self.map_arm64_register(register)?;
                self.engine
                    .reg_write(uc_reg, value)
                    .map_err(|_| SimulationError::UnknownRegister(register.name().to_string()))
            }
            _ => Err(SimulationError::UnsupportedFeature(format!(
                "Architecture {:?} not supported",
                self.architecture.arch_type
            ))),
        }
    }

    /// Execute code from start to end address
    pub fn execute(&mut self, start: u64, end: u64, timeout: Option<u64>) -> SimulationResult<()> {
        let timeout = timeout.unwrap_or(10 * SECOND_SCALE);
        self.engine
            .emu_start(start, end, timeout, 0)
            .map_err(|e| SimulationError::InvalidOperation(format!("Execution failed: {:?}", e)))
    }

    /// Stop execution
    pub fn stop(&mut self) -> SimulationResult<()> {
        self.engine
            .emu_stop()
            .map_err(|e| SimulationError::InvalidOperation(format!("Failed to stop: {:?}", e)))
    }

    /// Save current context with a label
    pub fn save_context(&mut self, label: String) -> SimulationResult<()> {
        let context = self.engine.context_init().map_err(|e| {
            SimulationError::InvalidOperation(format!("Failed to save context: {:?}", e))
        })?;
        self.contexts.insert(label, context);
        Ok(())
    }

    /// Restore a saved context
    pub fn restore_context(&mut self, label: &str) -> SimulationResult<()> {
        let context = self.contexts.get(label).ok_or_else(|| {
            SimulationError::InvalidOperation(format!("Context '{}' not found", label))
        })?;
        self.engine.context_restore(context).map_err(|e| {
            SimulationError::InvalidOperation(format!("Failed to restore context: {:?}", e))
        })
    }

    /// Map x86/x86_64 register names
    fn map_x86_register(&self, register: &Register) -> SimulationResult<RegisterX86> {
        // Handle FPU registers
        let name = register.name();
        if name.starts_with("st") && name.len() == 3 {
            if let Ok(index) = name[2..].parse::<u8>() {
                if index < 8 {
                    return Ok(match index {
                        0 => RegisterX86::ST0,
                        1 => RegisterX86::ST1,
                        2 => RegisterX86::ST2,
                        3 => RegisterX86::ST3,
                        4 => RegisterX86::ST4,
                        5 => RegisterX86::ST5,
                        6 => RegisterX86::ST6,
                        7 => RegisterX86::ST7,
                        _ => unreachable!(),
                    });
                }
            }
        }

        // Map common registers
        let uc_reg = match register.name() {
            // 64-bit general purpose
            "rax" => RegisterX86::RAX,
            "rbx" => RegisterX86::RBX,
            "rcx" => RegisterX86::RCX,
            "rdx" => RegisterX86::RDX,
            "rsi" => RegisterX86::RSI,
            "rdi" => RegisterX86::RDI,
            "rbp" => RegisterX86::RBP,
            "rsp" => RegisterX86::RSP,
            "r8" => RegisterX86::R8,
            "r9" => RegisterX86::R9,
            "r10" => RegisterX86::R10,
            "r11" => RegisterX86::R11,
            "r12" => RegisterX86::R12,
            "r13" => RegisterX86::R13,
            "r14" => RegisterX86::R14,
            "r15" => RegisterX86::R15,
            "rip" => RegisterX86::RIP,

            // 32-bit general purpose
            "eax" => RegisterX86::EAX,
            "ebx" => RegisterX86::EBX,
            "ecx" => RegisterX86::ECX,
            "edx" => RegisterX86::EDX,
            "esi" => RegisterX86::ESI,
            "edi" => RegisterX86::EDI,
            "ebp" => RegisterX86::EBP,
            "esp" => RegisterX86::ESP,

            // 16-bit general purpose
            "ax" => RegisterX86::AX,
            "bx" => RegisterX86::BX,
            "cx" => RegisterX86::CX,
            "dx" => RegisterX86::DX,

            // 8-bit general purpose
            "al" => RegisterX86::AL,
            "ah" => RegisterX86::AH,
            "bl" => RegisterX86::BL,
            "bh" => RegisterX86::BH,
            "cl" => RegisterX86::CL,
            "ch" => RegisterX86::CH,
            "dl" => RegisterX86::DL,
            "dh" => RegisterX86::DH,

            // Flags
            "rflags" => RegisterX86::RFLAGS,
            "eflags" => RegisterX86::EFLAGS,

            // FPU control/status
            "fpsw" => RegisterX86::FPSW,

            _ => {
                return Err(SimulationError::UnknownRegister(
                    register.name().to_string(),
                ));
            }
        };

        Ok(uc_reg)
    }

    /// Map ARM32 register names
    fn map_arm_register(&self, register: &Register) -> SimulationResult<RegisterARM> {
        let uc_reg = match register.name() {
            "r0" => RegisterARM::R0,
            "r1" => RegisterARM::R1,
            "r2" => RegisterARM::R2,
            "r3" => RegisterARM::R3,
            "r4" => RegisterARM::R4,
            "r5" => RegisterARM::R5,
            "r6" => RegisterARM::R6,
            "r7" => RegisterARM::R7,
            "r8" => RegisterARM::R8,
            "r9" => RegisterARM::R9,
            "r10" => RegisterARM::R10,
            "r11" => RegisterARM::R11,
            "r12" => RegisterARM::R12,
            "sp" | "r13" => RegisterARM::SP,
            "lr" | "r14" => RegisterARM::LR,
            "pc" | "r15" => RegisterARM::PC,
            "cpsr" => RegisterARM::CPSR,
            _ => {
                return Err(SimulationError::UnknownRegister(
                    register.name().to_string(),
                ));
            }
        };

        Ok(uc_reg)
    }

    /// Map ARM64 register names
    fn map_arm64_register(&self, register: &Register) -> SimulationResult<RegisterARM64> {
        let uc_reg = match register.name() {
            // General purpose registers
            "x0" => RegisterARM64::X0,
            "x1" => RegisterARM64::X1,
            "x2" => RegisterARM64::X2,
            "x3" => RegisterARM64::X3,
            "x4" => RegisterARM64::X4,
            "x5" => RegisterARM64::X5,
            "x6" => RegisterARM64::X6,
            "x7" => RegisterARM64::X7,
            "x8" => RegisterARM64::X8,
            "x9" => RegisterARM64::X9,
            "x10" => RegisterARM64::X10,
            "x11" => RegisterARM64::X11,
            "x12" => RegisterARM64::X12,
            "x13" => RegisterARM64::X13,
            "x14" => RegisterARM64::X14,
            "x15" => RegisterARM64::X15,
            "x16" => RegisterARM64::X16,
            "x17" => RegisterARM64::X17,
            "x18" => RegisterARM64::X18,
            "x19" => RegisterARM64::X19,
            "x20" => RegisterARM64::X20,
            "x21" => RegisterARM64::X21,
            "x22" => RegisterARM64::X22,
            "x23" => RegisterARM64::X23,
            "x24" => RegisterARM64::X24,
            "x25" => RegisterARM64::X25,
            "x26" => RegisterARM64::X26,
            "x27" => RegisterARM64::X27,
            "x28" => RegisterARM64::X28,
            "x29" | "fp" => RegisterARM64::X29,
            "x30" | "lr" => RegisterARM64::X30,
            "sp" => RegisterARM64::SP,
            "pc" => RegisterARM64::PC,
            _ => {
                return Err(SimulationError::UnknownRegister(
                    register.name().to_string(),
                ));
            }
        };

        Ok(uc_reg)
    }

    /// Add a code execution hook
    pub fn add_code_hook<F>(&mut self, callback: F) -> SimulationResult<()>
    where
        F: FnMut(&mut Unicorn<()>, u64, u32) + 'static,
    {
        self.engine.add_code_hook(1, 0, callback).map_err(|e| {
            SimulationError::InvalidOperation(format!("Failed to add hook: {:?}", e))
        })?;
        Ok(())
    }

    /// Add a memory read hook
    pub fn add_mem_read_hook<F>(&mut self, callback: F) -> SimulationResult<()>
    where
        F: FnMut(&mut Unicorn<()>, unicorn_engine::MemType, u64, usize, i64) -> bool + 'static,
    {
        self.engine
            .add_mem_hook(unicorn_engine::HookType::MEM_READ, 1, 0, callback)
            .map_err(|e| {
                SimulationError::InvalidOperation(format!("Failed to add hook: {:?}", e))
            })?;
        Ok(())
    }

    /// Add a memory write hook
    pub fn add_mem_write_hook<F>(&mut self, callback: F) -> SimulationResult<()>
    where
        F: FnMut(&mut Unicorn<()>, unicorn_engine::MemType, u64, usize, i64) -> bool + 'static,
    {
        self.engine
            .add_mem_hook(unicorn_engine::HookType::MEM_WRITE, 1, 0, callback)
            .map_err(|e| {
                SimulationError::InvalidOperation(format!("Failed to add hook: {:?}", e))
            })?;
        Ok(())
    }

    /// Get the architecture info
    pub fn architecture(&self) -> &ArchitectureInfo {
        &self.architecture
    }
}

/// Helper to create standard x86_64 memory layout
pub fn create_x86_64_memory_layout(sim: &mut UnicornSimulator) -> SimulationResult<()> {
    // Code segment
    sim.map_memory(0x400000, 0x100000, Permission::READ | Permission::EXEC)?;

    // Data segment
    sim.map_memory(0x600000, 0x100000, Permission::READ | Permission::WRITE)?;

    // Stack (grows down from high address)
    sim.map_memory(0x7fff0000, 0x10000, Permission::READ | Permission::WRITE)?;

    // Set initial stack pointer
    sim.write_register(&Register::new("rsp", 0..64), 0x7ffff000)?;

    Ok(())
}

/// Helper to create standard ARM memory layout
pub fn create_arm_memory_layout(sim: &mut UnicornSimulator) -> SimulationResult<()> {
    // Code segment
    sim.map_memory(0x8000, 0x8000, Permission::READ | Permission::EXEC)?;

    // Data segment
    sim.map_memory(0x10000, 0x8000, Permission::READ | Permission::WRITE)?;

    // Stack
    sim.map_memory(0x70000, 0x10000, Permission::READ | Permission::WRITE)?;

    // Set initial stack pointer
    let sp_reg = if sim.architecture().pointer_size == 64 {
        Register::new("sp", 0..64)
    } else {
        Register::new("sp", 0..32)
    };
    sim.write_register(&sp_reg, 0x7fff0)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicorn_simulator_creation() {
        let arch_info = ArchitectureInfo {
            arch_type: ArchType::X86_64,
            pointer_size: 64,
            endianness: Endianness::Little,
            register_count: 16,
            instruction_alignment: 1,
        };

        let sim = UnicornSimulator::new(arch_info);
        assert!(sim.is_ok());
    }

    #[test]
    fn test_memory_operations() {
        let arch_info = ArchitectureInfo {
            arch_type: ArchType::X86_64,
            pointer_size: 64,
            endianness: Endianness::Little,
            register_count: 16,
            instruction_alignment: 1,
        };

        let mut sim = UnicornSimulator::new(arch_info).unwrap();

        // Map memory
        assert!(sim.map_memory(0x1000, 0x1000, Permission::ALL).is_ok());

        // Write and read
        let data = vec![0x41, 0x42, 0x43, 0x44];
        assert!(sim.write_memory(0x1000, &data).is_ok());

        let read_data = sim.read_memory(0x1000, 4).unwrap();
        assert_eq!(read_data, data);
    }
}

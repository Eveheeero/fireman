//! Architecture-specific calling convention support
//!
//! This module provides comprehensive calling convention support across different
//! architectures and platforms, handling parameter passing, return values, and
//! register preservation rules.

use crate::arch::architecture::{ArchType, OperatingSystem};
use crate::ir::low_ir::CallConv;

/// Calling convention parameter type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    /// Integer or pointer type
    Integer,
    /// Floating-point type
    Float,
    /// Double precision floating-point
    Double,
    /// Vector/SIMD type
    Vector,
    /// Aggregate type (struct/union)
    Aggregate { size: usize },
}

/// How a parameter is passed
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamLocation {
    /// Passed in a register
    Register(&'static str),
    /// Passed in a floating-point register
    FloatRegister(&'static str),
    /// Passed on the stack
    Stack { offset: i32 },
    /// Split between registers and stack
    Split {
        registers: Vec<&'static str>,
        stack_bytes: usize,
    },
}

/// Detailed calling convention information
#[derive(Debug, Clone)]
pub struct CallingConventionInfo {
    /// Name of the calling convention
    pub name: &'static str,
    /// Registers used for integer/pointer arguments (in order)
    pub int_arg_registers: Vec<&'static str>,
    /// Registers used for floating-point arguments (in order)
    pub float_arg_registers: Vec<&'static str>,
    /// Register(s) used for return values
    pub return_registers: Vec<&'static str>,
    /// Floating-point return registers
    pub float_return_registers: Vec<&'static str>,
    /// Caller-saved registers (volatile)
    pub caller_saved: Vec<&'static str>,
    /// Callee-saved registers (non-volatile)
    pub callee_saved: Vec<&'static str>,
    /// Stack alignment requirement in bytes
    pub stack_alignment: usize,
    /// Parameter alignment on stack
    pub param_stack_alignment: usize,
    /// Stack grows down (true) or up (false)
    pub stack_grows_down: bool,
    /// Shadow space size (Windows x64)
    pub shadow_space: usize,
    /// Red zone size (System V x64)
    pub red_zone_size: usize,
    /// Whether to clean up stack after call (caller vs callee cleanup)
    pub caller_cleanup: bool,
    /// Maximum aggregate size passed in registers
    pub max_aggregate_in_regs: usize,
}

impl CallingConventionInfo {
    /// Get parameter location for a specific parameter
    pub fn get_param_location(
        &self,
        _param_index: usize,
        param_type: ParamType,
        used_int_regs: &mut usize,
        used_float_regs: &mut usize,
        stack_offset: &mut i32,
    ) -> ParamLocation {
        match param_type {
            ParamType::Integer => {
                if *used_int_regs < self.int_arg_registers.len() {
                    let reg = self.int_arg_registers[*used_int_regs];
                    *used_int_regs += 1;
                    ParamLocation::Register(reg)
                } else {
                    let offset = *stack_offset;
                    *stack_offset += self.param_stack_alignment as i32;
                    ParamLocation::Stack { offset }
                }
            }
            ParamType::Float | ParamType::Double => {
                if *used_float_regs < self.float_arg_registers.len() {
                    let reg = self.float_arg_registers[*used_float_regs];
                    *used_float_regs += 1;
                    ParamLocation::FloatRegister(reg)
                } else {
                    let offset = *stack_offset;
                    *stack_offset += if param_type == ParamType::Double {
                        8
                    } else {
                        4
                    };
                    ParamLocation::Stack { offset }
                }
            }
            ParamType::Vector => {
                // Most calling conventions pass vectors in float registers
                if *used_float_regs < self.float_arg_registers.len() {
                    let reg = self.float_arg_registers[*used_float_regs];
                    *used_float_regs += 1;
                    ParamLocation::FloatRegister(reg)
                } else {
                    let offset = *stack_offset;
                    *stack_offset += 16; // Most vectors are 128-bit
                    ParamLocation::Stack { offset }
                }
            }
            ParamType::Aggregate { size } => {
                if size <= self.max_aggregate_in_regs
                    && *used_int_regs < self.int_arg_registers.len()
                {
                    // Small aggregates might be passed in registers
                    let regs_needed = size.div_ceil(8); // Round up to 8-byte chunks
                    if *used_int_regs + regs_needed <= self.int_arg_registers.len() {
                        let mut registers = Vec::new();
                        for _ in 0..regs_needed {
                            registers.push(self.int_arg_registers[*used_int_regs]);
                            *used_int_regs += 1;
                        }
                        ParamLocation::Split {
                            registers,
                            stack_bytes: 0,
                        }
                    } else {
                        // Pass on stack
                        let offset = *stack_offset;
                        *stack_offset += size as i32;
                        ParamLocation::Stack { offset }
                    }
                } else {
                    // Large aggregates always on stack
                    let offset = *stack_offset;
                    *stack_offset += size as i32;
                    ParamLocation::Stack { offset }
                }
            }
        }
    }
}

/// Trait for getting calling convention information
pub trait CallingConventionProvider {
    /// Get calling convention info for a specific convention
    fn get_convention_info(&self, conv: CallConv) -> CallingConventionInfo;

    /// Get the default calling convention for this architecture/platform
    fn get_default_convention(&self) -> CallConv;

    /// Check if a calling convention is supported
    fn is_supported(&self, conv: CallConv) -> bool;
}

/// x86 (32-bit) calling conventions
pub struct X86CallingConventions {
    os: OperatingSystem,
}

impl X86CallingConventions {
    pub fn new(os: OperatingSystem) -> Self {
        Self { os }
    }
}

impl CallingConventionProvider for X86CallingConventions {
    fn get_convention_info(&self, conv: CallConv) -> CallingConventionInfo {
        match conv {
            CallConv::C => CallingConventionInfo {
                name: "cdecl",
                int_arg_registers: vec![], // All params on stack
                float_arg_registers: vec![],
                return_registers: vec!["eax", "edx"], // edx:eax for 64-bit returns
                float_return_registers: vec!["st0"],  // x87 FPU stack
                caller_saved: vec!["eax", "ecx", "edx"],
                callee_saved: vec!["ebx", "esi", "edi", "ebp"],
                stack_alignment: 4,
                param_stack_alignment: 4,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 0,
                caller_cleanup: true,
                max_aggregate_in_regs: 0,
            },
            CallConv::X86Stdcall => CallingConventionInfo {
                name: "stdcall",
                int_arg_registers: vec![], // All params on stack
                float_arg_registers: vec![],
                return_registers: vec!["eax", "edx"],
                float_return_registers: vec!["st0"],
                caller_saved: vec!["eax", "ecx", "edx"],
                callee_saved: vec!["ebx", "esi", "edi", "ebp"],
                stack_alignment: 4,
                param_stack_alignment: 4,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 0,
                caller_cleanup: false, // Callee cleans up
                max_aggregate_in_regs: 0,
            },
            CallConv::X86Fastcall => CallingConventionInfo {
                name: "fastcall",
                int_arg_registers: vec!["ecx", "edx"], // First 2 params in registers
                float_arg_registers: vec![],
                return_registers: vec!["eax", "edx"],
                float_return_registers: vec!["st0"],
                caller_saved: vec!["eax", "ecx", "edx"],
                callee_saved: vec!["ebx", "esi", "edi", "ebp"],
                stack_alignment: 4,
                param_stack_alignment: 4,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 0,
                caller_cleanup: false,
                max_aggregate_in_regs: 0,
            },
            CallConv::X86Thiscall => CallingConventionInfo {
                name: "thiscall",
                int_arg_registers: vec!["ecx"], // this pointer in ECX
                float_arg_registers: vec![],
                return_registers: vec!["eax", "edx"],
                float_return_registers: vec!["st0"],
                caller_saved: vec!["eax", "ecx", "edx"],
                callee_saved: vec!["ebx", "esi", "edi", "ebp"],
                stack_alignment: 4,
                param_stack_alignment: 4,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 0,
                caller_cleanup: false,
                max_aggregate_in_regs: 0,
            },
            CallConv::X86Vectorcall => CallingConventionInfo {
                name: "vectorcall",
                int_arg_registers: vec!["ecx", "edx"], // First 2 integer params
                float_arg_registers: vec!["xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5"], // Up to 6 vector params
                return_registers: vec!["eax", "edx"],
                float_return_registers: vec!["xmm0"],
                caller_saved: vec!["eax", "ecx", "edx"],
                callee_saved: vec!["ebx", "esi", "edi", "ebp"],
                stack_alignment: 4,
                param_stack_alignment: 4,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 0,
                caller_cleanup: false,
                max_aggregate_in_regs: 0,
            },
            CallConv::Arm64AapcsDarwin => CallingConventionInfo {
                name: "AAPCS64 Darwin",
                int_arg_registers: vec!["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"],
                float_arg_registers: vec!["v0", "v1", "v2", "v3", "v4", "v5", "v6", "v7"],
                return_registers: vec!["x0", "x1"],
                float_return_registers: vec!["v0", "v1", "v2", "v3"],
                caller_saved: vec![
                    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11",
                    "x12", "x13", "x14", "x15", "x16", "x17",
                ],
                callee_saved: vec![
                    "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28", "x29",
                    "x30",
                ],
                stack_alignment: 16,
                param_stack_alignment: 8,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 128, // Darwin has red zone
                caller_cleanup: true,
                max_aggregate_in_regs: 16,
            },
            _ => self.get_convention_info(CallConv::Arm64Aapcs), // Fallback
        }
    }

    fn get_default_convention(&self) -> CallConv {
        match self.os {
            OperatingSystem::Windows => CallConv::X86Stdcall,
            _ => CallConv::C,
        }
    }

    fn is_supported(&self, conv: CallConv) -> bool {
        matches!(
            conv,
            CallConv::C
                | CallConv::X86Stdcall
                | CallConv::X86Fastcall
                | CallConv::X86Thiscall
                | CallConv::X86Vectorcall
        )
    }
}

/// x86_64 calling conventions
pub struct X86_64CallingConventions {
    os: OperatingSystem,
}

impl X86_64CallingConventions {
    pub fn new(os: OperatingSystem) -> Self {
        Self { os }
    }
}

impl CallingConventionProvider for X86_64CallingConventions {
    fn get_convention_info(&self, conv: CallConv) -> CallingConventionInfo {
        match conv {
            CallConv::X86_64SysV => CallingConventionInfo {
                name: "System V AMD64 ABI",
                int_arg_registers: vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"],
                float_arg_registers: vec![
                    "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7",
                ],
                return_registers: vec!["rax", "rdx"],
                float_return_registers: vec!["xmm0", "xmm1"],
                caller_saved: vec!["rax", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11"],
                callee_saved: vec!["rbx", "rbp", "r12", "r13", "r14", "r15"],
                stack_alignment: 16,
                param_stack_alignment: 8,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 128,
                caller_cleanup: true,
                max_aggregate_in_regs: 16,
            },
            CallConv::X86_64Win64 => CallingConventionInfo {
                name: "Microsoft x64",
                int_arg_registers: vec!["rcx", "rdx", "r8", "r9"],
                float_arg_registers: vec!["xmm0", "xmm1", "xmm2", "xmm3"],
                return_registers: vec!["rax"],
                float_return_registers: vec!["xmm0"],
                caller_saved: vec!["rax", "rcx", "rdx", "r8", "r9", "r10", "r11"],
                callee_saved: vec!["rbx", "rbp", "rdi", "rsi", "r12", "r13", "r14", "r15"],
                stack_alignment: 16,
                param_stack_alignment: 8,
                stack_grows_down: true,
                shadow_space: 32, // 4 * 8 bytes
                red_zone_size: 0,
                caller_cleanup: true,
                max_aggregate_in_regs: 8,
            },
            CallConv::C => {
                // Default to platform-specific convention
                match self.os {
                    OperatingSystem::Windows => self.get_convention_info(CallConv::X86_64Win64),
                    _ => self.get_convention_info(CallConv::X86_64SysV),
                }
            }
            CallConv::PreserveAll => CallingConventionInfo {
                name: "preserve_all",
                int_arg_registers: vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"],
                float_arg_registers: vec![
                    "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7",
                ],
                return_registers: vec!["rax", "rdx"],
                float_return_registers: vec!["xmm0", "xmm1"],
                caller_saved: vec![], // All registers preserved
                callee_saved: vec![
                    "rbx", "rbp", "r12", "r13", "r14", "r15", "rdi", "rsi", "rdx", "rcx", "r8",
                    "r9", "r10", "r11", "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6",
                    "xmm7", "xmm8", "xmm9", "xmm10", "xmm11", "xmm12", "xmm13", "xmm14", "xmm15",
                ],
                stack_alignment: 16,
                param_stack_alignment: 8,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 128,
                caller_cleanup: true,
                max_aggregate_in_regs: 16,
            },
            CallConv::PreserveMost => CallingConventionInfo {
                name: "preserve_most",
                int_arg_registers: vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"],
                float_arg_registers: vec![
                    "xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7",
                ],
                return_registers: vec!["rax", "rdx"],
                float_return_registers: vec!["xmm0", "xmm1"],
                caller_saved: vec!["rax", "rdx", "rcx", "rsi", "rdi", "r8", "r9", "r10", "r11"],
                callee_saved: vec![
                    "rbx", "rbp", "r12", "r13", "r14", "r15", "xmm8", "xmm9", "xmm10", "xmm11",
                    "xmm12", "xmm13", "xmm14", "xmm15",
                ],
                stack_alignment: 16,
                param_stack_alignment: 8,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 128,
                caller_cleanup: true,
                max_aggregate_in_regs: 16,
            },
            _ => self.get_convention_info(CallConv::X86_64SysV), // Fallback
        }
    }

    fn get_default_convention(&self) -> CallConv {
        match self.os {
            OperatingSystem::Windows => CallConv::X86_64Win64,
            _ => CallConv::X86_64SysV,
        }
    }

    fn is_supported(&self, conv: CallConv) -> bool {
        matches!(
            conv,
            CallConv::C
                | CallConv::X86_64SysV
                | CallConv::X86_64Win64
                | CallConv::PreserveAll
                | CallConv::PreserveMost
        )
    }
}

/// ARM32 calling conventions
pub struct Arm32CallingConventions {
    os: OperatingSystem,
}

impl Arm32CallingConventions {
    pub fn new(os: OperatingSystem) -> Self {
        Self { os }
    }
}

impl CallingConventionProvider for Arm32CallingConventions {
    fn get_convention_info(&self, conv: CallConv) -> CallingConventionInfo {
        match conv {
            CallConv::C | CallConv::ArmAapcs => CallingConventionInfo {
                name: "AAPCS (ARM)",
                int_arg_registers: vec!["r0", "r1", "r2", "r3"],
                float_arg_registers: vec!["s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7"],
                return_registers: vec!["r0", "r1"], // r1:r0 for 64-bit
                float_return_registers: vec!["s0", "s1"], // or d0 for double
                caller_saved: vec!["r0", "r1", "r2", "r3", "r12"],
                callee_saved: vec!["r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11"],
                stack_alignment: 8,
                param_stack_alignment: 4,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 0,
                caller_cleanup: true,
                max_aggregate_in_regs: 16,
            },
            CallConv::Arm64AapcsDarwin => CallingConventionInfo {
                name: "AAPCS64 Darwin",
                int_arg_registers: vec!["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"],
                float_arg_registers: vec!["v0", "v1", "v2", "v3", "v4", "v5", "v6", "v7"],
                return_registers: vec!["x0", "x1"],
                float_return_registers: vec!["v0", "v1", "v2", "v3"],
                caller_saved: vec![
                    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11",
                    "x12", "x13", "x14", "x15", "x16", "x17",
                ],
                callee_saved: vec![
                    "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28", "x29",
                    "x30",
                ],
                stack_alignment: 16,
                param_stack_alignment: 8,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 128, // Darwin has red zone
                caller_cleanup: true,
                max_aggregate_in_regs: 16,
            },
            _ => self.get_convention_info(CallConv::Arm64Aapcs),
        }
    }

    fn get_default_convention(&self) -> CallConv {
        CallConv::C // AAPCS is the standard
    }

    fn is_supported(&self, conv: CallConv) -> bool {
        matches!(
            conv,
            CallConv::C | CallConv::ArmAapcs | CallConv::ArmAapcsVfp
        )
    }
}

/// ARM64 calling conventions
pub struct Arm64CallingConventions {
    os: OperatingSystem,
}

impl Arm64CallingConventions {
    pub fn new(os: OperatingSystem) -> Self {
        Self { os }
    }
}

impl CallingConventionProvider for Arm64CallingConventions {
    fn get_convention_info(&self, conv: CallConv) -> CallingConventionInfo {
        match conv {
            CallConv::C | CallConv::Arm64Aapcs => CallingConventionInfo {
                name: "AAPCS64",
                int_arg_registers: vec!["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"],
                float_arg_registers: vec!["v0", "v1", "v2", "v3", "v4", "v5", "v6", "v7"],
                return_registers: vec!["x0", "x1"], // x1:x0 for 128-bit
                float_return_registers: vec!["v0", "v1", "v2", "v3"], // Up to 4 vectors
                caller_saved: vec![
                    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11",
                    "x12", "x13", "x14", "x15", "x16", "x17", "x18",
                ],
                callee_saved: vec![
                    "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28", "x29",
                    "x30",
                ],
                stack_alignment: 16,
                param_stack_alignment: 8,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 0,
                caller_cleanup: true,
                max_aggregate_in_regs: 16,
            },
            CallConv::Arm64AapcsDarwin => CallingConventionInfo {
                name: "AAPCS64 Darwin",
                int_arg_registers: vec!["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"],
                float_arg_registers: vec!["v0", "v1", "v2", "v3", "v4", "v5", "v6", "v7"],
                return_registers: vec!["x0", "x1"],
                float_return_registers: vec!["v0", "v1", "v2", "v3"],
                caller_saved: vec![
                    "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11",
                    "x12", "x13", "x14", "x15", "x16", "x17",
                ],
                callee_saved: vec![
                    "x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26", "x27", "x28", "x29",
                    "x30",
                ],
                stack_alignment: 16,
                param_stack_alignment: 8,
                stack_grows_down: true,
                shadow_space: 0,
                red_zone_size: 128, // Darwin has red zone
                caller_cleanup: true,
                max_aggregate_in_regs: 16,
            },
            _ => self.get_convention_info(CallConv::Arm64Aapcs),
        }
    }

    fn get_default_convention(&self) -> CallConv {
        CallConv::C // AAPCS64 is the standard
    }

    fn is_supported(&self, conv: CallConv) -> bool {
        matches!(
            conv,
            CallConv::C | CallConv::Arm64Aapcs | CallConv::Arm64AapcsDarwin
        )
    }
}

/// Get a calling convention provider for a specific architecture
pub fn get_calling_convention_provider(
    arch: ArchType,
    os: OperatingSystem,
) -> Box<dyn CallingConventionProvider> {
    match arch {
        ArchType::X86 => Box::new(X86CallingConventions::new(os)),
        ArchType::X86_64 => Box::new(X86_64CallingConventions::new(os)),
        ArchType::Arm32 => Box::new(Arm32CallingConventions::new(os)),
        ArchType::Arm64 => Box::new(Arm64CallingConventions::new(os)),
        _ => Box::new(X86_64CallingConventions::new(os)), // Default fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x86_cdecl() {
        let provider = X86CallingConventions::new(OperatingSystem::Linux);
        let info = provider.get_convention_info(CallConv::C);

        assert_eq!(info.name, "cdecl");
        assert!(info.int_arg_registers.is_empty());
        assert!(info.caller_cleanup);
        assert_eq!(info.stack_alignment, 4);
    }

    #[test]
    fn test_x86_64_sysv() {
        let provider = X86_64CallingConventions::new(OperatingSystem::Linux);
        let info = provider.get_convention_info(CallConv::X86_64SysV);

        assert_eq!(info.name, "System V AMD64 ABI");
        assert_eq!(info.int_arg_registers.len(), 6);
        assert_eq!(info.int_arg_registers[0], "rdi");
        assert_eq!(info.float_arg_registers.len(), 8);
        assert_eq!(info.red_zone_size, 128);
        assert_eq!(info.shadow_space, 0);
    }

    #[test]
    fn test_x86_64_win64() {
        let provider = X86_64CallingConventions::new(OperatingSystem::Windows);
        let info = provider.get_convention_info(CallConv::X86_64Win64);

        assert_eq!(info.name, "Microsoft x64");
        assert_eq!(info.int_arg_registers.len(), 4);
        assert_eq!(info.int_arg_registers[0], "rcx");
        assert_eq!(info.shadow_space, 32);
        assert_eq!(info.red_zone_size, 0);
    }

    #[test]
    fn test_param_location_x64_sysv() {
        let provider = X86_64CallingConventions::new(OperatingSystem::Linux);
        let info = provider.get_convention_info(CallConv::X86_64SysV);

        let mut used_int = 0;
        let mut used_float = 0;
        let mut stack_offset = 0;

        // First integer param -> rdi
        let loc1 = info.get_param_location(
            0,
            ParamType::Integer,
            &mut used_int,
            &mut used_float,
            &mut stack_offset,
        );
        assert_eq!(loc1, ParamLocation::Register("rdi"));

        // First float param -> xmm0
        let loc2 = info.get_param_location(
            1,
            ParamType::Float,
            &mut used_int,
            &mut used_float,
            &mut stack_offset,
        );
        assert_eq!(loc2, ParamLocation::FloatRegister("xmm0"));

        // 7th integer param -> stack
        used_int = 6; // All int registers used
        let loc3 = info.get_param_location(
            7,
            ParamType::Integer,
            &mut used_int,
            &mut used_float,
            &mut stack_offset,
        );
        assert!(matches!(loc3, ParamLocation::Stack { .. }));
    }

    #[test]
    fn test_arm64_calling_convention() {
        let provider = Arm64CallingConventions::new(OperatingSystem::Linux);
        let info = provider.get_convention_info(CallConv::C);

        assert_eq!(info.name, "AAPCS64");
        assert_eq!(info.int_arg_registers.len(), 8);
        assert_eq!(info.int_arg_registers[0], "x0");
        assert_eq!(info.float_arg_registers[0], "v0");
        assert_eq!(info.stack_alignment, 16);
    }
}

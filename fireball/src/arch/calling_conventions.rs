//! Architecture-specific calling conventions
//!
//! This module handles calling conventions for different architectures and platforms,
//! including parameter passing, return values, and callee/caller-saved registers.

use crate::arch::{ArchType, ArchitectureInfo};
use crate::ir::data::IrData;
use crate::ir::register::Register;
use crate::utils::Aos;
use std::collections::BTreeMap;

/// Platform types that affect calling conventions
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Platform {
    Linux,
    Windows,
    MacOS,
    BSD,
    Unknown,
}

/// Calling convention types
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CallingConvention {
    // x86 conventions
    Cdecl,      // C declaration - default for x86
    Stdcall,    // Windows API
    Fastcall,   // Microsoft fastcall
    Thiscall,   // C++ member functions

    // x86_64 conventions
    SystemV,    // Unix/Linux x86_64 ABI
    Win64,      // Windows x64 ABI

    // ARM conventions
    AAPCS,      // ARM Architecture Procedure Call Standard
    AAPCS_VFP,  // AAPCS with VFP (hard-float)

    // Generic
    Unknown,
}

/// Information about how parameters are passed
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterLocation {
    /// Register used for this parameter (if any)
    pub register: Option<&'static str>,
    /// Stack offset for this parameter (if any)
    pub stack_offset: Option<i64>,
    /// Size of the parameter in bytes
    pub size: usize,
}

/// Calling convention information
#[derive(Debug, Clone)]
pub struct ConventionInfo {
    /// Name of the convention
    pub name: CallingConvention,
    /// Registers used for integer parameters (in order)
    pub int_param_regs: Vec<&'static str>,
    /// Registers used for floating-point parameters
    pub float_param_regs: Vec<&'static str>,
    /// Register(s) used for return values
    pub return_regs: Vec<&'static str>,
    /// Callee-saved registers (must be preserved)
    pub callee_saved: Vec<&'static str>,
    /// Caller-saved registers (may be clobbered)
    pub caller_saved: Vec<&'static str>,
    /// Stack alignment requirement in bytes
    pub stack_alignment: usize,
    /// Direction of stack growth (true = down, false = up)
    pub stack_grows_down: bool,
    /// Whether to use a frame pointer
    pub uses_frame_pointer: bool,
    /// Frame pointer register
    pub frame_pointer: Option<&'static str>,
    /// Stack pointer register
    pub stack_pointer: &'static str,
}

/// Calling convention analyzer
pub struct CallingConventionAnalyzer {
    /// Convention mappings by architecture and platform
    conventions: BTreeMap<(ArchType, Platform), ConventionInfo>,
}

impl CallingConventionAnalyzer {
    /// Create a new calling convention analyzer
    pub fn new() -> Self {
        let mut conventions = BTreeMap::new();

        // Initialize x86_64 conventions
        Self::init_x86_64_conventions(&mut conventions);

        // Initialize x86 conventions
        Self::init_x86_conventions(&mut conventions);

        // Initialize ARM64 conventions
        Self::init_arm64_conventions(&mut conventions);

        // Initialize ARM32 conventions
        Self::init_arm32_conventions(&mut conventions);

        Self { conventions }
    }

    fn init_x86_64_conventions(conventions: &mut BTreeMap<(ArchType, Platform), ConventionInfo>) {
        // System V AMD64 ABI (Linux, macOS, BSD)
        let sysv = ConventionInfo {
            name: CallingConvention::SystemV,
            int_param_regs: vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"],
            float_param_regs: vec!["xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7"],
            return_regs: vec!["rax", "rdx"], // rdx for 128-bit returns
            callee_saved: vec!["rbx", "rsp", "rbp", "r12", "r13", "r14", "r15"],
            caller_saved: vec!["rax", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11"],
            stack_alignment: 16,
            stack_grows_down: true,
            uses_frame_pointer: false, // Optional in System V
            frame_pointer: Some("rbp"),
            stack_pointer: "rsp",
        };

        conventions.insert((ArchType::X86_64, Platform::Linux), sysv.clone());
        conventions.insert((ArchType::X86_64, Platform::MacOS), sysv.clone());
        conventions.insert((ArchType::X86_64, Platform::BSD), sysv);

        // Windows x64 ABI
        let win64 = ConventionInfo {
            name: CallingConvention::Win64,
            int_param_regs: vec!["rcx", "rdx", "r8", "r9"],
            float_param_regs: vec!["xmm0", "xmm1", "xmm2", "xmm3"],
            return_regs: vec!["rax"],
            callee_saved: vec!["rbx", "rsp", "rbp", "rdi", "rsi", "r12", "r13", "r14", "r15"],
            caller_saved: vec!["rax", "rcx", "rdx", "r8", "r9", "r10", "r11"],
            stack_alignment: 16,
            stack_grows_down: true,
            uses_frame_pointer: false,
            frame_pointer: Some("rbp"),
            stack_pointer: "rsp",
        };

        conventions.insert((ArchType::X86_64, Platform::Windows), win64);
    }

    fn init_x86_conventions(conventions: &mut BTreeMap<(ArchType, Platform), ConventionInfo>) {
        // cdecl - standard C calling convention
        let cdecl = ConventionInfo {
            name: CallingConvention::Cdecl,
            int_param_regs: vec![], // All parameters on stack
            float_param_regs: vec![],
            return_regs: vec!["eax", "edx"], // edx:eax for 64-bit returns
            callee_saved: vec!["ebx", "esi", "edi", "ebp", "esp"],
            caller_saved: vec!["eax", "ecx", "edx"],
            stack_alignment: 4, // Some compilers use 16
            stack_grows_down: true,
            uses_frame_pointer: true,
            frame_pointer: Some("ebp"),
            stack_pointer: "esp",
        };

        conventions.insert((ArchType::X86, Platform::Linux), cdecl.clone());
        conventions.insert((ArchType::X86, Platform::MacOS), cdecl.clone());
        conventions.insert((ArchType::X86, Platform::BSD), cdecl);

        // stdcall - Windows API convention
        let stdcall = ConventionInfo {
            name: CallingConvention::Stdcall,
            int_param_regs: vec![], // All parameters on stack
            float_param_regs: vec![],
            return_regs: vec!["eax", "edx"],
            callee_saved: vec!["ebx", "esi", "edi", "ebp", "esp"],
            caller_saved: vec!["eax", "ecx", "edx"],
            stack_alignment: 4,
            stack_grows_down: true,
            uses_frame_pointer: true,
            frame_pointer: Some("ebp"),
            stack_pointer: "esp",
        };

        conventions.insert((ArchType::X86, Platform::Windows), stdcall);
    }

    fn init_arm64_conventions(conventions: &mut BTreeMap<(ArchType, Platform), ConventionInfo>) {
        // AAPCS64 - ARM 64-bit Architecture Procedure Call Standard
        let aapcs64 = ConventionInfo {
            name: CallingConvention::AAPCS,
            int_param_regs: vec!["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"],
            float_param_regs: vec!["v0", "v1", "v2", "v3", "v4", "v5", "v6", "v7"],
            return_regs: vec!["x0", "x1"], // x1:x0 for 128-bit returns
            callee_saved: vec!["x19", "x20", "x21", "x22", "x23", "x24", "x25", "x26",
                               "x27", "x28", "x29", "x30"], // x30 is LR
            caller_saved: vec!["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7",
                               "x8", "x9", "x10", "x11", "x12", "x13", "x14", "x15",
                               "x16", "x17", "x18"], // x16, x17 are IP0, IP1
            stack_alignment: 16,
            stack_grows_down: true,
            uses_frame_pointer: true,
            frame_pointer: Some("x29"),
            stack_pointer: "sp",
        };

        conventions.insert((ArchType::Arm64, Platform::Linux), aapcs64.clone());
        conventions.insert((ArchType::Arm64, Platform::MacOS), aapcs64.clone());
        conventions.insert((ArchType::Arm64, Platform::Windows), aapcs64.clone());
        conventions.insert((ArchType::Arm64, Platform::BSD), aapcs64);
    }

    fn init_arm32_conventions(conventions: &mut BTreeMap<(ArchType, Platform), ConventionInfo>) {
        // AAPCS - ARM Architecture Procedure Call Standard
        let aapcs = ConventionInfo {
            name: CallingConvention::AAPCS,
            int_param_regs: vec!["r0", "r1", "r2", "r3"],
            float_param_regs: vec![], // Soft-float by default
            return_regs: vec!["r0", "r1"], // r1:r0 for 64-bit returns
            callee_saved: vec!["r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11"],
            caller_saved: vec!["r0", "r1", "r2", "r3", "r12"], // r12 is IP
            stack_alignment: 8,
            stack_grows_down: true,
            uses_frame_pointer: true,
            frame_pointer: Some("r11"),
            stack_pointer: "sp",
        };

        conventions.insert((ArchType::Arm32, Platform::Linux), aapcs.clone());
        conventions.insert((ArchType::Arm32, Platform::MacOS), aapcs.clone());
        conventions.insert((ArchType::Arm32, Platform::Windows), aapcs.clone());
        conventions.insert((ArchType::Arm32, Platform::BSD), aapcs);
    }

    /// Get the calling convention for an architecture and platform
    pub fn get_convention(&self, arch: ArchType, platform: Platform) -> Option<&ConventionInfo> {
        self.conventions.get(&(arch, platform))
    }

    /// Detect platform from binary characteristics
    pub fn detect_platform(data: &[u8]) -> Platform {
        // Simple heuristics - could be expanded
        if data.starts_with(&[0x7F, 0x45, 0x4C, 0x46]) {
            // ELF format - likely Linux/BSD
            Platform::Linux
        } else if data.starts_with(&[0x4D, 0x5A]) {
            // PE format - Windows
            Platform::Windows
        } else if data.starts_with(&[0xFE, 0xED, 0xFA]) || data.starts_with(&[0xCE, 0xFA, 0xED]) {
            // Mach-O format - macOS
            Platform::MacOS
        } else {
            Platform::Unknown
        }
    }

    /// Get parameter locations for a function call
    pub fn get_parameter_locations(
        &self,
        arch: ArchType,
        platform: Platform,
        param_types: &[ParameterType],
    ) -> Vec<ParameterLocation> {
        let Some(convention) = self.get_convention(arch, platform) else {
            return vec![];
        };

        let mut locations = Vec::new();
        let mut int_reg_idx = 0;
        let mut float_reg_idx = 0;
        let mut stack_offset = 0i64;

        for param_type in param_types {
            match param_type {
                ParameterType::Integer(size) => {
                    if int_reg_idx < convention.int_param_regs.len() {
                        // Pass in register
                        locations.push(ParameterLocation {
                            register: Some(convention.int_param_regs[int_reg_idx]),
                            stack_offset: None,
                            size: *size,
                        });
                        int_reg_idx += 1;
                    } else {
                        // Pass on stack
                        locations.push(ParameterLocation {
                            register: None,
                            stack_offset: Some(stack_offset),
                            size: *size,
                        });
                        stack_offset += align_to(*size as i64, convention.stack_alignment as i64);
                    }
                }
                ParameterType::Float(size) => {
                    if float_reg_idx < convention.float_param_regs.len() {
                        // Pass in register
                        locations.push(ParameterLocation {
                            register: Some(convention.float_param_regs[float_reg_idx]),
                            stack_offset: None,
                            size: *size,
                        });
                        float_reg_idx += 1;
                    } else {
                        // Pass on stack
                        locations.push(ParameterLocation {
                            register: None,
                            stack_offset: Some(stack_offset),
                            size: *size,
                        });
                        stack_offset += align_to(*size as i64, convention.stack_alignment as i64);
                    }
                }
                ParameterType::Composite(size) => {
                    // Composite types often passed by reference or split across registers
                    // This is a simplified implementation
                    if *size <= 16 && int_reg_idx < convention.int_param_regs.len() {
                        locations.push(ParameterLocation {
                            register: Some(convention.int_param_regs[int_reg_idx]),
                            stack_offset: None,
                            size: *size,
                        });
                        int_reg_idx += (*size + 7) / 8; // Number of registers needed
                    } else {
                        locations.push(ParameterLocation {
                            register: None,
                            stack_offset: Some(stack_offset),
                            size: *size,
                        });
                        stack_offset += align_to(*size as i64, convention.stack_alignment as i64);
                    }
                }
            }
        }

        locations
    }

    /// Generate prologue for a function
    pub fn generate_prologue(
        &self,
        arch: ArchType,
        platform: Platform,
        local_size: usize,
    ) -> Vec<String> {
        let Some(convention) = self.get_convention(arch, platform) else {
            return vec![];
        };

        let mut prologue = Vec::new();

        // Push frame pointer if used
        if convention.uses_frame_pointer {
            if let Some(fp) = convention.frame_pointer {
                prologue.push(format!("push {}", fp));
                prologue.push(format!("mov {}, {}", fp, convention.stack_pointer));
            }
        }

        // Allocate local space
        if local_size > 0 {
            let aligned_size = align_to(local_size as i64, convention.stack_alignment as i64);
            prologue.push(format!("sub {}, {}", convention.stack_pointer, aligned_size));
        }

        // Save callee-saved registers (simplified)
        for reg in &convention.callee_saved {
            if *reg != convention.stack_pointer && Some(*reg) != convention.frame_pointer {
                prologue.push(format!("push {}", reg));
            }
        }

        prologue
    }

    /// Generate epilogue for a function
    pub fn generate_epilogue(
        &self,
        arch: ArchType,
        platform: Platform,
    ) -> Vec<String> {
        let Some(convention) = self.get_convention(arch, platform) else {
            return vec![];
        };

        let mut epilogue = Vec::new();

        // Restore callee-saved registers (in reverse order)
        for reg in convention.callee_saved.iter().rev() {
            if *reg != convention.stack_pointer && Some(*reg) != convention.frame_pointer {
                epilogue.push(format!("pop {}", reg));
            }
        }

        // Restore stack pointer
        if convention.uses_frame_pointer {
            if let Some(fp) = convention.frame_pointer {
                epilogue.push(format!("mov {}, {}", convention.stack_pointer, fp));
                epilogue.push(format!("pop {}", fp));
            }
        }

        epilogue.push("ret".to_string());
        epilogue
    }
}

/// Parameter type for calling convention analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
    Integer(usize),    // Size in bytes
    Float(usize),      // Size in bytes
    Composite(usize),  // Struct/union size in bytes
}

/// Helper function to align a value
fn align_to(value: i64, alignment: i64) -> i64 {
    (value + alignment - 1) & !(alignment - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x86_64_sysv_convention() {
        let analyzer = CallingConventionAnalyzer::new();
        let conv = analyzer.get_convention(ArchType::X86_64, Platform::Linux).unwrap();

        assert_eq!(conv.name, CallingConvention::SystemV);
        assert_eq!(conv.int_param_regs, vec!["rdi", "rsi", "rdx", "rcx", "r8", "r9"]);
        assert_eq!(conv.stack_alignment, 16);
    }

    #[test]
    fn test_parameter_locations() {
        let analyzer = CallingConventionAnalyzer::new();

        // Test System V x86_64 with mixed parameter types
        let params = vec![
            ParameterType::Integer(8),  // rdi
            ParameterType::Integer(4),  // rsi
            ParameterType::Float(8),    // xmm0
            ParameterType::Integer(8),  // rdx
        ];

        let locations = analyzer.get_parameter_locations(ArchType::X86_64, Platform::Linux, &params);

        assert_eq!(locations.len(), 4);
        assert_eq!(locations[0].register, Some("rdi"));
        assert_eq!(locations[1].register, Some("rsi"));
        assert_eq!(locations[2].register, Some("xmm0"));
        assert_eq!(locations[3].register, Some("rdx"));
    }

    #[test]
    fn test_platform_detection() {
        // ELF header
        let elf_data = &[0x7F, 0x45, 0x4C, 0x46];
        assert_eq!(CallingConventionAnalyzer::detect_platform(elf_data), Platform::Linux);

        // PE header
        let pe_data = &[0x4D, 0x5A];
        assert_eq!(CallingConventionAnalyzer::detect_platform(pe_data), Platform::Windows);
    }
}

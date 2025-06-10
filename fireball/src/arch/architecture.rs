//! Architecture abstraction and detection
//!
//! This module provides a unified interface for handling multiple architectures
//! and automatic detection of architecture from binary files.

use crate::core::{Instruction, Sections};
use crate::ir::{low_ir, statements::IrStatement};
use std::sync::Arc;

/// Architecture information and capabilities
#[derive(Debug, Clone)]
pub struct ArchitectureInfo {
    /// Architecture type
    pub arch_type: ArchType,
    /// Pointer size in bits (32 or 64)
    pub pointer_size: u8,
    /// Endianness
    pub endianness: Endianness,
    /// Register count
    pub register_count: usize,
    /// Instruction alignment requirement
    pub instruction_alignment: u8,
}

/// Endianness of the architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

/// Supported architecture types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchType {
    X86,
    X86_64,
    Arm32,
    Arm64,
    Unknown,
}

impl ArchType {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            ArchType::X86 => "x86",
            ArchType::X86_64 => "x86_64",
            ArchType::Arm32 => "arm32",
            ArchType::Arm64 => "arm64",
            ArchType::Unknown => "unknown",
        }
    }

    /// Get default pointer size for this architecture
    pub fn default_pointer_size(&self) -> u8 {
        match self {
            ArchType::X86 | ArchType::Arm32 => 32,
            ArchType::X86_64 | ArchType::Arm64 => 64,
            ArchType::Unknown => 64, // Default to 64-bit
        }
    }

    /// Get default endianness for this architecture
    pub fn default_endianness(&self) -> Endianness {
        match self {
            // Most common architectures are little-endian
            ArchType::X86 | ArchType::X86_64 | ArchType::Arm32 | ArchType::Arm64 => {
                Endianness::Little
            }
            ArchType::Unknown => Endianness::Little,
        }
    }

    /// Get instruction alignment requirement
    pub fn instruction_alignment(&self) -> u8 {
        match self {
            ArchType::X86 | ArchType::X86_64 => 1, // Variable length instructions
            ArchType::Arm32 => 4,                  // 32-bit aligned
            ArchType::Arm64 => 4,                  // 32-bit aligned
            ArchType::Unknown => 1,
        }
    }
}

/// Architecture detection from binary data
pub struct ArchitectureDetector;

impl ArchitectureDetector {
    /// Detect architecture from raw binary data
    pub fn detect_from_bytes(data: &[u8]) -> ArchitectureInfo {
        if data.len() < 4 {
            return ArchitectureInfo {
                arch_type: ArchType::Unknown,
                pointer_size: 64,
                endianness: Endianness::Little,
                register_count: 0,
                instruction_alignment: 1,
            };
        }

        // Check magic bytes to determine format
        match &data[0..4] {
            // PE format
            [0x4D, 0x5A, _, _] => Self::detect_from_pe(data),
            // ELF format
            [0x7F, 0x45, 0x4C, 0x46] => Self::detect_from_elf(data),
            // Mach-O format
            [0xFE, 0xED, 0xFA, 0xCE] | [0xCE, 0xFA, 0xED, 0xFE] => Self::detect_from_macho(data),
            [0xFE, 0xED, 0xFA, 0xCF] | [0xCF, 0xFA, 0xED, 0xFE] => Self::detect_from_macho(data),
            _ => ArchitectureInfo {
                arch_type: ArchType::Unknown,
                pointer_size: 64,
                endianness: Endianness::Little,
                register_count: 0,
                instruction_alignment: 1,
            },
        }
    }

    /// Detect architecture from PE file
    fn detect_from_pe(data: &[u8]) -> ArchitectureInfo {
        // Try to detect architecture from PE headers directly
        if data.len() < 0x40 {
            return ArchitectureInfo {
                arch_type: ArchType::Unknown,
                pointer_size: 64,
                endianness: Endianness::Little,
                register_count: 0,
                instruction_alignment: 1,
            };
        }

        // Get PE header offset from DOS header
        if data.len() > 0x3C + 4 {
            let pe_offset =
                u32::from_le_bytes([data[0x3C], data[0x3D], data[0x3E], data[0x3F]]) as usize;

            // Check PE signature
            if pe_offset + 24 < data.len() && &data[pe_offset..pe_offset + 4] == b"PE\0\0" {
                // Machine type at PE + 4
                let machine = u16::from_le_bytes([data[pe_offset + 4], data[pe_offset + 5]]);

                // Check optional header magic (determines 32/64 bit) at PE + 24
                let is_64bit = if pe_offset + 24 < data.len() {
                    let magic = u16::from_le_bytes([data[pe_offset + 24], data[pe_offset + 25]]);
                    magic == 0x020B // PE32+ (64-bit)
                } else {
                    false
                };

                let arch_type = match machine {
                    0x014C => ArchType::X86,    // IMAGE_FILE_MACHINE_I386
                    0x8664 => ArchType::X86_64, // IMAGE_FILE_MACHINE_AMD64
                    0x01C0 => ArchType::Arm32,  // IMAGE_FILE_MACHINE_ARM
                    0xAA64 => ArchType::Arm64,  // IMAGE_FILE_MACHINE_ARM64
                    _ => {
                        if is_64bit {
                            ArchType::X86_64
                        } else {
                            ArchType::X86
                        }
                    }
                };

                let pointer_size = match arch_type {
                    ArchType::X86_64 | ArchType::Arm64 => 64,
                    ArchType::X86 | ArchType::Arm32 => 32,
                    _ => {
                        if is_64bit {
                            64
                        } else {
                            32
                        }
                    }
                };

                return ArchitectureInfo {
                    arch_type,
                    pointer_size,
                    endianness: Endianness::Little, // PE is always little-endian
                    register_count: match arch_type {
                        ArchType::X86 => 8,     // General purpose registers
                        ArchType::X86_64 => 16, // Including R8-R15
                        ArchType::Arm32 => 16,  // R0-R15
                        ArchType::Arm64 => 31,  // X0-X30
                        _ => 0,
                    },
                    instruction_alignment: match arch_type {
                        ArchType::X86 | ArchType::X86_64 => 1,
                        ArchType::Arm32 | ArchType::Arm64 => 4,
                        _ => 1,
                    },
                };
            }
        }

        // If we can't parse, try goblin as fallback
        if let Ok(pe) = goblin::pe::PE::parse(data) {
            let arch_type = if pe.is_64 {
                ArchType::X86_64
            } else {
                ArchType::X86
            };

            let pointer_size = if pe.is_64 { 64 } else { 32 };

            ArchitectureInfo {
                arch_type,
                pointer_size,
                endianness: Endianness::Little,
                register_count: match arch_type {
                    ArchType::X86 => 8,
                    ArchType::X86_64 => 16,
                    _ => 0,
                },
                instruction_alignment: 1,
            }
        } else {
            // Default to unknown
            ArchitectureInfo {
                arch_type: ArchType::Unknown,
                pointer_size: 64,
                endianness: Endianness::Little,
                register_count: 0,
                instruction_alignment: 1,
            }
        }
    }

    /// Detect architecture from ELF file
    fn detect_from_elf(data: &[u8]) -> ArchitectureInfo {
        if data.len() < 20 {
            return ArchitectureInfo {
                arch_type: ArchType::Unknown,
                pointer_size: 64,
                endianness: Endianness::Little,
                register_count: 0,
                instruction_alignment: 1,
            };
        }

        // Check ELF class (32/64-bit)
        let is_64bit = data[4] == 2;
        let endianness = match data[5] {
            1 => Endianness::Little,
            2 => Endianness::Big,
            _ => Endianness::Little,
        };

        // Machine type is at offset 18
        let machine = if endianness == Endianness::Little {
            u16::from_le_bytes([data[18], data[19]])
        } else {
            u16::from_be_bytes([data[18], data[19]])
        };
        let arch_type = match machine {
            0x03 => {
                if is_64bit {
                    ArchType::X86_64
                } else {
                    ArchType::X86
                }
            }
            0x3E => ArchType::X86_64,
            0x28 => ArchType::Arm32,
            0xB7 => ArchType::Arm64,
            _ => ArchType::Unknown,
        };

        let pointer_size = if is_64bit { 64 } else { 32 };

        ArchitectureInfo {
            arch_type,
            pointer_size,
            endianness,
            register_count: match arch_type {
                ArchType::X86 => 8,
                ArchType::X86_64 => 16,
                ArchType::Arm32 => 16,
                ArchType::Arm64 => 31, // X0-X30
                ArchType::Unknown => 0,
            },
            instruction_alignment: arch_type.instruction_alignment(),
        }
    }

    /// Detect architecture from Mach-O file
    fn detect_from_macho(data: &[u8]) -> ArchitectureInfo {
        if data.len() < 8 {
            return ArchitectureInfo {
                arch_type: ArchType::Unknown,
                pointer_size: 64,
                endianness: Endianness::Little,
                register_count: 0,
                instruction_alignment: 1,
            };
        }

        // Check magic for 64-bit and endianness
        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let (is_64bit, endianness) = match magic {
            0xfeedface => (false, Endianness::Little), // MH_MAGIC
            0xcefaedfe => (false, Endianness::Big),    // MH_CIGAM
            0xfeedfacf => (true, Endianness::Little),  // MH_MAGIC_64
            0xcffaedfe => (true, Endianness::Big),     // MH_CIGAM_64
            _ => (true, Endianness::Little),
        };

        // CPU type is at offset 4
        let cpu_type = if endianness == Endianness::Little {
            u32::from_le_bytes([data[4], data[5], data[6], data[7]])
        } else {
            u32::from_be_bytes([data[4], data[5], data[6], data[7]])
        };

        let arch_type = match cpu_type {
            0x00000007 => ArchType::X86,
            0x01000007 => ArchType::X86_64,
            0x0000000C => ArchType::Arm32,
            0x0100000C => ArchType::Arm64,
            _ => ArchType::Unknown,
        };

        let pointer_size = if is_64bit { 64 } else { 32 };

        ArchitectureInfo {
            arch_type,
            pointer_size,
            endianness,
            register_count: match arch_type {
                ArchType::X86 => 8,
                ArchType::X86_64 => 16,
                ArchType::Arm32 => 16,
                ArchType::Arm64 => 31,
                ArchType::Unknown => 0,
            },
            instruction_alignment: arch_type.instruction_alignment(),
        }
    }
}

/// Architecture context for instruction analysis
pub struct ArchitectureContext {
    pub info: ArchitectureInfo,
    pub sections: Arc<Sections>,
}

impl ArchitectureContext {
    /// Create a new architecture context
    pub fn new(info: ArchitectureInfo, sections: Arc<Sections>) -> Self {
        Self { info, sections }
    }

    /// Convert instruction to IR based on architecture
    pub fn instruction_to_ir(&self, instruction: &Instruction) -> Option<&'static [IrStatement]> {
        match self.info.arch_type {
            ArchType::X86_64 => {
                // Use existing x86_64 instruction analyzer
                crate::arch::x86_64::instruction_analyze::create_ir_statement(instruction)
            }
            ArchType::X86 => {
                // TODO: Implement x86 instruction analyzer
                None
            }
            ArchType::Arm32 => {
                // TODO: Implement ARM32 instruction analyzer
                None
            }
            ArchType::Arm64 => {
                // TODO: Implement ARM64 instruction analyzer
                None
            }
            ArchType::Unknown => None,
        }
    }

    /// Get Low IR target info for this architecture
    pub fn get_target_info(&self) -> low_ir::TargetInfo {
        low_ir::TargetInfo {
            arch: match self.info.arch_type {
                ArchType::X86 => "x86".to_string(),
                ArchType::X86_64 => "x86_64".to_string(),
                ArchType::Arm32 => "arm32".to_string(),
                ArchType::Arm64 => "arm64".to_string(),
                ArchType::Unknown => "unknown".to_string(),
            },
            bits: self.info.pointer_size as u32,
            endian: match self.info.endianness {
                Endianness::Little => low_ir::Endianness::Little,
                Endianness::Big => low_ir::Endianness::Big,
            },
        }
    }
}

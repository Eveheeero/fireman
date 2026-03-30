//! Module containing implementations for multiple architectures

use crate::{core::Instruction, ir::statements::IrStatement};
use capstone::arch::BuildsCapstone;
pub(crate) use iceball::MachineArchitecture;
use std::pin::Pin;

pub mod x86_64;

pub(crate) fn from_pe_machine(machine: u16, is_64: bool) -> MachineArchitecture {
    match machine {
        goblin::pe::header::COFF_MACHINE_X86_64 => MachineArchitecture::X64,
        goblin::pe::header::COFF_MACHINE_ARM | goblin::pe::header::COFF_MACHINE_ARMNT => {
            MachineArchitecture::Arm
        }
        goblin::pe::header::COFF_MACHINE_ARM64 => MachineArchitecture::Arm64,
        _ if is_64 => MachineArchitecture::X64,
        _ => MachineArchitecture::X86,
    }
}

pub(crate) fn from_elf_machine(e_machine: u16, is_64: bool) -> MachineArchitecture {
    match e_machine {
        goblin::elf::header::EM_X86_64 => MachineArchitecture::X64,
        goblin::elf::header::EM_386 => MachineArchitecture::X86,
        goblin::elf::header::EM_ARM => MachineArchitecture::Arm,
        goblin::elf::header::EM_AARCH64 => MachineArchitecture::Arm64,
        _ if is_64 => MachineArchitecture::X64,
        _ => MachineArchitecture::X86,
    }
}

pub(crate) fn from_mach_cputype(cputype: u32) -> Option<MachineArchitecture> {
    match cputype {
        goblin::mach::cputype::CPU_TYPE_X86_64 => Some(MachineArchitecture::X64),
        goblin::mach::cputype::CPU_TYPE_X86 => Some(MachineArchitecture::X86),
        goblin::mach::cputype::CPU_TYPE_ARM => Some(MachineArchitecture::Arm),
        goblin::mach::cputype::CPU_TYPE_ARM64 => Some(MachineArchitecture::Arm64),
        _ => None,
    }
}

pub(crate) fn build_capstone(
    architecture: MachineArchitecture,
) -> Result<Pin<Box<capstone::Capstone>>, capstone::Error> {
    let capstone = match architecture {
        MachineArchitecture::X86 => capstone::Capstone::new()
            .x86()
            .mode(capstone::arch::x86::ArchMode::Mode32)
            .build()?,
        MachineArchitecture::X64 => capstone::Capstone::new()
            .x86()
            .mode(capstone::arch::x86::ArchMode::Mode64)
            .build()?,
        MachineArchitecture::Arm => capstone::Capstone::new()
            .arm()
            .mode(capstone::arch::arm::ArchMode::Arm)
            .build()?,
        MachineArchitecture::Arm64 => capstone::Capstone::new()
            .arm64()
            .mode(capstone::arch::arm64::ArchMode::Arm)
            .build()?,
    };
    Ok(Box::pin(capstone))
}

pub(crate) fn create_ir_statement(
    architecture: MachineArchitecture,
    instruction: &Instruction,
) -> Option<&'static [IrStatement]> {
    match architecture {
        MachineArchitecture::X86 | MachineArchitecture::X64 => {
            x86_64::instruction_analyze::create_ir_statement(instruction)
        }
        MachineArchitecture::Arm | MachineArchitecture::Arm64 => None,
    }
}

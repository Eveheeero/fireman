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

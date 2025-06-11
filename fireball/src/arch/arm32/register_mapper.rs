//! ARM32 register mapping utilities

use crate::arch::arm32::register::str_to_arm32_register;
use crate::ir::data::IrData;
use crate::utils::Aos;

/// Convert an iceball ARM32 register to IrData
pub fn arm32_register_to_ir_data(reg: &iceball::Register) -> IrData {
    match reg {
        // Core registers
        iceball::Register::Arm32(iceball::Arm32Register::R0) => {
            str_to_arm32_register("r0").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R1) => {
            str_to_arm32_register("r1").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R2) => {
            str_to_arm32_register("r2").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R3) => {
            str_to_arm32_register("r3").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R4) => {
            str_to_arm32_register("r4").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R5) => {
            str_to_arm32_register("r5").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R6) => {
            str_to_arm32_register("r6").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R7) => {
            str_to_arm32_register("r7").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R8) => {
            str_to_arm32_register("r8").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R9) => {
            str_to_arm32_register("r9").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R10) => {
            str_to_arm32_register("r10").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R11) => {
            str_to_arm32_register("r11").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R12) => {
            str_to_arm32_register("r12").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R13)
        | iceball::Register::Arm32(iceball::Arm32Register::SP) => {
            str_to_arm32_register("sp").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R14)
        | iceball::Register::Arm32(iceball::Arm32Register::LR) => {
            str_to_arm32_register("lr").as_ref().clone()
        }
        iceball::Register::Arm32(iceball::Arm32Register::R15)
        | iceball::Register::Arm32(iceball::Arm32Register::PC) => {
            str_to_arm32_register("pc").as_ref().clone()
        }

        // Status registers
        iceball::Register::Arm32(iceball::Arm32Register::CPSR) => {
            str_to_arm32_register("cpsr").as_ref().clone()
        }

        // TODO: Handle other registers (banked, FP/NEON)
        _ => IrData::Constant(0), // Placeholder
    }
}

/// Create an IrData register reference for ARM32
pub fn arm32_register(name: &str) -> Aos<IrData> {
    str_to_arm32_register(name)
}

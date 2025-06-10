//! ARM32 instruction analysis and decoding

use super::{Arm32Error, Condition, InstructionSet};
use crate::core::{Address, Instruction as CoreInstruction};
use crate::ir::statements::IrStatement;

/// ARM32 instruction analyzer
pub struct Arm32Analyzer {
    /// Current instruction set mode
    mode: InstructionSet,
}

impl Arm32Analyzer {
    /// Create a new ARM32 analyzer
    pub fn new(mode: InstructionSet) -> Self {
        Self { mode }
    }

    /// Analyze instruction at given address
    pub fn analyze_instruction(
        &self,
        data: &[u8],
        address: &Address,
    ) -> Result<CoreInstruction, Arm32Error> {
        match self.mode {
            InstructionSet::ARM => self.analyze_arm_instruction(data, address),
            InstructionSet::Thumb => self.analyze_thumb_instruction(data, address),
            InstructionSet::Thumb2 => self.analyze_thumb2_instruction(data, address),
        }
    }

    /// Analyze ARM mode (32-bit) instruction
    fn analyze_arm_instruction(
        &self,
        data: &[u8],
        address: &Address,
    ) -> Result<CoreInstruction, Arm32Error> {
        if data.len() < 4 {
            return Err(Arm32Error::InvalidEncoding(0));
        }

        // ARM instructions are 32-bit little-endian
        let encoding = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

        // Extract condition code (bits 28-31)
        let cond_bits = (encoding >> 28) & 0xF;
        let condition = match cond_bits {
            0x0 => Condition::EQ,
            0x1 => Condition::NE,
            0x2 => Condition::CS,
            0x3 => Condition::CC,
            0x4 => Condition::MI,
            0x5 => Condition::PL,
            0x6 => Condition::VS,
            0x7 => Condition::VC,
            0x8 => Condition::HI,
            0x9 => Condition::LS,
            0xA => Condition::GE,
            0xB => Condition::LT,
            0xC => Condition::GT,
            0xD => Condition::LE,
            0xE => Condition::AL,
            0xF => Condition::NV,
            _ => unreachable!(),
        };

        // Decode instruction based on encoding pattern
        let ir_statements = self.decode_arm_instruction(encoding, condition)?;

        // TODO: Create proper CoreInstruction with mnemonic, operands, etc.
        // For now, return a placeholder
        Err(Arm32Error::Unimplemented(
            "ARM instruction decoding".to_string(),
        ))
    }

    /// Analyze Thumb mode (16-bit) instruction
    fn analyze_thumb_instruction(
        &self,
        data: &[u8],
        address: &Address,
    ) -> Result<CoreInstruction, Arm32Error> {
        if data.len() < 2 {
            return Err(Arm32Error::InvalidEncoding(0));
        }

        // Thumb instructions are 16-bit little-endian
        let encoding = u16::from_le_bytes([data[0], data[1]]);

        // TODO: Implement Thumb instruction decoding
        Err(Arm32Error::Unimplemented(
            "Thumb instruction decoding".to_string(),
        ))
    }

    /// Analyze Thumb-2 mode (mixed 16/32-bit) instruction
    fn analyze_thumb2_instruction(
        &self,
        data: &[u8],
        address: &Address,
    ) -> Result<CoreInstruction, Arm32Error> {
        if data.len() < 2 {
            return Err(Arm32Error::InvalidEncoding(0));
        }

        // Check if this is a 32-bit Thumb-2 instruction
        let first_halfword = u16::from_le_bytes([data[0], data[1]]);
        let is_32bit = match first_halfword >> 11 {
            0b11101 | 0b11110 | 0b11111 => true,
            _ => false,
        };

        if is_32bit {
            if data.len() < 4 {
                return Err(Arm32Error::InvalidEncoding(first_halfword as u32));
            }
            // 32-bit Thumb-2 instruction
            let encoding = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            // TODO: Decode 32-bit Thumb-2
            return Err(Arm32Error::Unimplemented(
                "Thumb-2 32-bit instruction decoding".to_string(),
            ));
        } else {
            // 16-bit Thumb instruction
            self.analyze_thumb_instruction(data, address)
        }
    }

    /// Decode ARM instruction to IR statements
    fn decode_arm_instruction(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Vec<IrStatement>, Arm32Error> {
        // Extract instruction type from bits 25-27
        let op_type = (encoding >> 25) & 0x7;

        match op_type {
            0b000 | 0b001 => {
                // Data processing instructions
                self.decode_data_processing(encoding, condition)
            }
            0b010 | 0b011 => {
                // Load/store immediate offset
                self.decode_load_store_immediate(encoding, condition)
            }
            0b100 => {
                // Load/store multiple
                self.decode_load_store_multiple(encoding, condition)
            }
            0b101 => {
                // Branch and branch with link
                self.decode_branch(encoding, condition)
            }
            _ => Err(Arm32Error::UnsupportedInstruction(format!(
                "Unknown instruction type: {:03b}",
                op_type
            ))),
        }
    }

    /// Decode data processing instructions (ADD, SUB, MOV, etc.)
    fn decode_data_processing(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Vec<IrStatement>, Arm32Error> {
        let opcode = (encoding >> 21) & 0xF;
        let set_flags = ((encoding >> 20) & 1) != 0;
        let rn = ((encoding >> 16) & 0xF) as u8;
        let rd = ((encoding >> 12) & 0xF) as u8;

        // TODO: Decode operand2 (register or immediate)
        // TODO: Generate appropriate IR statements

        Err(Arm32Error::Unimplemented(
            "Data processing decoding".to_string(),
        ))
    }

    /// Decode load/store with immediate offset
    fn decode_load_store_immediate(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Vec<IrStatement>, Arm32Error> {
        let is_load = ((encoding >> 20) & 1) != 0;
        let is_byte = ((encoding >> 22) & 1) != 0;
        let add_offset = ((encoding >> 23) & 1) != 0;
        let pre_index = ((encoding >> 24) & 1) != 0;

        let rn = ((encoding >> 16) & 0xF) as u8;
        let rt = ((encoding >> 12) & 0xF) as u8;
        let imm12 = (encoding & 0xFFF) as u16;

        // TODO: Generate load/store IR statements

        Err(Arm32Error::Unimplemented(
            "Load/store immediate decoding".to_string(),
        ))
    }

    /// Decode load/store multiple
    fn decode_load_store_multiple(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Vec<IrStatement>, Arm32Error> {
        let is_load = ((encoding >> 20) & 1) != 0;
        let writeback = ((encoding >> 21) & 1) != 0;
        let user_mode = ((encoding >> 22) & 1) != 0;
        let increment = ((encoding >> 23) & 1) != 0;
        let before = ((encoding >> 24) & 1) != 0;

        let rn = ((encoding >> 16) & 0xF) as u8;
        let register_list = (encoding & 0xFFFF) as u16;

        // TODO: Generate multiple load/store IR statements

        Err(Arm32Error::Unimplemented(
            "Load/store multiple decoding".to_string(),
        ))
    }

    /// Decode branch instructions
    fn decode_branch(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Vec<IrStatement>, Arm32Error> {
        let link = ((encoding >> 24) & 1) != 0;

        // Sign-extend 24-bit offset to 32-bit
        let offset = ((encoding & 0xFFFFFF) as i32) << 8 >> 6; // Shift left 2 for word alignment

        // TODO: Generate branch IR statement

        Err(Arm32Error::Unimplemented("Branch decoding".to_string()))
    }
}

/// Common ARM32 instruction opcodes
pub enum Arm32Opcode {
    // Data processing
    And,
    Eor,
    Sub,
    Rsb,
    Add,
    Adc,
    Sbc,
    Rsc,
    Tst,
    Teq,
    Cmp,
    Cmn,
    Orr,
    Mov,
    Bic,
    Mvn,

    // Multiply
    Mul,
    Mla,
    Umull,
    Umlal,
    Smull,
    Smlal,

    // Load/Store
    Ldr,
    Ldrb,
    Ldrh,
    Ldrsb,
    Ldrsh,
    Str,
    Strb,
    Strh,

    // Load/Store multiple
    Ldm,
    Stm,
    Push,
    Pop,

    // Branch
    B,
    Bl,
    Bx,
    Blx,

    // System
    Swi,
    Bkpt,

    // Thumb-specific
    Adr,
    Cbz,
    Cbnz,
    It,
}

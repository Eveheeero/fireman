//! ARM32 instruction decoder that produces iceball Instructions

use super::{Arm32Error, Condition, InstructionSet};
use crate::core::Address;
use iceball::{Architecture, Argument, Instruction};

/// ARM32 instruction decoder
pub struct Arm32Decoder {
    /// Current instruction set mode
    mode: InstructionSet,
}

impl Arm32Decoder {
    /// Create a new ARM32 decoder
    pub fn new(mode: InstructionSet) -> Self {
        Self { mode }
    }

    /// Decode instruction at given address
    pub fn decode_instruction(
        &self,
        data: &[u8],
        address: &Address,
    ) -> Result<(Instruction, usize), Arm32Error> {
        match self.mode {
            InstructionSet::ARM => self.decode_arm_instruction(data, address),
            InstructionSet::Thumb => self.decode_thumb_instruction(data, address),
            InstructionSet::Thumb2 => self.decode_thumb2_instruction(data, address),
        }
    }

    /// Decode ARM mode (32-bit) instruction
    fn decode_arm_instruction(
        &self,
        data: &[u8],
        _address: &Address,
    ) -> Result<(Instruction, usize), Arm32Error> {
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
        let instruction = self.decode_arm_encoding(encoding, condition)?;

        Ok((instruction, 4))
    }

    /// Decode Thumb mode (16-bit) instruction
    fn decode_thumb_instruction(
        &self,
        data: &[u8],
        _address: &Address,
    ) -> Result<(Instruction, usize), Arm32Error> {
        if data.len() < 2 {
            return Err(Arm32Error::InvalidEncoding(0));
        }

        // Thumb instructions are 16-bit little-endian
        let encoding = u16::from_le_bytes([data[0], data[1]]);

        // Decode based on Thumb encoding patterns
        let instruction = self.decode_thumb_encoding(encoding)?;

        Ok((instruction, 2))
    }

    /// Decode Thumb-2 mode (mixed 16/32-bit) instruction
    fn decode_thumb2_instruction(
        &self,
        data: &[u8],
        address: &Address,
    ) -> Result<(Instruction, usize), Arm32Error> {
        if data.len() < 2 {
            return Err(Arm32Error::InvalidEncoding(0));
        }

        // Check if this is a 32-bit Thumb-2 instruction
        let first_halfword = u16::from_le_bytes([data[0], data[1]]);
        let is_32bit = match first_halfword >> 11 {
            0b11101..=0b11111 => true,
            _ => false,
        };

        if is_32bit {
            if data.len() < 4 {
                return Err(Arm32Error::InvalidEncoding(first_halfword as u32));
            }
            // 32-bit Thumb-2 instruction
            let encoding = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
            let instruction = self.decode_thumb2_32bit_encoding(encoding)?;
            Ok((instruction, 4))
        } else {
            // 16-bit Thumb instruction
            self.decode_thumb_instruction(data, address)
        }
    }

    /// Decode ARM encoding to instruction
    fn decode_arm_encoding(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Instruction, Arm32Error> {
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
            _ => {
                // More complex encoding - check bits 4-7 and 20-27
                if (encoding & 0x0F000000) == 0x0F000000 {
                    // Software interrupt
                    self.decode_software_interrupt(encoding)
                } else if (encoding & 0x0E000090) == 0x00000090 {
                    // Multiply
                    self.decode_multiply(encoding, condition)
                } else {
                    Err(Arm32Error::UnsupportedInstruction(format!(
                        "Unknown instruction encoding: 0x{:08x}",
                        encoding
                    )))
                }
            }
        }
    }

    /// Decode data processing instructions (ADD, SUB, MOV, etc.)
    fn decode_data_processing(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Instruction, Arm32Error> {
        let opcode = (encoding >> 21) & 0xF;
        let set_flags = ((encoding >> 20) & 1) != 0;
        let rn = ((encoding >> 16) & 0xF) as u8;
        let rd = ((encoding >> 12) & 0xF) as u8;
        let immediate = ((encoding >> 25) & 1) != 0;

        // Determine mnemonic
        let base_mnemonic = match opcode {
            0x0 => "and",
            0x1 => "eor",
            0x2 => "sub",
            0x3 => "rsb",
            0x4 => "add",
            0x5 => "adc",
            0x6 => "sbc",
            0x7 => "rsc",
            0x8 => "tst",
            0x9 => "teq",
            0xA => "cmp",
            0xB => "cmn",
            0xC => "orr",
            0xD => "mov",
            0xE => "bic",
            0xF => "mvn",
            _ => unreachable!(),
        };

        // Add condition suffix if not AL (always)
        let mnemonic = if condition != Condition::AL {
            format!("{}{}", base_mnemonic, condition_suffix(condition))
        } else {
            base_mnemonic.to_string()
        };

        // Add 'S' suffix if setting flags
        let mnemonic = if set_flags && !matches!(opcode, 0x8..=0xB) {
            format!("{}s", mnemonic)
        } else {
            mnemonic
        };

        // Parse statement
        let statement = iceball::parse_statement(Architecture::Arm32, &mnemonic)?;

        // Decode operands
        let mut arguments = Vec::new();

        // Some instructions don't have rd as destination
        match opcode {
            0x8..=0xB => {
                // TST, TEQ, CMP, CMN - no destination register
                arguments.push(create_register_argument(rn));
            }
            0xD | 0xF => {
                // MOV, MVN - no rn operand
                arguments.push(create_register_argument(rd));
            }
            _ => {
                // Normal data processing - rd, rn, operand2
                arguments.push(create_register_argument(rd));
                arguments.push(create_register_argument(rn));
            }
        }

        // Decode operand2
        if immediate {
            // Immediate operand
            let imm = encoding & 0xFF;
            let rotate = ((encoding >> 8) & 0xF) * 2;
            let value = imm.rotate_right(rotate);
            arguments.push(Argument::Constant(value as u64));
        } else {
            // Register operand with optional shift
            let rm = (encoding & 0xF) as u8;
            let shift = (encoding >> 4) & 0xFF;

            if shift == 0 {
                // No shift
                arguments.push(create_register_argument(rm));
            } else {
                // TODO: Handle shifted register operands properly
                arguments.push(create_register_argument(rm));
            }
        }

        Ok(Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Decode load/store with immediate offset
    fn decode_load_store_immediate(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Instruction, Arm32Error> {
        let is_load = ((encoding >> 20) & 1) != 0;
        let is_byte = ((encoding >> 22) & 1) != 0;
        let add_offset = ((encoding >> 23) & 1) != 0;
        let pre_index = ((encoding >> 24) & 1) != 0;
        let writeback = ((encoding >> 21) & 1) != 0;

        let rn = ((encoding >> 16) & 0xF) as u8;
        let rt = ((encoding >> 12) & 0xF) as u8;
        let imm12 = (encoding & 0xFFF) as u16;

        // Determine mnemonic
        let base_mnemonic = match (is_load, is_byte) {
            (true, false) => "ldr",
            (true, true) => "ldrb",
            (false, false) => "str",
            (false, true) => "strb",
        };

        // Add condition suffix
        let mnemonic = if condition != Condition::AL {
            format!("{}{}", base_mnemonic, condition_suffix(condition))
        } else {
            base_mnemonic.to_string()
        };

        let statement = iceball::parse_statement(Architecture::Arm32, &mnemonic)?;

        // Create arguments
        let mut arguments = Vec::new();

        // Destination/source register
        arguments.push(create_register_argument(rt));

        // Memory operand
        let mut mem_str = format!("[{}", register_name(rn));

        if (imm12 != 0 || !pre_index) && pre_index {
            // Pre-indexed: [Rn, #offset]
            if add_offset {
                mem_str.push_str(&format!(", #{}", imm12));
            } else {
                mem_str.push_str(&format!(", #-{}", imm12));
            }
        }

        mem_str.push(']');

        if !pre_index {
            // Post-indexed: [Rn], #offset
            if imm12 != 0 {
                if add_offset {
                    mem_str.push_str(&format!(", #{}", imm12));
                } else {
                    mem_str.push_str(&format!(", #-{}", imm12));
                }
            }
        }

        if writeback && pre_index {
            mem_str.push('!');
        }

        arguments.push(iceball::parse_argument(Architecture::Arm32, &mem_str)?);

        Ok(Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Decode load/store multiple
    fn decode_load_store_multiple(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Instruction, Arm32Error> {
        let is_load = ((encoding >> 20) & 1) != 0;
        let writeback = ((encoding >> 21) & 1) != 0;
        let user_mode = ((encoding >> 22) & 1) != 0;
        let increment = ((encoding >> 23) & 1) != 0;
        let before = ((encoding >> 24) & 1) != 0;

        let rn = ((encoding >> 16) & 0xF) as u8;
        let register_list = (encoding & 0xFFFF) as u16;

        // Determine addressing mode suffix
        let addr_mode = match (increment, before) {
            (true, true) => "ib",   // Increment Before
            (true, false) => "ia",  // Increment After
            (false, true) => "db",  // Decrement Before
            (false, false) => "da", // Decrement After
        };

        // Check for PUSH/POP aliases
        let (base_mnemonic, is_push_pop) = if rn == 13 {
            // SP
            if is_load && increment && !before {
                ("pop", true)
            } else if !is_load && !increment && before {
                ("push", true)
            } else {
                (if is_load { "ldm" } else { "stm" }, false)
            }
        } else {
            (if is_load { "ldm" } else { "stm" }, false)
        };

        // Build mnemonic
        let mut mnemonic = base_mnemonic.to_string();
        if !is_push_pop {
            mnemonic.push_str(addr_mode);
        }

        // Add condition suffix
        if condition != Condition::AL {
            mnemonic.push_str(condition_suffix(condition));
        }

        let statement = iceball::parse_statement(Architecture::Arm32, &mnemonic)?;

        // Create arguments
        let mut arguments = Vec::new();

        if !is_push_pop {
            // Base register with optional writeback
            let reg_str = if writeback {
                format!("{}!", register_name(rn))
            } else {
                register_name(rn).to_string()
            };
            arguments.push(iceball::parse_argument(Architecture::Arm32, &reg_str)?);
        }

        // Register list
        let mut reg_list = String::from("{");
        let mut first = true;

        for i in 0..16 {
            if (register_list & (1 << i)) != 0 {
                if !first {
                    reg_list.push_str(", ");
                }
                reg_list.push_str(register_name(i));
                first = false;
            }
        }

        reg_list.push('}');

        if user_mode {
            reg_list.push('^');
        }

        // For iceball, we need to parse this as individual register arguments
        // This is a simplification - proper implementation would handle register lists
        for i in 0..16 {
            if (register_list & (1 << i)) != 0 {
                arguments.push(create_register_argument(i));
            }
        }

        Ok(Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Decode branch instructions
    fn decode_branch(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Instruction, Arm32Error> {
        let link = ((encoding >> 24) & 1) != 0;

        // Sign-extend 24-bit offset to 32-bit
        let offset = ((encoding & 0xFFFFFF) as i32) << 8 >> 6; // Shift left 2 for word alignment

        // Determine mnemonic
        let base_mnemonic = if link { "bl" } else { "b" };

        // Add condition suffix
        let mnemonic = if condition != Condition::AL {
            format!("{}{}", base_mnemonic, condition_suffix(condition))
        } else {
            base_mnemonic.to_string()
        };

        let statement = iceball::parse_statement(Architecture::Arm32, &mnemonic)?;

        // Target address is PC + 8 + offset (due to ARM pipeline)
        // But for now we'll just store the offset
        let target = if offset >= 0 {
            Argument::Constant(offset as u64)
        } else {
            // TODO: Handle negative offsets properly
            Argument::Constant((-offset) as u64)
        };

        Ok(Instruction {
            statement: Ok(statement),
            arguments: vec![target].into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Decode multiply instructions
    fn decode_multiply(
        &self,
        encoding: u32,
        condition: Condition,
    ) -> Result<Instruction, Arm32Error> {
        let accumulate = ((encoding >> 21) & 1) != 0;
        let set_flags = ((encoding >> 20) & 1) != 0;
        let is_long = ((encoding >> 23) & 1) != 0;
        let is_signed = ((encoding >> 22) & 1) != 0;

        let rd = ((encoding >> 16) & 0xF) as u8;
        let rn = ((encoding >> 12) & 0xF) as u8;
        let rs = ((encoding >> 8) & 0xF) as u8;
        let rm = (encoding & 0xF) as u8;

        // Determine mnemonic
        let base_mnemonic = match (is_long, is_signed, accumulate) {
            (false, _, false) => "mul",
            (false, _, true) => "mla",
            (true, false, false) => "umull",
            (true, false, true) => "umlal",
            (true, true, false) => "smull",
            (true, true, true) => "smlal",
        };

        // Add condition suffix
        let mnemonic = if condition != Condition::AL {
            format!("{}{}", base_mnemonic, condition_suffix(condition))
        } else {
            base_mnemonic.to_string()
        };

        // Add 'S' suffix if setting flags
        let mnemonic = if set_flags {
            format!("{}s", mnemonic)
        } else {
            mnemonic
        };

        let statement = iceball::parse_statement(Architecture::Arm32, &mnemonic)?;

        // Create arguments
        let mut arguments = Vec::new();

        if is_long {
            // Long multiply: RdLo, RdHi, Rm, Rs
            arguments.push(create_register_argument(rn)); // RdLo
            arguments.push(create_register_argument(rd)); // RdHi
            arguments.push(create_register_argument(rm));
            arguments.push(create_register_argument(rs));
        } else if accumulate {
            // MLA: Rd, Rm, Rs, Rn
            arguments.push(create_register_argument(rd));
            arguments.push(create_register_argument(rm));
            arguments.push(create_register_argument(rs));
            arguments.push(create_register_argument(rn));
        } else {
            // MUL: Rd, Rm, Rs
            arguments.push(create_register_argument(rd));
            arguments.push(create_register_argument(rm));
            arguments.push(create_register_argument(rs));
        }

        Ok(Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Decode software interrupt
    fn decode_software_interrupt(&self, encoding: u32) -> Result<Instruction, Arm32Error> {
        let imm24 = encoding & 0xFFFFFF;

        let statement = iceball::parse_statement(Architecture::Arm32, "svc")?;

        Ok(Instruction {
            statement: Ok(statement),
            arguments: vec![Argument::Constant(imm24 as u64)].into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Decode Thumb 16-bit instruction
    fn decode_thumb_encoding(&self, encoding: u16) -> Result<Instruction, Arm32Error> {
        // Decode based on bits 13-15
        match encoding >> 13 {
            0b000 => {
                // Shift, add, subtract, move, compare
                if (encoding & 0x1800) == 0x1800 {
                    // Add/subtract
                    self.decode_thumb_addsub(encoding)
                } else {
                    // Logical shift
                    self.decode_thumb_shift(encoding)
                }
            }
            0b001 => {
                // Move/compare/add/subtract immediate
                self.decode_thumb_immediate(encoding)
            }
            0b010 => {
                // Data processing or special data/branch exchange
                if (encoding & 0xFC00) == 0x4000 {
                    // Data processing
                    self.decode_thumb_data_processing(encoding)
                } else if (encoding & 0xFC00) == 0x4400 {
                    // Special data processing
                    self.decode_thumb_special_data(encoding)
                } else if (encoding & 0xF800) == 0x4800 {
                    // Load from literal pool
                    self.decode_thumb_literal_load(encoding)
                } else {
                    // Load/store register offset
                    self.decode_thumb_load_store_register(encoding)
                }
            }
            0b011 => {
                // Load/store word/byte immediate offset
                self.decode_thumb_load_store_immediate(encoding)
            }
            0b100 => {
                // Load/store halfword or stack operations
                if (encoding & 0xF000) == 0x8000 {
                    // Load/store halfword
                    self.decode_thumb_load_store_halfword(encoding)
                } else {
                    // Stack operations or multiple load/store
                    self.decode_thumb_stack_ops(encoding)
                }
            }
            0b101 => {
                // Miscellaneous 16-bit instructions
                if (encoding & 0xF000) == 0xA000 {
                    // Add to PC or SP
                    self.decode_thumb_add_sp_pc(encoding)
                } else {
                    // Conditional branch, SVC, etc.
                    self.decode_thumb_misc(encoding)
                }
            }
            0b110 => {
                // Conditional branch or SVC
                if (encoding & 0xF000) == 0xD000 {
                    // Conditional branch
                    self.decode_thumb_conditional_branch(encoding)
                } else {
                    // Undefined or SVC
                    self.decode_thumb_svc(encoding)
                }
            }
            0b111 => {
                // Unconditional branch
                self.decode_thumb_unconditional_branch(encoding)
            }
            _ => Err(Arm32Error::InvalidEncoding(encoding as u32)),
        }
    }

    /// Decode Thumb add/subtract
    fn decode_thumb_addsub(&self, encoding: u16) -> Result<Instruction, Arm32Error> {
        let is_immediate = ((encoding >> 10) & 1) != 0;
        let is_subtract = ((encoding >> 9) & 1) != 0;
        let rm_or_imm = ((encoding >> 6) & 0x7) as u8;
        let rn = ((encoding >> 3) & 0x7) as u8;
        let rd = (encoding & 0x7) as u8;

        let mnemonic = if is_subtract { "sub" } else { "add" };
        let statement = iceball::parse_statement(Architecture::Arm32, mnemonic)?;

        let mut arguments = vec![create_register_argument(rd), create_register_argument(rn)];

        if is_immediate {
            arguments.push(Argument::Constant(rm_or_imm as u64));
        } else {
            arguments.push(create_register_argument(rm_or_imm));
        }

        Ok(Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Decode Thumb shift instructions
    fn decode_thumb_shift(&self, encoding: u16) -> Result<Instruction, Arm32Error> {
        let opcode = (encoding >> 11) & 0x3;
        let imm5 = ((encoding >> 6) & 0x1F) as u8;
        let rm = ((encoding >> 3) & 0x7) as u8;
        let rd = (encoding & 0x7) as u8;

        let mnemonic = match opcode {
            0b00 => "lsl",
            0b01 => "lsr",
            0b10 => "asr",
            _ => return Err(Arm32Error::InvalidEncoding(encoding as u32)),
        };

        let statement = iceball::parse_statement(Architecture::Arm32, mnemonic)?;

        let mut arguments = vec![create_register_argument(rd), create_register_argument(rm)];

        if imm5 != 0 || opcode != 0 {
            arguments.push(Argument::Constant(imm5 as u64));
        }

        Ok(Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Decode Thumb immediate operations
    fn decode_thumb_immediate(&self, encoding: u16) -> Result<Instruction, Arm32Error> {
        let opcode = (encoding >> 11) & 0x3;
        let rd = ((encoding >> 8) & 0x7) as u8;
        let imm8 = (encoding & 0xFF) as u8;

        let mnemonic = match opcode {
            0b00 => "mov",
            0b01 => "cmp",
            0b10 => "add",
            0b11 => "sub",
            _ => unreachable!(),
        };

        let statement = iceball::parse_statement(Architecture::Arm32, mnemonic)?;

        let arguments = match opcode {
            0b01 => vec![
                create_register_argument(rd),
                Argument::Constant(imm8 as u64),
            ],
            _ => vec![
                create_register_argument(rd),
                Argument::Constant(imm8 as u64),
            ],
        };

        Ok(Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Additional Thumb decoding methods would go here...
    /// For brevity, I'll include a few more key ones:

    /// Decode Thumb data processing
    fn decode_thumb_data_processing(&self, encoding: u16) -> Result<Instruction, Arm32Error> {
        let opcode = (encoding >> 6) & 0xF;
        let rm = ((encoding >> 3) & 0x7) as u8;
        let rd = (encoding & 0x7) as u8;

        let mnemonic = match opcode {
            0x0 => "and",
            0x1 => "eor",
            0x2 => "lsl",
            0x3 => "lsr",
            0x4 => "asr",
            0x5 => "adc",
            0x6 => "sbc",
            0x7 => "ror",
            0x8 => "tst",
            0x9 => "neg",
            0xA => "cmp",
            0xB => "cmn",
            0xC => "orr",
            0xD => "mul",
            0xE => "bic",
            0xF => "mvn",
            _ => unreachable!(),
        };

        let statement = iceball::parse_statement(Architecture::Arm32, mnemonic)?;

        let arguments = match opcode {
            0x8 | 0xA | 0xB => vec![create_register_argument(rd), create_register_argument(rm)],
            0x9 => vec![create_register_argument(rd), create_register_argument(rm)],
            _ => vec![create_register_argument(rd), create_register_argument(rm)],
        };

        Ok(Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Decode Thumb unconditional branch
    fn decode_thumb_unconditional_branch(&self, encoding: u16) -> Result<Instruction, Arm32Error> {
        let offset = ((encoding & 0x7FF) as i16) << 1; // Sign extend and multiply by 2

        let statement = iceball::parse_statement(Architecture::Arm32, "b")?;

        let target = if offset >= 0 {
            Argument::Constant(offset as u64)
        } else {
            // TODO: Handle negative offsets
            Argument::Constant((-offset) as u64)
        };

        Ok(Instruction {
            statement: Ok(statement),
            arguments: vec![target].into_boxed_slice(),
            bytes: Some(encoding.to_le_bytes().to_vec().into_boxed_slice()),
        })
    }

    /// Decode Thumb-2 32-bit instruction
    fn decode_thumb2_32bit_encoding(&self, _encoding: u32) -> Result<Instruction, Arm32Error> {
        // This is a complex area - Thumb-2 has many instruction formats
        // For now, return an error
        Err(Arm32Error::Unimplemented(
            "Thumb-2 32-bit decoding".to_string(),
        ))
    }

    // Stub implementations for remaining Thumb decoders
    fn decode_thumb_special_data(&self, _encoding: u16) -> Result<Instruction, Arm32Error> {
        Err(Arm32Error::Unimplemented("Thumb special data".to_string()))
    }

    fn decode_thumb_literal_load(&self, _encoding: u16) -> Result<Instruction, Arm32Error> {
        Err(Arm32Error::Unimplemented("Thumb literal load".to_string()))
    }

    fn decode_thumb_load_store_register(&self, _encoding: u16) -> Result<Instruction, Arm32Error> {
        Err(Arm32Error::Unimplemented(
            "Thumb load/store register".to_string(),
        ))
    }

    fn decode_thumb_load_store_immediate(&self, _encoding: u16) -> Result<Instruction, Arm32Error> {
        Err(Arm32Error::Unimplemented(
            "Thumb load/store immediate".to_string(),
        ))
    }

    fn decode_thumb_load_store_halfword(&self, _encoding: u16) -> Result<Instruction, Arm32Error> {
        Err(Arm32Error::Unimplemented(
            "Thumb load/store halfword".to_string(),
        ))
    }

    fn decode_thumb_stack_ops(&self, _encoding: u16) -> Result<Instruction, Arm32Error> {
        Err(Arm32Error::Unimplemented(
            "Thumb stack operations".to_string(),
        ))
    }

    fn decode_thumb_add_sp_pc(&self, _encoding: u16) -> Result<Instruction, Arm32Error> {
        Err(Arm32Error::Unimplemented("Thumb add SP/PC".to_string()))
    }

    fn decode_thumb_misc(&self, _encoding: u16) -> Result<Instruction, Arm32Error> {
        Err(Arm32Error::Unimplemented("Thumb miscellaneous".to_string()))
    }

    fn decode_thumb_conditional_branch(&self, _encoding: u16) -> Result<Instruction, Arm32Error> {
        Err(Arm32Error::Unimplemented(
            "Thumb conditional branch".to_string(),
        ))
    }

    fn decode_thumb_svc(&self, _encoding: u16) -> Result<Instruction, Arm32Error> {
        Err(Arm32Error::Unimplemented("Thumb SVC".to_string()))
    }
}

/// Helper function to create register argument
fn create_register_argument(reg_num: u8) -> Argument {
    let reg_str = register_name(reg_num);
    match iceball::parse_argument(Architecture::Arm32, reg_str) {
        Ok(arg) => arg,
        Err(_) => Argument::Constant(reg_num as u64), // Fallback
    }
}

/// Get register name from number
fn register_name(reg_num: u8) -> &'static str {
    match reg_num {
        0 => "r0",
        1 => "r1",
        2 => "r2",
        3 => "r3",
        4 => "r4",
        5 => "r5",
        6 => "r6",
        7 => "r7",
        8 => "r8",
        9 => "r9",
        10 => "r10",
        11 => "r11",
        12 => "r12",
        13 => "sp",
        14 => "lr",
        15 => "pc",
        _ => "unknown",
    }
}

/// Get condition suffix string
fn condition_suffix(cond: Condition) -> &'static str {
    match cond {
        Condition::EQ => "eq",
        Condition::NE => "ne",
        Condition::CS => "cs",
        Condition::CC => "cc",
        Condition::MI => "mi",
        Condition::PL => "pl",
        Condition::VS => "vs",
        Condition::VC => "vc",
        Condition::HI => "hi",
        Condition::LS => "ls",
        Condition::GE => "ge",
        Condition::LT => "lt",
        Condition::GT => "gt",
        Condition::LE => "le",
        Condition::AL => "",
        Condition::NV => "nv",
    }
}

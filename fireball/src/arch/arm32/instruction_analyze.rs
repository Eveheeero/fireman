//! ARM32 instruction analysis and decoding

use super::{Arm32Error, InstructionSet, decoder::Arm32Decoder};
use crate::core::{Address, Instruction as CoreInstruction};

/// ARM32 instruction analyzer
pub struct Arm32Analyzer {
    /// Current instruction set mode
    mode: InstructionSet,
    /// Instruction decoder
    decoder: Arm32Decoder,
}

impl Arm32Analyzer {
    /// Create a new ARM32 analyzer
    pub fn new(mode: InstructionSet) -> Self {
        Self {
            mode,
            decoder: Arm32Decoder::new(mode),
        }
    }

    /// Analyze instruction at given address
    pub fn analyze_instruction(
        &self,
        data: &[u8],
        address: &Address,
    ) -> Result<CoreInstruction, Arm32Error> {
        // Use the decoder to get an iceball Instruction
        let (iceball_inst, size) = self.decoder.decode_instruction(data, address)?;

        // Convert iceball Instruction to CoreInstruction
        self.convert_to_core_instruction(iceball_inst, address, size)
    }

    /// Convert iceball Instruction to CoreInstruction
    fn convert_to_core_instruction(
        &self,
        iceball_inst: iceball::Instruction,
        address: &Address,
        _size: usize,
    ) -> Result<CoreInstruction, Arm32Error> {
        // Create the CoreInstruction with virtual address and iceball instruction
        let core_inst = CoreInstruction::new(address.get_virtual_address(), iceball_inst);

        Ok(core_inst)
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

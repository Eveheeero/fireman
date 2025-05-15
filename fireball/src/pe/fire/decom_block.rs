use super::Pe;
use crate::{
    core::{Address, Block},
    ir::{Ir, IrBlock},
    prelude::*,
};
use std::sync::Arc;

impl Pe {
    pub(super) fn _decom_block(&self, address: &Address) -> Result<Arc<Block>, DecompileError> {
        debug!(?address, "Block decompilation started");

        // Create the block
        let block = self.generate_block_from_address(address);
        // Parse instructions in the block
        let block_size = if let Some(block_size) = block.get_block_size() {
            *block_size
        } else {
            warn!(?address, "Unable to determine block end location");
            1
        };

        /* Instruction conversion */
        let instructions = self.parse_assem_range(address, block_size)?;
        let mut ir_block = Vec::new();
        let mut instruction_address = address.clone();
        for instruction in instructions {
            let instruction_size = instruction
                .inner
                .bytes
                .as_ref()
                .expect("Assembly parsing result always includes byte data")
                .len();

            /* IR generation */
            let statements =
                crate::arch::x86_64::instruction_analyze::create_ir_statement(&instruction);
            if statements.is_none() {
                warn!(?address, "Instruction conversion failed");
            };
            let ir = Ir {
                address: instruction_address.clone(),
                instruction: instruction.into(),
                statements,
            };
            ir_block.push(ir);

            /* Post-processing */
            // Move instruction address
            instruction_address += instruction_size as u64;
        }
        debug!(
            "Completed IR conversion for block instructions, total {} statements",
            ir_block
                .iter()
                .filter(|x| x.statements.is_some())
                .map(|x| x.statements.as_ref().unwrap().len())
                .sum::<usize>()
        );
        let mut ir_block = IrBlock::new(ir_block);

        /* Analysis */
        // Data access analysis
        ir_block.analyze_data_access();
        // Determine accessed memory areas and specify types according to used instructions
        ir_block.analyze_datatypes();
        // Set block internal variables
        ir_block.analyze_variables().unwrap();
        // Re-specify types according to native API call arguments
        // TODO
        // Identify used arguments within the block
        // TODO If there are many used arguments, threat as inner block of the function
        // Check analysis results
        let validate_result = ir_block.validate();
        if let Err(e) = validate_result {
            error!(?e, "IR analyzed data is invalid");
        }
        // Save analysis results in the block
        block.set_ir(ir_block);

        Ok(block)
    }
}

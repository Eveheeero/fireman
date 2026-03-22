use super::MachO;
use crate::{
    core::{Address, Block},
    ir::{Ir, IrBlock},
    prelude::*,
};
use std::sync::Arc;

impl MachO {
    pub(super) fn _analyze_block(&self, address: &Address) -> Result<Arc<Block>, DecompileError> {
        let start_va = address.get_virtual_address();
        let start_file_offset = address.get_file_offset();
        info!(
            start_va,
            start_file_offset = ?start_file_offset,
            "Block analysis started"
        );

        // Create the block
        let block = self.generate_block_from_address(address);
        debug!(start_va, "Block generated from address {block}");

        /* Instruction conversion */
        let instructions = block.get_instructions().clone();
        let instruction_count = instructions.len();
        let mut ir_block = Vec::new();
        let mut instruction_address = address.clone();
        debug!(
            start_va,
            instruction_count, "Converting block instructions to IR"
        );
        for instruction in instructions.iter() {
            let instruction_size = instruction
                .inner
                .bytes
                .as_ref()
                .expect("Assembly parsing result always includes byte data")
                .len();

            /* IR generation */
            let statements = crate::arch::create_ir_statement(self.architecture(), &instruction);
            if statements.is_none() {
                warn!("Instruction conversion failed: {}", instruction);
            };
            let ir = Ir {
                address: instruction_address.clone(),
                statements,
            };
            ir_block.push(ir);

            /* Post-processing */
            // Move instruction address
            instruction_address += instruction_size as u64;
        }
        let ir_statement_count = ir_block
            .iter()
            .filter(|x| x.statements.is_some())
            .map(|x| x.statements.as_ref().unwrap().len())
            .sum::<usize>();
        debug!(
            start_va,
            instruction_count, ir_statement_count, "Completed IR conversion for block"
        );
        let mut ir_block = IrBlock::new(ir_block, instructions);

        /* Analysis */
        // Data access analysis
        ir_block.analyze_data_access();
        // Determine accessed memory areas and specify types according to used instructions
        ir_block.analyze_datatypes();
        // Set block internal variables
        ir_block.analyze_variables().unwrap();
        // Check analysis results
        let validate_result = ir_block.validate();
        if let Err(e) = validate_result {
            error!(?e, "IR analyzed data is invalid");
        }
        // Save analysis results in the block
        block.set_ir(ir_block);

        info!(
            start_va,
            instruction_count, ir_statement_count, "Block analysis completed"
        );
        Ok(block)
    }
}

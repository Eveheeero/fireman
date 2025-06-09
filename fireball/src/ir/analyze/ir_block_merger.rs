use crate::{
    core::{Block, Instruction},
    ir::{Ir, IrBlock, analyze::DataType, data::DataAccess, utils::IrStatementDescriptorMap},
    prelude::*,
};
use std::sync::Arc;

pub fn merge_blocks(blocks: &[Arc<Block>]) -> MergedIr {
    info!("Merging IR from {} blocks", blocks.len());
    // Merge IRs from all blocks in execution order
    let mut combined_ir = Vec::new();
    let mut instructions = Vec::new();
    for block in blocks {
        let ir_block = block.get_ir();
        let Some(ir_block) = ir_block.as_ref() else {
            continue;
        };
        // TODO should we return err when ir not analyzed?
        // If block not analyzed, skip
        combined_ir.extend(ir_block.ir().iter().cloned());
        // if ir not sent, instruction must not be sent, it causes invalid ir analysis
        instructions.extend(block.get_instructions().iter().cloned());
    }

    debug!("Merged IR size: {}", combined_ir.len());
    // Analyze merged IR
    let mut ir_block = IrBlock::new(combined_ir.clone(), instructions.into());
    let instructions = ir_block.instructions().clone();
    ir_block.analyze_data_access();
    ir_block.analyze_datatypes();
    ir_block
        .analyze_variables()
        .expect("Variable analysis failed");

    // Collect merged variables
    let vars = ir_block.variables.unwrap();
    let merged_vars = vars
        .into_iter()
        .map(|v| MergedIrVariable {
            data_type: v.data_type,
            data_accesses: v.into_data_accesses(),
        })
        .collect();

    info!("Merge completed");
    MergedIr {
        instructions,
        ir: combined_ir,
        variables: merged_vars,
    }
}

#[derive(Debug, Clone)]
pub struct MergedIr {
    instructions: Arc<[Instruction]>,
    ir: Vec<Ir>,
    variables: Vec<MergedIrVariable>,
}

impl MergedIr {
    pub fn new(
        instructions: Arc<[Instruction]>,
        ir: Vec<Ir>,
        variables: Vec<MergedIrVariable>,
    ) -> Self {
        Self {
            instructions,
            ir,
            variables,
        }
    }
    pub fn get_ir(&self) -> &Vec<Ir> {
        &self.ir
    }
    pub fn get_instructions(&self) -> &Arc<[Instruction]> {
        &self.instructions
    }
    pub fn get_variables(&self) -> &Vec<MergedIrVariable> {
        &self.variables
    }
}

#[derive(Debug, Clone)]
pub struct MergedIrVariable {
    data_accesses: IrStatementDescriptorMap<Vec<DataAccess>>,
    pub data_type: DataType,
}

impl MergedIrVariable {
    pub fn get_data_accesses(&self) -> &IrStatementDescriptorMap<Vec<DataAccess>> {
        &self.data_accesses
    }
}

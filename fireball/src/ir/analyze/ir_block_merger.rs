use crate::{
    core::Block,
    ir::{analyze::DataType, data::DataAccess, utils::IrStatementDescriptorMap, Ir, IrBlock},
    prelude::*,
};
use std::sync::Arc;

pub fn merge_blocks(blocks: &[Arc<Block>]) -> MergedIr {
    info!("Merging IR from {} blocks", blocks.len());
    // Merge IRs from all blocks in execution order
    let mut combined_ir = Vec::new();
    for block in blocks {
        if let Some(ir_block) = block.get_ir().as_ref() {
            combined_ir.extend(ir_block.ir().iter().cloned());
        }
    }

    debug!("Merged IR size: {}", combined_ir.len());
    // Analyze merged IR
    let mut ir_block = IrBlock::new(combined_ir.clone());
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
            accesses: v.into_data_accesses(),
        })
        .collect();

    info!("Merge completed");
    MergedIr {
        ir: combined_ir,
        variables: merged_vars,
    }
}

pub struct MergedIr {
    pub ir: Vec<Ir>,
    pub variables: Vec<MergedIrVariable>,
}

pub struct MergedIrVariable {
    pub accesses: IrStatementDescriptorMap<Vec<DataAccess>>,
    pub data_type: DataType,
}

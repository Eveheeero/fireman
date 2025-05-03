use crate::{
    core::Address,
    ir::{data::IrData, statements::IrStatement, Ir, IrBlock},
    utils::Aos,
};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IrVariable {
    live_in: Address,
    live_out: Address,
    location: Aos<IrData>,
}

pub fn analyze_variables(ir_block: &IrBlock) -> Result<HashSet<IrVariable>, &'static str> {
    let mut result = HashSet::new();
    let irs = ir_block.ir.as_ref();
    let known_datatypes_per_ir = ir_block
        .known_datatypes_per_ir
        .as_ref()
        .ok_or_else(|| "Datatypes Not Analyzed")?;
    let data_access_per_ir = ir_block
        .data_access_per_ir
        .as_ref()
        .ok_or_else(|| "Data Access Not Analyzed")?;

    for (ir_index, ir) in irs.iter().enumerate() {}

    Ok(result)
}

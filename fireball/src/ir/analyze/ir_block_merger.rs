use crate::{
    core::Block,
    ir::{analyze::DataType, data::DataAccess, Ir},
};
use intmap::IntMap;
use std::sync::Arc;

pub fn merge_blocks(blocks: &[Arc<Block>]) -> MergedIr {
    todo!()
}

// Placeholder for the IR block merger.
pub struct MergedIr {
    pub ir: Vec<Ir>,
    pub variables: Vec<MergedIrVariable>,
}

pub struct MergedIrVariable {
    /// Key is statement hash
    pub accesses: IntMap<usize, Vec<DataAccess>>,
    pub data_type: DataType,
}

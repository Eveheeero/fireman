use crate::core::Block;
use std::sync::Arc;

pub fn merge_blocks(blocks: &[Arc<Block>]) -> MerrgedIr {}

// Placeholder for the IR block merger.
pub struct MergedIr {
    pub ir: Vec<Ir>,
    pub variables: Vec<MergedIrVariable>,
}

pub struct MergedIrVariable {
    /// Key is statement hash
    pub accesses: HashMap<usize, Vec<DataAccess>>,
    pub data_type: DataType,
}

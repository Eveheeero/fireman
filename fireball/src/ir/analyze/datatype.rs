use crate::{
    core::Address,
    ir::{data::IRData, Ir},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KnownDataType {
    pub shown_in: Address,
    pub location: IRData,
    pub data_type: DataType,
    pub data_size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Unknown,
    Int,
    Float,
    StringPointer,
    Char,
    Address,
}

pub fn analyze_datatype(ir: &Ir) -> Vec<KnownDataType> {
    todo!()
}

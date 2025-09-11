use crate::{abstract_syntax_tree::AstValueType, ir::data::IrData, utils::Aos};

#[derive(Debug, Clone)]
pub struct AstParameter {
    pub name: String,
    pub var_type: AstValueType,
    pub location: AstParameterLocation,
}
#[derive(Debug, Clone)]
pub enum AstParameterLocation {
    Register(Aos<IrData>),
    Stack(isize),
}

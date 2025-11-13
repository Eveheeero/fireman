use crate::{
    abstract_syntax_tree::objects::*,
    ir::{data::IrDataAccess, utils::IrStatementDescriptorMap},
};
use std::fmt::Formatter;

#[derive(Clone, PartialEq)]
pub struct AstVariable {
    pub name: Option<String>,
    pub id: AstVariableId,
    pub var_type: AstValueType,
    pub const_value: Option<Wrapped<AstValue>>,
    /// None if origin is not ir
    pub data_access_ir: Option<IrStatementDescriptorMap<Vec<IrDataAccess>>>,
}

impl std::fmt::Debug for AstVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AstVariable")
            .field("name", &self.name)
            .field("id", &self.id)
            .field("type", &self.var_type)
            .field("const_value", &self.const_value)
            .finish()
    }
}

impl AstVariable {
    pub fn name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| self.id.get_default_name())
    }
}

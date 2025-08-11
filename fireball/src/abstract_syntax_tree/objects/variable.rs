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
        write!(
            f,
            "AstVariable {{ name: {:?}, id: {:?}, type: {:?}{}}}",
            self.name(),
            self.id,
            self.var_type,
            if let Some(val) = &self.const_value {
                format!(", const_value: {:?}", val)
            } else {
                String::new()
            }
        )
    }
}

impl AstVariable {
    pub fn name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| self.id.get_default_name())
    }
}

use crate::ir::analyze::ir_to_ast::abstract_syntax_tree::objects::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AstVariable {
    pub name: Option<String>,
    pub id: AstVariableId,
    pub var_type: AstValueType,
    pub const_value: Option<Wrapped<AstValue>>,
}

impl AstVariable {
    pub fn name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| self.id.get_default_name())
    }
}

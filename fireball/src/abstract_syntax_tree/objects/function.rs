use crate::{abstract_syntax_tree::objects::*, ir::analyze::IrFunction};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AstFunction {
    pub name: Option<String>,
    pub id: AstFunctionId,
    pub ir: Arc<IrFunction>,
    pub return_type: AstValueType,
    pub parameters: Vec<AstParameter>,
    pub variables: ArcAstVariableMap,
    pub body: Vec<WrappedAstStatement>,

    pub processed_optimizations: Vec<ProcessedOptimization>,
}

impl AstFunction {
    pub fn name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| self.id.get_default_name())
    }
}

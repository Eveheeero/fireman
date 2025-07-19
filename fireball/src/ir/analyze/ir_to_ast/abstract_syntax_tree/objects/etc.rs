use crate::{
    ir::analyze::ir_to_ast::abstract_syntax_tree::objects::*, utils::version_map::VersionMap,
};
use hashbrown::HashMap;
use std::sync::{Arc, RwLock};

pub type ArcAstFunctionMap =
    Arc<RwLock<HashMap<AstFunctionId, VersionMap<AstFunctionVersion, AstFunction>>>>;
pub type ArcAstVariableMap = Arc<RwLock<HashMap<AstVariableId, AstVariable>>>;

#[derive(Debug, Clone)]
pub enum AstJumpTarget {
    Variable {
        scope: AstFunctionId,
        id: AstVariableId,
    },
    Function {
        target: AstFunctionId,
    },
    Instruction {
        target: AstDescriptor,
    },
    Unknown(String),
}

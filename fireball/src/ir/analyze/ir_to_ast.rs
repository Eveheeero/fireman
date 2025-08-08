use crate::{
    abstract_syntax_tree::Ast,
    core::Block,
    ir::analyze::{ControlFlowGraphAnalyzer, ir_function::generate_ir_function},
    prelude::*,
};
use std::sync::Arc;

/// Generate AST from targets
pub fn generate_ast(targets: impl IntoIterator<Item = Arc<Block>>) -> Result<Ast, DecompileError> {
    let mut ast = Ast::new();
    let mut cfg_analyzer = ControlFlowGraphAnalyzer::new();
    cfg_analyzer.add_targets(targets);
    let cfgs = cfg_analyzer.analyze();
    for cfg in cfgs.into_iter() {
        let merged = generate_ir_function(&cfg.get_blocks());
        let merged = Arc::new(merged);
        ast.generate_default_function(merged);
    }
    Ok(ast)
}

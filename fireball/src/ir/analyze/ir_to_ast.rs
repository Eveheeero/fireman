use crate::{
    abstract_syntax_tree::Ast,
    core::Block,
    ir::analyze::{BlockGrouper, ir_function::generate_ir_function},
    prelude::*,
};
use std::sync::Arc;

/// Generate AST from targets
pub fn generate_ast(targets: impl IntoIterator<Item = Arc<Block>>) -> Result<Ast, DecompileError> {
    let mut ast = Ast::new();
    let mut block_grouper = BlockGrouper::new();
    block_grouper.add_targets(targets);
    let block_groups = block_grouper.analyze();
    for block_group in block_groups.into_iter() {
        let merged = generate_ir_function(&block_group.get_blocks());
        let merged = Arc::new(merged);
        ast.generate_default_function(merged);
    }
    Ok(ast)
}

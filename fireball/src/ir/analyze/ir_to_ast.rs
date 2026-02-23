use crate::{
    abstract_syntax_tree::Ast,
    core::{Block, PreDefinedOffsets},
    ir::analyze::{BlockGrouper, ir_function::generate_ir_function},
    prelude::*,
};
use std::sync::Arc;

/// Generate AST from targets
pub fn generate_ast(targets: impl IntoIterator<Item = Arc<Block>>) -> Result<Ast, DecompileError> {
    generate_ast_with_pre_defined_symbols(targets, PreDefinedOffsets::new())
}

pub fn generate_ast_with_pre_defined_symbols(
    targets: impl IntoIterator<Item = Arc<Block>>,
    pre_defined_symbols: Arc<PreDefinedOffsets>,
) -> Result<Ast, DecompileError> {
    let mut ast = Ast::new();
    ast.set_pre_defined_symbols(pre_defined_symbols);
    let mut block_grouper = BlockGrouper::new();
    block_grouper.add_targets(targets);
    let block_groups = block_grouper.analyze();
    for block_group in block_groups.into_iter() {
        let merged = generate_ir_function(&block_group.get_blocks());
        if merged.get_ir().is_empty() {
            continue;
        }
        let merged = Arc::new(merged);
        ast.generate_default_function(merged);
    }
    Ok(ast)
}

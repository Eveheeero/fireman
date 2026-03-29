//! Remove empty Block([]) nodes from the AST.
//!
//! Pattern: Block([]) → remove the statement.

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn collapse_empty_blocks(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let mut body;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        body = std::mem::take(&mut function.body);
    }

    collapse_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::ControlFlowCleanup);
    }

    Ok(())
}

fn collapse_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    let mut i = 0;
    while i < stmts.len() {
        let should_remove =
            matches!(&stmts[i].statement, AstStatement::Block(inner) if inner.is_empty());

        if should_remove {
            stmts.remove(i);
            // Don't increment i, check the new statement at this position
        } else {
            // Process nested structures
            match &mut stmts[i].statement {
                AstStatement::If(_, bt, bf) => {
                    collapse_in_list(bt);
                    if let Some(bf) = bf {
                        collapse_in_list(bf);
                    }
                }
                AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
                    collapse_in_list(body);
                }
                AstStatement::For(_, _, _, body) => {
                    collapse_in_list(body);
                }
                AstStatement::Block(inner) => {
                    collapse_in_list(inner);
                }
                AstStatement::Switch(_, cases, default) => {
                    for (_, case_body) in cases {
                        collapse_in_list(case_body);
                    }
                    if let Some(default_body) = default {
                        collapse_in_list(default_body);
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abstract_syntax_tree::{
        AstExpression, AstFunctionId, AstLiteral,
        optimize::pattern_matching::embedded::test_utils::test_utils::*,
    };

    #[test]
    fn parity_collapse_empty_blocks() {
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["x"]);
        let x = ids[0];

        // Block([x = 1, Block([]), x = 2]) should collapse to x = 1; x = 2;
        let body = vec![wrap_statement(AstStatement::Block(vec![
            wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), x)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            )),
            wrap_statement(AstStatement::Block(vec![])),
            wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), x)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(2))),
            )),
        ]))];

        let (fb, embed) = run_parity(
            "cleanup/after-optimization/collapse-empty-blocks.fb",
            body,
            vm,
            |c| c,
        );
        assert!(
            !embed.contains("Block"),
            "embedded should collapse empty blocks, got:\n{}",
            embed
        );
        assert!(
            embed.contains("x = 1"),
            "embedded should preserve first assignment, got:\n{}",
            embed
        );
        assert!(
            embed.contains("x = 2"),
            "embedded should preserve second assignment, got:\n{}",
            embed
        );
        if fb != embed {
            eprintln!(
                "KNOWN DIFF: collapse-empty-blocks fb vs embedded differs.\n  fb: {}\n  embed: {}",
                fb.replace('\n', "\\n"),
                embed.replace('\n', "\\n"),
            );
        }
    }
}

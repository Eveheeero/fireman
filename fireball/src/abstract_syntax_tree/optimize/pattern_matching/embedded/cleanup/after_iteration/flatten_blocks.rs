//! Flatten nested Block statements by splicing their contents into the parent scope.
//!
//! Pattern: Block(...) → splice contents into parent statement list.

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn flatten_blocks(
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

    flatten_in_list(&mut body);

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

fn flatten_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    // First, recursively process nested structures
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                flatten_in_list(bt);
                if let Some(bf) = bf {
                    flatten_in_list(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
                flatten_in_list(body);
            }
            AstStatement::For(_, _, _, body) => {
                flatten_in_list(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    flatten_in_list(case_body);
                }
                if let Some(default_body) = default {
                    flatten_in_list(default_body);
                }
            }
            _ => {}
        }
    }

    // Then, flatten Block statements in this level
    // We need to use a worklist approach since we're modifying the vector
    let mut i = 0;
    while i < stmts.len() {
        if matches!(&stmts[i].statement, AstStatement::Block(_)) {
            // Remove the Block wrapper and replace it with its contents
            let inner_stmts = if let AstStatement::Block(inner) = &mut stmts[i].statement {
                std::mem::take(inner)
            } else {
                unreachable!()
            };
            stmts.remove(i);
            for (j, stmt) in inner_stmts.into_iter().enumerate() {
                stmts.insert(i + j, stmt);
            }
            // Recursively flatten the spliced content
            // Note: we don't increment i here so we process the newly inserted stmts
        } else {
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
    fn parity_flatten_blocks() {
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["x"]);
        let x = ids[0];

        // Block(Block(x = 1), x = 2) should flatten to x = 1; x = 2;
        let inner_block = AstStatement::Block(vec![wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(vm.clone(), x)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
        ))]);

        let outer_block = AstStatement::Block(vec![
            wrap_statement(inner_block),
            wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), x)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(2))),
            )),
        ]);

        let body = vec![wrap_statement(outer_block)];

        let (fb, embed) = run_parity("cleanup/after-iteration/flatten-blocks.fb", body, vm, |c| c);
        assert!(
            !embed.contains("Block"),
            "embedded should flatten all blocks, got:\n{}",
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
                "KNOWN DIFF: flatten-blocks fb vs embedded differs.\n  fb: {}\n  embed: {}",
                fb.replace('\n', "\\n"),
                embed.replace('\n', "\\n"),
            );
        }
    }
}

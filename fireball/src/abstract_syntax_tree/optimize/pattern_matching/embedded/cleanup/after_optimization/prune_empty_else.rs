//! Remove empty else branches (Some([])) from if statements.
//!
//! Pattern: if(cond, then, Some([])) → if(cond, then, None)

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn prune_empty_else(
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

    prune_in_list(&mut body);

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

fn prune_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                prune_in_list(bt);
                // Check if else branch is Some([]) and remove it
                if bf.as_ref().map_or(false, |v| v.is_empty()) {
                    *bf = None;
                } else if let Some(else_branch) = bf {
                    prune_in_list(else_branch);
                }
            }
            AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
                prune_in_list(body);
            }
            AstStatement::For(_, _, _, body) => {
                prune_in_list(body);
            }
            AstStatement::Block(body) => {
                prune_in_list(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    prune_in_list(case_body);
                }
                if let Some(default_body) = default {
                    prune_in_list(default_body);
                }
            }
            _ => {}
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
    fn test_prune_empty_else_embedded() {
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["cond", "x"]);
        let (cond, x) = (ids[0], ids[1]);

        // if (cond) { x = 1; } else { } should become if (cond) { x = 1; }
        let body = vec![wrap_statement(AstStatement::If(
            wrap_expression(AstExpression::Variable(vm.clone(), cond)),
            vec![wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), x)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            ))],
            Some(vec![]), // Empty else branch
        ))];

        let embed =
            run_direct_embedded_pass(body, |ast, fid, ver| super::prune_empty_else(ast, fid, ver));

        assert!(
            !embed.contains("else"),
            "embedded should prune empty else branch, got:\n{}",
            embed
        );
        assert!(
            embed.contains("x = 1"),
            "embedded should preserve then branch, got:\n{}",
            embed
        );
    }
}

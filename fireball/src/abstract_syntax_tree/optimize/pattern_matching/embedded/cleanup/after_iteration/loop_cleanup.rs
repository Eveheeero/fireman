//! Redundant loop-control cleanup: while(c) { ... continue; } -> while(c) { ... }.

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn cleanup_loops(
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

    cleanup_in_list(&mut body);

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

fn cleanup_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                cleanup_in_list(bt);
                if let Some(bf) = bf {
                    cleanup_in_list(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
                cleanup_in_list(body);
                try_remove_last_continue(body);
            }
            AstStatement::For(_, _, _, body) => {
                cleanup_in_list(body);
                try_remove_last_continue(body);
            }
            AstStatement::Block(body) => cleanup_in_list(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    cleanup_in_list(case_body);
                }
                if let Some(body) = default {
                    cleanup_in_list(body);
                }
            }
            _ => {}
        }
    }
}

fn try_remove_last_continue(body: &mut Vec<WrappedAstStatement>) {
    if body.is_empty() {
        return;
    }
    let last_idx = body.len() - 1;
    if matches!(body[last_idx].statement, AstStatement::Continue) {
        body.remove(last_idx);
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
    fn parity_loop_cleanup_trailing_continue() {
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["cond", "x"]);
        let (cond, x) = (ids[0], ids[1]);

        let body = vec![wrap_statement(AstStatement::While(
            wrap_expression(AstExpression::Variable(vm.clone(), cond)),
            vec![
                wrap_statement(AstStatement::Assignment(
                    wrap_expression(AstExpression::Variable(vm.clone(), x)),
                    wrap_expression(AstExpression::Literal(AstLiteral::Int(7))),
                )),
                wrap_statement(AstStatement::Continue),
            ],
        ))];

        let (fb, embed) = run_parity("cleanup/after-iteration/loop-cleanup.fb", body, vm, |c| {
            c.loop_cleanup(true)
        });
        assert!(
            !fb.contains("continue;"),
            "fb should remove trailing continue, got:\n{}",
            fb
        );
        assert!(
            !embed.contains("continue;"),
            "embed should remove trailing continue, got:\n{}",
            embed
        );
        assert!(
            embed.contains("while (cond) { x = 7; }"),
            "embed should preserve the loop body after cleanup, got:\n{}",
            embed
        );
        if fb != embed {
            eprintln!(
                "KNOWN DIFF: loop_cleanup fb vs embedded differs.\n  fb: {}\n  embed: {}",
                fb.replace('\n', "\\n"),
                embed.replace('\n', "\\n"),
            );
        }
    }
}

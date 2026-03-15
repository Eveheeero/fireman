//! Assertion pattern recovery: if(!cond) { abort(); } → assert(cond).

use crate::{
    abstract_syntax_tree::{
        Ast, AstCall, AstExpression, AstFunctionId, AstFunctionVersion, AstStatement,
        ProcessedOptimization, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn recover_assertions(
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

    recover_assertions_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::AssertionRecovery);
    }

    Ok(())
}

fn recover_assertions_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                recover_assertions_in_list(bt);
                if let Some(bf) = bf {
                    recover_assertions_in_list(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
                recover_assertions_in_list(body)
            }
            AstStatement::For(_, _, _, body) => recover_assertions_in_list(body),
            AstStatement::Block(body) => recover_assertions_in_list(body),
            _ => {}
        }
    }

    for stmt in stmts.iter_mut() {
        try_recover_assertion(stmt);
    }
}

fn try_recover_assertion(stmt: &mut WrappedAstStatement) {
    let AstStatement::If(cond, branch_true, None) = &stmt.statement else {
        return;
    };

    if branch_true.len() != 1 {
        return;
    }

    let AstStatement::Call(call) = &branch_true[0].statement else {
        return;
    };

    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return,
    };

    if name.contains("abort") || name.contains("assert_fail") || name.contains("builtin_trap") {
        // if (!cond) abort() => assert(cond)
        // if (cond) abort() => assert(!cond)
        let final_cond = if let AstExpression::UnaryOp(
            crate::abstract_syntax_tree::AstUnaryOperator::Not,
            inner,
        ) = &cond.item
        {
            (**inner).clone()
        } else {
            crate::abstract_syntax_tree::Wrapped {
                item: AstExpression::UnaryOp(
                    crate::abstract_syntax_tree::AstUnaryOperator::Not,
                    Box::new(cond.clone()),
                ),
                origin: cond.origin.clone(),
                comment: None,
            }
        };

        stmt.statement =
            AstStatement::Call(AstCall::Unknown("assert".to_string(), vec![final_cond]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abstract_syntax_tree::optimize::pattern_matching::embedded::test_utils::test_utils::*;

    #[test]
    fn parity_assertion_recovery_abort() {
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["cond"]);
        let cond = ids[0];

        // if (!cond) abort();
        let body = vec![wrap_statement(AstStatement::If(
            wrap_expression(AstExpression::UnaryOp(
                crate::abstract_syntax_tree::AstUnaryOperator::Not,
                Box::new(wrap_expression(AstExpression::Variable(vm.clone(), cond))),
            )),
            vec![wrap_statement(AstStatement::Call(AstCall::Unknown(
                "abort".to_string(),
                vec![],
            )))],
            None,
        ))];

        let (fb, embed) = run_parity(
            "optimization/after-iteration/assertion-recovery.fb",
            body,
            vm,
            |c| c.ternary_recovery(true),
        ); // ternary_recovery enables this pass in coordinator
        assert_eq!(fb, embed, "assertion_recovery parity failed");
        assert!(embed.contains("assert"), "should contain assert");
    }
}

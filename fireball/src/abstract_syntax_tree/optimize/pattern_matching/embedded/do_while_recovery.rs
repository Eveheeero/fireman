//! Do-While recovery from while(true) patterns.

use crate::{
    abstract_syntax_tree::{
        Ast, AstExpression, AstFunctionId, AstFunctionVersion, AstStatement,
        ProcessedOptimization, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn recover_do_while(
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

    recover_do_while_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::DoWhileRecovery);
    }

    Ok(())
}

fn recover_do_while_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                recover_do_while_in_list(bt);
                if let Some(bf) = bf {
                    recover_do_while_in_list(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
                recover_do_while_in_list(body)
            }
            AstStatement::For(_, _, _, body) => recover_do_while_in_list(body),
            AstStatement::Block(body) => recover_do_while_in_list(body),
            _ => {}
        }
    }

    for stmt in stmts.iter_mut() {
        try_recover_do_while(stmt);
    }
}

fn try_recover_do_while(stmt: &mut WrappedAstStatement) {
    let AstStatement::While(cond, body) = &stmt.statement else {
        return;
    };

    // Check for while(true)
    if !matches!(&cond.item, AstExpression::Literal(crate::abstract_syntax_tree::AstLiteral::Bool(true))) {
        return;
    }

    if body.is_empty() {
        return;
    }

    let last_idx = body.len() - 1;
    let last = &body[last_idx];

    match &last.statement {
        // Pattern 1: if (!cond) break;
        AstStatement::If(inner_cond, branch_true, None) => {
            if branch_true.len() == 1 && matches!(branch_true[0].statement, AstStatement::Break) {
                // Recover: do { ... } while (!!inner_cond)
                let mut new_body = body.clone();
                new_body.remove(last_idx);
                
                // Simplified: if (!cond) break; => while (cond)
                // If inner_cond is UnaryOp(Not, C), then we use C.
                let final_cond = if let AstExpression::UnaryOp(crate::abstract_syntax_tree::AstUnaryOperator::Not, inner) = &inner_cond.item {
                    (**inner).clone()
                } else {
                    crate::abstract_syntax_tree::Wrapped {
                        item: AstExpression::UnaryOp(crate::abstract_syntax_tree::AstUnaryOperator::Not, Box::new(inner_cond.clone())),
                        origin: inner_cond.origin.clone(),
                        comment: None,
                    }
                };

                stmt.statement = AstStatement::DoWhile(final_cond, new_body);
            }
        }
        // Pattern 2: if (cond) {} else break;
        AstStatement::If(inner_cond, branch_true, Some(branch_false)) => {
            if branch_true.is_empty() && branch_false.len() == 1 && matches!(branch_false[0].statement, AstStatement::Break) {
                let mut new_body = body.clone();
                new_body.remove(last_idx);
                stmt.statement = AstStatement::DoWhile(inner_cond.clone(), new_body);
            }
        }
        _ => {}
    }
}

//! Prune branches with constant integer conditions.
//! Extracted from control_flow_cleanup.rs as embedded fallback.
//!
//! These transformations remove dead code from constant condition branches:
//!   - `if(0) { A }` → (empty)
//!   - `if(0) { A } else { B }` → B
//!   - `if(1) { A }` → A
//!   - `if(true) { A }` → A
//!   - `if(false) { A }` → (empty)

use crate::{
    abstract_syntax_tree::{
        Ast, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral, AstStatement,
        AstUnaryOperator, ProcessedOptimization, WrappedAstStatement,
    },
    prelude::DecompileError,
};

/// Prune constant condition branches in a function.
pub(crate) fn prune_constant_conditions(
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

    prune_constant_condition_branches(&mut body);

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

/// Determine the truth value of a constant condition expression.
fn constant_condition_truth(expr: &AstExpression) -> Option<bool> {
    match expr {
        AstExpression::Literal(AstLiteral::Int(value)) => Some(*value != 0),
        AstExpression::Literal(AstLiteral::UInt(value)) => Some(*value != 0),
        AstExpression::Literal(AstLiteral::Bool(value)) => Some(*value),
        AstExpression::UnaryOp(AstUnaryOperator::Not, inner) => {
            constant_condition_truth(&inner.item).map(|value| !value)
        }
        _ => None,
    }
}

/// Recursively prune constant condition branches from statement lists.
pub(crate) fn prune_constant_condition_branches(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(cond, bt, bf) => {
                prune_constant_condition_branches(bt);
                if let Some(bf) = bf {
                    prune_constant_condition_branches(bf);
                }
                let const_truth = constant_condition_truth(&cond.item);
                if let Some(is_true) = const_truth {
                    if is_true {
                        let body = std::mem::take(bt);
                        stmt.statement = AstStatement::Block(body);
                    } else if let Some(else_body) = bf.take() {
                        stmt.statement = AstStatement::Block(else_body);
                    } else {
                        stmt.statement = AstStatement::Empty;
                    }
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                prune_constant_condition_branches(body);
            }
            AstStatement::For(_, _, _, body) => prune_constant_condition_branches(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    prune_constant_condition_branches(case_body);
                }
                if let Some(default_body) = default {
                    prune_constant_condition_branches(default_body);
                }
            }
            _ => {}
        }
    }
}

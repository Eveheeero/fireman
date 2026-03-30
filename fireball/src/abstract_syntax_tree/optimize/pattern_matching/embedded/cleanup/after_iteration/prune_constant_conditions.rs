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
        let mut functions = ast
            .functions
            .write()
            .map_err(|_| DecompileError::LockPoisoned("ast.functions.write()".to_string()))?;
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .ok_or_else(|| DecompileError::FunctionNotFound(function_id, function_version))?;
        body = std::mem::take(&mut function.body);
    }

    prune_constant_condition_branches(&mut body);

    {
        let mut functions = ast
            .functions
            .write()
            .map_err(|_| DecompileError::LockPoisoned("ast.functions.write()".to_string()))?;
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .ok_or_else(|| DecompileError::FunctionNotFound(function_id, function_version))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abstract_syntax_tree::{
        AstExpression, AstFunctionId, AstLiteral, AstStatement,
        optimize::pattern_matching::embedded::test_utils::test_utils::*,
    };

    #[test]
    fn test_prune_constant_condition_if_false_removed() {
        // Test: if(0) { x = 1 } should be removed entirely
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["x"]);
        let x = ids[0];

        let if_stmt = AstStatement::If(
            wrap_expression(AstExpression::Literal(AstLiteral::Int(0))),
            vec![wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), x)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            ))],
            None,
        );

        let body = vec![wrap_statement(if_stmt)];
        let printed = run_direct_embedded_pass(body, |ast, fid, version| {
            prune_constant_conditions(ast, fid, version)
        });

        assert!(
            !printed.contains("x = 1"),
            "if(0) branch should be removed, but assignment still present:\n{}",
            printed
        );
        // Empty statement results in function with no body statements
        assert!(
            printed.trim() == "int test_fn() {\n}" || !printed.contains("if"),
            "if(0) should be replaced with Empty (empty body), got:\n{}",
            printed
        );
    }

    #[test]
    fn test_prune_constant_condition_if_true_inlined() {
        // Test: if(1) { x = 2 } should be inlined to just x = 2
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["x"]);
        let x = ids[0];

        let if_stmt = AstStatement::If(
            wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            vec![wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), x)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(2))),
            ))],
            None,
        );

        let body = vec![wrap_statement(if_stmt)];
        let printed = run_direct_embedded_pass(body, |ast, fid, version| {
            prune_constant_conditions(ast, fid, version)
        });

        assert!(
            printed.contains("x = 2"),
            "if(1) body should be inlined, but assignment not found:\n{}",
            printed
        );
    }

    #[test]
    fn test_prune_constant_condition_if_false_else_kept() {
        // Test: if(0) { x = 1 } else { x = 3 } should become x = 3
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["x"]);
        let x = ids[0];

        let if_stmt = AstStatement::If(
            wrap_expression(AstExpression::Literal(AstLiteral::Int(0))),
            vec![wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), x)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            ))],
            Some(vec![wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), x)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(3))),
            ))]),
        );

        let body = vec![wrap_statement(if_stmt)];
        let printed = run_direct_embedded_pass(body, |ast, fid, version| {
            prune_constant_conditions(ast, fid, version)
        });

        assert!(
            !printed.contains("x = 1"),
            "if(0) then-branch should be removed:\n{}",
            printed
        );
        assert!(
            printed.contains("x = 3"),
            "if(0) else-branch should be kept:\n{}",
            printed
        );
    }

    #[test]
    fn test_prune_constant_condition_nested() {
        // Test: if(1) { if(0) { x = 1 } } - outer kept, inner removed
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["x"]);
        let x = ids[0];

        let inner_if = AstStatement::If(
            wrap_expression(AstExpression::Literal(AstLiteral::Int(0))),
            vec![wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), x)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            ))],
            None,
        );

        let outer_if = AstStatement::If(
            wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            vec![wrap_statement(inner_if)],
            None,
        );

        let body = vec![wrap_statement(outer_if)];
        let printed = run_direct_embedded_pass(body, |ast, fid, version| {
            prune_constant_conditions(ast, fid, version)
        });

        assert!(
            !printed.contains("x = 1"),
            "Nested if(0) branch should be removed:\n{}",
            printed
        );
    }

    #[test]
    fn test_prune_constant_condition_with_boolean() {
        // Test: if(false) { x = 1 } should be removed
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["x"]);
        let x = ids[0];

        let if_stmt = AstStatement::If(
            wrap_expression(AstExpression::Literal(AstLiteral::Bool(false))),
            vec![wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), x)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            ))],
            None,
        );

        let body = vec![wrap_statement(if_stmt)];
        let printed = run_direct_embedded_pass(body, |ast, fid, version| {
            prune_constant_conditions(ast, fid, version)
        });

        assert!(
            !printed.contains("x = 1"),
            "if(false) branch should be removed:\n{}",
            printed
        );
    }
}

//! Boolean AND/OR chain recovery from nested if-else trees.
//!
//! Rewrites depth-2 boolean assignment patterns:
//!   if(a) { if(b) { v=true } else { v=false } } else { v=false }  →  v = a && b
//!   if(a) { v=true } else { if(b) { v=true } else { v=false } }   →  v = a || b

use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral,
        AstStatement, AstValueOrigin, AstVariableId, ProcessedOptimization, Wrapped,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn recover_boolean(
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

    recover_boolean_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::BooleanRecovery);
    }

    Ok(())
}

fn recover_boolean_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures first
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                recover_boolean_in_list(bt);
                if let Some(bf) = bf {
                    recover_boolean_in_list(bf);
                }
            }
            AstStatement::While(_, body) => recover_boolean_in_list(body),
            AstStatement::For(_, _, _, body) => recover_boolean_in_list(body),
            AstStatement::Block(body) => recover_boolean_in_list(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    recover_boolean_in_list(case_body);
                }
                if let Some(default_body) = default {
                    recover_boolean_in_list(default_body);
                }
            }
            _ => {}
        }
    }

    // Try to recover boolean short-circuits
    for stmt in stmts.iter_mut() {
        try_recover_and(stmt);
        try_recover_or(stmt);
    }
}

/// Recursively extract an AND chain from nested if-else patterns.
///
/// Matches:
///   `if(cond) { <inner> } else { v = false }`
/// where `<inner>` is either:
///   - `v = true` (base case: returns `[cond]`)
///   - another AND-chain if-else (recursive: returns `[cond, ...]`)
///
/// Returns the target variable and the list of conditions in chain order.
fn extract_and_chain(stmt: &AstStatement) -> Option<(AstVariableId, Vec<Wrapped<AstExpression>>)> {
    let AstStatement::If(cond, branch_true, Some(branch_false)) = stmt else {
        return None;
    };

    // Else branch: must be single assignment `v = false`
    if branch_false.len() != 1 {
        return None;
    }
    let (false_var, false_val) = match_bool_assignment(&branch_false[0].statement);
    let target_var = false_var?;
    if false_val != Some(false) {
        return None;
    }

    // True branch: must be a single statement
    if branch_true.len() != 1 {
        return None;
    }

    // Try recursive case: true branch is another AND-chain if-else
    if let Some((inner_var, inner_conditions)) = extract_and_chain(&branch_true[0].statement) {
        if inner_var == target_var {
            let mut conditions = vec![cond.clone()];
            conditions.extend(inner_conditions);
            return Some((target_var, conditions));
        }
    }

    // Base case: true branch is `v = true`
    let (true_var, true_val) = match_bool_assignment(&branch_true[0].statement);
    if true_var == Some(target_var) && true_val == Some(true) {
        return Some((target_var, vec![cond.clone()]));
    }

    None
}

/// Detect:
///   if (a) { if (b) { ... { v = true; } else { v = false; } ... } else { v = false; } } else { v = false; }
/// Rewrite to:
///   v = a && b && ...;
fn try_recover_and(stmt: &mut WrappedAstStatement) {
    let Some((_target_var, conditions)) = extract_and_chain(&stmt.statement) else {
        return;
    };

    if conditions.len() < 2 {
        return;
    }

    // Find the lhs from the else branch (which is `v = false`)
    let AstStatement::If(_, _, Some(branch_false)) = &stmt.statement else {
        return;
    };
    let lhs = match &branch_false[0].statement {
        AstStatement::Assignment(lhs, _) => lhs.clone(),
        _ => return,
    };

    let rhs = build_chain_expr(conditions, AstBinaryOperator::LogicAnd);
    stmt.statement = AstStatement::Assignment(lhs, rhs);
}

/// Recursively extract an OR chain from nested if-else patterns.
///
/// Matches:
///   `if(cond) { v = true } else { <inner> }`
/// where `<inner>` is either:
///   - `v = false` (base case: returns `[cond]`)
///   - another OR-chain if-else (recursive: returns `[cond, ...]`)
///
/// Returns the target variable and the list of conditions in chain order.
fn extract_or_chain(stmt: &AstStatement) -> Option<(AstVariableId, Vec<Wrapped<AstExpression>>)> {
    let AstStatement::If(cond, branch_true, Some(branch_false)) = stmt else {
        return None;
    };

    // True branch: must be single assignment `v = true`
    if branch_true.len() != 1 {
        return None;
    }
    let (true_var, true_val) = match_bool_assignment(&branch_true[0].statement);
    let target_var = true_var?;
    if true_val != Some(true) {
        return None;
    }

    // False branch: must be a single statement
    if branch_false.len() != 1 {
        return None;
    }

    // Try recursive case: false branch is another OR-chain if-else
    if let Some((inner_var, inner_conditions)) = extract_or_chain(&branch_false[0].statement) {
        if inner_var == target_var {
            let mut conditions = vec![cond.clone()];
            conditions.extend(inner_conditions);
            return Some((target_var, conditions));
        }
    }

    // Base case: false branch is `v = false`
    let (false_var, false_val) = match_bool_assignment(&branch_false[0].statement);
    if false_var == Some(target_var) && false_val == Some(false) {
        return Some((target_var, vec![cond.clone()]));
    }

    None
}

/// Detect:
///   if (a) { v = true; } else { if (b) { v = true; } else { ... { v = false; } ... } }
/// Rewrite to:
///   v = a || b || ...;
fn try_recover_or(stmt: &mut WrappedAstStatement) {
    let Some((_target_var, conditions)) = extract_or_chain(&stmt.statement) else {
        return;
    };

    if conditions.len() < 2 {
        return;
    }

    // Find the lhs from the true branch (which is `v = true`)
    let AstStatement::If(_, branch_true, _) = &stmt.statement else {
        return;
    };
    let lhs = match &branch_true[0].statement {
        AstStatement::Assignment(lhs, _) => lhs.clone(),
        _ => return,
    };

    let rhs = build_chain_expr(conditions, AstBinaryOperator::LogicOr);
    stmt.statement = AstStatement::Assignment(lhs, rhs);
}

/// Build a left-associative chain expression from a list of conditions and a binary operator.
///
/// For conditions `[c1, c2, c3]` and operator `&&`, produces `(c1 && c2) && c3`.
fn build_chain_expr(
    conditions: Vec<Wrapped<AstExpression>>,
    op: AstBinaryOperator,
) -> Wrapped<AstExpression> {
    let mut iter = conditions.into_iter();
    let mut result = iter.next().expect("conditions must be non-empty");
    for cond in iter {
        result = Wrapped {
            item: AstExpression::BinaryOp(op.clone(), Box::new(result), Box::new(cond)),
            origin: AstValueOrigin::Unknown,
            comment: None,
        };
    }
    result
}

/// Match `v = true` or `v = false` assignment patterns.
/// Returns (Some(var_id), Some(bool_val)) on match, (None, None) otherwise.
fn match_bool_assignment(
    stmt: &AstStatement,
) -> (
    Option<crate::abstract_syntax_tree::AstVariableId>,
    Option<bool>,
) {
    if let AstStatement::Assignment(lhs, rhs) = stmt {
        if let AstExpression::Variable(_, var_id) = &lhs.item {
            if let AstExpression::Literal(AstLiteral::Bool(val)) = &rhs.item {
                return (Some(*var_id), Some(*val));
            }
        }
    }
    (None, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abstract_syntax_tree::optimize::pattern_matching::embedded::test_utils::test_utils::*;

    #[test]
    fn parity_boolean_recovery_and() {
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["a", "b", "v"]);
        let (a, b, v) = (ids[0], ids[1], ids[2]);

        let body = vec![wrap_statement(AstStatement::If(
            wrap_expression(AstExpression::Variable(vm.clone(), a)),
            vec![wrap_statement(AstStatement::If(
                wrap_expression(AstExpression::Variable(vm.clone(), b)),
                vec![wrap_statement(AstStatement::Assignment(
                    wrap_expression(AstExpression::Variable(vm.clone(), v)),
                    wrap_expression(AstExpression::Literal(AstLiteral::Bool(true))),
                ))],
                Some(vec![wrap_statement(AstStatement::Assignment(
                    wrap_expression(AstExpression::Variable(vm.clone(), v)),
                    wrap_expression(AstExpression::Literal(AstLiteral::Bool(false))),
                ))]),
            ))],
            Some(vec![wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), v)),
                wrap_expression(AstExpression::Literal(AstLiteral::Bool(false))),
            ))]),
        ))];

        let (fb, embed) = run_parity(
            "optimization/after-iteration/boolean-recovery.fb",
            body,
            vm,
            |c| c.boolean_recovery(true),
        );
        assert_eq!(fb, embed, "boolean_recovery AND parity failed");
    }

    #[test]
    fn parity_boolean_recovery_or() {
        let fid = AstFunctionId { address: 0x9000 };
        let (ids, vm) = make_var_map(fid, &["a", "b", "v"]);
        let (a, b, v) = (ids[0], ids[1], ids[2]);

        let body = vec![wrap_statement(AstStatement::If(
            wrap_expression(AstExpression::Variable(vm.clone(), a)),
            vec![wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(vm.clone(), v)),
                wrap_expression(AstExpression::Literal(AstLiteral::Bool(true))),
            ))],
            Some(vec![wrap_statement(AstStatement::If(
                wrap_expression(AstExpression::Variable(vm.clone(), b)),
                vec![wrap_statement(AstStatement::Assignment(
                    wrap_expression(AstExpression::Variable(vm.clone(), v)),
                    wrap_expression(AstExpression::Literal(AstLiteral::Bool(true))),
                ))],
                Some(vec![wrap_statement(AstStatement::Assignment(
                    wrap_expression(AstExpression::Variable(vm.clone(), v)),
                    wrap_expression(AstExpression::Literal(AstLiteral::Bool(false))),
                ))]),
            ))]),
        ))];

        let (fb, embed) = run_parity(
            "optimization/after-iteration/boolean-recovery.fb",
            body,
            vm,
            |c| c.boolean_recovery(true),
        );
        assert_eq!(fb, embed, "boolean_recovery OR parity failed");
    }
}

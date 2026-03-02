use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral,
        AstStatement, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn recover_boolean(
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

/// Detect:
///   if (a) { if (b) { v = true; } else { v = false; } } else { v = false; }
/// Rewrite to:
///   v = a && b;
fn try_recover_and(stmt: &mut WrappedAstStatement) {
    let AstStatement::If(cond_a, branch_true, Some(branch_false)) = &stmt.statement else {
        return;
    };

    // Outer else: must be single assignment `v = false`
    if branch_false.len() != 1 {
        return;
    }
    let (outer_false_var, outer_false_val) = match_bool_assignment(&branch_false[0].statement);
    let Some(target_var) = outer_false_var else {
        return;
    };
    if outer_false_val != Some(false) {
        return;
    }

    // Outer true: must be a single if(b) statement
    if branch_true.len() != 1 {
        return;
    }
    let AstStatement::If(cond_b, inner_true, Some(inner_false)) = &branch_true[0].statement else {
        return;
    };

    // Inner true: v = true
    if inner_true.len() != 1 {
        return;
    }
    let (inner_true_var, inner_true_val) = match_bool_assignment(&inner_true[0].statement);
    if inner_true_var != Some(target_var) || inner_true_val != Some(true) {
        return;
    }

    // Inner false: v = false
    if inner_false.len() != 1 {
        return;
    }
    let (inner_false_var, inner_false_val) = match_bool_assignment(&inner_false[0].statement);
    if inner_false_var != Some(target_var) || inner_false_val != Some(false) {
        return;
    }

    // Build v = a && b
    let lhs = match &branch_false[0].statement {
        AstStatement::Assignment(lhs, _) => lhs.clone(),
        _ => return,
    };

    let and_expr = AstExpression::BinaryOp(
        AstBinaryOperator::LogicAnd,
        Box::new(cond_a.clone()),
        Box::new(cond_b.clone()),
    );
    let rhs = Wrapped {
        item: and_expr,
        origin: cond_a.origin.clone(),
        comment: None,
    };

    stmt.statement = AstStatement::Assignment(lhs, rhs);
}

/// Detect:
///   if (a) { v = true; } else { if (b) { v = true; } else { v = false; } }
/// Rewrite to:
///   v = a || b;
fn try_recover_or(stmt: &mut WrappedAstStatement) {
    let AstStatement::If(cond_a, branch_true, Some(branch_false)) = &stmt.statement else {
        return;
    };

    // Outer true: must be single assignment `v = true`
    if branch_true.len() != 1 {
        return;
    }
    let (outer_true_var, outer_true_val) = match_bool_assignment(&branch_true[0].statement);
    let Some(target_var) = outer_true_var else {
        return;
    };
    if outer_true_val != Some(true) {
        return;
    }

    // Outer false: must be a single if(b) statement
    if branch_false.len() != 1 {
        return;
    }
    let AstStatement::If(cond_b, inner_true, Some(inner_false)) = &branch_false[0].statement
    else {
        return;
    };

    // Inner true: v = true
    if inner_true.len() != 1 {
        return;
    }
    let (inner_true_var, inner_true_val) = match_bool_assignment(&inner_true[0].statement);
    if inner_true_var != Some(target_var) || inner_true_val != Some(true) {
        return;
    }

    // Inner false: v = false
    if inner_false.len() != 1 {
        return;
    }
    let (inner_false_var, inner_false_val) = match_bool_assignment(&inner_false[0].statement);
    if inner_false_var != Some(target_var) || inner_false_val != Some(false) {
        return;
    }

    // Build v = a || b
    let lhs = match &branch_true[0].statement {
        AstStatement::Assignment(lhs, _) => lhs.clone(),
        _ => return,
    };

    let or_expr = AstExpression::BinaryOp(
        AstBinaryOperator::LogicOr,
        Box::new(cond_a.clone()),
        Box::new(cond_b.clone()),
    );
    let rhs = Wrapped {
        item: or_expr,
        origin: cond_a.origin.clone(),
        comment: None,
    };

    stmt.statement = AstStatement::Assignment(lhs, rhs);
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

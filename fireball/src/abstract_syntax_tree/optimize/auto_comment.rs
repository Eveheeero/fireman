use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstLiteral, AstStatement, AstStatementOrigin, AstUnaryOperator, AstValueType,
        ProcessedOptimization, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn synthesize_comments(
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

    annotate_statement_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::AutoComment);
    }

    Ok(())
}

fn annotate_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    // First, recurse into nested statement lists.
    for stmt in stmts.iter_mut() {
        annotate_statement(stmt);
    }

    // Collect indices where comments should be inserted (index, comment text).
    let mut insertions: Vec<(usize, String)> = Vec::new();

    for (i, stmt) in stmts.iter().enumerate() {
        match &stmt.statement {
            AstStatement::If(cond, branch_true, branch_false) => {
                // Stack canary check: if-block containing a call to stack_chk_fail.
                if statement_list_contains_stack_canary(branch_true) {
                    insertions.push((i, "// stack canary check".to_string()));
                }
                // Null pointer check: if (var == 0) or if (var != 0) where var is a pointer.
                if is_null_pointer_check(cond) {
                    insertions.push((i, "// null pointer check".to_string()));
                }
                // Assertion pattern: if (!cond) { abort/exit/assert_fail(); }
                if is_assertion_pattern(cond, branch_true, branch_false.as_deref()) {
                    insertions.push((i, "// assertion".to_string()));
                }
                // Guarded call: if (ptr) { call(ptr, ...); }
                if is_guarded_call_pattern(cond, branch_true, branch_false.as_deref()) {
                    insertions.push((i, "// guarded call".to_string()));
                }
            }
            AstStatement::Assignment(_, rhs) => {
                // Macro-like pattern: var = (a < b) ? a : b → MIN, etc.
                if let Some(macro_name) = detect_min_max_pattern(&rhs.item) {
                    insertions.push((i, format!("// {macro_name}")));
                }
            }
            AstStatement::Call(call) => {
                if call_name_matches_noreturn(call) {
                    insertions.push((i, "// does not return".to_string()));
                }
                if call_name_matches_seh(call) {
                    insertions.push((i, "// SEH setup".to_string()));
                }
            }
            _ => {}
        }
    }

    // Insert from back to front so earlier indices remain valid.
    for (idx, comment_text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(comment_text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn annotate_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            annotate_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                annotate_statement_list(branch_false);
            }
        }
        AstStatement::While(_, body) => annotate_statement_list(body),
        AstStatement::For(init, _, update, body) => {
            annotate_statement(init);
            annotate_statement(update);
            annotate_statement_list(body);
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases.iter_mut() {
                annotate_statement_list(case_body);
            }
            if let Some(default_body) = default {
                annotate_statement_list(default_body);
            }
        }
        AstStatement::Block(body) => annotate_statement_list(body),
        AstStatement::Declaration(_, _)
        | AstStatement::Assignment(_, _)
        | AstStatement::Return(_)
        | AstStatement::Call(_)
        | AstStatement::Label(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Ir(_)
        | AstStatement::Empty => {}
    }
}

/// Check whether a statement list contains a call to a stack canary function.
fn statement_list_contains_stack_canary(stmts: &[WrappedAstStatement]) -> bool {
    stmts.iter().any(|s| statement_contains_stack_canary(s))
}

fn statement_contains_stack_canary(stmt: &WrappedAstStatement) -> bool {
    match &stmt.statement {
        AstStatement::Call(call) => call_name_matches_stack_canary(call),
        AstStatement::If(_, branch_true, branch_false) => {
            statement_list_contains_stack_canary(branch_true)
                || branch_false
                    .as_ref()
                    .is_some_and(|bf| statement_list_contains_stack_canary(bf))
        }
        AstStatement::Block(body) => statement_list_contains_stack_canary(body),
        _ => false,
    }
}

fn call_name_matches_stack_canary(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    let lower = name.to_ascii_lowercase();
    lower.contains("stack_chk_fail") || lower.contains("__stack_chk")
}

fn call_name_matches_noreturn(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    let lower = name.to_ascii_lowercase();
    lower.contains("exit")
        || lower.contains("abort")
        || lower.contains("panic")
        || lower.contains("terminate")
}

fn call_name_matches_seh(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.contains("SEH") || name.contains("_except_handler")
}

/// Assertion pattern: `if (!cond) { noreturn_call(); }` with no else branch,
/// or `if (cond) { noreturn_call(); }` where the condition is a negation.
fn is_assertion_pattern(
    cond: &crate::abstract_syntax_tree::Wrapped<AstExpression>,
    branch_true: &[WrappedAstStatement],
    branch_false: Option<&[WrappedAstStatement]>,
) -> bool {
    // Must have no else branch (single-armed if).
    if branch_false.is_some() {
        return false;
    }
    // The then-branch must be exactly one noreturn call.
    if branch_true.len() != 1 {
        return false;
    }
    let call = match &branch_true[0].statement {
        AstStatement::Call(call) => call,
        _ => return false,
    };
    if !call_name_matches_noreturn(call) && !call_name_matches_assert(call) {
        return false;
    }
    // The condition should be a negation or comparison (typical assertion guard).
    matches!(
        &cond.item,
        AstExpression::UnaryOp(AstUnaryOperator::Not, _)
            | AstExpression::BinaryOp(AstBinaryOperator::Equal, _, _)
            | AstExpression::BinaryOp(AstBinaryOperator::NotEqual, _, _)
    )
}

fn call_name_matches_assert(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    let lower = name.to_ascii_lowercase();
    lower.contains("assert") || lower.contains("__assert")
}

/// Guarded call pattern: `if (var) { call(var, ...); }` with no else branch.
fn is_guarded_call_pattern(
    cond: &crate::abstract_syntax_tree::Wrapped<AstExpression>,
    branch_true: &[WrappedAstStatement],
    branch_false: Option<&[WrappedAstStatement]>,
) -> bool {
    if branch_false.is_some() {
        return false;
    }
    if branch_true.len() != 1 {
        return false;
    }
    // Condition must be a plain variable.
    let cond_var_id = match &cond.item {
        AstExpression::Variable(_, var_id) => *var_id,
        _ => return false,
    };
    // The single statement must be a call that uses the same variable.
    let call_args = match &branch_true[0].statement {
        AstStatement::Call(call) => match call {
            AstCall::Unknown(_, args) | AstCall::Function { args, .. } => args,
            AstCall::Variable { args, .. } => args,
            _ => return false,
        },
        _ => return false,
    };
    call_args
        .iter()
        .any(|arg| matches!(&arg.item, AstExpression::Variable(_, vid) if *vid == cond_var_id))
}

/// Detect MIN/MAX ternary patterns: `(a < b) ? a : b` → MIN, `(a > b) ? a : b` → MAX.
fn detect_min_max_pattern(expr: &AstExpression) -> Option<&'static str> {
    let AstExpression::Ternary(cond, true_expr, false_expr) = expr else {
        return None;
    };
    let AstExpression::BinaryOp(op, cond_lhs, cond_rhs) = &cond.item else {
        return None;
    };
    // Check if true_expr structurally matches cond_lhs and false_expr matches cond_rhs.
    let true_matches_lhs = exprs_structurally_equal(&true_expr.item, &cond_lhs.item);
    let false_matches_rhs = exprs_structurally_equal(&false_expr.item, &cond_rhs.item);
    let true_matches_rhs = exprs_structurally_equal(&true_expr.item, &cond_rhs.item);
    let false_matches_lhs = exprs_structurally_equal(&false_expr.item, &cond_lhs.item);

    match op {
        // (a < b) ? a : b → MIN(a, b)   or   (a < b) ? b : a → MAX(a, b)
        AstBinaryOperator::Less | AstBinaryOperator::LessEqual => {
            if true_matches_lhs && false_matches_rhs {
                Some("MIN")
            } else if true_matches_rhs && false_matches_lhs {
                Some("MAX")
            } else {
                None
            }
        }
        // (a > b) ? a : b → MAX(a, b)   or   (a > b) ? b : a → MIN(a, b)
        AstBinaryOperator::Greater | AstBinaryOperator::GreaterEqual => {
            if true_matches_lhs && false_matches_rhs {
                Some("MAX")
            } else if true_matches_rhs && false_matches_lhs {
                Some("MIN")
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Conservative structural equality check for expressions (variable identity only).
fn exprs_structurally_equal(a: &AstExpression, b: &AstExpression) -> bool {
    match (a, b) {
        (AstExpression::Variable(_, id_a), AstExpression::Variable(_, id_b)) => id_a == id_b,
        (AstExpression::Literal(lit_a), AstExpression::Literal(lit_b)) => lit_a == lit_b,
        _ => false,
    }
}

/// Check if an expression is a null pointer check: `var == 0` or `var != 0`
/// where the variable has a pointer type.
fn is_null_pointer_check(cond: &crate::abstract_syntax_tree::Wrapped<AstExpression>) -> bool {
    let AstExpression::BinaryOp(op, lhs, rhs) = &cond.item else {
        return false;
    };

    if !matches!(op, AstBinaryOperator::Equal | AstBinaryOperator::NotEqual) {
        return false;
    }

    let (var_side, lit_side) = match (&lhs.item, &rhs.item) {
        (AstExpression::Variable(_, _), AstExpression::Literal(_)) => (&lhs.item, &rhs.item),
        (AstExpression::Literal(_), AstExpression::Variable(_, _)) => (&rhs.item, &lhs.item),
        _ => return false,
    };

    // The literal side must be 0.
    let is_zero = match lit_side {
        AstExpression::Literal(AstLiteral::Int(0)) => true,
        AstExpression::Literal(AstLiteral::UInt(0)) => true,
        _ => false,
    };
    if !is_zero {
        return false;
    }

    // The variable side must be a pointer type.
    if let AstExpression::Variable(var_map, var_id) = var_side {
        let vars = var_map.read().unwrap();
        if let Some(var) = vars.get(var_id) {
            return matches!(&var.var_type, AstValueType::Pointer(_));
        }
    }

    false
}

use super::{
    construct::construct_wrapped_expr,
    matcher::match_wrapped_expr,
    predicate::eval_where,
    types::{Captured, Captures, PatTree, WherePredicate},
};
use crate::abstract_syntax_tree::{
    AstBinaryOperator, AstBuiltinFunctionArgument, AstCall, AstExpression, AstLiteral,
    AstStatement, Wrapped, WrappedAstStatement,
};

// ---------------------------------------------------------------------------
// Expression-level transform: walk all expressions in statements, match & replace
// ---------------------------------------------------------------------------

/// Walk all expressions in `stmts` bottom-up. For each expression, try to match
/// `match_pat` with `predicates`. If matched, construct the replacement from
/// `replace_pat` and swap it in-place. Returns `true` if any replacement was made.
pub fn transform_expressions_in_stmts(
    stmts: &mut [WrappedAstStatement],
    match_pat: &PatTree,
    predicates: &[WherePredicate],
    replace_pat: &PatTree,
) -> bool {
    let mut changed = false;
    for stmt in stmts.iter_mut() {
        changed |= transform_expressions_in_statement(stmt, match_pat, predicates, replace_pat);
    }
    changed
}

fn transform_expressions_in_statement(
    stmt: &mut WrappedAstStatement,
    match_pat: &PatTree,
    predicates: &[WherePredicate],
    replace_pat: &PatTree,
) -> bool {
    match &mut stmt.statement {
        AstStatement::Declaration(_lhs, rhs) => {
            if let Some(rhs) = rhs {
                transform_expression(rhs, match_pat, predicates, replace_pat)
            } else {
                false
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            let a = transform_expression(lhs, match_pat, predicates, replace_pat);
            let b = transform_expression(rhs, match_pat, predicates, replace_pat);
            a || b
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            let a = transform_expression(cond, match_pat, predicates, replace_pat);
            let b = transform_expressions_in_stmts(branch_true, match_pat, predicates, replace_pat);
            let c = if let Some(branch_false) = branch_false {
                transform_expressions_in_stmts(branch_false, match_pat, predicates, replace_pat)
            } else {
                false
            };
            a || b || c
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            let a = transform_expression(cond, match_pat, predicates, replace_pat);
            let b = transform_expressions_in_stmts(body, match_pat, predicates, replace_pat);
            a || b
        }
        AstStatement::For(init, cond, update, body) => {
            let a = transform_expressions_in_statement(init, match_pat, predicates, replace_pat);
            let b = transform_expression(cond, match_pat, predicates, replace_pat);
            let c = transform_expressions_in_statement(update, match_pat, predicates, replace_pat);
            let d = transform_expressions_in_stmts(body, match_pat, predicates, replace_pat);
            a || b || c || d
        }
        AstStatement::Switch(discrim, cases, default) => {
            let mut changed = transform_expression(discrim, match_pat, predicates, replace_pat);
            for (_lit, case_body) in cases.iter_mut() {
                changed |=
                    transform_expressions_in_stmts(case_body, match_pat, predicates, replace_pat);
            }
            if let Some(default_body) = default {
                changed |= transform_expressions_in_stmts(
                    default_body,
                    match_pat,
                    predicates,
                    replace_pat,
                );
            }
            changed
        }
        AstStatement::Block(body) => {
            transform_expressions_in_stmts(body, match_pat, predicates, replace_pat)
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                transform_expression(expr, match_pat, predicates, replace_pat)
            } else {
                false
            }
        }
        AstStatement::Call(call) => transform_call(call, match_pat, predicates, replace_pat),
        AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Label(_)
        | AstStatement::Comment(_)
        | AstStatement::Break
        | AstStatement::Continue
        | AstStatement::Empty => false,
    }
}

fn transform_call(
    call: &mut AstCall,
    match_pat: &PatTree,
    predicates: &[WherePredicate],
    replace_pat: &PatTree,
) -> bool {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            let mut changed = false;
            for arg in args.iter_mut() {
                changed |= transform_expression(arg, match_pat, predicates, replace_pat);
            }
            changed
        }
        AstCall::Builtin(_, args) => match args.as_mut() {
            AstBuiltinFunctionArgument::None => false,
            AstBuiltinFunctionArgument::Print(items) => {
                let mut changed = false;
                for item in items.iter_mut() {
                    changed |= transform_expression(item, match_pat, predicates, replace_pat);
                }
                changed
            }
            AstBuiltinFunctionArgument::ByteSizeOf(expr)
            | AstBuiltinFunctionArgument::BitSizeOf(expr)
            | AstBuiltinFunctionArgument::OperandExists(expr)
            | AstBuiltinFunctionArgument::SignedMax(expr)
            | AstBuiltinFunctionArgument::SignedMin(expr)
            | AstBuiltinFunctionArgument::UnsignedMax(expr)
            | AstBuiltinFunctionArgument::UnsignedMin(expr)
            | AstBuiltinFunctionArgument::BitOnes(expr)
            | AstBuiltinFunctionArgument::BitZeros(expr) => {
                transform_expression(expr, match_pat, predicates, replace_pat)
            }
            AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
                let a = transform_expression(expr1, match_pat, predicates, replace_pat);
                let b = transform_expression(expr2, match_pat, predicates, replace_pat);
                a || b
            }
        },
    }
}

fn transform_expression(
    expr: &mut Wrapped<AstExpression>,
    match_pat: &PatTree,
    predicates: &[WherePredicate],
    replace_pat: &PatTree,
) -> bool {
    // Recurse into children first (bottom-up).
    let mut changed = match &mut expr.item {
        AstExpression::UnaryOp(_, arg) => {
            transform_expression(arg, match_pat, predicates, replace_pat)
        }
        AstExpression::BinaryOp(_, left, right) => {
            let a = transform_expression(left, match_pat, predicates, replace_pat);
            let b = transform_expression(right, match_pat, predicates, replace_pat);
            a || b
        }
        AstExpression::Call(call) => transform_call(call, match_pat, predicates, replace_pat),
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            transform_expression(arg, match_pat, predicates, replace_pat)
        }
        AstExpression::ArrayAccess(base, idx) => {
            let a = transform_expression(base, match_pat, predicates, replace_pat);
            let b = transform_expression(idx, match_pat, predicates, replace_pat);
            a || b
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            let a = transform_expression(cond, match_pat, predicates, replace_pat);
            let b = transform_expression(true_expr, match_pat, predicates, replace_pat);
            let c = transform_expression(false_expr, match_pat, predicates, replace_pat);
            a || b || c
        }
        AstExpression::Literal(_)
        | AstExpression::Variable(_, _)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => false,
    };

    // Now try to match the current expression against the pattern.
    // Loop until no more matches (handles chained patterns like triple casts).
    loop {
        let mut caps = Captures::new();
        if !match_wrapped_expr(match_pat, expr, &mut caps) {
            break;
        }
        let preds_ok = predicates.iter().all(|pred| eval_where(pred, &caps));
        if !preds_ok {
            break;
        }
        if let Some(replacement) = construct_wrapped_expr(replace_pat, &caps) {
            expr.item = replacement.item;
            changed = true;
        } else {
            break;
        }
    }

    changed
}

// ---------------------------------------------------------------------------
// Expression-level builtin transform: match & apply builtin function
// ---------------------------------------------------------------------------

/// Walk all expressions in `stmts` bottom-up. For each expression, try to match
/// `match_pat` with `predicates`. If matched, call the builtin function with captured
/// args to produce the replacement. Returns `true` if any replacement was made.
pub fn transform_expressions_in_stmts_builtin(
    stmts: &mut [WrappedAstStatement],
    match_pat: &PatTree,
    predicates: &[WherePredicate],
    func: &str,
    arg_names: &[String],
) -> bool {
    let mut changed = false;
    for stmt in stmts.iter_mut() {
        changed |= transform_expressions_in_statement_builtin(
            stmt, match_pat, predicates, func, arg_names,
        );
    }
    changed
}

fn transform_expressions_in_statement_builtin(
    stmt: &mut WrappedAstStatement,
    match_pat: &PatTree,
    predicates: &[WherePredicate],
    func: &str,
    arg_names: &[String],
) -> bool {
    match &mut stmt.statement {
        AstStatement::Declaration(_lhs, rhs) => {
            if let Some(rhs) = rhs {
                transform_expression_builtin(rhs, match_pat, predicates, func, arg_names)
            } else {
                false
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            let a = transform_expression_builtin(lhs, match_pat, predicates, func, arg_names);
            let b = transform_expression_builtin(rhs, match_pat, predicates, func, arg_names);
            a || b
        }
        AstStatement::If(cond, branch_true, branch_false) => {
            let a = transform_expression_builtin(cond, match_pat, predicates, func, arg_names);
            let b = transform_expressions_in_stmts_builtin(
                branch_true,
                match_pat,
                predicates,
                func,
                arg_names,
            );
            let c = if let Some(branch_false) = branch_false {
                transform_expressions_in_stmts_builtin(
                    branch_false,
                    match_pat,
                    predicates,
                    func,
                    arg_names,
                )
            } else {
                false
            };
            a || b || c
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            let a = transform_expression_builtin(cond, match_pat, predicates, func, arg_names);
            let b = transform_expressions_in_stmts_builtin(
                body, match_pat, predicates, func, arg_names,
            );
            a || b
        }
        AstStatement::For(init, cond, update, body) => {
            let a = transform_expressions_in_statement_builtin(
                init, match_pat, predicates, func, arg_names,
            );
            let b = transform_expression_builtin(cond, match_pat, predicates, func, arg_names);
            let c = transform_expressions_in_statement_builtin(
                update, match_pat, predicates, func, arg_names,
            );
            let d = transform_expressions_in_stmts_builtin(
                body, match_pat, predicates, func, arg_names,
            );
            a || b || c || d
        }
        AstStatement::Switch(discrim, cases, default) => {
            let mut changed =
                transform_expression_builtin(discrim, match_pat, predicates, func, arg_names);
            for (_lit, case_body) in cases.iter_mut() {
                changed |= transform_expressions_in_stmts_builtin(
                    case_body, match_pat, predicates, func, arg_names,
                );
            }
            if let Some(default_body) = default {
                changed |= transform_expressions_in_stmts_builtin(
                    default_body,
                    match_pat,
                    predicates,
                    func,
                    arg_names,
                );
            }
            changed
        }
        AstStatement::Block(body) => {
            transform_expressions_in_stmts_builtin(body, match_pat, predicates, func, arg_names)
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                transform_expression_builtin(expr, match_pat, predicates, func, arg_names)
            } else {
                false
            }
        }
        AstStatement::Call(call) => {
            transform_call_builtin(call, match_pat, predicates, func, arg_names)
        }
        AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Label(_)
        | AstStatement::Comment(_)
        | AstStatement::Break
        | AstStatement::Continue
        | AstStatement::Empty => false,
    }
}

fn transform_call_builtin(
    call: &mut AstCall,
    match_pat: &PatTree,
    predicates: &[WherePredicate],
    func: &str,
    arg_names: &[String],
) -> bool {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            let mut changed = false;
            for arg in args.iter_mut() {
                changed |=
                    transform_expression_builtin(arg, match_pat, predicates, func, arg_names);
            }
            changed
        }
        AstCall::Builtin(_, args) => match args.as_mut() {
            AstBuiltinFunctionArgument::None => false,
            AstBuiltinFunctionArgument::Print(items) => {
                let mut changed = false;
                for item in items.iter_mut() {
                    changed |=
                        transform_expression_builtin(item, match_pat, predicates, func, arg_names);
                }
                changed
            }
            AstBuiltinFunctionArgument::ByteSizeOf(expr)
            | AstBuiltinFunctionArgument::BitSizeOf(expr)
            | AstBuiltinFunctionArgument::OperandExists(expr)
            | AstBuiltinFunctionArgument::SignedMax(expr)
            | AstBuiltinFunctionArgument::SignedMin(expr)
            | AstBuiltinFunctionArgument::UnsignedMax(expr)
            | AstBuiltinFunctionArgument::UnsignedMin(expr)
            | AstBuiltinFunctionArgument::BitOnes(expr)
            | AstBuiltinFunctionArgument::BitZeros(expr) => {
                transform_expression_builtin(expr, match_pat, predicates, func, arg_names)
            }
            AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
                let a = transform_expression_builtin(expr1, match_pat, predicates, func, arg_names);
                let b = transform_expression_builtin(expr2, match_pat, predicates, func, arg_names);
                a || b
            }
        },
    }
}

fn transform_expression_builtin(
    expr: &mut Wrapped<AstExpression>,
    match_pat: &PatTree,
    predicates: &[WherePredicate],
    func: &str,
    arg_names: &[String],
) -> bool {
    // Recurse into children first (bottom-up).
    let mut changed = match &mut expr.item {
        AstExpression::UnaryOp(_, arg) => {
            transform_expression_builtin(arg, match_pat, predicates, func, arg_names)
        }
        AstExpression::BinaryOp(_, left, right) => {
            let a = transform_expression_builtin(left, match_pat, predicates, func, arg_names);
            let b = transform_expression_builtin(right, match_pat, predicates, func, arg_names);
            a || b
        }
        AstExpression::Call(call) => {
            transform_call_builtin(call, match_pat, predicates, func, arg_names)
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            transform_expression_builtin(arg, match_pat, predicates, func, arg_names)
        }
        AstExpression::ArrayAccess(base, idx) => {
            let a = transform_expression_builtin(base, match_pat, predicates, func, arg_names);
            let b = transform_expression_builtin(idx, match_pat, predicates, func, arg_names);
            a || b
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            let a = transform_expression_builtin(cond, match_pat, predicates, func, arg_names);
            let b = transform_expression_builtin(true_expr, match_pat, predicates, func, arg_names);
            let c =
                transform_expression_builtin(false_expr, match_pat, predicates, func, arg_names);
            a || b || c
        }
        AstExpression::Literal(_)
        | AstExpression::Variable(_, _)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => false,
    };

    // Now try to match and apply the builtin function.
    loop {
        let mut caps = Captures::new();
        if !match_wrapped_expr(match_pat, expr, &mut caps) {
            break;
        }
        let preds_ok = predicates.iter().all(|pred| eval_where(pred, &caps));
        if !preds_ok {
            break;
        }
        if let Some(replacement) = eval_builtin_fn(func, arg_names, &caps, expr) {
            expr.item = replacement.item;
            changed = true;
        } else {
            break;
        }
    }

    changed
}

/// Evaluate a builtin function given captured values.
/// Supported builtins:
///   - `eval_binop($op, $a, $b)` — evaluate binary op on two literals
///   - `eval_unary($op, $a)` — evaluate unary op on one literal
///   - `eval_reassociate($op, $x, $c1, $c2)` — `(x op c1) op c2` → `x op (c1 op c2)`
///   - `eval_reassociate_left($op, $x, $c1, $c2)` — `c1 op (c2 op x)` → `(c1 op c2) op x`
fn eval_builtin_fn(
    func: &str,
    arg_names: &[String],
    caps: &Captures,
    source: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    use crate::abstract_syntax_tree::optimize::opt_utils;

    match func {
        "eval_binop" => {
            if arg_names.len() != 3 {
                return None;
            }
            let op = caps.get(&arg_names[0]).and_then(|c| match c {
                Captured::BinaryOp(op) => Some(op),
                _ => None,
            })?;
            let lhs = caps.get(&arg_names[1]).and_then(|c| match c {
                Captured::Literal(lit) => Some(lit),
                _ => None,
            })?;
            let rhs = caps.get(&arg_names[2]).and_then(|c| match c {
                Captured::Literal(lit) => Some(lit),
                _ => None,
            })?;
            let result = opt_utils::eval_binary(op, lhs, rhs)?;
            Some(Wrapped {
                item: AstExpression::Literal(result),
                origin: source.origin.clone(),
                comment: source.comment.clone(),
            })
        }
        "eval_unary" => {
            if arg_names.len() != 2 {
                return None;
            }
            let op = caps.get(&arg_names[0]).and_then(|c| match c {
                Captured::UnaryOp(op) => Some(op),
                _ => None,
            })?;
            let val = caps.get(&arg_names[1]).and_then(|c| match c {
                Captured::Literal(lit) => Some(lit),
                _ => None,
            })?;
            let result = opt_utils::eval_unary(op, val)?;
            Some(Wrapped {
                item: AstExpression::Literal(result),
                origin: source.origin.clone(),
                comment: source.comment.clone(),
            })
        }
        // eval_reassociate($op, $x, $c1, $c2): (x op c1) op c2 → x op (c1 op c2)
        "eval_reassociate" => {
            if arg_names.len() != 4 {
                return None;
            }
            let op = caps.get(&arg_names[0]).and_then(|c| match c {
                Captured::BinaryOp(op) => Some(op),
                _ => None,
            })?;
            if !is_reassociable_op(op) {
                return None;
            }
            let x = extract_expr_from_captured(caps.get(&arg_names[1])?)?;
            let c1 = caps.get(&arg_names[2]).and_then(|c| match c {
                Captured::Literal(lit) => Some(lit),
                _ => None,
            })?;
            let c2 = caps.get(&arg_names[3]).and_then(|c| match c {
                Captured::Literal(lit) => Some(lit),
                _ => None,
            })?;
            let folded = opt_utils::eval_binary(op, c1, c2)?;
            Some(Wrapped {
                item: AstExpression::BinaryOp(
                    op.clone(),
                    Box::new(x.clone()),
                    Box::new(Wrapped {
                        item: AstExpression::Literal(folded),
                        origin: source.origin.clone(),
                        comment: None,
                    }),
                ),
                origin: source.origin.clone(),
                comment: source.comment.clone(),
            })
        }
        // eval_reassociate_left($op, $x, $c1, $c2): c1 op (c2 op x) → (c1 op c2) op x
        "eval_reassociate_left" => {
            if arg_names.len() != 4 {
                return None;
            }
            let op = caps.get(&arg_names[0]).and_then(|c| match c {
                Captured::BinaryOp(op) => Some(op),
                _ => None,
            })?;
            if !is_reassociable_op(op) {
                return None;
            }
            let x = extract_expr_from_captured(caps.get(&arg_names[1])?)?;
            let c1 = caps.get(&arg_names[2]).and_then(|c| match c {
                Captured::Literal(lit) => Some(lit),
                _ => None,
            })?;
            let c2 = caps.get(&arg_names[3]).and_then(|c| match c {
                Captured::Literal(lit) => Some(lit),
                _ => None,
            })?;
            let folded = opt_utils::eval_binary(op, c1, c2)?;
            Some(Wrapped {
                item: AstExpression::BinaryOp(
                    op.clone(),
                    Box::new(Wrapped {
                        item: AstExpression::Literal(folded),
                        origin: source.origin.clone(),
                        comment: None,
                    }),
                    Box::new(x.clone()),
                ),
                origin: source.origin.clone(),
                comment: source.comment.clone(),
            })
        }
        "eval_rotate_right" | "eval_rotate_left" => {
            if arg_names.len() != 2 {
                return None;
            }
            let x = extract_expr_from_captured(caps.get(&arg_names[0])?)?;
            let n = extract_expr_from_captured(caps.get(&arg_names[1])?)?;
            let name = if func == "eval_rotate_right" {
                "__builtin_rotate_right"
            } else {
                "__builtin_rotate_left"
            };
            Some(Wrapped {
                item: AstExpression::Call(AstCall::Unknown(
                    name.to_string(),
                    vec![x.clone(), n.clone()],
                )),
                origin: source.origin.clone(),
                comment: source.comment.clone(),
            })
        }
        "eval_strength_reduce_add" | "eval_strength_reduce_sub" => {
            if arg_names.len() != 2 {
                return None;
            }
            let x = extract_expr_from_captured(caps.get(&arg_names[0])?)?;
            let n_val = extract_int_from_captured(caps.get(&arg_names[1])?)?;
            if n_val < 1 || n_val >= 64 {
                return None;
            }
            let multiplier = if func == "eval_strength_reduce_add" {
                (1i64 << n_val) + 1
            } else {
                (1i64 << n_val) - 1
            };
            if multiplier <= 1 {
                return None;
            }
            Some(Wrapped {
                item: AstExpression::BinaryOp(
                    AstBinaryOperator::Mul,
                    Box::new(x.clone()),
                    Box::new(Wrapped {
                        item: AstExpression::Literal(AstLiteral::Int(multiplier)),
                        origin: source.origin.clone(),
                        comment: None,
                    }),
                ),
                origin: source.origin.clone(),
                comment: source.comment.clone(),
            })
        }
        // eval_magic_division($x, $magic, $shift): (x * magic) >> shift → x / divisor
        "eval_magic_division" => {
            if arg_names.len() != 3 {
                return None;
            }
            let x = extract_expr_from_captured(caps.get(&arg_names[0])?)?;
            let magic_captured = caps.get(&arg_names[1])?;
            let shift_captured = caps.get(&arg_names[2])?;

            // Extract the magic constant (supports signed via Int negative)
            let magic_expr = match magic_captured {
                Captured::Expression(e) => &e.item,
                Captured::ExpressionBox(e) => &e.item,
                Captured::Literal(lit) => {
                    // Build a temporary expression to reuse extract_magic_constant
                    let tmp = AstExpression::Literal(lit.clone());
                    let (magic_val, is_signed) =
                        crate::abstract_syntax_tree::optimize::magic_division_recovery::extract_magic_constant(&tmp)?;
                    let shift_val = extract_uint_from_captured(shift_captured)?;
                    let divisor =
                        crate::abstract_syntax_tree::optimize::magic_division_recovery::try_recover_division(
                            if is_signed { magic_val as u64 } else { magic_val },
                            shift_val,
                        )?;
                    let lit = if is_signed {
                        AstLiteral::Int(divisor as i64)
                    } else {
                        AstLiteral::UInt(divisor)
                    };
                    return Some(Wrapped {
                        item: AstExpression::BinaryOp(
                            AstBinaryOperator::Div,
                            Box::new(x.clone()),
                            Box::new(Wrapped {
                                item: AstExpression::Literal(lit),
                                origin: source.origin.clone(),
                                comment: None,
                            }),
                        ),
                        origin: source.origin.clone(),
                        comment: source.comment.clone(),
                    });
                }
                _ => return None,
            };
            let (magic_val, is_signed) =
                crate::abstract_syntax_tree::optimize::magic_division_recovery::extract_magic_constant(magic_expr)?;
            let shift_val = extract_uint_from_captured(shift_captured)?;
            let divisor =
                crate::abstract_syntax_tree::optimize::magic_division_recovery::try_recover_division(
                    if is_signed { magic_val as u64 } else { magic_val },
                    shift_val,
                )?;
            let lit = if is_signed {
                AstLiteral::Int(divisor as i64)
            } else {
                AstLiteral::UInt(divisor)
            };
            Some(Wrapped {
                item: AstExpression::BinaryOp(
                    AstBinaryOperator::Div,
                    Box::new(x.clone()),
                    Box::new(Wrapped {
                        item: AstExpression::Literal(lit),
                        origin: source.origin.clone(),
                        comment: None,
                    }),
                ),
                origin: source.origin.clone(),
                comment: source.comment.clone(),
            })
        }
        "eval_strength_reduce_dual" => {
            if arg_names.len() != 3 {
                return None;
            }
            let x = extract_expr_from_captured(caps.get(&arg_names[0])?)?;
            let n_val = extract_int_from_captured(caps.get(&arg_names[1])?)?;
            let m_val = extract_int_from_captured(caps.get(&arg_names[2])?)?;
            if n_val < 1 || n_val >= 64 || m_val < 1 || m_val >= 64 {
                return None;
            }
            let multiplier = (1i64 << n_val) + (1i64 << m_val);
            if multiplier <= 1 {
                return None;
            }
            Some(Wrapped {
                item: AstExpression::BinaryOp(
                    AstBinaryOperator::Mul,
                    Box::new(x.clone()),
                    Box::new(Wrapped {
                        item: AstExpression::Literal(AstLiteral::Int(multiplier)),
                        origin: source.origin.clone(),
                        comment: None,
                    }),
                ),
                origin: source.origin.clone(),
                comment: source.comment.clone(),
            })
        }
        _ => None,
    }
}

fn extract_uint_from_captured(captured: &Captured) -> Option<u64> {
    match captured {
        Captured::Literal(AstLiteral::UInt(n)) => Some(*n),
        Captured::Literal(AstLiteral::Int(n)) if *n >= 0 => Some(*n as u64),
        Captured::Expression(e) => match &e.item {
            AstExpression::Literal(AstLiteral::UInt(n)) => Some(*n),
            AstExpression::Literal(AstLiteral::Int(n)) if *n >= 0 => Some(*n as u64),
            _ => None,
        },
        Captured::ExpressionBox(e) => match &e.item {
            AstExpression::Literal(AstLiteral::UInt(n)) => Some(*n),
            AstExpression::Literal(AstLiteral::Int(n)) if *n >= 0 => Some(*n as u64),
            _ => None,
        },
        _ => None,
    }
}

fn extract_int_from_captured(captured: &Captured) -> Option<i64> {
    match captured {
        Captured::Literal(AstLiteral::Int(n)) => Some(*n),
        Captured::Expression(e) => match &e.item {
            AstExpression::Literal(AstLiteral::Int(n)) => Some(*n),
            _ => None,
        },
        Captured::ExpressionBox(e) => match &e.item {
            AstExpression::Literal(AstLiteral::Int(n)) => Some(*n),
            _ => None,
        },
        _ => None,
    }
}

fn is_reassociable_op(op: &AstBinaryOperator) -> bool {
    matches!(
        op,
        AstBinaryOperator::Add
            | AstBinaryOperator::Mul
            | AstBinaryOperator::BitAnd
            | AstBinaryOperator::BitOr
            | AstBinaryOperator::BitXor
    )
}

fn extract_expr_from_captured(captured: &Captured) -> Option<&Wrapped<AstExpression>> {
    match captured {
        Captured::Expression(e) => Some(e),
        Captured::ExpressionBox(e) => Some(e.as_ref()),
        _ => None,
    }
}

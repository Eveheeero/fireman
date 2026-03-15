use super::{
    node_name::{CaptureRef, FitsTarget, FitsTypeName},
    types::{
        BinaryCapturePredicate, CallNameMatchesPredicate, Captured, Captures,
        CaseInsensitivePattern, FitsPredicate, StmtListPredicate, StmtListPredicateKind,
        SumEqualsPredicate, SumEqualsTarget, UnaryCapturePredicate, WherePredicate,
    },
};
use crate::abstract_syntax_tree::{
    AstCall, AstExpression, AstLiteral, AstStatement, AstUnaryOperator, AstValueType,
    WrappedAstStatement,
};

fn parse_capture_ref(value: &str) -> CaptureRef {
    CaptureRef::new(value)
}

fn capture<'a>(caps: &'a Captures, name: &CaptureRef) -> Option<&'a Captured> {
    caps.get(name.as_str())
}

fn unary(capture: CaptureRef) -> UnaryCapturePredicate {
    UnaryCapturePredicate { capture }
}

fn binary(left: CaptureRef, right: CaptureRef) -> BinaryCapturePredicate {
    BinaryCapturePredicate { left, right }
}

fn stmt_list(capture: CaptureRef, kind: StmtListPredicateKind) -> StmtListPredicate {
    StmtListPredicate { capture, kind }
}

pub fn parse_where(input: &str) -> Result<WherePredicate, String> {
    let input = input.trim();
    // Try to parse as a two-argument predicate: name($a, $b)
    // Single-argument predicates
    if let Some(rest) = input.strip_prefix("fits(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let parts: Vec<&str> = rest.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(format!("fits() requires exactly 2 arguments: {input}"));
        }
        let capture_name =
            parse_capture_ref(parts[0].trim().strip_prefix('$').ok_or_else(|| {
                format!(
                    "fits() first argument must be a capture ($name): {}",
                    parts[0].trim()
                )
            })?);
        let type_arg = parts[1].trim();
        let target = if let Some(cap) = type_arg.strip_prefix('$') {
            FitsTarget::Capture(parse_capture_ref(cap))
        } else {
            let ty = match type_arg {
                "Int8" => FitsTypeName::Int8,
                "Int16" => FitsTypeName::Int16,
                "Int32" => FitsTypeName::Int32,
                "Int64" => FitsTypeName::Int64,
                "Int" => FitsTypeName::Int,
                _ => {
                    return Err(format!(
                        "fits() second argument must be a type name (Int8, Int16, Int32, Int64, Int) or a $capture, got: {type_arg}"
                    ));
                }
            };
            FitsTarget::TypeName(ty)
        };
        return Ok(WherePredicate::Fits(FitsPredicate {
            capture: capture_name,
            target,
        }));
    }

    if let Some(rest) = input.strip_prefix("is_literal(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "is_literal() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::IsLiteral(unary(name)));
    }

    if let Some(rest) = input.strip_prefix("not_literal(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "not_literal() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::NotLiteral(unary(name)));
    }

    if let Some(rest) = input.strip_prefix("is_pure(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "is_pure() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::IsPure(unary(name)));
    }

    if let Some(rest) = input.strip_prefix("structurally_equal(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let parts: Vec<&str> = rest.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(format!(
                "structurally_equal() requires exactly 2 arguments: {input}"
            ));
        }
        let a = parse_capture_ref(parts[0].trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "structurally_equal() argument must be a capture ($name): {}",
                parts[0].trim()
            )
        })?);
        let b = parse_capture_ref(parts[1].trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "structurally_equal() argument must be a capture ($name): {}",
                parts[1].trim()
            )
        })?);
        return Ok(WherePredicate::StructurallyEqual(binary(a, b)));
    }

    if let Some(rest) = input.strip_prefix("same_discriminant(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let parts: Vec<&str> = rest.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(format!(
                "same_discriminant() requires exactly 2 arguments: {input}"
            ));
        }
        let a = parse_capture_ref(parts[0].trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "same_discriminant() argument must be a capture ($name): {}",
                parts[0].trim()
            )
        })?);
        let b = parse_capture_ref(parts[1].trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "same_discriminant() argument must be a capture ($name): {}",
                parts[1].trim()
            )
        })?);
        return Ok(WherePredicate::SameDiscriminant(binary(a, b)));
    }

    if let Some(rest) = input.strip_prefix("greater_count(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let parts: Vec<&str> = rest.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(format!(
                "greater_count() requires exactly 2 arguments: {input}"
            ));
        }
        let a = parse_capture_ref(parts[0].trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "greater_count() argument must be a capture ($name): {}",
                parts[0].trim()
            )
        })?);
        let b = parse_capture_ref(parts[1].trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "greater_count() argument must be a capture ($name): {}",
                parts[1].trim()
            )
        })?);
        return Ok(WherePredicate::GreaterCount(binary(a, b)));
    }

    if let Some(rest) = input.strip_prefix("is_zero(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "is_zero() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::IsZero(unary(name)));
    }

    if let Some(rest) = input.strip_prefix("is_nonzero(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "is_nonzero() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::IsNonZero(unary(name)));
    }

    if let Some(rest) = input.strip_prefix("is_variable(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "is_variable() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::IsVariable(unary(name)));
    }

    if let Some(rest) = input.strip_prefix("sum_equals(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let parts: Vec<&str> = rest.splitn(3, ',').collect();
        if parts.len() != 3 {
            return Err(format!(
                "sum_equals() requires exactly 3 arguments: {input}"
            ));
        }
        let a = parse_capture_ref(parts[0].trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "sum_equals() argument must be a capture ($name): {}",
                parts[0].trim()
            )
        })?);
        let b = parse_capture_ref(parts[1].trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "sum_equals() argument must be a capture ($name): {}",
                parts[1].trim()
            )
        })?);
        let c_raw = parts[2].trim();
        let c = if let Some(cap_name) = c_raw.strip_prefix('$') {
            SumEqualsTarget::Capture(parse_capture_ref(cap_name))
        } else {
            let lit = c_raw.parse::<i64>().map_err(|_| {
                format!(
                    "sum_equals() third argument must be a capture ($name) or integer literal: {}",
                    c_raw
                )
            })?;
            SumEqualsTarget::Literal(lit)
        };
        return Ok(WherePredicate::SumEquals(SumEqualsPredicate {
            left: a,
            right: b,
            target: c,
        }));
    }

    if let Some(rest) = input.strip_prefix("call_name_matches(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let parts: Vec<&str> = rest.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err(format!(
                "call_name_matches() requires exactly 2 arguments: {input}"
            ));
        }
        let capture_name =
            parse_capture_ref(parts[0].trim().strip_prefix('$').ok_or_else(|| {
                format!(
                    "call_name_matches() first argument must be a capture ($name): {}",
                    parts[0].trim()
                )
            })?);
        let pattern_str = parts[1].trim();
        // Accept quoted or unquoted string
        let pattern_str = pattern_str
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(pattern_str)
            .to_string();
        return Ok(WherePredicate::CallNameMatches(CallNameMatchesPredicate {
            capture: capture_name,
            pattern: CaseInsensitivePattern::new(pattern_str),
        }));
    }

    if let Some(rest) = input.strip_prefix("no_unsafe_stmts(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "no_unsafe_stmts() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::StmtList(stmt_list(
            name,
            StmtListPredicateKind::NoUnsafeStmts,
        )));
    }

    if let Some(rest) = input.strip_prefix("ends_with_continue(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "ends_with_continue() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::StmtList(stmt_list(
            name,
            StmtListPredicateKind::EndsWithContinue,
        )));
    }

    if let Some(rest) = input.strip_prefix("is_end_if_not_cond_break(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "is_end_if_not_cond_break() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::StmtList(stmt_list(
            name,
            StmtListPredicateKind::EndsWithIfNotCondBreak,
        )));
    }

    if let Some(rest) = input.strip_prefix("is_end_if_cond_else_break(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "is_end_if_cond_else_break() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::StmtList(stmt_list(
            name,
            StmtListPredicateKind::EndsWithIfCondElseBreak,
        )));
    }

    if let Some(rest) = input.strip_prefix("is_empty_stmt_list(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "is_empty_stmt_list() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::StmtList(stmt_list(
            name,
            StmtListPredicateKind::IsEmpty,
        )));
    }

    if let Some(rest) = input.strip_prefix("is_nonempty_stmt_list(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "is_nonempty_stmt_list() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::StmtList(stmt_list(
            name,
            StmtListPredicateKind::IsNonEmpty,
        )));
    }

    if let Some(rest) = input.strip_prefix("ends_with_break(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "ends_with_break() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::StmtList(stmt_list(
            name,
            StmtListPredicateKind::EndsWithBreak,
        )));
    }

    if let Some(rest) = input.strip_prefix("ends_with_return(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = parse_capture_ref(rest.trim().strip_prefix('$').ok_or_else(|| {
            format!(
                "ends_with_return() argument must be a capture ($name): {}",
                rest.trim()
            )
        })?);
        return Ok(WherePredicate::StmtList(stmt_list(
            name,
            StmtListPredicateKind::EndsWithReturn,
        )));
    }

    let (pred_name, rest) = if let Some(rest) = input.strip_prefix("eq(") {
        ("eq", rest)
    } else if let Some(rest) = input.strip_prefix("ne(") {
        ("ne", rest)
    } else {
        return Err(format!("unknown where predicate: {input}"));
    };
    let rest = rest
        .strip_suffix(')')
        .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
    let parts: Vec<&str> = rest.splitn(2, ',').collect();
    if parts.len() != 2 {
        return Err(format!(
            "{pred_name}() requires exactly 2 arguments: {input}"
        ));
    }
    let a = parts[0].trim();
    let b = parts[1].trim();
    let a = parse_capture_ref(
        a.strip_prefix('$')
            .ok_or_else(|| format!("{pred_name}() argument must be a capture ($name): {a}"))?,
    );
    let b = parse_capture_ref(
        b.strip_prefix('$')
            .ok_or_else(|| format!("{pred_name}() argument must be a capture ($name): {b}"))?,
    );
    match pred_name {
        "eq" => Ok(WherePredicate::Eq(binary(a, b))),
        "ne" => Ok(WherePredicate::Ne(binary(a, b))),
        _ => unreachable!(),
    }
}

pub fn eval_where(pred: &WherePredicate, caps: &Captures) -> bool {
    match pred {
        WherePredicate::Eq(pair) => {
            let Some(cap_a) = capture(caps, &pair.left) else {
                return false;
            };
            let Some(cap_b) = capture(caps, &pair.right) else {
                return false;
            };
            captured_structurally_equal(cap_a, cap_b)
        }
        WherePredicate::Ne(pair) => {
            let Some(cap_a) = capture(caps, &pair.left) else {
                return false;
            };
            let Some(cap_b) = capture(caps, &pair.right) else {
                return false;
            };
            !captured_structurally_equal(cap_a, cap_b)
        }
        WherePredicate::StmtList(predicate) => {
            let Some(captured) = capture(caps, &predicate.capture) else {
                return false;
            };
            match (&predicate.kind, captured) {
                (StmtListPredicateKind::NoUnsafeStmts, Captured::StmtList(stmts)) => {
                    !branch_contains_unsafe_stmts(stmts)
                }
                (StmtListPredicateKind::NoUnsafeStmts, Captured::OptStmtList(Some(stmts))) => {
                    !branch_contains_unsafe_stmts(stmts)
                }
                (StmtListPredicateKind::NoUnsafeStmts, Captured::OptStmtList(None)) => true,
                (StmtListPredicateKind::EndsWithContinue, Captured::StmtList(stmts)) => {
                    stmt_list_ends_with_continue(stmts)
                }
                (StmtListPredicateKind::EndsWithContinue, Captured::OptStmtList(Some(stmts))) => {
                    stmt_list_ends_with_continue(stmts)
                }
                (StmtListPredicateKind::EndsWithIfNotCondBreak, Captured::StmtList(stmts)) => {
                    stmt_list_ends_with_if_not_cond_break(stmts)
                }
                (
                    StmtListPredicateKind::EndsWithIfNotCondBreak,
                    Captured::OptStmtList(Some(stmts)),
                ) => stmt_list_ends_with_if_not_cond_break(stmts),
                (StmtListPredicateKind::EndsWithIfCondElseBreak, Captured::StmtList(stmts)) => {
                    stmt_list_ends_with_if_cond_else_break(stmts)
                }
                (
                    StmtListPredicateKind::EndsWithIfCondElseBreak,
                    Captured::OptStmtList(Some(stmts)),
                ) => stmt_list_ends_with_if_cond_else_break(stmts),
                (StmtListPredicateKind::IsEmpty, Captured::StmtList(stmts)) => {
                    stmt_list_is_empty(stmts)
                }
                (StmtListPredicateKind::IsEmpty, Captured::OptStmtList(Some(stmts))) => {
                    stmt_list_is_empty(stmts)
                }
                (StmtListPredicateKind::IsEmpty, Captured::OptStmtList(None)) => true,
                (StmtListPredicateKind::IsNonEmpty, Captured::StmtList(stmts)) => {
                    !stmt_list_is_empty(stmts)
                }
                (StmtListPredicateKind::IsNonEmpty, Captured::OptStmtList(Some(stmts))) => {
                    !stmt_list_is_empty(stmts)
                }
                (StmtListPredicateKind::EndsWithBreak, Captured::StmtList(stmts)) => {
                    stmt_list_ends_with_break(stmts)
                }
                (StmtListPredicateKind::EndsWithBreak, Captured::OptStmtList(Some(stmts))) => {
                    stmt_list_ends_with_break(stmts)
                }
                (StmtListPredicateKind::EndsWithReturn, Captured::StmtList(stmts)) => {
                    stmt_list_ends_with_return(stmts)
                }
                (StmtListPredicateKind::EndsWithReturn, Captured::OptStmtList(Some(stmts))) => {
                    stmt_list_ends_with_return(stmts)
                }
                _ => false,
            }
        }
        WherePredicate::Fits(predicate) => {
            let Some(captured) = capture(caps, &predicate.capture) else {
                return false;
            };
            let n = match captured {
                Captured::Literal(AstLiteral::Int(n)) => *n,
                _ => return false,
            };
            let fits_type = match &predicate.target {
                FitsTarget::TypeName(ty) => *ty,
                FitsTarget::Capture(var_name) => match capture(caps, var_name) {
                    Some(Captured::ValueType(ty)) => match value_type_to_fits_type(ty) {
                        Some(ft) => ft,
                        None => return false,
                    },
                    _ => return false,
                },
            };
            match fits_type {
                FitsTypeName::Int8 => n >= i64::from(i8::MIN) && n <= i64::from(i8::MAX),
                FitsTypeName::Int16 => n >= i64::from(i16::MIN) && n <= i64::from(i16::MAX),
                FitsTypeName::Int32 => n >= i64::from(i32::MIN) && n <= i64::from(i32::MAX),
                FitsTypeName::Int64 | FitsTypeName::Int => true,
            }
        }
        WherePredicate::IsLiteral(predicate) => {
            let Some(captured) = capture(caps, &predicate.capture) else {
                return false;
            };
            matches!(
                captured,
                Captured::Expression(e) if matches!(e.item, AstExpression::Literal(_))
            ) || matches!(
                captured,
                Captured::ExpressionBox(e) if matches!(e.item, AstExpression::Literal(_))
            ) || matches!(captured, Captured::Literal(_))
        }
        WherePredicate::NotLiteral(predicate) => {
            let Some(captured) = capture(caps, &predicate.capture) else {
                return false;
            };
            match captured {
                Captured::Expression(e) => !matches!(e.item, AstExpression::Literal(_)),
                Captured::ExpressionBox(e) => !matches!(e.item, AstExpression::Literal(_)),
                Captured::Literal(_) => false,
                _ => true,
            }
        }
        WherePredicate::IsPure(predicate) => {
            let Some(captured) = capture(caps, &predicate.capture) else {
                return false;
            };
            match captured {
                Captured::Expression(e) => {
                    crate::abstract_syntax_tree::optimize::opt_utils::is_pure_expression(&e.item)
                }
                Captured::ExpressionBox(e) => {
                    crate::abstract_syntax_tree::optimize::opt_utils::is_pure_expression(&e.item)
                }
                Captured::Literal(_) => true,
                _ => false,
            }
        }
        WherePredicate::StructurallyEqual(pair) => {
            let Some(cap_a) = capture(caps, &pair.left) else {
                return false;
            };
            let Some(cap_b) = capture(caps, &pair.right) else {
                return false;
            };
            captured_structurally_equal(cap_a, cap_b)
        }
        WherePredicate::SameDiscriminant(pair) => {
            let Some(cap_a) = capture(caps, &pair.left) else {
                return false;
            };
            let Some(cap_b) = capture(caps, &pair.right) else {
                return false;
            };
            match (cap_a, cap_b) {
                (Captured::BinaryOp(a), Captured::BinaryOp(b)) => {
                    std::mem::discriminant(a) == std::mem::discriminant(b)
                }
                (Captured::UnaryOp(a), Captured::UnaryOp(b)) => {
                    std::mem::discriminant(a) == std::mem::discriminant(b)
                }
                _ => false,
            }
        }
        WherePredicate::GreaterCount(pair) => {
            let Some(cap_a) = capture(caps, &pair.left) else {
                return false;
            };
            let Some(cap_b) = capture(caps, &pair.right) else {
                return false;
            };
            let len_a = match cap_a {
                Captured::StmtList(stmts) => stmts.len(),
                Captured::OptStmtList(Some(stmts)) => stmts.len(),
                Captured::OptStmtList(None) => 0,
                _ => return false,
            };
            let len_b = match cap_b {
                Captured::StmtList(stmts) => stmts.len(),
                Captured::OptStmtList(Some(stmts)) => stmts.len(),
                Captured::OptStmtList(None) => 0,
                _ => return false,
            };
            len_a > len_b
        }
        WherePredicate::IsZero(predicate) => {
            let Some(captured) = capture(caps, &predicate.capture) else {
                return false;
            };
            match captured {
                Captured::Literal(AstLiteral::Int(0)) => true,
                Captured::Literal(AstLiteral::UInt(0)) => true,
                Captured::Literal(AstLiteral::Bool(false)) => true,
                Captured::Expression(e) => matches!(
                    e.item,
                    AstExpression::Literal(AstLiteral::Int(0))
                        | AstExpression::Literal(AstLiteral::UInt(0))
                        | AstExpression::Literal(AstLiteral::Bool(false))
                ),
                Captured::ExpressionBox(e) => matches!(
                    e.item,
                    AstExpression::Literal(AstLiteral::Int(0))
                        | AstExpression::Literal(AstLiteral::UInt(0))
                        | AstExpression::Literal(AstLiteral::Bool(false))
                ),
                _ => false,
            }
        }
        WherePredicate::IsNonZero(predicate) => {
            let Some(captured) = capture(caps, &predicate.capture) else {
                return false;
            };
            match captured {
                Captured::Literal(AstLiteral::Int(n)) => *n != 0,
                Captured::Literal(AstLiteral::UInt(n)) => *n != 0,
                Captured::Literal(AstLiteral::Bool(b)) => *b,
                Captured::Expression(e) => match &e.item {
                    AstExpression::Literal(AstLiteral::Int(n)) => *n != 0,
                    AstExpression::Literal(AstLiteral::UInt(n)) => *n != 0,
                    AstExpression::Literal(AstLiteral::Bool(b)) => *b,
                    _ => false,
                },
                Captured::ExpressionBox(e) => match &e.item {
                    AstExpression::Literal(AstLiteral::Int(n)) => *n != 0,
                    AstExpression::Literal(AstLiteral::UInt(n)) => *n != 0,
                    AstExpression::Literal(AstLiteral::Bool(b)) => *b,
                    _ => false,
                },
                _ => false,
            }
        }
        WherePredicate::IsVariable(predicate) => {
            let Some(captured) = capture(caps, &predicate.capture) else {
                return false;
            };
            match captured {
                Captured::Expression(e) => matches!(e.item, AstExpression::Variable(_, _)),
                Captured::ExpressionBox(e) => matches!(e.item, AstExpression::Variable(_, _)),
                _ => false,
            }
        }
        WherePredicate::SumEquals(predicate) => {
            let Some(cap_a) = capture(caps, &predicate.left) else {
                return false;
            };
            let Some(cap_b) = capture(caps, &predicate.right) else {
                return false;
            };
            let val_a = match cap_a {
                Captured::Literal(AstLiteral::Int(n)) => *n,
                Captured::Expression(e) => match &e.item {
                    AstExpression::Literal(AstLiteral::Int(n)) => *n,
                    _ => return false,
                },
                Captured::ExpressionBox(e) => match &e.item {
                    AstExpression::Literal(AstLiteral::Int(n)) => *n,
                    _ => return false,
                },
                _ => return false,
            };
            let val_b = match cap_b {
                Captured::Literal(AstLiteral::Int(n)) => *n,
                Captured::Expression(e) => match &e.item {
                    AstExpression::Literal(AstLiteral::Int(n)) => *n,
                    _ => return false,
                },
                Captured::ExpressionBox(e) => match &e.item {
                    AstExpression::Literal(AstLiteral::Int(n)) => *n,
                    _ => return false,
                },
                _ => return false,
            };
            let val_total = match &predicate.target {
                SumEqualsTarget::Literal(n) => *n,
                SumEqualsTarget::Capture(name) => {
                    let Some(cap_total) = capture(caps, name) else {
                        return false;
                    };
                    match cap_total {
                        Captured::Literal(AstLiteral::Int(n)) => *n,
                        Captured::Expression(e) => match &e.item {
                            AstExpression::Literal(AstLiteral::Int(n)) => *n,
                            _ => return false,
                        },
                        Captured::ExpressionBox(e) => match &e.item {
                            AstExpression::Literal(AstLiteral::Int(n)) => *n,
                            _ => return false,
                        },
                        _ => return false,
                    }
                }
            };
            val_a + val_b == val_total
        }
        WherePredicate::CallNameMatches(predicate) => {
            let Some(captured) = capture(caps, &predicate.capture) else {
                return false;
            };
            let call = match captured {
                Captured::Call(c) => c,
                _ => return false,
            };
            let name_string;
            let name: &str = match call {
                AstCall::Unknown(name, _) => name.as_str(),
                AstCall::Variable {
                    var_map, var_id, ..
                } => {
                    let Ok(map) = var_map.read() else {
                        return false;
                    };
                    name_string = map
                        .get(var_id)
                        .and_then(|v| v.name.clone())
                        .unwrap_or_default();
                    &name_string
                }
                _ => return false,
            };
            if name.is_empty() {
                return false;
            }
            predicate.pattern.matches(name)
        }
    }
}

fn value_type_to_fits_type(ty: &AstValueType) -> Option<FitsTypeName> {
    match ty {
        AstValueType::Int8 => Some(FitsTypeName::Int8),
        AstValueType::Int16 => Some(FitsTypeName::Int16),
        AstValueType::Int32 => Some(FitsTypeName::Int32),
        AstValueType::Int64 => Some(FitsTypeName::Int64),
        AstValueType::Int => Some(FitsTypeName::Int),
        _ => None,
    }
}

/// Check if a statement list contains Label, Goto, or Declaration at any nesting depth.
fn branch_contains_unsafe_stmts(stmts: &[WrappedAstStatement]) -> bool {
    for stmt in stmts {
        if stmt_contains_unsafe(&stmt.statement) {
            return true;
        }
    }
    false
}

fn stmt_list_is_empty(stmts: &[WrappedAstStatement]) -> bool {
    stmts.is_empty()
}

fn stmt_list_ends_with_continue(stmts: &[WrappedAstStatement]) -> bool {
    matches!(stmts.last(), Some(last) if matches!(last.statement, AstStatement::Continue))
}

fn stmt_list_ends_with_break(stmts: &[WrappedAstStatement]) -> bool {
    matches!(stmts.last(), Some(last) if matches!(last.statement, AstStatement::Break))
}

fn stmt_list_ends_with_return(stmts: &[WrappedAstStatement]) -> bool {
    matches!(stmts.last(), Some(last) if matches!(last.statement, AstStatement::Return(_)))
}

fn stmt_list_ends_with_if_not_cond_break(stmts: &[WrappedAstStatement]) -> bool {
    let Some(last) = stmts.last() else {
        return false;
    };
    match &last.statement {
        AstStatement::If(inner_cond, branch_true, None) => {
            branch_true.len() == 1
                && matches!(branch_true[0].statement, AstStatement::Break)
                && matches!(
                    inner_cond.item,
                    AstExpression::UnaryOp(AstUnaryOperator::Not, _)
                )
        }
        _ => false,
    }
}

fn stmt_list_ends_with_if_cond_else_break(stmts: &[WrappedAstStatement]) -> bool {
    let Some(last) = stmts.last() else {
        return false;
    };
    match &last.statement {
        AstStatement::If(_, branch_true, Some(branch_false)) => {
            branch_true.is_empty()
                && branch_false.len() == 1
                && matches!(branch_false[0].statement, AstStatement::Break)
        }
        _ => false,
    }
}

fn stmt_contains_unsafe(stmt: &AstStatement) -> bool {
    match stmt {
        AstStatement::Label(_) | AstStatement::Goto(_) | AstStatement::Declaration(_, _) => true,
        AstStatement::If(_, bt, bf) => {
            branch_contains_unsafe_stmts(bt)
                || bf
                    .as_ref()
                    .is_some_and(|bf| branch_contains_unsafe_stmts(bf))
        }
        AstStatement::While(_, body)
        | AstStatement::DoWhile(_, body)
        | AstStatement::Block(body) => branch_contains_unsafe_stmts(body),
        AstStatement::For(init, _, update, body) => {
            stmt_contains_unsafe(&init.statement)
                || stmt_contains_unsafe(&update.statement)
                || branch_contains_unsafe_stmts(body)
        }
        AstStatement::Switch(_, cases, default) => {
            cases
                .iter()
                .any(|(_, body)| branch_contains_unsafe_stmts(body))
                || default
                    .as_ref()
                    .is_some_and(|d| branch_contains_unsafe_stmts(d))
        }
        _ => false,
    }
}

fn captured_structurally_equal(a: &Captured, b: &Captured) -> bool {
    match (a, b) {
        (Captured::VariableId(a), Captured::VariableId(b)) => a == b,
        (Captured::Literal(a), Captured::Literal(b)) => a == b,
        (Captured::UnaryOp(a), Captured::UnaryOp(b)) => {
            std::mem::discriminant(a) == std::mem::discriminant(b)
        }
        (Captured::BinaryOp(a), Captured::BinaryOp(b)) => {
            std::mem::discriminant(a) == std::mem::discriminant(b)
        }
        (Captured::Expression(a), Captured::Expression(b)) => {
            crate::abstract_syntax_tree::optimize::opt_utils::expr_structurally_equal(
                &a.item, &b.item,
            )
        }
        (Captured::ExpressionBox(a), Captured::ExpressionBox(b)) => {
            crate::abstract_syntax_tree::optimize::opt_utils::expr_structurally_equal(
                &a.item, &b.item,
            )
        }
        (Captured::Expression(a), Captured::ExpressionBox(b)) => {
            crate::abstract_syntax_tree::optimize::opt_utils::expr_structurally_equal(
                &a.item, &b.item,
            )
        }
        (Captured::ExpressionBox(a), Captured::Expression(b)) => {
            crate::abstract_syntax_tree::optimize::opt_utils::expr_structurally_equal(
                &a.item, &b.item,
            )
        }
        (Captured::ValueType(a), Captured::ValueType(b)) => a == b,
        _ => false,
    }
}

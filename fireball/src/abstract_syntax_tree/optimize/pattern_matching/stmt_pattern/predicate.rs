use super::{
    node_name::{FitsTarget, FitsTypeName},
    types::{Captured, Captures, SumEqualsTarget, WherePredicate},
};
use crate::abstract_syntax_tree::{
    AstCall, AstExpression, AstLiteral, AstStatement, AstUnaryOperator, AstValueType,
    WrappedAstStatement,
};

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
        let capture_name = parts[0]
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "fits() first argument must be a capture ($name): {}",
                    parts[0].trim()
                )
            })?
            .to_string();
        let type_arg = parts[1].trim();
        let target = if let Some(cap) = type_arg.strip_prefix('$') {
            FitsTarget::Capture(cap.to_string())
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
        return Ok(WherePredicate::Fits(capture_name, target));
    }

    if let Some(rest) = input.strip_prefix("is_literal(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = rest
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "is_literal() argument must be a capture ($name): {}",
                    rest.trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::IsLiteral(name));
    }

    if let Some(rest) = input.strip_prefix("not_literal(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = rest
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "not_literal() argument must be a capture ($name): {}",
                    rest.trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::NotLiteral(name));
    }

    if let Some(rest) = input.strip_prefix("is_pure(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = rest
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "is_pure() argument must be a capture ($name): {}",
                    rest.trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::IsPure(name));
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
        let a = parts[0]
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "structurally_equal() argument must be a capture ($name): {}",
                    parts[0].trim()
                )
            })?
            .to_string();
        let b = parts[1]
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "structurally_equal() argument must be a capture ($name): {}",
                    parts[1].trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::StructurallyEqual(a, b));
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
        let a = parts[0]
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "same_discriminant() argument must be a capture ($name): {}",
                    parts[0].trim()
                )
            })?
            .to_string();
        let b = parts[1]
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "same_discriminant() argument must be a capture ($name): {}",
                    parts[1].trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::SameDiscriminant(a, b));
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
        let a = parts[0]
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "greater_count() argument must be a capture ($name): {}",
                    parts[0].trim()
                )
            })?
            .to_string();
        let b = parts[1]
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "greater_count() argument must be a capture ($name): {}",
                    parts[1].trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::GreaterCount(a, b));
    }

    if let Some(rest) = input.strip_prefix("is_zero(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = rest
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "is_zero() argument must be a capture ($name): {}",
                    rest.trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::IsZero(name));
    }

    if let Some(rest) = input.strip_prefix("is_nonzero(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = rest
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "is_nonzero() argument must be a capture ($name): {}",
                    rest.trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::IsNonZero(name));
    }

    if let Some(rest) = input.strip_prefix("is_variable(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = rest
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "is_variable() argument must be a capture ($name): {}",
                    rest.trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::IsVariable(name));
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
        let a = parts[0]
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "sum_equals() argument must be a capture ($name): {}",
                    parts[0].trim()
                )
            })?
            .to_string();
        let b = parts[1]
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "sum_equals() argument must be a capture ($name): {}",
                    parts[1].trim()
                )
            })?
            .to_string();
        let c_raw = parts[2].trim();
        let c = if let Some(cap_name) = c_raw.strip_prefix('$') {
            SumEqualsTarget::Capture(cap_name.to_string())
        } else {
            let lit = c_raw.parse::<i64>().map_err(|_| {
                format!(
                    "sum_equals() third argument must be a capture ($name) or integer literal: {}",
                    c_raw
                )
            })?;
            SumEqualsTarget::Literal(lit)
        };
        return Ok(WherePredicate::SumEquals(a, b, c));
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
        let capture_name = parts[0]
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "call_name_matches() first argument must be a capture ($name): {}",
                    parts[0].trim()
                )
            })?
            .to_string();
        let pattern_str = parts[1].trim();
        // Accept quoted or unquoted string
        let pattern_str = pattern_str
            .strip_prefix('"')
            .and_then(|s| s.strip_suffix('"'))
            .unwrap_or(pattern_str)
            .to_string();
        return Ok(WherePredicate::CallNameMatches(capture_name, pattern_str));
    }

    if let Some(rest) = input.strip_prefix("no_unsafe_stmts(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = rest
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "no_unsafe_stmts() argument must be a capture ($name): {}",
                    rest.trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::NoUnsafeStmts(name));
    }

    if let Some(rest) = input.strip_prefix("ends_with_continue(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = rest
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "ends_with_continue() argument must be a capture ($name): {}",
                    rest.trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::EndsWithContinue(name));
    }

    if let Some(rest) = input.strip_prefix("is_end_if_not_cond_break(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = rest
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "is_end_if_not_cond_break() argument must be a capture ($name): {}",
                    rest.trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::IsEndIfNotCondBreak(name));
    }

    if let Some(rest) = input.strip_prefix("is_end_if_cond_else_break(") {
        let rest = rest
            .strip_suffix(')')
            .ok_or_else(|| format!("missing closing ')' in where predicate: {input}"))?;
        let name = rest
            .trim()
            .strip_prefix('$')
            .ok_or_else(|| {
                format!(
                    "is_end_if_cond_else_break() argument must be a capture ($name): {}",
                    rest.trim()
                )
            })?
            .to_string();
        return Ok(WherePredicate::IsEndIfCondElseBreak(name));
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
    let a = a
        .strip_prefix('$')
        .ok_or_else(|| format!("{pred_name}() argument must be a capture ($name): {a}"))?
        .to_string();
    let b = b
        .strip_prefix('$')
        .ok_or_else(|| format!("{pred_name}() argument must be a capture ($name): {b}"))?
        .to_string();
    match pred_name {
        "eq" => Ok(WherePredicate::Eq(a, b)),
        "ne" => Ok(WherePredicate::Ne(a, b)),
        _ => unreachable!(),
    }
}

pub fn eval_where(pred: &WherePredicate, caps: &Captures) -> bool {
    match pred {
        WherePredicate::Eq(a, b) => {
            let Some(cap_a) = caps.get(a) else {
                return false;
            };
            let Some(cap_b) = caps.get(b) else {
                return false;
            };
            captured_structurally_equal(cap_a, cap_b)
        }
        WherePredicate::Ne(a, b) => {
            let Some(cap_a) = caps.get(a) else {
                return false;
            };
            let Some(cap_b) = caps.get(b) else {
                return false;
            };
            !captured_structurally_equal(cap_a, cap_b)
        }
        WherePredicate::NoUnsafeStmts(name) => {
            let Some(captured) = caps.get(name) else {
                return false;
            };
            match captured {
                Captured::StmtList(stmts) => !branch_contains_unsafe_stmts(stmts),
                Captured::OptStmtList(Some(stmts)) => !branch_contains_unsafe_stmts(stmts),
                Captured::OptStmtList(None) => true,
                _ => false,
            }
        }
        WherePredicate::EndsWithContinue(name) => {
            let Some(captured) = caps.get(name) else {
                return false;
            };
            match captured {
                Captured::StmtList(stmts) => stmt_list_ends_with_continue(stmts),
                Captured::OptStmtList(Some(stmts)) => stmt_list_ends_with_continue(stmts),
                Captured::OptStmtList(None) => false,
                _ => false,
            }
        }
        WherePredicate::IsEndIfNotCondBreak(name) => {
            let Some(captured) = caps.get(name) else {
                return false;
            };
            match captured {
                Captured::StmtList(stmts) => stmt_list_ends_with_if_not_cond_break(stmts),
                Captured::OptStmtList(Some(stmts)) => stmt_list_ends_with_if_not_cond_break(stmts),
                Captured::OptStmtList(None) => false,
                _ => false,
            }
        }
        WherePredicate::IsEndIfCondElseBreak(name) => {
            let Some(captured) = caps.get(name) else {
                return false;
            };
            match captured {
                Captured::StmtList(stmts) => stmt_list_ends_with_if_cond_else_break(stmts),
                Captured::OptStmtList(Some(stmts)) => stmt_list_ends_with_if_cond_else_break(stmts),
                Captured::OptStmtList(None) => false,
                _ => false,
            }
        }
        WherePredicate::Fits(capture_name, target) => {
            let Some(captured) = caps.get(capture_name) else {
                return false;
            };
            let n = match captured {
                Captured::Literal(AstLiteral::Int(n)) => *n,
                _ => return false,
            };
            let fits_type = match target {
                FitsTarget::TypeName(ty) => *ty,
                FitsTarget::Capture(var_name) => match caps.get(var_name) {
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
        WherePredicate::IsLiteral(name) => {
            let Some(captured) = caps.get(name) else {
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
        WherePredicate::NotLiteral(name) => {
            let Some(captured) = caps.get(name) else {
                return false;
            };
            match captured {
                Captured::Expression(e) => !matches!(e.item, AstExpression::Literal(_)),
                Captured::ExpressionBox(e) => !matches!(e.item, AstExpression::Literal(_)),
                Captured::Literal(_) => false,
                _ => true,
            }
        }
        WherePredicate::IsPure(name) => {
            let Some(captured) = caps.get(name) else {
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
        WherePredicate::StructurallyEqual(a, b) => {
            let Some(cap_a) = caps.get(a) else {
                return false;
            };
            let Some(cap_b) = caps.get(b) else {
                return false;
            };
            captured_structurally_equal(cap_a, cap_b)
        }
        WherePredicate::SameDiscriminant(a, b) => {
            let Some(cap_a) = caps.get(a) else {
                return false;
            };
            let Some(cap_b) = caps.get(b) else {
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
        WherePredicate::GreaterCount(a, b) => {
            let Some(cap_a) = caps.get(a) else {
                return false;
            };
            let Some(cap_b) = caps.get(b) else {
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
        WherePredicate::IsZero(name) => {
            let Some(captured) = caps.get(name) else {
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
        WherePredicate::IsNonZero(name) => {
            let Some(captured) = caps.get(name) else {
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
        WherePredicate::IsVariable(name) => {
            let Some(captured) = caps.get(name) else {
                return false;
            };
            match captured {
                Captured::Expression(e) => matches!(e.item, AstExpression::Variable(_, _)),
                Captured::ExpressionBox(e) => matches!(e.item, AstExpression::Variable(_, _)),
                _ => false,
            }
        }
        WherePredicate::SumEquals(a, b, target) => {
            let Some(cap_a) = caps.get(a) else {
                return false;
            };
            let Some(cap_b) = caps.get(b) else {
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
            let val_total = match target {
                SumEqualsTarget::Literal(n) => *n,
                SumEqualsTarget::Capture(name) => {
                    let Some(cap_total) = caps.get(name) else {
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
        WherePredicate::CallNameMatches(capture_name, pattern) => {
            let Some(captured) = caps.get(capture_name) else {
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
            let lower = name.to_ascii_lowercase();
            let pattern_lower = pattern.to_ascii_lowercase();
            lower.contains(&pattern_lower)
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

fn stmt_list_ends_with_continue(stmts: &[WrappedAstStatement]) -> bool {
    matches!(stmts.last(), Some(last) if matches!(last.statement, AstStatement::Continue))
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

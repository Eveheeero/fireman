//! Structural AST pattern matching with named captures, predicates, and emit.
//!
//! Parses patterns like `If($cond, [Assignment(Variable($_, $v1), $a)], Some([Assignment(Variable($_, $v2), $b)]))`
//! into a `PatTree`, matches them against `AstStatement` nodes to produce `Captures`,
//! evaluates `where` predicates (e.g. `eq($v1, $v2)`), and constructs replacement
//! statements via `emit`.

use crate::abstract_syntax_tree::{
    ArcAstVariableMap, AstBinaryOperator, AstBuiltinFunctionArgument, AstCall, AstExpression,
    AstLiteral, AstStatement, AstStatementOrigin, AstUnaryOperator, AstValueOrigin, AstValueType,
    AstVariable, AstVariableId, Wrapped, WrappedAstStatement,
};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Pattern tree
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum PatTree {
    /// `$name` — captures a subtree
    Capture(String),
    /// `_` — matches anything, discards
    Wildcard,
    /// `Name(child1, child2, ...)` — matches a constructor
    Node {
        name: String,
        children: Vec<PatTree>,
    },
    /// `[a, b, c]` — matches a Vec
    List(Vec<PatTree>),
    /// `Some(inner)` — matches Option::Some
    OptionSome(Box<PatTree>),
    /// `None` — matches Option::None
    OptionNone,
}

// ---------------------------------------------------------------------------
// Captured values
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Captured {
    Statement(AstStatement),
    Expression(Wrapped<AstExpression>),
    ExpressionBox(Box<Wrapped<AstExpression>>),
    VariableId(AstVariableId),
    VariableMap(ArcAstVariableMap),
    Literal(AstLiteral),
    StmtList(Vec<WrappedAstStatement>),
    OptStmtList(Option<Vec<WrappedAstStatement>>),
    OptExpression(Option<Wrapped<AstExpression>>),
    UnaryOp(AstUnaryOperator),
    BinaryOp(AstBinaryOperator),
    Variable(AstVariable),
    ValueType(AstValueType),
    Call(AstCall),
}

pub type Captures = HashMap<String, Captured>;

// ---------------------------------------------------------------------------
// Where predicates
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum WherePredicate {
    Eq(String, String),
    Ne(String, String),
    /// Check that a captured StmtList does not contain Label, Goto, or Declaration at any depth.
    NoUnsafeStmts(String),
    /// Check that a captured literal fits in the named type's range.
    /// `fits($lit_capture, TypeName)` where TypeName is e.g. Int8, Int16, Int32, Int64, Int.
    Fits(String, String),
    /// Check that a captured expression is a literal value.
    IsLiteral(String),
    /// Check that a captured expression is NOT a literal value.
    NotLiteral(String),
    /// Check that a captured expression is pure (no side effects: no calls, deref, etc.).
    IsPure(String),
    /// Check that two captured expressions are structurally equal.
    StructurallyEqual(String, String),
    /// Check that two captured values have the same discriminant (e.g. same operator variant).
    SameDiscriminant(String, String),
    /// Check that the first captured StmtList has more statements than the second.
    GreaterCount(String, String),
    /// Check that a captured literal is zero (Int(0), UInt(0), Bool(false)).
    IsZero(String),
    /// Check that a captured literal is non-zero (Int(n) where n!=0, UInt(n) where n!=0, Bool(true)).
    IsNonZero(String),
    /// Check that a captured expression is a Variable (not a complex expression).
    IsVariable(String),
    /// Check that two captured Int literals sum to a specific value (capture or literal).
    SumEquals(String, String, SumEqualsTarget),
    /// Check that a captured Call's name contains a given substring (case-insensitive).
    /// `call_name_matches($f, "pattern")`
    CallNameMatches(String, String),
}

/// The third operand of `sum_equals()`: either a capture reference or an inline integer literal.
#[derive(Debug, Clone)]
pub enum SumEqualsTarget {
    Capture(String),
    Literal(i64),
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
        let type_arg = parts[1].trim().to_string();
        // Accept either a literal type name or a $capture variable
        if !type_arg.starts_with('$') {
            match type_arg.as_str() {
                "Int8" | "Int16" | "Int32" | "Int64" | "Int" => {}
                _ => {
                    return Err(format!(
                        "fits() second argument must be a type name (Int8, Int16, Int32, Int64, Int) or a $capture, got: {type_arg}"
                    ));
                }
            }
        }
        return Ok(WherePredicate::Fits(capture_name, type_arg));
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
        WherePredicate::Fits(capture_name, type_arg) => {
            let Some(captured) = caps.get(capture_name) else {
                return false;
            };
            let n = match captured {
                Captured::Literal(AstLiteral::Int(n)) => *n,
                _ => return false,
            };
            // Resolve the type name: if it starts with '$', look up the capture
            let resolved_type = if let Some(var_name) = type_arg.strip_prefix('$') {
                match caps.get(var_name) {
                    Some(Captured::ValueType(ty)) => value_type_to_fits_name(ty),
                    _ => return false,
                }
            } else {
                type_arg.as_str()
            };
            match resolved_type {
                "Int8" => n >= i64::from(i8::MIN) && n <= i64::from(i8::MAX),
                "Int16" => n >= i64::from(i16::MIN) && n <= i64::from(i16::MAX),
                "Int32" => n >= i64::from(i32::MIN) && n <= i64::from(i32::MAX),
                "Int64" | "Int" => true,
                _ => false,
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

fn value_type_to_fits_name(ty: &AstValueType) -> &'static str {
    match ty {
        AstValueType::Int8 => "Int8",
        AstValueType::Int16 => "Int16",
        AstValueType::Int32 => "Int32",
        AstValueType::Int64 => "Int64",
        AstValueType::Int => "Int",
        _ => "",
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
            super::super::opt_utils::expr_structurally_equal(&a.item, &b.item)
        }
        (Captured::ExpressionBox(a), Captured::ExpressionBox(b)) => {
            super::super::opt_utils::expr_structurally_equal(&a.item, &b.item)
        }
        (Captured::Expression(a), Captured::ExpressionBox(b)) => {
            super::super::opt_utils::expr_structurally_equal(&a.item, &b.item)
        }
        (Captured::ExpressionBox(a), Captured::Expression(b)) => {
            super::super::opt_utils::expr_structurally_equal(&a.item, &b.item)
        }
        (Captured::ValueType(a), Captured::ValueType(b)) => a == b,
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Pattern parser (tokenizer + recursive descent)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Capture(String), // $name
    Wildcard,        // _
    Number(i64),     // integer literal (positive or negative)
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\n' | '\r' => i += 1,
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            '[' => {
                tokens.push(Token::LBracket);
                i += 1;
            }
            ']' => {
                tokens.push(Token::RBracket);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            '$' => {
                i += 1;
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                if i == start {
                    return Err("expected capture name after '$'".to_string());
                }
                tokens.push(Token::Capture(chars[start..i].iter().collect()));
            }
            '-' if i + 1 < chars.len() && chars[i + 1].is_ascii_digit() => {
                i += 1; // skip '-'
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let n: i64 = num_str
                    .parse()
                    .map_err(|e| format!("invalid number literal -{num_str}: {e}"))?;
                tokens.push(Token::Number(-n));
            }
            c if c.is_ascii_digit() => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let n: i64 = num_str
                    .parse()
                    .map_err(|e| format!("invalid number literal {num_str}: {e}"))?;
                tokens.push(Token::Number(n));
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                if word == "_" {
                    tokens.push(Token::Wildcard);
                } else {
                    tokens.push(Token::Ident(word));
                }
            }
            other => return Err(format!("unexpected character in pattern: '{other}'")),
        }
    }
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.next() {
            Some(ref t) if t == expected => Ok(()),
            Some(t) => Err(format!("expected {expected:?}, got {t:?}")),
            None => Err(format!("expected {expected:?}, got end of input")),
        }
    }

    fn parse_tree(&mut self) -> Result<PatTree, String> {
        match self.next() {
            Some(Token::Capture(name)) => Ok(PatTree::Capture(name)),
            Some(Token::Wildcard) => Ok(PatTree::Wildcard),
            Some(Token::Number(n)) => Ok(PatTree::Node {
                name: format!("{n}"),
                children: Vec::new(),
            }),
            Some(Token::LBracket) => {
                let mut items = Vec::new();
                if self.peek() != Some(&Token::RBracket) {
                    items.push(self.parse_tree()?);
                    while self.peek() == Some(&Token::Comma) {
                        self.next();
                        if self.peek() == Some(&Token::RBracket) {
                            break;
                        }
                        items.push(self.parse_tree()?);
                    }
                }
                self.expect(&Token::RBracket)?;
                Ok(PatTree::List(items))
            }
            Some(Token::Ident(name)) => {
                if name == "None" {
                    if self.peek() == Some(&Token::LParen) {
                        // None() — still treat as None
                        self.next();
                        self.expect(&Token::RParen)?;
                    }
                    return Ok(PatTree::OptionNone);
                }
                if name == "Some" {
                    self.expect(&Token::LParen)?;
                    let inner = self.parse_tree()?;
                    self.expect(&Token::RParen)?;
                    return Ok(PatTree::OptionSome(Box::new(inner)));
                }
                if self.peek() == Some(&Token::LParen) {
                    self.next();
                    let mut children = Vec::new();
                    if self.peek() != Some(&Token::RParen) {
                        children.push(self.parse_tree()?);
                        while self.peek() == Some(&Token::Comma) {
                            self.next();
                            if self.peek() == Some(&Token::RParen) {
                                break;
                            }
                            children.push(self.parse_tree()?);
                        }
                    }
                    self.expect(&Token::RParen)?;
                    Ok(PatTree::Node { name, children })
                } else {
                    // Bare identifier — zero-arg constructor
                    Ok(PatTree::Node {
                        name,
                        children: Vec::new(),
                    })
                }
            }
            Some(t) => Err(format!("unexpected token: {t:?}")),
            None => Err("unexpected end of pattern".to_string()),
        }
    }
}

pub fn parse_pattern(input: &str) -> Result<PatTree, String> {
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Err("empty pattern".to_string());
    }
    let mut parser = Parser::new(tokens);
    let tree = parser.parse_tree()?;
    if parser.pos < parser.tokens.len() {
        return Err(format!(
            "trailing tokens after pattern: {:?}",
            &parser.tokens[parser.pos..]
        ));
    }
    Ok(tree)
}

// ---------------------------------------------------------------------------
// Structural matcher
// ---------------------------------------------------------------------------

pub fn match_statement(pat: &PatTree, stmt: &AstStatement) -> Option<Captures> {
    let mut caps = Captures::new();
    if match_stmt_inner(pat, stmt, &mut caps) {
        Some(caps)
    } else {
        None
    }
}

fn match_stmt_inner(pat: &PatTree, stmt: &AstStatement, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Statement(stmt.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_stmt_node(name, children, stmt, caps),
        _ => false,
    }
}

fn match_stmt_node(
    name: &str,
    children: &[PatTree],
    stmt: &AstStatement,
    caps: &mut Captures,
) -> bool {
    match (name, stmt) {
        ("Assignment", AstStatement::Assignment(lhs, rhs)) if children.len() == 2 => {
            match_wrapped_expr(&children[0], lhs, caps)
                && match_wrapped_expr(&children[1], rhs, caps)
        }
        ("If", AstStatement::If(cond, branch_true, branch_false)) if children.len() == 3 => {
            match_wrapped_expr(&children[0], cond, caps)
                && match_stmt_list(&children[1], branch_true, caps)
                && match_opt_stmt_list(&children[2], branch_false, caps)
        }
        ("While", AstStatement::While(cond, body)) if children.len() == 2 => {
            match_wrapped_expr(&children[0], cond, caps)
                && match_stmt_list(&children[1], body, caps)
        }
        ("DoWhile", AstStatement::DoWhile(cond, body)) if children.len() == 2 => {
            match_wrapped_expr(&children[0], cond, caps)
                && match_stmt_list(&children[1], body, caps)
        }
        ("For", AstStatement::For(init, cond, update, body)) if children.len() == 4 => {
            match_stmt_inner(&children[0], &init.statement, caps)
                && match_wrapped_expr(&children[1], cond, caps)
                && match_stmt_inner(&children[2], &update.statement, caps)
                && match_stmt_list(&children[3], body, caps)
        }
        ("Return", AstStatement::Return(opt_expr)) if children.len() == 1 => {
            match_opt_wrapped_expr(&children[0], opt_expr, caps)
        }
        ("Return", AstStatement::Return(None)) if children.is_empty() => true,
        ("Block", AstStatement::Block(body)) if children.len() == 1 => {
            match_stmt_list(&children[0], body, caps)
        }
        ("Label", AstStatement::Label(s)) if children.len() == 1 => {
            match_string_pat(&children[0], s, caps)
        }
        ("Comment", AstStatement::Comment(s)) if children.len() == 1 => {
            match_string_pat(&children[0], s, caps)
        }
        ("Assembly", AstStatement::Assembly(s)) if children.len() == 1 => {
            match_string_pat(&children[0], s, caps)
        }
        ("Declaration", AstStatement::Declaration(var, opt_init)) if children.len() == 2 => {
            match_variable_pat(&children[0], var, caps)
                && match_opt_wrapped_expr(&children[1], opt_init, caps)
        }
        ("Call", AstStatement::Call(call)) if children.len() == 1 => match &children[0] {
            PatTree::Capture(name) => {
                caps.insert(name.clone(), Captured::Call(call.clone()));
                true
            }
            PatTree::Wildcard => true,
            _ => false,
        },
        ("Goto", AstStatement::Goto(crate::abstract_syntax_tree::AstJumpTarget::Unknown(s)))
            if children.len() == 1 =>
        {
            match_string_pat(&children[0], s, caps)
        }
        ("Empty", AstStatement::Empty) if children.is_empty() => true,
        ("Break", AstStatement::Break) if children.is_empty() => true,
        ("Continue", AstStatement::Continue) if children.is_empty() => true,
        ("Undefined", AstStatement::Undefined) if children.is_empty() => true,
        _ => false,
    }
}

fn match_wrapped_expr(pat: &PatTree, expr: &Wrapped<AstExpression>, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Expression(expr.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_expr_node(name, children, &expr.item, caps),
        _ => false,
    }
}

fn match_boxed_wrapped_expr(
    pat: &PatTree,
    expr: &Box<Wrapped<AstExpression>>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::ExpressionBox(expr.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_expr_node(name, children, &expr.item, caps),
        _ => false,
    }
}

fn match_expr_node(
    name: &str,
    children: &[PatTree],
    expr: &AstExpression,
    caps: &mut Captures,
) -> bool {
    match (name, expr) {
        ("Variable", AstExpression::Variable(map, var_id)) if children.len() == 2 => {
            match_variable_map_pat(&children[0], map, caps)
                && match_variable_id_pat(&children[1], var_id, caps)
        }
        ("Literal", AstExpression::Literal(lit)) if children.len() == 1 => {
            match_literal_pat(&children[0], lit, caps)
        }
        ("UnaryOp", AstExpression::UnaryOp(op, arg)) if children.len() == 2 => {
            match_unary_op_pat(&children[0], op, caps)
                && match_boxed_wrapped_expr(&children[1], arg, caps)
        }
        ("BinaryOp", AstExpression::BinaryOp(op, lhs, rhs)) if children.len() == 3 => {
            match_binary_op_pat(&children[0], op, caps)
                && match_boxed_wrapped_expr(&children[1], lhs, caps)
                && match_boxed_wrapped_expr(&children[2], rhs, caps)
        }
        ("Ternary", AstExpression::Ternary(cond, t, f)) if children.len() == 3 => {
            match_boxed_wrapped_expr(&children[0], cond, caps)
                && match_boxed_wrapped_expr(&children[1], t, caps)
                && match_boxed_wrapped_expr(&children[2], f, caps)
        }
        ("Cast", AstExpression::Cast(ty, arg)) if children.len() == 2 => {
            match_value_type_pat(&children[0], ty, caps)
                && match_boxed_wrapped_expr(&children[1], arg, caps)
        }
        ("Deref", AstExpression::Deref(arg)) if children.len() == 1 => {
            match_boxed_wrapped_expr(&children[0], arg, caps)
        }
        ("AddressOf", AstExpression::AddressOf(arg)) if children.len() == 1 => {
            match_boxed_wrapped_expr(&children[0], arg, caps)
        }
        ("ArrayAccess", AstExpression::ArrayAccess(base, idx)) if children.len() == 2 => {
            match_boxed_wrapped_expr(&children[0], base, caps)
                && match_boxed_wrapped_expr(&children[1], idx, caps)
        }
        ("MemberAccess", AstExpression::MemberAccess(base, _field)) if children.len() == 2 => {
            match_boxed_wrapped_expr(&children[0], base, caps)
                && match pat_is_wildcard_or_capture(&children[1], caps) {
                    true => true,
                    false => false,
                }
        }
        ("Call", AstExpression::Call(call)) if children.len() == 1 => match &children[0] {
            PatTree::Capture(name) => {
                caps.insert(name.clone(), Captured::Call(call.clone()));
                true
            }
            PatTree::Wildcard => true,
            _ => false,
        },
        ("Unknown", AstExpression::Unknown) if children.is_empty() => true,
        ("Undefined", AstExpression::Undefined) if children.is_empty() => true,
        _ => false,
    }
}

fn pat_is_wildcard_or_capture(pat: &PatTree, _caps: &mut Captures) -> bool {
    matches!(pat, PatTree::Wildcard | PatTree::Capture(_))
}

fn match_variable_map_pat(pat: &PatTree, map: &ArcAstVariableMap, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::VariableMap(map.clone()));
            true
        }
        PatTree::Wildcard => true,
        _ => false,
    }
}

fn match_variable_id_pat(pat: &PatTree, var_id: &AstVariableId, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::VariableId(*var_id));
            true
        }
        PatTree::Wildcard => true,
        _ => false,
    }
}

fn match_value_type_pat(pat: &PatTree, ty: &AstValueType, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::ValueType(ty.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } if children.is_empty() => match (name.as_str(), ty) {
            ("Void", AstValueType::Void) => true,
            ("Unknown", AstValueType::Unknown) => true,
            ("Int", AstValueType::Int) => true,
            ("Int8", AstValueType::Int8) => true,
            ("Int16", AstValueType::Int16) => true,
            ("Int32", AstValueType::Int32) => true,
            ("Int64", AstValueType::Int64) => true,
            ("UInt", AstValueType::UInt) => true,
            ("UInt8", AstValueType::UInt8) => true,
            ("UInt16", AstValueType::UInt16) => true,
            ("UInt32", AstValueType::UInt32) => true,
            ("UInt64", AstValueType::UInt64) => true,
            ("Char", AstValueType::Char) => true,
            ("Float", AstValueType::Float) => true,
            ("Double", AstValueType::Double) => true,
            ("Bool", AstValueType::Bool) => true,
            _ => false,
        },
        _ => false,
    }
}

fn construct_value_type(pat: &PatTree, caps: &Captures) -> Option<AstValueType> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::ValueType(ty) => Some(ty.clone()),
            _ => None,
        },
        PatTree::Node { name, children } if children.is_empty() => match name.as_str() {
            "Void" => Some(AstValueType::Void),
            "Unknown" => Some(AstValueType::Unknown),
            "Int" => Some(AstValueType::Int),
            "Int8" => Some(AstValueType::Int8),
            "Int16" => Some(AstValueType::Int16),
            "Int32" => Some(AstValueType::Int32),
            "Int64" => Some(AstValueType::Int64),
            "UInt" => Some(AstValueType::UInt),
            "UInt8" => Some(AstValueType::UInt8),
            "UInt16" => Some(AstValueType::UInt16),
            "UInt32" => Some(AstValueType::UInt32),
            "UInt64" => Some(AstValueType::UInt64),
            "Char" => Some(AstValueType::Char),
            "Float" => Some(AstValueType::Float),
            "Double" => Some(AstValueType::Double),
            "Bool" => Some(AstValueType::Bool),
            _ => None,
        },
        _ => None,
    }
}

fn match_literal_pat(pat: &PatTree, lit: &AstLiteral, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Literal(lit.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } => match_literal_node(name, children, lit, caps),
        _ => false,
    }
}

fn match_literal_node(
    name: &str,
    children: &[PatTree],
    lit: &AstLiteral,
    caps: &mut Captures,
) -> bool {
    match (name, lit) {
        ("Bool", AstLiteral::Bool(b)) if children.len() == 1 => match &children[0] {
            PatTree::Node {
                name: val,
                children: inner,
            } if inner.is_empty() => match (val.as_str(), b) {
                ("true", true) | ("false", false) => true,
                _ => false,
            },
            PatTree::Capture(cap_name) => {
                caps.insert(cap_name.clone(), Captured::Literal(lit.clone()));
                true
            }
            PatTree::Wildcard => true,
            _ => false,
        },
        ("Int", AstLiteral::Int(n)) if children.len() == 1 => match &children[0] {
            PatTree::Node {
                name: val,
                children: inner,
            } if inner.is_empty() => val.parse::<i64>().ok() == Some(*n),
            PatTree::Capture(cap_name) => {
                caps.insert(cap_name.clone(), Captured::Literal(lit.clone()));
                true
            }
            PatTree::Wildcard => true,
            _ => false,
        },
        ("UInt", AstLiteral::UInt(n)) if children.len() == 1 => match &children[0] {
            PatTree::Node {
                name: val,
                children: inner,
            } if inner.is_empty() => val.parse::<u64>().ok() == Some(*n),
            PatTree::Capture(cap_name) => {
                caps.insert(cap_name.clone(), Captured::Literal(lit.clone()));
                true
            }
            PatTree::Wildcard => true,
            _ => false,
        },
        _ => false,
    }
}

fn match_unary_op_pat(pat: &PatTree, op: &AstUnaryOperator, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::UnaryOp(op.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } if children.is_empty() => match (name.as_str(), op) {
            ("Negate", AstUnaryOperator::Negate) => true,
            ("Not", AstUnaryOperator::Not) => true,
            ("BitNot", AstUnaryOperator::BitNot) => true,
            ("PreInc", AstUnaryOperator::PreInc) => true,
            ("PreDec", AstUnaryOperator::PreDec) => true,
            ("PostInc", AstUnaryOperator::PostInc) => true,
            ("PostDec", AstUnaryOperator::PostDec) => true,
            ("CastSigned", AstUnaryOperator::CastSigned) => true,
            ("CastUnsigned", AstUnaryOperator::CastUnsigned) => true,
            _ => false,
        },
        _ => false,
    }
}

fn match_binary_op_pat(pat: &PatTree, op: &AstBinaryOperator, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::BinaryOp(op.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::Node { name, children } if children.is_empty() => match (name.as_str(), op) {
            ("Add", AstBinaryOperator::Add) => true,
            ("Sub", AstBinaryOperator::Sub) => true,
            ("Mul", AstBinaryOperator::Mul) => true,
            ("Div", AstBinaryOperator::Div) => true,
            ("Mod", AstBinaryOperator::Mod) => true,
            ("BitAnd", AstBinaryOperator::BitAnd) => true,
            ("BitOr", AstBinaryOperator::BitOr) => true,
            ("BitXor", AstBinaryOperator::BitXor) => true,
            ("LogicAnd", AstBinaryOperator::LogicAnd) => true,
            ("LogicOr", AstBinaryOperator::LogicOr) => true,
            ("Equal", AstBinaryOperator::Equal) => true,
            ("NotEqual", AstBinaryOperator::NotEqual) => true,
            ("Less", AstBinaryOperator::Less) => true,
            ("LessEqual", AstBinaryOperator::LessEqual) => true,
            ("Greater", AstBinaryOperator::Greater) => true,
            ("GreaterEqual", AstBinaryOperator::GreaterEqual) => true,
            ("LeftShift", AstBinaryOperator::LeftShift) => true,
            ("RightShift", AstBinaryOperator::RightShift) => true,
            _ => false,
        },
        _ => false,
    }
}

fn match_variable_pat(pat: &PatTree, var: &AstVariable, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::Variable(var.clone()));
            true
        }
        PatTree::Wildcard => true,
        _ => false,
    }
}

fn match_string_pat(pat: &PatTree, s: &str, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Wildcard => true,
        PatTree::Capture(name) => {
            caps.insert(
                name.clone(),
                Captured::Literal(AstLiteral::String(s.to_string())),
            );
            true
        }
        _ => false,
    }
}

fn construct_string(pat: &PatTree, caps: &Captures) -> Option<String> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Literal(AstLiteral::String(s)) => Some(s.clone()),
            _ => None,
        },
        PatTree::Node { name, children } if children.is_empty() => {
            // Bare string literal in the pattern, e.g. Comment("hello")
            Some(name.clone())
        }
        _ => None,
    }
}

fn match_stmt_list(pat: &PatTree, stmts: &Vec<WrappedAstStatement>, caps: &mut Captures) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::StmtList(stmts.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::List(pats) => {
            if pats.len() != stmts.len() {
                return false;
            }
            for (p, s) in pats.iter().zip(stmts.iter()) {
                if !match_stmt_inner(p, &s.statement, caps) {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

fn match_opt_stmt_list(
    pat: &PatTree,
    opt: &Option<Vec<WrappedAstStatement>>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::OptStmtList(opt.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::OptionNone => opt.is_none(),
        PatTree::OptionSome(inner) => match opt {
            Some(stmts) => match_stmt_list(inner, stmts, caps),
            None => false,
        },
        _ => false,
    }
}

fn match_opt_wrapped_expr(
    pat: &PatTree,
    opt: &Option<Wrapped<AstExpression>>,
    caps: &mut Captures,
) -> bool {
    match pat {
        PatTree::Capture(name) => {
            caps.insert(name.clone(), Captured::OptExpression(opt.clone()));
            true
        }
        PatTree::Wildcard => true,
        PatTree::OptionNone => opt.is_none(),
        PatTree::OptionSome(inner) => match opt {
            Some(expr) => match_wrapped_expr(inner, expr, caps),
            None => false,
        },
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Emit (construct replacement)
// ---------------------------------------------------------------------------

pub fn construct_statement(pat: &PatTree, caps: &Captures) -> Option<AstStatement> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Statement(s) => Some(s.clone()),
            _ => None,
        },
        PatTree::Node { name, children } => construct_stmt_node(name, children, caps),
        _ => None,
    }
}

fn construct_stmt_node(name: &str, children: &[PatTree], caps: &Captures) -> Option<AstStatement> {
    match name {
        "Assignment" if children.len() == 2 => {
            let lhs = construct_wrapped_expr(&children[0], caps)?;
            let rhs = construct_wrapped_expr(&children[1], caps)?;
            Some(AstStatement::Assignment(lhs, rhs))
        }
        "If" if children.len() == 3 => {
            let cond = construct_wrapped_expr(&children[0], caps)?;
            let branch_true = construct_stmt_list(&children[1], caps)?;
            let branch_false = construct_opt_stmt_list(&children[2], caps)?;
            Some(AstStatement::If(cond, branch_true, branch_false))
        }
        "While" if children.len() == 2 => {
            let cond = construct_wrapped_expr(&children[0], caps)?;
            let body = construct_stmt_list(&children[1], caps)?;
            Some(AstStatement::While(cond, body))
        }
        "DoWhile" if children.len() == 2 => {
            let cond = construct_wrapped_expr(&children[0], caps)?;
            let body = construct_stmt_list(&children[1], caps)?;
            Some(AstStatement::DoWhile(cond, body))
        }
        "Return" if children.len() == 1 => {
            let opt = construct_opt_wrapped_expr(&children[0], caps)?;
            Some(AstStatement::Return(opt))
        }
        "Return" if children.is_empty() => Some(AstStatement::Return(None)),
        "Block" if children.len() == 1 => {
            let body = construct_stmt_list(&children[0], caps)?;
            Some(AstStatement::Block(body))
        }
        "Call" if children.len() == 1 => {
            let call = construct_call(&children[0], caps)?;
            Some(AstStatement::Call(call))
        }
        "Comment" if children.len() == 1 => {
            let s = construct_string(&children[0], caps)?;
            Some(AstStatement::Comment(s))
        }
        "Label" if children.len() == 1 => {
            let s = construct_string(&children[0], caps)?;
            Some(AstStatement::Label(s))
        }
        "Goto" if children.len() == 1 => {
            let s = construct_string(&children[0], caps)?;
            Some(AstStatement::Goto(
                crate::abstract_syntax_tree::AstJumpTarget::Unknown(s),
            ))
        }
        "Empty" if children.is_empty() => Some(AstStatement::Empty),
        "Break" if children.is_empty() => Some(AstStatement::Break),
        "Continue" if children.is_empty() => Some(AstStatement::Continue),
        _ => None,
    }
}

fn construct_wrapped_expr(pat: &PatTree, caps: &Captures) -> Option<Wrapped<AstExpression>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Expression(e) => Some(e.clone()),
            Captured::ExpressionBox(e) => Some((**e).clone()),
            _ => None,
        },
        PatTree::Node { name, children } => {
            let expr = construct_expr_node(name, children, caps)?;
            // Find an origin from any captured expression to use
            let origin = find_any_origin(caps);
            Some(Wrapped {
                item: expr,
                origin,
                comment: None,
            })
        }
        _ => None,
    }
}

fn construct_boxed_wrapped_expr(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Box<Wrapped<AstExpression>>> {
    construct_wrapped_expr(pat, caps).map(Box::new)
}

fn construct_expr_node(name: &str, children: &[PatTree], caps: &Captures) -> Option<AstExpression> {
    match name {
        "Variable" if children.len() == 2 => {
            let map = construct_variable_map(&children[0], caps)?;
            let var_id = construct_variable_id(&children[1], caps)?;
            Some(AstExpression::Variable(map, var_id))
        }
        "Literal" if children.len() == 1 => {
            let lit = construct_literal(&children[0], caps)?;
            Some(AstExpression::Literal(lit))
        }
        "UnaryOp" if children.len() == 2 => {
            let op = construct_unary_op(&children[0], caps)?;
            let arg = construct_boxed_wrapped_expr(&children[1], caps)?;
            Some(AstExpression::UnaryOp(op, arg))
        }
        "BinaryOp" if children.len() == 3 => {
            let op = construct_binary_op(&children[0], caps)?;
            let lhs = construct_boxed_wrapped_expr(&children[1], caps)?;
            let rhs = construct_boxed_wrapped_expr(&children[2], caps)?;
            Some(AstExpression::BinaryOp(op, lhs, rhs))
        }
        "Ternary" if children.len() == 3 => {
            let cond = construct_boxed_wrapped_expr(&children[0], caps)?;
            let t = construct_boxed_wrapped_expr(&children[1], caps)?;
            let f = construct_boxed_wrapped_expr(&children[2], caps)?;
            Some(AstExpression::Ternary(cond, t, f))
        }
        "Cast" if children.len() == 2 => {
            let ty = construct_value_type(&children[0], caps)?;
            let arg = construct_boxed_wrapped_expr(&children[1], caps)?;
            Some(AstExpression::Cast(ty, arg))
        }
        "Deref" if children.len() == 1 => {
            let arg = construct_boxed_wrapped_expr(&children[0], caps)?;
            Some(AstExpression::Deref(arg))
        }
        "AddressOf" if children.len() == 1 => {
            let arg = construct_boxed_wrapped_expr(&children[0], caps)?;
            Some(AstExpression::AddressOf(arg))
        }
        "ArrayAccess" if children.len() == 2 => {
            let base = construct_boxed_wrapped_expr(&children[0], caps)?;
            let idx = construct_boxed_wrapped_expr(&children[1], caps)?;
            Some(AstExpression::ArrayAccess(base, idx))
        }
        "Call" if children.len() == 1 => {
            let call = construct_call(&children[0], caps)?;
            Some(AstExpression::Call(call))
        }
        "Unknown" if children.is_empty() => Some(AstExpression::Unknown),
        "Undefined" if children.is_empty() => Some(AstExpression::Undefined),
        _ => None,
    }
}

fn construct_call(pat: &PatTree, caps: &Captures) -> Option<AstCall> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Call(c) => Some(c.clone()),
            _ => None,
        },
        _ => None,
    }
}

fn construct_variable_map(pat: &PatTree, caps: &Captures) -> Option<ArcAstVariableMap> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::VariableMap(m) => Some(m.clone()),
            _ => None,
        },
        PatTree::Wildcard => {
            // Fallback: find any VariableMap in captures
            for v in caps.values() {
                if let Captured::VariableMap(m) = v {
                    return Some(m.clone());
                }
            }
            None
        }
        _ => None,
    }
}

fn construct_variable_id(pat: &PatTree, caps: &Captures) -> Option<AstVariableId> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::VariableId(id) => Some(*id),
            _ => None,
        },
        _ => None,
    }
}

fn construct_literal(pat: &PatTree, caps: &Captures) -> Option<AstLiteral> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::Literal(l) => Some(l.clone()),
            _ => None,
        },
        PatTree::Node { name, children } => construct_literal_node(name, children, caps),
        _ => None,
    }
}

fn construct_literal_node(name: &str, children: &[PatTree], caps: &Captures) -> Option<AstLiteral> {
    match name {
        "Bool" if children.len() == 1 => match &children[0] {
            PatTree::Node {
                name: val,
                children: inner,
            } if inner.is_empty() => match val.as_str() {
                "true" => Some(AstLiteral::Bool(true)),
                "false" => Some(AstLiteral::Bool(false)),
                _ => None,
            },
            PatTree::Capture(cap_name) => match caps.get(cap_name)? {
                Captured::Literal(l) => Some(l.clone()),
                _ => None,
            },
            _ => None,
        },
        "Int" if children.len() == 1 => match &children[0] {
            PatTree::Node {
                name: val,
                children: inner,
            } if inner.is_empty() => val.parse::<i64>().ok().map(AstLiteral::Int),
            PatTree::Capture(cap_name) => match caps.get(cap_name)? {
                Captured::Literal(l) => Some(l.clone()),
                _ => None,
            },
            _ => None,
        },
        "UInt" if children.len() == 1 => match &children[0] {
            PatTree::Node {
                name: val,
                children: inner,
            } if inner.is_empty() => val.parse::<u64>().ok().map(AstLiteral::UInt),
            PatTree::Capture(cap_name) => match caps.get(cap_name)? {
                Captured::Literal(l) => Some(l.clone()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn construct_unary_op(pat: &PatTree, caps: &Captures) -> Option<AstUnaryOperator> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::UnaryOp(op) => Some(op.clone()),
            _ => None,
        },
        PatTree::Node { name, children } if children.is_empty() => match name.as_str() {
            "Negate" => Some(AstUnaryOperator::Negate),
            "Not" => Some(AstUnaryOperator::Not),
            "BitNot" => Some(AstUnaryOperator::BitNot),
            "PreInc" => Some(AstUnaryOperator::PreInc),
            "PreDec" => Some(AstUnaryOperator::PreDec),
            "PostInc" => Some(AstUnaryOperator::PostInc),
            "PostDec" => Some(AstUnaryOperator::PostDec),
            "CastSigned" => Some(AstUnaryOperator::CastSigned),
            "CastUnsigned" => Some(AstUnaryOperator::CastUnsigned),
            _ => None,
        },
        _ => None,
    }
}

fn construct_binary_op(pat: &PatTree, caps: &Captures) -> Option<AstBinaryOperator> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::BinaryOp(op) => Some(op.clone()),
            _ => None,
        },
        PatTree::Node { name, children } if children.is_empty() => match name.as_str() {
            "Add" => Some(AstBinaryOperator::Add),
            "Sub" => Some(AstBinaryOperator::Sub),
            "Mul" => Some(AstBinaryOperator::Mul),
            "Div" => Some(AstBinaryOperator::Div),
            "Mod" => Some(AstBinaryOperator::Mod),
            "BitAnd" => Some(AstBinaryOperator::BitAnd),
            "BitOr" => Some(AstBinaryOperator::BitOr),
            "BitXor" => Some(AstBinaryOperator::BitXor),
            "LogicAnd" => Some(AstBinaryOperator::LogicAnd),
            "LogicOr" => Some(AstBinaryOperator::LogicOr),
            "Equal" => Some(AstBinaryOperator::Equal),
            "NotEqual" => Some(AstBinaryOperator::NotEqual),
            "Less" => Some(AstBinaryOperator::Less),
            "LessEqual" => Some(AstBinaryOperator::LessEqual),
            "Greater" => Some(AstBinaryOperator::Greater),
            "GreaterEqual" => Some(AstBinaryOperator::GreaterEqual),
            "LeftShift" => Some(AstBinaryOperator::LeftShift),
            "RightShift" => Some(AstBinaryOperator::RightShift),
            _ => None,
        },
        _ => None,
    }
}

fn construct_stmt_list(pat: &PatTree, caps: &Captures) -> Option<Vec<WrappedAstStatement>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::StmtList(l) => Some(l.clone()),
            _ => None,
        },
        PatTree::List(pats) => {
            let mut result = Vec::new();
            for p in pats {
                let stmt = construct_statement(p, caps)?;
                result.push(WrappedAstStatement {
                    statement: stmt,
                    origin: AstStatementOrigin::Unknown,
                    comment: None,
                });
            }
            Some(result)
        }
        _ => None,
    }
}

fn construct_opt_stmt_list(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Option<Vec<WrappedAstStatement>>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::OptStmtList(l) => Some(l.clone()),
            _ => None,
        },
        PatTree::OptionNone => Some(None),
        PatTree::OptionSome(inner) => {
            let list = construct_stmt_list(inner, caps)?;
            Some(Some(list))
        }
        _ => None,
    }
}

fn construct_opt_wrapped_expr(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Option<Wrapped<AstExpression>>> {
    match pat {
        PatTree::Capture(name) => match caps.get(name)? {
            Captured::OptExpression(e) => Some(e.clone()),
            _ => None,
        },
        PatTree::OptionNone => Some(None),
        PatTree::OptionSome(inner) => {
            let expr = construct_wrapped_expr(inner, caps)?;
            Some(Some(expr))
        }
        _ => None,
    }
}

/// Inject captured values into a rhai `Scope` as debug-string variables.
/// Each capture `$name` becomes a rhai variable `name` with its `Debug` representation.
pub fn inject_captures_into_rhai_scope(caps: &Captures, scope: &mut rhai::Scope<'static>) {
    for (name, captured) in caps {
        let value = match captured {
            Captured::Statement(s) => format!("{s:?}"),
            Captured::Expression(e) => format!("{:?}", e.item),
            Captured::ExpressionBox(e) => format!("{:?}", e.item),
            Captured::VariableId(id) => format!("{id:?}"),
            Captured::VariableMap(_) => "VariableMap(...)".to_string(),
            Captured::Literal(l) => format!("{l:?}"),
            Captured::StmtList(l) => format!("{l:?}"),
            Captured::OptStmtList(l) => format!("{l:?}"),
            Captured::OptExpression(e) => format!("{e:?}"),
            Captured::UnaryOp(op) => format!("{op:?}"),
            Captured::BinaryOp(op) => format!("{op:?}"),
            Captured::Variable(v) => format!("{v:?}"),
            Captured::ValueType(t) => format!("{t:?}"),
            Captured::Call(c) => format!("{c:?}"),
        };
        scope.push(name.clone(), value);
    }
}

fn find_any_origin(caps: &Captures) -> AstValueOrigin {
    for v in caps.values() {
        match v {
            Captured::Expression(e) => return e.origin.clone(),
            Captured::ExpressionBox(e) => return e.origin.clone(),
            _ => {}
        }
    }
    AstValueOrigin::Unknown
}

// ---------------------------------------------------------------------------
// Construct a Vec<WrappedAstStatement> from a capture (for emit_after)
// ---------------------------------------------------------------------------

/// Construct a list of statements from a pattern and captures.
/// Used by `emit_after` to produce the statements to splice after the matched one.
pub fn construct_emit_after_list(
    pat: &PatTree,
    caps: &Captures,
) -> Option<Vec<WrappedAstStatement>> {
    construct_stmt_list(pat, caps)
}

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

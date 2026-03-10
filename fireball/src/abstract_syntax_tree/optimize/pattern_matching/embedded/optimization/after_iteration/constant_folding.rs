//! Constant folding helper functions extracted from the original constant_folding.rs.
//!
//! Contains fold_identity, fold_reassociate, double-unary cancellation,
//! eval_unary, eval_binary (and all eval_binary_* helpers), wrap_with_source,
//! and rewrap_child.

// For fold_identity, use opt_utils versions:
use crate::abstract_syntax_tree::{
    AstBinaryOperator, AstExpression, AstLiteral, AstUnaryOperator, Wrapped,
    optimize::opt_utils::{eval_binary, expr_structurally_equal, is_pure_expression},
};

/// Identity element and absorbing element simplifications for binary operations.
///
/// Identity rules: x op identity = x (e.g., x + 0 = x, x * 1 = x)
/// Absorbing rules: x op absorber = absorber (e.g., x * 0 = 0, if x is pure)
/// Same-operand rules: x op x simplifications (e.g., x & x = x, x - x = 0)
pub(crate) fn fold_identity(
    source: &Wrapped<AstExpression>,
    operator: &AstBinaryOperator,
    left: &Wrapped<AstExpression>,
    right: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    // Identity element rules: x op identity = x
    match (operator, &left.item, &right.item) {
        (AstBinaryOperator::Add, _, AstExpression::Literal(AstLiteral::Int(0)))
        | (AstBinaryOperator::Add, _, AstExpression::Literal(AstLiteral::UInt(0)))
        | (AstBinaryOperator::Sub, _, AstExpression::Literal(AstLiteral::Int(0)))
        | (AstBinaryOperator::Sub, _, AstExpression::Literal(AstLiteral::UInt(0)))
        | (AstBinaryOperator::Mul, _, AstExpression::Literal(AstLiteral::Int(1)))
        | (AstBinaryOperator::Mul, _, AstExpression::Literal(AstLiteral::UInt(1)))
        | (AstBinaryOperator::Div, _, AstExpression::Literal(AstLiteral::Int(1)))
        | (AstBinaryOperator::Div, _, AstExpression::Literal(AstLiteral::UInt(1)))
        | (AstBinaryOperator::LogicAnd, _, AstExpression::Literal(AstLiteral::Bool(true)))
        | (AstBinaryOperator::LogicOr, _, AstExpression::Literal(AstLiteral::Bool(false))) => {
            return Some(rewrap_child(source, left));
        }
        (AstBinaryOperator::Add, AstExpression::Literal(AstLiteral::Int(0)), _)
        | (AstBinaryOperator::Add, AstExpression::Literal(AstLiteral::UInt(0)), _)
        | (AstBinaryOperator::Mul, AstExpression::Literal(AstLiteral::Int(1)), _)
        | (AstBinaryOperator::Mul, AstExpression::Literal(AstLiteral::UInt(1)), _)
        | (AstBinaryOperator::LogicAnd, AstExpression::Literal(AstLiteral::Bool(true)), _)
        | (AstBinaryOperator::LogicOr, AstExpression::Literal(AstLiteral::Bool(false)), _) => {
            return Some(rewrap_child(source, right));
        }
        _ => {}
    }

    // Absorbing element rules: x op absorber = absorber (purity guard on discarded operand)
    match (operator, &left.item, &right.item) {
        // x * 0 = 0 (if x is pure)
        (AstBinaryOperator::Mul, _, AstExpression::Literal(AstLiteral::Int(0)))
        | (AstBinaryOperator::Mul, _, AstExpression::Literal(AstLiteral::UInt(0)))
        | (AstBinaryOperator::BitAnd, _, AstExpression::Literal(AstLiteral::Int(0)))
        | (AstBinaryOperator::BitAnd, _, AstExpression::Literal(AstLiteral::UInt(0))) => {
            if is_pure_expression(&left.item) {
                return Some(rewrap_child(source, right));
            }
        }
        // 0 * x = 0 (if x is pure)
        (AstBinaryOperator::Mul, AstExpression::Literal(AstLiteral::Int(0)), _)
        | (AstBinaryOperator::Mul, AstExpression::Literal(AstLiteral::UInt(0)), _)
        | (AstBinaryOperator::BitAnd, AstExpression::Literal(AstLiteral::Int(0)), _)
        | (AstBinaryOperator::BitAnd, AstExpression::Literal(AstLiteral::UInt(0)), _) => {
            if is_pure_expression(&right.item) {
                return Some(rewrap_child(source, left));
            }
        }
        // false && x = false (if x is pure)
        (AstBinaryOperator::LogicAnd, AstExpression::Literal(AstLiteral::Bool(false)), _) => {
            if is_pure_expression(&right.item) {
                return Some(rewrap_child(source, left));
            }
        }
        // true || x = true (if x is pure)
        (AstBinaryOperator::LogicOr, AstExpression::Literal(AstLiteral::Bool(true)), _) => {
            if is_pure_expression(&right.item) {
                return Some(rewrap_child(source, left));
            }
        }
        _ => {}
    }

    // Same-operand simplifications (both operands must be pure to avoid folding f()-f())
    if expr_structurally_equal(&left.item, &right.item)
        && is_pure_expression(&left.item)
        && is_pure_expression(&right.item)
    {
        match operator {
            // x & x = x, x | x = x
            AstBinaryOperator::BitAnd | AstBinaryOperator::BitOr => {
                return Some(rewrap_child(source, left));
            }
            // x ^ x = 0, x - x = 0
            AstBinaryOperator::BitXor | AstBinaryOperator::Sub => {
                return Some(wrap_with_source(
                    source,
                    AstExpression::Literal(AstLiteral::Int(0)),
                ));
            }
            // x == x, x <= x, x >= x -> true
            AstBinaryOperator::Equal
            | AstBinaryOperator::LessEqual
            | AstBinaryOperator::GreaterEqual => {
                return Some(wrap_with_source(
                    source,
                    AstExpression::Literal(AstLiteral::Bool(true)),
                ));
            }
            // x != x, x < x, x > x -> false
            AstBinaryOperator::NotEqual | AstBinaryOperator::Less | AstBinaryOperator::Greater => {
                return Some(wrap_with_source(
                    source,
                    AstExpression::Literal(AstLiteral::Bool(false)),
                ));
            }
            _ => {}
        }
    }

    None
}

/// Commutative constant reassociation:
///   (x op c1) op c2  ->  x op (c1 op c2)
///   c2 op (c1 op x)  ->  (c2 op c1) op x
/// Only for commutative+associative ops: Add, Mul, BitAnd, BitOr, BitXor.
pub(crate) fn fold_reassociate(
    source: &Wrapped<AstExpression>,
    operator: &AstBinaryOperator,
    left: &Wrapped<AstExpression>,
    right: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    let is_reassociable = matches!(
        operator,
        AstBinaryOperator::Add
            | AstBinaryOperator::Mul
            | AstBinaryOperator::BitAnd
            | AstBinaryOperator::BitOr
            | AstBinaryOperator::BitXor
    );
    if !is_reassociable {
        return None;
    }

    // Form: (non_lit op c1) op c2
    if let AstExpression::Literal(c2) = &right.item {
        if let AstExpression::BinaryOp(inner_op, inner_left, inner_right) = &left.item {
            if std::mem::discriminant(operator) == std::mem::discriminant(inner_op) {
                if let AstExpression::Literal(c1) = &inner_right.item {
                    if let Some(folded) = eval_binary(operator, c1, c2) {
                        return Some(wrap_with_source(
                            source,
                            AstExpression::BinaryOp(
                                operator.clone(),
                                inner_left.clone(),
                                Box::new(wrap_with_source(source, AstExpression::Literal(folded))),
                            ),
                        ));
                    }
                }
            }
        }
    }

    // Mirrored form: c2 op (c1 op non_lit)
    if let AstExpression::Literal(c2) = &left.item {
        if let AstExpression::BinaryOp(inner_op, inner_left, inner_right) = &right.item {
            if std::mem::discriminant(operator) == std::mem::discriminant(inner_op) {
                if let AstExpression::Literal(c1) = &inner_left.item {
                    if let Some(folded) = eval_binary(operator, c2, c1) {
                        return Some(wrap_with_source(
                            source,
                            AstExpression::BinaryOp(
                                operator.clone(),
                                Box::new(wrap_with_source(source, AstExpression::Literal(folded))),
                                inner_right.clone(),
                            ),
                        ));
                    }
                }
            }
        }
    }

    None
}

/// Double-unary cancellation: ~~x -> x, --x -> x, !!x -> x
///
/// Extracted from the original fold_current function's unary handling.
pub(crate) fn double_unary_cancellation(
    expression: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    if let AstExpression::UnaryOp(operator, arg) = &expression.item {
        if let AstExpression::UnaryOp(inner_op, inner_arg) = &arg.item {
            let cancels = matches!(
                (operator, inner_op),
                (AstUnaryOperator::BitNot, AstUnaryOperator::BitNot)
                    | (AstUnaryOperator::Negate, AstUnaryOperator::Negate)
                    | (AstUnaryOperator::Not, AstUnaryOperator::Not)
            );
            if cancels {
                return Some(wrap_with_source(expression, inner_arg.item.clone()));
            }
        }
    }
    None
}

/// Evaluate a unary operator on a literal value.
pub(crate) fn eval_unary(operator: &AstUnaryOperator, value: &AstLiteral) -> Option<AstLiteral> {
    match (operator, value) {
        (AstUnaryOperator::Negate, AstLiteral::Int(v)) => v.checked_neg().map(AstLiteral::Int),
        (AstUnaryOperator::Not, AstLiteral::Bool(v)) => Some(AstLiteral::Bool(!v)),
        (AstUnaryOperator::BitNot, AstLiteral::Int(v)) => Some(AstLiteral::Int(!v)),
        (AstUnaryOperator::BitNot, AstLiteral::UInt(v)) => Some(AstLiteral::UInt(!v)),
        (AstUnaryOperator::CastSigned, AstLiteral::Int(v)) => Some(AstLiteral::Int(*v)),
        (AstUnaryOperator::CastSigned, AstLiteral::UInt(v)) => {
            i64::try_from(*v).ok().map(AstLiteral::Int)
        }
        (AstUnaryOperator::CastSigned, AstLiteral::Bool(v)) => {
            Some(AstLiteral::Int(if *v { 1 } else { 0 }))
        }
        (AstUnaryOperator::CastUnsigned, AstLiteral::UInt(v)) => Some(AstLiteral::UInt(*v)),
        (AstUnaryOperator::CastUnsigned, AstLiteral::Int(v)) => {
            u64::try_from(*v).ok().map(AstLiteral::UInt)
        }
        (AstUnaryOperator::CastUnsigned, AstLiteral::Bool(v)) => {
            Some(AstLiteral::UInt(if *v { 1 } else { 0 }))
        }
        _ => None,
    }
}

/// Evaluate a binary operator on two literal values.
///
/// This is the ORIGINAL version from constant_folding.rs, preserved here
/// as reference. The canonical version now lives in opt_utils.
pub(crate) fn eval_binary_orig(
    operator: &AstBinaryOperator,
    left: &AstLiteral,
    right: &AstLiteral,
) -> Option<AstLiteral> {
    match (left, right) {
        (AstLiteral::Int(a), AstLiteral::Int(b)) => eval_binary_i64(operator, *a, *b),
        (AstLiteral::UInt(a), AstLiteral::UInt(b)) => eval_binary_u64(operator, *a, *b),
        (AstLiteral::Float(a), AstLiteral::Float(b)) => eval_binary_f64(operator, *a, *b),
        (AstLiteral::Bool(a), AstLiteral::Bool(b)) => eval_binary_bool(operator, *a, *b),
        (AstLiteral::Char(a), AstLiteral::Char(b)) => eval_binary_char(operator, *a, *b),
        (AstLiteral::String(a), AstLiteral::String(b)) => eval_binary_str(operator, a, b),
        _ => None,
    }
}

fn eval_binary_i64(operator: &AstBinaryOperator, a: i64, b: i64) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Add => a.checked_add(b).map(AstLiteral::Int),
        AstBinaryOperator::Sub => a.checked_sub(b).map(AstLiteral::Int),
        AstBinaryOperator::Mul => a.checked_mul(b).map(AstLiteral::Int),
        AstBinaryOperator::Div => {
            if b == 0 {
                None
            } else {
                a.checked_div(b).map(AstLiteral::Int)
            }
        }
        AstBinaryOperator::Mod => {
            if b == 0 {
                None
            } else {
                a.checked_rem(b).map(AstLiteral::Int)
            }
        }
        AstBinaryOperator::BitAnd => Some(AstLiteral::Int(a & b)),
        AstBinaryOperator::BitOr => Some(AstLiteral::Int(a | b)),
        AstBinaryOperator::BitXor => Some(AstLiteral::Int(a ^ b)),
        AstBinaryOperator::LeftShift => {
            if b < 0 || b >= 64 {
                None
            } else {
                Some(AstLiteral::Int(a.wrapping_shl(b as u32)))
            }
        }
        AstBinaryOperator::RightShift => {
            if b < 0 || b >= 64 {
                None
            } else {
                Some(AstLiteral::Int(a.wrapping_shr(b as u32)))
            }
        }
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Less => Some(AstLiteral::Bool(a < b)),
        AstBinaryOperator::LessEqual => Some(AstLiteral::Bool(a <= b)),
        AstBinaryOperator::Greater => Some(AstLiteral::Bool(a > b)),
        AstBinaryOperator::GreaterEqual => Some(AstLiteral::Bool(a >= b)),
        AstBinaryOperator::LogicAnd | AstBinaryOperator::LogicOr => None,
    }
}

fn eval_binary_u64(operator: &AstBinaryOperator, a: u64, b: u64) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Add => a.checked_add(b).map(AstLiteral::UInt),
        AstBinaryOperator::Sub => a.checked_sub(b).map(AstLiteral::UInt),
        AstBinaryOperator::Mul => a.checked_mul(b).map(AstLiteral::UInt),
        AstBinaryOperator::Div => {
            if b == 0 {
                None
            } else {
                a.checked_div(b).map(AstLiteral::UInt)
            }
        }
        AstBinaryOperator::Mod => {
            if b == 0 {
                None
            } else {
                a.checked_rem(b).map(AstLiteral::UInt)
            }
        }
        AstBinaryOperator::BitAnd => Some(AstLiteral::UInt(a & b)),
        AstBinaryOperator::BitOr => Some(AstLiteral::UInt(a | b)),
        AstBinaryOperator::BitXor => Some(AstLiteral::UInt(a ^ b)),
        AstBinaryOperator::LeftShift => {
            if b >= 64 {
                None
            } else {
                Some(AstLiteral::UInt(a.wrapping_shl(b as u32)))
            }
        }
        AstBinaryOperator::RightShift => {
            if b >= 64 {
                None
            } else {
                Some(AstLiteral::UInt(a.wrapping_shr(b as u32)))
            }
        }
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Less => Some(AstLiteral::Bool(a < b)),
        AstBinaryOperator::LessEqual => Some(AstLiteral::Bool(a <= b)),
        AstBinaryOperator::Greater => Some(AstLiteral::Bool(a > b)),
        AstBinaryOperator::GreaterEqual => Some(AstLiteral::Bool(a >= b)),
        AstBinaryOperator::LogicAnd | AstBinaryOperator::LogicOr => None,
    }
}

fn eval_binary_f64(operator: &AstBinaryOperator, a: f64, b: f64) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Add => Some(AstLiteral::Float(a + b)),
        AstBinaryOperator::Sub => Some(AstLiteral::Float(a - b)),
        AstBinaryOperator::Mul => Some(AstLiteral::Float(a * b)),
        AstBinaryOperator::Div => Some(AstLiteral::Float(a / b)),
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Less => Some(AstLiteral::Bool(a < b)),
        AstBinaryOperator::LessEqual => Some(AstLiteral::Bool(a <= b)),
        AstBinaryOperator::Greater => Some(AstLiteral::Bool(a > b)),
        AstBinaryOperator::GreaterEqual => Some(AstLiteral::Bool(a >= b)),
        AstBinaryOperator::Mod
        | AstBinaryOperator::BitAnd
        | AstBinaryOperator::BitOr
        | AstBinaryOperator::BitXor
        | AstBinaryOperator::LogicAnd
        | AstBinaryOperator::LogicOr
        | AstBinaryOperator::LeftShift
        | AstBinaryOperator::RightShift => None,
    }
}

fn eval_binary_bool(operator: &AstBinaryOperator, a: bool, b: bool) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::LogicAnd => Some(AstLiteral::Bool(a && b)),
        AstBinaryOperator::LogicOr => Some(AstLiteral::Bool(a || b)),
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Less => Some(AstLiteral::Bool((a as u8) < (b as u8))),
        AstBinaryOperator::LessEqual => Some(AstLiteral::Bool((a as u8) <= (b as u8))),
        AstBinaryOperator::Greater => Some(AstLiteral::Bool((a as u8) > (b as u8))),
        AstBinaryOperator::GreaterEqual => Some(AstLiteral::Bool((a as u8) >= (b as u8))),
        AstBinaryOperator::Add
        | AstBinaryOperator::Sub
        | AstBinaryOperator::Mul
        | AstBinaryOperator::Div
        | AstBinaryOperator::Mod
        | AstBinaryOperator::BitAnd
        | AstBinaryOperator::BitOr
        | AstBinaryOperator::BitXor
        | AstBinaryOperator::LeftShift
        | AstBinaryOperator::RightShift => None,
    }
}

fn eval_binary_char(operator: &AstBinaryOperator, a: char, b: char) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Less => Some(AstLiteral::Bool(a < b)),
        AstBinaryOperator::LessEqual => Some(AstLiteral::Bool(a <= b)),
        AstBinaryOperator::Greater => Some(AstLiteral::Bool(a > b)),
        AstBinaryOperator::GreaterEqual => Some(AstLiteral::Bool(a >= b)),
        AstBinaryOperator::Add
        | AstBinaryOperator::Sub
        | AstBinaryOperator::Mul
        | AstBinaryOperator::Div
        | AstBinaryOperator::Mod
        | AstBinaryOperator::BitAnd
        | AstBinaryOperator::BitOr
        | AstBinaryOperator::BitXor
        | AstBinaryOperator::LogicAnd
        | AstBinaryOperator::LogicOr
        | AstBinaryOperator::LeftShift
        | AstBinaryOperator::RightShift => None,
    }
}

fn eval_binary_str(operator: &AstBinaryOperator, a: &str, b: &str) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Add => Some(AstLiteral::String(format!("{}{}", a, b))),
        AstBinaryOperator::Sub
        | AstBinaryOperator::Mul
        | AstBinaryOperator::Div
        | AstBinaryOperator::Mod
        | AstBinaryOperator::BitAnd
        | AstBinaryOperator::BitOr
        | AstBinaryOperator::BitXor
        | AstBinaryOperator::LogicAnd
        | AstBinaryOperator::LogicOr
        | AstBinaryOperator::Less
        | AstBinaryOperator::LessEqual
        | AstBinaryOperator::Greater
        | AstBinaryOperator::GreaterEqual
        | AstBinaryOperator::LeftShift
        | AstBinaryOperator::RightShift => None,
    }
}

/// Wrap a new expression item with the origin/comment from a source expression.
pub(crate) fn wrap_with_source(
    source: &Wrapped<AstExpression>,
    item: AstExpression,
) -> Wrapped<AstExpression> {
    Wrapped {
        item,
        origin: source.origin.clone(),
        comment: source.comment.clone(),
    }
}

/// Re-wrap a child expression's item with the parent source's origin/comment.
pub(crate) fn rewrap_child(
    source: &Wrapped<AstExpression>,
    child: &Wrapped<AstExpression>,
) -> Wrapped<AstExpression> {
    wrap_with_source(source, child.item.clone())
}

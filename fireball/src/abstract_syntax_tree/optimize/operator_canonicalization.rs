//! Canonicalize condition expressions and simplify zero comparisons.

use crate::abstract_syntax_tree::{
    AstBinaryOperator, AstExpression, AstLiteral, AstUnaryOperator, Wrapped,
};

/// Canonicalize a condition expression: normalize operators and simplify zero comparisons.
/// Used by control_flow_cleanup for condition normalization.
pub(super) fn canonicalize_condition_expression(expr: &mut Wrapped<AstExpression>) {
    canonicalize_expression(expr);
    simplify_condition_zero_cmp(expr);
}

fn canonicalize_expression(expr: &mut Wrapped<AstExpression>) {
    // Recurse into children first (bottom-up canonicalization).
    match &mut expr.item {
        AstExpression::UnaryOp(_, arg) => canonicalize_expression(arg),
        AstExpression::BinaryOp(_, left, right) => {
            canonicalize_expression(left);
            canonicalize_expression(right);
        }
        AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => canonicalize_expression(arg),
        AstExpression::ArrayAccess(base, idx) => {
            canonicalize_expression(base);
            canonicalize_expression(idx);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            canonicalize_expression(cond);
            canonicalize_expression(true_expr);
            canonicalize_expression(false_expr);
        }
        _ => {}
    }

    // Double negation elimination: Not(Not(x)) → x
    if let AstExpression::UnaryOp(AstUnaryOperator::Not, inner) = &expr.item {
        if let AstExpression::UnaryOp(AstUnaryOperator::Not, innermost) = &inner.item {
            expr.item = innermost.item.clone();
            return;
        }
    }

    // Comparison negation: Not(cmp(a,b)) → inv_cmp(a,b)
    if let AstExpression::UnaryOp(AstUnaryOperator::Not, inner) = &expr.item {
        if let AstExpression::BinaryOp(op, left, right) = &inner.item {
            let inverted = match op {
                AstBinaryOperator::Equal => Some(AstBinaryOperator::NotEqual),
                AstBinaryOperator::NotEqual => Some(AstBinaryOperator::Equal),
                AstBinaryOperator::Less => Some(AstBinaryOperator::GreaterEqual),
                AstBinaryOperator::LessEqual => Some(AstBinaryOperator::Greater),
                AstBinaryOperator::Greater => Some(AstBinaryOperator::LessEqual),
                AstBinaryOperator::GreaterEqual => Some(AstBinaryOperator::Less),
                _ => None,
            };
            if let Some(new_op) = inverted {
                expr.item = AstExpression::BinaryOp(new_op, left.clone(), right.clone());
                return;
            }
        }
    }

    // Commutative literal normalization and comparison flipping.
    if let AstExpression::BinaryOp(op, left, right) = &expr.item {
        if matches!(left.item, AstExpression::Literal(_))
            && !matches!(right.item, AstExpression::Literal(_))
        {
            if is_commutative(op) {
                expr.item = AstExpression::BinaryOp(op.clone(), right.clone(), left.clone());
            } else if let Some(flipped_op) = flip_comparison(op) {
                expr.item = AstExpression::BinaryOp(flipped_op, right.clone(), left.clone());
            }
        }
    }
}

/// Simplify comparison-with-zero in condition contexts:
/// `x != 0` → `x`, `x == 0` → `!x`
fn simplify_condition_zero_cmp(expr: &mut Wrapped<AstExpression>) {
    match &mut expr.item {
        AstExpression::UnaryOp(_, arg) => simplify_condition_zero_cmp(arg),
        AstExpression::BinaryOp(AstBinaryOperator::LogicAnd | AstBinaryOperator::LogicOr, l, r) => {
            simplify_condition_zero_cmp(l);
            simplify_condition_zero_cmp(r);
        }
        _ => {}
    }

    if let AstExpression::BinaryOp(op, left, right) = &expr.item {
        let is_zero = |e: &AstExpression| {
            matches!(
                e,
                AstExpression::Literal(AstLiteral::Int(0) | AstLiteral::UInt(0))
            )
        };
        match op {
            AstBinaryOperator::NotEqual if is_zero(&right.item) => {
                expr.item = left.item.clone();
            }
            AstBinaryOperator::Equal if is_zero(&right.item) => {
                expr.item = AstExpression::UnaryOp(AstUnaryOperator::Not, left.clone());
            }
            _ => {}
        }
    }
}

fn is_commutative(op: &AstBinaryOperator) -> bool {
    matches!(
        op,
        AstBinaryOperator::Add
            | AstBinaryOperator::Mul
            | AstBinaryOperator::BitAnd
            | AstBinaryOperator::BitOr
            | AstBinaryOperator::BitXor
            | AstBinaryOperator::Equal
            | AstBinaryOperator::NotEqual
    )
}

fn flip_comparison(op: &AstBinaryOperator) -> Option<AstBinaryOperator> {
    match op {
        AstBinaryOperator::Less => Some(AstBinaryOperator::Greater),
        AstBinaryOperator::LessEqual => Some(AstBinaryOperator::GreaterEqual),
        AstBinaryOperator::Greater => Some(AstBinaryOperator::Less),
        AstBinaryOperator::GreaterEqual => Some(AstBinaryOperator::LessEqual),
        _ => None,
    }
}

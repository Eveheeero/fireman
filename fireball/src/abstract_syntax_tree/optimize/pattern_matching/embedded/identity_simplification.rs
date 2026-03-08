//! Identity operation simplification extracted from bit_trick_recognition.rs.
//!
//! Simplifies identity operations inserted by obfuscators or unoptimized code:
//!   x ^ 0 -> x,  x | 0 -> x,  x + 0 -> x,  x - 0 -> x,
//!   x * 1 -> x,  x & all-ones -> x

use crate::abstract_syntax_tree::{AstBinaryOperator, AstExpression, AstLiteral, Wrapped};

/// Extract a u64 value from a literal expression (supports both Int and UInt).
pub(crate) fn extract_literal_u64(expr: &AstExpression) -> Option<u64> {
    match expr {
        AstExpression::Literal(AstLiteral::Int(v)) => u64::try_from(*v).ok(),
        AstExpression::Literal(AstLiteral::UInt(v)) => Some(*v),
        _ => None,
    }
}

/// Simplify identity operations: x ^ 0 -> x, x | 0 -> x, x + 0 -> x, x - 0 -> x,
/// x * 1 -> x, x & all-ones -> x.
///
/// Returns Some(simplified) if a simplification was applied, None otherwise.
pub(crate) fn try_simplify_identity_op(
    expr: &Wrapped<AstExpression>,
) -> Option<Wrapped<AstExpression>> {
    let AstExpression::BinaryOp(op, left, right) = &expr.item else {
        return None;
    };

    let left_lit = extract_literal_u64(&left.item);
    let right_lit = extract_literal_u64(&right.item);

    // Helper: build a replacement from a kept operand, preserving the outer origin/comment
    let keep = |operand: &Wrapped<AstExpression>| -> Wrapped<AstExpression> {
        Wrapped {
            item: operand.item.clone(),
            origin: expr.origin.clone(),
            comment: expr.comment.clone(),
        }
    };

    // x OP 0 -> x  for +, -, ^, |
    if right_lit == Some(0)
        && matches!(
            op,
            AstBinaryOperator::Add
                | AstBinaryOperator::Sub
                | AstBinaryOperator::BitXor
                | AstBinaryOperator::BitOr
        )
    {
        return Some(keep(left));
    }

    // 0 OP x -> x  for +, ^, |  (not sub)
    if left_lit == Some(0)
        && matches!(
            op,
            AstBinaryOperator::Add | AstBinaryOperator::BitXor | AstBinaryOperator::BitOr
        )
    {
        return Some(keep(right));
    }

    // x * 1 -> x, 1 * x -> x
    if matches!(op, AstBinaryOperator::Mul) {
        if right_lit == Some(1) {
            return Some(keep(left));
        }
        if left_lit == Some(1) {
            return Some(keep(right));
        }
    }

    // x & all-ones -> x (common mask widths in decompiled code)
    if matches!(op, AstBinaryOperator::BitAnd) {
        let all_ones: &[u64] = &[0xFF, 0xFFFF, 0xFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
        if let Some(r) = right_lit {
            if all_ones.contains(&r) {
                return Some(keep(left));
            }
        }
        if let Some(l) = left_lit {
            if all_ones.contains(&l) {
                return Some(keep(right));
            }
        }
    }

    None
}

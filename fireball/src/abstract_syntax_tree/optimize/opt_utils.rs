//! Shared utility functions for optimization passes (structural equality, purity, eval).

use crate::abstract_syntax_tree::{AstExpression, AstVariableId};

/// Recursive structural comparison of two expressions.
/// Variables are compared by their `AstVariableId`, not by the variable map Arc.
pub fn expr_structurally_equal(a: &AstExpression, b: &AstExpression) -> bool {
    match (a, b) {
        (AstExpression::Unknown, AstExpression::Unknown)
        | (AstExpression::Undefined, AstExpression::Undefined)
        | (AstExpression::ArchitectureBitSize, AstExpression::ArchitectureBitSize)
        | (AstExpression::ArchitectureByteSize, AstExpression::ArchitectureByteSize) => true,
        (AstExpression::Literal(a), AstExpression::Literal(b)) => literal_equal(a, b),
        (AstExpression::Variable(_, a_id), AstExpression::Variable(_, b_id)) => a_id == b_id,
        (AstExpression::UnaryOp(op_a, arg_a), AstExpression::UnaryOp(op_b, arg_b)) => {
            unary_op_equal(op_a, op_b) && expr_structurally_equal(&arg_a.item, &arg_b.item)
        }
        (AstExpression::BinaryOp(op_a, l_a, r_a), AstExpression::BinaryOp(op_b, l_b, r_b)) => {
            binary_op_equal(op_a, op_b)
                && expr_structurally_equal(&l_a.item, &l_b.item)
                && expr_structurally_equal(&r_a.item, &r_b.item)
        }
        (AstExpression::Cast(ty_a, arg_a), AstExpression::Cast(ty_b, arg_b)) => {
            ty_a == ty_b && expr_structurally_equal(&arg_a.item, &arg_b.item)
        }
        (AstExpression::Deref(a), AstExpression::Deref(b))
        | (AstExpression::AddressOf(a), AstExpression::AddressOf(b)) => {
            expr_structurally_equal(&a.item, &b.item)
        }
        (AstExpression::ArrayAccess(base_a, idx_a), AstExpression::ArrayAccess(base_b, idx_b)) => {
            expr_structurally_equal(&base_a.item, &base_b.item)
                && expr_structurally_equal(&idx_a.item, &idx_b.item)
        }
        (
            AstExpression::MemberAccess(expr_a, field_a),
            AstExpression::MemberAccess(expr_b, field_b),
        ) => field_a == field_b && expr_structurally_equal(&expr_a.item, &expr_b.item),
        (AstExpression::Ternary(cond_a, t_a, f_a), AstExpression::Ternary(cond_b, t_b, f_b)) => {
            expr_structurally_equal(&cond_a.item, &cond_b.item)
                && expr_structurally_equal(&t_a.item, &t_b.item)
                && expr_structurally_equal(&f_a.item, &f_b.item)
        }
        _ => false,
    }
}

/// Check if an expression is pure (no side effects, no aliasing concerns).
/// This is the same as `is_safe_to_inline` from expression_inlining.
pub fn is_pure_expression(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Literal(_)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => true,
        AstExpression::Variable(_, _) => true,
        AstExpression::UnaryOp(_, arg) => is_pure_expression(&arg.item),
        AstExpression::BinaryOp(_, left, right) => {
            is_pure_expression(&left.item) && is_pure_expression(&right.item)
        }
        AstExpression::Cast(_, arg) => is_pure_expression(&arg.item),
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            is_pure_expression(&cond.item)
                && is_pure_expression(&true_expr.item)
                && is_pure_expression(&false_expr.item)
        }
        AstExpression::Call(_)
        | AstExpression::Deref(_)
        | AstExpression::AddressOf(_)
        | AstExpression::ArrayAccess(_, _)
        | AstExpression::MemberAccess(_, _) => false,
    }
}

/// Check if an expression could write to memory or have externally visible effects.
///
/// This is less conservative than `is_pure_expression`: read-only memory operations
/// (Deref, AddressOf, ArrayAccess, MemberAccess) return `false` here because they
/// do not modify state. Only `Call` returns `true` since it may write to memory or
/// produce externally visible side effects.
///
/// Use this to decide whether an expression is safe to reorder past memory reads,
/// whereas `is_pure_expression` decides whether it is safe to reorder past writes.
pub fn has_write_side_effects(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Literal(_)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Variable(_, _) => false,
        AstExpression::UnaryOp(_, arg) => has_write_side_effects(&arg.item),
        AstExpression::BinaryOp(_, left, right) => {
            has_write_side_effects(&left.item) || has_write_side_effects(&right.item)
        }
        AstExpression::Cast(_, arg) => has_write_side_effects(&arg.item),
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            has_write_side_effects(&cond.item)
                || has_write_side_effects(&true_expr.item)
                || has_write_side_effects(&false_expr.item)
        }
        AstExpression::Deref(arg) | AstExpression::AddressOf(arg) => {
            has_write_side_effects(&arg.item)
        }
        AstExpression::ArrayAccess(base, idx) => {
            has_write_side_effects(&base.item) || has_write_side_effects(&idx.item)
        }
        AstExpression::MemberAccess(expr, _) => has_write_side_effects(&expr.item),
        AstExpression::Call(_) => true,
    }
}

/// Collect all variable IDs referenced (read) in an expression.
pub fn collect_expr_variables(expr: &AstExpression, out: &mut hashbrown::HashSet<AstVariableId>) {
    use crate::abstract_syntax_tree::{AstBuiltinFunctionArgument, AstCall};

    match expr {
        AstExpression::Variable(_, var_id) => {
            out.insert(*var_id);
        }
        AstExpression::UnaryOp(_, arg) => collect_expr_variables(&arg.item, out),
        AstExpression::BinaryOp(_, left, right) => {
            collect_expr_variables(&left.item, out);
            collect_expr_variables(&right.item, out);
        }
        AstExpression::Cast(_, arg) => collect_expr_variables(&arg.item, out),
        AstExpression::Call(call) => match call {
            AstCall::Variable { var_id, args, .. } => {
                out.insert(*var_id);
                for arg in args {
                    collect_expr_variables(&arg.item, out);
                }
            }
            AstCall::Function { args, .. } | AstCall::Unknown(_, args) => {
                for arg in args {
                    collect_expr_variables(&arg.item, out);
                }
            }
            AstCall::Builtin(_, args) => match args.as_ref() {
                AstBuiltinFunctionArgument::None => {}
                AstBuiltinFunctionArgument::Print(items) => {
                    for item in items {
                        collect_expr_variables(&item.item, out);
                    }
                }
                AstBuiltinFunctionArgument::ByteSizeOf(e)
                | AstBuiltinFunctionArgument::BitSizeOf(e)
                | AstBuiltinFunctionArgument::OperandExists(e)
                | AstBuiltinFunctionArgument::SignedMax(e)
                | AstBuiltinFunctionArgument::SignedMin(e)
                | AstBuiltinFunctionArgument::UnsignedMax(e)
                | AstBuiltinFunctionArgument::UnsignedMin(e)
                | AstBuiltinFunctionArgument::BitOnes(e)
                | AstBuiltinFunctionArgument::BitZeros(e) => {
                    collect_expr_variables(&e.item, out);
                }
                AstBuiltinFunctionArgument::Sized(e1, e2) => {
                    collect_expr_variables(&e1.item, out);
                    collect_expr_variables(&e2.item, out);
                }
            },
        },
        AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            collect_expr_variables(&arg.item, out);
        }
        AstExpression::ArrayAccess(base, idx) => {
            collect_expr_variables(&base.item, out);
            collect_expr_variables(&idx.item, out);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            collect_expr_variables(&cond.item, out);
            collect_expr_variables(&true_expr.item, out);
            collect_expr_variables(&false_expr.item, out);
        }
        AstExpression::Literal(_)
        | AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize => {}
    }
}

use crate::abstract_syntax_tree::{AstBinaryOperator, AstLiteral, AstUnaryOperator};

fn literal_equal(a: &AstLiteral, b: &AstLiteral) -> bool {
    match (a, b) {
        (AstLiteral::Int(a), AstLiteral::Int(b)) => a == b,
        (AstLiteral::UInt(a), AstLiteral::UInt(b)) => a == b,
        (AstLiteral::Float(a), AstLiteral::Float(b)) => a.to_bits() == b.to_bits(),
        (AstLiteral::String(a), AstLiteral::String(b)) => a == b,
        (AstLiteral::Char(a), AstLiteral::Char(b)) => a == b,
        (AstLiteral::Bool(a), AstLiteral::Bool(b)) => a == b,
        _ => false,
    }
}

fn unary_op_equal(a: &AstUnaryOperator, b: &AstUnaryOperator) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn binary_op_equal(a: &AstBinaryOperator, b: &AstBinaryOperator) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

// ---------------------------------------------------------------------------
// Literal evaluation (moved from constant_folding.rs for reuse by .fb patterns)
// ---------------------------------------------------------------------------

pub fn eval_unary(operator: &AstUnaryOperator, value: &AstLiteral) -> Option<AstLiteral> {
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

pub fn eval_binary(
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
        _ => None,
    }
}

fn eval_binary_bool(operator: &AstBinaryOperator, a: bool, b: bool) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::LogicAnd => Some(AstLiteral::Bool(a && b)),
        AstBinaryOperator::LogicOr => Some(AstLiteral::Bool(a || b)),
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        _ => None,
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
        _ => None,
    }
}

fn eval_binary_str(operator: &AstBinaryOperator, a: &str, b: &str) -> Option<AstLiteral> {
    match operator {
        AstBinaryOperator::Equal => Some(AstLiteral::Bool(a == b)),
        AstBinaryOperator::NotEqual => Some(AstLiteral::Bool(a != b)),
        AstBinaryOperator::Add => Some(AstLiteral::String(format!("{a}{b}"))),
        _ => None,
    }
}

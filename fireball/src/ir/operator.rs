use crate::{
    ir::data::{AccessSize, IrData, IrDataContainable},
    utils::Aos,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Not,
    Negation,
    SignExtend,
    ZeroExtend,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Sar,
    Add,
    Sub,
    Mul,
    SignedDiv,
    SignedRem,
    UnsignedDiv,
    UnsignedRem,
    Equal(AccessSize),
    /// <
    SignedLess(AccessSize),
    /// <=
    SignedLessOrEqual(AccessSize),
    /// <
    UnsignedLess(AccessSize),
    /// <=
    UnsignedLessOrEqual(AccessSize),
}

/// Unified operator enum for atomic operations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    // Basic operations
    Add,
    Sub,
    And,
    Or,
    Xor,
    Mov,
    // Min/Max operations
    Max,
    Min,
    Umax,
    Umin,
    // Other operations
    Not,
    Neg,
}

impl IrDataContainable for UnaryOperator {
    fn get_related_ir_data<'d>(&'d self, _v: &mut Vec<&'d Aos<IrData>>) {}
}
impl IrDataContainable for BinaryOperator {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        match self {
            BinaryOperator::Equal(access_size)
            | BinaryOperator::SignedLess(access_size)
            | BinaryOperator::SignedLessOrEqual(access_size)
            | BinaryOperator::UnsignedLess(access_size)
            | BinaryOperator::UnsignedLessOrEqual(access_size) => {
                access_size.get_related_ir_data(v)
            }
            _ => {}
        }
    }
}
impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "!",),
            UnaryOperator::Negation => write!(f, "-",),
            UnaryOperator::SignExtend => write!(f, "sign_extend",),
            UnaryOperator::ZeroExtend => write!(f, "zero_extend",),
        }
    }
}
impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::And => write!(f, "&"),
            BinaryOperator::Or => write!(f, "|"),
            BinaryOperator::Xor => write!(f, "^"),
            BinaryOperator::Shl => write!(f, "<<"),
            BinaryOperator::Shr => write!(f, ">>"),
            BinaryOperator::Sar => write!(f, "sar"),
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::SignedDiv => write!(f, "/"),
            BinaryOperator::SignedRem => write!(f, "%"),
            BinaryOperator::UnsignedDiv => write!(f, "div"),
            BinaryOperator::UnsignedRem => write!(f, "rem"),
            BinaryOperator::Equal(access_size) => write!(f, "== ({})", access_size),
            BinaryOperator::SignedLess(access_size) => write!(f, "< ({})", access_size),
            BinaryOperator::SignedLessOrEqual(access_size) => write!(f, "<= ({})", access_size),
            BinaryOperator::UnsignedLess(access_size) => write!(f, "< ({})", access_size),
            BinaryOperator::UnsignedLessOrEqual(access_size) => write!(f, "<= ({})", access_size),
        }
    }
}

use crate::{
    ir::data::{IrAccessSize, IrData, IrDataContainable},
    utils::Aos,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrUnaryOperator {
    Not,
    Negation,
    SignExtend,
    ZeroExtend,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrBinaryOperator {
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
    Equal(IrAccessSize),
    /// <
    SignedLess(IrAccessSize),
    /// <=
    SignedLessOrEqual(IrAccessSize),
    /// <
    UnsignedLess(IrAccessSize),
    /// <=
    UnsignedLessOrEqual(IrAccessSize),
}

impl IrDataContainable for IrUnaryOperator {
    fn get_related_ir_data<'d>(&'d self, _v: &mut Vec<&'d Aos<IrData>>) {}
}
impl IrDataContainable for IrBinaryOperator {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        match self {
            IrBinaryOperator::Equal(access_size)
            | IrBinaryOperator::SignedLess(access_size)
            | IrBinaryOperator::SignedLessOrEqual(access_size)
            | IrBinaryOperator::UnsignedLess(access_size)
            | IrBinaryOperator::UnsignedLessOrEqual(access_size) => {
                access_size.get_related_ir_data(v)
            }
            _ => {}
        }
    }
}
impl std::fmt::Display for IrUnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrUnaryOperator::Not => write!(f, "!",),
            IrUnaryOperator::Negation => write!(f, "-",),
            IrUnaryOperator::SignExtend => write!(f, "sign_extend",),
            IrUnaryOperator::ZeroExtend => write!(f, "zero_extend",),
        }
    }
}
impl std::fmt::Display for IrBinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrBinaryOperator::And => write!(f, "&"),
            IrBinaryOperator::Or => write!(f, "|"),
            IrBinaryOperator::Xor => write!(f, "^"),
            IrBinaryOperator::Shl => write!(f, "<<"),
            IrBinaryOperator::Shr => write!(f, ">>"),
            IrBinaryOperator::Sar => write!(f, "sar"),
            IrBinaryOperator::Add => write!(f, "+"),
            IrBinaryOperator::Sub => write!(f, "-"),
            IrBinaryOperator::Mul => write!(f, "*"),
            IrBinaryOperator::SignedDiv => write!(f, "/"),
            IrBinaryOperator::SignedRem => write!(f, "%"),
            IrBinaryOperator::UnsignedDiv => write!(f, "div"),
            IrBinaryOperator::UnsignedRem => write!(f, "rem"),
            IrBinaryOperator::Equal(access_size) => write!(f, "== ({})", access_size),
            IrBinaryOperator::SignedLess(access_size) => write!(f, "< ({})", access_size),
            IrBinaryOperator::SignedLessOrEqual(access_size) => write!(f, "<= ({})", access_size),
            IrBinaryOperator::UnsignedLess(access_size) => write!(f, "< ({})", access_size),
            IrBinaryOperator::UnsignedLessOrEqual(access_size) => write!(f, "<= ({})", access_size),
        }
    }
}

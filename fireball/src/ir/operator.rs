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

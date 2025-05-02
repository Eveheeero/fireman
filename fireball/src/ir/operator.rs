use crate::ir::data::AccessSize;

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

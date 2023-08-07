#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    Negation,
    SignExtend,
    ZeroExtend,
    Truncate,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Equal,
    SignedLess,
    SignedLessOrEqual,
    UnsignedLess,
    UnsignedLessOrEqual,
}

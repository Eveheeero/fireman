use bitvec::vec::BitVec;

#[derive(Debug, Clone)]
pub struct Constant(pub BitVec<u8>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intrinsic {
    Unknown,
    Undefined,
    ZeroStackOffset,
    ReturnAddress,
}
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

pub mod traits {
    use bitvec::vec::BitVec;

    pub trait Constant {
        fn constant(&self) -> &BitVec<u8>;
    }
    impl Constant for super::Constant {
        fn constant(&self) -> &BitVec<u8> {
            &self.0
        }
    }

    pub trait Intrinsic {
        fn intrinsic(&self) -> super::Intrinsic;
    }
    impl Intrinsic for super::Intrinsic {
        fn intrinsic(&self) -> super::Intrinsic {
            *self
        }
    }
    pub trait UnaryOperator {
        fn unary_operator(&self) -> super::UnaryOperator;
    }
    impl UnaryOperator for super::UnaryOperator {
        fn unary_operator(&self) -> super::UnaryOperator {
            *self
        }
    }
    pub trait BinaryOperator {
        fn binary_operator(&self) -> super::BinaryOperator;
    }
    impl BinaryOperator for super::BinaryOperator {
        fn binary_operator(&self) -> super::BinaryOperator {
            *self
        }
    }
}

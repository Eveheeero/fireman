use bitvec::vec::BitVec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    pub term_type: TermType,
    pub access_type: AccessType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TermType {
    Constant(BitVec<u8>),
    Intrinsic,
    Memory,
    Dereference,
    UnaryOperator,
    BinaryOperator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessType {
    Read,
    Write,
}

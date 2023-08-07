use crate::ir::operator::{BinaryOperator, UnaryOperator};

/// IR 내부에 사용되는 데이터
///
/// ### Note
/// snowman's Term + classes based ExpressionBase class
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRData {
    /// mov eax, 0x1234의 0x1234
    Constant(usize),
    /// Special (undefined, data remained before, return address..)
    Intrinsic(IntrinsicType),
    // mov eax, ebx의 ebx
    Register(crate::ir::Register),
    /// mov eax, dword ptr [eax]의 dword ptr [eax]
    Dereference(Box<IRData>),
    /// Operator
    Operator(IRDataOperator),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntrinsicType {
    Unknown(Box<IRData>),
    Undefined(Box<IRData>),
    ReturnAddress(Box<IRData>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessType {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRDataOperator {
    Unary(UnaryOperator, Box<IRData>),
    Binary(BinaryOperator, Box<IRData>, Box<IRData>),
}

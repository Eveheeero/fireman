use crate::ir::operator::{BinaryOperator, UnaryOperator};
use std::num::NonZeroU16;

/// IR 내부에 사용되는 데이터
///
/// ### Note
/// snowman's Term + classes based ExpressionBase class
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntrinsicType {
    Unknown(Box<IRData>),
    Undefined(Box<IRData>),
    ReturnAddress(Box<IRData>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccessType {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IRDataOperator {
    Unary {
        operator: UnaryOperator,
        arg: Box<IRData>,
        /// arg size
        size: NonZeroU16,
    },
    Binary {
        operator: BinaryOperator,
        arg1: Box<IRData>,
        arg2: Box<IRData>,
        /// arg size
        size: NonZeroU16,
    },
}

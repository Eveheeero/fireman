use crate::ir::operator::{BinaryOperator, UnaryOperator};
use std::num::NonZeroU16;

/// IR 내부에 사용되는 데이터
///
/// ### Note
/// snowman's Term + classes based ExpressionBase class
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrData {
    /// mov eax, 0x1234의 0x1234
    Constant(usize),
    /// Special (undefined, data remained before, return address..)
    Intrinsic(IntrinsicType),
    // mov eax, ebx의 ebx
    Register(crate::ir::Register),
    /// mov eax, dword ptr [eax]의 dword ptr [eax]
    Dereference(Box<IrData>),
    /// Operator
    Operator(IrDataOperator),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntrinsicType {
    Unknown(Box<IrData>),
    Undefined(Box<IrData>),
    ReturnAddress(Box<IrData>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccessType {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrDataOperator {
    Unary {
        operator: UnaryOperator,
        arg: Box<IrData>,
        /// arg size
        size: NonZeroU16,
    },
    Binary {
        operator: BinaryOperator,
        arg1: Box<IrData>,
        arg2: Box<IrData>,
        /// arg size
        size: NonZeroU16,
    },
}

use crate::ir::operator::{BinaryOperator, UnaryOperator};
use std::num::{NonZeroU16, NonZeroU8};

/// IR 내부에 사용되는 데이터
///
/// ### Note
/// snowman's Term + classes based ExpressionBase class
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrData {
    /// mov eax, 0x1234의 0x1234
    Constant(usize),
    /// Special (undefined, data remained before..)
    Intrinsic(IntrinsicType),
    // mov eax, ebx의 ebx
    Register(crate::ir::Register),
    /// mov eax, dword ptr [eax]의 dword ptr [eax]
    Dereference(Box<IrData>),
    /// Operation
    Operation(IrDataOperation),
    /// Nth operand
    Operand(NonZeroU8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntrinsicType {
    Unknown,
    Undefined,
    SignedMax(AccessSize),
    SignedMin(AccessSize),
    UnsignedMax(AccessSize),
    UnsignedMin(AccessSize),
    BitOnes(AccessSize),
    BitZeros(AccessSize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataAccess {
    data: IrData,
    access_type: DataAccessType,
    size: AccessSize,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataAccessType {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrDataOperation {
    Unary {
        operator: UnaryOperator,
        arg: Box<IrData>,
    },
    Binary {
        operator: BinaryOperator,
        arg1: Box<IrData>,
        arg2: Box<IrData>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccessSize {
    Fixed { bit_len: NonZeroU16 },
    Relative { with: Box<IrData> },
    ArchitectureSize,
    Unlimited,
}

impl Into<Box<IrData>> for &IrData {
    fn into(self) -> Box<IrData> {
        Box::new(self.clone())
    }
}
impl Into<IrData> for &IrData {
    fn into(self) -> IrData {
        self.clone()
    }
}

impl Into<AccessSize> for &AccessSize {
    fn into(self) -> AccessSize {
        self.clone()
    }
}

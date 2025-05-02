use crate::ir::operator::{BinaryOperator, UnaryOperator};
use std::{
    num::{NonZeroU16, NonZeroU8},
    sync::Arc,
};

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
    Dereference(Arc<IrData>),
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
    ArchitectureByteSize,
    ArchitectureBitSize,
    ArchitectureBitPerByte,
    InstructionByteSize,
    ByteSizeOf(Arc<IrData>),
    BitSizeOf(Arc<IrData>),
    Sized(Arc<IrData>, Arc<IrData>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataAccess {
    data: Arc<IrData>,
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
        arg: Arc<IrData>,
    },
    Binary {
        operator: BinaryOperator,
        arg1: Arc<IrData>,
        arg2: Arc<IrData>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccessSize {
    Fixed { bit_len: NonZeroU16 },
    Relative { with: Arc<IrData> },
    ArchitectureSize,
    Unlimited,
}

impl Into<AccessSize> for &AccessSize {
    fn into(self) -> AccessSize {
        self.clone()
    }
}

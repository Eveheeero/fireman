use crate::{
    ir::operator::{BinaryOperator, UnaryOperator},
    utils::Aos,
};
use std::num::NonZeroU8;

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
    Dereference(Aos<IrData>),
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
    ByteSizeOf(Aos<IrData>),
    BitSizeOf(Aos<IrData>),
    Sized(Aos<IrData>, AccessSize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataAccess {
    data: Aos<IrData>,
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
        arg: Aos<IrData>,
    },
    Binary {
        operator: BinaryOperator,
        arg1: Aos<IrData>,
        arg2: Aos<IrData>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccessSize {
    ResultOfBit(Aos<IrData>),
    ResultOfByte(Aos<IrData>),
    RelativeWith(Aos<IrData>),
    ArchitectureSize,
    Unlimited,
}

impl Into<AccessSize> for &AccessSize {
    fn into(self) -> AccessSize {
        self.clone()
    }
}
impl Into<Aos<IrData>> for &crate::ir::Register {
    fn into(self) -> Aos<IrData> {
        IrData::Register(*self).into()
    }
}

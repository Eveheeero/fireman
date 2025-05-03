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
    Intrinsic(IrIntrinsic),
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
pub enum IrIntrinsic {
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
    OperandExists(NonZeroU8),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataAccess {
    location: Aos<IrData>,
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
pub trait IrDataContainable {
    /// Return Does not contain self
    fn get_related_ir_data(&self, v: &mut Vec<Aos<IrData>>);
}

impl From<&AccessSize> for AccessSize {
    fn from(value: &AccessSize) -> Self {
        value.clone()
    }
}
impl From<&crate::ir::Register> for Aos<IrData> {
    fn from(value: &crate::ir::Register) -> Self {
        IrData::Register(*value).into()
    }
}

impl DataAccess {
    pub fn new(location: Aos<IrData>, access_type: DataAccessType, size: AccessSize) -> Self {
        Self {
            location,
            access_type,
            size,
        }
    }
    pub fn location(&self) -> &Aos<IrData> {
        &self.location
    }
    pub fn access_type(&self) -> &DataAccessType {
        &self.access_type
    }
    pub fn size(&self) -> &AccessSize {
        &self.size
    }
}

impl IrDataContainable for IrData {
    fn get_related_ir_data(&self, v: &mut Vec<Aos<IrData>>) {
        match self {
            IrData::Intrinsic(intrinsic) => intrinsic.get_related_ir_data(v),
            IrData::Dereference(data) => {
                data.get_related_ir_data(v);
                v.push(data.clone());
            }
            IrData::Operation(operation) => match operation {
                IrDataOperation::Unary { operator, arg } => {
                    operator.get_related_ir_data(v);
                    arg.get_related_ir_data(v);
                    v.push(arg.clone());
                }
                IrDataOperation::Binary {
                    operator,
                    arg1,
                    arg2,
                } => {
                    operator.get_related_ir_data(v);
                    arg1.get_related_ir_data(v);
                    arg2.get_related_ir_data(v);
                    v.push(arg1.clone());
                    v.push(arg2.clone());
                }
            },
            _ => {}
        }
    }
}

impl IrDataContainable for DataAccess {
    fn get_related_ir_data(&self, v: &mut Vec<Aos<IrData>>) {
        self.location.get_related_ir_data(v);
        v.push(self.location.clone());
    }
}

impl IrDataContainable for AccessSize {
    fn get_related_ir_data(&self, v: &mut Vec<Aos<IrData>>) {
        match self {
            AccessSize::ResultOfBit(aos)
            | AccessSize::ResultOfByte(aos)
            | AccessSize::RelativeWith(aos) => {
                aos.get_related_ir_data(v);
                v.push(aos.clone());
            }
            _ => {}
        }
    }
}

impl IrDataContainable for IrIntrinsic {
    fn get_related_ir_data(&self, v: &mut Vec<Aos<IrData>>) {
        match self {
            IrIntrinsic::SignedMax(access_size)
            | IrIntrinsic::SignedMin(access_size)
            | IrIntrinsic::UnsignedMax(access_size)
            | IrIntrinsic::UnsignedMin(access_size)
            | IrIntrinsic::BitOnes(access_size)
            | IrIntrinsic::BitZeros(access_size) => access_size.get_related_ir_data(v),
            IrIntrinsic::ByteSizeOf(aos) | IrIntrinsic::BitSizeOf(aos) => {
                aos.get_related_ir_data(v);
                v.push(aos.clone());
            }
            IrIntrinsic::Sized(aos, access_size) => {
                aos.get_related_ir_data(v);
                v.push(aos.clone());
                access_size.get_related_ir_data(v);
            }
            _ => {}
        }
    }
}

use crate::{
    ir::operator::{BinaryOperator, UnaryOperator},
    utils::Aos,
};
use std::num::NonZeroU8;

/// Data used internally by the IR
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrData {
    /// The literal value (e.g., 0x1234) in `mov eax, 0x1234`
    Constant(usize),
    /// Special data (undefined, residual data)
    Intrinsic(IrIntrinsic),
    /// The register operand (e.g., ebx) in `mov eax, ebx`
    Register(crate::ir::Register),
    /// The memory operand (e.g., dword ptr [eax]) in `mov eax, dword ptr [eax]`
    Dereference(Aos<IrData>),
    /// An IR data operation
    Operation(IrDataOperation),
    /// Nth operand index
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
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
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>);
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
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        match self {
            IrData::Intrinsic(intrinsic) => intrinsic.get_related_ir_data(v),
            IrData::Dereference(data) => {
                data.get_related_ir_data(v);
                v.push(data);
            }
            IrData::Operation(operation) => match operation {
                IrDataOperation::Unary { operator, arg } => {
                    operator.get_related_ir_data(v);
                    arg.get_related_ir_data(v);
                    v.push(arg);
                }
                IrDataOperation::Binary {
                    operator,
                    arg1,
                    arg2,
                } => {
                    operator.get_related_ir_data(v);
                    arg1.get_related_ir_data(v);
                    arg2.get_related_ir_data(v);
                    v.push(arg1);
                    v.push(arg2);
                }
            },
            _ => {}
        }
    }
}

impl IrDataContainable for DataAccess {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        self.location.get_related_ir_data(v);
        v.push(&self.location);
    }
}

impl IrDataContainable for AccessSize {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        match self {
            AccessSize::ResultOfBit(aos)
            | AccessSize::ResultOfByte(aos)
            | AccessSize::RelativeWith(aos) => {
                aos.get_related_ir_data(v);
                v.push(aos);
            }
            _ => {}
        }
    }
}

impl IrDataContainable for IrIntrinsic {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        match self {
            IrIntrinsic::SignedMax(access_size)
            | IrIntrinsic::SignedMin(access_size)
            | IrIntrinsic::UnsignedMax(access_size)
            | IrIntrinsic::UnsignedMin(access_size)
            | IrIntrinsic::BitOnes(access_size)
            | IrIntrinsic::BitZeros(access_size) => access_size.get_related_ir_data(v),
            IrIntrinsic::ByteSizeOf(aos) | IrIntrinsic::BitSizeOf(aos) => {
                aos.get_related_ir_data(v);
                v.push(aos);
            }
            IrIntrinsic::Sized(aos, access_size) => {
                aos.get_related_ir_data(v);
                v.push(aos);
                access_size.get_related_ir_data(v);
            }
            _ => {}
        }
    }
}

impl From<&iceball::Argument> for Aos<IrData> {
    fn from(value: &iceball::Argument) -> Self {
        use iceball::{AddressingOperator, Argument, Memory, Register, RelativeAddressingArgument};
        match value {
            Argument::Constant(c) => IrData::Constant((*c).try_into().unwrap()).into(),
            Argument::Memory(Memory::AbsoluteAddressing(v)) => {
                IrData::Dereference(IrData::Constant((*v).try_into().unwrap()).into()).into()
            }
            Argument::Memory(Memory::RelativeAddressing(v)) => {
                let v = v.as_ref();
                let arg_count = v.len();
                assert_ne!(arg_count, 0);
                assert!(matches!(arg_count, 1 | 3));

                let mut iter = v.iter();
                let arg1 = iter.next().unwrap();
                let mut current_expr: Aos<IrData> = match arg1 {
                    RelativeAddressingArgument::Register(reg) => match reg {
                        Register::X64(x64_reg) => x64_reg_to_ir_reg(*x64_reg),
                    },
                    RelativeAddressingArgument::Constant(c) => {
                        if *c >= 0 {
                            IrData::Constant((*c).try_into().unwrap()).into()
                        } else {
                            IrData::Operation(IrDataOperation::Unary {
                                operator: UnaryOperator::Negation,
                                arg: IrData::Constant((0 - *c).try_into().unwrap()).into(),
                            })
                            .into()
                        }
                    }
                    RelativeAddressingArgument::Operator(_) => unreachable!(),
                };

                if let Some(operator) = iter.next() {
                    let operator = match operator {
                        RelativeAddressingArgument::Operator(op) => op,
                        _ => unreachable!(),
                    };
                    let operand = iter.next().unwrap();
                    let operand: Aos<IrData> = match operand {
                        RelativeAddressingArgument::Register(reg) => match reg {
                            Register::X64(x64_reg) => x64_reg_to_ir_reg(*x64_reg),
                        },
                        RelativeAddressingArgument::Constant(c) => {
                            if *c >= 0 {
                                IrData::Constant((*c).try_into().unwrap()).into()
                            } else {
                                IrData::Operation(IrDataOperation::Unary {
                                    operator: UnaryOperator::Negation,
                                    arg: IrData::Constant((0 - *c).try_into().unwrap()).into(),
                                })
                                .into()
                            }
                        }
                        RelativeAddressingArgument::Operator(_) => unreachable!(),
                    };

                    let binary_op_ir = match operator {
                        AddressingOperator::Add => BinaryOperator::Add,
                        AddressingOperator::Sub => BinaryOperator::Sub,
                        AddressingOperator::Mul => BinaryOperator::Mul,
                    };

                    current_expr = IrData::Operation(IrDataOperation::Binary {
                        operator: binary_op_ir,
                        arg1: current_expr,
                        arg2: operand,
                    })
                    .into();
                }

                IrData::Dereference(current_expr).into()
            }
            Argument::Register(Register::X64(register)) => x64_reg_to_ir_reg(*register),
        }
    }
}
fn x64_reg_to_ir_reg(reg: iceball::X64Register) -> Aos<IrData> {
    crate::arch::x86_64::str_to_x64_register(reg.name())
}

impl std::fmt::Display for IrData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrData::Constant(c) => write!(f, "{:#X}", c),
            IrData::Register(reg) => write!(f, "{}", reg),
            IrData::Dereference(data) => write!(f, "{}", data),
            IrData::Operation(operation) => write!(f, "{}", operation),
            IrData::Intrinsic(intrinsic) => write!(f, "{}", intrinsic),
            IrData::Operand(operand) => write!(f, "o{}", operand.get()),
        }
    }
}
impl std::fmt::Display for AccessSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessSize::ResultOfBit(aos) => write!(f, "({})bit", aos),
            AccessSize::ResultOfByte(aos) => write!(f, "({})byte", aos),
            AccessSize::RelativeWith(aos) => write!(f, "({})of", aos),
            AccessSize::ArchitectureSize => write!(f, "arch_len"),
            AccessSize::Unlimited => write!(f, "unlimited"),
        }
    }
}
impl std::fmt::Display for IrIntrinsic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrIntrinsic::Unknown => write!(f, "unknown"),
            IrIntrinsic::Undefined => write!(f, "undefined"),
            IrIntrinsic::SignedMax(access_size) => write!(f, "signed_max({})", access_size),
            IrIntrinsic::SignedMin(access_size) => write!(f, "signed_min({})", access_size),
            IrIntrinsic::UnsignedMax(access_size) => write!(f, "unsigned_max({})", access_size),
            IrIntrinsic::UnsignedMin(access_size) => write!(f, "unsigned_min({})", access_size),
            IrIntrinsic::BitOnes(access_size) => write!(f, "bit_ones({})", access_size),
            IrIntrinsic::BitZeros(access_size) => write!(f, "bit_zeros({})", access_size),
            IrIntrinsic::ArchitectureByteSize => write!(f, "arch_byte_size"),
            IrIntrinsic::ArchitectureBitSize => write!(f, "arch_bit_size"),
            IrIntrinsic::ArchitectureBitPerByte => write!(f, "arch_bit_per_byte"),
            IrIntrinsic::InstructionByteSize => write!(f, "instruction_byte_size"),
            IrIntrinsic::ByteSizeOf(aos) => write!(f, "byte_size_of({})", aos),
            IrIntrinsic::BitSizeOf(aos) => write!(f, "bit_size_of({})", aos),
            IrIntrinsic::Sized(aos, access_size) => write!(f, "sized({},{})", aos, access_size),
            IrIntrinsic::OperandExists(operand) => write!(f, "operand_exists({})", operand),
        }
    }
}
impl std::fmt::Display for IrDataOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrDataOperation::Unary { operator, arg } => write!(f, "{} {}", operator, arg),
            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => {
                write!(f, "{} {} {}", arg1, operator, arg2)
            }
        }
    }
}
impl std::fmt::Display for DataAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}) {}", self.access_type, self.location, self.size)
    }
}
impl std::fmt::Display for DataAccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataAccessType::Read => write!(f, "r"),
            DataAccessType::Write => write!(f, "w"),
        }
    }
}

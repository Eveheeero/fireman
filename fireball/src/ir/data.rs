use crate::{
    ir::operator::{IrBinaryOperator, IrUnaryOperator},
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
    SignedMax(IrAccessSize),
    SignedMin(IrAccessSize),
    UnsignedMax(IrAccessSize),
    UnsignedMin(IrAccessSize),
    BitOnes(IrAccessSize),
    BitZeros(IrAccessSize),
    ArchitectureByteSize,
    ArchitectureBitSize,
    ArchitectureBitPerByte,
    InstructionByteSize,
    ByteSizeOf(Aos<IrData>),
    BitSizeOf(Aos<IrData>),
    Sized(Aos<IrData>, IrAccessSize),
    OperandExists(NonZeroU8),
    ArchitectureByteSizeCondition(NumCondition),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum NumCondition {
    Higher(u16),
    HigherOrEqual(u16),
    Lower(u16),
    LowerOrEqual(u16),
    Equal(u16),
    NotEqual(u16),
    RangeInclusive(u16, u16),
    ExcludesRange(u16, u16),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IrDataAccess {
    location: Aos<IrData>,
    access_type: IrDataAccessType,
    size: IrAccessSize,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum IrDataAccessType {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrDataOperation {
    Unary {
        operator: IrUnaryOperator,
        arg: Aos<IrData>,
    },
    Binary {
        operator: IrBinaryOperator,
        arg1: Aos<IrData>,
        arg2: Aos<IrData>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrAccessSize {
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

impl From<&IrAccessSize> for IrAccessSize {
    fn from(value: &IrAccessSize) -> Self {
        value.clone()
    }
}
impl From<&crate::ir::Register> for Aos<IrData> {
    fn from(value: &crate::ir::Register) -> Self {
        IrData::Register(*value).into()
    }
}

impl IrDataAccess {
    pub fn new(location: Aos<IrData>, access_type: IrDataAccessType, size: IrAccessSize) -> Self {
        Self {
            location,
            access_type,
            size,
        }
    }
    pub fn location(&self) -> &Aos<IrData> {
        &self.location
    }
    pub fn access_type(&self) -> &IrDataAccessType {
        &self.access_type
    }
    pub fn size(&self) -> &IrAccessSize {
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

impl IrDataContainable for IrDataAccess {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        self.location.get_related_ir_data(v);
        v.push(&self.location);
    }
}

impl IrDataContainable for IrAccessSize {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        match self {
            IrAccessSize::ResultOfBit(aos)
            | IrAccessSize::ResultOfByte(aos)
            | IrAccessSize::RelativeWith(aos) => {
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
            IrIntrinsic::Unknown
            | IrIntrinsic::Undefined
            | IrIntrinsic::ArchitectureByteSize
            | IrIntrinsic::ArchitectureBitSize
            | IrIntrinsic::ArchitectureBitPerByte
            | IrIntrinsic::InstructionByteSize
            | IrIntrinsic::OperandExists(..)
            | IrIntrinsic::ArchitectureByteSizeCondition(..) => {}
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
                                operator: IrUnaryOperator::Negation,
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
                                    operator: IrUnaryOperator::Negation,
                                    arg: IrData::Constant((0 - *c).try_into().unwrap()).into(),
                                })
                                .into()
                            }
                        }
                        RelativeAddressingArgument::Operator(_) => unreachable!(),
                    };

                    let binary_op_ir = match operator {
                        AddressingOperator::Add => IrBinaryOperator::Add,
                        AddressingOperator::Sub => IrBinaryOperator::Sub,
                        AddressingOperator::Mul => IrBinaryOperator::Mul,
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
impl std::fmt::Display for IrAccessSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrAccessSize::ResultOfBit(aos) => write!(f, "{}bit", aos),
            IrAccessSize::ResultOfByte(aos) => write!(f, "{}byte", aos),
            IrAccessSize::RelativeWith(aos) => write!(f, "sizeof({})", aos),
            IrAccessSize::ArchitectureSize => write!(f, "arch_len"),
            IrAccessSize::Unlimited => write!(f, "unlimited"),
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
            IrIntrinsic::ArchitectureByteSizeCondition(num_condition) => {
                write!(f, "arch_byte_size_condition({})", num_condition)
            }
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
impl std::fmt::Display for IrDataAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}) {}", self.access_type, self.location, self.size)
    }
}
impl std::fmt::Display for IrDataAccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrDataAccessType::Read => write!(f, "r"),
            IrDataAccessType::Write => write!(f, "w"),
        }
    }
}

impl std::fmt::Display for NumCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumCondition::Higher(value) => write!(f, "{} > {}", self, value),
            NumCondition::HigherOrEqual(value) => write!(f, "{} >= {}", self, value),
            NumCondition::Lower(value) => write!(f, "{} < {}", self, value),
            NumCondition::LowerOrEqual(value) => write!(f, "{} <= {}", self, value),
            NumCondition::Equal(value) => write!(f, "{} == {}", self, value),
            NumCondition::NotEqual(value) => write!(f, "{} != {}", self, value),
            NumCondition::RangeInclusive(value1, value2) => {
                write!(f, "{} in [{}..{}]", self, value1, value2)
            }
            NumCondition::ExcludesRange(value1, value2) => {
                write!(f, "{} not in [{}..{}]", self, value1, value2)
            }
        }
    }
}

use crate::{
    ir::operator::{BinaryOperator, UnaryOperator},
    utils::Aos,
};
use std::num::NonZeroU8;

/// Data used internally by the IR
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
    ArchitectureByteSizeCondition(NumCondition),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DataAccess {
    location: Aos<IrData>,
    access_type: DataAccessType,
    size: AccessSize,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, PartialOrd, Ord)]
pub enum DataAccessType {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
        use iceball::{Argument, Register};
        match value {
            Argument::Constant(c) => IrData::Constant((*c).try_into().unwrap()).into(),
            Argument::Memory(mem) => {
                // Build memory access expression from the Memory struct
                let mut expr: Option<Aos<IrData>> = None;

                // Add base register
                if let Some(base_reg) = &mem.base {
                    let base_ir = match base_reg {
                        Register::X64(x64_reg) => x64_reg_to_ir_reg(*x64_reg),
                        Register::X86(x86_reg) => x86_reg_to_ir_reg(*x86_reg),
                        Register::Arm32(arm32_reg) => arm32_reg_to_ir_reg(*arm32_reg),
                        Register::Arm64(arm64_reg) => arm64_reg_to_ir_reg(*arm64_reg),
                    };
                    expr = Some(base_ir);
                }

                // Add scaled index register
                if let Some(index_reg) = &mem.index {
                    let index_ir = match index_reg {
                        Register::X64(x64_reg) => x64_reg_to_ir_reg(*x64_reg),
                        Register::X86(x86_reg) => x86_reg_to_ir_reg(*x86_reg),
                        Register::Arm32(arm32_reg) => arm32_reg_to_ir_reg(*arm32_reg),
                        Register::Arm64(arm64_reg) => arm64_reg_to_ir_reg(*arm64_reg),
                    };

                    let scaled_index = if mem.scale > 1 {
                        IrData::Operation(IrDataOperation::Binary {
                            operator: BinaryOperator::Mul,
                            arg1: index_ir,
                            arg2: IrData::Constant(mem.scale as usize).into(),
                        })
                        .into()
                    } else {
                        index_ir
                    };

                    expr = if let Some(base_expr) = expr {
                        Some(
                            IrData::Operation(IrDataOperation::Binary {
                                operator: BinaryOperator::Add,
                                arg1: base_expr,
                                arg2: scaled_index,
                            })
                            .into(),
                        )
                    } else {
                        Some(scaled_index)
                    };
                }

                // Add displacement
                if mem.displacement != 0 {
                    let disp_ir = if mem.displacement >= 0 {
                        IrData::Constant(mem.displacement as usize).into()
                    } else {
                        IrData::Operation(IrDataOperation::Unary {
                            operator: UnaryOperator::Negation,
                            arg: IrData::Constant((-mem.displacement) as usize).into(),
                        })
                        .into()
                    };

                    expr = if let Some(base_expr) = expr {
                        if mem.displacement >= 0 {
                            Some(
                                IrData::Operation(IrDataOperation::Binary {
                                    operator: BinaryOperator::Add,
                                    arg1: base_expr,
                                    arg2: disp_ir,
                                })
                                .into(),
                            )
                        } else {
                            Some(
                                IrData::Operation(IrDataOperation::Binary {
                                    operator: BinaryOperator::Sub,
                                    arg1: base_expr,
                                    arg2: disp_ir,
                                })
                                .into(),
                            )
                        }
                    } else {
                        Some(disp_ir)
                    };
                }

                // If no expression was built, this is an absolute address
                let final_expr = expr.unwrap_or_else(|| IrData::Constant(0).into());
                IrData::Dereference(final_expr).into()
            }
            Argument::Register(Register::X64(register)) => x64_reg_to_ir_reg(*register),
            Argument::Register(Register::X86(register)) => x86_reg_to_ir_reg(*register),
            Argument::Register(Register::Arm32(register)) => arm32_reg_to_ir_reg(*register),
            Argument::Register(Register::Arm64(register)) => arm64_reg_to_ir_reg(*register),
        }
    }
}
fn x64_reg_to_ir_reg(reg: iceball::X64Register) -> Aos<IrData> {
    crate::arch::x86_64::str_to_x64_register(reg.name())
}

fn x86_reg_to_ir_reg(reg: iceball::X86Register) -> Aos<IrData> {
    // X86 registers are a subset of X64 registers, so we can use the same function
    // Most x86 registers have direct x64 equivalents
    crate::arch::x86_64::str_to_x64_register(reg.to_string().as_str())
}

fn arm32_reg_to_ir_reg(reg: iceball::Arm32Register) -> Aos<IrData> {
    crate::arch::arm32::str_to_arm32_register(reg.to_string().as_str())
}

fn arm64_reg_to_ir_reg(reg: iceball::Arm64Register) -> Aos<IrData> {
    crate::arch::arm64::str_to_arm64_register(reg.to_string().as_str())
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

impl std::fmt::Display for NumCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumCondition::Higher(value) => write!(f, "> {}", value),
            NumCondition::HigherOrEqual(value) => write!(f, ">= {}", value),
            NumCondition::Lower(value) => write!(f, "< {}", value),
            NumCondition::LowerOrEqual(value) => write!(f, "<= {}", value),
            NumCondition::Equal(value) => write!(f, "== {}", value),
            NumCondition::NotEqual(value) => write!(f, "!= {}", value),
            NumCondition::RangeInclusive(value1, value2) => {
                write!(f, "in [{}..{}]", value1, value2)
            }
            NumCondition::ExcludesRange(value1, value2) => {
                write!(f, "not in [{}..{}]", value1, value2)
            }
        }
    }
}

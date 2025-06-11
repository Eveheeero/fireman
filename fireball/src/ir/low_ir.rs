//! Low-level IR (Direct Translation Layer)
//!
//! This module implements the lowest level of the multi-level IR system.
//! Low IR preserves all assembly semantics with 100% fidelity, including:
//! - All CPU flags and side effects
//! - Exact memory access patterns
//! - One-to-one mapping with assembly instructions

use crate::core::Address;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Display, Formatter};

pub mod ssa;
pub use ssa::SSABuilder;

/// Low IR types with explicit sizes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    /// Void type
    Void,

    /// Boolean (1 bit)
    Bool,

    /// Integers with explicit bit width
    I1,
    I8,
    I16,
    I32,
    I64,
    I128,

    /// Floating point
    F32,
    F64,
    F80,

    /// Pointer with optional pointee type
    Pointer(Option<Box<Type>>),

    /// Fixed-size array
    Array(Box<Type>, usize),

    /// Structure (ordered fields)
    Struct(Vec<Type>),

    /// Function type
    Function {
        ret: Box<Type>,
        params: Vec<Type>,
        varargs: bool,
    },

    /// Unknown type (for initial lifting)
    Unknown,
}

impl Type {
    /// Get size in bytes
    pub fn size(&self) -> Option<usize> {
        match self {
            Type::Void => Some(0),
            Type::Bool | Type::I1 => Some(1),
            Type::I8 => Some(1),
            Type::I16 => Some(2),
            Type::I32 | Type::F32 => Some(4),
            Type::I64 | Type::F64 | Type::Pointer(_) => Some(8),
            Type::I128 => Some(16),
            Type::F80 => Some(10),
            Type::Array(elem, count) => elem.size().map(|elem_size| elem_size * count),
            Type::Struct(fields) => {
                let mut size = 0;
                for field in fields {
                    if let Some(field_size) = field.size() {
                        size += field_size;
                    } else {
                        return None;
                    }
                }
                Some(size)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Constant {
    /// Integer constant with type
    Int { value: i128, ty: Type },

    /// Floating point constant
    Float { bits: u64, ty: Type },

    /// Null pointer
    Null(Type),

    /// Undefined value
    Undef(Type),

    /// Aggregate constant
    Aggregate(Vec<Constant>),
}

/// IR values (operands) - with deterministic ordering
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    /// Constant values (always ordered first)
    Constant(Constant),

    /// Global variable
    Global(GlobalId),

    /// Local SSA variable
    Local(LocalId),

    /// Function reference
    Function(FunctionId),

    /// Basic block label
    Label(BlockId),
}

/// Local SSA variable - deterministic naming
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalId {
    /// Source instruction address (primary key)
    pub source: Address,

    /// Purpose/type (e.g., "load", "addr", "result")
    pub purpose: &'static str,

    /// Index for same purpose at same address
    pub index: u32,

    /// SSA version (assigned during SSA construction)
    pub version: u32,
}

/// Deterministic ordering: by address, then purpose, then index
impl Ord for LocalId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source
            .cmp(&other.source)
            .then_with(|| self.purpose.cmp(other.purpose))
            .then_with(|| self.index.cmp(&other.index))
            .then_with(|| self.version.cmp(&other.version))
    }
}

impl PartialOrd for LocalId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for LocalId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Deterministic format: %purpose_address_index.version
        if self.version == 0 {
            write!(
                f,
                "%{}_{:016x}_{}",
                self.purpose,
                self.source.get_virtual_address(),
                self.index
            )
        } else {
            write!(
                f,
                "%{}_{:016x}_{}.{}",
                self.purpose,
                self.source.get_virtual_address(),
                self.index,
                self.version
            )
        }
    }
}

/// Helper for creating temporaries deterministically
pub struct TempAllocator {
    counters: BTreeMap<(Address, &'static str), u32>,
}

impl TempAllocator {
    pub fn new() -> Self {
        Self {
            counters: BTreeMap::new(),
        }
    }

    pub fn new_temp(&mut self, addr: Address, purpose: &'static str) -> LocalId {
        let key = (addr.clone(), purpose);
        let index = self.counters.entry(key).or_insert(0);
        let current = *index;
        *index += 1;

        LocalId {
            source: addr,
            purpose,
            index: current,
            version: 0, // Will be set during SSA
        }
    }

    /// MUST reset between functions
    pub fn reset(&mut self) {
        self.counters.clear();
    }
}

/// CRITICAL: Deterministic ordering for canonical forms
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use Value::*;
        match (self, other) {
            // Constants always come first
            (Constant(a), Constant(b)) => a.cmp(b),
            (Constant(_), _) => Ordering::Less,
            (_, Constant(_)) => Ordering::Greater,

            // Then globals (by ID)
            (Global(a), Global(b)) => a.cmp(b),
            (Global(_), _) => Ordering::Less,
            (_, Global(_)) => Ordering::Greater,

            // Then functions (by address)
            (Function(a), Function(b)) => a.cmp(b),
            (Function(_), _) => Ordering::Less,
            (_, Function(_)) => Ordering::Greater,

            // Then locals (by source addr, name, version)
            (Local(a), Local(b)) => a.cmp(b),
            (Local(_), Label(_)) => Ordering::Less,
            (Label(_), Local(_)) => Ordering::Greater,

            // Finally labels (by block address)
            (Label(a), Label(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
    // Arithmetic (canonical order for commutative ops)
    Add,
    Sub,
    Mul,
    SDiv,
    UDiv,
    SRem,
    URem,

    // Bitwise (canonical order)
    And,
    Or,
    Xor,
    Shl,
    LShr,
    AShr,

    // Comparison (not commutative)
    Eq,
    Ne,
    Slt,
    Sle,
    Sgt,
    Sge,
    Ult,
    Ule,
    Ugt,
    Uge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryOp {
    Neg,  // Arithmetic negation
    Not,  // Bitwise NOT
    FNeg, // Floating-point negation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CastOp {
    // Integer casts
    Trunc, // Truncate to smaller integer
    ZExt,  // Zero extend
    SExt,  // Sign extend

    // Pointer casts
    PtrToInt,
    IntToPtr,

    // Float casts
    FPTrunc,
    FPExt,
    FPToUI,
    FPToSI,
    UIToFP,
    SIToFP,

    // Bitcast (no value change)
    Bitcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CallConv {
    /// Default C calling convention (platform-specific)
    C,
    /// Fast calling convention (compiler optimized)
    Fast,
    /// Cold calling convention (for rarely called functions)
    Cold,
    /// x86 stdcall (callee cleanup)
    X86Stdcall,
    /// x86 fastcall (first 2 params in ECX, EDX)
    X86Fastcall,
    /// x86 thiscall (this in ECX)
    X86Thiscall,
    /// x86 vectorcall (extended fastcall with vector support)
    X86Vectorcall,
    /// x86-64 System V ABI (Linux, macOS, BSD)
    X86_64SysV,
    /// x86-64 Windows calling convention
    X86_64Win64,
    /// ARM AAPCS (32-bit ARM)
    ArmAapcs,
    /// ARM AAPCS-VFP (with VFP hardware)
    ArmAapcsVfp,
    /// ARM64 AAPCS (64-bit ARM)
    Arm64Aapcs,
    /// ARM64 AAPCS with Darwin extensions
    Arm64AapcsDarwin,
    /// Preserve all registers
    PreserveAll,
    /// Preserve most registers
    PreserveMost,
}

/// CPU flags that can be affected by operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Flag {
    CF, // Carry
    PF, // Parity
    AF, // Auxiliary carry
    ZF, // Zero
    SF, // Sign
    OF, // Overflow
    DF, // Direction
    IF, // Interrupt enable
}

/// Flag update specification
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlagUpdate {
    /// Flags unchanged
    Unchanged,

    /// Specific flags updated with values
    Update(BTreeMap<Flag, Value>),

    /// All arithmetic flags updated based on operation result
    ArithmeticFlags { result: Value, ty: Type },

    /// All logical flags updated (OF=CF=0, SF/ZF/PF from result)
    LogicalFlags { result: Value, ty: Type },
}

/// Low IR instructions - deterministic and preserving all semantics
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    /// Binary operation: %dst = op %lhs, %rhs
    BinOp {
        op: BinaryOp,
        dst: LocalId,
        lhs: Value,
        rhs: Value,
        ty: Type,
        flags: FlagUpdate,
    },

    /// Unary operation: %dst = op %src
    UnOp {
        op: UnaryOp,
        dst: LocalId,
        src: Value,
        ty: Type,
        flags: FlagUpdate,
    },

    /// Memory load: %dst = load ty* %ptr
    Load {
        dst: LocalId,
        ptr: Value,
        ty: Type,
        align: Option<u32>,
        volatile: bool,
    },

    /// Memory store: store ty %val, ty* %ptr
    Store {
        val: Value,
        ptr: Value,
        ty: Type,
        align: Option<u32>,
        volatile: bool,
    },

    /// Type cast: %dst = cast op %src to ty
    Cast {
        op: CastOp,
        dst: LocalId,
        src: Value,
        src_ty: Type,
        dst_ty: Type,
    },

    /// Function call: %dst = call fn(%args...)
    Call {
        dst: Option<LocalId>,
        func: Value,
        args: Vec<(Value, Type)>,
        conv: CallConv,
    },

    /// PHI node: %dst = phi [%val1, %bb1], [%val2, %bb2], ...
    Phi {
        dst: LocalId,
        incoming: BTreeMap<BlockId, Value>,
        ty: Type,
    },

    /// Select: %dst = select %cond, %true_val, %false_val
    Select {
        dst: LocalId,
        cond: Value,
        true_val: Value,
        false_val: Value,
        ty: Type,
    },

    /// Direct assignment: %dst = %src
    Assign {
        dst: LocalId,
        value: Value,
        source_addr: Address,
    },

    /// Flag read: %dst = read flag
    FlagRead { dst: LocalId, flag: Flag },

    /// Flag write: write flag, %val
    FlagWrite { flag: Flag, value: Value },

    /// x86 specific: CPUID instruction
    CpuId {
        eax_out: LocalId,
        ebx_out: LocalId,
        ecx_out: LocalId,
        edx_out: LocalId,
        eax_in: Value,
        ecx_in: Option<Value>,
    },
}

impl Instruction {
    /// Get the source address for deterministic ordering
    pub fn source_address(&self) -> Address {
        match self {
            Instruction::Assign { source_addr, .. } => source_addr.clone(),
            // For other instructions, extract from the LocalId
            Instruction::BinOp { dst, .. }
            | Instruction::UnOp { dst, .. }
            | Instruction::Load { dst, .. }
            | Instruction::Cast { dst, .. }
            | Instruction::Phi { dst, .. }
            | Instruction::Select { dst, .. }
            | Instruction::FlagRead { dst, .. }
            | Instruction::CpuId { eax_out: dst, .. } => dst.source.clone(),

            // Instructions without dst use Address(0) - they should be rare
            Instruction::Store { .. }
            | Instruction::Call { dst: None, .. }
            | Instruction::FlagWrite { .. } => {
                // Create a dummy address for instructions without dst
                // This should be rare in practice
                let sections = std::sync::Arc::new(crate::core::Sections::default());
                Address::from_virtual_address(&sections, 0)
            }

            Instruction::Call { dst: Some(dst), .. } => dst.source.clone(),
        }
    }
}

/// Block terminators - control flow
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Terminator {
    /// Return: ret ty %val
    Return(Option<(Value, Type)>),

    /// Unconditional branch: br label %dest
    Branch(BlockId),

    /// Conditional branch: br %cond, label %true, label %false
    CondBranch {
        cond: Value,
        true_dest: BlockId,
        false_dest: BlockId,
    },

    /// Switch: switch %val, label %default [val1, label1], ...
    Switch {
        value: Value,
        default: BlockId,
        cases: BTreeMap<Constant, BlockId>,
    },

    /// Indirect branch: indirectbr %addr, [%bb1, %bb2, ...]
    IndirectBranch {
        addr: Value,
        destinations: BTreeSet<BlockId>,
    },

    /// Unreachable
    Unreachable,
}

/// Basic block with deterministic ordering
#[derive(Debug, Clone)]
pub struct BasicBlock {
    /// Block identifier (address-based)
    pub id: BlockId,

    /// PHI nodes (MUST be sorted by destination variable)
    pub phis: Vec<Instruction>,

    /// Regular instructions (in address order)
    pub instructions: Vec<Instruction>,

    /// Block terminator
    pub terminator: Terminator,
}

impl BasicBlock {
    /// Create a new basic block
    pub fn new(id: BlockId, terminator: Terminator) -> Self {
        Self {
            id,
            phis: Vec::new(),
            instructions: Vec::new(),
            terminator,
        }
    }

    /// Ensure deterministic PHI ordering
    pub fn sort_phis(&mut self) {
        self.phis.sort_by_key(|phi| match phi {
            Instruction::Phi { dst, .. } => dst.clone(),
            _ => panic!("Non-PHI in PHI list"),
        });
    }

    /// Verify block is in canonical form
    pub fn verify_determinism(&self) -> Result<(), String> {
        // Check PHIs are sorted
        for window in self.phis.windows(2) {
            if let (Instruction::Phi { dst: dst1, .. }, Instruction::Phi { dst: dst2, .. }) =
                (&window[0], &window[1])
            {
                if dst1 >= dst2 {
                    return Err(format!("PHIs not sorted: {:?} >= {:?}", dst1, dst2));
                }
            }
        }

        // Check instructions are in address order
        let sections = std::sync::Arc::new(crate::core::Sections::default());
        let mut last_addr = Address::from_virtual_address(&sections, 0);
        for inst in &self.instructions {
            let addr = inst.source_address();
            if addr < last_addr {
                return Err("Instructions not in address order".to_string());
            }
            last_addr = addr;
        }

        Ok(())
    }
}

/// Function representation
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name/address
    pub id: FunctionId,

    /// Function signature
    pub signature: Type,

    /// Entry block
    pub entry: BlockId,

    /// All blocks (ordered by address)
    pub blocks: BTreeMap<BlockId, BasicBlock>,

    /// Local variable types
    pub locals: BTreeMap<LocalId, Type>,
}

/// Module - collection of functions
#[derive(Debug, Clone)]
pub struct Module {
    /// Target architecture
    pub target: TargetInfo,

    /// Global variables (ordered)
    pub globals: BTreeMap<GlobalId, Global>,

    /// Functions (ordered by address)
    pub functions: BTreeMap<FunctionId, Function>,

    /// External functions
    pub externals: BTreeMap<String, Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetInfo {
    pub arch: String,
    pub bits: u32,
    pub endian: Endianness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

/// Global ID - deterministic naming
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlobalId(pub String);

/// Function ID - based on address
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionId(pub u64);

/// Block ID - based on address
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockId(pub u64);

#[derive(Debug, Clone)]
pub struct Global {
    pub id: GlobalId,
    pub ty: Type,
    pub init: Option<Constant>,
    pub is_const: bool,
}

impl Module {
    pub fn new(target: TargetInfo) -> Self {
        Self {
            target,
            globals: BTreeMap::new(),
            functions: BTreeMap::new(),
            externals: BTreeMap::new(),
        }
    }
}

impl TargetInfo {
    pub fn x86_64() -> Self {
        Self {
            arch: "x86_64".to_string(),
            bits: 64,
            endian: Endianness::Little,
        }
    }
}

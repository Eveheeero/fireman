//! Medium-level IR (Pattern Recognition Layer)
//!
//! This module implements pattern recognition on top of Low IR,
//! identifying common code patterns like loops, function calls,
//! and control structures with confidence scoring.

use crate::core::Address;
use crate::ir::low_ir::{self, Module as LowModule};
use std::collections::BTreeMap;

pub mod analyzer;
pub mod confidence;
pub mod pattern_database;
pub mod pattern_matcher;
pub mod pattern_parser;
pub mod patterns;

// Re-export commonly used items
pub use pattern_database::{PatternDatabaseBuilder, create_standard_pattern_database};
pub use pattern_matcher::PatternMatcher as PatternMatcherEngine;
pub use pattern_parser::{ParsedPattern, PatternParser};

/// Pattern match confidence level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Confidence(pub u8); // 0-100

impl Confidence {
    pub const CERTAIN: Self = Self(100);
    pub const HIGH: Self = Self(85);
    pub const MEDIUM: Self = Self(70);
    pub const LOW: Self = Self(50);
    pub const NONE: Self = Self(0);
}

/// High-level pattern types detected in Medium IR
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pattern {
    /// For loop: for(init; condition; increment) body
    ForLoop {
        init: Option<PatternRef>,
        condition: PatternRef,
        increment: Option<PatternRef>,
        body: PatternRef,
        confidence: Confidence,
    },

    /// While loop: while(condition) body
    WhileLoop {
        condition: PatternRef,
        body: PatternRef,
        confidence: Confidence,
    },

    /// Do-while loop: do body while(condition)
    DoWhileLoop {
        body: PatternRef,
        condition: PatternRef,
        confidence: Confidence,
    },

    /// Function call with arguments
    FunctionCall {
        target: FunctionRef,
        arguments: Vec<PatternRef>,
        return_value: Option<PatternRef>,
        confidence: Confidence,
    },

    /// Switch/case statement
    SwitchCase {
        value: PatternRef,
        cases: BTreeMap<i64, PatternRef>,
        default: Option<PatternRef>,
        confidence: Confidence,
    },

    /// If-else conditional
    IfElse {
        condition: PatternRef,
        then_branch: PatternRef,
        else_branch: Option<PatternRef>,
        confidence: Confidence,
    },

    /// Array access pattern: arr[index]
    ArrayAccess {
        base: PatternRef,
        index: PatternRef,
        element_type: TypeRef,
        is_write: bool,
        confidence: Confidence,
    },

    /// Structure field access: struct->field
    FieldAccess {
        base: PatternRef,
        offset: usize,
        field_type: TypeRef,
        confidence: Confidence,
    },

    /// String operation (strcpy, strlen, etc.)
    StringOperation {
        operation: StringOp,
        operands: Vec<PatternRef>,
        confidence: Confidence,
    },

    /// Memory allocation (malloc, new, etc.)
    MemoryAllocation {
        size: PatternRef,
        allocator: AllocatorType,
        confidence: Confidence,
    },

    /// Memory deallocation (free, delete, etc.)
    MemoryDeallocation {
        pointer: PatternRef,
        deallocator: DeallocatorType,
        confidence: Confidence,
    },

    /// Arithmetic expression
    Expression {
        operation: ExpressionOp,
        operands: Vec<PatternRef>,
        confidence: Confidence,
    },

    /// Direct Low IR reference (no pattern matched)
    LowIR {
        instructions: Vec<low_ir::Instruction>,
        terminator: Option<low_ir::Terminator>,
        source_block: low_ir::BlockId,
        confidence: Confidence,
    },
}

/// Reference to a pattern in the pattern store
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PatternRef(pub u32);

/// Reference to a function
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FunctionRef {
    /// Known library function
    Library { name: String, library: String },
    /// Address-based function
    Address(Address),
    /// Unknown indirect call
    Indirect(PatternRef),
}

/// Type reference for Medium IR
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeRef {
    /// Primitive type
    Primitive(PrimitiveType),
    /// Pointer to another type
    Pointer(Box<TypeRef>),
    /// Array of elements
    Array {
        element: Box<TypeRef>,
        size: Option<usize>,
    },
    /// Structure type
    Struct { name: Option<String>, size: usize },
    /// Unknown type
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrimitiveType {
    Void,
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StringOp {
    Copy,    // strcpy
    Length,  // strlen
    Compare, // strcmp
    Concat,  // strcat
    Find,    // strstr
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AllocatorType {
    Malloc,
    Calloc,
    Realloc,
    New,
    NewArray,
    Custom(u64), // Address of custom allocator
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeallocatorType {
    Free,
    Delete,
    DeleteArray,
    Custom(u64), // Address of custom deallocator
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExpressionOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr,
    Sar,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

/// Medium IR function representation
#[derive(Debug, Clone)]
pub struct Function {
    /// Function identifier
    pub id: low_ir::FunctionId,

    /// Detected function signature
    pub signature: FunctionSignature,

    /// Pattern store for this function
    pub patterns: PatternStore,

    /// Root pattern (function body)
    pub body: PatternRef,

    /// Confidence in overall analysis
    pub confidence: Confidence,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    /// Return type
    pub return_type: TypeRef,

    /// Parameter types
    pub parameters: Vec<(String, TypeRef)>, // (name, type)

    /// Calling convention
    pub convention: low_ir::CallConv,

    /// Is variadic
    pub variadic: bool,
}

/// Storage for patterns with deduplication
#[derive(Debug, Clone)]
pub struct PatternStore {
    patterns: BTreeMap<PatternRef, Pattern>,
    next_id: u32,
}

impl PatternStore {
    pub fn new() -> Self {
        Self {
            patterns: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, pattern: Pattern) -> PatternRef {
        let id = PatternRef(self.next_id);
        self.next_id += 1;
        self.patterns.insert(id, pattern);
        id
    }

    pub fn get(&self, id: PatternRef) -> Option<&Pattern> {
        self.patterns.get(&id)
    }

    pub fn get_mut(&mut self, id: PatternRef) -> Option<&mut Pattern> {
        self.patterns.get_mut(&id)
    }
}

/// Medium IR module
#[derive(Debug, Clone)]
pub struct Module {
    /// Target architecture info
    pub target: low_ir::TargetInfo,

    /// Functions with pattern analysis
    pub functions: BTreeMap<low_ir::FunctionId, Function>,

    /// Global pattern database
    pub global_patterns: PatternDatabase,
}

/// Database of known patterns for matching
#[derive(Debug, Clone)]
pub struct PatternDatabase {
    /// Known library function patterns
    pub library_functions: BTreeMap<String, LibraryPattern>,

    /// Common code idioms
    pub idioms: Vec<IdiomPattern>,

    /// Architecture-specific patterns
    pub arch_patterns: Vec<ArchPattern>,
}

#[derive(Debug, Clone)]
pub struct LibraryPattern {
    pub name: String,
    pub library: String,
    pub signature: FunctionSignature,
    pub behavior: PatternBehavior,
}

#[derive(Debug, Clone)]
pub struct IdiomPattern {
    pub name: String,
    pub description: String,
    pub matcher: PatternMatcher,
    pub confidence_boost: i8, // -100 to +100
}

#[derive(Debug, Clone)]
pub struct ArchPattern {
    pub name: String,
    pub arch: String,
    pub matcher: PatternMatcher,
}

/// Pattern matching rules
#[derive(Debug, Clone)]
pub enum PatternMatcher {
    /// Sequence of instructions
    InstructionSequence(Vec<InstructionMatcher>),

    /// Control flow pattern
    ControlFlow(ControlFlowMatcher),

    /// Data flow pattern
    DataFlow(DataFlowMatcher),

    /// Composite matcher (all must match)
    All(Vec<Box<PatternMatcher>>),

    /// Composite matcher (any must match)
    Any(Vec<Box<PatternMatcher>>),
}

#[derive(Debug, Clone)]
pub enum InstructionMatcher {
    /// Push register
    Push(&'static str),

    /// Move between registers
    MovReg(&'static str, &'static str),

    /// Subtract immediate from register
    SubImm(&'static str, Box<InstructionMatcher>),

    /// Load effective address
    Lea(&'static str, &'static str),

    /// Add register to register
    AddReg(&'static str, &'static str, &'static str),

    /// Pop register
    Pop(&'static str),

    /// Leave instruction
    Leave,

    /// Return instruction
    Ret,

    /// Any value placeholder
    Any,

    /// Custom matcher (note: this can't be Clone, so we'll use a placeholder)
    #[allow(dead_code)]
    Custom,
}

#[derive(Debug, Clone)]
pub struct ControlFlowMatcher {
    // TODO: Define control flow matching rules
}

#[derive(Debug, Clone)]
pub struct DataFlowMatcher {
    // TODO: Define data flow matching rules
}

#[derive(Debug, Clone)]
pub enum PatternBehavior {
    /// Pure function (no side effects)
    Pure,

    /// Modifies memory
    ModifiesMemory { regions: Vec<MemoryRegion> },

    /// I/O operation
    IO { operation: IOOperation },

    /// System call
    SystemCall { number: Option<u32> },
}

#[derive(Debug, Clone)]
pub enum MemoryRegion {
    Stack,
    Heap,
    Global,
    Parameter(usize),
}

#[derive(Debug, Clone)]
pub enum IOOperation {
    FileRead,
    FileWrite,
    NetworkSend,
    NetworkReceive,
    ConsoleInput,
    ConsoleOutput,
}

impl Module {
    /// Create Medium IR from Low IR with pattern analysis
    pub fn from_low_ir(low_module: &LowModule) -> Self {
        let analyzer = analyzer::MediumIRAnalyzer::new();
        analyzer.analyze(low_module)
    }
}

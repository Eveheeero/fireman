# Fireman Decompiler - Idiomatic Rust Style Guide

This guide extends the [official Rust Style Guide](https://doc.rust-lang.org/stable/style-guide/) with decompiler-specific conventions and patterns for the Fireman project.

## Core Principles for Decompiler Development

### 1. Determinism First
```rust
// ✅ Use BTreeMap for deterministic iteration
use std::collections::BTreeMap;
let mut instruction_map: BTreeMap<u64, Instruction> = BTreeMap::new();

// ❌ HashMap can cause non-deterministic output
use std::collections::HashMap;
let mut instruction_map: HashMap<u64, Instruction> = HashMap::new();
```

### 2. Memory Layout Clarity
```rust
// ✅ Document memory representations explicitly
/// Memory layout for x86_64 instruction:
/// ```
/// [REX Prefix] [Opcode] [ModR/M] [SIB] [Displacement] [Immediate]
/// [0-1 byte  ] [1-3 b ] [0-1 b ] [0-1] [0,1,2,4 b  ] [0,1,2,4,8]
/// ```
#[repr(C)]
pub struct X86Instruction {
    pub rex_prefix: Option<u8>,
    pub opcode: OpCode,
    pub modrm: Option<ModRM>,
    pub sib: Option<Sib>,
    pub displacement: Option<Displacement>,
    pub immediate: Option<Immediate>,
}
```

### 3. Error Context for Analysis
```rust
// ✅ Rich error context for decompilation failures
#[derive(Debug, thiserror::Error)]
pub enum DecompilationError {
    #[error("Invalid instruction at address {address:#x}: {reason}")]
    InvalidInstruction { address: u64, reason: String },

    #[error("Unsupported architecture: {arch}")]
    UnsupportedArchitecture { arch: String },

    #[error("Memory access violation at {address:#x}, size {size}")]
    MemoryViolation { address: u64, size: usize },
}
```

## Formatting Conventions

### Indentation and Line Width
- **4 spaces** for indentation (no tabs)
- **100 characters** maximum line width
- **Block indent** over visual indent for better diffs

### Items Organization

#### Import Ordering
```rust
// 1. Standard library
use std::collections::BTreeMap;
use std::fmt;

// 2. External crates (alphabetically)
use capstone::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// 3. Internal modules (alphabetically)
use crate::arch::x86_64::decoder::X86Decoder;
use crate::ir::high_ir::HighIR;
use crate::utils::error::DecompilerResult;

// 4. Local imports
use super::instruction::Instruction;
```

#### Function Definitions
```rust
// ✅ Proper spacing and attribute placement
#[inline]
pub fn decode_instruction(
    bytes: &[u8],
    address: u64,
    arch: Architecture,
) -> DecompilerResult<Instruction> {
    // Implementation
}
```

### Naming Conventions for Decompiler Context

#### Types and Structs
```rust
// ✅ Architecture-specific prefixes
pub struct X86Instruction { /* ... */ }
pub struct ArmInstruction { /* ... */ }
pub struct RiscVInstruction { /* ... */ }

// ✅ IR level indicators
pub struct HighIR { /* ... */ }
pub struct MediumIR { /* ... */ }
pub struct LowIR { /* ... */ }

// ✅ Analysis result types
pub struct ControlFlowGraph { /* ... */ }
pub struct DataFlowAnalysis { /* ... */ }
pub struct CallGraph { /* ... */ }
```

#### Constants for Binary Analysis
```rust
// ✅ Architecture constants
pub const X86_64_MAX_INSTRUCTION_SIZE: usize = 15;
pub const ARM_INSTRUCTION_SIZE: usize = 4;
pub const RISCV_INSTRUCTION_SIZE: usize = 4;

// ✅ Memory layout constants
pub const DEFAULT_TEXT_SECTION_OFFSET: u64 = 0x1000;
pub const DEFAULT_STACK_SIZE: usize = 0x100000;
pub const PAGE_SIZE: usize = 0x1000;
```

## Decompiler-Specific Patterns

### 1. Instruction Modeling
```rust
/// Base trait for all instruction types across architectures
pub trait InstructionTrait {
    type Operand;
    type Register;

    /// Returns the instruction's memory address
    fn address(&self) -> u64;

    /// Returns the instruction's byte representation
    fn bytes(&self) -> &[u8];

    /// Returns the instruction's mnemonic
    fn mnemonic(&self) -> &str;

    /// Returns operands in order
    fn operands(&self) -> &[Self::Operand];

    /// Returns the size in bytes
    fn size(&self) -> usize { self.bytes().len() }
}

// ✅ Architecture-specific implementation
impl InstructionTrait for X86Instruction {
    type Operand = X86Operand;
    type Register = X86Register;

    fn address(&self) -> u64 { self.address }
    fn bytes(&self) -> &[u8] { &self.bytes }
    fn mnemonic(&self) -> &str { &self.mnemonic }
    fn operands(&self) -> &[Self::Operand] { &self.operands }
}
```

### 2. Memory Management for Binary Data
```rust
/// Safe wrapper for binary data with bounds checking
#[derive(Debug, Clone)]
pub struct BinaryData {
    data: Vec<u8>,
    base_address: u64,
}

impl BinaryData {
    pub fn new(data: Vec<u8>, base_address: u64) -> Self {
        Self { data, base_address }
    }

    /// Read bytes with bounds checking
    pub fn read_bytes(&self, address: u64, size: usize) -> Option<&[u8]> {
        let offset = address.checked_sub(self.base_address)? as usize;
        self.data.get(offset..offset.checked_add(size)?)
    }

    /// Read value with endianness handling
    pub fn read_u32_le(&self, address: u64) -> Option<u32> {
        let bytes = self.read_bytes(address, 4)?;
        Some(u32::from_le_bytes(bytes.try_into().ok()?))
    }
}
```

### 3. IR Transformation Patterns
```rust
/// High-level IR node for decompilation
#[derive(Debug, Clone, PartialEq)]
pub enum HighIRNode {
    // Control flow
    Block(Vec<HighIRNode>),
    If { condition: Box<HighIRNode>, then_branch: Box<HighIRNode>, else_branch: Option<Box<HighIRNode>> },
    While { condition: Box<HighIRNode>, body: Box<HighIRNode> },

    // Expressions
    BinaryOp { op: BinaryOperator, left: Box<HighIRNode>, right: Box<HighIRNode> },
    UnaryOp { op: UnaryOperator, operand: Box<HighIRNode> },
    Variable(String),
    Constant(i64),

    // Memory operations
    Load { address: Box<HighIRNode>, size: usize },
    Store { address: Box<HighIRNode>, value: Box<HighIRNode>, size: usize },

    // Function calls
    Call { target: Box<HighIRNode>, args: Vec<HighIRNode> },
    Return(Option<Box<HighIRNode>>),
}

/// Transformation trait for IR levels
pub trait IRTransform<From, To> {
    type Error;

    fn transform(&self, ir: From) -> Result<To, Self::Error>;
}
```

### 4. Analysis Result Documentation
```rust
/// Control flow analysis results with comprehensive documentation
#[derive(Debug, Clone)]
pub struct ControlFlowAnalysis {
    /// Entry points to the function (typically one, but can be multiple for shared code)
    pub entry_points: BTreeSet<u64>,

    /// Basic blocks indexed by their starting address
    pub basic_blocks: BTreeMap<u64, BasicBlock>,

    /// Control flow edges: (from_address, to_address, edge_type)
    pub edges: Vec<(u64, u64, EdgeType)>,

    /// Detected loops with their headers and backedges
    pub loops: Vec<Loop>,

    /// Unreachable code blocks (possibly dead code or data)
    pub unreachable_blocks: BTreeSet<u64>,
}

impl ControlFlowAnalysis {
    /// Find all paths from entry to a specific address
    pub fn find_paths_to(&self, target: u64) -> Vec<Vec<u64>> {
        // Implementation with proper error handling
        todo!()
    }

    /// Detect natural loops using dominance analysis
    pub fn detect_natural_loops(&mut self) -> Result<(), AnalysisError> {
        // Implementation
        todo!()
    }
}
```

## Error Handling for Decompilers

### Comprehensive Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum DecompilerError {
    // Binary parsing errors
    #[error("Invalid ELF header: {reason}")]
    InvalidElfHeader { reason: String },

    #[error("Invalid PE header: {reason}")]
    InvalidPeHeader { reason: String },

    #[error("Invalid Mach-O header: {reason}")]
    InvalidMachoHeader { reason: String },

    // Instruction decoding errors
    #[error("Failed to decode instruction at {address:#x}: {reason}")]
    InstructionDecodeError { address: u64, reason: String },

    #[error("Unsupported instruction: {mnemonic} at {address:#x}")]
    UnsupportedInstruction { mnemonic: String, address: u64 },

    // Analysis errors
    #[error("Control flow analysis failed: {reason}")]
    ControlFlowError { reason: String },

    #[error("Data flow analysis failed: {reason}")]
    DataFlowError { reason: String },

    // IR transformation errors
    #[error("Failed to transform {from} to {to}: {reason}")]
    IRTransformError { from: String, to: String, reason: String },

    // Memory errors
    #[error("Memory access out of bounds: address {address:#x}, size {size}")]
    MemoryOutOfBounds { address: u64, size: usize },

    #[error("Invalid memory alignment: address {address:#x}, required alignment {alignment}")]
    InvalidMemoryAlignment { address: u64, alignment: usize },
}

/// Specialized Result type for decompiler operations
pub type DecompilerResult<T> = Result<T, DecompilerError>;
```

## Testing Patterns for Decompilers

### Property-Based Testing for Instruction Decoding
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn instruction_roundtrip_property(
            bytes in prop::collection::vec(any::<u8>(), 1..=15)
        ) {
            // Property: If we can decode an instruction, re-encoding should produce the same bytes
            if let Ok(instruction) = decode_instruction(&bytes, 0x1000, Architecture::X86_64) {
                let reencoded = instruction.encode()?;
                prop_assert_eq!(bytes[..instruction.size()], reencoded);
            }
        }

        #[test]
        fn control_flow_analysis_determinism(
            instructions in prop::collection::vec(any_valid_instruction(), 1..100)
        ) {
            // Property: CFG analysis should always produce the same result for same input
            let cfg1 = analyze_control_flow(&instructions)?;
            let cfg2 = analyze_control_flow(&instructions)?;
            prop_assert_eq!(cfg1, cfg2);
        }
    }

    #[test]
    fn decode_common_x86_instructions() {
        let test_cases = [
            (vec![0x48, 0x89, 0xe5], "mov rbp, rsp"),  // mov rbp, rsp
            (vec![0x90], "nop"),                        // nop
            (vec![0xc3], "ret"),                        // ret
            (vec![0xe8, 0x00, 0x00, 0x00, 0x00], "call"), // call rel32
        ];

        for (bytes, expected_mnemonic) in test_cases {
            let instruction = decode_instruction(&bytes, 0x1000, Architecture::X86_64)
                .expect("Failed to decode instruction");
            assert_eq!(instruction.mnemonic(), expected_mnemonic);
        }
    }
}
```

### Snapshot Testing for IR Transformations
```rust
#[cfg(test)]
mod ir_transformation_tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_high_ir_to_c_transformation() {
        let high_ir = HighIRNode::If {
            condition: Box::new(HighIRNode::BinaryOp {
                op: BinaryOperator::GreaterThan,
                left: Box::new(HighIRNode::Variable("x".to_string())),
                right: Box::new(HighIRNode::Constant(10)),
            }),
            then_branch: Box::new(HighIRNode::Return(Some(Box::new(HighIRNode::Constant(1))))),
            else_branch: Some(Box::new(HighIRNode::Return(Some(Box::new(HighIRNode::Constant(0)))))),
        };

        let c_code = transform_to_c(&high_ir).expect("Transformation failed");
        assert_debug_snapshot!(c_code);
    }
}
```

## Performance Considerations

### Memory-Efficient Collections
```rust
// ✅ Use appropriate collection types for decompiler data
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use smallvec::SmallVec;

/// Basic block with small-vector optimization for common case of few instructions
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub start_address: u64,
    pub end_address: u64,
    /// Most basic blocks have < 16 instructions, optimize for this case
    pub instructions: SmallVec<[Instruction; 16]>,
    pub successors: SmallVec<[u64; 2]>,  // Most blocks have 0-2 successors
    pub predecessors: SmallVec<[u64; 4]>, // Can have more predecessors due to branches
}
```

### Zero-Copy Parsing Where Possible
```rust
/// Zero-copy string table for binary format parsing
#[derive(Debug)]
pub struct StringTable<'a> {
    data: &'a [u8],
    base_offset: usize,
}

impl<'a> StringTable<'a> {
    pub fn new(data: &'a [u8], base_offset: usize) -> Self {
        Self { data, base_offset }
    }

    /// Get string at offset without copying
    pub fn get_str(&self, offset: usize) -> Option<&'a str> {
        let start = self.base_offset.checked_add(offset)?;
        let string_data = self.data.get(start..)?;
        let null_pos = string_data.iter().position(|&b| b == 0)?;
        std::str::from_utf8(&string_data[..null_pos]).ok()
    }
}
```

## Documentation Standards

### API Documentation Template
```rust
/// Analyzes the control flow of a function starting at the given address.
///
/// This function performs a comprehensive control flow analysis including:
/// - Basic block identification
/// - Edge detection (conditional/unconditional jumps, calls, returns)
/// - Loop detection using dominance analysis
/// - Dead code identification
///
/// # Arguments
///
/// * `instructions` - A slice of decoded instructions in address order
/// * `entry_point` - The function's entry point address
/// * `arch` - Target architecture for architecture-specific analysis
///
/// # Returns
///
/// Returns a `ControlFlowAnalysis` containing all discovered control flow information,
/// or a `DecompilerError` if analysis fails.
///
/// # Errors
///
/// This function will return an error if:
/// - The entry point is not found in the instruction list
/// - Instructions contain invalid control flow (e.g., jump to invalid address)
/// - Memory constraints prevent analysis completion
///
/// # Examples
///
/// ```rust
/// let instructions = decode_function_instructions(&binary_data, 0x1000)?;
/// let cfg = analyze_control_flow(&instructions, 0x1000, Architecture::X86_64)?;
///
/// println!("Found {} basic blocks", cfg.basic_blocks.len());
/// for (addr, block) in &cfg.basic_blocks {
///     println!("Block at {:#x}: {} instructions", addr, block.instructions.len());
/// }
/// ```
///
/// # Note
///
/// This analysis assumes that all instructions in the slice belong to a single function.
/// For analysis across function boundaries, use `analyze_program_control_flow`.
///
/// # Performance
///
/// Time complexity: O(n log n) where n is the number of instructions.
/// Space complexity: O(n) for storing the control flow graph.
pub fn analyze_control_flow(
    instructions: &[Instruction],
    entry_point: u64,
    arch: Architecture,
) -> DecompilerResult<ControlFlowAnalysis> {
    // Implementation
    todo!()
}
```

This comprehensive style guide ensures that Fireman's codebase maintains high quality, readability, and consistency while addressing the specific needs of decompiler development.

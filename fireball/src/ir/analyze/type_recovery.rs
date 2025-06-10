//! Enhanced type recovery system for decompilation
//!
//! This module implements advanced type inference algorithms that go beyond
//! the basic type tracking in datatype.rs. It uses multiple analysis passes:
//! 1. Usage-based inference (how values are used in operations)
//! 2. Constraint propagation (forward and backward type flow)
//! 3. Function signature matching
//! 4. Struct/array pattern detection
//! 5. Confidence scoring for type decisions

use crate::{
    ir::{
        Ir,
        analyze::DataType,
        data::{AccessSize, IrData, IrDataOperation},
        operator::{BinaryOperator, UnaryOperator},
        statements::{IrStatement, IrStatementSpecial},
    },
    utils::Aos,
};
use std::collections::BTreeMap;

/// Enhanced type information with confidence scoring
#[derive(Debug, Clone)]
pub struct TypeInfo {
    /// The inferred type
    pub ty: InferredType,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Sources of type information
    pub sources: Vec<TypeSource>,
    /// Constraints this type must satisfy
    pub constraints: Vec<TypeConstraint>,
}

/// More detailed type representation than the basic DataType
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferredType {
    /// Unknown type
    Unknown,
    /// Boolean
    Bool,
    /// Integer with optional signedness and bit width
    Integer {
        signed: Option<bool>,
        bits: Option<u32>,
    },
    /// Floating point with bit width
    Float { bits: u32 },
    /// Pointer to another type
    Pointer {
        pointee: Box<InferredType>,
        is_array: bool,
    },
    /// Array with element type and optional size
    Array {
        element: Box<InferredType>,
        size: Option<usize>,
    },
    /// Structure with fields
    Struct {
        fields: BTreeMap<usize, StructField>,
        size: Option<usize>,
        name: Option<String>,
    },
    /// Function pointer
    Function {
        return_type: Box<InferredType>,
        params: Vec<InferredType>,
        varargs: bool,
    },
    /// String (special case of char array)
    String,
    /// Wide string (wchar_t array)
    WideString,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub offset: usize,
    pub ty: InferredType,
    pub name: Option<String>,
}

/// Source of type information
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeSource {
    /// Explicit type specification in IR
    Explicit,
    /// Inferred from instruction usage
    InstructionUsage(String),
    /// Inferred from function call
    FunctionCall {
        function: String,
        param_index: Option<usize>,
    },
    /// Inferred from comparison with constant
    ConstantComparison(i128),
    /// Inferred from memory access pattern
    MemoryAccess { stride: usize, count: usize },
    /// Propagated from another value
    Propagated(String),
    /// Standard library pattern
    LibraryPattern(String),
}

/// Constraints that a type must satisfy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeConstraint {
    /// Must be compatible with another type
    Compatible(InferredType),
    /// Must have specific size
    Size(usize),
    /// Must be numeric
    Numeric,
    /// Must be a pointer
    Pointer,
    /// Must be indexable (array or pointer)
    Indexable,
    /// Must be callable
    Callable,
    /// Must support arithmetic operations
    Arithmetic,
    /// Must support bitwise operations
    Bitwise,
}

/// Type recovery engine
pub struct TypeRecoveryEngine {
    /// Type information for each value
    types: BTreeMap<String, TypeInfo>, // Using string representation for now
    /// Type constraints between values
    constraints: Vec<(String, String, TypeConstraint)>,
    /// Known function signatures
    function_signatures: BTreeMap<String, FunctionSignature>,
    /// Confidence threshold for type decisions
    confidence_threshold: f32,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: InferredType,
    pub params: Vec<(String, InferredType)>,
    pub varargs: bool,
}

impl Default for TypeRecoveryEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeRecoveryEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            types: BTreeMap::new(),
            constraints: Vec::new(),
            function_signatures: BTreeMap::new(),
            confidence_threshold: 0.7,
        };

        // Initialize with known library functions
        engine.init_library_functions();
        engine
    }

    /// Initialize known library function signatures
    fn init_library_functions(&mut self) {
        // malloc: void* malloc(size_t size)
        self.function_signatures.insert(
            "malloc".to_string(),
            FunctionSignature {
                name: "malloc".to_string(),
                return_type: InferredType::Pointer {
                    pointee: Box::new(InferredType::Unknown),
                    is_array: false,
                },
                params: vec![(
                    "size".to_string(),
                    InferredType::Integer {
                        signed: Some(false),
                        bits: Some(64), // size_t on 64-bit
                    },
                )],
                varargs: false,
            },
        );

        // printf: int printf(const char* format, ...)
        self.function_signatures.insert(
            "printf".to_string(),
            FunctionSignature {
                name: "printf".to_string(),
                return_type: InferredType::Integer {
                    signed: Some(true),
                    bits: Some(32),
                },
                params: vec![(
                    "format".to_string(),
                    InferredType::Pointer {
                        pointee: Box::new(InferredType::String),
                        is_array: false,
                    },
                )],
                varargs: true,
            },
        );

        // strlen: size_t strlen(const char* str)
        self.function_signatures.insert(
            "strlen".to_string(),
            FunctionSignature {
                name: "strlen".to_string(),
                return_type: InferredType::Integer {
                    signed: Some(false),
                    bits: Some(64),
                },
                params: vec![(
                    "str".to_string(),
                    InferredType::Pointer {
                        pointee: Box::new(InferredType::String),
                        is_array: false,
                    },
                )],
                varargs: false,
            },
        );

        // memcpy: void* memcpy(void* dst, const void* src, size_t n)
        self.function_signatures.insert(
            "memcpy".to_string(),
            FunctionSignature {
                name: "memcpy".to_string(),
                return_type: InferredType::Pointer {
                    pointee: Box::new(InferredType::Unknown),
                    is_array: false,
                },
                params: vec![
                    (
                        "dst".to_string(),
                        InferredType::Pointer {
                            pointee: Box::new(InferredType::Unknown),
                            is_array: false,
                        },
                    ),
                    (
                        "src".to_string(),
                        InferredType::Pointer {
                            pointee: Box::new(InferredType::Unknown),
                            is_array: false,
                        },
                    ),
                    (
                        "n".to_string(),
                        InferredType::Integer {
                            signed: Some(false),
                            bits: Some(64),
                        },
                    ),
                ],
                varargs: false,
            },
        );
    }

    /// Main entry point for type recovery
    pub fn recover_types(&mut self, ir: &Ir) -> BTreeMap<String, TypeInfo> {
        // Phase 1: Initial type collection from explicit sources
        self.collect_explicit_types(ir);

        // Phase 2: Infer types from instruction usage
        self.infer_from_usage(ir);

        // Phase 3: Detect arrays and structs
        self.detect_compound_types(ir);

        // Phase 4: Propagate types through constraints
        self.propagate_types();

        // Phase 5: Resolve conflicts and finalize types
        self.resolve_conflicts();

        self.types.clone()
    }

    /// Collect explicitly specified types
    fn collect_explicit_types(&mut self, ir: &Ir) {
        if let Some(statements) = &ir.statements {
            for statement in statements.iter() {
                if let IrStatement::Special(IrStatementSpecial::TypeSpecified {
                    location,
                    size,
                    data_type,
                }) = statement
                {
                    let inferred = self.datatype_to_inferred(data_type, size);
                    self.add_type_info(location.to_string(), inferred, 1.0, TypeSource::Explicit);
                }
            }
        }
    }

    /// Infer types from how values are used in instructions
    fn infer_from_usage(&mut self, ir: &Ir) {
        if let Some(statements) = &ir.statements {
            for statement in statements.iter() {
                match statement {
                    IrStatement::Assignment { from, to, size } => {
                        self.analyze_assignment(from, to, size);
                    }
                    IrStatement::Jump { target } => {
                        self.add_type_info(
                            target.to_string(),
                            InferredType::Pointer {
                                pointee: Box::new(InferredType::Unknown),
                                is_array: false,
                            },
                            0.9,
                            TypeSource::InstructionUsage(target.to_string()),
                        );
                    }
                    IrStatement::JumpByCall { target } => {
                        self.analyze_function_call(target);
                    }
                    IrStatement::Condition { condition, .. } => {
                        self.add_type_info(
                            condition.to_string(),
                            InferredType::Bool,
                            0.8,
                            TypeSource::InstructionUsage(condition.to_string()),
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    /// Analyze assignment for type information
    fn analyze_assignment(&mut self, from: &Aos<IrData>, to: &Aos<IrData>, size: &AccessSize) {
        // If we know the type of one side, propagate to the other
        let from_addr = from.to_string();
        let to_addr = to.to_string();

        // Add size constraint
        if let AccessSize::ResultOfByte(data) = size {
            if let IrData::Constant(bytes) = &**data {
                let bits = bytes * 8;

                // Common patterns
                match bits {
                    8 => {
                        // Could be char or small int
                        self.add_constraint(&from_addr, &to_addr, TypeConstraint::Size(1));
                    }
                    32 => {
                        // Likely int or float
                        self.add_constraint(&from_addr, &to_addr, TypeConstraint::Size(4));
                    }
                    64 => {
                        // Likely pointer, long, or double
                        self.add_constraint(&from_addr, &to_addr, TypeConstraint::Size(8));
                    }
                    _ => {}
                }
            }
        }

        // Add compatibility constraint
        self.constraints.push((
            from_addr,
            to_addr,
            TypeConstraint::Compatible(InferredType::Unknown),
        ));
    }

    /// Analyze function calls for type information
    fn analyze_function_call(&mut self, target: &Aos<IrData>) {
        // TODO: Match against known function signatures
        // For now, just mark as function pointer
        self.add_type_info(
            target.to_string(),
            InferredType::Function {
                return_type: Box::new(InferredType::Unknown),
                params: vec![],
                varargs: false,
            },
            0.7,
            TypeSource::InstructionUsage(target.to_string()),
        );
    }

    /// Detect arrays and structs from access patterns
    fn detect_compound_types(&mut self, ir: &Ir) {
        // Track memory access patterns to detect arrays and structs
        // Look for patterns where addresses are computed by adding offsets
        if let Some(statements) = &ir.statements {
            let mut base_accesses: BTreeMap<String, Vec<usize>> = BTreeMap::new();

            for statement in statements.iter() {
                // Look for memory dereferences in assignments
                if let IrStatement::Assignment { from, to, size: _ } = statement {
                    // Check if 'from' is a dereference of an address calculation
                    if let IrData::Dereference(addr) = &**from {
                        if let IrData::Operation(IrDataOperation::Binary {
                            operator: BinaryOperator::Add,
                            arg1,
                            arg2,
                        }) = &**addr
                        {
                            // Pattern: *(base + offset)
                            if let IrData::Constant(offset) = &**arg2 {
                                base_accesses
                                    .entry(arg1.to_string())
                                    .or_default()
                                    .push(*offset);
                            }
                        }
                    }

                    // Also check 'to' for similar patterns
                    if let IrData::Dereference(addr) = &**to {
                        if let IrData::Operation(IrDataOperation::Binary {
                            operator: BinaryOperator::Add,
                            arg1,
                            arg2,
                        }) = &**addr
                        {
                            if let IrData::Constant(offset) = &**arg2 {
                                base_accesses
                                    .entry(arg1.to_string())
                                    .or_default()
                                    .push(*offset);
                            }
                        }
                    }
                }
            }

            // Analyze the collected access patterns
            for (base, offsets) in base_accesses {
                if offsets.len() >= 2 {
                    let mut sorted_offsets = offsets.clone();
                    sorted_offsets.sort();
                    sorted_offsets.dedup();

                    // Check for array pattern: regular stride
                    if let Some(stride) = self.detect_regular_stride(&sorted_offsets) {
                        self.add_type_info(
                            base.clone(),
                            InferredType::Pointer {
                                pointee: Box::new(InferredType::Array {
                                    element: Box::new(InferredType::Unknown),
                                    size: None,
                                }),
                                is_array: true,
                            },
                            0.7,
                            TypeSource::MemoryAccess {
                                stride,
                                count: sorted_offsets.len(),
                            },
                        );
                    }
                    // Check for struct pattern: specific field offsets
                    else if sorted_offsets.len() >= 2 && sorted_offsets[0] < 256 {
                        // Heuristic: struct fields usually start at low offsets
                        let mut fields = BTreeMap::new();
                        for offset in &sorted_offsets {
                            fields.insert(
                                *offset,
                                StructField {
                                    offset: *offset,
                                    ty: InferredType::Unknown,
                                    name: None,
                                },
                            );
                        }

                        self.add_type_info(
                            base,
                            InferredType::Pointer {
                                pointee: Box::new(InferredType::Struct {
                                    fields,
                                    size: None,
                                    name: None,
                                }),
                                is_array: false,
                            },
                            0.6,
                            TypeSource::InstructionUsage("struct_access".to_string()),
                        );
                    }
                }
            }
        }
    }

    /// Detect if offsets follow a regular stride pattern
    fn detect_regular_stride(&self, offsets: &[usize]) -> Option<usize> {
        if offsets.len() < 2 {
            return None;
        }

        // Calculate differences between consecutive offsets
        let mut diffs = Vec::new();
        for i in 1..offsets.len() {
            diffs.push(offsets[i] - offsets[i - 1]);
        }

        // Check if all differences are the same
        let first_diff = diffs[0];
        if first_diff > 0 && diffs.iter().all(|&d| d == first_diff) {
            Some(first_diff)
        } else {
            None
        }
    }

    /// Propagate types through constraints
    fn propagate_types(&mut self) {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            // Process each constraint
            for (addr1, addr2, constraint) in self.constraints.clone() {
                match constraint {
                    TypeConstraint::Compatible(_) => {
                        // If one side has a type, propagate to the other
                        if let Some(type1) = self.types.get(&addr1).cloned() {
                            if !self.types.contains_key(&addr2) {
                                self.add_type_info(
                                    addr2,
                                    type1.ty.clone(),
                                    type1.confidence * 0.9,
                                    TypeSource::Propagated(addr1),
                                );
                                changed = true;
                            }
                        } else if let Some(type2) = self.types.get(&addr2).cloned() {
                            if !self.types.contains_key(&addr1) {
                                self.add_type_info(
                                    addr1,
                                    type2.ty.clone(),
                                    type2.confidence * 0.9,
                                    TypeSource::Propagated(addr2),
                                );
                                changed = true;
                            }
                        }
                    }
                    _ => {} // TODO: Handle other constraints
                }
            }
        }
    }

    /// Resolve type conflicts and choose best types
    fn resolve_conflicts(&mut self) {
        // TODO: Implement conflict resolution
        // For now, keep the type with highest confidence
    }

    /// Add type information for an address
    fn add_type_info(
        &mut self,
        addr: String,
        ty: InferredType,
        confidence: f32,
        source: TypeSource,
    ) {
        let entry = self.types.entry(addr).or_insert_with(|| TypeInfo {
            ty: InferredType::Unknown,
            confidence: 0.0,
            sources: vec![],
            constraints: vec![],
        });

        // Update if this has higher confidence
        if confidence > entry.confidence {
            entry.ty = ty;
            entry.confidence = confidence;
        }

        entry.sources.push(source);
    }

    /// Add a constraint between two addresses
    fn add_constraint(&mut self, addr1: &str, addr2: &str, constraint: TypeConstraint) {
        self.constraints
            .push((addr1.to_string(), addr2.to_string(), constraint));
    }

    /// Convert basic DataType to InferredType
    fn datatype_to_inferred(&self, data_type: &DataType, size: &AccessSize) -> InferredType {
        match data_type {
            DataType::Unknown => InferredType::Unknown,
            DataType::Bool => InferredType::Bool,
            DataType::Int => {
                let bits = match size {
                    AccessSize::ResultOfByte(data) => {
                        if let IrData::Constant(val) = &**data {
                            match val {
                                1 => Some(8),
                                2 => Some(16),
                                4 => Some(32),
                                8 => Some(64),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
                InferredType::Integer { signed: None, bits }
            }
            DataType::Float => {
                let bits = match size {
                    AccessSize::ResultOfByte(data) => {
                        if let IrData::Constant(val) = &**data {
                            match val {
                                4 => 32,
                                8 => 64,
                                _ => 32,
                            }
                        } else {
                            32
                        }
                    }
                    _ => 32,
                };
                InferredType::Float { bits }
            }
            DataType::StringPointer => InferredType::Pointer {
                pointee: Box::new(InferredType::String),
                is_array: false,
            },
            DataType::Address => InferredType::Pointer {
                pointee: Box::new(InferredType::Unknown),
                is_array: false,
            },
            DataType::Char => InferredType::Integer {
                signed: Some(false),
                bits: Some(8),
            },
        }
    }

    /// Infer type from a constant value
    fn infer_type_from_constant(&mut self, value: usize, addr: &str, size: &AccessSize) {
        // Common constant patterns
        match value {
            0 => {
                // NULL pointer
                self.add_type_info(
                    addr.to_string(),
                    InferredType::Pointer {
                        pointee: Box::new(InferredType::Unknown),
                        is_array: false,
                    },
                    0.5,
                    TypeSource::ConstantComparison(0),
                );
            }
            1 => {
                // Likely boolean true
                self.add_type_info(
                    addr.to_string(),
                    InferredType::Bool,
                    0.6,
                    TypeSource::ConstantComparison(1),
                );
            }
            0x20..=0x7E => {
                // ASCII printable character range
                if let AccessSize::ResultOfByte(data) = size {
                    if let IrData::Constant(1) = &**data {
                        self.add_type_info(
                            addr.to_string(),
                            InferredType::Integer {
                                signed: Some(false),
                                bits: Some(8),
                            },
                            0.7,
                            TypeSource::ConstantComparison(value as i128),
                        );
                    }
                }
            }
            _ => {
                // Try to infer size from the value
                let bits = if value <= 0xFF {
                    Some(8)
                } else if value <= 0xFFFF {
                    Some(16)
                } else if value <= 0xFFFFFFFF {
                    Some(32)
                } else {
                    Some(64)
                };

                self.add_type_info(
                    addr.to_string(),
                    InferredType::Integer { signed: None, bits },
                    0.4,
                    TypeSource::ConstantComparison(value as i128),
                );
            }
        }
    }

    /// Analyze operations for type information
    fn analyze_operation_for_types(&mut self, op: &IrDataOperation, result_addr: &str) {
        match op {
            IrDataOperation::Binary {
                operator,
                arg1,
                arg2,
            } => {
                match operator {
                    BinaryOperator::Add | BinaryOperator::Sub => {
                        // If adding/subtracting to a register like SP/BP, likely pointer arithmetic
                        if let IrData::Register(reg) = &**arg1 {
                            if reg.name().contains("sp") || reg.name().contains("bp") {
                                self.add_type_info(
                                    result_addr.to_string(),
                                    InferredType::Pointer {
                                        pointee: Box::new(InferredType::Unknown),
                                        is_array: false,
                                    },
                                    0.7,
                                    TypeSource::InstructionUsage("pointer_arithmetic".to_string()),
                                );
                            }
                        }

                        // Arithmetic operations preserve numeric types
                        self.add_constraint(
                            &arg1.to_string(),
                            result_addr,
                            TypeConstraint::Arithmetic,
                        );
                        self.add_constraint(
                            &arg2.to_string(),
                            result_addr,
                            TypeConstraint::Arithmetic,
                        );
                    }
                    BinaryOperator::And | BinaryOperator::Or | BinaryOperator::Xor => {
                        // Bitwise operations require integer types
                        self.add_constraint(result_addr, result_addr, TypeConstraint::Bitwise);
                    }
                    BinaryOperator::Equal(_)
                    | BinaryOperator::SignedLess(_)
                    | BinaryOperator::SignedLessOrEqual(_)
                    | BinaryOperator::UnsignedLess(_)
                    | BinaryOperator::UnsignedLessOrEqual(_) => {
                        // Comparison results are boolean
                        self.add_type_info(
                            result_addr.to_string(),
                            InferredType::Bool,
                            0.9,
                            TypeSource::InstructionUsage("comparison".to_string()),
                        );
                    }
                    _ => {}
                }
            }
            IrDataOperation::Unary { operator, arg } => {
                match operator {
                    UnaryOperator::Not => {
                        // Not can be bitwise or logical - infer boolean result
                        self.add_type_info(
                            result_addr.to_string(),
                            InferredType::Bool,
                            0.8,
                            TypeSource::InstructionUsage("not_operation".to_string()),
                        );

                        // The argument should support bitwise operations
                        self.add_constraint(&arg.to_string(), result_addr, TypeConstraint::Bitwise);
                    }
                    UnaryOperator::Negation => {
                        // Negation preserves numeric type
                        self.add_constraint(
                            &arg.to_string(),
                            result_addr,
                            TypeConstraint::Arithmetic,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

// TODO: Implement proper mapping from IrData to addresses
// For now, we'll use a placeholder approach

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_recovery_engine_creation() {
        let engine = TypeRecoveryEngine::new();
        assert!(!engine.function_signatures.is_empty());
        assert!(engine.function_signatures.contains_key("malloc"));
        assert!(engine.function_signatures.contains_key("printf"));
    }

    #[test]
    fn test_library_function_signatures() {
        let engine = TypeRecoveryEngine::new();
        let malloc_sig = engine.function_signatures.get("malloc").unwrap();

        match &malloc_sig.return_type {
            InferredType::Pointer { pointee, .. } => {
                assert!(matches!(**pointee, InferredType::Unknown));
            }
            _ => panic!("malloc should return a pointer"),
        }
    }
}

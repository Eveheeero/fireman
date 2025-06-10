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
        data::{AccessSize, IrData},
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
    fn detect_compound_types(&mut self, _ir: &Ir) {
        // TODO: Implement array stride detection
        // TODO: Implement struct field access pattern detection
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

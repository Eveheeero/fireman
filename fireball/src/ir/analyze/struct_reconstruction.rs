//! Struct and class reconstruction from binary analysis
//!
//! This module implements algorithms to recover high-level struct/class
//! definitions from low-level memory access patterns. It builds on the
//! type recovery system to identify:
//! 1. Struct field layouts and types
//! 2. Inheritance relationships (for C++ classes)
//! 3. Virtual function tables
//! 4. Constructor/destructor patterns
//! 5. Member function associations

use crate::{
    ir::{
        Ir,
        analyze::type_recovery::{InferredType, TypeInfo},
        data::{AccessSize, IrData, IrDataOperation},
        operator::BinaryOperator,
        statements::IrStatement,
    },
    utils::Aos,
};
use std::collections::BTreeMap;

/// Reconstructed structure definition
#[derive(Debug, Clone)]
pub struct ReconstructedStruct {
    /// Name of the struct (if determined)
    pub name: Option<String>,
    /// Size of the struct in bytes
    pub size: usize,
    /// Fields indexed by offset
    pub fields: BTreeMap<usize, ReconstructedField>,
    /// Confidence in the reconstruction (0.0 to 1.0)
    pub confidence: f32,
    /// Source of reconstruction
    pub source: ReconstructionSource,
    /// Methods associated with this struct (for classes)
    pub methods: Vec<String>,
    /// Virtual function table address (if detected)
    pub vtable_addr: Option<usize>,
    /// Parent struct (for inheritance)
    pub parent: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ReconstructedField {
    /// Offset within the struct
    pub offset: usize,
    /// Size of the field
    pub size: usize,
    /// Type of the field
    pub field_type: InferredType,
    /// Name of the field (if determined)
    pub name: Option<String>,
    /// How this field is accessed
    pub access_patterns: Vec<FieldAccess>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldAccess {
    /// Direct read/write
    Direct,
    /// Accessed via pointer
    Indirect,
    /// Passed to function
    FunctionArgument(String),
    /// Returned from function
    FunctionReturn(String),
    /// Used in comparison
    Comparison,
    /// Used in arithmetic
    Arithmetic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReconstructionSource {
    /// Reconstructed from memory access patterns
    MemoryPattern,
    /// Reconstructed from function signatures
    FunctionSignature,
    /// Reconstructed from debug information
    DebugInfo,
    /// Reconstructed from RTTI (C++)
    RuntimeTypeInfo,
    /// Reconstructed from constructor pattern
    Constructor,
}

/// Struct reconstruction engine
pub struct StructReconstructionEngine {
    /// Reconstructed structs indexed by base address pattern
    structs: BTreeMap<String, ReconstructedStruct>,
    /// Type information from type recovery
    type_info: BTreeMap<String, TypeInfo>,
    /// Access patterns for each address
    access_patterns: BTreeMap<String, Vec<MemoryAccess>>,
    /// Function information
    function_info: BTreeMap<String, FunctionInfo>,
    /// Minimum confidence threshold
    confidence_threshold: f32,
}

#[derive(Debug, Clone)]
struct MemoryAccess {
    /// Base address being accessed
    base: String,
    /// Offset from base
    offset: usize,
    /// Size of access
    size: usize,
    /// Type of access
    access_type: AccessType,
    /// Context of access
    context: AccessContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AccessType {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AccessContext {
    /// Access in normal code
    Normal,
    /// Access in initialization code
    Initialization,
    /// Access in cleanup code
    Cleanup,
    /// Access in loop
    Loop,
    /// Access in function prologue
    FunctionPrologue,
    /// Access in function epilogue
    FunctionEpilogue,
}

#[derive(Debug, Clone)]
struct FunctionInfo {
    /// Function address
    address: String,
    /// Parameters with their types
    parameters: Vec<InferredType>,
    /// Return type
    return_type: InferredType,
    /// Whether this looks like a constructor
    is_constructor: bool,
    /// Whether this looks like a destructor
    is_destructor: bool,
    /// Struct types this function operates on
    struct_parameters: Vec<String>,
}

impl StructReconstructionEngine {
    pub fn new() -> Self {
        Self {
            structs: BTreeMap::new(),
            type_info: BTreeMap::new(),
            access_patterns: BTreeMap::new(),
            function_info: BTreeMap::new(),
            confidence_threshold: 0.6,
        }
    }

    /// Set type information from type recovery
    pub fn set_type_info(&mut self, type_info: BTreeMap<String, TypeInfo>) {
        self.type_info = type_info;
    }

    /// Main entry point for struct reconstruction
    pub fn reconstruct_structs(&mut self, ir: &Ir) -> BTreeMap<String, ReconstructedStruct> {
        // Phase 1: Collect memory access patterns
        self.collect_access_patterns(ir);

        // Phase 2: Group accesses by base address
        self.group_accesses_by_base();

        // Phase 3: Reconstruct struct layouts
        self.reconstruct_layouts();

        // Phase 4: Analyze functions for methods
        self.analyze_functions(ir);

        // Phase 5: Detect inheritance and vtables
        self.detect_inheritance();

        // Phase 6: Refine field types
        self.refine_field_types();

        // Phase 7: Generate field names
        self.generate_field_names();

        self.structs.clone()
    }

    /// Collect all memory access patterns from IR
    fn collect_access_patterns(&mut self, ir: &Ir) {
        if let Some(statements) = &ir.statements {
            for statement in statements.iter() {
                match statement {
                    IrStatement::Assignment { from, to, size } => {
                        // Analyze source
                        if let IrData::Dereference(addr) = &**from {
                            self.analyze_memory_access(addr, size, AccessType::Read);
                        }

                        // Analyze destination
                        if let IrData::Dereference(addr) = &**to {
                            self.analyze_memory_access(addr, size, AccessType::Write);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Analyze a memory access for struct patterns
    fn analyze_memory_access(
        &mut self,
        addr: &Aos<IrData>,
        size: &AccessSize,
        access_type: AccessType,
    ) {
        // Look for base + offset pattern
        if let IrData::Operation(IrDataOperation::Binary {
            operator: BinaryOperator::Add,
            arg1,
            arg2,
        }) = &**addr
        {
            if let IrData::Constant(offset) = &**arg2 {
                let base = arg1.to_string();
                let size_bytes = self.get_access_size_bytes(size);

                self.access_patterns
                    .entry(base.clone())
                    .or_insert_with(Vec::new)
                    .push(MemoryAccess {
                        base,
                        offset: *offset,
                        size: size_bytes,
                        access_type,
                        context: AccessContext::Normal, // TODO: Determine context
                    });
            }
        }
    }

    /// Get size in bytes from AccessSize
    fn get_access_size_bytes(&self, size: &AccessSize) -> usize {
        match size {
            AccessSize::ResultOfByte(data) => {
                if let IrData::Constant(bytes) = &**data {
                    *bytes
                } else {
                    8 // Default to pointer size
                }
            }
            AccessSize::ArchitectureSize => 8, // Assume 64-bit
            _ => 8,
        }
    }

    /// Group memory accesses by their base address
    fn group_accesses_by_base(&mut self) {
        // Already grouped in access_patterns
    }

    /// Reconstruct struct layouts from access patterns
    fn reconstruct_layouts(&mut self) {
        let access_patterns = self.access_patterns.clone();

        for (base, accesses) in access_patterns {
            if accesses.len() < 2 {
                continue; // Need multiple accesses to infer struct
            }

            // Find unique offsets and their sizes
            let mut field_map: BTreeMap<usize, (usize, Vec<FieldAccess>)> = BTreeMap::new();

            for access in &accesses {
                let field_access = match access.access_type {
                    AccessType::Read => FieldAccess::Direct,
                    AccessType::Write => FieldAccess::Direct,
                    AccessType::ReadWrite => FieldAccess::Direct,
                };

                field_map
                    .entry(access.offset)
                    .or_insert((access.size, Vec::new()))
                    .1
                    .push(field_access);
            }

            // Create struct from fields
            let mut fields = BTreeMap::new();
            let mut max_offset = 0;

            for (offset, (size, access_patterns)) in field_map {
                fields.insert(
                    offset,
                    ReconstructedField {
                        offset,
                        size,
                        field_type: self.infer_field_type(offset, size, &base),
                        name: None,
                        access_patterns,
                    },
                );

                max_offset = max_offset.max(offset + size);
            }

            // Only create struct if we have reasonable confidence
            if fields.len() >= 2 {
                self.structs.insert(
                    base,
                    ReconstructedStruct {
                        name: None,
                        size: max_offset,
                        fields,
                        confidence: 0.7,
                        source: ReconstructionSource::MemoryPattern,
                        methods: Vec::new(),
                        vtable_addr: None,
                        parent: None,
                    },
                );
            }
        }
    }

    /// Infer field type based on offset, size, and type info
    fn infer_field_type(&self, offset: usize, size: usize, base: &str) -> InferredType {
        // Check if we have type info for this specific access
        let addr = format!("{}+{}", base, offset);
        if let Some(type_info) = self.type_info.get(&addr) {
            return type_info.ty.clone();
        }

        // Otherwise, infer from size
        match size {
            1 => InferredType::Integer {
                signed: None,
                bits: Some(8),
            },
            2 => InferredType::Integer {
                signed: None,
                bits: Some(16),
            },
            4 => InferredType::Integer {
                signed: None,
                bits: Some(32),
            },
            8 => InferredType::Pointer {
                pointee: Box::new(InferredType::Unknown),
                is_array: false,
            },
            _ => InferredType::Unknown,
        }
    }

    /// Analyze functions to find methods and constructors
    fn analyze_functions(&mut self, _ir: &Ir) {
        // TODO: Implement function analysis
        // - Look for functions that take struct pointers as first parameter
        // - Identify constructor patterns (allocation + initialization)
        // - Identify destructor patterns (cleanup + deallocation)
    }

    /// Detect inheritance relationships and virtual tables
    fn detect_inheritance(&mut self) {
        // Look for vtable pointers at offset 0
        for (base, struct_def) in self.structs.iter_mut() {
            if let Some(field) = struct_def.fields.get(&0) {
                if matches!(field.field_type, InferredType::Pointer { .. }) && field.size == 8 {
                    // Potential vtable pointer
                    // TODO: Verify by checking if it points to a table of function pointers
                    struct_def.confidence *= 1.1; // Increase confidence
                }
            }
        }
    }

    /// Refine field types based on usage patterns
    fn refine_field_types(&mut self) {
        // TODO: Implement type refinement
        // - If field is used in string operations, mark as char*
        // - If field is used in arithmetic, mark as numeric
        // - If field is dereferenced, mark as pointer
    }

    /// Generate meaningful field names
    fn generate_field_names(&mut self) {
        // Collect field names first to avoid borrow checker issues
        let mut field_names = Vec::new();
        for (_base, struct_def) in self.structs.iter() {
            for (offset, field) in &struct_def.fields {
                let name = self.generate_field_name(&field.field_type, *offset);
                field_names.push((*offset, name));
            }
        }

        // Apply the names
        let mut name_idx = 0;
        for (_base, struct_def) in self.structs.iter_mut() {
            for (_offset, field) in struct_def.fields.iter_mut() {
                if name_idx < field_names.len() {
                    field.name = Some(field_names[name_idx].1.clone());
                    name_idx += 1;
                }
            }
        }
    }

    /// Generate a field name based on type and offset
    fn generate_field_name(&self, field_type: &InferredType, offset: usize) -> String {
        let base_name = match field_type {
            InferredType::Bool => "flag",
            InferredType::Integer { signed, bits } => match (signed, bits) {
                (Some(false), Some(8)) => "byte",
                (Some(true), Some(32)) => "int",
                (Some(false), Some(32)) => "uint",
                (_, Some(64)) => "val",
                _ => "field",
            },
            InferredType::Float { bits } => match bits {
                32 => "f",
                64 => "d",
                _ => "float",
            },
            InferredType::Pointer { pointee, .. } => match &**pointee {
                InferredType::String => "str",
                InferredType::Function { .. } => {
                    if offset == 0 {
                        "vtable"
                    } else {
                        "func_ptr"
                    }
                }
                _ => "ptr",
            },
            InferredType::Array { .. } => "arr",
            _ => "field",
        };

        format!("{}_{:x}", base_name, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_reconstruction_engine_creation() {
        let engine = StructReconstructionEngine::new();
        assert!(engine.structs.is_empty());
        assert_eq!(engine.confidence_threshold, 0.6);
    }

    #[test]
    fn test_field_name_generation() {
        let engine = StructReconstructionEngine::new();

        let name = engine.generate_field_name(
            &InferredType::Pointer {
                pointee: Box::new(InferredType::String),
                is_array: false,
            },
            0x10,
        );
        assert_eq!(name, "str_10");

        let name = engine.generate_field_name(
            &InferredType::Integer {
                signed: Some(true),
                bits: Some(32),
            },
            0x20,
        );
        assert_eq!(name, "int_20");
    }
}

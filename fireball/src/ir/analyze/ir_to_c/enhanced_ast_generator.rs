//! Enhanced AST generator with architecture-agnostic support
//!
//! This module extends the existing C AST structure to support Enhanced C features
//! while maintaining compatibility with the existing AST infrastructure.
//! It provides architecture-agnostic code generation with configurable numeric display.

use crate::{
    arch::architecture::{ArchitectureInfo, ArchType},
    core::Address,
    ir::{
        analyze::{
            ir_to_c::{
                c_abstract_syntax_tree::{
                    BinaryOperator, CAst, CAstPrintConfig, CType, CValue, Expression, FunctionId,
                    Literal, PrintWithConfig, Statement, UnaryOperator, Variable, VariableId,
                    Wrapped, WrappedStatement,
                },
            },
            struct_reconstruction::ReconstructedStruct,
            type_recovery::InferredType,
        },
        medium_ir::Pattern,
    },
    prelude::*,
};
use std::collections::BTreeMap;

/// Configuration for Enhanced C AST generation
#[derive(Debug, Clone)]
pub struct EnhancedAstConfig {
    /// Use auto for complex type inference
    pub use_auto: bool,
    /// Use nullptr instead of NULL
    pub use_nullptr: bool,
    /// Use fixed-width types (uint32_t vs unsigned int)
    pub use_fixed_width_types: bool,
    /// Use range-based for loops where applicable
    pub use_range_for: bool,
    /// Generate inline comments for uncertainty
    pub generate_uncertainty_comments: bool,
    /// Numeric display format
    pub numeric_format: NumericFormat,
    /// Architecture info for proper type sizing
    pub architecture: Option<ArchitectureInfo>,
    /// Confidence threshold for using enhanced features
    pub confidence_threshold: f32,
}

/// Numeric display format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericFormat {
    /// Hexadecimal (default)
    Hexadecimal,
    /// Decimal
    Decimal,
    /// Binary
    Binary,
    /// Auto-detect based on value
    Auto,
}

impl Default for NumericFormat {
    fn default() -> Self {
        NumericFormat::Hexadecimal
    }
}

impl Default for EnhancedAstConfig {
    fn default() -> Self {
        Self {
            use_auto: true,
            use_nullptr: true,
            use_fixed_width_types: true,
            use_range_for: true,
            generate_uncertainty_comments: true,
            numeric_format: NumericFormat::Hexadecimal,
            architecture: None,
            confidence_threshold: 0.7,
        }
    }
}
/// Get pointer type based on architecture
pub fn get_pointer_type(&self, pointee_type: CType) -> CType {
    CType::Pointer(Box::new(pointee_type))
}

/// Get size_t type based on architecture
pub fn get_size_type(&self) -> CType {
    if self.config.use_fixed_width_types {
        if let Some(arch) = &self.config.architecture {
            match arch.pointer_size {
                32 => CType::UInt32,
                64 => CType::UInt64,
                _ => CType::UInt,
            }
        } else {
            CType::UInt
        }
    } else {
        CType::UInt // size_t
    }
}

/// Get ptrdiff_t type based on architecture
pub fn get_ptrdiff_type(&self) -> CType {
    if self.config.use_fixed_width_types {
        if let Some(arch) = &self.config.architecture {
            match arch.pointer_size {
                32 => CType::Int32,
                64 => CType::Int64,
                _ => CType::Int,
            }
        } else {
            CType::Int
        }
    } else {
        CType::Int // ptrdiff_t
    }
}

/// Get register-sized integer type based on architecture
pub fn get_register_type(&self, signed: bool) -> CType {
    if let Some(arch) = &self.config.architecture {
        match arch.pointer_size {
            32 => if signed { CType::Int32 } else { CType::UInt32 },
            64 => if signed { CType::Int64 } else { CType::UInt64 },
            _ => if signed { CType::Int } else { CType::UInt },
        }
    } else {
        if signed { CType::Int } else { CType::UInt }
    }
}

/// Get type for memory addresses based on architecture
pub fn get_address_type(&self) -> CType {
    if self.config.use_fixed_width_types {
        if let Some(arch) = &self.config.architecture {
            match arch.pointer_size {
                32 => CType::UInt32,
                64 => CType::UInt64,
                _ => CType::UInt,
            }
        } else {
            CType::UInt64 // Default to 64-bit addresses
        }
    } else {
        CType::Pointer(Box::new(CType::Void))
    }
}

/// Calculate struct alignment based on architecture
pub fn get_struct_alignment(&self, max_field_alignment: usize) -> usize {
    if let Some(arch) = &self.config.architecture {
        // Architecture-specific alignment rules
        match arch.arch_type {
            ArchType::X86 | ArchType::X86_64 => {
                // x86/x64 typically aligns to the largest field, max 8 bytes
                max_field_alignment.min(8)
            }
            ArchType::Arm32 => {
                // ARM32 typically aligns to 4 bytes max
                max_field_alignment.min(4)
            }
            ArchType::Arm64 => {
                // ARM64 aligns to the largest field, max 8 bytes
                max_field_alignment.min(8)
            }
            _ => max_field_alignment,
        }
    } else {
        max_field_alignment
    }
}

/// Get type size in bytes based on architecture
pub fn get_type_size(&self, c_type: &CType) -> Option<usize> {
    match c_type {
        CType::Void => Some(0),
        CType::Bool | CType::Char | CType::Int8 | CType::UInt8 => Some(1),
        CType::Int16 | CType::UInt16 => Some(2),
        CType::Int32 | CType::UInt32 | CType::Float => Some(4),
        CType::Int64 | CType::UInt64 | CType::Double => Some(8),
        CType::Int | CType::UInt => {
            // Architecture-dependent sizes
            if let Some(arch) = &self.config.architecture {
                match arch.arch_type {
                    ArchType::X86 | ArchType::Arm32 => Some(4),
                    ArchType::X86_64 | ArchType::Arm64 => Some(4), // int is still 32-bit on 64-bit platforms
                    _ => None,
                }
            } else {
                Some(4) // Default to 32-bit int
            }
        }
        CType::Pointer(_) => {
            if let Some(arch) = &self.config.architecture {
                Some((arch.pointer_size / 8) as usize)
            } else {
                Some(8) // Default to 64-bit pointers
            }
        }
        CType::Array(elem_type, count) => {
            self.get_type_size(elem_type).map(|elem_size| elem_size * count)
        }
        _ => None, // Structs/Unions need more complex calculation
    }
}

/// Format an address literal based on architecture
pub fn format_address(&self, address: u64) -> String {
    match self.config.numeric_format {
        NumericFormat::Hexadecimal => {
            if let Some(arch) = &self.config.architecture {
                match arch.pointer_size {
                    32 => format!("0x{:08x}", address as u32),
                    64 => format!("0x{:016x}", address),
                    _ => format!("0x{:x}", address),
                }
            } else {
                format!("0x{:016x}", address) // Default to 64-bit
            }
        }
        NumericFormat::Decimal => format!("{}", address),
        NumericFormat::Binary => format!("0b{:b}", address),
        NumericFormat::Auto => {
            // For addresses, always use hex
            self.format_address_hex(address)
        }
    }
}

/// Helper to format address as hex with proper width
fn format_address_hex(&self, address: u64) -> String {
    if let Some(arch) = &self.config.architecture {
        match arch.pointer_size {
            32 => format!("0x{:08x}", address as u32),
            64 => format!("0x{:016x}", address),
            _ => format!("0x{:x}", address),
        }
    } else {
        format!("0x{:016x}", address)
    }
}

/// Enhanced AST generator that works with existing AST structure
pub struct EnhancedAstGenerator {
    config: EnhancedAstConfig,
    ast: CAst,
    structs: BTreeMap<String, ReconstructedStruct>,
}

impl EnhancedAstGenerator {
    pub fn new(config: EnhancedAstConfig) -> Self {
        Self {
            config,
            ast: CAst::new(),
            structs: BTreeMap::new(),
        }
    }

    /// Set architecture information
    pub fn set_architecture(&mut self, arch: ArchitectureInfo) {
        self.config.architecture = Some(arch);
    }

    /// Set reconstructed structs
    pub fn set_structs(&mut self, structs: BTreeMap<String, ReconstructedStruct>) {
        self.structs = structs;
    }

    /// Convert medium IR patterns to enhanced AST
    pub fn patterns_to_ast(&mut self, patterns: &[Pattern], start_address: &Address) -> &CAst {
        // Generate default function
        let func_id = self.ast.generate_default_function(start_address);

        // Convert patterns to statements
        let statements = self.patterns_to_statements(patterns, &func_id);

        // Add statements to function
        if let Some(func) = self.ast.functions.write().unwrap().get_mut(&func_id) {
            func.body = statements;
        }

        &self.ast
    }

    /// Convert patterns to wrapped statements
    fn patterns_to_statements(
        &mut self,
        patterns: &[Pattern],
        func_id: &FunctionId,
    ) -> Vec<WrappedStatement> {
        patterns
            .iter()
            .filter_map(|pattern| self.pattern_to_statement(pattern, func_id))
            .collect()
    }

    /// Convert a single pattern to statement
    fn pattern_to_statement(
        &mut self,
        pattern: &Pattern,
        func_id: &FunctionId,
    ) -> Option<WrappedStatement> {
        let statement = match pattern {
            Pattern::ForLoop {
                init,
                condition,
                increment,
                body,
                confidence,
            } => self.create_for_loop(init, condition, increment, body, func_id, *confidence),
            Pattern::WhileLoop {
                condition,
                body,
                confidence,
            } => self.create_while_loop(condition, body, func_id, *confidence),
            Pattern::IfElse {
                condition,
                then_branch,
                else_branch,
                confidence,
            } => self.create_if_else(condition, then_branch, else_branch, func_id, *confidence),
            Pattern::SwitchCase {
                value,
                cases,
                default,
                confidence,
            } => self.create_switch(value, cases, default, func_id, *confidence),
            _ => self.create_comment_statement("TODO: Unimplemented pattern"),
        }?;

        Some(WrappedStatement {
            statement,
            from: None,
            comment: if self.config.generate_uncertainty_comments {
                Some("Generated from pattern".to_string())
            } else {
                None
            },
        })
    }

    /// Create a for loop statement
    fn create_for_loop(
        &mut self,
        _init: &Option<Box<Pattern>>,
        _condition: &Pattern,
        _increment: &Option<Box<Pattern>>,
        _body: &Pattern,
        func_id: &FunctionId,
        confidence: crate::ir::medium_ir::Confidence,
    ) -> Option<Statement> {
        // If confidence is too low, generate comment instead
        if confidence.0 < (self.config.confidence_threshold * 100.0) as u8 {
            return self.create_comment_statement(&format!(
                "Low confidence for loop ({}%)",
                confidence.0
            ));
        }

        // Create loop variable with architecture-aware type
        let loop_var = self.create_loop_variable(func_id);

        // TODO: Properly convert patterns to expressions and statements
        // For now, create a placeholder
        Some(Statement::Comment("Enhanced for loop".to_string()))
    }

    /// Create a while loop statement
    fn create_while_loop(
        &mut self,
        _condition: &Pattern,
        _body: &Pattern,
        _func_id: &FunctionId,
        _confidence: crate::ir::medium_ir::Confidence,
    ) -> Option<Statement> {
        // TODO: Implement
        Some(Statement::Comment("Enhanced while loop".to_string()))
    }

    /// Create an if-else statement
    fn create_if_else(
        &mut self,
        _condition: &Pattern,
        _then_branch: &Pattern,
        _else_branch: &Option<Box<Pattern>>,
        _func_id: &FunctionId,
        _confidence: crate::ir::medium_ir::Confidence,
    ) -> Option<Statement> {
        // TODO: Implement
        Some(Statement::Comment("Enhanced if-else".to_string()))
    }

    /// Create a switch statement
    fn create_switch(
        &mut self,
        _value: &Pattern,
        _cases: &BTreeMap<i64, Pattern>,
        _default: &Option<Box<Pattern>>,
        _func_id: &FunctionId,
        _confidence: crate::ir::medium_ir::Confidence,
    ) -> Option<Statement> {
        // TODO: Implement
        Some(Statement::Comment("Enhanced switch".to_string()))
    }

    /// Create a comment statement
    fn create_comment_statement(&self, comment: &str) -> Option<Statement> {
        Some(Statement::Comment(comment.to_string()))
    }

    /// Create a loop variable with proper type based on architecture
    fn create_loop_variable(&mut self, func_id: &FunctionId) -> Variable {
        let var_id = self.ast.new_variable_id(func_id);
        let var_type = self.get_index_type();

        Variable {
            name: format!("i_{}", var_id.get_default_name()),
            id: var_id,
            var_type,
            const_value: None,
        }
    }

    /// Get appropriate index type based on architecture
    fn get_index_type(&self) -> CType {
        if let Some(arch) = &self.config.architecture {
            match arch.pointer_size {
                32 => {
                    if self.config.use_fixed_width_types {
                        CType::UInt32
                    } else {
                        CType::UInt
                    }
                }
                64 => {
                    if self.config.use_fixed_width_types {
                        CType::UInt64
                    } else {
                        CType::UInt
                    }
                }
                _ => CType::UInt,
            }
        } else {
            CType::UInt
        }
    }

    /// Convert InferredType to CType with architecture awareness
    pub fn inferred_to_ctype(&self, inferred: &InferredType) -> CType {
        match inferred {
            InferredType::Bool => CType::Bool,
            InferredType::Integer { signed, bits } => {
                self.get_integer_type(signed.as_ref(), bits.as_ref())
            }
            InferredType::Float { bits } => match bits {
                32 => CType::Float,
                64 => CType::Double,
                _ => CType::Double,
            },
            InferredType::Pointer { pointee, .. } => {
                CType::Pointer(Box::new(self.inferred_to_ctype(pointee)))
            }
            InferredType::Array { element, size } => {
                let elem_type = self.inferred_to_ctype(element);
                CType::Array(Box::new(elem_type), size.unwrap_or(0))
            }
            InferredType::String => CType::Pointer(Box::new(CType::Char)),
            InferredType::WideString => {
                // wchar_t* - use Int16 as base for wide char
                CType::Pointer(Box::new(CType::Int16))
            }
            InferredType::Struct { name, .. } => {
                if let Some(n) = name {
                    CType::Struct(n.clone(), Vec::new())
                } else {
                    CType::Unknown
                }
            }
            _ => CType::Unknown,
        }
    }

    /// Get integer type based on signedness and bit width
    fn get_integer_type(&self, signed: Option<&bool>, bits: Option<&u32>) -> CType {
        match (signed, bits) {
            (Some(true), Some(8)) => CType::Int8,
            (Some(false), Some(8)) => CType::UInt8,
            (Some(true), Some(16)) => CType::Int16,
            (Some(false), Some(16)) => CType::UInt16,
            (Some(true), Some(32)) => CType::Int32,
            (Some(false), Some(32)) => CType::UInt32,
            (Some(true), Some(64)) => CType::Int64,
            (Some(false), Some(64)) => CType::UInt64,
            _ => {
                // Use architecture default
                if let Some(arch) = &self.config.architecture {
                    match arch.pointer_size {
                        32 => CType::Int32,
                        64 => CType::Int64,
                        _ => CType::Int,
                    }
                } else {
                    CType::Int
                }
            }
        }
    }

    /// Create a literal with proper formatting
    pub fn create_literal(&self, value: i64) -> Literal {
        match self.config.numeric_format {
            NumericFormat::Hexadecimal => {
                // Format based on architecture pointer size
                if let Some(arch) = &self.config.architecture {
                    match arch.pointer_size {
                        32 => Literal::UInt(value as u64), // Will be formatted as hex
                        64 => Literal::UInt(value as u64),
                        _ => Literal::Int(value),
                    }
                } else {
                    Literal::UInt(value as u64)
                }
            }
            NumericFormat::Decimal => Literal::Int(value),
            NumericFormat::Binary => Literal::UInt(value as u64), // Will need custom formatting
            NumericFormat::Auto => {
                // Auto-detect: use hex for addresses, decimal for small values
                if value > 0x1000 || (value & 0xFF00) != 0 {
                    Literal::UInt(value as u64)
                } else {
                    Literal::Int(value)
                }
            }
        }
    }
}

/// Extension trait for enhanced printing with numeric format control
pub trait EnhancedPrintWithConfig: PrintWithConfig {
    fn to_enhanced_string(&self, config: EnhancedPrintConfig) -> String;
}

/// Enhanced print configuration
#[derive(Debug, Clone)]
pub struct EnhancedPrintConfig {
    pub base_config: CAstPrintConfig,
    pub numeric_format: NumericFormat,
    pub architecture: Option<ArchitectureInfo>,
}

impl EnhancedPrintWithConfig for CAst {
    fn to_enhanced_string(&self, config: EnhancedPrintConfig) -> String {
        let mut output = String::new();

        // Add header
        output.push_str("/* Enhanced C - Architecture: ");
        if let Some(arch) = &config.architecture {
            output.push_str(arch.arch_type.name());
            output.push_str(&format!(" ({}-bit) */\n", arch.pointer_size));
        } else {
            output.push_str("Unknown */\n");
        }

        // Add includes based on features used
        output.push_str("#include <stdint.h>\n");
        output.push_str("#include <stdbool.h>\n");
        output.push_str("#include <stddef.h> // for nullptr\n\n");

        // Generate functions
        for func in self.functions.read().unwrap().values() {
            output.push_str(&format!("// Function at 0x{:016x}\n", func.id.address));
            output.push_str(&func.to_enhanced_string(config.clone()));
            output.push_str("\n");
        }

        output
    }
}

impl EnhancedPrintWithConfig for crate::ir::analyze::ir_to_c::c_abstract_syntax_tree::Function {
    fn to_enhanced_string(&self, config: EnhancedPrintConfig) -> String {
        let mut output = String::new();

        // Function signature
        output.push_str(&self.return_type.to_enhanced_string(config.clone()));
        output.push(' ');
        output.push_str(&self.name);
        output.push('(');

        // Parameters
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                output.push_str(", ");
            }
            output.push_str(&param.var_type.to_enhanced_string(config.clone()));
            output.push(' ');
            output.push_str(&param.name);
        }

        output.push_str(") {\n");

        // Function body
        for stmt in &self.body {
            output.push_str("    ");
            output.push_str(&stmt.statement.to_enhanced_string(config.clone()));
            output.push('\n');
        }

        output.push_str("}\n");
        output
    }
}

impl EnhancedPrintWithConfig for Statement {
    fn to_enhanced_string(&self, config: EnhancedPrintConfig) -> String {
        // Use existing print logic but with enhanced literal formatting
        self.to_string_with_config(Some(config.base_config))
    }
}

impl EnhancedPrintWithConfig for CType {
    fn to_enhanced_string(&self, _config: EnhancedPrintConfig) -> String {
        match self {
            CType::Void => "void".to_string(),
            CType::Bool => "bool".to_string(),
            CType::Char => "char".to_string(),
            CType::Int8 => "int8_t".to_string(),
            CType::Int16 => "int16_t".to_string(),
            CType::Int32 => "int32_t".to_string(),
            CType::Int64 => "int64_t".to_string(),
            CType::UInt8 => "uint8_t".to_string(),
            CType::UInt16 => "uint16_t".to_string(),
            CType::UInt32 => "uint32_t".to_string(),
            CType::UInt64 => "uint64_t".to_string(),
            CType::Float => "float".to_string(),
            CType::Double => "double".to_string(),
            CType::Pointer(inner) => format!("{}*", inner.to_enhanced_string(_config)),
            CType::Array(inner, size) => {
                format!("{}[{}]", inner.to_enhanced_string(_config), size)
            }
            CType::Struct(name, _) => format!("struct {}", name),
            CType::Union(name, _) => format!("union {}", name),
            _ => self.to_string_with_config(None),
        }
    }
}

/// Format literals according to numeric format configuration
pub fn format_literal(literal: &Literal, format: NumericFormat) -> String {
    match literal {
        Literal::Int(v) => match format {
            NumericFormat::Hexadecimal => {
                if *v < 0 {
                    format!("-0x{:x}", v.abs())
                } else {
                    format!("0x{:x}", v)
                }
            }
            NumericFormat::Decimal => v.to_string(),
            NumericFormat::Binary => format!("0b{:b}", v),
            NumericFormat::Auto => {
                if v.abs() > 256 {
                    format!("0x{:x}", v)
                } else {
                    v.to_string()
                }
            }
        },
        Literal::UInt(v) => match format {
            NumericFormat::Hexadecimal => format!("0x{:x}", v),
            NumericFormat::Decimal => v.to_string(),
            NumericFormat::Binary => format!("0b{:b}", v),
            NumericFormat::Auto => {
                if *v > 256 {
                    format!("0x{:x}", v)
                } else {
                    v.to_string()
                }
            }
        },
        _ => literal.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_format_default() {
        assert_eq!(NumericFormat::default(), NumericFormat::Hexadecimal);
    }

    #[test]
    fn test_format_literal_hex() {
        let lit = Literal::UInt(0xFF);
        assert_eq!(format_literal(&lit, NumericFormat::Hexadecimal), "0xff");
    }

    #[test]
    fn test_format_literal_decimal() {
        let lit = Literal::Int(42);
        assert_eq!(format_literal(&lit, NumericFormat::Decimal), "42");
    }

    #[test]
    fn test_architecture_aware_type() {
        let config = EnhancedAstConfig::default();
        let gen = EnhancedAstGenerator::new(config);

        let int_type = gen.get_integer_type(Some(&true), Some(&32));
        assert_eq!(int_type, CType::Int32);

        let uint_type = gen.get_integer_type(Some(&false), Some(&64));
        assert_eq!(uint_type, CType::UInt64);
    }

    #[test]
    fn test_architecture_aware_size_type() {
        // Test with 32-bit architecture
        let mut config = EnhancedAstConfig::default();
        config.architecture = Some(ArchitectureInfo {
            arch_type: ArchType::X86,
            pointer_size: 32,
            endianness: crate::arch::architecture::Endianness::Little,
            register_count: 8,
            instruction_alignment: 1,
        });
        let gen = EnhancedAstGenerator::new(config);

        assert_eq!(gen.get_size_type(), CType::UInt32);
        assert_eq!(gen.get_ptrdiff_type(), CType::Int32);
        assert_eq!(gen.get_register_type(true), CType::Int32);
        assert_eq!(gen.get_register_type(false), CType::UInt32);
        assert_eq!(gen.get_address_type(), CType::UInt32);

        // Test with 64-bit architecture
        let mut config = EnhancedAstConfig::default();
        config.architecture = Some(ArchitectureInfo {
            arch_type: ArchType::X86_64,
            pointer_size: 64,
            endianness: crate::arch::architecture::Endianness::Little,
            register_count: 16,
            instruction_alignment: 1,
        });
        let gen = EnhancedAstGenerator::new(config);

        assert_eq!(gen.get_size_type(), CType::UInt64);
        assert_eq!(gen.get_ptrdiff_type(), CType::Int64);
        assert_eq!(gen.get_register_type(true), CType::Int64);
        assert_eq!(gen.get_register_type(false), CType::UInt64);
        assert_eq!(gen.get_address_type(), CType::UInt64);
    }

    #[test]
    fn test_type_sizes() {
        let config = EnhancedAstConfig {
            architecture: Some(ArchitectureInfo {
                arch_type: ArchType::X86_64,
                pointer_size: 64,
                endianness: crate::arch::architecture::Endianness::Little,
                register_count: 16,
                instruction_alignment: 1,
            }),
            ..Default::default()
        };
        let gen = EnhancedAstGenerator::new(config);

        assert_eq!(gen.get_type_size(&CType::Void), Some(0));
        assert_eq!(gen.get_type_size(&CType::Bool), Some(1));
        assert_eq!(gen.get_type_size(&CType::Int8), Some(1));
        assert_eq!(gen.get_type_size(&CType::Int16), Some(2));
        assert_eq!(gen.get_type_size(&CType::Int32), Some(4));
        assert_eq!(gen.get_type_size(&CType::Int64), Some(8));
        assert_eq!(gen.get_type_size(&CType::Float), Some(4));
        assert_eq!(gen.get_type_size(&CType::Double), Some(8));
        assert_eq!(gen.get_type_size(&CType::Int), Some(4)); // int is 32-bit even on 64-bit
        assert_eq!(gen.get_type_size(&CType::Pointer(Box::new(CType::Void))), Some(8));
        assert_eq!(gen.get_type_size(&CType::Array(Box::new(CType::Int32), 10)), Some(40));
    }

    #[test]
    fn test_address_formatting() {
        let config = EnhancedAstConfig {
            numeric_format: NumericFormat::Hexadecimal,
            architecture: Some(ArchitectureInfo {
                arch_type: ArchType::X86,
                pointer_size: 32,
                endianness: crate::arch::architecture::Endianness::Little,
                register_count: 8,
                instruction_alignment: 1,
            }),
            ..Default::default()
        };
        let gen = EnhancedAstGenerator::new(config);

        assert_eq!(gen.format_address(0x12345678), "0x12345678");
        assert_eq!(gen.format_address(0x1000), "0x00001000");

        // Test with 64-bit architecture
        let config = EnhancedAstConfig {
            numeric_format: NumericFormat::Hexadecimal,
            architecture: Some(ArchitectureInfo {
                arch_type: ArchType::X86_64,
                pointer_size: 64,
                endianness: crate::arch::architecture::Endianness::Little,
                register_count: 16,
                instruction_alignment: 1,
            }),
            ..Default::default()
        };
        let gen = EnhancedAstGenerator::new(config);

        assert_eq!(gen.format_address(0x12345678), "0x0000000012345678");
        assert_eq!(gen.format_address(0x7fff00001000), "0x00007fff00001000");
    }

    #[test]
    fn test_struct_alignment() {
        let config = EnhancedAstConfig {
            architecture: Some(ArchitectureInfo {
                arch_type: ArchType::X86_64,
                pointer_size: 64,
                endianness: crate::arch::architecture::Endianness::Little,
                register_count: 16,
                instruction_alignment: 1,
            }),
            ..Default::default()
        };
        let gen = EnhancedAstGenerator::new(config);

        assert_eq!(gen.get_struct_alignment(1), 1);
        assert_eq!(gen.get_struct_alignment(4), 4);
        assert_eq!(gen.get_struct_alignment(8), 8);
        assert_eq!(gen.get_struct_alignment(16), 8); // Max 8 bytes on x86_64

        // Test ARM32
        let config = EnhancedAstConfig {
            architecture: Some(ArchitectureInfo {
                arch_type: ArchType::Arm32,
                pointer_size: 32,
                endianness: crate::arch::architecture::Endianness::Little,
                register_count: 16,
                instruction_alignment: 4,
            }),
            ..Default::default()
        };
        let gen = EnhancedAstGenerator::new(config);

        assert_eq!(gen.get_struct_alignment(8), 4); // Max 4 bytes on ARM32
    }
}

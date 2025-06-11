//! Enhanced C code generation for decompiler output
//!
//! This module generates "Enhanced C" - a C-like language that leverages
//! minimal modern C++ features to improve readability and expressiveness
//! while maintaining the decompiler's ability to represent low-level details.
//!
//! Key features:
//! 1. Auto type inference (C++11) for complex types
//! 2. Range-based for loops for array iteration
//! 3. nullptr instead of NULL
//! 4. Fixed-width integer types (uint32_t, int64_t)
//! 5. Inline variable declarations in loops
//! 6. Function overloading hints via comments
//! 7. Anonymous structs/unions for memory layouts

use crate::ir::{
    analyze::{
        struct_reconstruction::ReconstructedStruct,
        type_recovery::{InferredType, TypeInfo},
        variable_naming::VariableName,
    },
    low_ir,
    medium_ir::{
        AllocatorType, Confidence, DeallocatorType, ExpressionOp, FunctionRef, Pattern, PatternRef,
        StringOp, TypeRef,
    },
};
use std::collections::BTreeMap;

/// Configuration for Enhanced C generation
#[derive(Debug, Clone)]
pub struct EnhancedCConfig {
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
    /// Use anonymous structs for memory layouts
    pub use_anonymous_structs: bool,
    /// Maximum line width before breaking
    pub max_line_width: usize,
    /// Confidence threshold for using enhanced features
    pub confidence_threshold: f32,
}

impl Default for EnhancedCConfig {
    fn default() -> Self {
        Self {
            use_auto: true,
            use_nullptr: true,
            use_fixed_width_types: true,
            use_range_for: true,
            generate_uncertainty_comments: true,
            use_anonymous_structs: true,
            max_line_width: 100,
            confidence_threshold: 0.7,
        }
    }
}

/// Enhanced C code generator
pub struct EnhancedCGenerator {
    /// Configuration options
    config: EnhancedCConfig,
    /// Type information from recovery
    type_info: BTreeMap<String, TypeInfo>,
    /// Variable names
    variable_names: BTreeMap<String, VariableName>,
    /// Reconstructed structs
    structs: BTreeMap<String, ReconstructedStruct>,
    /// Current indentation level
    indent_level: usize,
    /// Output buffer
    output: String,
}

impl EnhancedCGenerator {
    pub fn new(config: EnhancedCConfig) -> Self {
        Self {
            config,
            type_info: BTreeMap::new(),
            variable_names: BTreeMap::new(),
            structs: BTreeMap::new(),
            indent_level: 0,
            output: String::new(),
        }
    }

    /// Set analysis results
    pub fn set_analysis_results(
        &mut self,
        type_info: BTreeMap<String, TypeInfo>,
        variable_names: BTreeMap<String, VariableName>,
        structs: BTreeMap<String, ReconstructedStruct>,
    ) {
        self.type_info = type_info;
        self.variable_names = variable_names;
        self.structs = structs;
    }

    /// Generate Enhanced C code from Medium IR patterns
    pub fn generate(&mut self, patterns: &[Pattern]) -> String {
        self.output.clear();

        // Add header comment
        self.emit_header();

        // Generate forward declarations
        self.generate_forward_declarations();

        // Generate struct definitions
        self.generate_struct_definitions();

        // Generate code from patterns
        for pattern in patterns {
            self.generate_pattern(pattern);
        }

        self.output.clone()
    }

    /// Emit header comment
    fn emit_header(&mut self) {
        self.emit_line("/* Enhanced C - Decompiled Output */");
        self.emit_line("/* This code uses minimal C++ features for improved readability */");
        self.emit_line("");

        // Include necessary headers
        self.emit_line("#include <stdint.h>");
        self.emit_line("#include <stdbool.h>");
        if self.config.use_nullptr {
            self.emit_line("#include <cstddef> // for nullptr");
        }
        self.emit_line("");
    }

    /// Generate forward declarations
    fn generate_forward_declarations(&mut self) {
        let structs_clone = self.structs.clone();
        if !structs_clone.is_empty() {
            self.emit_line("/* Forward declarations */");
            for name in structs_clone.keys() {
                let struct_name = self
                    .get_struct_name(name)
                    .unwrap_or_else(|| format!("struct_{}", name));
                self.emit_line(&format!("struct {};", struct_name));
            }
            self.emit_line("");
        }
    }

    /// Generate struct definitions
    fn generate_struct_definitions(&mut self) {
        let structs_clone = self.structs.clone();
        if structs_clone.is_empty() {
            return;
        }

        self.emit_line("/* Recovered struct definitions */");

        for (base, struct_def) in &structs_clone {
            let struct_name = format!("struct_{}", base.replace("+", "_"));

            // Add confidence comment if below threshold
            if struct_def.confidence < self.config.confidence_threshold
                && self.config.generate_uncertainty_comments
            {
                self.emit_line(&format!(
                    "/* WARNING: Low confidence struct ({}%) */",
                    (struct_def.confidence * 100.0) as u32
                ));
            }

            self.emit_line(&format!("struct {} {{", struct_name));
            self.indent();

            // Emit fields
            for (offset, field) in &struct_def.fields {
                let field_type = self.type_to_enhanced_c(&field.field_type);
                let field_name = field
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("field_{:x}", offset));

                // Add offset comment
                self.emit_line(&format!(
                    "{} {}; /* offset: 0x{:x} */",
                    field_type, field_name, offset
                ));
            }

            self.outdent();
            self.emit_line("};");
            self.emit_line("");
        }
    }

    /// Generate pattern code
    fn generate_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::ForLoop {
                init,
                condition,
                increment,
                body,
                confidence,
            } => {
                self.generate_for_loop(
                    init.as_ref(),
                    *condition,
                    increment.as_ref(),
                    *body,
                    *confidence,
                );
            }
            Pattern::WhileLoop {
                condition,
                body,
                confidence,
            } => {
                self.generate_while_loop(*condition, *body, *confidence);
            }
            Pattern::DoWhileLoop {
                body,
                condition,
                confidence,
            } => {
                self.generate_do_while_loop(*body, *condition, *confidence);
            }
            Pattern::IfElse {
                condition,
                then_branch,
                else_branch,
                confidence,
            } => {
                self.generate_if_else(*condition, *then_branch, else_branch.as_ref(), *confidence);
            }
            Pattern::FunctionCall {
                target,
                arguments,
                return_value,
                confidence,
            } => {
                self.generate_function_call_pattern(
                    target,
                    arguments,
                    return_value.as_ref(),
                    *confidence,
                );
            }
            Pattern::ArrayAccess {
                base,
                index,
                element_type,
                is_write,
                confidence,
            } => {
                self.generate_array_access_pattern(
                    *base,
                    *index,
                    element_type,
                    *is_write,
                    *confidence,
                );
            }
            Pattern::FieldAccess {
                base,
                offset,
                field_type,
                confidence,
            } => {
                self.generate_field_access(*base, *offset, field_type, *confidence);
            }
            Pattern::Expression {
                operation,
                operands,
                confidence,
            } => {
                self.generate_expression(operation, operands, *confidence);
            }
            Pattern::SwitchCase {
                value,
                cases,
                default,
                confidence,
            } => {
                self.generate_switch(*value, cases, default.as_ref(), *confidence);
            }
            Pattern::StringOperation {
                operation,
                operands,
                confidence,
            } => {
                self.generate_string_operation(operation, operands, *confidence);
            }
            Pattern::MemoryAllocation {
                size,
                allocator,
                confidence,
            } => {
                self.generate_memory_allocation(*size, allocator, *confidence);
            }
            Pattern::MemoryDeallocation {
                pointer,
                deallocator,
                confidence,
            } => {
                self.generate_memory_deallocation(*pointer, deallocator, *confidence);
            }
            Pattern::LowIR {
                instructions,
                terminator,
                source_block,
                confidence,
            } => {
                self.generate_low_ir(instructions, terminator.as_ref(), source_block, *confidence);
            }
        }
    }

    /// Generate for loop
    fn generate_for_loop(
        &mut self,
        _init: Option<&PatternRef>,
        _condition: PatternRef,
        _increment: Option<&PatternRef>,
        _body: PatternRef,
        _confidence: Confidence,
    ) {
        // TODO: Implement proper pattern resolution
        self.emit_line("/* TODO: For loop generation */");
    }

    /// Generate while loop
    fn generate_while_loop(
        &mut self,
        _condition: PatternRef,
        _body: PatternRef,
        _confidence: Confidence,
    ) {
        // TODO: Implement proper pattern resolution
        self.emit_line("/* TODO: While loop generation */");
    }

    /// Generate do-while loop
    fn generate_do_while_loop(
        &mut self,
        _body: PatternRef,
        _condition: PatternRef,
        _confidence: Confidence,
    ) {
        // TODO: Implement proper pattern resolution
        self.emit_line("/* TODO: Do-while loop generation */");
    }

    /// Generate if-else
    fn generate_if_else(
        &mut self,
        _condition: PatternRef,
        _then_branch: PatternRef,
        _else_branch: Option<&PatternRef>,
        _confidence: Confidence,
    ) {
        // TODO: Implement proper pattern resolution
        self.emit_line("/* TODO: If-else generation */");
    }

    /// Generate function call pattern
    fn generate_function_call_pattern(
        &mut self,
        target: &FunctionRef,
        arguments: &[PatternRef],
        _return_value: Option<&PatternRef>,
        confidence: Confidence,
    ) {
        let func_name = match target {
            FunctionRef::Library { name, .. } => name.clone(),
            FunctionRef::Address(addr) => format!("func_{:016x}", addr.get_virtual_address()),
            FunctionRef::Indirect(_) => "indirect_call".to_string(),
        };

        self.emit(&format!("{}(", func_name));
        for (i, _arg) in arguments.iter().enumerate() {
            if i > 0 {
                self.emit(", ");
            }
            self.emit("/* arg */");
        }
        self.emit_line(");");

        if confidence.0 < self.config.confidence_threshold as u8
            && self.config.generate_uncertainty_comments
        {
            self.emit_line(&format!("/* confidence: {}% */", confidence.0));
        }
    }

    /// Generate array access pattern
    fn generate_array_access_pattern(
        &mut self,
        _base: PatternRef,
        _index: PatternRef,
        _element_type: &TypeRef,
        is_write: bool,
        _confidence: Confidence,
    ) {
        // TODO: Implement proper pattern resolution
        if is_write {
            self.emit_line("/* TODO: Array write */");
        } else {
            self.emit_line("/* TODO: Array read */");
        }
    }

    /// Generate field access
    fn generate_field_access(
        &mut self,
        _base: PatternRef,
        offset: usize,
        _field_type: &TypeRef,
        _confidence: Confidence,
    ) {
        // TODO: Implement proper pattern resolution
        self.emit_line(&format!(
            "/* TODO: Field access at offset 0x{:x} */",
            offset
        ));
    }

    /// Generate expression
    fn generate_expression(
        &mut self,
        operation: &ExpressionOp,
        _operands: &[PatternRef],
        _confidence: Confidence,
    ) {
        use crate::ir::medium_ir::ExpressionOp;

        let op_str = match operation {
            ExpressionOp::Add => "+",
            ExpressionOp::Sub => "-",
            ExpressionOp::Mul => "*",
            ExpressionOp::Div => "/",
            ExpressionOp::Mod => "%",
            ExpressionOp::And => "&&",
            ExpressionOp::Or => "||",
            ExpressionOp::Xor => "^",
            ExpressionOp::Shl => "<<",
            ExpressionOp::Shr => ">>",
            ExpressionOp::Sar => ">>", // arithmetic shift right
            ExpressionOp::Eq => "==",
            ExpressionOp::Ne => "!=",
            ExpressionOp::Lt => "<",
            ExpressionOp::Le => "<=",
            ExpressionOp::Gt => ">",
            ExpressionOp::Ge => ">=",
            ExpressionOp::Not => "!",
        };

        self.emit_line(&format!("/* TODO: Expression {} */", op_str));
    }

    /// Generate switch statement
    fn generate_switch(
        &mut self,
        _value: PatternRef,
        cases: &BTreeMap<i64, PatternRef>,
        default: Option<&PatternRef>,
        _confidence: Confidence,
    ) {
        self.emit_line("switch (/* value */) {");
        self.indent();

        for case_val in cases.keys() {
            self.emit_line(&format!("case {}:", case_val));
            self.indent();
            self.emit_line("/* TODO: Case body */");
            self.emit_line("break;");
            self.outdent();
        }

        if default.is_some() {
            self.emit_line("default:");
            self.indent();
            self.emit_line("/* TODO: Default case */");
            self.emit_line("break;");
            self.outdent();
        }

        self.outdent();
        self.emit_line("}");
    }

    /// Generate string operation
    fn generate_string_operation(
        &mut self,
        operation: &StringOp,
        operands: &[PatternRef],
        _confidence: Confidence,
    ) {
        use crate::ir::medium_ir::StringOp;

        let op_name = match operation {
            StringOp::Copy => "strcpy",
            StringOp::Length => "strlen",
            StringOp::Compare => "strcmp",
            StringOp::Concat => "strcat",
            StringOp::Find => "strstr",
        };

        self.emit(&format!("{}(", op_name));
        for (i, _op) in operands.iter().enumerate() {
            if i > 0 {
                self.emit(", ");
            }
            self.emit("/* operand */");
        }
        self.emit_line(");");
    }

    /// Generate memory allocation
    fn generate_memory_allocation(
        &mut self,
        _size: PatternRef,
        allocator: &AllocatorType,
        confidence: Confidence,
    ) {
        use crate::ir::medium_ir::AllocatorType;

        let alloc_name = match allocator {
            AllocatorType::Malloc => "malloc",
            AllocatorType::Calloc => "calloc",
            AllocatorType::Realloc => "realloc",
            AllocatorType::New => "new",
            AllocatorType::NewArray => "new[]",
            AllocatorType::Custom(_) => "custom_alloc",
        };

        if self.config.use_auto && confidence.0 >= self.config.confidence_threshold as u8 {
            self.emit_line(&format!("auto* ptr = {}(/* size */);", alloc_name));
        } else {
            self.emit_line(&format!("void* ptr = {}(/* size */);", alloc_name));
        }
    }

    /// Generate memory deallocation
    fn generate_memory_deallocation(
        &mut self,
        _pointer: PatternRef,
        deallocator: &DeallocatorType,
        _confidence: Confidence,
    ) {
        use crate::ir::medium_ir::DeallocatorType;

        let dealloc_name = match deallocator {
            DeallocatorType::Free => "free",
            DeallocatorType::Delete => "delete",
            DeallocatorType::DeleteArray => "delete[]",
            DeallocatorType::Custom(_) => "custom_free",
        };

        self.emit_line(&format!("{}(/* pointer */);", dealloc_name));
    }

    /// Generate low IR code
    fn generate_low_ir(
        &mut self,
        instructions: &[low_ir::Instruction],
        terminator: Option<&low_ir::Terminator>,
        source_block: &low_ir::BlockId,
        confidence: Confidence,
    ) {
        self.emit_line(&format!("/* Low IR from block {:016x} */", source_block.0));

        // Generate instructions
        for inst in instructions {
            self.generate_low_ir_instruction(inst);
        }

        // Generate terminator
        if let Some(term) = terminator {
            self.generate_low_ir_terminator(term);
        }

        if confidence.0 < self.config.confidence_threshold as u8
            && self.config.generate_uncertainty_comments
        {
            self.emit_line(&format!("/* confidence: {}% */", confidence.0));
        }
    }

    /// Generate low IR instruction
    fn generate_low_ir_instruction(&mut self, inst: &low_ir::Instruction) {
        use crate::ir::low_ir::Instruction;

        match inst {
            Instruction::Assign { dst, value, .. } => {
                let dst_name = self.convert_local_id(dst);
                let value_str = self.convert_value(value);
                self.emit_line(&format!("{} = {};", dst_name, value_str));
            }
            Instruction::BinOp {
                op, dst, lhs, rhs, ..
            } => {
                let dst_name = self.convert_local_id(dst);
                let lhs_str = self.convert_value(lhs);
                let rhs_str = self.convert_value(rhs);
                let op_str = self.convert_binop(op);
                self.emit_line(&format!(
                    "{} = {} {} {};",
                    dst_name, lhs_str, op_str, rhs_str
                ));
            }
            _ => {
                self.emit_line("/* TODO: Other instruction */");
            }
        }
    }

    /// Generate low IR terminator
    fn generate_low_ir_terminator(&mut self, term: &low_ir::Terminator) {
        use crate::ir::low_ir::Terminator;

        match term {
            Terminator::Return(None) => {
                self.emit_line("return;");
            }
            Terminator::Return(Some((value, _))) => {
                let value_str = self.convert_value(value);
                self.emit_line(&format!("return {};", value_str));
            }
            Terminator::Branch(target) => {
                self.emit_line(&format!("goto L_{:016x};", target.0));
            }
            Terminator::CondBranch {
                cond,
                true_dest,
                false_dest,
            } => {
                let cond_str = self.convert_value(cond);
                self.emit_line(&format!("if ({}) {{", cond_str));
                self.indent();
                self.emit_line(&format!("goto L_{:016x};", true_dest.0));
                self.outdent();
                self.emit_line("} else {");
                self.indent();
                self.emit_line(&format!("goto L_{:016x};", false_dest.0));
                self.outdent();
                self.emit_line("}");
            }
            _ => {
                self.emit_line("/* TODO: Other terminator */");
            }
        }
    }

    /// Convert LocalId to variable name
    fn convert_local_id(&self, local: &low_ir::LocalId) -> String {
        if !local.purpose.is_empty() && local.purpose != "temp" {
            if local.version == 0 {
                local.purpose.to_string()
            } else {
                format!("{}_{}", local.purpose, local.version)
            }
        } else {
            format!("tmp_{}", local.version)
        }
    }

    /// Convert Value to string
    fn convert_value(&self, value: &low_ir::Value) -> String {
        use crate::ir::low_ir::{Constant, Value};

        match value {
            Value::Local(local) => self.convert_local_id(local),
            Value::Constant(constant) => match constant {
                Constant::Int { value, .. } => {
                    format!("{}", value)
                }
                Constant::Float { bits, .. } => format!("0x{:x}", bits),
                _ => "/* const */".to_string(),
            },
            _ => "/* value */".to_string(),
        }
    }

    /// Convert BinaryOp to string
    fn convert_binop(&self, op: &low_ir::BinaryOp) -> &'static str {
        use crate::ir::low_ir::BinaryOp;

        match op {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::UDiv | BinaryOp::SDiv => "/",
            BinaryOp::URem | BinaryOp::SRem => "%",
            BinaryOp::And => "&",
            BinaryOp::Or => "|",
            BinaryOp::Xor => "^",
            BinaryOp::Shl => "<<",
            BinaryOp::LShr | BinaryOp::AShr => ">>",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Ult | BinaryOp::Slt => "<",
            BinaryOp::Ule | BinaryOp::Sle => "<=",
            BinaryOp::Ugt | BinaryOp::Sgt => ">",
            BinaryOp::Uge | BinaryOp::Sge => ">=",
        }
    }

    /// Convert type to Enhanced C representation
    fn type_to_enhanced_c(&self, ty: &InferredType) -> String {
        match ty {
            InferredType::Bool => "bool".to_string(),
            InferredType::Integer { signed, bits } => {
                if self.config.use_fixed_width_types {
                    match (signed, bits) {
                        (Some(true), Some(8)) => "int8_t".to_string(),
                        (Some(false), Some(8)) => "uint8_t".to_string(),
                        (Some(true), Some(16)) => "int16_t".to_string(),
                        (Some(false), Some(16)) => "uint16_t".to_string(),
                        (Some(true), Some(32)) => "int32_t".to_string(),
                        (Some(false), Some(32)) => "uint32_t".to_string(),
                        (Some(true), Some(64)) => "int64_t".to_string(),
                        (Some(false), Some(64)) => "uint64_t".to_string(),
                        _ => "int".to_string(),
                    }
                } else {
                    match (signed, bits) {
                        (Some(false), Some(8)) => "unsigned char".to_string(),
                        (Some(true), Some(16)) => "short".to_string(),
                        (Some(false), Some(16)) => "unsigned short".to_string(),
                        (Some(false), Some(32)) => "unsigned int".to_string(),
                        (Some(true), Some(64)) => "long long".to_string(),
                        (Some(false), Some(64)) => "unsigned long long".to_string(),
                        _ => "int".to_string(),
                    }
                }
            }
            InferredType::Float { bits } => match bits {
                32 => "float".to_string(),
                64 => "double".to_string(),
                _ => "double".to_string(),
            },
            InferredType::Pointer { pointee, .. } => {
                let base = self.type_to_enhanced_c(pointee);
                if self.config.use_auto && matches!(**pointee, InferredType::Unknown) {
                    "auto*".to_string()
                } else {
                    format!("{}*", base)
                }
            }
            InferredType::Array { element, size } => {
                let elem_type = self.type_to_enhanced_c(element);
                if let Some(sz) = size {
                    format!("{}[{}]", elem_type, sz)
                } else {
                    format!("{}[]", elem_type)
                }
            }
            InferredType::Struct { name, .. } => name
                .as_ref()
                .unwrap_or(&"struct unknown".to_string())
                .clone(),
            InferredType::Function {
                return_type,
                params,
                ..
            } => {
                let ret = self.type_to_enhanced_c(return_type);
                let param_types: Vec<String> =
                    params.iter().map(|p| self.type_to_enhanced_c(p)).collect();
                format!("{}({})", ret, param_types.join(", "))
            }
            InferredType::String => "char*".to_string(),
            InferredType::WideString => "wchar_t*".to_string(),
            InferredType::Unknown => {
                if self.config.use_auto {
                    "auto".to_string()
                } else {
                    "void*".to_string()
                }
            }
        }
    }

    /// Get struct name
    fn get_struct_name(&self, base: &str) -> Option<String> {
        self.structs
            .get(base)
            .and_then(|s| s.name.clone())
            .or_else(|| Some(format!("struct_{}", base.replace("+", "_"))))
    }

    /// Enhance a type string
    fn enhance_type(&self, ty: &str) -> String {
        if self.config.use_fixed_width_types {
            match ty {
                "int" => "int32_t",
                "unsigned int" => "uint32_t",
                "long" => "int64_t",
                "unsigned long" => "uint64_t",
                "short" => "int16_t",
                "unsigned short" => "uint16_t",
                "char" => "int8_t",
                "unsigned char" => "uint8_t",
                _ => ty,
            }
        } else {
            ty
        }
        .to_string()
    }

    /// Emit a line with indentation
    fn emit_line(&mut self, line: &str) {
        let indent = "    ".repeat(self.indent_level);
        self.output.push_str(&indent);
        self.output.push_str(line);
        self.output.push('\n');
    }

    /// Emit text without newline
    fn emit(&mut self, text: &str) {
        self.output.push_str(text);
    }

    /// Increase indentation
    fn indent(&mut self) {
        self.indent_level += 1;
    }

    /// Decrease indentation
    fn outdent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_c_generator_creation() {
        let config = EnhancedCConfig::default();
        let generator = EnhancedCGenerator::new(config);
        assert_eq!(generator.indent_level, 0);
        assert!(generator.output.is_empty());
    }

    #[test]
    fn test_type_enhancement() {
        let config = EnhancedCConfig {
            use_fixed_width_types: true,
            ..Default::default()
        };
        let generator = EnhancedCGenerator::new(config);

        assert_eq!(generator.enhance_type("int"), "int32_t");
        assert_eq!(generator.enhance_type("unsigned int"), "uint32_t");
        assert_eq!(generator.enhance_type("long"), "int64_t");
    }

    #[test]
    fn test_type_to_enhanced_c() {
        let config = EnhancedCConfig::default();
        let generator = EnhancedCGenerator::new(config);

        let bool_type = generator.type_to_enhanced_c(&InferredType::Bool);
        assert_eq!(bool_type, "bool");

        let int32_type = generator.type_to_enhanced_c(&InferredType::Integer {
            signed: Some(true),
            bits: Some(32),
        });
        assert_eq!(int32_type, "int32_t");

        let ptr_type = generator.type_to_enhanced_c(&InferredType::Pointer {
            pointee: Box::new(InferredType::Unknown),
            is_array: false,
        });
        assert_eq!(ptr_type, "auto*");
    }
}

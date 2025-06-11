//! Variable naming heuristics for decompilation
//!
//! This module implements intelligent variable naming based on:
//! 1. Variable usage patterns (loop counters, pointers, etc.)
//! 2. Type information from type recovery
//! 3. Function context (parameters, return values)
//! 4. Common programming patterns
//! 5. Library function conventions

use crate::{
    ir::{
        Ir,
        analyze::type_recovery::{InferredType, TypeInfo},
        data::IrData,
        statements::IrStatement,
    },
    utils::Aos,
};
use std::collections::{BTreeMap, BTreeSet};

/// Variable naming engine
pub struct VariableNamingEngine {
    /// Generated names for variables
    names: BTreeMap<String, VariableName>,
    /// Type information from type recovery
    type_info: BTreeMap<String, TypeInfo>,
    /// Usage context for each variable
    usage_contexts: BTreeMap<String, Vec<UsageContext>>,
    /// Name counters for uniqueness
    name_counters: BTreeMap<String, usize>,
    /// Reserved names to avoid
    reserved_names: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct VariableName {
    /// The generated name
    pub name: String,
    /// Confidence in the name (0.0 to 1.0)
    pub confidence: f32,
    /// Source of the naming decision
    pub source: NamingSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamingSource {
    /// Named based on type (e.g., "ptr" for pointers)
    TypeBased,
    /// Named based on usage (e.g., "i" for loop counter)
    UsageBased(String),
    /// Named based on function parameter position
    Parameter(usize),
    /// Named based on return value
    ReturnValue,
    /// Named based on library function convention
    LibraryConvention(String),
    /// Generic fallback name
    Generic,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsageContext {
    /// Used as loop counter
    LoopCounter,
    /// Used in comparison operations
    Comparison,
    /// Used in arithmetic operations
    Arithmetic,
    /// Used as array index
    ArrayIndex,
    /// Used as pointer offset
    PointerOffset,
    /// Passed to function
    FunctionArgument(String),
    /// Returned from function
    FunctionReturn(String),
    /// Used in string operations
    StringOperation,
    /// Used in memory allocation
    MemoryAllocation,
}

impl VariableNamingEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            names: BTreeMap::new(),
            type_info: BTreeMap::new(),
            usage_contexts: BTreeMap::new(),
            name_counters: BTreeMap::new(),
            reserved_names: BTreeSet::new(),
        };

        // Initialize reserved names (C keywords, common macros, etc.)
        engine.init_reserved_names();
        engine
    }

    /// Initialize reserved names that should not be used
    fn init_reserved_names(&mut self) {
        // C keywords
        let keywords = [
            "auto",
            "break",
            "case",
            "char",
            "const",
            "continue",
            "default",
            "do",
            "double",
            "else",
            "enum",
            "extern",
            "float",
            "for",
            "goto",
            "if",
            "int",
            "long",
            "register",
            "return",
            "short",
            "signed",
            "sizeof",
            "static",
            "struct",
            "switch",
            "typedef",
            "union",
            "unsigned",
            "void",
            "volatile",
            "while",
            // C99/C11 keywords
            "_Bool",
            "_Complex",
            "_Imaginary",
            "inline",
            "restrict",
            "_Alignas",
            "_Alignof",
            "_Atomic",
            "_Static_assert",
            "_Noreturn",
            "_Thread_local",
            "_Generic",
            // Common macros
            "NULL",
            "EOF",
            "TRUE",
            "FALSE",
            "MAX",
            "MIN",
        ];

        for keyword in keywords {
            self.reserved_names.insert(keyword.to_string());
        }
    }

    /// Set type information from type recovery
    pub fn set_type_info(&mut self, type_info: BTreeMap<String, TypeInfo>) {
        self.type_info = type_info;
    }

    /// Main entry point for variable naming
    pub fn generate_names(&mut self, ir: &Ir) -> BTreeMap<String, VariableName> {
        // Phase 1: Analyze usage contexts
        self.analyze_usage_contexts(ir);

        // Phase 2: Generate names based on types
        self.generate_type_based_names();

        // Phase 3: Generate names based on usage
        self.generate_usage_based_names();

        // Phase 4: Generate remaining generic names
        self.generate_generic_names();

        // Phase 5: Ensure uniqueness
        self.ensure_unique_names();

        self.names.clone()
    }

    /// Analyze how variables are used in the IR
    fn analyze_usage_contexts(&mut self, ir: &Ir) {
        if let Some(statements) = &ir.statements {
            for statement in statements.iter() {
                match statement {
                    IrStatement::Assignment { from, to, .. } => {
                        // Check for loop counter patterns
                        if self.looks_like_increment(from, to) {
                            self.add_usage_context(to.to_string(), UsageContext::LoopCounter);
                        }

                        // Check for array indexing
                        if let IrData::Dereference(addr) = &**to {
                            if self.looks_like_array_access(addr) {
                                self.add_usage_context(addr.to_string(), UsageContext::ArrayIndex);
                            }
                        }
                    }
                    IrStatement::Condition { condition, .. } => {
                        self.add_usage_context(condition.to_string(), UsageContext::Comparison);
                    }
                    IrStatement::JumpByCall { target } => {
                        // TODO: Track function arguments
                        if let IrData::Constant(_) = &**target {
                            // Function call to known address
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Check if an assignment looks like an increment operation
    fn looks_like_increment(&self, _from: &Aos<IrData>, _to: &Aos<IrData>) -> bool {
        // Simple heuristic: check if we're adding 1 to a variable
        // TODO: Implement more sophisticated pattern matching
        false
    }

    /// Check if an address calculation looks like array access
    fn looks_like_array_access(&self, _addr: &Aos<IrData>) -> bool {
        // Check if it's base + (index * size) pattern
        // TODO: Implement pattern matching
        false
    }

    /// Add a usage context for a variable
    fn add_usage_context(&mut self, var: String, context: UsageContext) {
        self.usage_contexts.entry(var).or_default().push(context);
    }

    /// Generate names based on type information
    fn generate_type_based_names(&mut self) {
        let type_info_copy = self.type_info.clone();
        for (addr, type_info) in &type_info_copy {
            if self.names.contains_key(addr) {
                continue;
            }

            let name = match &type_info.ty {
                InferredType::Bool => self.get_unique_name("flag"),
                InferredType::Integer { signed, bits } => match (signed, bits) {
                    (Some(false), Some(8)) => self.get_unique_name("byte"),
                    (_, Some(32)) => self.get_unique_name("val"),
                    (_, Some(64)) => self.get_unique_name("num"),
                    _ => self.get_unique_name("n"),
                },
                InferredType::Float { bits } => match bits {
                    32 => self.get_unique_name("f"),
                    64 => self.get_unique_name("d"),
                    _ => self.get_unique_name("float_val"),
                },
                InferredType::Pointer { pointee, .. } => match &**pointee {
                    InferredType::String => self.get_unique_name("str"),
                    InferredType::Integer {
                        signed: Some(false),
                        bits: Some(8),
                    } => self.get_unique_name("buf"),
                    _ => self.get_unique_name("ptr"),
                },
                InferredType::Array { .. } => self.get_unique_name("arr"),
                InferredType::Struct { name, .. } => {
                    if let Some(struct_name) = name {
                        self.get_unique_name(&format!("{}_inst", struct_name.to_lowercase()))
                    } else {
                        self.get_unique_name("obj")
                    }
                }
                InferredType::Function { .. } => self.get_unique_name("func"),
                InferredType::String => self.get_unique_name("str"),
                InferredType::WideString => self.get_unique_name("wstr"),
                _ => continue,
            };

            self.names.insert(
                addr.clone(),
                VariableName {
                    name,
                    confidence: type_info.confidence * 0.8,
                    source: NamingSource::TypeBased,
                },
            );
        }
    }

    /// Generate names based on usage patterns
    fn generate_usage_based_names(&mut self) {
        let usage_contexts_copy = self.usage_contexts.clone();
        for (addr, contexts) in &usage_contexts_copy {
            if self.names.contains_key(addr) {
                continue;
            }

            // Score each possible name based on usage
            let mut name_scores: BTreeMap<String, f32> = BTreeMap::new();

            for context in contexts {
                match context {
                    UsageContext::LoopCounter => {
                        *name_scores.entry("i".to_string()).or_insert(0.0) += 0.9;
                        *name_scores.entry("j".to_string()).or_insert(0.0) += 0.7;
                        *name_scores.entry("k".to_string()).or_insert(0.0) += 0.5;
                        *name_scores.entry("idx".to_string()).or_insert(0.0) += 0.6;
                    }
                    UsageContext::ArrayIndex => {
                        *name_scores.entry("idx".to_string()).or_insert(0.0) += 0.8;
                        *name_scores.entry("index".to_string()).or_insert(0.0) += 0.7;
                        *name_scores.entry("i".to_string()).or_insert(0.0) += 0.6;
                    }
                    UsageContext::StringOperation => {
                        *name_scores.entry("str".to_string()).or_insert(0.0) += 0.8;
                        *name_scores.entry("s".to_string()).or_insert(0.0) += 0.7;
                        *name_scores.entry("text".to_string()).or_insert(0.0) += 0.6;
                    }
                    UsageContext::PointerOffset => {
                        *name_scores.entry("offset".to_string()).or_insert(0.0) += 0.8;
                        *name_scores.entry("pos".to_string()).or_insert(0.0) += 0.6;
                    }
                    UsageContext::MemoryAllocation => {
                        *name_scores.entry("size".to_string()).or_insert(0.0) += 0.8;
                        *name_scores.entry("len".to_string()).or_insert(0.0) += 0.7;
                        *name_scores.entry("bytes".to_string()).or_insert(0.0) += 0.6;
                    }
                    _ => {}
                }
            }

            // Choose the highest scoring name
            if let Some((base_name, score)) = name_scores
                .iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            {
                let name = self.get_unique_name(base_name);
                self.names.insert(
                    addr.clone(),
                    VariableName {
                        name,
                        confidence: score / contexts.len() as f32,
                        source: NamingSource::UsageBased("pattern".to_string()),
                    },
                );
            }
        }
    }

    /// Generate generic names for remaining variables
    fn generate_generic_names(&mut self) {
        // TODO: Generate names for variables without specific patterns
        // For now, we'll skip this as we need the actual variable addresses
    }

    /// Ensure all names are unique
    fn ensure_unique_names(&mut self) {
        // Names are already unique due to get_unique_name()
    }

    /// Get a unique name based on a base name
    fn get_unique_name(&mut self, base: &str) -> String {
        // Check if base name is available
        if !self.reserved_names.contains(base) && !self.names.values().any(|v| v.name == base) {
            return base.to_string();
        }

        // Generate numbered variants
        let counter = self.name_counters.entry(base.to_string()).or_insert(0);
        loop {
            *counter += 1;
            let candidate = format!("{}{}", base, counter);
            if !self.reserved_names.contains(&candidate)
                && !self.names.values().any(|v| v.name == candidate)
            {
                return candidate;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_naming_engine_creation() {
        let engine = VariableNamingEngine::new();
        assert!(!engine.reserved_names.is_empty());
        assert!(engine.reserved_names.contains("int"));
        assert!(engine.reserved_names.contains("while"));
    }

    #[test]
    fn test_unique_name_generation() {
        let mut engine = VariableNamingEngine::new();

        let name1 = engine.get_unique_name("i");
        assert_eq!(name1, "i");

        // Simulate that "i" is now used
        engine.names.insert(
            "test".to_string(),
            VariableName {
                name: "i".to_string(),
                confidence: 1.0,
                source: NamingSource::Generic,
            },
        );

        let name2 = engine.get_unique_name("i");
        assert_eq!(name2, "i1");

        // Check reserved names
        let name3 = engine.get_unique_name("int");
        assert_eq!(name3, "int1");
    }
}

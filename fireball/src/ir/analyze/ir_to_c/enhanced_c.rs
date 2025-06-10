//! Enhanced C extensions for the C AST
//!
//! This module provides Enhanced C features by extending the existing C AST
//! with minimal changes, preserving the AST structure for analysis.

use super::c_abstract_syntax_tree::*;
use crate::ir::medium_ir::Confidence;
use std::collections::BTreeMap;

/// Enhanced C configuration options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnhancedCConfig {
    /// Enable Enhanced C features
    pub enabled: bool,
    /// Use auto keyword for type inference
    pub use_auto: bool,
    /// Use nullptr instead of NULL
    pub use_nullptr: bool,
    /// Use fixed-width integer types (int32_t, uint64_t, etc.)
    pub use_fixed_width_types: bool,
    /// Use range-based for loops where applicable
    pub use_range_for: bool,
    /// Generate inline variable declarations in loops
    pub use_inline_declarations: bool,
    /// Add comments for low-confidence transformations
    pub annotate_uncertainty: bool,
    /// Minimum confidence level (0-100) to apply transformations
    pub confidence_threshold: u8,
}

impl Default for EnhancedCConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            use_auto: true,
            use_nullptr: true,
            use_fixed_width_types: true,
            use_range_for: true,
            use_inline_declarations: true,
            annotate_uncertainty: true,
            confidence_threshold: 70,
        }
    }
}

/// Extended print configuration that includes Enhanced C options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtendedPrintConfig {
    /// Base C AST print configuration
    pub base: CAstPrintConfig,
    /// Enhanced C configuration
    pub enhanced: EnhancedCConfig,
}

impl Default for ExtendedPrintConfig {
    fn default() -> Self {
        Self {
            base: CAstPrintConfig::default(),
            enhanced: EnhancedCConfig::default(),
        }
    }
}

/// Enhanced C type mappings
pub fn map_to_enhanced_type(ty: &CType, config: &EnhancedCConfig) -> Option<String> {
    if !config.enabled || !config.use_fixed_width_types {
        return None;
    }

    match ty {
        CType::Int8 => Some("int8_t".to_string()),
        CType::Int16 => Some("int16_t".to_string()),
        CType::Int32 => Some("int32_t".to_string()),
        CType::Int64 => Some("int64_t".to_string()),
        CType::UInt8 => Some("uint8_t".to_string()),
        CType::UInt16 => Some("uint16_t".to_string()),
        CType::UInt32 => Some("uint32_t".to_string()),
        CType::UInt64 => Some("uint64_t".to_string()),
        _ => None,
    }
}

/// Transform nullptr literals in expressions
pub fn transform_nullptr_literal(lit: &Literal, config: &EnhancedCConfig) -> Option<String> {
    if !config.enabled || !config.use_nullptr {
        return None;
    }

    match lit {
        Literal::Int(0) => Some("nullptr".to_string()),
        _ => None,
    }
}

/// Enhanced C AST transformer
pub struct EnhancedCAstTransformer {
    config: EnhancedCConfig,
    confidence_map: BTreeMap<FunctionId, BTreeMap<String, Confidence>>,
}

impl EnhancedCAstTransformer {
    pub fn new(config: EnhancedCConfig) -> Self {
        Self {
            config,
            confidence_map: BTreeMap::new(),
        }
    }

    /// Set confidence information for a function
    pub fn set_confidence(
        &mut self,
        func_id: FunctionId,
        confidence_map: BTreeMap<String, Confidence>,
    ) {
        self.confidence_map.insert(func_id, confidence_map);
    }

    /// Check if a transformation should be applied based on confidence
    fn should_apply(&self, func_id: &FunctionId, key: &str) -> bool {
        if let Some(func_confidence) = self.confidence_map.get(func_id) {
            if let Some(confidence) = func_confidence.get(key) {
                return confidence.0 >= self.config.confidence_threshold;
            }
        }
        // Default to applying if no confidence info
        true
    }

    /// Analyze if a variable declaration can use auto
    pub fn can_use_auto(&self, var: &Variable, init: &Option<Wrapped<Expression>>) -> bool {
        if !self.config.use_auto {
            return false;
        }

        // Only use auto if we have an initializer and the type is complex
        if let Some(_init_expr) = init {
            match &var.var_type {
                CType::Pointer(inner) => {
                    // Use auto for complex pointer types
                    matches!(**inner, CType::Struct(_, _) | CType::Union(_, _))
                }
                CType::Array(_, _) => false, // Can't use auto for arrays
                _ => false,
            }
        } else {
            false
        }
    }

    /// Check if a for loop can be converted to range-based
    pub fn can_convert_to_range_for(
        &self,
        _init: &Statement,
        _cond: &Expression,
        _update: &Statement,
    ) -> Option<(Variable, String)> {
        if !self.config.use_range_for {
            return None;
        }

        // This is a simplified check - in reality, we'd need more complex pattern matching
        // to detect array iteration patterns

        // TODO: Implement proper pattern matching for:
        // - for (int i = 0; i < array_size; i++) { ... array[i] ... }
        // - for (Type* p = array; p < array + size; p++) { ... *p ... }

        None
    }
}

/// Enhanced header generator
pub fn generate_enhanced_headers(config: &EnhancedCConfig) -> String {
    let mut headers = String::new();

    if config.use_fixed_width_types {
        headers.push_str("#include <stdint.h>\n");
    }

    if config.use_nullptr {
        headers.push_str("#ifdef __cplusplus\n");
        headers.push_str("  /* Using built-in nullptr */\n");
        headers.push_str("#else\n");
        headers.push_str("  #define nullptr ((void*)0)\n");
        headers.push_str("#endif\n");
    }

    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_type_mapping() {
        let config = EnhancedCConfig::default();

        assert_eq!(
            map_to_enhanced_type(&CType::Int32, &config),
            Some("int32_t".to_string())
        );

        assert_eq!(
            map_to_enhanced_type(&CType::UInt64, &config),
            Some("uint64_t".to_string())
        );

        // Non-fixed-width types should return None
        assert_eq!(map_to_enhanced_type(&CType::Int, &config), None);
    }

    #[test]
    fn test_nullptr_transform() {
        let config = EnhancedCConfig::default();

        assert_eq!(
            transform_nullptr_literal(&Literal::Int(0), &config),
            Some("nullptr".to_string())
        );

        // Non-zero literals shouldn't be transformed
        assert_eq!(transform_nullptr_literal(&Literal::Int(42), &config), None);
    }
}

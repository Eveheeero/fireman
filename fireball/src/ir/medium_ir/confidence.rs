//! Confidence scoring system for pattern recognition

use super::*;

/// Confidence calculator for patterns
pub struct ConfidenceCalculator {
    /// Base confidence for different pattern types
    base_confidences: BTreeMap<PatternType, u8>,

    /// Confidence modifiers
    modifiers: Vec<ConfidenceModifier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatternType {
    ForLoop,
    WhileLoop,
    DoWhileLoop,
    FunctionCall,
    SwitchCase,
    IfElse,
    ArrayAccess,
    FieldAccess,
    StringOperation,
    MemoryAllocation,
    MemoryDeallocation,
    Expression,
    LowIR,
}

#[derive(Debug, Clone)]
pub struct ConfidenceModifier {
    pub name: String,
    pub condition: ModifierCondition,
    pub adjustment: i8, // -100 to +100
}

#[derive(Debug, Clone)]
pub enum ModifierCondition {
    /// Pattern has specific characteristic
    HasCharacteristic(String),

    /// Pattern matches known idiom
    MatchesIdiom(String),

    /// Pattern has minimum number of supporting evidence
    MinEvidence(usize),

    /// Pattern complexity is within range
    ComplexityRange { min: usize, max: usize },

    /// All sub-patterns have minimum confidence
    SubPatternsConfident(Confidence),
}

impl Default for ConfidenceCalculator {
    fn default() -> Self {
        Self {
            base_confidences: Self::default_base_confidences(),
            modifiers: Self::default_modifiers(),
        }
    }
}

impl ConfidenceCalculator {
    /// Calculate confidence for a pattern
    pub fn calculate(&self, pattern: &Pattern) -> Confidence {
        let pattern_type = Self::get_pattern_type(pattern);
        let base = self
            .base_confidences
            .get(&pattern_type)
            .copied()
            .unwrap_or(50);

        // Apply modifiers
        let mut final_confidence = base as i16;

        for modifier in &self.modifiers {
            if self.check_modifier_condition(&modifier.condition, pattern) {
                final_confidence += modifier.adjustment as i16;
            }
        }

        // Clamp to valid range
        Confidence(final_confidence.clamp(0, 100) as u8)
    }

    /// Get pattern type for a pattern
    fn get_pattern_type(pattern: &Pattern) -> PatternType {
        match pattern {
            Pattern::ForLoop { .. } => PatternType::ForLoop,
            Pattern::WhileLoop { .. } => PatternType::WhileLoop,
            Pattern::DoWhileLoop { .. } => PatternType::DoWhileLoop,
            Pattern::FunctionCall { .. } => PatternType::FunctionCall,
            Pattern::SwitchCase { .. } => PatternType::SwitchCase,
            Pattern::IfElse { .. } => PatternType::IfElse,
            Pattern::ArrayAccess { .. } => PatternType::ArrayAccess,
            Pattern::FieldAccess { .. } => PatternType::FieldAccess,
            Pattern::StringOperation { .. } => PatternType::StringOperation,
            Pattern::MemoryAllocation { .. } => PatternType::MemoryAllocation,
            Pattern::MemoryDeallocation { .. } => PatternType::MemoryDeallocation,
            Pattern::Expression { .. } => PatternType::Expression,
            Pattern::LowIR { .. } => PatternType::LowIR,
        }
    }

    /// Check if a modifier condition applies to a pattern
    fn check_modifier_condition(&self, condition: &ModifierCondition, pattern: &Pattern) -> bool {
        match condition {
            ModifierCondition::HasCharacteristic(char) => {
                self.pattern_has_characteristic(pattern, char)
            }
            ModifierCondition::MatchesIdiom(idiom) => self.pattern_matches_idiom(pattern, idiom),
            ModifierCondition::MinEvidence(min) => self.count_evidence(pattern) >= *min,
            ModifierCondition::ComplexityRange { min, max } => {
                let complexity = self.calculate_complexity(pattern);
                complexity >= *min && complexity <= *max
            }
            ModifierCondition::SubPatternsConfident(min_conf) => {
                self.check_subpattern_confidence(pattern, *min_conf)
            }
        }
    }

    /// Check if pattern has a specific characteristic
    fn pattern_has_characteristic(&self, pattern: &Pattern, characteristic: &str) -> bool {
        match (pattern, characteristic) {
            (
                Pattern::ForLoop {
                    init: Some(_),
                    increment: Some(_),
                    ..
                },
                "complete_for_loop",
            ) => true,
            (Pattern::ForLoop { init: None, .. }, "while_style_for") => true,
            (
                Pattern::FunctionCall {
                    target: FunctionRef::Library { .. },
                    ..
                },
                "library_call",
            ) => true,
            (
                Pattern::FunctionCall {
                    target: FunctionRef::Indirect(_),
                    ..
                },
                "indirect_call",
            ) => true,
            (
                Pattern::SwitchCase {
                    default: Some(_), ..
                },
                "has_default",
            ) => true,
            (
                Pattern::IfElse {
                    else_branch: Some(_),
                    ..
                },
                "has_else",
            ) => true,
            _ => false,
        }
    }

    /// Check if pattern matches a known idiom
    fn pattern_matches_idiom(&self, pattern: &Pattern, idiom: &str) -> bool {
        match (pattern, idiom) {
            // Common C idioms
            (Pattern::ForLoop { .. }, "counting_loop") => {
                // TODO: Check if it's a simple counting loop
                false
            }
            (Pattern::WhileLoop { .. }, "linked_list_traversal") => {
                // TODO: Check if it's traversing a linked list
                false
            }
            (Pattern::IfElse { .. }, "null_check") => {
                // TODO: Check if condition is a null pointer check
                false
            }
            _ => false,
        }
    }

    /// Count supporting evidence for a pattern
    fn count_evidence(&self, pattern: &Pattern) -> usize {
        match pattern {
            Pattern::ForLoop {
                init, increment, ..
            } => {
                let mut count = 1; // Base evidence
                if init.is_some() {
                    count += 1;
                }
                if increment.is_some() {
                    count += 1;
                }
                count
            }
            Pattern::FunctionCall {
                arguments,
                return_value,
                ..
            } => {
                let mut count = 1;
                count += arguments.len();
                if return_value.is_some() {
                    count += 1;
                }
                count
            }
            Pattern::SwitchCase { cases, default, .. } => {
                let mut count = cases.len();
                if default.is_some() {
                    count += 1;
                }
                count
            }
            _ => 1,
        }
    }

    /// Calculate pattern complexity
    fn calculate_complexity(&self, pattern: &Pattern) -> usize {
        match pattern {
            Pattern::ForLoop { .. } => 10, // TODO: Calculate based on body
            Pattern::WhileLoop { .. } => 8,
            Pattern::DoWhileLoop { .. } => 8,
            Pattern::FunctionCall { arguments, .. } => 3 + arguments.len(),
            Pattern::SwitchCase { cases, .. } => 5 + cases.len() * 2,
            Pattern::IfElse { else_branch, .. } => {
                if else_branch.is_some() {
                    4
                } else {
                    2
                }
            }
            Pattern::ArrayAccess { .. } => 2,
            Pattern::FieldAccess { .. } => 1,
            Pattern::StringOperation { .. } => 3,
            Pattern::MemoryAllocation { .. } => 2,
            Pattern::MemoryDeallocation { .. } => 2,
            Pattern::Expression { operands, .. } => operands.len(),
            Pattern::LowIR { instructions, .. } => instructions.len(),
        }
    }

    /// Check if sub-patterns meet minimum confidence
    fn check_subpattern_confidence(&self, pattern: &Pattern, min_confidence: Confidence) -> bool {
        let sub_confidences = match pattern {
            Pattern::ForLoop { confidence, .. }
            | Pattern::WhileLoop { confidence, .. }
            | Pattern::DoWhileLoop { confidence, .. }
            | Pattern::FunctionCall { confidence, .. }
            | Pattern::SwitchCase { confidence, .. }
            | Pattern::IfElse { confidence, .. }
            | Pattern::ArrayAccess { confidence, .. }
            | Pattern::FieldAccess { confidence, .. }
            | Pattern::StringOperation { confidence, .. }
            | Pattern::MemoryAllocation { confidence, .. }
            | Pattern::MemoryDeallocation { confidence, .. }
            | Pattern::Expression { confidence, .. }
            | Pattern::LowIR { confidence, .. } => vec![*confidence],
        };

        sub_confidences.iter().all(|&conf| conf >= min_confidence)
    }

    /// Default base confidences for pattern types
    fn default_base_confidences() -> BTreeMap<PatternType, u8> {
        let mut confidences = BTreeMap::new();

        // High confidence patterns
        confidences.insert(PatternType::LowIR, 100); // Direct IR is certain
        confidences.insert(PatternType::Expression, 90); // Simple expressions

        // Medium-high confidence patterns
        confidences.insert(PatternType::FunctionCall, 85);
        confidences.insert(PatternType::IfElse, 85);
        confidences.insert(PatternType::ArrayAccess, 80);
        confidences.insert(PatternType::FieldAccess, 80);

        // Medium confidence patterns
        confidences.insert(PatternType::ForLoop, 75);
        confidences.insert(PatternType::WhileLoop, 75);
        confidences.insert(PatternType::DoWhileLoop, 70);
        confidences.insert(PatternType::SwitchCase, 70);

        // Lower confidence patterns (need more evidence)
        confidences.insert(PatternType::StringOperation, 65);
        confidences.insert(PatternType::MemoryAllocation, 65);
        confidences.insert(PatternType::MemoryDeallocation, 65);

        confidences
    }

    /// Default confidence modifiers
    fn default_modifiers() -> Vec<ConfidenceModifier> {
        vec![
            // Loop modifiers
            ConfidenceModifier {
                name: "Complete for loop".to_string(),
                condition: ModifierCondition::HasCharacteristic("complete_for_loop".to_string()),
                adjustment: 10,
            },
            ConfidenceModifier {
                name: "While-style for loop".to_string(),
                condition: ModifierCondition::HasCharacteristic("while_style_for".to_string()),
                adjustment: -5,
            },
            // Function call modifiers
            ConfidenceModifier {
                name: "Library function call".to_string(),
                condition: ModifierCondition::HasCharacteristic("library_call".to_string()),
                adjustment: 15,
            },
            ConfidenceModifier {
                name: "Indirect call".to_string(),
                condition: ModifierCondition::HasCharacteristic("indirect_call".to_string()),
                adjustment: -10,
            },
            // Control flow modifiers
            ConfidenceModifier {
                name: "Has else branch".to_string(),
                condition: ModifierCondition::HasCharacteristic("has_else".to_string()),
                adjustment: 5,
            },
            ConfidenceModifier {
                name: "Switch has default".to_string(),
                condition: ModifierCondition::HasCharacteristic("has_default".to_string()),
                adjustment: 5,
            },
            // Evidence-based modifiers
            ConfidenceModifier {
                name: "Strong evidence".to_string(),
                condition: ModifierCondition::MinEvidence(5),
                adjustment: 10,
            },
            ConfidenceModifier {
                name: "Weak evidence".to_string(),
                condition: ModifierCondition::MinEvidence(2),
                adjustment: -10,
            },
            // Complexity modifiers
            ConfidenceModifier {
                name: "Simple pattern".to_string(),
                condition: ModifierCondition::ComplexityRange { min: 1, max: 5 },
                adjustment: 5,
            },
            ConfidenceModifier {
                name: "Complex pattern".to_string(),
                condition: ModifierCondition::ComplexityRange { min: 20, max: 1000 },
                adjustment: -10,
            },
        ]
    }
}

/// Pattern confidence analyzer for cross-pattern validation
pub struct PatternValidator {
    confidence_calc: ConfidenceCalculator,
}

impl Default for PatternValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternValidator {
    pub fn new() -> Self {
        Self {
            confidence_calc: ConfidenceCalculator::default(),
        }
    }

    /// Validate patterns in a function and adjust confidences
    pub fn validate_patterns(&self, function: &mut Function) {
        // Collect pattern relationships
        let relationships = self.analyze_pattern_relationships(function);

        // Adjust confidences based on relationships
        for (pattern_ref, adjustment) in relationships {
            if let Some(pattern) = function.patterns.get_mut(pattern_ref) {
                self.adjust_pattern_confidence(pattern, adjustment);
            }
        }
    }

    /// Analyze relationships between patterns
    fn analyze_pattern_relationships(&self, _function: &Function) -> Vec<(PatternRef, i8)> {
        // TODO: Implement relationship analysis
        // Examples:
        // - If a loop contains known library calls, increase confidence
        // - If control flow is inconsistent, decrease confidence
        // - If patterns form known idioms together, increase confidence

        Vec::new()
    }

    /// Adjust pattern confidence
    fn adjust_pattern_confidence(&self, pattern: &mut Pattern, adjustment: i8) {
        let current = match pattern {
            Pattern::ForLoop { confidence, .. }
            | Pattern::WhileLoop { confidence, .. }
            | Pattern::DoWhileLoop { confidence, .. }
            | Pattern::FunctionCall { confidence, .. }
            | Pattern::SwitchCase { confidence, .. }
            | Pattern::IfElse { confidence, .. }
            | Pattern::ArrayAccess { confidence, .. }
            | Pattern::FieldAccess { confidence, .. }
            | Pattern::StringOperation { confidence, .. }
            | Pattern::MemoryAllocation { confidence, .. }
            | Pattern::MemoryDeallocation { confidence, .. }
            | Pattern::Expression { confidence, .. }
            | Pattern::LowIR { confidence, .. } => confidence,
        };

        let new_value = (current.0 as i16 + adjustment as i16).clamp(0, 100) as u8;
        *current = Confidence(new_value);
    }
}

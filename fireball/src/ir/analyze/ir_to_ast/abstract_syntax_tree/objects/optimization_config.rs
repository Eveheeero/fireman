use crate::ir::analyze::ir_to_ast::abstract_syntax_tree::pattern_matching::AstPattern;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstOptimizationConfig {
    pub ir_analyzation: bool,
    pub collapse_unused_varaible: bool,
    pub pattern_matching: Vec<AstPattern>, // TODO, should we set this vec?
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessedOptimization {
    IrAnalyzation,
    CollapseUnusedVariables,
    PatternMatching,
}
impl AstOptimizationConfig {
    pub const DEFAULT: Self = Self {
        ir_analyzation: true,
        collapse_unused_varaible: true,
        pattern_matching: AstPattern::ALL,
    };
    pub const ALL: Self = Self {
        ir_analyzation: true,
        collapse_unused_varaible: true,
        pattern_matching: AstPattern::ALL,
    };
    pub const NONE: Self = Self {
        ir_analyzation: false,
        collapse_unused_varaible: false,
        pattern_matching: Vec::new(),
    };

    pub fn ir_analyzation(mut self, value: bool) -> Self {
        self.ir_analyzation = value;
        self
    }
    pub fn collapse_unused_varaible(mut self, value: bool) -> Self {
        self.collapse_unused_varaible = value;
        self
    }
    pub fn pattern_matching(mut self, value: Vec<AstPattern>) -> Self {
        self.pattern_matching = value;
        self
    }
}
impl Default for AstOptimizationConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

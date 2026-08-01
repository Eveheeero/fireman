use crate::abstract_syntax_tree::pattern_matching::AstPattern;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstOptimizationConfig {
    pub ir_analyzation: bool,
    pub parameter_analyzation: bool,
    pub constant_folding: bool,
    pub collapse_unused_variable: bool,
    pub pattern_matching: Vec<AstPattern>,
    pub max_pass_iterations: usize,
    /// When true, use the original embedded Rust implementations instead of
    /// `.fb` pattern files for migrated passes
    pub use_embedded_passes: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstOptimizationKind {
    IrAnalyzation,
    ParameterAnalyzation,
    ConstantFolding,
    CollapseUnusedVariables,
    PatternMatching(Box<AstPattern>),
}
impl AstOptimizationConfig {
    pub fn all() -> Self {
        Self {
            ir_analyzation: true,
            parameter_analyzation: true,
            constant_folding: true,
            collapse_unused_variable: true,
            pattern_matching: AstPattern::predefined_patterns(),
            max_pass_iterations: 3,
            use_embedded_passes: false,
        }
    }
    pub fn none() -> Self {
        Self {
            ir_analyzation: false,
            parameter_analyzation: false,
            constant_folding: false,
            collapse_unused_variable: false,
            pattern_matching: Vec::new(),
            max_pass_iterations: 1,
            use_embedded_passes: false,
        }
    }

    pub fn ir_analyzation(mut self, value: bool) -> Self {
        self.ir_analyzation = value;
        self
    }
    pub fn parameter_analyzation(mut self, value: bool) -> Self {
        self.parameter_analyzation = value;
        self
    }
    pub fn constant_folding(mut self, value: bool) -> Self {
        self.constant_folding = value;
        self
    }
    pub fn collapse_unused_variable(mut self, value: bool) -> Self {
        self.collapse_unused_variable = value;
        self
    }
    pub fn pattern_matching(mut self, value: Vec<AstPattern>) -> Self {
        self.pattern_matching = value;
        self
    }
    pub fn max_pass_iterations(mut self, value: usize) -> Self {
        self.max_pass_iterations = value;
        self
    }
    pub fn use_embedded_passes(mut self, value: bool) -> Self {
        self.use_embedded_passes = value;
        self
    }
}
impl Default for AstOptimizationConfig {
    fn default() -> Self {
        Self {
            ir_analyzation: true,
            parameter_analyzation: true,
            constant_folding: true,
            collapse_unused_variable: true,
            pattern_matching: AstPattern::predefined_patterns(),
            max_pass_iterations: 3,
            use_embedded_passes: false,
        }
    }
}

impl From<AstOptimizationKind> for AstOptimizationConfig {
    fn from(value: AstOptimizationKind) -> Self {
        let mut n = AstOptimizationConfig::none();
        match value {
            AstOptimizationKind::IrAnalyzation => {
                n.ir_analyzation = true;
            }
            AstOptimizationKind::ParameterAnalyzation => {
                n.parameter_analyzation = true;
            }
            AstOptimizationKind::ConstantFolding => {
                n.constant_folding = true;
            }
            AstOptimizationKind::CollapseUnusedVariables => {
                n.collapse_unused_variable = true;
            }
            AstOptimizationKind::PatternMatching(p) => {
                n.pattern_matching = Vec::from([*p]);
            }
        }
        n
    }
}

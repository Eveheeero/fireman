use crate::abstract_syntax_tree::pattern_matching::AstPattern;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstOptimizationConfig {
    pub ir_analyzation: bool,
    pub parameter_analyzation: bool,
    pub call_argument_analyzation: bool,
    pub constant_folding: bool,
    pub control_flow_cleanup: bool,
    pub collapse_unused_varaible: bool,
    pub pattern_matching_enabled: bool,
    pub pattern_matching: Vec<AstPattern>,
    pub loop_analyzation: bool,
    pub copy_propagation: bool,
    pub expression_inlining: bool,
    pub max_pass_iterations: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessedOptimization {
    IrAnalyzation,
    ParameterAnalyzation,
    CallArgumentAnalyzation,
    ConstantFolding,
    ControlFlowCleanup,
    CollapseUnusedVariables,
    PatternMatching,
    LoopAnalyzation,
    CopyPropagation,
    ExpressionInlining,
}
impl AstOptimizationConfig {
    pub const DEFAULT: Self = Self {
        ir_analyzation: true,
        parameter_analyzation: true,
        call_argument_analyzation: true,
        constant_folding: true,
        control_flow_cleanup: true,
        collapse_unused_varaible: true,
        pattern_matching_enabled: true,
        pattern_matching: AstPattern::ALL,
        loop_analyzation: true,
        copy_propagation: true,
        expression_inlining: true,
        max_pass_iterations: 3,
    };
    pub const ALL: Self = Self {
        ir_analyzation: true,
        parameter_analyzation: true,
        call_argument_analyzation: true,
        constant_folding: true,
        control_flow_cleanup: true,
        collapse_unused_varaible: true,
        pattern_matching_enabled: true,
        pattern_matching: AstPattern::ALL,
        loop_analyzation: true,
        copy_propagation: true,
        expression_inlining: true,
        max_pass_iterations: 3,
    };
    pub const NONE: Self = Self {
        ir_analyzation: false,
        parameter_analyzation: false,
        call_argument_analyzation: false,
        constant_folding: false,
        control_flow_cleanup: false,
        collapse_unused_varaible: false,
        pattern_matching_enabled: false,
        pattern_matching: Vec::new(),
        loop_analyzation: false,
        copy_propagation: false,
        expression_inlining: false,
        max_pass_iterations: 1,
    };

    pub fn ir_analyzation(mut self, value: bool) -> Self {
        self.ir_analyzation = value;
        self
    }
    pub fn parameter_analyzation(mut self, value: bool) -> Self {
        self.parameter_analyzation = value;
        self
    }
    pub fn call_argument_analyzation(mut self, value: bool) -> Self {
        self.call_argument_analyzation = value;
        self
    }
    pub fn constant_folding(mut self, value: bool) -> Self {
        self.constant_folding = value;
        self
    }
    pub fn control_flow_cleanup(mut self, value: bool) -> Self {
        self.control_flow_cleanup = value;
        self
    }
    pub fn collapse_unused_varaible(mut self, value: bool) -> Self {
        self.collapse_unused_varaible = value;
        self
    }
    pub fn collapse_unused_variable(self, value: bool) -> Self {
        self.collapse_unused_varaible(value)
    }
    pub fn pattern_matching_enabled(mut self, value: bool) -> Self {
        self.pattern_matching_enabled = value;
        self
    }
    pub fn pattern_matching(mut self, value: Vec<AstPattern>) -> Self {
        self.pattern_matching = value;
        self
    }
    pub fn loop_analyzation(mut self, value: bool) -> Self {
        self.loop_analyzation = value;
        self
    }
    pub fn copy_propagation(mut self, value: bool) -> Self {
        self.copy_propagation = value;
        self
    }
    pub fn expression_inlining(mut self, value: bool) -> Self {
        self.expression_inlining = value;
        self
    }
    pub fn max_pass_iterations(mut self, value: usize) -> Self {
        self.max_pass_iterations = value;
        self
    }
}
impl Default for AstOptimizationConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

use crate::abstract_syntax_tree::pattern_matching::AstPattern;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstOptimizationConfig {
    pub ir_analyzation: bool,
    pub parameter_analyzation: bool,
    pub call_argument_analyzation: bool,
    pub constant_folding: bool,
    pub control_flow_cleanup: bool,
    pub collapse_unused_varaible: bool,
    pub dead_store_elimination: bool,
    pub pattern_matching_enabled: bool,
    pub pattern_matching: Vec<AstPattern>,
    pub loop_analyzation: bool,
    pub copy_propagation: bool,
    pub expression_inlining: bool,
    pub operator_canonicalization: bool,
    pub magic_division_recovery: bool,
    pub identity_simplification: bool,
    pub bit_trick_recognition: bool,
    pub cast_minimization: bool,
    pub ternary_recovery: bool,
    pub boolean_recovery: bool,
    pub assertion_recovery: bool,
    pub do_while_recovery: bool,
    pub clamp_recovery: bool,
    pub loop_cleanup: bool,
    pub if_conversion_reversal: bool,
    pub switch_reconstruction: bool,
    pub lifetime_scoping: bool,
    pub signedness_inference: bool,
    pub name_recovery: bool,
    pub auto_comment: bool,
    pub early_return_normalization: bool,
    pub max_pass_iterations: usize,
    /// When true, use the original embedded Rust implementations instead of
    /// `.fb` pattern files for migrated passes
    pub use_embedded_passes: bool,
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
    TernaryRecovery,
    IfConversionReversal,
    BooleanRecovery,
    SwitchReconstruction,
    OperatorCanonicalization,
    CommonSubexpressionElimination,
    BitTrickRecognition,
    CastMinimization,
    MagicDivisionRecovery,
    GotoContainment,
    InductionVariableAnalysis,
    TemporaryElimination,
    LifetimeScoping,
    VariableCoalescing,
    SignednessInference,
    NameRecovery,
    AutoComment,
    EarlyReturnNormalization,
    AssertionRecovery,
    DoWhileRecovery,
}
impl AstOptimizationConfig {
    pub const DEFAULT: Self = Self {
        ir_analyzation: true,
        parameter_analyzation: true,
        call_argument_analyzation: true,
        constant_folding: true,
        control_flow_cleanup: true,
        collapse_unused_varaible: true,
        dead_store_elimination: true,
        pattern_matching_enabled: true,
        pattern_matching: AstPattern::ALL,
        loop_analyzation: true,
        copy_propagation: true,
        expression_inlining: true,
        operator_canonicalization: true,
        magic_division_recovery: true,
        identity_simplification: true,
        bit_trick_recognition: true,
        cast_minimization: true,
        ternary_recovery: true,
        boolean_recovery: true,
        assertion_recovery: true,
        do_while_recovery: true,
        clamp_recovery: true,
        loop_cleanup: true,
        if_conversion_reversal: true,
        switch_reconstruction: true,
        lifetime_scoping: true,
        signedness_inference: true,
        name_recovery: true,
        auto_comment: true,
        early_return_normalization: true,
        max_pass_iterations: 3,
        use_embedded_passes: false,
    };
    pub const ALL: Self = Self {
        ir_analyzation: true,
        parameter_analyzation: true,
        call_argument_analyzation: true,
        constant_folding: true,
        control_flow_cleanup: true,
        collapse_unused_varaible: true,
        dead_store_elimination: true,
        pattern_matching_enabled: true,
        pattern_matching: AstPattern::ALL,
        loop_analyzation: true,
        copy_propagation: true,
        expression_inlining: true,
        operator_canonicalization: true,
        magic_division_recovery: true,
        identity_simplification: true,
        bit_trick_recognition: true,
        cast_minimization: true,
        ternary_recovery: true,
        boolean_recovery: true,
        assertion_recovery: true,
        do_while_recovery: true,
        clamp_recovery: true,
        loop_cleanup: true,
        if_conversion_reversal: true,
        switch_reconstruction: true,
        lifetime_scoping: true,
        signedness_inference: true,
        name_recovery: true,
        auto_comment: true,
        early_return_normalization: true,
        max_pass_iterations: 3,
        use_embedded_passes: false,
    };
    pub const NONE: Self = Self {
        ir_analyzation: false,
        parameter_analyzation: false,
        call_argument_analyzation: false,
        constant_folding: false,
        control_flow_cleanup: false,
        collapse_unused_varaible: false,
        dead_store_elimination: false,
        pattern_matching_enabled: false,
        pattern_matching: Vec::new(),
        loop_analyzation: false,
        copy_propagation: false,
        expression_inlining: false,
        operator_canonicalization: false,
        magic_division_recovery: false,
        identity_simplification: false,
        bit_trick_recognition: false,
        cast_minimization: false,
        ternary_recovery: false,
        boolean_recovery: false,
        assertion_recovery: false,
        do_while_recovery: false,
        clamp_recovery: false,
        loop_cleanup: false,
        if_conversion_reversal: false,
        switch_reconstruction: false,
        lifetime_scoping: false,
        signedness_inference: false,
        name_recovery: false,
        auto_comment: false,
        early_return_normalization: false,
        max_pass_iterations: 1,
        use_embedded_passes: false,
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
        if value {
            self.operator_canonicalization = true;
            self.magic_division_recovery = true;
            self.identity_simplification = true;
            self.bit_trick_recognition = true;
            self.cast_minimization = true;
            self.if_conversion_reversal = true;
        }
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
    pub fn dead_store_elimination(mut self, value: bool) -> Self {
        self.dead_store_elimination = value;
        self
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
    pub fn operator_canonicalization(mut self, value: bool) -> Self {
        self.operator_canonicalization = value;
        self
    }
    pub fn magic_division_recovery(mut self, value: bool) -> Self {
        self.magic_division_recovery = value;
        self
    }
    pub fn identity_simplification(mut self, value: bool) -> Self {
        self.identity_simplification = value;
        self
    }
    pub fn bit_trick_recognition(mut self, value: bool) -> Self {
        self.bit_trick_recognition = value;
        self
    }
    pub fn cast_minimization(mut self, value: bool) -> Self {
        self.cast_minimization = value;
        self
    }
    pub fn ternary_recovery(mut self, value: bool) -> Self {
        self.ternary_recovery = value;
        if value {
            self.assertion_recovery = true;
            self.do_while_recovery = true;
            self.clamp_recovery = true;
            self.loop_cleanup = true;
        }
        self
    }
    pub fn boolean_recovery(mut self, value: bool) -> Self {
        self.boolean_recovery = value;
        self
    }
    pub fn assertion_recovery(mut self, value: bool) -> Self {
        self.assertion_recovery = value;
        self
    }
    pub fn do_while_recovery(mut self, value: bool) -> Self {
        self.do_while_recovery = value;
        self
    }
    pub fn clamp_recovery(mut self, value: bool) -> Self {
        self.clamp_recovery = value;
        self
    }
    pub fn loop_cleanup(mut self, value: bool) -> Self {
        self.loop_cleanup = value;
        self
    }
    pub fn if_conversion_reversal(mut self, value: bool) -> Self {
        self.if_conversion_reversal = value;
        self
    }
    pub fn switch_reconstruction(mut self, value: bool) -> Self {
        self.switch_reconstruction = value;
        self
    }
    pub fn lifetime_scoping(mut self, value: bool) -> Self {
        self.lifetime_scoping = value;
        self
    }
    pub fn signedness_inference(mut self, value: bool) -> Self {
        self.signedness_inference = value;
        self
    }
    pub fn name_recovery(mut self, value: bool) -> Self {
        self.name_recovery = value;
        self
    }
    pub fn auto_comment(mut self, value: bool) -> Self {
        self.auto_comment = value;
        self
    }
    pub fn early_return_normalization(mut self, value: bool) -> Self {
        self.early_return_normalization = value;
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
        Self::DEFAULT
    }
}

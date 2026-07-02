use fireball::abstract_syntax_tree::{AstOptimizationConfig, pattern_matching::AstPattern};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct KnownSectionData {
    pub start_address: u64,
    pub end_address: Option<u64>,
    pub analyzed: bool,
}

#[derive(Clone)]
pub struct KnownSection {
    pub selected: bool,
    pub data: KnownSectionData,
}

#[derive(Clone)]
pub struct AstLine {
    pub row: usize,
    pub data: String,
}

#[derive(Clone)]
pub struct DecompileResult {
    pub ast: Vec<AstLine>,
    pub ast_sync_message: Option<String>,
}

/// Decompile output bundled with the underlying Ast for incremental optimization.
pub struct DecompileWithAst {
    pub ast: fireball::abstract_syntax_tree::Ast,
    pub result: DecompileResult,
}

/// Request to optimize an existing Ast with a single-pass config.
pub struct OptimizeAstRequest {
    pub ast: fireball::abstract_syntax_tree::Ast,
    pub settings: DefaultOptimizationSetting,
    pub script_paths: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct DisassembleRequest {
    pub start_addresses: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefaultOptimizationSetting {
    pub ir_analyzation: bool,
    pub parameter_analyzation: bool,
    pub call_argument_analyzation: bool,
    pub constant_folding: bool,
    pub control_flow_cleanup: bool,
    pub collapse_unused_varaible: bool,
    pub dead_store_elimination: bool,
    pub loop_analyzation: bool,
    pub copy_propagation: bool,
    pub expression_inlining: bool,
    pub ternary_recovery: bool,
    pub boolean_recovery: bool,
    pub switch_reconstruction: bool,
    pub lifetime_scoping: bool,
    pub signedness_inference: bool,
    pub name_recovery: bool,
    pub early_return_normalization: bool,
    pub operator_canonicalization: bool,
    pub magic_division_recovery: bool,
    pub identity_simplification: bool,
    pub bit_trick_recognition: bool,
    pub cast_minimization: bool,
    pub assertion_recovery: bool,
    pub do_while_recovery: bool,
    pub clamp_recovery: bool,
    pub loop_cleanup: bool,
    pub if_conversion_reversal: bool,
    pub anti_debug_ast_suppression: bool,
    pub logging_suppression: bool,
    pub static_guard_suppression: bool,
    pub security_scaffold_suppression: bool,
    pub max_pass_iterations: usize,
    pub use_embedded_passes: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserOptimizationScript {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OptimizationConfig {
    pub default_optimizations: DefaultOptimizationSetting,
    pub user_scripts: Vec<UserOptimizationScript>,
}

impl Default for DefaultOptimizationSetting {
    fn default() -> Self {
        let none = AstOptimizationConfig::none();
        Self {
            ir_analyzation: none.ir_analyzation,
            parameter_analyzation: none.parameter_analyzation,
            call_argument_analyzation: none.call_argument_analyzation,
            constant_folding: none.constant_folding,
            control_flow_cleanup: none.control_flow_cleanup,
            collapse_unused_varaible: none.collapse_unused_varaible,
            dead_store_elimination: none.dead_store_elimination,
            loop_analyzation: none.loop_analyzation,
            copy_propagation: none.copy_propagation,
            expression_inlining: none.expression_inlining,
            ternary_recovery: none.ternary_recovery,
            boolean_recovery: none.boolean_recovery,
            switch_reconstruction: none.switch_reconstruction,
            lifetime_scoping: none.lifetime_scoping,
            signedness_inference: none.signedness_inference,
            name_recovery: none.name_recovery,
            early_return_normalization: none.early_return_normalization,
            operator_canonicalization: none.operator_canonicalization,
            magic_division_recovery: none.magic_division_recovery,
            identity_simplification: none.identity_simplification,
            bit_trick_recognition: none.bit_trick_recognition,
            cast_minimization: none.cast_minimization,
            assertion_recovery: none.assertion_recovery,
            do_while_recovery: none.do_while_recovery,
            clamp_recovery: none.clamp_recovery,
            loop_cleanup: none.loop_cleanup,
            if_conversion_reversal: none.if_conversion_reversal,
            anti_debug_ast_suppression: none.anti_debug_ast_suppression,
            logging_suppression: none.logging_suppression,
            static_guard_suppression: none.static_guard_suppression,
            security_scaffold_suppression: none.security_scaffold_suppression,
            max_pass_iterations: none.max_pass_iterations,
            use_embedded_passes: none.use_embedded_passes,
        }
    }
}

pub fn build_optimization_config(
    args: OptimizationConfig,
) -> Result<AstOptimizationConfig, String> {
    let script_paths: Vec<_> = args
        .user_scripts
        .iter()
        .map(|preset| preset.path.clone())
        .collect();
    let settings = args.default_optimizations;
    let mut config = AstOptimizationConfig {
        ir_analyzation: settings.ir_analyzation,
        parameter_analyzation: settings.parameter_analyzation,
        call_argument_analyzation: settings.call_argument_analyzation,
        constant_folding: settings.constant_folding,
        control_flow_cleanup: settings.control_flow_cleanup,
        collapse_unused_varaible: settings.collapse_unused_varaible,
        dead_store_elimination: settings.dead_store_elimination,
        pattern_matching_enabled: true,
        pattern_matching: Vec::new(),
        loop_analyzation: settings.loop_analyzation,
        copy_propagation: settings.copy_propagation,
        expression_inlining: settings.expression_inlining,
        ternary_recovery: settings.ternary_recovery,
        boolean_recovery: settings.boolean_recovery,
        switch_reconstruction: settings.switch_reconstruction,
        lifetime_scoping: settings.lifetime_scoping,
        signedness_inference: settings.signedness_inference,
        name_recovery: settings.name_recovery,
        early_return_normalization: settings.early_return_normalization,
        operator_canonicalization: settings.operator_canonicalization,
        magic_division_recovery: settings.magic_division_recovery,
        identity_simplification: settings.identity_simplification,
        bit_trick_recognition: settings.bit_trick_recognition,
        cast_minimization: settings.cast_minimization,
        assertion_recovery: settings.assertion_recovery,
        do_while_recovery: settings.do_while_recovery,
        clamp_recovery: settings.clamp_recovery,
        loop_cleanup: settings.loop_cleanup,
        if_conversion_reversal: settings.if_conversion_reversal,
        anti_debug_ast_suppression: settings.anti_debug_ast_suppression,
        logging_suppression: settings.logging_suppression,
        static_guard_suppression: settings.static_guard_suppression,
        security_scaffold_suppression: settings.security_scaffold_suppression,
        max_pass_iterations: settings.max_pass_iterations,
        use_embedded_passes: settings.use_embedded_passes,
    };

    let mut patterns = AstPattern::predefined_patterns();
    patterns.extend(
        script_paths
            .iter()
            .filter(|path| !path.trim().is_empty())
            .cloned()
            .map(AstPattern::from_file),
    );

    config.pattern_matching = patterns;
    Ok(config)
}

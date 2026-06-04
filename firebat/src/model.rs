use fireball::abstract_syntax_tree::{Ast, AstOptimizationConfig, pattern_matching::AstPattern};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownSectionData {
    pub start_address: u64,
    pub end_address: Option<u64>,
    pub analyzed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownSection {
    pub selected: bool,
    pub data: KnownSectionData,
}

#[derive(Clone, Debug)]
pub struct Assembly {
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct Ir {
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct AstLine {
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct DecompileResult {
    pub assembly: Vec<Assembly>,
    pub ir: Vec<Ir>,
    pub ast: Vec<AstLine>,
    pub ast_object: Option<Arc<Ast>>, // Actual AST for tree rendering
}

#[derive(Clone, Debug)]
pub struct DecompileRequest {
    pub start_addresses: Vec<u64>,
    pub settings: OptimizationSettings,
    pub script_paths: Vec<String>,
    pub buffer_script: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationSettings {
    pub ir_analyzation: bool,
    pub parameter_analyzation: bool,
    pub call_argument_analyzation: bool,
    pub constant_folding: bool,
    pub control_flow_cleanup: bool,
    pub collapse_unused_varaible: bool,
    pub dead_store_elimination: bool,
    pub pattern_matching_enabled: bool,
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

impl OptimizationSettings {
    pub const fn none() -> Self {
        Self {
            ir_analyzation: false,
            parameter_analyzation: false,
            call_argument_analyzation: false,
            constant_folding: false,
            control_flow_cleanup: false,
            collapse_unused_varaible: false,
            dead_store_elimination: false,
            pattern_matching_enabled: false,
            loop_analyzation: false,
            copy_propagation: false,
            expression_inlining: false,
            ternary_recovery: false,
            boolean_recovery: false,
            switch_reconstruction: false,
            lifetime_scoping: false,
            signedness_inference: false,
            name_recovery: false,
            early_return_normalization: false,
            operator_canonicalization: false,
            magic_division_recovery: false,
            identity_simplification: false,
            bit_trick_recognition: false,
            cast_minimization: false,
            assertion_recovery: false,
            do_while_recovery: false,
            clamp_recovery: false,
            loop_cleanup: false,
            if_conversion_reversal: false,
            anti_debug_ast_suppression: false,
            logging_suppression: false,
            static_guard_suppression: false,
            security_scaffold_suppression: false,
            max_pass_iterations: 1,
            use_embedded_passes: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationScriptPreset {
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub applied_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationStore {
    pub draft_settings: OptimizationSettings,
    pub applied_settings: OptimizationSettings,
    pub script_presets: Vec<OptimizationScriptPreset>,
    pub editor_buffer: String,
    pub editor_path: Option<String>,
    pub applied_buffer_script: Option<String>,
    #[serde(default)]
    pub fb_script_enabled: bool,
    #[serde(default)]
    pub applied_fb_script_enabled: bool,
}

impl OptimizationStore {
    pub fn normalize(&mut self) {
        if self
            .applied_buffer_script
            .as_deref()
            .is_some_and(|script| !script.is_empty())
        {
            self.applied_fb_script_enabled = true;
        }
        if self.editor_buffer.is_empty() {
            self.editor_buffer = self.applied_buffer_script.clone().unwrap_or_default();
        }
        if self.applied_fb_script_enabled && !self.fb_script_enabled {
            self.fb_script_enabled = true;
        }
        if self.fb_script_enabled {
            self.draft_settings.pattern_matching_enabled = true;
        }
        if self.applied_fb_script_enabled {
            self.applied_settings.pattern_matching_enabled = true;
        }
    }
}

/// Request to optimize an existing AST with a given config.
pub struct OptimizeAstRequest {
    pub ast: fireball::abstract_syntax_tree::Ast,
    pub settings: OptimizationSettings,
    pub script_paths: Vec<String>,
    pub buffer_script: Option<String>,
}

/// Lightweight result carrying the optimized AST and its text lines.
pub struct OptimizeAstResult {
    pub ast: Arc<fireball::abstract_syntax_tree::Ast>,
    pub ast_lines: Vec<AstLine>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersistedViewport {
    pub camera_offset_x: f32,
    pub camera_offset_y: f32,
    pub zoom: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersistedNode {
    pub id: u64,
    pub kind: String,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedConnection {
    pub from: u64,
    #[serde(default)]
    pub from_port: usize,
    pub to: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphPreset {
    pub schema_version: u32,
    pub viewport: PersistedViewport,
    pub nodes: Vec<PersistedNode>,
    pub connections: Vec<PersistedConnection>,
    pub known_sections: Vec<KnownSection>,
    pub analyze_target_address: String,
}

pub fn build_optimization_config(
    settings: &OptimizationSettings,
    script_paths: &[String],
    buffer_script: Option<&str>,
) -> Result<AstOptimizationConfig, String> {
    let defaults = AstOptimizationConfig::default();
    let mut config = AstOptimizationConfig {
        ir_analyzation: settings.ir_analyzation,
        parameter_analyzation: settings.parameter_analyzation,
        call_argument_analyzation: settings.call_argument_analyzation,
        constant_folding: settings.constant_folding,
        control_flow_cleanup: settings.control_flow_cleanup,
        collapse_unused_varaible: settings.collapse_unused_varaible,
        dead_store_elimination: settings.dead_store_elimination,
        pattern_matching_enabled: settings.pattern_matching_enabled,
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

    if !config.pattern_matching_enabled {
        config.pattern_matching = defaults.pattern_matching;
        return Ok(config);
    }

    let mut patterns = AstPattern::predefined_patterns();
    patterns.extend(
        script_paths
            .iter()
            .filter(|path| !path.trim().is_empty())
            .cloned()
            .map(AstPattern::from_file),
    );

    if let Some(source) = buffer_script
        .map(str::trim)
        .filter(|source| !source.is_empty())
    {
        patterns.push(AstPattern::new("firebat-buffer", source));
    }

    config.pattern_matching = patterns;
    Ok(config)
}

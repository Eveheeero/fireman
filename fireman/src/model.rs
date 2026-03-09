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
pub struct Assembly {
    pub index: usize,
    pub parents_start_address: u64,
    pub data: String,
}

#[derive(Clone)]
pub struct Ir {
    pub parents_assembly_index: usize,
    pub data: String,
}

#[derive(Clone)]
pub struct AstLine {
    pub row: usize,
    pub data: String,
}

#[derive(Clone)]
pub struct DecompileResult {
    pub assembly: Vec<Assembly>,
    pub ir: Vec<Ir>,
    pub ast: Vec<AstLine>,
    pub ast_sync_message: Option<String>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorLayer {
    Assembly,
    Ir,
    Ast,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EditPosition {
    Replace,
    Before,
    After,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EditorTarget {
    pub layer: EditorLayer,
    pub row: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct PatchOperation {
    pub layer: EditorLayer,
    pub position: EditPosition,
    pub target: String,
    pub text: String,
}

#[derive(Clone, Debug)]
pub struct AssemblyEditorDraft {
    pub raw_text: String,
    pub mnemonic: String,
    pub operands: String,
    pub status_message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct IrEditorDraft {
    pub raw_text: String,
    pub opcode: String,
    pub detail: String,
    pub position: EditPosition,
    pub status_message: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AstEditorDraft {
    pub raw_text: String,
    pub position: EditPosition,
    pub status_message: Option<String>,
}

#[derive(Clone, Debug)]
pub enum EditorDraft {
    Assembly(AssemblyEditorDraft),
    Ir(IrEditorDraft),
    Ast(AstEditorDraft),
}

#[derive(Clone, Debug)]
pub struct EditRequest {
    pub layer: EditorLayer,
    pub row: usize,
    pub position: EditPosition,
    pub text: String,
}

#[derive(Clone)]
pub struct AppliedEditResult {
    pub result: DecompileResult,
    pub selected_target: EditorTarget,
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
    pub auto_comment: bool,
    pub early_return_normalization: bool,
    pub max_pass_iterations: usize,
    pub use_embedded_passes: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptimizationScriptPreset {
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub applied_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OptimizationStore {
    pub draft_settings: OptimizationSettings,
    pub applied_settings: OptimizationSettings,
    pub script_presets: Vec<OptimizationScriptPreset>,
    pub editor_buffer: String,
    pub editor_path: Option<String>,
    pub applied_buffer_script: Option<String>,
}

impl Default for OptimizationSettings {
    fn default() -> Self {
        let defaults = AstOptimizationConfig::default();
        Self {
            ir_analyzation: defaults.ir_analyzation,
            parameter_analyzation: defaults.parameter_analyzation,
            call_argument_analyzation: defaults.call_argument_analyzation,
            constant_folding: defaults.constant_folding,
            control_flow_cleanup: defaults.control_flow_cleanup,
            collapse_unused_varaible: defaults.collapse_unused_varaible,
            dead_store_elimination: defaults.dead_store_elimination,
            pattern_matching_enabled: defaults.pattern_matching_enabled,
            loop_analyzation: defaults.loop_analyzation,
            copy_propagation: defaults.copy_propagation,
            expression_inlining: defaults.expression_inlining,
            ternary_recovery: defaults.ternary_recovery,
            boolean_recovery: defaults.boolean_recovery,
            switch_reconstruction: defaults.switch_reconstruction,
            lifetime_scoping: defaults.lifetime_scoping,
            signedness_inference: defaults.signedness_inference,
            name_recovery: defaults.name_recovery,
            auto_comment: defaults.auto_comment,
            early_return_normalization: defaults.early_return_normalization,
            max_pass_iterations: defaults.max_pass_iterations,
            use_embedded_passes: defaults.use_embedded_passes,
        }
    }
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
        auto_comment: settings.auto_comment,
        early_return_normalization: settings.early_return_normalization,
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

impl EditPosition {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Replace => "Replace",
            Self::Before => "Insert Before",
            Self::After => "Insert After",
        }
    }
}

impl AssemblyEditorDraft {
    pub fn from_display_text(text: &str) -> Self {
        let normalized = strip_assembly_address(text);
        let (mnemonic, operands) = split_head_tail(normalized);
        Self {
            raw_text: text.to_string(),
            mnemonic,
            operands,
            status_message: None,
        }
    }

    pub fn compose_line(&self) -> String {
        compose_head_tail(&self.mnemonic, &self.operands)
    }
}

impl IrEditorDraft {
    pub fn from_text(text: &str) -> Self {
        let (opcode, detail) = split_head_tail(text);
        Self {
            raw_text: text.to_string(),
            opcode,
            detail,
            position: EditPosition::Replace,
            status_message: None,
        }
    }

    pub fn compose_line(&self) -> String {
        compose_head_tail(&self.opcode, &self.detail)
    }
}

impl AstEditorDraft {
    pub fn from_text(text: &str) -> Self {
        Self {
            raw_text: text.to_string(),
            position: EditPosition::Replace,
            status_message: None,
        }
    }
}

fn strip_assembly_address(text: &str) -> &str {
    let trimmed = text.trim();
    let Some((head, tail)) = trimmed.split_once(char::is_whitespace) else {
        return trimmed;
    };
    if head.starts_with("0x")
        && head.len() > 2
        && head[2..].chars().all(|ch| ch.is_ascii_hexdigit())
    {
        tail.trim()
    } else {
        trimmed
    }
}

fn split_head_tail(text: &str) -> (String, String) {
    let trimmed = text.trim();
    let Some((head, tail)) = trimmed.split_once(char::is_whitespace) else {
        return (trimmed.to_string(), String::new());
    };
    (head.trim().to_string(), tail.trim().to_string())
}

fn compose_head_tail(head: &str, tail: &str) -> String {
    let head = head.trim();
    let tail = tail.trim();
    if tail.is_empty() {
        head.to_string()
    } else {
        format!("{head} {tail}")
    }
}

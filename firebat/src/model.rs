use eframe::egui::Color32;
use fireball::abstract_syntax_tree::{AstOptimizationConfig, pattern_matching::AstPattern};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct KnownSectionData {
    pub(crate) start_address: u64,
    pub(crate) end_address: Option<u64>,
    pub(crate) analyzed: bool,
}

#[derive(Clone)]
pub(crate) struct KnownSection {
    pub(crate) selected: bool,
    pub(crate) data: KnownSectionData,
}

#[derive(Clone)]
pub(crate) struct Assembly {
    pub(crate) index: usize,
    pub(crate) parents_start_address: u64,
    pub(crate) data: String,
}

#[derive(Clone)]
pub(crate) struct Ir {
    pub(crate) parents_assembly_index: usize,
    pub(crate) data: String,
}

#[derive(Clone)]
pub(crate) struct AstLine {
    pub(crate) row: usize,
    pub(crate) data: String,
}

#[derive(Clone)]
pub(crate) struct DecompileResult {
    pub(crate) assembly: Vec<Assembly>,
    pub(crate) ir: Vec<Ir>,
    pub(crate) ast: Vec<AstLine>,
    pub(crate) ast_sync_message: Option<String>,
}

#[derive(Clone)]
pub(crate) struct DecompileResultView {
    pub(crate) colors: HashMap<usize, Color32>,
    pub(crate) assembly_parent_by_index: HashMap<usize, u64>,
    pub(crate) data: DecompileResult,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EditorLayer {
    Assembly,
    Ir,
    Ast,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum EditPosition {
    Replace,
    Before,
    After,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct EditorTarget {
    pub(crate) layer: EditorLayer,
    pub(crate) row: usize,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct PatchOperation {
    pub(crate) layer: EditorLayer,
    pub(crate) position: EditPosition,
    pub(crate) target: String,
    pub(crate) text: String,
}

#[derive(Clone, Debug)]
pub(crate) struct AssemblyEditorDraft {
    pub(crate) raw_text: String,
    pub(crate) mnemonic: String,
    pub(crate) operands: String,
    pub(crate) status_message: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct IrEditorDraft {
    pub(crate) raw_text: String,
    pub(crate) opcode: String,
    pub(crate) detail: String,
    pub(crate) position: EditPosition,
    pub(crate) status_message: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct AstEditorDraft {
    pub(crate) raw_text: String,
    pub(crate) position: EditPosition,
    pub(crate) status_message: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) enum EditorDraft {
    Assembly(AssemblyEditorDraft),
    Ir(IrEditorDraft),
    Ast(AstEditorDraft),
}

#[derive(Clone, Debug)]
pub(crate) struct EditRequest {
    pub(crate) layer: EditorLayer,
    pub(crate) row: usize,
    pub(crate) position: EditPosition,
    pub(crate) text: String,
}

#[derive(Clone)]
pub(crate) struct AppliedEditResult {
    pub(crate) result: DecompileResult,
    pub(crate) selected_target: EditorTarget,
}

#[derive(Clone, Debug)]
pub(crate) struct DecompileRequest {
    pub(crate) start_addresses: Vec<u64>,
    pub(crate) settings: OptimizationSettings,
    pub(crate) script_paths: Vec<String>,
    pub(crate) buffer_script: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OptimizationSettings {
    pub(crate) ir_analyzation: bool,
    pub(crate) parameter_analyzation: bool,
    pub(crate) call_argument_analyzation: bool,
    pub(crate) constant_folding: bool,
    pub(crate) control_flow_cleanup: bool,
    pub(crate) collapse_unused_varaible: bool,
    pub(crate) dead_store_elimination: bool,
    pub(crate) pattern_matching_enabled: bool,
    pub(crate) loop_analyzation: bool,
    pub(crate) copy_propagation: bool,
    pub(crate) expression_inlining: bool,
    pub(crate) ternary_recovery: bool,
    pub(crate) boolean_recovery: bool,
    pub(crate) switch_reconstruction: bool,
    pub(crate) lifetime_scoping: bool,
    pub(crate) signedness_inference: bool,
    pub(crate) name_recovery: bool,
    pub(crate) auto_comment: bool,
    pub(crate) early_return_normalization: bool,
    pub(crate) max_pass_iterations: usize,
    pub(crate) use_embedded_passes: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OptimizationScriptPreset {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) enabled: bool,
    pub(crate) applied_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub(crate) struct OptimizationStore {
    pub(crate) draft_settings: OptimizationSettings,
    pub(crate) applied_settings: OptimizationSettings,
    pub(crate) script_presets: Vec<OptimizationScriptPreset>,
    pub(crate) editor_buffer: String,
    pub(crate) editor_path: Option<String>,
    pub(crate) applied_buffer_script: Option<String>,
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

pub(crate) fn build_optimization_config(
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
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Replace => "Replace",
            Self::Before => "Insert Before",
            Self::After => "Insert After",
        }
    }
}

impl AssemblyEditorDraft {
    pub(crate) fn from_display_text(text: &str) -> Self {
        let normalized = strip_assembly_address(text);
        let (mnemonic, operands) = split_head_tail(normalized);
        Self {
            raw_text: text.to_string(),
            mnemonic,
            operands,
            status_message: None,
        }
    }

    pub(crate) fn compose_line(&self) -> String {
        compose_head_tail(&self.mnemonic, &self.operands)
    }
}

impl IrEditorDraft {
    pub(crate) fn from_text(text: &str) -> Self {
        let (opcode, detail) = split_head_tail(text);
        Self {
            raw_text: text.to_string(),
            opcode,
            detail,
            position: EditPosition::Replace,
            status_message: None,
        }
    }

    pub(crate) fn compose_line(&self) -> String {
        compose_head_tail(&self.opcode, &self.detail)
    }
}

impl AstEditorDraft {
    pub(crate) fn from_text(text: &str) -> Self {
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

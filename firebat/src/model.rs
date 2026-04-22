use eframe::egui::Color32;
use fireball::abstract_syntax_tree::{
    Ast, AstNodeEdit, AstNodePath, AstOptimizationConfig, pattern_matching::AstPattern,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

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

#[derive(Clone, Debug)]
pub struct Assembly {
    pub index: usize,
    pub parents_start_address: u64,
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct Ir {
    pub parents_assembly_index: usize,
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct AstLine {
    pub row: usize,
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct DecompileResult {
    pub assembly: Vec<Assembly>,
    pub ir: Vec<Ir>,
    pub ast: Vec<AstLine>,
    pub ast_object: Option<Arc<Ast>>, // Actual AST for tree rendering
    pub ast_sync_message: Option<String>,
}

#[derive(Clone)]
pub struct DecompileResultView {
    pub colors: HashMap<usize, Color32>,
    pub assembly_parent_by_index: HashMap<usize, u64>,
    pub data: DecompileResult,
    pub ast: Option<Arc<Ast>>, // Reference to AST object for rendering
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
pub struct AstNodeEditorDraft {
    pub path: AstNodePath,
    pub edit_type: AstNodeEditType,
    pub draft_data: AstNodeDraftData,
    pub status_message: Option<String>,
}

#[derive(Clone, Debug)]
pub enum AstNodeEditType {
    Variable,
    Literal,
    UnaryOperator,
    BinaryOperator,
    Statement,
    Function,
}

#[derive(Clone, Debug)]
pub enum AstNodeDraftData {
    Variable {
        current_name: String,
        current_type: String,
        new_name: String,
        new_type: String,
    },
    Literal {
        current_value: String,
        current_type: String,
        new_value: String,
    },
    UnaryOperator {
        current_op: String,
        new_op: String,
        operand: String,
    },
    BinaryOperator {
        current_op: String,
        new_op: String,
        left: String,
        right: String,
    },
    Statement {
        statement_type: String,
        replacement: String,
    },
    Function {
        current_name: String,
        current_return_type: String,
        new_name: String,
        new_return_type: String,
    },
}

#[derive(Clone, Debug)]
pub enum EditorDraft {
    Assembly(AssemblyEditorDraft),
    Ir(IrEditorDraft),
    Ast(AstEditorDraft),
    AstNode(AstNodeEditorDraft),
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
            use_embedded_passes: false,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OptimizationStore {
    pub draft_settings: OptimizationSettings,
    pub applied_settings: OptimizationSettings,
    pub script_presets: Vec<OptimizationScriptPreset>,
    pub editor_buffer: String,
    pub editor_path: Option<String>,
    pub applied_buffer_script: Option<String>,
    #[serde(default)]
    pub fb_script_enabled: bool,
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
            early_return_normalization: defaults.early_return_normalization,
            operator_canonicalization: defaults.operator_canonicalization,
            magic_division_recovery: defaults.magic_division_recovery,
            identity_simplification: defaults.identity_simplification,
            bit_trick_recognition: defaults.bit_trick_recognition,
            cast_minimization: defaults.cast_minimization,
            assertion_recovery: defaults.assertion_recovery,
            do_while_recovery: defaults.do_while_recovery,
            clamp_recovery: defaults.clamp_recovery,
            loop_cleanup: defaults.loop_cleanup,
            if_conversion_reversal: defaults.if_conversion_reversal,
            anti_debug_ast_suppression: defaults.anti_debug_ast_suppression,
            logging_suppression: defaults.logging_suppression,
            static_guard_suppression: defaults.static_guard_suppression,
            security_scaffold_suppression: defaults.security_scaffold_suppression,
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

impl AstNodeEditorDraft {
    pub fn to_edit(&self) -> Option<AstNodeEdit> {
        match &self.draft_data {
            AstNodeDraftData::Variable {
                new_name,
                new_type,
                current_name,
                current_type,
            } => {
                if new_name != current_name {
                    return Some(AstNodeEdit::RenameVariable {
                        path: self.path.clone(),
                        new_name: new_name.clone(),
                    });
                }
                if new_type != current_type {
                    return Some(AstNodeEdit::ChangeVariableType {
                        path: self.path.clone(),
                        new_type: new_type.clone(),
                    });
                }
                None
            }
            AstNodeDraftData::Literal {
                new_value,
                current_value,
                ..
            } => {
                if new_value != current_value {
                    Some(AstNodeEdit::ChangeLiteral {
                        path: self.path.clone(),
                        new_value: new_value.clone(),
                    })
                } else {
                    None
                }
            }
            AstNodeDraftData::UnaryOperator {
                new_op, current_op, ..
            } => {
                if new_op != current_op {
                    Some(AstNodeEdit::ChangeUnaryOperator {
                        path: self.path.clone(),
                        new_op: new_op.clone(),
                    })
                } else {
                    None
                }
            }
            AstNodeDraftData::BinaryOperator {
                new_op, current_op, ..
            } => {
                if new_op != current_op {
                    Some(AstNodeEdit::ChangeBinaryOperator {
                        path: self.path.clone(),
                        new_op: new_op.clone(),
                    })
                } else {
                    None
                }
            }
            AstNodeDraftData::Statement { replacement, .. } => {
                Some(AstNodeEdit::ReplaceStatement {
                    path: self.path.clone(),
                    replacement: replacement.clone(),
                })
            }
            AstNodeDraftData::Function {
                new_name,
                new_return_type,
                current_name,
                current_return_type,
            } => {
                if new_name != current_name {
                    return Some(AstNodeEdit::RenameVariable {
                        path: self.path.clone(),
                        new_name: new_name.clone(),
                    });
                }
                if new_return_type != current_return_type {
                    return Some(AstNodeEdit::ChangeVariableType {
                        path: self.path.clone(),
                        new_type: new_return_type.clone(),
                    });
                }
                None
            }
        }
    }

    pub fn validate(&self) -> Option<String> {
        match &self.draft_data {
            AstNodeDraftData::Variable { new_name, .. } => {
                if new_name.is_empty() {
                    return Some("Variable name cannot be empty".to_string());
                }
                None
            }
            AstNodeDraftData::Literal { new_value, .. } => {
                if new_value.is_empty() {
                    return Some("Literal value cannot be empty".to_string());
                }
                None
            }
            AstNodeDraftData::Statement { replacement, .. } => {
                if replacement.is_empty() {
                    return Some("Statement cannot be empty".to_string());
                }
                None
            }
            _ => None,
        }
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

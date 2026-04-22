use crate::{
    model::{DecompileResult, OptimizationSettings, OptimizationStore, build_optimization_config},
    node::{
        Node, NodeColors, NodeContext, NodeError, NodeId, NodePosition, NodeResponse, NodeType,
    },
    pipeline::PipelineData,
};
use egui::{Color32, Ui};
use fireball::abstract_syntax_tree::Ast;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Types of optimization passes available (kept for backward compat / presets).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OptimizationPass {
    IrAnalyzation,
    ConstantFolding,
    ControlFlowCleanup,
    CopyPropagation,
    DeadStoreElimination,
    ExpressionInlining,
    LoopAnalysis,
    ParameterAnalysis,
    CallArgumentAnalysis,
    PatternMatching(Vec<String>),
    BooleanRecovery,
    SwitchReconstruction,
    LifetimeScoping,
    SignednessInference,
    NameRecovery,
    EarlyReturnNormalization,
    CollapseUnusedVariable,
    TernaryRecovery,
    OperatorCanonicalization,
    MagicDivisionRecovery,
    IdentitySimplification,
    BitTrickRecognition,
    CastMinimization,
    AssertionRecovery,
    DoWhileRecovery,
    ClampRecovery,
    LoopCleanup,
    IfConversionReversal,
    AntiDebugAstSuppression,
    LoggingSuppression,
    StaticGuardSuppression,
    SecurityScaffoldSuppression,
}

impl OptimizationPass {
    pub fn name(&self) -> &'static str {
        match self {
            Self::IrAnalyzation => "IR Analyzation",
            Self::ConstantFolding => "Constant Folding",
            Self::ControlFlowCleanup => "Control Flow Cleanup",
            Self::CopyPropagation => "Copy Propagation",
            Self::DeadStoreElimination => "Dead Store Elimination",
            Self::ExpressionInlining => "Expression Inlining",
            Self::LoopAnalysis => "Loop Analysis",
            Self::ParameterAnalysis => "Parameter Analysis",
            Self::CallArgumentAnalysis => "Call Argument Analysis",
            Self::PatternMatching(_) => "Pattern Matching",
            Self::BooleanRecovery => "Boolean Recovery",
            Self::SwitchReconstruction => "Switch Reconstruction",
            Self::LifetimeScoping => "Lifetime Scoping",
            Self::SignednessInference => "Signedness Inference",
            Self::NameRecovery => "Name Recovery",
            Self::EarlyReturnNormalization => "Early Return Normalization",
            Self::CollapseUnusedVariable => "Collapse Unused Variable",
            Self::TernaryRecovery => "Ternary Recovery",
            Self::OperatorCanonicalization => "Operator Canonicalization",
            Self::MagicDivisionRecovery => "Magic Division Recovery",
            Self::IdentitySimplification => "Identity Simplification",
            Self::BitTrickRecognition => "Bit Trick Recognition",
            Self::CastMinimization => "Cast Minimization",
            Self::AssertionRecovery => "Assertion Recovery",
            Self::DoWhileRecovery => "Do-While Recovery",
            Self::ClampRecovery => "Clamp Recovery",
            Self::LoopCleanup => "Loop Cleanup",
            Self::IfConversionReversal => "If-Conversion Reversal",
            Self::AntiDebugAstSuppression => "Anti-Debug AST Suppression",
            Self::LoggingSuppression => "Logging Suppression",
            Self::StaticGuardSuppression => "Static Guard Suppression",
            Self::SecurityScaffoldSuppression => "Security Scaffold Suppression",
        }
    }

    pub fn to_settings(&self) -> OptimizationSettings {
        let mut settings = OptimizationSettings::none();

        match self {
            Self::IrAnalyzation => settings.ir_analyzation = true,
            Self::ConstantFolding => settings.constant_folding = true,
            Self::ControlFlowCleanup => settings.control_flow_cleanup = true,
            Self::CopyPropagation => settings.copy_propagation = true,
            Self::DeadStoreElimination => settings.dead_store_elimination = true,
            Self::ExpressionInlining => settings.expression_inlining = true,
            Self::LoopAnalysis => settings.loop_analyzation = true,
            Self::ParameterAnalysis => settings.parameter_analyzation = true,
            Self::CallArgumentAnalysis => settings.call_argument_analyzation = true,
            Self::PatternMatching(_) => settings.pattern_matching_enabled = true,
            Self::BooleanRecovery => settings.boolean_recovery = true,
            Self::SwitchReconstruction => settings.switch_reconstruction = true,
            Self::LifetimeScoping => settings.lifetime_scoping = true,
            Self::SignednessInference => settings.signedness_inference = true,
            Self::NameRecovery => settings.name_recovery = true,
            Self::EarlyReturnNormalization => settings.early_return_normalization = true,
            Self::CollapseUnusedVariable => settings.collapse_unused_varaible = true,
            Self::TernaryRecovery => settings.ternary_recovery = true,
            Self::OperatorCanonicalization => settings.operator_canonicalization = true,
            Self::MagicDivisionRecovery => settings.magic_division_recovery = true,
            Self::IdentitySimplification => settings.identity_simplification = true,
            Self::BitTrickRecognition => settings.bit_trick_recognition = true,
            Self::CastMinimization => settings.cast_minimization = true,
            Self::AssertionRecovery => settings.assertion_recovery = true,
            Self::DoWhileRecovery => settings.do_while_recovery = true,
            Self::ClampRecovery => settings.clamp_recovery = true,
            Self::LoopCleanup => settings.loop_cleanup = true,
            Self::IfConversionReversal => settings.if_conversion_reversal = true,
            Self::AntiDebugAstSuppression => settings.anti_debug_ast_suppression = true,
            Self::LoggingSuppression => settings.logging_suppression = true,
            Self::StaticGuardSuppression => settings.static_guard_suppression = true,
            Self::SecurityScaffoldSuppression => settings.security_scaffold_suppression = true,
        }

        settings
    }

    fn build_store(&self) -> OptimizationStore {
        let settings = self.to_settings();
        let fb_script_enabled = matches!(self, Self::PatternMatching(_));
        OptimizationStore {
            draft_settings: settings.clone(),
            applied_settings: settings,
            script_presets: Vec::new(),
            editor_buffer: String::new(),
            editor_path: None,
            applied_buffer_script: None,
            fb_script_enabled,
            applied_fb_script_enabled: fb_script_enabled,
        }
    }
}

/// Optimization node holding full OptimizationStore with cached output.
#[derive(Clone, Debug)]
pub struct OptNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    is_expanded: bool,
    pub store: OptimizationStore,
    /// Cached output after this node's optimization is applied.
    pub output_ast: Option<Arc<Ast>>,
    pub output: Option<DecompileResult>,
}

impl OptNode {
    pub fn new() -> Self {
        Self::for_pass(OptimizationPass::ConstantFolding)
    }

    pub fn for_pass(pass: OptimizationPass) -> Self {
        Self {
            id: NodeId::new(),
            name: pass.name().to_string(),
            position: NodePosition::default(),
            is_expanded: false,
            store: pass.build_store(),
            output_ast: None,
            output: None,
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = NodePosition::new(x, y);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn supports_pattern_editor(&self) -> bool {
        self.store.fb_script_enabled
            || self.store.applied_fb_script_enabled
            || self.store.draft_settings.pattern_matching_enabled
            || self.store.applied_settings.pattern_matching_enabled
    }

    pub fn has_pending_changes(&self) -> bool {
        self.store.editor_buffer != self.store.applied_buffer_script.clone().unwrap_or_default()
    }

    pub fn apply_changes(&mut self) {
        self.store.applied_buffer_script = self
            .supports_pattern_editor()
            .then_some(self.store.editor_buffer.clone());
    }

    pub fn reset_draft_changes(&mut self) {
        self.store.editor_buffer = self.store.applied_buffer_script.clone().unwrap_or_default();
    }
}

impl Default for OptNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for OptNode {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn id(&self) -> NodeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn node_type(&self) -> NodeType {
        NodeType::Opt
    }

    fn color(&self) -> Color32 {
        NodeColors::optimization()
    }

    fn position(&self) -> NodePosition {
        self.position
    }

    fn set_position(&mut self, pos: NodePosition) {
        self.position = pos;
    }

    fn is_expanded(&self) -> bool {
        self.is_expanded
    }

    fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    fn summary(&self) -> String {
        if self.output_ast.is_some() {
            "Cached output available".to_string()
        } else {
            "Runs this optimization pass once".to_string()
        }
    }

    fn process(&self, input: PipelineData) -> Result<PipelineData, NodeError> {
        if let Some(ref ast) = self.output_ast {
            Ok(PipelineData::Ast(ast.clone()))
        } else {
            match input {
                PipelineData::Ast(ast) => {
                    let has_active_work = self.store.applied_settings
                        != OptimizationSettings::none()
                        || (self.store.applied_fb_script_enabled
                            && self
                                .store
                                .applied_buffer_script
                                .as_deref()
                                .is_some_and(|script| !script.trim().is_empty()));
                    if !has_active_work {
                        return Ok(PipelineData::Ast(ast));
                    }

                    let buffer_script = self
                        .store
                        .applied_fb_script_enabled
                        .then_some(self.store.applied_buffer_script.as_deref())
                        .flatten();
                    let config =
                        build_optimization_config(&self.store.applied_settings, &[], buffer_script)
                            .map_err(NodeError::ProcessingError)?;
                    let function_ids: Vec<_> = ast.function_versions.keys().cloned().collect();
                    let optimized = ast
                        .as_ref()
                        .clone()
                        .optimize_functions(&function_ids, Some(config))
                        .map_err(|error| NodeError::ProcessingError(error.to_string()))?;
                    Ok(PipelineData::Ast(Arc::new(optimized)))
                }
                _ => Ok(input),
            }
        }
    }

    fn ui(&mut self, ui: &mut Ui, ctx: &NodeContext) -> NodeResponse {
        ui.small(self.summary());
        if ctx.is_selected && self.has_pending_changes() {
            ui.small("Pending changes");
        }

        NodeResponse::None
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "OptNode",
            "id": self.id.0.to_string(),
            "name": self.name,
            "position": {"x": self.position.x, "y": self.position.y},
            "is_expanded": self.is_expanded,
            "store": serde_json::to_value(&self.store).unwrap_or_default(),
        })
    }

    fn deserialize(&mut self, value: &serde_json::Value) {
        if let Some(pos) = value.get("position") {
            if let (Some(x), Some(y)) = (
                pos.get("x").and_then(|v| v.as_f64()),
                pos.get("y").and_then(|v| v.as_f64()),
            ) {
                self.position = NodePosition::new(x as f32, y as f32);
            }
        }
        if let Some(expanded) = value.get("is_expanded").and_then(|v| v.as_bool()) {
            self.is_expanded = expanded;
        }
        if let Some(name) = value.get("name").and_then(|v| v.as_str()) {
            self.name = name.to_string();
        }
        if let Some(store_val) = value.get("store") {
            if let Ok(store) = serde_json::from_value::<OptimizationStore>(store_val.clone()) {
                self.store = store;
                self.store.normalize();
            }
        }
        if self.store.editor_buffer.is_empty() {
            self.store.editor_buffer = self.store.applied_buffer_script.clone().unwrap_or_default();
        }
    }
}

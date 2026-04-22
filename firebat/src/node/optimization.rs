use crate::{
    model::{DecompileResult, OptimizationSettings, OptimizationStore},
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

    pub fn icon(&self) -> &'static str {
        match self {
            Self::ConstantFolding => "+",
            Self::ControlFlowCleanup => "~",
            Self::CopyPropagation => ">",
            Self::DeadStoreElimination => "x",
            Self::ExpressionInlining => "->",
            Self::LoopAnalysis => "@",
            Self::ParameterAnalysis => "*",
            Self::CallArgumentAnalysis => "*",
            Self::PatternMatching(_) => "#",
            Self::BooleanRecovery => "T",
            Self::SwitchReconstruction => "<>",
            Self::LifetimeScoping => "|",
            Self::SignednessInference => "+-",
            Self::NameRecovery => "#",
            Self::EarlyReturnNormalization => "<-",
            Self::CollapseUnusedVariable => "-",
            Self::TernaryRecovery => "?",
            Self::OperatorCanonicalization => "=",
            Self::MagicDivisionRecovery => "/",
            Self::IdentitySimplification => "1",
            Self::BitTrickRecognition => "&",
            Self::CastMinimization => "(",
            Self::AssertionRecovery => "!",
            Self::DoWhileRecovery => "d",
            Self::ClampRecovery => "c",
            Self::LoopCleanup => "~@",
            Self::IfConversionReversal => "<?",
            Self::AntiDebugAstSuppression => "%%",
            Self::LoggingSuppression => "L",
            Self::StaticGuardSuppression => "G",
            Self::SecurityScaffoldSuppression => "S",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "Constant Folding" => Self::ConstantFolding,
            "Control Flow Cleanup" => Self::ControlFlowCleanup,
            "Copy Propagation" => Self::CopyPropagation,
            "Dead Store Elimination" => Self::DeadStoreElimination,
            "Expression Inlining" => Self::ExpressionInlining,
            "Loop Analysis" => Self::LoopAnalysis,
            "Parameter Analysis" => Self::ParameterAnalysis,
            "Call Argument Analysis" => Self::CallArgumentAnalysis,
            "Pattern Matching" => Self::PatternMatching(Vec::new()),
            "Boolean Recovery" => Self::BooleanRecovery,
            "Switch Reconstruction" => Self::SwitchReconstruction,
            "Lifetime Scoping" => Self::LifetimeScoping,
            "Signedness Inference" => Self::SignednessInference,
            "Name Recovery" => Self::NameRecovery,
            "Early Return Normalization" => Self::EarlyReturnNormalization,
            "Collapse Unused Variable" => Self::CollapseUnusedVariable,
            "Ternary Recovery" => Self::TernaryRecovery,
            "Operator Canonicalization" => Self::OperatorCanonicalization,
            "Magic Division Recovery" => Self::MagicDivisionRecovery,
            "Identity Simplification" => Self::IdentitySimplification,
            "Bit Trick Recognition" => Self::BitTrickRecognition,
            "Cast Minimization" => Self::CastMinimization,
            "Assertion Recovery" => Self::AssertionRecovery,
            "Do-While Recovery" => Self::DoWhileRecovery,
            "Clamp Recovery" => Self::ClampRecovery,
            "Loop Cleanup" => Self::LoopCleanup,
            "If-Conversion Reversal" => Self::IfConversionReversal,
            "Anti-Debug AST Suppression" => Self::AntiDebugAstSuppression,
            "Logging Suppression" => Self::LoggingSuppression,
            "Static Guard Suppression" => Self::StaticGuardSuppression,
            "Security Scaffold Suppression" => Self::SecurityScaffoldSuppression,
            _ => return None,
        })
    }

    pub fn to_settings(&self) -> OptimizationSettings {
        let mut settings = OptimizationSettings::none();

        match self {
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

    fn from_settings(settings: &OptimizationSettings) -> Option<Self> {
        let candidates = [
            (settings.constant_folding, Self::ConstantFolding),
            (settings.control_flow_cleanup, Self::ControlFlowCleanup),
            (settings.copy_propagation, Self::CopyPropagation),
            (settings.dead_store_elimination, Self::DeadStoreElimination),
            (settings.expression_inlining, Self::ExpressionInlining),
            (settings.loop_analyzation, Self::LoopAnalysis),
            (settings.parameter_analyzation, Self::ParameterAnalysis),
            (
                settings.call_argument_analyzation,
                Self::CallArgumentAnalysis,
            ),
            (
                settings.pattern_matching_enabled,
                Self::PatternMatching(Vec::new()),
            ),
            (settings.boolean_recovery, Self::BooleanRecovery),
            (settings.switch_reconstruction, Self::SwitchReconstruction),
            (settings.lifetime_scoping, Self::LifetimeScoping),
            (settings.signedness_inference, Self::SignednessInference),
            (settings.name_recovery, Self::NameRecovery),
            (
                settings.early_return_normalization,
                Self::EarlyReturnNormalization,
            ),
            (
                settings.collapse_unused_varaible,
                Self::CollapseUnusedVariable,
            ),
            (settings.ternary_recovery, Self::TernaryRecovery),
            (
                settings.operator_canonicalization,
                Self::OperatorCanonicalization,
            ),
            (
                settings.magic_division_recovery,
                Self::MagicDivisionRecovery,
            ),
            (
                settings.identity_simplification,
                Self::IdentitySimplification,
            ),
            (settings.bit_trick_recognition, Self::BitTrickRecognition),
            (settings.cast_minimization, Self::CastMinimization),
            (settings.assertion_recovery, Self::AssertionRecovery),
            (settings.do_while_recovery, Self::DoWhileRecovery),
            (settings.clamp_recovery, Self::ClampRecovery),
            (settings.loop_cleanup, Self::LoopCleanup),
            (settings.if_conversion_reversal, Self::IfConversionReversal),
            (
                settings.anti_debug_ast_suppression,
                Self::AntiDebugAstSuppression,
            ),
            (settings.logging_suppression, Self::LoggingSuppression),
            (
                settings.static_guard_suppression,
                Self::StaticGuardSuppression,
            ),
            (
                settings.security_scaffold_suppression,
                Self::SecurityScaffoldSuppression,
            ),
        ];

        let mut enabled = candidates
            .into_iter()
            .filter_map(|(is_enabled, pass)| is_enabled.then_some(pass));
        let first = enabled.next()?;
        enabled.next().is_none().then_some(first)
    }

    fn build_store(&self) -> OptimizationStore {
        let settings = self.to_settings();
        OptimizationStore {
            draft_settings: settings.clone(),
            applied_settings: settings,
            script_presets: Vec::new(),
            editor_buffer: String::new(),
            editor_path: None,
            applied_buffer_script: None,
            fb_script_enabled: false,
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

    pub fn with_store(mut self, store: OptimizationStore) -> Self {
        self.store = store;
        if let Some(pass) = OptimizationPass::from_settings(&self.store.applied_settings)
            .or_else(|| OptimizationPass::from_settings(&self.store.draft_settings))
        {
            self.name = pass.name().to_string();
        }
        self
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
        // OptNode no longer processes synchronously; the async pipeline handles it.
        // Pass through whatever input we have, or return cached output if available.
        if let Some(ref ast) = self.output_ast {
            Ok(PipelineData::Ast(ast.clone()))
        } else {
            Ok(input)
        }
    }

    fn ui(&mut self, ui: &mut Ui, ctx: &NodeContext) -> NodeResponse {
        let mut response = NodeResponse::None;

        ui.horizontal(|ui| {
            if ctx.can_delete && ui.button("x").clicked() {
                response = NodeResponse::Deleted;
            }
        });

        ui.small(self.summary());

        response
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
            }
        }
        if value.get("name").is_none() {
            if let Some(pass) = OptimizationPass::from_settings(&self.store.applied_settings)
                .or_else(|| OptimizationPass::from_settings(&self.store.draft_settings))
            {
                self.name = pass.name().to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_pass_is_reflected_in_node_configuration() {
        let node = OptNode::for_pass(OptimizationPass::CopyPropagation);

        assert_eq!(node.name(), "Copy Propagation");
        assert!(node.store.applied_settings.copy_propagation);
        assert!(node.store.draft_settings.copy_propagation);
        assert!(!node.store.applied_settings.constant_folding);
        assert!(!node.store.applied_settings.dead_store_elimination);
    }
}

use crate::{
    model::{DecompileResult, OptimizationFocus, OptimizationStore},
    node::{
        Node, NodeColors, NodeContext, NodeError, NodeId, NodePosition, NodeResponse, NodeType,
    },
    pipeline::PipelineData,
};
use egui::{Color32, Ui};
use fireball::abstract_syntax_tree::{Ast, AstOptimizationConfig, AstPrintConfig};
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
}

/// Optimization node holding full OptimizationStore with cached output.
#[derive(Clone, Debug)]
pub struct OptNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    is_expanded: bool,
    pub store: OptimizationStore,
    pub focus: OptimizationFocus,
    pub setting_cursor: usize,
    pub script_cursor: usize,
    /// Cached output after this node's optimization is applied.
    pub output_ast: Option<Arc<Ast>>,
    pub output: Option<DecompileResult>,
}

impl OptNode {
    pub fn new() -> Self {
        Self {
            id: NodeId::new(),
            name: "Optimization".to_string(),
            position: NodePosition::default(),
            is_expanded: false,
            store: OptimizationStore::default(),
            focus: OptimizationFocus::Settings,
            setting_cursor: 0,
            script_cursor: 0,
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
            "[Opt (cached)]".to_string()
        } else {
            "[Opt (not run)]".to_string()
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
            ui.label(self.summary());
            if ctx.can_delete && ui.button("x").clicked() {
                response = NodeResponse::Deleted;
            }
        });

        if self.is_expanded {
            ui.separator();
            ui.label("Optimization settings are configured per-node.");
            ui.label(format!("Focus: {:?}", self.focus));
        }

        ui.horizontal(|ui| {
            if ui
                .button(if self.is_expanded { "^" } else { "v" })
                .clicked()
            {
                self.toggle_expanded();
                response = NodeResponse::ToggleExpanded;
            }
        });

        response
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "OptNode",
            "id": self.id.0.to_string(),
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
        if let Some(store_val) = value.get("store") {
            if let Ok(store) = serde_json::from_value::<OptimizationStore>(store_val.clone()) {
                self.store = store;
            }
        }
    }
}

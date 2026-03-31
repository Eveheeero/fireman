use crate::{
    node::{
        Node, NodeColors, NodeContext, NodeError, NodeId, NodePosition, NodeResponse, NodeType,
    },
    pipeline::PipelineData,
};
use egui::{Color32, Ui};
use fireball::abstract_syntax_tree::{Ast, AstOptimizationConfig, AstPrintConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Types of optimization passes available
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

/// Optimization node that stores before/after AST and applies a specific pass
#[derive(Clone, Debug)]
pub struct OptimizationNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    pass_type: OptimizationPass,
    is_enabled: bool,
    is_expanded: bool,
    before_ast: Option<Arc<Ast>>,
    after_ast: Option<Arc<Ast>>,
    before_code: String,
    after_code: String,
    pass_number: usize, // To distinguish multiple instances
}

impl OptimizationNode {
    pub fn new(pass_type: OptimizationPass, pass_number: usize) -> Self {
        let name = format!("{} (#{})", pass_type.name(), pass_number);

        Self {
            id: NodeId::new(),
            name,
            position: NodePosition::default(),
            pass_type,
            is_enabled: true,
            is_expanded: false,
            before_ast: None,
            after_ast: None,
            before_code: String::new(),
            after_code: String::new(),
            pass_number,
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = NodePosition::new(x, y);
        self
    }

    pub fn pass_type(&self) -> &OptimizationPass {
        &self.pass_type
    }

    pub fn pass_number(&self) -> usize {
        self.pass_number
    }

    fn apply_optimization(&self, ast: &Ast) -> Result<Ast, NodeError> {
        let config = self.build_config();

        match ast.optimize(Some(config)) {
            Ok(optimized) => Ok(optimized),
            Err(e) => Err(NodeError::ProcessingError(format!(
                "Optimization failed: {:?}",
                e
            ))),
        }
    }

    fn build_config(&self) -> AstOptimizationConfig {
        let mut config = AstOptimizationConfig::default();

        // Enable only the specific pass for this node
        config.ir_analyzation = false;
        config.parameter_analyzation = false;
        config.call_argument_analyzation = false;
        config.constant_folding = false;
        config.control_flow_cleanup = false;
        config.collapse_unused_varaible = false;
        config.dead_store_elimination = false;
        config.pattern_matching = Vec::new();
        config.loop_analyzation = false;
        config.copy_propagation = false;
        config.expression_inlining = false;
        config.ternary_recovery = false;
        config.boolean_recovery = false;
        config.switch_reconstruction = false;
        config.lifetime_scoping = false;
        config.signedness_inference = false;
        config.name_recovery = false;
        config.early_return_normalization = false;

        // Enable only this specific pass
        match self.pass_type {
            OptimizationPass::ConstantFolding => config.constant_folding = true,
            OptimizationPass::ControlFlowCleanup => config.control_flow_cleanup = true,
            OptimizationPass::CopyPropagation => config.copy_propagation = true,
            OptimizationPass::DeadStoreElimination => config.dead_store_elimination = true,
            OptimizationPass::ExpressionInlining => config.expression_inlining = true,
            OptimizationPass::LoopAnalysis => config.loop_analyzation = true,
            OptimizationPass::ParameterAnalysis => config.parameter_analyzation = true,
            OptimizationPass::CallArgumentAnalysis => config.call_argument_analyzation = true,
            OptimizationPass::PatternMatching(ref patterns) => {
                // Convert string paths to actual patterns
                // This will need to be implemented with actual pattern loading
                config.pattern_matching = Vec::new();
            }
            OptimizationPass::BooleanRecovery => config.boolean_recovery = true,
            OptimizationPass::SwitchReconstruction => config.switch_reconstruction = true,
            OptimizationPass::LifetimeScoping => config.lifetime_scoping = true,
            OptimizationPass::SignednessInference => config.signedness_inference = true,
            OptimizationPass::NameRecovery => config.name_recovery = true,
            OptimizationPass::EarlyReturnNormalization => config.early_return_normalization = true,
            OptimizationPass::CollapseUnusedVariable => config.collapse_unused_varaible = true,
            OptimizationPass::TernaryRecovery => config.ternary_recovery = true,
            OptimizationPass::OperatorCanonicalization => {
                config.operator_canonicalization = true
            }
            OptimizationPass::MagicDivisionRecovery => config.magic_division_recovery = true,
            OptimizationPass::IdentitySimplification => config.identity_simplification = true,
            OptimizationPass::BitTrickRecognition => config.bit_trick_recognition = true,
            OptimizationPass::CastMinimization => config.cast_minimization = true,
            OptimizationPass::AssertionRecovery => config.assertion_recovery = true,
            OptimizationPass::DoWhileRecovery => config.do_while_recovery = true,
            OptimizationPass::ClampRecovery => config.clamp_recovery = true,
            OptimizationPass::LoopCleanup => config.loop_cleanup = true,
            OptimizationPass::IfConversionReversal => config.if_conversion_reversal = true,
            OptimizationPass::AntiDebugAstSuppression => {
                config.anti_debug_ast_suppression = true
            }
            OptimizationPass::LoggingSuppression => config.logging_suppression = true,
            OptimizationPass::StaticGuardSuppression => config.static_guard_suppression = true,
            OptimizationPass::SecurityScaffoldSuppression => {
                config.security_scaffold_suppression = true
            }
        }

        config
    }

    fn format_ast(ast: &Ast) -> String {
        let config = AstPrintConfig::default();
        ast.print(Some(config))
    }
}

impl Node for OptimizationNode {
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
        NodeType::Optimization(self.pass_type.clone())
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

    fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    fn is_expanded(&self) -> bool {
        self.is_expanded
    }

    fn toggle_expanded(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    fn summary(&self) -> String {
        if let Some(ref after) = self.after_ast {
            let preview = self
                .after_code
                .lines()
                .find(|l| !l.trim().is_empty())
                .map(|l| l.trim().to_string())
                .unwrap_or_default();
            format!(
                "[{} {} - {}...]",
                self.pass_type.icon(),
                self.pass_type.name(),
                preview.chars().take(40).collect::<String>()
            )
        } else {
            format!(
                "[{} {} (not run)]",
                self.pass_type.icon(),
                self.pass_type.name()
            )
        }
    }

    fn process(&self, input: PipelineData) -> Result<PipelineData, NodeError> {
        if !self.is_enabled {
            return Ok(input);
        }

        match input {
            PipelineData::Ast(ast) => {
                // Apply optimization
                let after = self.apply_optimization(&ast)?;
                Ok(PipelineData::Ast(Arc::new(after)))
            }
            _ => Err(NodeError::InvalidInput {
                expected: "AST",
                got: input.type_name(),
            }),
        }
    }

    fn ui(&mut self, ui: &mut Ui, ctx: &NodeContext) -> NodeResponse {
        let mut response = NodeResponse::None;

        // Header
        ui.horizontal(|ui| {
            // Enable/disable checkbox
            let mut enabled = self.is_enabled;
            if ui.checkbox(&mut enabled, "").changed() {
                self.set_enabled(enabled);
                response = NodeResponse::ToggleEnabled;
            }

            ui.label(self.summary());

            if ctx.can_delete && ui.button("x").clicked() {
                response = NodeResponse::Deleted;
            }
        });

        // Expanded content showing before/after (populated when pipeline runs)
        if self.is_expanded {
            ui.separator();

            if self.before_code.is_empty() {
                ui.label("Run pipeline to see before/after comparison");
            } else {
                ui.strong("BEFORE:");
                egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(150.0)
                        .show(ui, |ui| {
                            ui.monospace(&self.before_code);
                        });
                });

                ui.add_space(8.0);
                ui.strong("AFTER:");
                egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(150.0)
                        .show(ui, |ui| {
                            ui.monospace(&self.after_code);
                        });
                });
            }
        }

        // Controls
        ui.horizontal(|ui| {
            if ui
                .button(if self.is_expanded { "^" } else { "v" })
                .clicked()
            {
                self.toggle_expanded();
                response = NodeResponse::ToggleExpanded;
            }

            ui.label(if self.is_enabled {
                "[x] Enabled"
            } else {
                "[ ] Disabled"
            });

            if !self.before_code.is_empty() {
                let before_lines = self.before_code.lines().count();
                let after_lines = self.after_code.lines().count();
                let diff = after_lines as i32 - before_lines as i32;
                let diff_str = if diff == 0 {
                    "(no change)".to_string()
                } else if diff > 0 {
                    format!("(+{} lines)", diff)
                } else {
                    format!("({} lines)", diff)
                };
                ui.label(diff_str);
            }
        });

        response
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "OptimizationNode",
            "id": self.id.0.to_string(),
            "position": {"x": self.position.x, "y": self.position.y},
            "pass_type": serde_json::to_string(&self.pass_type).unwrap_or_default(),
            "pass_number": self.pass_number,
            "is_enabled": self.is_enabled,
            "is_expanded": self.is_expanded,
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
        if let Some(enabled) = value.get("is_enabled").and_then(|v| v.as_bool()) {
            self.is_enabled = enabled;
        }
        if let Some(expanded) = value.get("is_expanded").and_then(|v| v.as_bool()) {
            self.is_expanded = expanded;
        }
        if let Some(number) = value.get("pass_number").and_then(|v| v.as_u64()) {
            self.pass_number = number as usize;
            self.name = format!("{} (#{})", self.pass_type.name(), self.pass_number);
        }
    }
}

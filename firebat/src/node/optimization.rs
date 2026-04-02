use crate::{
    model::{DecompileResult, OptimizationFocus, OptimizationSettings, OptimizationStore},
    node::{
        Node, NodeColors, NodeContext, NodeError, NodeId, NodePosition, NodeResponse, NodeType,
    },
    pipeline::PipelineData,
};
use egui::{Color32, RichText, ScrollArea, TextEdit, Ui};
use fireball::abstract_syntax_tree::Ast;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Copy)]
struct OptimizationField {
    label: &'static str,
    description: &'static str,
    get: fn(&OptimizationSettings) -> bool,
    set: fn(&mut OptimizationSettings, bool),
}

const OPTIMIZATION_FIELDS: &[OptimizationField] = &[
    OptimizationField {
        label: "IR analyzation",
        description: "Builds the IR analysis layer used by later AST passes",
        get: |s| s.ir_analyzation,
        set: |s, v| s.ir_analyzation = v,
    },
    OptimizationField {
        label: "Parameter analyzation",
        description: "Infers function parameters from recovered usage",
        get: |s| s.parameter_analyzation,
        set: |s, v| s.parameter_analyzation = v,
    },
    OptimizationField {
        label: "Call argument analyzation",
        description: "Propagates argument information into recovered calls",
        get: |s| s.call_argument_analyzation,
        set: |s, v| s.call_argument_analyzation = v,
    },
    OptimizationField {
        label: "Name recovery",
        description: "Recovers variable and helper names when possible",
        get: |s| s.name_recovery,
        set: |s, v| s.name_recovery = v,
    },
    OptimizationField {
        label: "Signedness inference",
        description: "Refines integer semantics from instruction behavior",
        get: |s| s.signedness_inference,
        set: |s, v| s.signedness_inference = v,
    },
    OptimizationField {
        label: "Constant folding",
        description: "Evaluates constant expressions during optimization",
        get: |s| s.constant_folding,
        set: |s, v| s.constant_folding = v,
    },
    OptimizationField {
        label: "Copy propagation",
        description: "Eliminates temporary copies when values can be forwarded",
        get: |s| s.copy_propagation,
        set: |s, v| s.copy_propagation = v,
    },
    OptimizationField {
        label: "Expression inlining",
        description: "Inlines short temporary expressions into their uses",
        get: |s| s.expression_inlining,
        set: |s, v| s.expression_inlining = v,
    },
    OptimizationField {
        label: "Dead store elimination",
        description: "Removes writes that never affect later behavior",
        get: |s| s.dead_store_elimination,
        set: |s, v| s.dead_store_elimination = v,
    },
    OptimizationField {
        label: "Collapse unused variable",
        description: "Drops redundant variables that do not survive analysis",
        get: |s| s.collapse_unused_varaible,
        set: |s, v| s.collapse_unused_varaible = v,
    },
    OptimizationField {
        label: "Lifetime scoping",
        description: "Shrinks recovered variable lifetimes around real usage",
        get: |s| s.lifetime_scoping,
        set: |s, v| s.lifetime_scoping = v,
    },
    OptimizationField {
        label: "Control flow cleanup",
        description: "Removes structural noise before higher-level recovery",
        get: |s| s.control_flow_cleanup,
        set: |s, v| s.control_flow_cleanup = v,
    },
    OptimizationField {
        label: "Loop analyzation",
        description: "Recovers loop constructs from CFG structure",
        get: |s| s.loop_analyzation,
        set: |s, v| s.loop_analyzation = v,
    },
    OptimizationField {
        label: "Ternary recovery",
        description: "Rebuilds ternary expressions from compact branches",
        get: |s| s.ternary_recovery,
        set: |s, v| s.ternary_recovery = v,
    },
    OptimizationField {
        label: "Boolean recovery",
        description: "Normalizes predicate-heavy code into boolean expressions",
        get: |s| s.boolean_recovery,
        set: |s, v| s.boolean_recovery = v,
    },
    OptimizationField {
        label: "Switch reconstruction",
        description: "Detects and prints switch-style control flow",
        get: |s| s.switch_reconstruction,
        set: |s, v| s.switch_reconstruction = v,
    },
    OptimizationField {
        label: "Early return normalization",
        description: "Prefers normalized early-return shapes in the AST",
        get: |s| s.early_return_normalization,
        set: |s, v| s.early_return_normalization = v,
    },
    OptimizationField {
        label: "Pattern matching",
        description: "Runs predefined and selected .fb pattern scripts",
        get: |s| s.pattern_matching_enabled,
        set: |s, v| s.pattern_matching_enabled = v,
    },
    OptimizationField {
        label: "Operator canonicalization",
        description: "Normalizes operator ordering for consistent comparison",
        get: |s| s.operator_canonicalization,
        set: |s, v| s.operator_canonicalization = v,
    },
    OptimizationField {
        label: "Magic division recovery",
        description: "Recovers division from magic-number multiplication patterns",
        get: |s| s.magic_division_recovery,
        set: |s, v| s.magic_division_recovery = v,
    },
    OptimizationField {
        label: "Identity simplification",
        description: "Simplifies identity operations like x+0, x*1",
        get: |s| s.identity_simplification,
        set: |s, v| s.identity_simplification = v,
    },
    OptimizationField {
        label: "Bit trick recognition",
        description: "Recognizes bit manipulation idioms",
        get: |s| s.bit_trick_recognition,
        set: |s, v| s.bit_trick_recognition = v,
    },
    OptimizationField {
        label: "Cast minimization",
        description: "Removes redundant type casts",
        get: |s| s.cast_minimization,
        set: |s, v| s.cast_minimization = v,
    },
    OptimizationField {
        label: "Assertion recovery",
        description: "Recovers assertion patterns from conditional aborts",
        get: |s| s.assertion_recovery,
        set: |s, v| s.assertion_recovery = v,
    },
    OptimizationField {
        label: "Do-while recovery",
        description: "Recovers do-while loops from CFG",
        get: |s| s.do_while_recovery,
        set: |s, v| s.do_while_recovery = v,
    },
    OptimizationField {
        label: "Clamp recovery",
        description: "Recovers clamp/min/max patterns",
        get: |s| s.clamp_recovery,
        set: |s, v| s.clamp_recovery = v,
    },
    OptimizationField {
        label: "Loop cleanup",
        description: "Cleans up loop structure after recovery",
        get: |s| s.loop_cleanup,
        set: |s, v| s.loop_cleanup = v,
    },
    OptimizationField {
        label: "If-conversion reversal",
        description: "Reverses compiler if-conversion optimizations",
        get: |s| s.if_conversion_reversal,
        set: |s, v| s.if_conversion_reversal = v,
    },
    OptimizationField {
        label: "Anti-debug AST suppression",
        description: "Suppresses anti-debug code patterns in output",
        get: |s| s.anti_debug_ast_suppression,
        set: |s, v| s.anti_debug_ast_suppression = v,
    },
    OptimizationField {
        label: "Logging suppression",
        description: "Suppresses logging boilerplate in output",
        get: |s| s.logging_suppression,
        set: |s, v| s.logging_suppression = v,
    },
    OptimizationField {
        label: "Static guard suppression",
        description: "Suppresses static guard patterns in output",
        get: |s| s.static_guard_suppression,
        set: |s, v| s.static_guard_suppression = v,
    },
    OptimizationField {
        label: "Security scaffold suppression",
        description: "Suppresses security scaffold patterns in output",
        get: |s| s.security_scaffold_suppression,
        set: |s, v| s.security_scaffold_suppression = v,
    },
];

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

    fn has_dirty_settings(&self) -> bool {
        self.store.draft_settings != self.store.applied_settings
    }

    fn has_dirty_script(&self) -> bool {
        let applied_script = self.store.applied_buffer_script.as_deref().unwrap_or("");
        let draft_script = self.store.editor_buffer.trim();
        let applied_enabled = self.store.applied_buffer_script.is_some();
        let draft_enabled = self.store.fb_script_enabled && !draft_script.is_empty();

        applied_enabled != draft_enabled || applied_script != draft_script
    }

    fn is_dirty(&self) -> bool {
        self.has_dirty_settings() || self.has_dirty_script()
    }

    fn apply_changes(&mut self) {
        self.store.applied_settings = self.store.draft_settings.clone();
        self.store.applied_buffer_script = if self.store.fb_script_enabled {
            let trimmed = self.store.editor_buffer.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        } else {
            None
        };
        self.output_ast = None;
        self.output = None;
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
        if self.is_dirty() {
            "[Opt (dirty)]".to_string()
        } else if self.output_ast.is_some() {
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
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.focus, OptimizationFocus::Settings, "Settings");
                ui.selectable_value(&mut self.focus, OptimizationFocus::Script, "Script");
                if self.is_dirty() {
                    ui.label(RichText::new("Unsaved changes").italics());
                }
            });

            ui.add_space(8.0);
            match self.focus {
                OptimizationFocus::Settings => {
                    ScrollArea::vertical().max_height(240.0).show(ui, |ui| {
                        for field in OPTIMIZATION_FIELDS {
                            let mut enabled = (field.get)(&self.store.draft_settings);
                            let applied = (field.get)(&self.store.applied_settings);
                            let dirty = enabled != applied;
                            let label = if dirty {
                                format!("{} *", field.label)
                            } else {
                                field.label.to_string()
                            };
                            if ui.checkbox(&mut enabled, label).changed() {
                                (field.set)(&mut self.store.draft_settings, enabled);
                            }
                            ui.small(field.description);
                            ui.add_space(4.0);
                        }
                    });

                    ui.separator();
                    ui.checkbox(
                        &mut self.store.draft_settings.use_embedded_passes,
                        "Use embedded passes",
                    );
                    ui.horizontal(|ui| {
                        ui.label("Max pass iterations");
                        ui.add(
                            egui::DragValue::new(
                                &mut self.store.draft_settings.max_pass_iterations,
                            )
                            .range(1..=64),
                        );
                    });
                }
                OptimizationFocus::Script => {
                    ui.checkbox(
                        &mut self.store.fb_script_enabled,
                        "Apply .fb script during optimization",
                    );
                    ui.label(
                        self.store
                            .editor_path
                            .as_deref()
                            .unwrap_or("Unsaved buffer"),
                    );
                    ui.add(
                        TextEdit::multiline(&mut self.store.editor_buffer)
                            .desired_rows(12)
                            .code_editor()
                            .hint_text("// Enter .fb script here"),
                    );
                }
            }

            ui.add_space(8.0);
            if ui
                .add_enabled(self.is_dirty(), egui::Button::new("Apply"))
                .clicked()
            {
                self.apply_changes();
                response = NodeResponse::RunPipeline;
            }
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

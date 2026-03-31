use crate::{
    node::{
        Node, NodeColors, NodeContext, NodeError, NodeId, NodePosition, NodeResponse, NodeType,
    },
    pipeline::PipelineData,
};
use egui::{Color32, Ui};
use fireball::abstract_syntax_tree::Ast;
use std::sync::Arc;

/// Output node displaying the final decompiled result
#[derive(Clone, Debug)]
pub struct OutputNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    is_expanded: bool,
    current_ast: Option<Arc<Ast>>,
}

impl OutputNode {
    pub fn new() -> Self {
        Self {
            id: NodeId::new(),
            name: "Output".to_string(),
            position: NodePosition::default(),
            is_expanded: false,
            current_ast: None,
        }
    }

    pub fn with_position(x: f32, y: f32) -> Self {
        let mut node = Self::new();
        node.position = NodePosition::new(x, y);
        node
    }

    pub fn set_ast(&mut self, ast: Arc<Ast>) {
        self.current_ast = Some(ast);
    }

    pub fn clear_ast(&mut self) {
        self.current_ast = None;
    }
}

impl Default for OutputNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for OutputNode {
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
        NodeType::Output
    }

    fn color(&self) -> Color32 {
        NodeColors::output()
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
        match &self.current_ast {
            None => "[Output (empty)]".to_string(),
            Some(ast) => {
                let config = fireball::abstract_syntax_tree::AstPrintConfig::default();
                let code = ast.print(Some(config));
                let func_count = code
                    .lines()
                    .filter(|l| l.contains("void ") || l.contains("int ") || l.contains("func "))
                    .count();
                let line_count = code.lines().count();
                format!("[Output ({} func, {} lines)]", func_count, line_count)
            }
        }
    }

    fn process(&self, input: PipelineData) -> Result<PipelineData, NodeError> {
        // Output node just passes through the AST
        match input {
            PipelineData::Ast(ast) => {
                // Just pass through - the UI will display the AST
                Ok(PipelineData::Ast(ast))
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
            ui.label(self.summary());

            // Output node can be deleted but not disabled
            if ctx.can_delete && ui.button("x").clicked() {
                response = NodeResponse::Deleted;
            }
        });

        // Expanded content - show the AST code
        if self.is_expanded {
            ui.separator();
            if let Some(ref ast) = self.current_ast {
                let config = fireball::abstract_syntax_tree::AstPrintConfig::default();
                let code = ast.print(Some(config));
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        ui.monospace(&code);
                    });
            } else {
                ui.label("No output available. Run the pipeline first.");
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

            // Note: Output node can't store AST during process(&self)
            // Stats will be shown in expanded view after pipeline runs
            ui.label("Expand to view output");
        });

        response
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "OutputNode",
            "id": self.id.0.to_string(),
            "position": {"x": self.position.x, "y": self.position.y},
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
        if let Some(expanded) = value.get("is_expanded").and_then(|v| v.as_bool()) {
            self.is_expanded = expanded;
        }
    }
}

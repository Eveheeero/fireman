use crate::{
    model::DecompileResult,
    node::{
        Node, NodeColors, NodeContext, NodeError, NodeId, NodePosition, NodeResponse, NodeType,
    },
    pipeline::PipelineData,
};
use egui::{Color32, Ui};
use fireball::abstract_syntax_tree::Ast;
use std::sync::Arc;

/// Passive read-only snapshot node displaying decompiled output.
#[derive(Clone, Debug)]
pub struct PreviewNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    is_expanded: bool,
    /// Snapshot from nearest preceding OptNode or base.
    pub snapshot_ast: Option<Arc<Ast>>,
    pub snapshot_output: Option<DecompileResult>,
}

impl PreviewNode {
    pub fn new() -> Self {
        Self {
            id: NodeId::new(),
            name: "Preview".to_string(),
            position: NodePosition::default(),
            is_expanded: false,
            snapshot_ast: None,
            snapshot_output: None,
        }
    }

    pub fn with_position(x: f32, y: f32) -> Self {
        let mut node = Self::new();
        node.position = NodePosition::new(x, y);
        node
    }

    pub fn set_snapshot(&mut self, ast: Arc<Ast>, output: Option<DecompileResult>) {
        self.snapshot_ast = Some(ast);
        self.snapshot_output = output;
    }

    pub fn clear_snapshot(&mut self) {
        self.snapshot_ast = None;
        self.snapshot_output = None;
    }

    pub fn rendered_code(&self) -> Option<String> {
        if let Some(output) = &self.snapshot_output {
            if !output.assembly.is_empty() {
                return Some(join_lines(
                    output.assembly.iter().map(|line| line.data.as_str()),
                ));
            }

            if !output.ir.is_empty() {
                return Some(join_lines(output.ir.iter().map(|line| line.data.as_str())));
            }

            if !output.ast.is_empty() {
                return Some(join_lines(output.ast.iter().map(|line| line.data.as_str())));
            }
        }

        self.snapshot_ast.as_ref().map(|ast| {
            let config = fireball::abstract_syntax_tree::AstPrintConfig::default();
            ast.print(Some(config))
        })
    }
}

fn join_lines<'a>(lines: impl Iterator<Item = &'a str>) -> String {
    lines.collect::<Vec<_>>().join("\n")
}

impl Default for PreviewNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for PreviewNode {
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
        NodeType::Preview
    }

    fn color(&self) -> Color32 {
        NodeColors::preview()
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
        self.rendered_code()
            .map(|code| format!("{} lines captured", code.lines().count()))
            .unwrap_or_else(|| "No output yet".to_string())
    }

    fn process(&self, input: PipelineData) -> Result<PipelineData, NodeError> {
        // Preview node is passive — just passes through.
        match input {
            PipelineData::Ast(ast) => Ok(PipelineData::Ast(ast)),
            _ => Err(NodeError::InvalidInput {
                expected: "AST",
                got: input.type_name(),
            }),
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
        ui.small("Select node to inspect preview output.");

        response
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "PreviewNode",
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

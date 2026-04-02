use crate::{
    node::{
        Node, NodeColors, NodeContext, NodeError, NodeId, NodePosition, NodeResponse, NodeType,
    },
    pipeline::PipelineData,
};
use egui::{Color32, Ui};
use std::path::PathBuf;

/// Input node for holding the active file path while the shell owns async loading.
#[derive(Clone, Debug)]
pub struct InputNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    file_path: Option<PathBuf>,
    is_expanded: bool,
}

impl InputNode {
    pub fn new() -> Self {
        Self {
            id: NodeId::new(),
            name: "Input".to_string(),
            position: NodePosition::default(),
            file_path: None,
            is_expanded: false,
        }
    }

    pub fn with_position(x: f32, y: f32) -> Self {
        let mut node = Self::new();
        node.position = NodePosition::new(x, y);
        node
    }

    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn set_file_path(&mut self, path: PathBuf) {
        self.file_path = Some(path);
    }
}

impl Default for InputNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for InputNode {
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
        NodeType::Input
    }

    fn color(&self) -> Color32 {
        NodeColors::input()
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
        if let Some(ref path) = self.file_path {
            if let Some(filename) = path.file_name() {
                return format!("[File: {}]", filename.to_string_lossy());
            }
        }
        "[No file selected]".to_string()
    }

    fn process(&self, input: PipelineData) -> Result<PipelineData, NodeError> {
        Ok(input)
    }

    fn ui(&mut self, ui: &mut Ui, ctx: &NodeContext) -> NodeResponse {
        let mut response = NodeResponse::None;

        // Header with file path
        ui.horizontal(|ui| {
            ui.label(self.summary());

            if ctx.can_delete && ui.button("x").clicked() {
                response = NodeResponse::Deleted;
            }
        });

        // Expanded content
        if self.is_expanded {
            ui.separator();

            if let Some(ref path) = self.file_path {
                ui.label(format!("Path: {}", path.display()));
                ui.label("Use the shell side panel to open, analyze, and select sections.");
            } else {
                ui.label("Select this node to open a binary from the shell side panel.");
            }
        }

        // Expand/collapse button
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
            "type": "InputNode",
            "id": self.id.0.to_string(),
            "position": {"x": self.position.x, "y": self.position.y},
            "file_path": self.file_path.as_ref().map(|p| p.to_string_lossy().to_string()),
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
        if let Some(path_str) = value.get("file_path").and_then(|v| v.as_str()) {
            self.file_path = Some(PathBuf::from(path_str));
        }
        if let Some(expanded) = value.get("is_expanded").and_then(|v| v.as_bool()) {
            self.is_expanded = expanded;
        }
    }
}

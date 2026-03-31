use crate::{
    node::{
        Node, NodeColors, NodeContext, NodeError, NodeId, NodePosition, NodeResponse, NodeType,
    },
    pipeline::PipelineData,
};
use egui::{Color32, Ui};
use fireball::{abstract_syntax_tree::Ast, core::FireRaw, ir::analyze::generate_ast};
use std::{path::PathBuf, sync::Arc};

/// Input node for loading binary files and converting directly to AST
#[derive(Clone, Debug)]
pub struct InputNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    file_path: Option<PathBuf>,
    is_expanded: bool,
    loaded_ast: Option<Arc<Ast>>,
}

impl InputNode {
    pub fn new() -> Self {
        Self {
            id: NodeId::new(),
            name: "Input".to_string(),
            position: NodePosition::default(),
            file_path: None,
            is_expanded: false,
            loaded_ast: None,
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
        self.loaded_ast = None;
    }

    fn load_binary_to_ast(&mut self) -> Result<Arc<Ast>, NodeError> {
        if let Some(ref path) = self.file_path {
            // Load binary with fireball
            let fireball =
                fireball::Fireball::from_path(path.to_str().unwrap_or("")).map_err(|e| {
                    NodeError::ProcessingError(format!("Failed to load binary: {:?}", e))
                })?;

            // Analyze to get blocks
            let blocks = fireball
                .analyze_all()
                .map_err(|e| NodeError::ProcessingError(format!("Analysis failed: {:?}", e)))?;

            // Convert blocks to AST using generate_ast
            let ast = generate_ast(blocks.into_iter()).map_err(|e| {
                NodeError::ProcessingError(format!("AST generation failed: {:?}", e))
            })?;

            let arc_ast = Arc::new(ast);
            self.loaded_ast = Some(arc_ast.clone());
            Ok(arc_ast)
        } else {
            Err(NodeError::MissingData("No file path specified"))
        }
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

    fn process(&self, _input: PipelineData) -> Result<PipelineData, NodeError> {
        // Input node doesn't accept input, creates AST from binary
        if let Some(ref ast) = self.loaded_ast {
            return Ok(PipelineData::Ast(ast.clone()));
        }

        // Try to load if path is set
        if self.file_path.is_some() {
            let mut clone = self.clone();
            let ast = clone.load_binary_to_ast()?;
            Ok(PipelineData::Ast(ast))
        } else {
            Err(NodeError::MissingData("No binary file loaded"))
        }
    }

    fn ui(&mut self, ui: &mut Ui, ctx: &NodeContext) -> NodeResponse {
        let mut response = NodeResponse::None;

        // Header with file path
        ui.horizontal(|ui| {
            ui.label(self.summary());

            if ui.button("Browse").clicked() {
                // Open file dialog using rfd
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter(
                        "Binary files",
                        &["exe", "dll", "so", "dylib", "bin", "o", "obj"],
                    )
                    .add_filter("All files", &["*"])
                    .pick_file()
                {
                    self.set_file_path(path);
                    // Try to load immediately
                    if let Err(e) = self.load_binary_to_ast() {
                        eprintln!("Failed to load binary: {:?}", e);
                    }
                    response = NodeResponse::Selected;
                }
            }

            if ctx.can_delete && ui.button("x").clicked() {
                response = NodeResponse::Deleted;
            }
        });

        // Expanded content
        if self.is_expanded {
            ui.separator();

            if let Some(ref path) = self.file_path {
                ui.label(format!("Path: {}", path.display()));

                if let Some(ref ast) = self.loaded_ast {
                    let config = fireball::abstract_syntax_tree::AstPrintConfig::default();
                    let code = ast.print(Some(config));
                    let func_count = code
                        .lines()
                        .filter(|l| {
                            l.contains("void ") || l.contains("int ") || l.contains("func ")
                        })
                        .count();
                    ui.label(format!("AST generated with {} functions", func_count));
                } else {
                    ui.label("Click Browse to select a binary file");
                }
            } else {
                ui.label("Click Browse to select a binary file");
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

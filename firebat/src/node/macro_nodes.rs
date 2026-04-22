use crate::{
    node::{Node, NodeContext, NodeError, NodeId, NodePosition, NodeResponse, NodeType},
    pipeline::PipelineData,
};
use egui::{Color32, Ui};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacroComparison {
    LessThan,
    LessEqual,
    Equal,
    NotEqual,
    GreaterEqual,
    GreaterThan,
}

impl MacroComparison {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LessThan => "<",
            Self::LessEqual => "<=",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::GreaterEqual => ">=",
            Self::GreaterThan => ">",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArithmeticOperation {
    Set,
    Add,
    Subtract,
    Multiply,
}

impl ArithmeticOperation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Set => "=",
            Self::Add => "+=",
            Self::Subtract => "-=",
            Self::Multiply => "*=",
        }
    }
}

fn macro_color(red: u8, green: u8, blue: u8) -> Color32 {
    Color32::from_rgb(red, green, blue)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoopMacroNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    is_expanded: bool,
    pub iterations: usize,
}

impl LoopMacroNode {
    pub fn new() -> Self {
        Self {
            id: NodeId::new(),
            name: "Loop".to_string(),
            position: NodePosition::default(),
            is_expanded: false,
            iterations: 3,
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = NodePosition::new(x, y);
        self
    }
}

impl Default for LoopMacroNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for LoopMacroNode {
    fn id(&self) -> NodeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn node_type(&self) -> NodeType {
        NodeType::LoopMacro
    }

    fn color(&self) -> Color32 {
        macro_color(0xB5, 0x5F, 0x1D)
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
        format!("Expands the following template {} time(s)", self.iterations)
    }

    fn process(&self, input: PipelineData) -> Result<PipelineData, NodeError> {
        Ok(input)
    }

    fn ui(&mut self, ui: &mut Ui, _ctx: &NodeContext) -> NodeResponse {
        ui.small(self.summary());
        ui.horizontal(|ui| {
            ui.label("Iterations");
            ui.add(egui::DragValue::new(&mut self.iterations).range(1..=64));
        });
        NodeResponse::None
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "LoopMacroNode",
            "id": self.id.0.to_string(),
            "name": self.name,
            "position": {"x": self.position.x, "y": self.position.y},
            "is_expanded": self.is_expanded,
            "iterations": self.iterations,
        })
    }

    fn deserialize(&mut self, value: &serde_json::Value) {
        if let Some(name) = value.get("name").and_then(|v| v.as_str()) {
            self.name = name.to_string();
        }
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
        if let Some(iterations) = value.get("iterations").and_then(|v| v.as_u64()) {
            self.iterations = iterations.max(1) as usize;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VariableMacroNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    is_expanded: bool,
    pub variable: String,
    pub initial_value: i64,
}

impl VariableMacroNode {
    pub fn new() -> Self {
        Self {
            id: NodeId::new(),
            name: "Var".to_string(),
            position: NodePosition::default(),
            is_expanded: false,
            variable: "i".to_string(),
            initial_value: 0,
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = NodePosition::new(x, y);
        self
    }
}

impl Default for VariableMacroNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for VariableMacroNode {
    fn id(&self) -> NodeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn node_type(&self) -> NodeType {
        NodeType::VariableMacro
    }

    fn color(&self) -> Color32 {
        macro_color(0x7C, 0x64, 0x19)
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
        format!("{} starts at {}", self.variable, self.initial_value)
    }

    fn process(&self, input: PipelineData) -> Result<PipelineData, NodeError> {
        Ok(input)
    }

    fn ui(&mut self, ui: &mut Ui, _ctx: &NodeContext) -> NodeResponse {
        ui.small(self.summary());
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.text_edit_singleline(&mut self.variable);
        });
        ui.horizontal(|ui| {
            ui.label("Initial");
            ui.add(egui::DragValue::new(&mut self.initial_value));
        });
        NodeResponse::None
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "VariableMacroNode",
            "id": self.id.0.to_string(),
            "name": self.name,
            "position": {"x": self.position.x, "y": self.position.y},
            "is_expanded": self.is_expanded,
            "variable": self.variable,
            "initial_value": self.initial_value,
        })
    }

    fn deserialize(&mut self, value: &serde_json::Value) {
        if let Some(name) = value.get("name").and_then(|v| v.as_str()) {
            self.name = name.to_string();
        }
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
        if let Some(variable) = value.get("variable").and_then(|v| v.as_str()) {
            self.variable = variable.to_string();
        }
        if let Some(initial_value) = value.get("initial_value").and_then(|v| v.as_i64()) {
            self.initial_value = initial_value;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IfMacroNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    is_expanded: bool,
    pub variable: String,
    pub comparison: MacroComparison,
    pub value: i64,
}

impl IfMacroNode {
    pub fn new() -> Self {
        Self {
            id: NodeId::new(),
            name: "If".to_string(),
            position: NodePosition::default(),
            is_expanded: false,
            variable: "i".to_string(),
            comparison: MacroComparison::LessThan,
            value: 1,
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = NodePosition::new(x, y);
        self
    }
}

impl Default for IfMacroNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for IfMacroNode {
    fn id(&self) -> NodeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn node_type(&self) -> NodeType {
        NodeType::IfMacro
    }

    fn color(&self) -> Color32 {
        macro_color(0x8B, 0x4A, 0x16)
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
        format!(
            "Applies to the next optimization when {} {} {}",
            self.variable,
            self.comparison.name(),
            self.value
        )
    }

    fn process(&self, input: PipelineData) -> Result<PipelineData, NodeError> {
        Ok(input)
    }

    fn ui(&mut self, ui: &mut Ui, _ctx: &NodeContext) -> NodeResponse {
        ui.small(self.summary());
        ui.horizontal(|ui| {
            ui.label("Var");
            ui.text_edit_singleline(&mut self.variable);
        });
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt(("if-macro-op", self.id.0))
                .selected_text(self.comparison.name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.comparison, MacroComparison::LessThan, "<");
                    ui.selectable_value(&mut self.comparison, MacroComparison::LessEqual, "<=");
                    ui.selectable_value(&mut self.comparison, MacroComparison::Equal, "==");
                    ui.selectable_value(&mut self.comparison, MacroComparison::NotEqual, "!=");
                    ui.selectable_value(&mut self.comparison, MacroComparison::GreaterEqual, ">=");
                    ui.selectable_value(&mut self.comparison, MacroComparison::GreaterThan, ">");
                });
            ui.add(egui::DragValue::new(&mut self.value));
        });
        NodeResponse::None
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "IfMacroNode",
            "id": self.id.0.to_string(),
            "name": self.name,
            "position": {"x": self.position.x, "y": self.position.y},
            "is_expanded": self.is_expanded,
            "variable": self.variable,
            "comparison": self.comparison,
            "value": self.value,
        })
    }

    fn deserialize(&mut self, value: &serde_json::Value) {
        if let Some(name) = value.get("name").and_then(|v| v.as_str()) {
            self.name = name.to_string();
        }
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
        if let Some(variable) = value.get("variable").and_then(|v| v.as_str()) {
            self.variable = variable.to_string();
        }
        if let Some(comparison) = value.get("comparison") {
            if let Ok(parsed) = serde_json::from_value::<MacroComparison>(comparison.clone()) {
                self.comparison = parsed;
            }
        }
        if let Some(parsed_value) = value.get("value").and_then(|v| v.as_i64()) {
            self.value = parsed_value;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArithmeticMacroNode {
    id: NodeId,
    name: String,
    position: NodePosition,
    is_expanded: bool,
    pub target_variable: String,
    pub operation: ArithmeticOperation,
    pub value: i64,
}

impl ArithmeticMacroNode {
    pub fn new() -> Self {
        Self {
            id: NodeId::new(),
            name: "Operation".to_string(),
            position: NodePosition::default(),
            is_expanded: false,
            target_variable: "i".to_string(),
            operation: ArithmeticOperation::Add,
            value: 1,
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = NodePosition::new(x, y);
        self
    }
}

impl Default for ArithmeticMacroNode {
    fn default() -> Self {
        Self::new()
    }
}

impl Node for ArithmeticMacroNode {
    fn id(&self) -> NodeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn node_type(&self) -> NodeType {
        NodeType::ArithmeticMacro
    }

    fn color(&self) -> Color32 {
        macro_color(0x9C, 0x52, 0x14)
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
        format!(
            "{} {} {}",
            self.target_variable,
            self.operation.name(),
            self.value
        )
    }

    fn process(&self, input: PipelineData) -> Result<PipelineData, NodeError> {
        Ok(input)
    }

    fn ui(&mut self, ui: &mut Ui, _ctx: &NodeContext) -> NodeResponse {
        ui.small(self.summary());
        ui.horizontal(|ui| {
            ui.label("Var");
            ui.text_edit_singleline(&mut self.target_variable);
        });
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt(("op-macro-op", self.id.0))
                .selected_text(self.operation.name())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.operation, ArithmeticOperation::Set, "=");
                    ui.selectable_value(&mut self.operation, ArithmeticOperation::Add, "+=");
                    ui.selectable_value(&mut self.operation, ArithmeticOperation::Subtract, "-=");
                    ui.selectable_value(&mut self.operation, ArithmeticOperation::Multiply, "*=");
                });
            ui.add(egui::DragValue::new(&mut self.value));
        });
        NodeResponse::None
    }

    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn serialize(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "ArithmeticMacroNode",
            "id": self.id.0.to_string(),
            "name": self.name,
            "position": {"x": self.position.x, "y": self.position.y},
            "is_expanded": self.is_expanded,
            "target_variable": self.target_variable,
            "operation": self.operation,
            "value": self.value,
        })
    }

    fn deserialize(&mut self, value: &serde_json::Value) {
        if let Some(name) = value.get("name").and_then(|v| v.as_str()) {
            self.name = name.to_string();
        }
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
        if let Some(target_variable) = value.get("target_variable").and_then(|v| v.as_str()) {
            self.target_variable = target_variable.to_string();
        }
        if let Some(operation) = value.get("operation") {
            if let Ok(parsed) = serde_json::from_value::<ArithmeticOperation>(operation.clone()) {
                self.operation = parsed;
            }
        }
        if let Some(parsed_value) = value.get("value").and_then(|v| v.as_i64()) {
            self.value = parsed_value;
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub mod input;
pub mod optimization;
pub mod output;

use crate::pipeline::PipelineData;
use egui::{Color32, Pos2, Ui};
pub use input::InputNode;
pub use optimization::{OPTIMIZATION_FIELDS, OptNode, OptimizationPass};
pub use output::PreviewNode;
use serde::{Deserialize, Serialize};
use std::{
    any::Any,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_NODE_ID: AtomicU64 = AtomicU64::new(1);

/// Unique identifier for nodes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

impl NodeId {
    pub fn new() -> Self {
        Self(NEXT_NODE_ID.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

/// Position of a node in 2D graph space
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodePosition {
    pub x: f32,
    pub y: f32,
}

impl NodePosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Convert to egui::Pos2
    pub fn to_pos2(&self) -> Pos2 {
        Pos2::new(self.x, self.y)
    }

    /// Create from egui::Pos2
    pub fn from_pos2(pos: Pos2) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

/// Response from node UI interaction
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeResponse {
    None,
    Selected,
    Deleted,
    ToggleExpanded,
    ToggleEnabled,
    RunPipeline,
    DraggedTo { new_pos: NodePosition },
    InputPortClicked,
    OutputPortClicked,
}

/// Type of node in the graph
#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    Input,
    Opt,
    Preview,
}

impl NodeType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Input => "Input",
            Self::Opt => "Optimization",
            Self::Preview => "Preview",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Input => "",
            Self::Opt => "",
            Self::Preview => "",
        }
    }
}

/// Error types for node processing
#[derive(Clone, Debug)]
pub enum NodeError {
    MissingData(&'static str),
    InvalidInput {
        expected: &'static str,
        got: &'static str,
    },
    ProcessingError(String),
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingData(msg) => write!(f, "Missing data: {}", msg),
            Self::InvalidInput { expected, got } => {
                write!(f, "Invalid input: expected {}, got {}", expected, got)
            }
            Self::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
        }
    }
}

impl std::error::Error for NodeError {}

/// Color palette for different node types
pub struct NodeColors;

impl NodeColors {
    pub fn input() -> Color32 {
        Color32::from_rgb(0x0F, 0x6C, 0xBD) // Blue
    }

    pub fn optimization() -> Color32 {
        Color32::from_rgb(0x03, 0x83, 0x87) // Cyan
    }

    pub fn preview() -> Color32 {
        Color32::from_rgb(0x0F, 0x7B, 0x0F) // Green
    }
}

/// Context passed to node UI rendering
pub struct NodeContext {
    pub is_selected: bool,
    pub is_dragging: bool,
    pub can_delete: bool,
}

/// Core trait for all pipeline nodes
pub trait Node: Any {
    fn id(&self) -> NodeId;
    fn name(&self) -> &str;
    fn node_type(&self) -> NodeType;
    fn color(&self) -> Color32;
    fn position(&self) -> NodePosition;
    fn set_position(&mut self, pos: NodePosition);
    fn is_expanded(&self) -> bool;
    fn toggle_expanded(&mut self);
    fn is_enabled(&self) -> bool {
        true
    }
    fn set_enabled(&mut self, _enabled: bool) {}
    fn summary(&self) -> String;
    fn process(&self, input: PipelineData) -> Result<PipelineData, NodeError>;
    fn ui(&mut self, ui: &mut Ui, ctx: &NodeContext) -> NodeResponse;
    fn clone_box(&self) -> Box<dyn Node>;
    fn serialize(&self) -> serde_json::Value;
    fn deserialize(&mut self, value: &serde_json::Value);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Graph containing multiple nodes connected in a pipeline
#[derive(Clone, Default)]
pub struct NodeGraph {
    nodes: Vec<Box<dyn Node>>,
    connections: Vec<(NodeId, NodeId)>,
}

impl NodeGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    }

    /// Remove a node from the graph by ID
    pub fn remove_node(&mut self, id: NodeId) -> Option<Box<dyn Node>> {
        if let Some(index) = self.nodes.iter().position(|n| n.id() == id) {
            // Remove connections involving this node
            self.connections
                .retain(|(from, to)| *from != id && *to != id);
            Some(self.nodes.remove(index))
        } else {
            None
        }
    }

    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> Option<&dyn Node> {
        self.nodes.iter().find(|n| n.id() == id).map(|n| n.as_ref())
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut dyn Node> {
        self.nodes
            .iter_mut()
            .find(|n| n.id() == id)
            .map(|n| n.as_mut())
    }

    /// Get all nodes
    pub fn nodes(&self) -> &[Box<dyn Node>] {
        &self.nodes
    }

    /// Get all nodes mutably
    pub fn nodes_mut(&mut self) -> &mut [Box<dyn Node>] {
        &mut self.nodes
    }

    /// Get all connections
    pub fn connections(&self) -> &[(NodeId, NodeId)] {
        &self.connections
    }

    /// Add a connection between two nodes
    pub fn add_connection(&mut self, from: NodeId, to: NodeId) {
        if from == to {
            return;
        }

        let Some(source) = self.get_node(from) else {
            return;
        };
        let Some(target) = self.get_node(to) else {
            return;
        };

        let source_supports_output = matches!(source.node_type(), NodeType::Input | NodeType::Opt);
        let target_supports_input = matches!(target.node_type(), NodeType::Opt | NodeType::Preview);
        if !source_supports_output || !target_supports_input {
            return;
        }

        if self
            .connections
            .iter()
            .any(|(existing_from, existing_to)| *existing_from == from && *existing_to == to)
        {
            return;
        }

        // Keep one upstream input per target, but allow the source to branch out.
        self.connections.retain(|(_, target_id)| *target_id != to);
        self.connections.push((from, to));
    }

    /// Remove a connection
    pub fn remove_connection(&mut self, from: NodeId, to: NodeId) {
        self.connections.retain(|(f, t)| *f != from || *t != to);
    }

    /// Collect OptNode IDs in pipeline order (for async queue).
    pub fn opt_node_ids(&self) -> Vec<NodeId> {
        self.nodes
            .iter()
            .filter(|n| matches!(n.node_type(), NodeType::Opt))
            .map(|n| n.id())
            .collect()
    }

    /// Clear all nodes and connections
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.connections.clear();
    }

    /// Get node count
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

// Implement as_any and as_any_mut for all node types
impl dyn Node {
    pub fn downcast_ref<T: Node>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    pub fn downcast_mut<T: Node>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut::<T>()
    }
}

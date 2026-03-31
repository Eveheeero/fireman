pub mod input;
pub mod optimization;
pub mod output;

use crate::pipeline::PipelineData;
use egui::{Color32, Pos2, Ui};
pub use input::InputNode;
pub use optimization::{OptimizationNode, OptimizationPass};
pub use output::OutputNode;
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
    DraggedTo { new_pos: NodePosition },
    InputPortClicked,
    OutputPortClicked,
}

/// Type of node in the graph
#[derive(Clone, Debug, PartialEq)]
pub enum NodeType {
    Input,
    Optimization(OptimizationPass),
    Output,
}

impl NodeType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Input => "Input",
            Self::Optimization(_) => "Optimization",
            Self::Output => "Output",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Input => "",
            Self::Optimization(_) => "",
            Self::Output => "",
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

    pub fn output() -> Color32 {
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

    /// Get the index of a node by ID
    pub fn get_node_index(&self, id: NodeId) -> Option<usize> {
        self.nodes.iter().position(|n| n.id() == id)
    }

    /// Move a node from one position to another in the vector
    pub fn move_node(&mut self, from: usize, to: usize) {
        if from < self.nodes.len() && to < self.nodes.len() && from != to {
            let node = self.nodes.remove(from);
            let new_index = if to > from { to - 1 } else { to };
            self.nodes.insert(new_index, node);
        }
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
        // Check if both nodes exist
        if self.get_node(from).is_some() && self.get_node(to).is_some() {
            // Remove existing connection to the target if any
            self.connections.retain(|(_, target)| *target != to);
            self.connections.push((from, to));
        }
    }

    /// Remove a connection
    pub fn remove_connection(&mut self, from: NodeId, to: NodeId) {
        self.connections.retain(|(f, t)| *f != from || *t != to);
    }

    /// Execute the pipeline and return the final PipelineData
    pub fn execute(&self) -> Result<PipelineData, NodeError> {
        if self.nodes.is_empty() {
            return Ok(PipelineData::Empty);
        }

        // Find input node (should be first or connected from nothing)
        let input_node = self
            .nodes
            .iter()
            .find(|n| matches!(n.node_type(), NodeType::Input))
            .or_else(|| self.nodes.first())
            .ok_or_else(|| NodeError::MissingData("No nodes in graph"))?;

        // Start with empty data
        let mut data = PipelineData::Empty;

        // Process nodes in order using connections to determine flow
        let mut processed_ids = std::collections::HashSet::new();
        let mut current_ids: Vec<NodeId> = self
            .nodes
            .iter()
            .filter(|n| !self.connections.iter().any(|(_, to)| *to == n.id()))
            .map(|n| n.id())
            .collect();

        while !current_ids.is_empty() {
            let mut next_ids = Vec::new();

            for id in current_ids {
                if processed_ids.contains(&id) {
                    continue;
                }

                if let Some(node) = self.get_node(id) {
                    if node.is_enabled() {
                        data = node.process(data)?;
                    }
                    processed_ids.insert(id);

                    // Find next nodes connected from this one
                    for (from, to) in &self.connections {
                        if *from == id && !processed_ids.contains(to) {
                            next_ids.push(*to);
                        }
                    }
                }
            }

            current_ids = next_ids;
        }

        Ok(data)
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

    /// Check if graph is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Apply an optimization preset by adding/removing nodes
    pub fn apply_preset(
        &mut self,
        preset: &crate::model::OptimizationScriptPreset,
    ) -> Result<(), String> {
        // For now, just log that we would apply the preset
        // In a full implementation, this would add/remove optimization nodes
        if preset.enabled {
            // Add an optimization node for this preset
            let pass = OptimizationPass::PatternMatching(vec![preset.path.clone()]);
            let node = OptimizationNode::new(pass, self.nodes.len() + 1);
            self.add_node(Box::new(node));
        }
        Ok(())
    }

    /// Serialize the graph to JSON
    pub fn serialize(&self) -> serde_json::Value {
        let nodes: Vec<serde_json::Value> = self.nodes.iter().map(|n| n.serialize()).collect();

        let connections: Vec<serde_json::Value> = self
            .connections
            .iter()
            .map(|(from, to)| {
                serde_json::json!({
                    "from": from.0,
                    "to": to.0
                })
            })
            .collect();

        serde_json::json!({
            "nodes": nodes,
            "connections": connections
        })
    }

    /// Deserialize the graph from JSON
    pub fn deserialize(&mut self, value: &serde_json::Value) -> Result<(), String> {
        self.clear();

        if let Some(nodes) = value.get("nodes").and_then(|n| n.as_array()) {
            for node_value in nodes {
                if let Some(node_type) = node_value.get("type").and_then(|t| t.as_str()) {
                    let mut node: Box<dyn Node> = match node_type {
                        "InputNode" => Box::new(InputNode::new()),
                        "OptimizationNode" => {
                            // Parse pass type from JSON
                            let pass_type = if let Some(pass_str) =
                                node_value.get("pass_type").and_then(|p| p.as_str())
                            {
                                serde_json::from_str::<OptimizationPass>(pass_str)
                                    .unwrap_or(OptimizationPass::ControlFlowCleanup)
                            } else {
                                OptimizationPass::ControlFlowCleanup
                            };
                            let pass_number = node_value
                                .get("pass_number")
                                .and_then(|n| n.as_u64())
                                .unwrap_or(1)
                                as usize;
                            Box::new(OptimizationNode::new(pass_type, pass_number))
                        }
                        "OutputNode" => Box::new(OutputNode::new()),
                        _ => continue,
                    };
                    node.deserialize(node_value);
                    self.add_node(node);
                }
            }
        }

        if let Some(connections) = value.get("connections").and_then(|c| c.as_array()) {
            for conn in connections {
                if let (Some(from), Some(to)) = (
                    conn.get("from").and_then(|f| f.as_u64()).map(NodeId),
                    conn.get("to").and_then(|t| t.as_u64()).map(NodeId),
                ) {
                    self.add_connection(from, to);
                }
            }
        }

        Ok(())
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

// Implement as_any methods for each node type
macro_rules! impl_node_as_any {
    ($type:ty) => {
        impl Node for $type {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }
    };
}

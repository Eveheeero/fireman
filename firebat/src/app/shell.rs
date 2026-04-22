use super::state::FirebatState;
use crate::{
    model::{
        DecompileRequest, DecompileResult, GraphPreset, OptimizeAstRequest, PersistedViewport,
    },
    node::{
        ArithmeticMacroNode, ArithmeticOperation, Connection, IfMacroNode, InputNode,
        MacroComparison, Node, NodeGraph, NodeId, NodeIdMap, NodePosition, NodeResponse, NodeType,
        OptNode, OptimizationPass, PreviewNode, VariableMacroNode,
    },
    pipeline::PipelineData,
    theme::configure_theme,
    ui::{GraphCanvas, GraphResponse},
    worker::WorkerRequest,
};
use eframe::{
    egui,
    egui::{RichText, ScrollArea, Vec2},
};
use fireball::abstract_syntax_tree::Ast;
use rfd::FileDialog;
use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

pub(crate) struct FirebatApp {
    graph: NodeGraph,
    state: FirebatState,
    show_perf_hud: bool,
    show_about: bool,
    show_add_node_menu: bool,
    applied_dark_mode: Option<bool>,
    last_frame_tick: Option<Instant>,
    frame_samples_ms: VecDeque<f32>,
    selected_node: Option<NodeId>,
    dragged_node: Option<NodeId>,
    camera_offset: Vec2,
    zoom: f32,
    status_message: Option<String>,
    status_timer: Option<Instant>,
    selected_pass_type: OptimizationPass,
    connecting_from: Option<(NodeId, usize)>,
    active_pipeline_input: Option<NodeId>,
}

impl Default for FirebatApp {
    fn default() -> Self {
        let graph = build_default_graph();
        let selected_node = graph.nodes().first().map(|node| node.id());

        Self {
            graph,
            state: FirebatState::default(),
            show_perf_hud: false,
            show_about: false,
            show_add_node_menu: false,
            applied_dark_mode: None,
            last_frame_tick: None,
            frame_samples_ms: VecDeque::with_capacity(240),
            selected_node,
            dragged_node: None,
            camera_offset: Vec2::new(-20.0, -20.0),
            zoom: 1.0,
            status_message: None,
            status_timer: None,
            selected_pass_type: OptimizationPass::ConstantFolding,
            connecting_from: None,
            active_pipeline_input: None,
        }
    }
}

impl FirebatApp {
    fn sync_system_theme(&mut self, ctx: &egui::Context) {
        let is_dark_mode = matches!(ctx.system_theme(), Some(egui::Theme::Dark));
        if self.applied_dark_mode != Some(is_dark_mode) {
            configure_theme(ctx, is_dark_mode);
            self.applied_dark_mode = Some(is_dark_mode);
        }
    }

    fn record_frame_timing(&mut self) {
        let now = Instant::now();
        if let Some(last_tick) = self.last_frame_tick {
            let frame_ms = (now - last_tick).as_secs_f32() * 1000.0;
            if frame_ms.is_finite() && frame_ms > 0.0 {
                self.frame_samples_ms.push_back(frame_ms);
                if self.frame_samples_ms.len() > 240 {
                    let _ = self.frame_samples_ms.pop_front();
                }
            }
        }
        self.last_frame_tick = Some(now);
    }

    fn perf_snapshot(&self) -> Option<(f32, f32, f32)> {
        if self.frame_samples_ms.is_empty() {
            return None;
        }

        let sum: f32 = self.frame_samples_ms.iter().sum();
        let avg_ms = sum / self.frame_samples_ms.len() as f32;

        let mut sorted: Vec<f32> = self.frame_samples_ms.iter().copied().collect();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let p95_index = ((sorted.len() as f32 * 0.95).floor() as usize).min(sorted.len() - 1);
        let p95_ms = sorted[p95_index];
        let fps = if avg_ms > 0.0 { 1000.0 / avg_ms } else { 0.0 };
        Some((avg_ms, p95_ms, fps))
    }

    fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
        self.status_timer = Some(Instant::now());
    }

    fn set_status_if_changed(&mut self, message: impl Into<String>) {
        let message = message.into();
        if self.status_message.as_deref() != Some(message.as_str()) {
            self.set_status(message);
        }
    }

    fn active_input_node_id(&self) -> Option<NodeId> {
        self.selected_node
            .filter(|selected| {
                self.graph
                    .get_node(*selected)
                    .is_some_and(|node| matches!(node.node_type(), NodeType::Input))
            })
            .or_else(|| {
                self.graph
                    .nodes()
                    .iter()
                    .find(|node| matches!(node.node_type(), NodeType::Input))
                    .map(|node| node.id())
            })
    }

    fn clear_graph(&mut self) {
        self.graph.clear();
        self.state.clear_loaded_input_session();
        self.selected_node = None;
        self.dragged_node = None;
        self.connecting_from = None;
        self.active_pipeline_input = None;
    }

    fn replace_graph(&mut self, graph: NodeGraph) {
        self.graph = graph;
        self.state.clear_loaded_input_session();
        self.selected_node = None;
        self.dragged_node = None;
        self.connecting_from = None;
        self.active_pipeline_input = None;
    }

    fn preset_snapshot(&self) -> GraphPreset {
        GraphPreset {
            schema_version: 2,
            viewport: PersistedViewport {
                camera_offset_x: self.camera_offset.x,
                camera_offset_y: self.camera_offset.y,
                zoom: self.zoom,
            },
            nodes: self.graph.serialize_nodes(),
            connections: self.graph.serialize_connections(),
            known_sections: self.state.known_sections.clone(),
            analyze_target_address: self.state.analyze_target_address.clone(),
        }
    }

    fn save_preset(&mut self) {
        let Some(path) = FileDialog::new()
            .add_filter("Firebat Preset", &["json"])
            .set_file_name("firebat-preset.json")
            .save_file()
        else {
            self.set_status("Preset save canceled");
            return;
        };

        match serde_json::to_string_pretty(&self.preset_snapshot()) {
            Ok(json) => match std::fs::write(&path, json) {
                Ok(()) => {
                    self.set_status(format!("Preset saved: {}", path.display()));
                    self.state.log(format!("Preset saved {}", path.display()));
                }
                Err(error) => self.set_status(format!("Preset save failed: {error}")),
            },
            Err(error) => self.set_status(format!("Preset save failed: {error}")),
        }
    }

    fn load_preset(&mut self) {
        let Some(path) = FileDialog::new()
            .add_filter("Firebat Preset", &["json"])
            .pick_file()
        else {
            self.set_status("Preset load canceled");
            return;
        };

        let preset = match std::fs::read_to_string(&path)
            .map_err(|error| error.to_string())
            .and_then(|json| {
                serde_json::from_str::<GraphPreset>(&json).map_err(|error| error.to_string())
            }) {
            Ok(preset) => preset,
            Err(error) => {
                self.set_status(format!("Preset load failed: {error}"));
                return;
            }
        };

        let input_path = first_input_path(&preset);
        let selected_node = self.selected_node_from_preset(&preset, input_path.as_ref());
        match selected_node {
            Ok((graph, id_map)) => {
                self.replace_graph(graph);
                self.state.restore_preset_state(&preset);
                self.camera_offset = Vec2::new(
                    preset.viewport.camera_offset_x,
                    preset.viewport.camera_offset_y,
                );
                self.zoom = preset.viewport.zoom.max(0.1);
                self.selected_node = selected_node_id(&preset, &id_map);

                if let Some(path) = input_path {
                    if path.exists() {
                        self.state.reopen_loaded_path(&path);
                        self.set_status(format!("Preset loaded: {}", path.display()));
                    } else {
                        self.set_status(format!(
                            "Preset loaded, but input file is missing: {}",
                            path.display()
                        ));
                    }
                } else {
                    self.set_status("Preset loaded");
                }
                self.state.log(format!("Preset loaded {}", path.display()));
            }
            Err(error) => self.set_status(format!("Preset load failed: {error}")),
        }
    }

    fn selected_node_from_preset(
        &self,
        preset: &GraphPreset,
        _input_path: Option<&PathBuf>,
    ) -> Result<(NodeGraph, NodeIdMap), String> {
        NodeGraph::from_preset(preset)
    }

    fn remove_node(&mut self, node_id: NodeId) {
        let removed_node_type = self.graph.get_node(node_id).map(|node| node.node_type());
        self.graph.remove_node(node_id);

        if matches!(removed_node_type, Some(NodeType::Input)) {
            self.state.clear_loaded_input_session();
            self.active_pipeline_input = None;
        }

        if self.selected_node == Some(node_id) {
            self.selected_node = None;
        }
        if self.dragged_node == Some(node_id) {
            self.dragged_node = None;
        }
        if self
            .connecting_from
            .is_some_and(|(connecting_node, _)| connecting_node == node_id)
        {
            self.connecting_from = None;
        }
    }

    fn toggle_node_expanded(&mut self, node_id: NodeId) {
        if let Some(node) = self.graph.get_node_mut(node_id) {
            node.toggle_expanded();
        }
    }

    fn execute_pipeline(&mut self) {
        if self.has_macro_nodes() {
            self.set_status("Expand macro nodes before running the pipeline");
            return;
        }
        self.set_status("Executing pipeline...");

        let mut data = self
            .state
            .base_ast
            .clone()
            .map(PipelineData::Ast)
            .unwrap_or(PipelineData::Empty);
        let mut failed = false;
        for node in self.graph.nodes_mut() {
            match node.process(data.clone()) {
                Ok(next) => data = next,
                Err(e) => {
                    self.set_status(format!("Pipeline failed: {}", e));
                    failed = true;
                    break;
                }
            }
        }

        if failed {
            return;
        }

        match data {
            PipelineData::Ast(ref ast) => {
                let config = fireball::abstract_syntax_tree::AstPrintConfig::default();
                let code = ast.print(Some(config));
                let funcs = code
                    .lines()
                    .filter(|l| l.contains("void ") || l.contains("int ") || l.contains("func "))
                    .count();
                self.set_status(format!("Pipeline completed: {} functions", funcs));
            }
            PipelineData::Empty => {
                self.set_status("Pipeline completed (empty)");
            }
        }
    }

    fn selected_input_node(&self) -> Option<&InputNode> {
        let selected = self.selected_node?;
        self.graph
            .get_node(selected)?
            .as_any()
            .downcast_ref::<InputNode>()
    }

    fn selected_input_node_mut(&mut self) -> Option<&mut InputNode> {
        let selected = self.selected_node?;
        self.graph
            .get_node_mut(selected)?
            .as_any_mut()
            .downcast_mut::<InputNode>()
    }

    fn is_input_node_selected(&self) -> bool {
        self.selected_input_node().is_some()
    }

    fn selected_preview_node(&self) -> Option<&PreviewNode> {
        let selected = self.selected_node?;
        self.graph
            .get_node(selected)?
            .as_any()
            .downcast_ref::<PreviewNode>()
    }

    fn is_preview_node_selected(&self) -> bool {
        self.selected_preview_node().is_some()
    }

    fn selected_optimization_node(&self) -> Option<&OptNode> {
        let selected = self.selected_node?;
        self.graph
            .get_node(selected)?
            .as_any()
            .downcast_ref::<OptNode>()
    }

    fn is_macro_editor_selected(&self) -> bool {
        self.selected_node.is_some_and(|selected| {
            self.graph.get_node(selected).is_some_and(|node| {
                matches!(
                    node.node_type(),
                    NodeType::IfMacro | NodeType::VariableMacro | NodeType::ArithmeticMacro
                )
            })
        })
    }

    fn selected_pattern_matching_node(&self) -> Option<&OptNode> {
        self.selected_optimization_node()
            .filter(|node| node.supports_pattern_editor())
    }

    fn is_pattern_matching_node_selected(&self) -> bool {
        self.selected_pattern_matching_node().is_some()
    }

    fn render_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Save Preset").clicked() {
                self.save_preset();
            }

            if ui.button("Load Preset").clicked() {
                self.load_preset();
            }

            ui.add_space(8.0);

            let add_response = ui.button("+ Add Node");
            if add_response.clicked() {
                self.show_add_node_menu = !self.show_add_node_menu;
            }

            ui.add_space(8.0);

            let exec_response = ui.button("Execute Pipeline");
            if exec_response.clicked() {
                if self.state.selected_addresses().is_empty() {
                    self.execute_pipeline();
                } else {
                    self.start_pipeline();
                }
            }

            ui.add_space(8.0);

            if ui
                .button(if self.state.log_expanded {
                    "Hide Logs"
                } else {
                    "Show Logs"
                })
                .clicked()
            {
                self.state.log_expanded = !self.state.log_expanded;
            }

            if ui.button("Clear Graph").clicked() {
                self.clear_graph();
                self.set_status("Graph cleared");
            }

            ui.add_space(16.0);

            if let Some(ref status) = self.status_message {
                ui.label(RichText::new(status).italics());

                if let Some(timer) = self.status_timer {
                    if timer.elapsed() > Duration::from_secs(3) {
                        self.status_message = None;
                        self.status_timer = None;
                    }
                }
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("{:.0}%", self.zoom * 100.0));
                ui.label("Zoom:");
                ui.add_space(16.0);

                if self.state.is_busy() {
                    ui.label("Processing...");
                } else {
                    ui.label("Ready");
                }
                ui.add_space(8.0);
                ui.label("Status:");
            });
        });

        if self.show_add_node_menu {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Add:");
                if ui.button("Input").clicked() {
                    self.add_node_at_center(NodeType::Input);
                    self.show_add_node_menu = false;
                }

                ui.add_space(8.0);
                if ui.button("If").clicked() {
                    self.add_node_at_center(NodeType::IfMacro);
                    self.show_add_node_menu = false;
                }
                if ui.button("Var").clicked() {
                    self.add_node_at_center(NodeType::VariableMacro);
                    self.show_add_node_menu = false;
                }
                if ui.button("Operation").clicked() {
                    self.add_node_at_center(NodeType::ArithmeticMacro);
                    self.show_add_node_menu = false;
                }

                ui.add_space(8.0);
                if ui.button("Pattern Matching").clicked() {
                    self.add_pattern_matching_node_at_center();
                    self.show_add_node_menu = false;
                }

                ui.add_space(12.0);
                ui.label("Optimization:");
                egui::ComboBox::from_id_salt("pass_selector")
                    .selected_text(self.selected_pass_type.name())
                    .width(320.0)
                    .show_ui(ui, |ui| {
                        ScrollArea::vertical()
                            .max_height(360.0)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                for pass in addable_optimization_passes() {
                                    ui.selectable_value(
                                        &mut self.selected_pass_type,
                                        pass.clone(),
                                        pass.name(),
                                    );
                                }
                            });
                    });

                if ui.button("Add Optimization").clicked() {
                    self.add_node_at_center(NodeType::Opt);
                    self.show_add_node_menu = false;
                }

                ui.add_space(8.0);
                if ui.button("Preview").clicked() {
                    self.add_node_at_center(NodeType::Preview);
                    self.show_add_node_menu = false;
                }
            });
        }
    }

    fn add_node_at_center(&mut self, node_type: NodeType) {
        let center_x = -self.camera_offset.x + 400.0;
        let center_y = -self.camera_offset.y + 300.0;
        let pos = NodePosition::new(center_x, center_y);

        let node: Box<dyn Node> = match node_type {
            NodeType::Input => Box::new(InputNode::with_position(pos.x, pos.y)),
            NodeType::Opt => Box::new(
                OptNode::for_pass(self.selected_pass_type.clone()).with_position(pos.x, pos.y),
            ),
            NodeType::IfMacro => Box::new(IfMacroNode::new().with_position(pos.x, pos.y)),
            NodeType::VariableMacro => {
                Box::new(VariableMacroNode::new().with_position(pos.x, pos.y))
            }
            NodeType::ArithmeticMacro => {
                Box::new(ArithmeticMacroNode::new().with_position(pos.x, pos.y))
            }
            NodeType::Preview => Box::new(PreviewNode::with_position(pos.x, pos.y)),
        };

        let id = node.id();
        self.graph.add_node(node);
        self.selected_node = Some(id);
        self.set_status(format!("Added {} node", node_type.name()));
    }

    fn add_pattern_matching_node_at_center(&mut self) {
        let center_x = -self.camera_offset.x + 400.0;
        let center_y = -self.camera_offset.y + 300.0;
        let node = Box::new(
            OptNode::for_pass(OptimizationPass::PatternMatching(Vec::new()))
                .with_position(center_x, center_y),
        );
        let id = node.id();
        self.graph.add_node(node);
        self.selected_node = Some(id);
        self.set_status("Added Pattern Matching node");
    }

    fn has_macro_nodes(&self) -> bool {
        self.graph
            .nodes()
            .iter()
            .any(|node| node.node_type().is_macro())
    }

    fn start_pipeline(&mut self) {
        if self.has_macro_nodes() {
            self.set_status("Macro nodes are not executable yet; remove them before running");
            return;
        }
        let Some(input_id) = self.active_input_node_id() else {
            self.set_status("No input node available");
            return;
        };
        let addresses = self.state.selected_addresses();
        if addresses.is_empty() {
            self.set_status("No sections selected");
            return;
        }

        // Clear all node caches
        for node in self.graph.nodes_mut() {
            if let Some(opt) = node.as_any_mut().downcast_mut::<OptNode>() {
                opt.output_ast = None;
                opt.output = None;
            }
            if let Some(prev) = node.as_any_mut().downcast_mut::<PreviewNode>() {
                prev.clear_snapshot();
            }
        }

        // Enqueue all OptNode IDs in graph order
        self.state.pending_optimize_queue.clear();
        for id in reachable_opt_nodes(&self.graph, input_id) {
            self.state.pending_optimize_queue.push_back(id);
        }
        self.active_pipeline_input = Some(input_id);

        // Send raw decompile request
        self.state.last_decompile_selection = addresses.clone();
        let selection_count = addresses.len();
        self.state.queue_request(WorkerRequest::DecompileSections(
            build_base_decompile_request(addresses),
        ));
        self.set_status(format!(
            "Decompiling {selection_count} selected section(s)..."
        ));
    }

    fn process_pipeline_results(&mut self) {
        // Handle completed raw decompile: fill leading PreviewNodes, start optimize queue
        if self.state.base_ast.is_some()
            && self.state.pending_target_node.is_none()
            && self.state.last_optimize_result.is_none()
        {
            self.fill_leading_previews();
            if !self.state.pending_optimize_queue.is_empty() {
                self.process_optimize_queue();
            } else {
                self.set_status_if_changed("Pipeline completed");
            }
        }

        // Handle completed OptimizeAst
        if let Some((node_id, result)) = self.state.last_optimize_result.take() {
            let ast = result.ast.clone();
            // Store in target OptNode
            if let Some(node) = self.graph.get_node_mut(node_id) {
                if let Some(opt) = node.as_any_mut().downcast_mut::<OptNode>() {
                    opt.output_ast = Some(result.ast.clone());
                    opt.output = Some(DecompileResult {
                        assembly: Vec::new(),
                        ir: Vec::new(),
                        ast: result.ast_lines,
                        ast_object: Some(result.ast),
                    });
                }
            }
            // Fill subsequent PreviewNodes
            self.fill_previews_after_node(node_id, &ast);
            // Continue queue
            self.process_optimize_queue();
        }
    }

    fn process_optimize_queue(&mut self) {
        if self.state.is_busy() {
            return;
        }
        let Some(node_id) = self.state.pending_optimize_queue.pop_front() else {
            self.set_status("Pipeline completed");
            return;
        };

        // Find input AST: from preceding OptNode or base_ast
        let input_ast = self.find_input_ast_for(node_id);
        let Some(ast) = input_ast else {
            self.set_status("No input AST available");
            self.state.pending_optimize_queue.clear();
            return;
        };

        // Get settings from this OptNode
        let (settings, buffer_script) = {
            let Some(node) = self.graph.get_node(node_id) else {
                self.set_status("OptNode not found");
                self.state.pending_optimize_queue.clear();
                return;
            };
            let Some(opt) = node.as_any().downcast_ref::<OptNode>() else {
                self.set_status("Node is not an OptNode");
                self.state.pending_optimize_queue.clear();
                return;
            };
            let buf = if opt.store.fb_script_enabled {
                opt.store.applied_buffer_script.clone()
            } else {
                None
            };
            (opt.store.applied_settings.clone(), buf)
        };

        let total_opt_nodes = self.graph.opt_node_ids().len();
        let optimizing_index =
            total_opt_nodes.saturating_sub(self.state.pending_optimize_queue.len());
        self.set_status(format!(
            "Optimizing Opt {optimizing_index}/{total_opt_nodes}..."
        ));
        self.state.pending_target_node = Some(node_id);
        self.state
            .queue_request(WorkerRequest::OptimizeAst(OptimizeAstRequest {
                ast: (*ast).clone(),
                settings,
                script_paths: vec![],
                buffer_script,
            }));
    }

    fn find_input_ast_for(&self, node_id: NodeId) -> Option<Arc<Ast>> {
        let source_id = resolve_input_source_node(&self.graph, node_id)?;
        let source = self.graph.get_node(source_id)?;
        if let Some(opt) = source.as_any().downcast_ref::<OptNode>() {
            return opt.output_ast.clone();
        }
        if source.as_any().downcast_ref::<InputNode>().is_some() {
            return self.state.base_ast.clone();
        }
        None
    }

    fn fill_leading_previews(&mut self) {
        let base_ast = self.state.base_ast.clone();
        let base_output = self.state.base_output.clone();
        let Some(input_id) = self.active_pipeline_input else {
            return;
        };
        if let Some(ast) = base_ast {
            let preview_ids = leading_preview_nodes(&self.graph, input_id);
            for preview_id in preview_ids {
                if let Some(node) = self.graph.get_node_mut(preview_id) {
                    if let Some(prev) = node.as_any_mut().downcast_mut::<PreviewNode>() {
                        prev.set_snapshot(ast.clone(), base_output.clone());
                    }
                }
            }
        }
    }

    fn fill_previews_after_node(&mut self, opt_node_id: NodeId, ast: &Arc<Ast>) {
        let connections: Vec<Connection> = self.graph.connections().to_vec();
        let mut to_fill: Vec<NodeId> = Vec::new();
        let mut frontier = vec![opt_node_id];

        while let Some(current) = frontier.pop() {
            for connection in &connections {
                if connection.from == current {
                    if let Some(node) = self.graph.get_node(connection.to) {
                        if node.as_any().downcast_ref::<PreviewNode>().is_some() {
                            to_fill.push(connection.to);
                            frontier.push(connection.to);
                        }
                        // Stop at OptNode boundaries
                    }
                }
            }
        }

        for id in to_fill {
            if let Some(node) = self.graph.get_node_mut(id) {
                if let Some(prev) = node.as_any_mut().downcast_mut::<PreviewNode>() {
                    prev.set_snapshot(ast.clone(), None);
                }
            }
        }
    }

    fn handle_graph_response(&mut self, response: GraphResponse) {
        self.camera_offset = response.camera_offset;
        self.zoom = response.zoom;
        self.dragged_node = response.dragged_node;
        self.selected_node = response.selected_node;
        self.connecting_from = response.connecting_from;

        if let Some(node_id) = response.deleted_node {
            self.remove_node(node_id);
            self.set_status("Node deleted");
        }

        if let Some(connection) = response.completed_connection {
            self.graph
                .add_connection(connection.from, connection.from_port, connection.to);
            self.connecting_from = None;
            self.set_status(format!(
                "Connected {:?}[{}] → {:?}",
                connection.from, connection.from_port, connection.to
            ));
        }

        if let Some(connection) = response.removed_connection {
            self.graph
                .remove_connection(connection.from, connection.from_port, connection.to);
            if self
                .connecting_from
                .is_some_and(|(node_id, _)| node_id == connection.from || node_id == connection.to)
            {
                self.connecting_from = None;
            }
            self.set_status(format!(
                "Removed connection {:?}[{}] → {:?}",
                connection.from, connection.from_port, connection.to
            ));
        }

        for (node_id, node_response) in response.node_responses {
            match node_response {
                NodeResponse::Deleted => {
                    self.remove_node(node_id);
                    self.set_status("Node deleted");
                }
                NodeResponse::DraggedTo { new_pos } => {
                    if let Some(node) = self.graph.get_node_mut(node_id) {
                        let node: &mut dyn Node = node;
                        node.set_position(new_pos);
                    }
                }
                NodeResponse::RunPipeline => {
                    if self.state.selected_addresses().is_empty() {
                        self.execute_pipeline();
                    } else {
                        self.start_pipeline();
                    }
                }
                NodeResponse::ToggleExpanded => self.toggle_node_expanded(node_id),
                _ => {}
            }
        }
    }

    fn render_perf_hud(&self, ctx: &egui::Context) {
        if !self.show_perf_hud {
            return;
        }

        let perf = self.perf_snapshot();

        egui::Area::new("perf-hud".into())
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-12.0, 58.0))
            .interactable(false)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new("Performance HUD").strong());
                    ui.label("Toggle: F12");
                    match perf {
                        Some((avg_ms, p95_ms, fps)) => {
                            ui.monospace(format!("avg: {:.2} ms  ({:.1} fps)", avg_ms, fps));
                            ui.monospace(format!("p95: {:.2} ms", p95_ms));
                        }
                        None => {
                            ui.monospace("avg: collecting...");
                            ui.monospace("p95: collecting...");
                        }
                    }
                    ui.separator();
                    ui.monospace(format!(
                        "pending worker jobs: {}",
                        self.state.pending_requests
                    ));
                    ui.monospace(format!("nodes: {}", self.graph.len()));
                    ui.monospace(format!("connections: {}", self.graph.connections().len()));
                });
            });
    }

    fn render_input_panel(&mut self, ctx: &egui::Context) {
        if !self.is_input_node_selected() {
            return;
        }

        let mut open_requested = false;
        let current_path = self
            .selected_input_node()
            .and_then(|node| node.file_path().cloned());
        let analyzed_count = self
            .state
            .known_sections
            .iter()
            .filter(|section| section.data.analyzed)
            .count();
        let selected_count = self.state.selected_addresses().len();
        let toggle_label = if analyzed_count > 0 && selected_count == analyzed_count {
            "Clear Selected"
        } else {
            "Select All"
        };

        egui::Panel::right("input-section-panel")
            .resizable(true)
            .default_size(300.0)
            .show(ctx, |ui| {
                ui.heading("Input");
                ui.add_space(8.0);

                if let Some(path) = current_path.as_ref() {
                    ui.label(path.display().to_string());
                } else {
                    ui.label("No binary file selected");
                }

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Open / Browse").clicked() {
                        open_requested = true;
                    }

                    if ui
                        .add_enabled(current_path.is_some(), egui::Button::new("Analyze All"))
                        .clicked()
                    {
                        self.state.analyze_all();
                        self.set_status("Analyzing all sections...");
                    }
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    let response = ui.text_edit_singleline(&mut self.state.analyze_target_address);
                    let submitted = response.lost_focus()
                        && ui.input(|input| input.key_pressed(egui::Key::Enter));
                    let clicked = ui.button("Analyze Address").clicked();
                    if submitted || clicked {
                        let address = self.state.analyze_target_address.clone();
                        self.state.analyze_section_from_address(&address);
                        if address.trim().is_empty() {
                            self.set_status("Analyzing entrypoint...");
                        } else {
                            self.set_status(format!("Analyzing address {address}..."));
                        }
                    }
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(analyzed_count > 0, egui::Button::new(toggle_label))
                        .clicked()
                    {
                        self.state.select_all();
                        self.set_status(if toggle_label == "Clear Selected" {
                            "Selection cleared"
                        } else {
                            "All analyzed sections selected"
                        });
                    }
                });

                ui.separator();
                ui.label(format!(
                    "{} known section(s), {} selected",
                    self.state.known_sections.len(),
                    selected_count
                ));
                ui.add_space(4.0);

                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.state.known_sections.is_empty() {
                            ui.label("Run Analyze All or Analyze Address to discover sections.");
                        } else {
                            let sections = self
                                .state
                                .known_sections
                                .iter()
                                .map(|section| {
                                    (
                                        section.data.start_address,
                                        section.data.end_address,
                                        section.data.analyzed,
                                        section.selected,
                                    )
                                })
                                .collect::<Vec<_>>();

                            for (start, end, analyzed, selected) in sections {
                                let mut next_selected = selected;
                                let range = end
                                    .map(|end| format!("0x{start:016X}..0x{end:016X}"))
                                    .unwrap_or_else(|| format!("0x{start:016X}"));
                                let label = if analyzed {
                                    range
                                } else {
                                    format!("{range} (pending)")
                                };
                                let response = ui.add_enabled(
                                    analyzed,
                                    egui::Checkbox::new(&mut next_selected, label),
                                );
                                if response.changed() {
                                    self.state.set_section_selected(start, next_selected);
                                }
                            }
                        }
                    });
            });

        if open_requested {
            if let Some(path) = self.state.open_file() {
                if let Some(node) = self.selected_input_node_mut() {
                    node.set_file_path(path.clone());
                }
                self.set_status(format!("Opening {}...", path.display()));
            }
        }
    }

    fn render_optimization_panel(&mut self, ctx: &egui::Context) {
        let Some(selected_node) = self.selected_node else {
            return;
        };
        if !self.is_pattern_matching_node_selected() {
            return;
        }

        let mut request_rerun = false;
        let mut next_status = None;

        egui::Panel::right("optimization-section-panel")
            .resizable(true)
            .default_size(360.0)
            .show(ctx, |ui| {
                let Some(node) = self
                    .graph
                    .get_node_mut(selected_node)
                    .and_then(|node| node.as_any_mut().downcast_mut::<OptNode>())
                else {
                    return;
                };

                let script_dirty =
                    node.store.editor_buffer != node.store.applied_buffer_script.clone().unwrap_or_default();

                ui.heading("Pattern Matching");
                ui.add_space(8.0);
                ui.label(node.name());
                ui.small(node.summary());
                if node.has_pending_changes() {
                    ui.small("Pending changes are not active until Apply is pressed.");
                }

                ui.separator();

                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if script_dirty {
                            ui.small(".fb Script *");
                        } else {
                            ui.small(".fb Script");
                        }
                        ui.small(
                            "The buffer script is applied together with the Pattern Matching node when Apply is pressed.",
                        );
                        ui.add_space(8.0);
                        ui.add(
                            egui::TextEdit::multiline(&mut node.store.editor_buffer)
                                .font(egui::TextStyle::Monospace)
                                .desired_rows(22)
                                .desired_width(f32::INFINITY),
                        );
                    });

                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Reset").clicked() {
                        node.reset_draft_changes();
                        next_status = Some("Optimization draft reset".to_string());
                    }

                    if ui
                        .add_enabled(node.has_pending_changes(), egui::Button::new("Apply"))
                        .clicked()
                    {
                        node.apply_changes();
                        request_rerun = true;
                        next_status =
                            Some("Optimization applied; rerunning pipeline...".to_string());
                    }
                });
            });

        if let Some(status) = next_status {
            self.set_status(status);
        }
        if request_rerun {
            if self.state.selected_addresses().is_empty() {
                self.execute_pipeline();
            } else {
                self.start_pipeline();
            }
        }
    }

    fn render_macro_editor_panel(&mut self, ctx: &egui::Context) {
        let Some(selected_node) = self.selected_node else {
            return;
        };
        if !self.is_macro_editor_selected() {
            return;
        }

        egui::Panel::right("macro-editor-panel")
            .resizable(true)
            .default_size(320.0)
            .show(ctx, |ui| {
                let Some(node) = self.graph.get_node_mut(selected_node) else {
                    return;
                };

                match node.node_type() {
                    NodeType::VariableMacro => {
                        let Some(var) = node.as_any_mut().downcast_mut::<VariableMacroNode>()
                        else {
                            return;
                        };
                        ui.heading("Var");
                        ui.add_space(8.0);
                        ui.small(var.summary());
                        ui.separator();
                        ui.label("Variable Name");
                        ui.text_edit_singleline(&mut var.variable);
                        ui.add_space(8.0);
                        ui.label("Initial Value");
                        ui.add(egui::DragValue::new(&mut var.initial_value));
                    }
                    NodeType::IfMacro => {
                        let Some(if_node) = node.as_any_mut().downcast_mut::<IfMacroNode>() else {
                            return;
                        };
                        ui.heading("If");
                        ui.add_space(8.0);
                        ui.small(if_node.summary());
                        ui.small("True = port 0, False = port 1");
                        ui.separator();
                        ui.label("Variable");
                        ui.text_edit_singleline(&mut if_node.variable);
                        ui.add_space(8.0);
                        ui.label("Comparison");
                        egui::ComboBox::from_id_salt(("if-macro-panel-op", if_node.id().0))
                            .selected_text(if_node.comparison.name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut if_node.comparison,
                                    MacroComparison::LessThan,
                                    "<",
                                );
                                ui.selectable_value(
                                    &mut if_node.comparison,
                                    MacroComparison::LessEqual,
                                    "<=",
                                );
                                ui.selectable_value(
                                    &mut if_node.comparison,
                                    MacroComparison::Equal,
                                    "==",
                                );
                                ui.selectable_value(
                                    &mut if_node.comparison,
                                    MacroComparison::NotEqual,
                                    "!=",
                                );
                                ui.selectable_value(
                                    &mut if_node.comparison,
                                    MacroComparison::GreaterEqual,
                                    ">=",
                                );
                                ui.selectable_value(
                                    &mut if_node.comparison,
                                    MacroComparison::GreaterThan,
                                    ">",
                                );
                            });
                        ui.add_space(8.0);
                        ui.label("Comparison Value");
                        ui.add(egui::DragValue::new(&mut if_node.value));
                    }
                    NodeType::ArithmeticMacro => {
                        let Some(op_node) = node.as_any_mut().downcast_mut::<ArithmeticMacroNode>()
                        else {
                            return;
                        };
                        ui.heading("Operation");
                        ui.add_space(8.0);
                        ui.small(op_node.summary());
                        ui.separator();
                        ui.label("Target Variable");
                        ui.text_edit_singleline(&mut op_node.target_variable);
                        ui.add_space(8.0);
                        ui.label("Operator");
                        egui::ComboBox::from_id_salt(("op-macro-panel-op", op_node.id().0))
                            .selected_text(op_node.operation.name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut op_node.operation,
                                    ArithmeticOperation::Set,
                                    "=",
                                );
                                ui.selectable_value(
                                    &mut op_node.operation,
                                    ArithmeticOperation::Add,
                                    "+=",
                                );
                                ui.selectable_value(
                                    &mut op_node.operation,
                                    ArithmeticOperation::Subtract,
                                    "-=",
                                );
                                ui.selectable_value(
                                    &mut op_node.operation,
                                    ArithmeticOperation::Multiply,
                                    "*=",
                                );
                            });
                        ui.add_space(8.0);
                        ui.label("Operand");
                        ui.add(egui::DragValue::new(&mut op_node.value));
                    }
                    _ => {}
                }
            });
    }

    fn render_preview_panel(&mut self, ctx: &egui::Context) {
        if !self.is_preview_node_selected() {
            return;
        }

        let (node_id, summary, code) = if let Some(node) = self.selected_preview_node() {
            (node.id(), node.summary(), node.rendered_code())
        } else {
            return;
        };

        egui::Panel::right("preview-section-panel")
            .resizable(true)
            .default_size(420.0)
            .show(ctx, |ui| {
                ui.heading("Preview");
                ui.add_space(8.0);
                ui.label(summary);
                ui.separator();

                ScrollArea::vertical()
                    .id_salt(("preview-panel", node_id.0))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if let Some(code) = code.as_deref() {
                            ui.monospace(code);
                        } else {
                            ui.label("No output available. Run the pipeline first.");
                        }
                    });
            });
    }

    fn render_about_window(&mut self, ctx: &egui::Context) {
        if !self.show_about {
            return;
        }

        egui::Window::new("About Firebat")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Firebat").strong().size(18.0));
                    ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                    ui.add_space(4.0);
                    ui.label("A GUI decompiler built on the Fireball engine.");
                    ui.label("Copyright (C) 2024 Eveheeero <xhve00000@gmail.com>");
                });
                ui.add_space(8.0);
                ui.separator();
                ui.label(RichText::new("License").strong());
                ui.label("GNU General Public License v2.0 (GPL-2.0-only)");
                ui.hyperlink_to("Source code", "https://github.com/Eveheeero/fireman");
                ui.add_space(8.0);
                ui.separator();
                ui.label(RichText::new("Third-party libraries").strong());
                ui.label("capstone-rs 0.14.0 — MIT");
                ui.hyperlink_to(
                    "capstone-rust/capstone-rs",
                    "https://github.com/capstone-rust/capstone-rs",
                );
                ui.label("Capstone Engine — BSD-3-Clause");
                ui.hyperlink_to(
                    "capstone-engine/capstone",
                    "https://github.com/capstone-engine/capstone",
                );
                #[cfg(feature = "unicorn")]
                {
                    ui.label("unicorn-engine 2.1.5 — GPL-2.0");
                    ui.hyperlink_to(
                        "unicorn-engine/unicorn",
                        "https://github.com/unicorn-engine/unicorn",
                    );
                }
                #[cfg(feature = "keystone")]
                {
                    ui.label("keystone-engine 0.1.0 — GPL-2.0");
                    ui.hyperlink_to(
                        "keystone-engine/keystone",
                        "https://github.com/keystone-engine/keystone",
                    );
                }
                ui.add_space(8.0);
                ui.vertical_centered(|ui| {
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
            });
    }

    fn render_logs_panel(&mut self, ctx: &egui::Context) {
        let panel_height = if self.state.log_expanded { 180.0 } else { 32.0 };
        egui::Panel::bottom("logs-panel")
            .resizable(true)
            .default_size(panel_height)
            .min_size(panel_height)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Logs").strong());
                    ui.label(format!("({})", self.state.logs.len()));
                    ui.separator();
                    if ui
                        .button(if self.state.log_expanded {
                            "Collapse"
                        } else {
                            "Expand"
                        })
                        .clicked()
                    {
                        self.state.log_expanded = !self.state.log_expanded;
                    }
                });

                if self.state.log_expanded {
                    ui.separator();
                    ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            if self.state.logs.is_empty() {
                                ui.monospace("No logs yet.");
                            } else {
                                for entry in &self.state.logs {
                                    ui.monospace(entry);
                                }
                            }
                        });
                }
            });
    }
}

impl eframe::App for FirebatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sync_system_theme(ctx);
        self.record_frame_timing();
        if ctx.input(|input| input.key_pressed(egui::Key::F12)) {
            self.show_perf_hud = !self.show_perf_hud;
        }

        self.state.poll_worker();
        if let Some(notice) = self.state.take_status_notice() {
            self.set_status(notice);
        }
        self.process_pipeline_results();
        if self.state.is_busy() {
            ctx.request_repaint_after(Duration::from_millis(16));
        }

        egui::Panel::top("top-toolbar")
            .exact_size(if self.show_add_node_menu { 80.0 } else { 50.0 })
            .show(ctx, |ui| {
                self.render_toolbar(ui);
            });

        if self.is_input_node_selected() {
            self.render_input_panel(ctx);
        } else if self.is_pattern_matching_node_selected() {
            self.render_optimization_panel(ctx);
        } else if self.is_macro_editor_selected() {
            self.render_macro_editor_panel(ctx);
        } else if self.is_preview_node_selected() {
            self.render_preview_panel(ctx);
        }
        self.render_logs_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            let connections: Vec<Connection> = self.graph.connections().to_vec();

            let graph_response = GraphCanvas::new(
                self.graph.nodes_mut(),
                &connections,
                self.selected_node,
                self.dragged_node,
                self.connecting_from,
            )
            .with_camera(self.camera_offset, self.zoom)
            .show(ui);

            self.handle_graph_response(graph_response);
        });

        self.render_perf_hud(ctx);
        self.render_about_window(ctx);
    }

    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Not used - we use update() instead for full control
    }
}

fn resolve_input_source_node(graph: &NodeGraph, node_id: NodeId) -> Option<NodeId> {
    graph
        .connections()
        .iter()
        .find(|connection| connection.to == node_id)
        .map(|connection| connection.from)
}

fn reachable_opt_nodes(graph: &NodeGraph, input_id: NodeId) -> Vec<NodeId> {
    let mut reachable = Vec::new();
    let mut visited = HashSet::new();
    let mut frontier = VecDeque::from([input_id]);

    while let Some(node_id) = frontier.pop_front() {
        if !visited.insert(node_id) {
            continue;
        }

        for target_id in outgoing_targets(graph.connections(), node_id) {
            let Some(node) = graph.get_node(target_id) else {
                continue;
            };
            if matches!(node.node_type(), NodeType::Opt) {
                reachable.push(target_id);
                frontier.push_back(target_id);
            }
        }
    }

    reachable
}

fn leading_preview_nodes(graph: &NodeGraph, input_id: NodeId) -> Vec<NodeId> {
    let mut previews = Vec::new();
    let mut visited = HashSet::new();
    let mut frontier = VecDeque::from([input_id]);

    while let Some(node_id) = frontier.pop_front() {
        if !visited.insert(node_id) {
            continue;
        }

        for target_id in outgoing_targets(graph.connections(), node_id) {
            let Some(node) = graph.get_node(target_id) else {
                continue;
            };
            if matches!(node.node_type(), NodeType::Preview) {
                previews.push(target_id);
            }
        }
    }

    previews
}

fn outgoing_targets(connections: &[Connection], from: NodeId) -> impl Iterator<Item = NodeId> + '_ {
    connections
        .iter()
        .filter(move |connection| connection.from == from)
        .map(|connection| connection.to)
}

fn selected_node_id(preset: &GraphPreset, id_map: &NodeIdMap) -> Option<NodeId> {
    preset
        .nodes
        .iter()
        .find(|node| node.kind == "InputNode")
        .and_then(|node| id_map.get(node.id))
}

fn first_input_path(preset: &GraphPreset) -> Option<PathBuf> {
    preset
        .nodes
        .iter()
        .find(|node| node.kind == "InputNode")
        .and_then(|node| node.data.get("file_path"))
        .and_then(|value| value.as_str())
        .map(PathBuf::from)
}

fn build_default_graph() -> NodeGraph {
    let mut graph = NodeGraph::new();
    let steps = default_graph_steps();
    let total_rows = steps.len().div_ceil(4);

    let input = Box::new(InputNode::with_position(40.0, 80.0));
    let input_id = input.id();
    graph.add_node(input);
    let mut previous_id = input_id;

    for (index, (label, pass)) in steps.into_iter().enumerate() {
        let row = index / 4;
        let column = index % 4;
        let x = 280.0 + (column as f32 * 260.0);
        let y = 80.0 + (row as f32 * 170.0);
        let node = Box::new(OptNode::for_pass(pass).with_name(label).with_position(x, y));
        let node_id = node.id();
        graph.add_node(node);
        graph.add_connection(previous_id, 0, node_id);
        previous_id = node_id;
    }

    let preview = Box::new(PreviewNode::with_position(
        40.0,
        80.0 + (total_rows as f32 * 170.0),
    ));
    let preview_id = preview.id();
    graph.add_node(preview);
    graph.add_connection(previous_id, 0, preview_id);

    graph
}

fn addable_optimization_passes() -> Vec<OptimizationPass> {
    vec![
        OptimizationPass::IrAnalyzation,
        OptimizationPass::ConstantFolding,
        OptimizationPass::ControlFlowCleanup,
        OptimizationPass::CopyPropagation,
        OptimizationPass::DeadStoreElimination,
        OptimizationPass::ExpressionInlining,
        OptimizationPass::LoopAnalysis,
        OptimizationPass::ParameterAnalysis,
        OptimizationPass::CallArgumentAnalysis,
        OptimizationPass::BooleanRecovery,
        OptimizationPass::SwitchReconstruction,
        OptimizationPass::TernaryRecovery,
        OptimizationPass::LifetimeScoping,
        OptimizationPass::SignednessInference,
        OptimizationPass::NameRecovery,
        OptimizationPass::EarlyReturnNormalization,
        OptimizationPass::CollapseUnusedVariable,
        OptimizationPass::OperatorCanonicalization,
        OptimizationPass::MagicDivisionRecovery,
        OptimizationPass::IdentitySimplification,
        OptimizationPass::BitTrickRecognition,
        OptimizationPass::CastMinimization,
        OptimizationPass::AssertionRecovery,
        OptimizationPass::DoWhileRecovery,
        OptimizationPass::ClampRecovery,
        OptimizationPass::LoopCleanup,
        OptimizationPass::IfConversionReversal,
        OptimizationPass::AntiDebugAstSuppression,
        OptimizationPass::LoggingSuppression,
        OptimizationPass::StaticGuardSuppression,
        OptimizationPass::SecurityScaffoldSuppression,
    ]
}

fn default_graph_steps() -> Vec<(String, OptimizationPass)> {
    let mut steps = vec![
        (
            "Pattern Matching Before IR".to_string(),
            OptimizationPass::PatternMatching(Vec::new()),
        ),
        (
            OptimizationPass::IrAnalyzation.name().to_string(),
            OptimizationPass::IrAnalyzation,
        ),
        (
            "Pattern Matching After IR".to_string(),
            OptimizationPass::PatternMatching(Vec::new()),
        ),
        (
            OptimizationPass::ParameterAnalysis.name().to_string(),
            OptimizationPass::ParameterAnalysis,
        ),
        (
            "Pattern Matching After Parameter".to_string(),
            OptimizationPass::PatternMatching(Vec::new()),
        ),
        (
            OptimizationPass::CallArgumentAnalysis.name().to_string(),
            OptimizationPass::CallArgumentAnalysis,
        ),
        (
            "Constant Folding After Call Args".to_string(),
            OptimizationPass::ConstantFolding,
        ),
        (
            "Pattern Matching After Call Args".to_string(),
            OptimizationPass::PatternMatching(Vec::new()),
        ),
        (
            OptimizationPass::SignednessInference.name().to_string(),
            OptimizationPass::SignednessInference,
        ),
    ];

    let iterative_passes = [
        OptimizationPass::OperatorCanonicalization,
        OptimizationPass::MagicDivisionRecovery,
        OptimizationPass::ConstantFolding,
        OptimizationPass::CopyPropagation,
        OptimizationPass::ExpressionInlining,
        OptimizationPass::LoopAnalysis,
        OptimizationPass::CollapseUnusedVariable,
        OptimizationPass::DeadStoreElimination,
        OptimizationPass::ControlFlowCleanup,
        OptimizationPass::BooleanRecovery,
        OptimizationPass::TernaryRecovery,
        OptimizationPass::AssertionRecovery,
        OptimizationPass::DoWhileRecovery,
        OptimizationPass::ClampRecovery,
        OptimizationPass::LoopCleanup,
        OptimizationPass::AntiDebugAstSuppression,
        OptimizationPass::LoggingSuppression,
        OptimizationPass::StaticGuardSuppression,
        OptimizationPass::SecurityScaffoldSuppression,
        OptimizationPass::IfConversionReversal,
        OptimizationPass::IdentitySimplification,
        OptimizationPass::BitTrickRecognition,
        OptimizationPass::CastMinimization,
    ];

    for iteration in 1..=3 {
        for pass in iterative_passes.iter().cloned() {
            steps.push((format!("{} Pass {}", pass.name(), iteration), pass));
        }
        steps.push((
            format!("Pattern Matching Pass {iteration}"),
            OptimizationPass::PatternMatching(Vec::new()),
        ));
    }

    steps.extend([
        (
            "Control Flow Cleanup Goto Containment".to_string(),
            OptimizationPass::ControlFlowCleanup,
        ),
        (
            "Loop Analysis Induction Variables".to_string(),
            OptimizationPass::LoopAnalysis,
        ),
        (
            OptimizationPass::SwitchReconstruction.name().to_string(),
            OptimizationPass::SwitchReconstruction,
        ),
        (
            OptimizationPass::EarlyReturnNormalization
                .name()
                .to_string(),
            OptimizationPass::EarlyReturnNormalization,
        ),
        (
            "Expression Inlining Temporary Elimination".to_string(),
            OptimizationPass::ExpressionInlining,
        ),
        (
            "Collapse Unused Variable Coalescing".to_string(),
            OptimizationPass::CollapseUnusedVariable,
        ),
        (
            OptimizationPass::LifetimeScoping.name().to_string(),
            OptimizationPass::LifetimeScoping,
        ),
        (
            OptimizationPass::NameRecovery.name().to_string(),
            OptimizationPass::NameRecovery,
        ),
        (
            "Pattern Matching After Optimization".to_string(),
            OptimizationPass::PatternMatching(Vec::new()),
        ),
    ]);

    steps
        .into_iter()
        .enumerate()
        .map(|(index, (label, pass))| (format!("{:02}. {label}", index + 1), pass))
        .collect()
}

fn build_base_decompile_request(addresses: Vec<u64>) -> DecompileRequest {
    DecompileRequest {
        start_addresses: addresses,
        settings: crate::model::OptimizationSettings::none(),
        script_paths: vec![],
        buffer_script: None,
    }
}

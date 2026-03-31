use super::state::FirebatState;
use crate::{
    node::{
        InputNode, Node, NodeGraph, NodeId, NodePosition, NodeResponse, NodeType, OptimizationNode,
        OptimizationPass, OutputNode,
    },
    pipeline::PipelineData,
    theme::configure_theme,
    ui::{GraphCanvas, GraphResponse},
};
use eframe::{
    egui,
    egui::{RichText, Vec2},
};
use std::{
    collections::VecDeque,
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
    pending_connection: Option<(NodeId, ConnectionPort)>, // Track clicked port for connection
}

#[derive(Clone, Copy, Debug)]
enum ConnectionPort {
    Input,
    Output,
}

impl Default for FirebatApp {
    fn default() -> Self {
        let mut graph = NodeGraph::new();

        // New simplified pipeline: Input → Output
        // Optimization nodes can be added between them
        let mut input = InputNode::new();
        input.set_position(NodePosition::new(50.0, 50.0));
        graph.add_node(Box::new(input));

        let mut output = OutputNode::new();
        output.set_position(NodePosition::new(500.0, 50.0));
        graph.add_node(Box::new(output));

        let input_id = graph.nodes()[0].id();
        let output_id = graph.nodes()[1].id();

        graph.add_connection(input_id, output_id);

        Self {
            graph,
            state: FirebatState::default(),
            show_perf_hud: false,
            show_about: false,
            show_add_node_menu: false,
            applied_dark_mode: None,
            last_frame_tick: None,
            frame_samples_ms: VecDeque::with_capacity(240),
            selected_node: None,
            dragged_node: None,
            camera_offset: Vec2::new(-20.0, -20.0),
            zoom: 1.0,
            status_message: None,
            status_timer: None,
            selected_pass_type: OptimizationPass::ConstantFolding,
            pending_connection: None,
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

    fn render_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Firebat");
            ui.add_space(16.0);

            let add_response = ui.button("+ Add Node");
            if add_response.clicked() {
                self.show_add_node_menu = !self.show_add_node_menu;
            }

            ui.add_space(8.0);

            let exec_response = ui.button("Execute Pipeline");
            if exec_response.clicked() {
                self.execute_pipeline();
            }

            ui.add_space(8.0);

            if ui.button("Clear Graph").clicked() {
                self.graph.clear();
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
                ui.label("Pass:");
                egui::ComboBox::from_id_salt("pass_selector")
                    .selected_text(self.selected_pass_type.name())
                    .width(200.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::ConstantFolding,
                            OptimizationPass::ConstantFolding.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::ControlFlowCleanup,
                            OptimizationPass::ControlFlowCleanup.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::CopyPropagation,
                            OptimizationPass::CopyPropagation.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::DeadStoreElimination,
                            OptimizationPass::DeadStoreElimination.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::ExpressionInlining,
                            OptimizationPass::ExpressionInlining.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::LoopAnalysis,
                            OptimizationPass::LoopAnalysis.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::ParameterAnalysis,
                            OptimizationPass::ParameterAnalysis.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::BooleanRecovery,
                            OptimizationPass::BooleanRecovery.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::SwitchReconstruction,
                            OptimizationPass::SwitchReconstruction.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::TernaryRecovery,
                            OptimizationPass::TernaryRecovery.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::NameRecovery,
                            OptimizationPass::NameRecovery.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::OperatorCanonicalization,
                            OptimizationPass::OperatorCanonicalization.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::MagicDivisionRecovery,
                            OptimizationPass::MagicDivisionRecovery.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::IdentitySimplification,
                            OptimizationPass::IdentitySimplification.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::BitTrickRecognition,
                            OptimizationPass::BitTrickRecognition.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::CastMinimization,
                            OptimizationPass::CastMinimization.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::AssertionRecovery,
                            OptimizationPass::AssertionRecovery.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::DoWhileRecovery,
                            OptimizationPass::DoWhileRecovery.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::ClampRecovery,
                            OptimizationPass::ClampRecovery.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::LoopCleanup,
                            OptimizationPass::LoopCleanup.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::IfConversionReversal,
                            OptimizationPass::IfConversionReversal.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::AntiDebugAstSuppression,
                            OptimizationPass::AntiDebugAstSuppression.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::LoggingSuppression,
                            OptimizationPass::LoggingSuppression.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::StaticGuardSuppression,
                            OptimizationPass::StaticGuardSuppression.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_pass_type,
                            OptimizationPass::SecurityScaffoldSuppression,
                            OptimizationPass::SecurityScaffoldSuppression.name(),
                        );
                    });

                if ui.button("Optimization").clicked() {
                    self.add_node_at_center(NodeType::Optimization(
                        self.selected_pass_type.clone(),
                    ));
                    self.show_add_node_menu = false;
                }

                ui.add_space(8.0);
                if ui.button("Output").clicked() {
                    self.add_node_at_center(NodeType::Output);
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
            NodeType::Optimization(ref pass) => Box::new(
                OptimizationNode::new(pass.clone(), self.graph.len() + 1)
                    .with_position(pos.x, pos.y),
            ),
            NodeType::Output => Box::new(OutputNode::with_position(pos.x, pos.y)),
        };

        let id = node.id();
        self.graph.add_node(node);
        self.selected_node = Some(id);
        self.set_status(format!("Added {} node", node_type.name()));
    }

    fn execute_pipeline(&mut self) {
        self.set_status("Executing pipeline...");

        match self.graph.execute() {
            Ok(data) => match data {
                PipelineData::Ast(ast) => {
                    let config = fireball::abstract_syntax_tree::AstPrintConfig::default();
                    let code = ast.print(Some(config));
                    let funcs = code
                        .lines()
                        .filter(|l| {
                            l.contains("void ") || l.contains("int ") || l.contains("func ")
                        })
                        .count();
                    self.set_status(format!("Pipeline completed: {} functions", funcs));

                    // Set the AST on all Output nodes so they can display it
                    let ast_arc = Arc::new((*ast).clone());
                    for node in self.graph.nodes_mut() {
                        if let Some(output_node) = node.as_any_mut().downcast_mut::<OutputNode>() {
                            output_node.set_ast(ast_arc.clone());
                        }
                    }
                }
                PipelineData::Empty => {
                    self.set_status("Pipeline completed (empty)");
                    // Clear AST from Output nodes
                    for node in self.graph.nodes_mut() {
                        if let Some(output_node) = node.as_any_mut().downcast_mut::<OutputNode>() {
                            output_node.clear_ast();
                        }
                    }
                }
            },
            Err(e) => {
                self.set_status(format!("Pipeline failed: {}", e));
            }
        }
    }

    fn handle_graph_response(&mut self, response: GraphResponse) {
        self.camera_offset = response.camera_offset;
        self.zoom = response.zoom;
        self.dragged_node = response.dragged_node;
        self.selected_node = response.selected_node;

        for (node_id, node_response) in response.node_responses {
            match node_response {
                NodeResponse::Deleted => {
                    self.graph.remove_node(node_id);
                    if self.selected_node == Some(node_id) {
                        self.selected_node = None;
                    }
                    self.set_status("Node deleted");
                }
                NodeResponse::DraggedTo { new_pos } => {
                    if let Some(node) = self.graph.get_node_mut(node_id) {
                        let node: &mut dyn Node = node;
                        node.set_position(new_pos);
                    }
                }
                NodeResponse::InputPortClicked => {
                    // Handle connection: Input port clicked
                    match self.pending_connection {
                        None => {
                            // First click - store this input port
                            self.pending_connection = Some((node_id, ConnectionPort::Input));
                            self.set_status("Click an output port to connect");
                        }
                        Some((pending_id, ConnectionPort::Output)) => {
                            // Second click - connect output to input
                            self.graph.add_connection(pending_id, node_id);
                            self.pending_connection = None;
                            self.set_status(format!("Connected {:?} → {:?}", pending_id, node_id));
                        }
                        Some((pending_id, ConnectionPort::Input)) => {
                            // Clicked another input port - switch to this one
                            self.pending_connection = Some((node_id, ConnectionPort::Input));
                            self.set_status("Click an output port to connect");
                        }
                    }
                }
                NodeResponse::OutputPortClicked => {
                    // Handle connection: Output port clicked
                    match self.pending_connection {
                        None => {
                            // First click - store this output port
                            self.pending_connection = Some((node_id, ConnectionPort::Output));
                            self.set_status("Click an input port to connect");
                        }
                        Some((pending_id, ConnectionPort::Input)) => {
                            // Second click - connect output to input
                            self.graph.add_connection(node_id, pending_id);
                            self.pending_connection = None;
                            self.set_status(format!("Connected {:?} → {:?}", node_id, pending_id));
                        }
                        Some((pending_id, ConnectionPort::Output)) => {
                            // Clicked another output port - switch to this one
                            self.pending_connection = Some((node_id, ConnectionPort::Output));
                            self.set_status("Click an input port to connect");
                        }
                    }
                }
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
}

impl eframe::App for FirebatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.sync_system_theme(ctx);
        self.record_frame_timing();
        if ctx.input(|input| input.key_pressed(egui::Key::F12)) {
            self.show_perf_hud = !self.show_perf_hud;
        }

        self.state.poll_worker();
        if self.state.is_busy() {
            ctx.request_repaint_after(Duration::from_millis(16));
        }

        egui::Panel::top("top-toolbar")
            .exact_size(if self.show_add_node_menu { 80.0 } else { 50.0 })
            .show(ctx, |ui| {
                self.render_toolbar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let connections: Vec<(NodeId, NodeId)> =
                self.graph.connections().iter().copied().collect();

            let graph_response = GraphCanvas::new(
                self.graph.nodes_mut(),
                &connections,
                self.selected_node,
                self.dragged_node,
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

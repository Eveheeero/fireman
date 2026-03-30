use super::{
    state::FirebatState,
    tabs::{FirebatTabViewer, PanelTab},
};
use crate::theme::configure_theme;
use eframe::{egui, egui::RichText};
use egui_dock::{DockArea, DockState, NodeIndex, Style as DockStyle};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

pub(crate) struct FirebatApp {
    dock_state: DockState<PanelTab>,
    state: FirebatState,
    show_perf_hud: bool,
    show_about: bool,
    applied_dark_mode: Option<bool>,
    last_frame_tick: Option<Instant>,
    frame_samples_ms: VecDeque<f32>,
}

impl Default for FirebatApp {
    fn default() -> Self {
        let mut dock_state = DockState::new(vec![PanelTab::Sections, PanelTab::Optimization]);
        {
            let surface = dock_state.main_surface_mut();
            let [left, right] =
                surface.split_right(NodeIndex::root(), 0.5, vec![PanelTab::Assembly]);
            surface.split_below(left, 0.5, vec![PanelTab::Ir]);
            surface.split_below(right, 0.5, vec![PanelTab::Ast]);
        }

        Self {
            dock_state,
            state: FirebatState::default(),
            show_perf_hud: false,
            show_about: false,
            applied_dark_mode: None,
            last_frame_tick: None,
            frame_samples_ms: VecDeque::with_capacity(240),
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

        let mut sum = 0.0_f32;
        for sample in &self.frame_samples_ms {
            sum += *sample;
        }
        let avg_ms = sum / self.frame_samples_ms.len() as f32;

        let mut sorted = self.frame_samples_ms.iter().copied().collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let p95_index = ((sorted.len() as f32 * 0.95).floor() as usize).min(sorted.len() - 1);
        let p95_ms = sorted[p95_index];
        let fps = if avg_ms > 0.0 { 1000.0 / avg_ms } else { 0.0 };
        Some((avg_ms, p95_ms, fps))
    }

    fn render_perf_hud(&self, ctx: &egui::Context) {
        if !self.show_perf_hud {
            return;
        }

        let perf = self.perf_snapshot();
        let assembly_len = self
            .state
            .decompile_result
            .as_ref()
            .map_or(0, |result| result.data.assembly.len());
        let ir_len = self
            .state
            .decompile_result
            .as_ref()
            .map_or(0, |result| result.data.ir.len());

        egui::Area::new("perf-hud".into())
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-12.0, 58.0))
            .interactable(false)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.label(RichText::new("Performance HUD").strong());
                    ui.label("Toggle: F12");
                    match perf {
                        Some((avg_ms, p95_ms, fps)) => {
                            ui.monospace(format!("avg: {avg_ms:.2} ms  ({fps:.1} fps)"));
                            ui.monospace(format!("p95: {p95_ms:.2} ms"));
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
                    ui.monospace(format!("sections: {}", self.state.known_sections.len()));
                    ui.monospace(format!("assembly rows: {assembly_len}"));
                    ui.monospace(format!("ir rows: {ir_len}"));
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

        egui::TopBottomPanel::top("top-nav")
            .exact_height(50.0)
            .show(ctx, |ui| {
                self.state.render_navigation(
                    ui,
                    &mut self.dock_state,
                    &mut self.show_perf_hud,
                    &mut self.show_about,
                );
            });

        let log_height = if self.state.log_expanded { 220.0 } else { 36.0 };
        egui::TopBottomPanel::bottom("log-bar")
            .exact_height(log_height)
            .show(ctx, |ui| {
                self.state.render_log_bar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.state.hover_candidate = None;
            let mut viewer = FirebatTabViewer {
                state: &mut self.state,
            };
            DockArea::new(&mut self.dock_state)
                .style(DockStyle::from_egui(ui.style().as_ref()))
                .show_inside(ui, &mut viewer);
            self.state.hovered_assembly_index = self.state.hover_candidate;
        });

        self.render_perf_hud(ctx);
        self.render_about_window(ctx);
    }
}

impl FirebatApp {
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
                ui.hyperlink_to(
                    "Source code",
                    "https://github.com/Eveheeero/fireman",
                );
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
                ui.label("unicorn-engine 2.1.5 — GPL-2.0");
                ui.hyperlink_to(
                    "unicorn-engine/unicorn",
                    "https://github.com/unicorn-engine/unicorn",
                );
                ui.label("keystone-engine 0.1.0 — GPL-2.0");
                ui.hyperlink_to(
                    "keystone-engine/keystone",
                    "https://github.com/keystone-engine/keystone",
                );
                ui.add_space(8.0);
                ui.vertical_centered(|ui| {
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
            });
    }
}

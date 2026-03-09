mod app;
pub mod backend;
mod core;
mod model;
mod theme;
mod worker;

use crate::{app::FirebatApp, theme::configure_theme};
use eframe::egui;

pub fn run() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([960.0, 640.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Firebat",
        native_options,
        Box::new(|creation_context| {
            let is_dark_mode = matches!(
                creation_context.egui_ctx.system_theme(),
                Some(egui::Theme::Dark)
            );
            configure_theme(&creation_context.egui_ctx, is_dark_mode);
            Ok(Box::new(FirebatApp::default()))
        }),
    );
}

pub use core::FirebatCore;
pub use model::*;
pub use worker::{FirebatWorker, WorkerRequest, WorkerResponse, WorkerTryRecv};

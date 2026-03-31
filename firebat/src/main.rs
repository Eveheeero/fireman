#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ast_editor;
mod core;
mod model;
mod node;
mod pipeline;
mod theme;
mod ui;
mod worker;

use crate::{app::FirebatApp, theme::configure_theme};
use eframe::egui;

fn main() {
    let icon_data = load_icon();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([960.0, 640.0])
            .with_icon(icon_data),
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

fn load_icon() -> egui::IconData {
    let icon_bytes = include_bytes!("../icons/icon.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon image")
        .into_rgba8();
    let (width, height) = image::GenericImageView::dimensions(&image);
    egui::IconData {
        rgba: image.into_raw(),
        width: width as u32,
        height: height as u32,
    }
}

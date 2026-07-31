#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;

use crate::gui::{BoardData, LogBarData};
use fireball::Fireball;
use std::sync::Arc;

const ICON_PNG: &[u8] = include_bytes!("../resources/icons/icon.png");
const FONT_REGULAR: &[u8] = include_bytes!("../resources/fonts/NotoSansCJKsc-Regular.otf");
const FONT_NAME: &str = "NotoSansCJKsc-Regular";

struct Firebat {
    fireball: Option<Fireball>,
    board: BoardData,
    log_bar: LogBarData,
}

fn main() {
    gui::init_tracing();
    let mut options = eframe::NativeOptions::default();
    options.persist_window = false;
    options.viewport.inner_size = Some(egui::vec2(800.0, 600.0));
    options.viewport.icon = load_icon();
    eframe::run_native(
        "Firebat",
        options,
        Box::new(|cc| Ok(Box::new(Firebat::new(cc)))),
    )
    .unwrap();
}

fn load_icon() -> Option<Arc<egui::IconData>> {
    let image = match image::load_from_memory(ICON_PNG) {
        Ok(image) => image.into_rgba8(),
        Err(e) => {
            tracing::warn!("failed to load application icon: {e}");
            return None;
        }
    };
    let (width, height) = image.dimensions();
    Some(Arc::new(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }))
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        FONT_NAME.to_owned(),
        Arc::new(egui::FontData::from_static(FONT_REGULAR)),
    );
    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        fonts
            .families
            .entry(family)
            .or_default()
            .insert(0, FONT_NAME.to_owned());
    }
    ctx.set_fonts(fonts);
}

impl Firebat {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);
        Self {
            fireball: None,
            board: BoardData::default(),
            log_bar: LogBarData::default(),
        }
    }
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;

use fireball::Fireball;
use gui::LogBarData;

struct Firebat {
    fireball: Option<Fireball>,
    log_bar: LogBarData,
}

fn main() {
    gui::init_tracing();
    let mut options = eframe::NativeOptions::default();
    options.persist_window = false;
    options.viewport.inner_size = Some(egui::vec2(800.0, 600.0));
    eframe::run_native(
        "Firebat",
        options,
        Box::new(|cc| Ok(Box::new(Firebat::new(cc)))),
    )
    .unwrap();
}

impl Firebat {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            fireball: None,
            log_bar: LogBarData::default(),
        }
    }
}

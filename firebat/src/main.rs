#![windows_subsystem = "windows"]
#![allow(unused_variables)]

use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Firebat",
        native_options,
        Box::new(|cc| Box::new(FirebatApp::new(cc))),
    );
}

#[derive(Default)]
struct FirebatApp {}

impl FirebatApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for FirebatApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

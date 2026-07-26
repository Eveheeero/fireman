mod board;
mod log_bar;
mod top_bar;

use crate::Firebat;
use eframe::egui;

impl eframe::App for Firebat {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        top_bar::ui(self, ui, frame);

        board::ui(self, ui, frame);

        log_bar::ui(self, ui, frame);
    }
}

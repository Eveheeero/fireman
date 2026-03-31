// Simplified renderer - currently just provides type definitions
// Full interactive rendering will be implemented in a future update
use eframe::egui::Ui;

pub struct AstRenderer;

impl AstRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn placeholder_render(&self, ui: &mut Ui, _text: &str) {
        // Simplified rendering - just show the text
        ui.monospace(_text);
    }
}

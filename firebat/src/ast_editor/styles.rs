use eframe::egui::{self, Color32, RichText};

pub struct AstStyles {
    pub keyword_color: Color32,
    pub type_color: Color32,
    pub variable_color: Color32,
    pub literal_color: Color32,
    pub operator_color: Color32,
    pub function_color: Color32,
    pub comment_color: Color32,
    pub selected_bg: Color32,
    pub hover_bg: Color32,
}

impl Default for AstStyles {
    fn default() -> Self {
        Self {
            keyword_color: Color32::from_rgb(0xC5, 0x86, 0xC0),
            type_color: Color32::from_rgb(0x8C, 0xBE, 0xD6),
            variable_color: Color32::from_rgb(0xD4, 0xD4, 0xD4),
            literal_color: Color32::from_rgb(0xB5, 0xCE, 0xA8),
            operator_color: Color32::from_rgb(0xD4, 0xD4, 0xD4),
            function_color: Color32::from_rgb(0xDC, 0xDC, 0xAA),
            comment_color: Color32::from_rgb(0x6A, 0x99, 0x55),
            selected_bg: Color32::from_rgb(0x04, 0x4E, 0x8A),
            hover_bg: Color32::from_rgb(0x2A, 0x2D, 0x2E),
        }
    }
}

impl AstStyles {
    pub fn keyword(&self, text: &str) -> RichText {
        RichText::new(text).color(self.keyword_color).monospace()
    }

    pub fn keyword_size(&self, text: &str, size: f32) -> RichText {
        RichText::new(text)
            .color(self.keyword_color)
            .monospace()
            .size(size)
    }

    pub fn variable(&self, text: &str) -> RichText {
        RichText::new(text).color(self.variable_color).monospace()
    }

    pub fn variable_size(&self, text: &str, size: f32) -> RichText {
        RichText::new(text)
            .color(self.variable_color)
            .monospace()
            .size(size)
    }

    pub fn literal(&self, text: &str) -> RichText {
        RichText::new(text).color(self.literal_color).monospace()
    }

    pub fn literal_size(&self, text: &str, size: f32) -> RichText {
        RichText::new(text)
            .color(self.literal_color)
            .monospace()
            .size(size)
    }

    pub fn operator(&self, text: &str) -> RichText {
        RichText::new(text).color(self.operator_color).monospace()
    }

    pub fn operator_size(&self, text: &str, size: f32) -> RichText {
        RichText::new(text)
            .color(self.operator_color)
            .monospace()
            .size(size)
    }

    pub fn function(&self, text: &str) -> RichText {
        RichText::new(text).color(self.function_color).monospace()
    }

    pub fn function_size(&self, text: &str, size: f32) -> RichText {
        RichText::new(text)
            .color(self.function_color)
            .monospace()
            .size(size)
    }

    pub fn type_name(&self, text: &str) -> RichText {
        RichText::new(text).color(self.type_color).monospace()
    }

    pub fn type_name_size(&self, text: &str, size: f32) -> RichText {
        RichText::new(text)
            .color(self.type_color)
            .monospace()
            .size(size)
    }

    pub fn comment(&self, text: &str) -> RichText {
        RichText::new(text).color(self.comment_color).monospace()
    }

    pub fn plain(&self, text: &str) -> RichText {
        RichText::new(text).monospace()
    }

    pub fn plain_size(&self, text: &str, size: f32) -> RichText {
        RichText::new(text).monospace().size(size)
    }
}

pub fn selectable_label(
    ui: &mut egui::Ui,
    text: RichText,
    is_selected: bool,
    styles: &AstStyles,
) -> egui::Response {
    let bg_color = if is_selected {
        Some(styles.selected_bg)
    } else {
        None
    };

    let response = ui.add(
        egui::Label::new(text)
            .selectable(false)
            .sense(egui::Sense::click()),
    );

    if let Some(bg) = bg_color {
        let rect = response.rect.expand2(egui::vec2(2.0, 0.0));
        ui.painter().rect_filled(rect, 0.0, bg);
    }

    response
}

pub fn hoverable_label(ui: &mut egui::Ui, text: RichText, styles: &AstStyles) -> egui::Response {
    let response = ui.add(
        egui::Label::new(text)
            .selectable(false)
            .sense(egui::Sense::click()),
    );

    if response.hovered() {
        let rect = response.rect.expand2(egui::vec2(2.0, 0.0));
        ui.painter().rect_filled(rect, 0.0, styles.hover_bg);
    }

    response
}

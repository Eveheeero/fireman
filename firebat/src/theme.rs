use eframe::egui::{self, Color32, Stroke};

pub(crate) fn configure_theme(ctx: &egui::Context, is_dark_mode: bool) {
    let mut visuals = if is_dark_mode {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    if is_dark_mode {
        visuals.panel_fill = Color32::from_rgb(32, 32, 32);
        visuals.window_fill = Color32::from_rgb(37, 37, 38);
        visuals.extreme_bg_color = Color32::from_rgb(44, 44, 46);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(50, 50, 52);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(61, 61, 64);
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(70, 70, 73));
        visuals.override_text_color = Some(Color32::from_rgb(235, 235, 235));
    } else {
        visuals.panel_fill = Color32::from_rgb(243, 242, 241);
        visuals.window_fill = Color32::from_rgb(243, 242, 241);
        visuals.extreme_bg_color = Color32::from_rgb(255, 255, 255);
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(250, 249, 248);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(237, 235, 233);
        visuals.widgets.noninteractive.bg_stroke =
            Stroke::new(1.0, Color32::from_rgb(210, 208, 206));
        visuals.override_text_color = Some(Color32::from_rgb(32, 31, 30));
    }
    visuals.selection.bg_fill = Color32::from_rgb(15, 108, 189);
    visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(15, 108, 189));
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(10.0, 5.0);
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(14.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(12.0, egui::FontFamily::Monospace),
    );
    ctx.set_style(style);
}

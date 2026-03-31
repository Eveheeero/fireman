use eframe::egui::{self, Color32, FontData, FontDefinitions, FontFamily, Stroke};
use std::sync::Arc;

/// Embedded NotoSansCJKsc-Regular font data (compile-time embedding)
const FONT_REGULAR: &[u8] = include_bytes!("../fonts/NotoSansCJKsc-Regular.otf");
/// Embedded NotoSansCJKsc-Bold font data (compile-time embedding)
const FONT_BOLD: &[u8] = include_bytes!("../fonts/NotoSansCJKsc-Bold.otf");

pub(crate) fn configure_theme(ctx: &egui::Context, is_dark_mode: bool) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "CJKFont".to_owned(),
        Arc::new(FontData::from_static(FONT_REGULAR)),
    );

    fonts.font_data.insert(
        "CJKFont-Bold".to_owned(),
        Arc::new(FontData::from_static(FONT_BOLD)),
    );

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "CJKFont".to_owned());
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "CJKFont".to_owned());

    ctx.set_fonts(fonts);

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

    let mut style = (*ctx.global_style()).clone();
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
    ctx.set_global_style(style);
}

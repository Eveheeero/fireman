use crate::{
    Firebat,
    gui::board::{default_tabs, select_target_block},
};
use eframe::egui;
use fireball::Fireball;

pub const TOP_BAR_HEIGHT: f32 = 26.0;
const MENU_PADDING_X: f32 = 16.0;
pub fn ui(app: &mut Firebat, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    egui::panel::Panel::top("top_bar")
        .exact_size(TOP_BAR_HEIGHT)
        .resizable(false)
        .frame(egui::Frame::NONE.fill(egui::Color32::BLACK))
        .show(ui, |ui| {
            style(ui);
            ui.horizontal_centered(|ui| {
                let response = menu_area(ui, "File");
                egui::Popup::menu(&response).show(|ui| {
                    if app.fireball.is_none() {
                        if ui.button("Open").clicked() {
                            ui.close();
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                app.fireball =
                                    Fireball::from_path(path.as_os_str().to_str().unwrap()).ok();
                                if app.fireball.is_some() {
                                    tracing::info!("opened {}", path.display());
                                    app.board.invalidate();
                                    let root_pos = egui::pos2(0.0, 0.0);
                                    app.board.add_window(select_target_block::window(root_pos));
                                    default_tabs::default_tabs(
                                        &mut app.board,
                                        select_target_block::WINDOW_ID,
                                        root_pos,
                                    );
                                } else {
                                    tracing::warn!("failed to open {}", path.display());
                                }
                            }
                        }
                    }
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                if menu_area(ui, "Decompile").clicked() {
                    app.board.decompile_requested = true;
                    tracing::debug!("decompilation requested");
                }
            });
        });
}

/// Draws a clickable area filling the whole bar height, without any button frame.
fn menu_area(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let font = egui::TextStyle::Body.resolve(ui.style());
    let galley = ui
        .painter()
        .layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE);
    let width = galley.size().x + MENU_PADDING_X * 2.0;
    let size = egui::vec2(width, ui.available_height());
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let fill = if response.is_pointer_button_down_on() {
            egui::Color32::from_gray(64)
        } else if response.hovered() {
            egui::Color32::from_gray(40)
        } else {
            egui::Color32::BLACK
        };
        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, fill);
        let pos = egui::pos2(
            rect.center().x - galley.size().x / 2.0,
            rect.center().y - galley.size().y / 2.0,
        );
        painter.galley(pos, galley, egui::Color32::WHITE);
    }

    response
}

fn style(ui: &mut egui::Ui) {
    let style = ui.style_mut();
    style.spacing.item_spacing = egui::vec2(0.0, 4.0);
    style.spacing.menu_margin = egui::Margin::same(6);
    for font in style.text_styles.values_mut() {
        *font = egui::FontId::proportional(18.0);
    }
}

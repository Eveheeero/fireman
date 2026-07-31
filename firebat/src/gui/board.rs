pub mod select_target_block;

use crate::Firebat;
use egui::emath::TSTransform;
use select_target_block::SelectTargetBlockData;
use std::collections::HashMap;

const ZOOM_RANGE: std::ops::RangeInclusive<f32> = 0.01..=4.0;

/// Zoom speed applied per scrolled point.
const ZOOM_SPEED: f32 = 0.005;

/// Size of the arrow head drawn in the middle of a connection.
const ARROW_SIZE: f32 = 10.0;

/// Shown while no window is open.
const EMPTY_HINT: &str = "No window is open, load a binary with File > Open.";

pub struct BoardData {
    scene_rect: egui::Rect,
    windows: Vec<BoardWindow>,
}

/// A single window living inside the board scene.
pub struct BoardWindow {
    /// Unique identifier of the window, since titles may repeat.
    id: String,
    title: String,
    /// Position of the window, in scene coordinates.
    pos: egui::Pos2,
    /// Identifiers of the windows this one points to.
    connected_to: Vec<String>,
    /// Whether the window shows a close button.
    closable: bool,
    /// Content of the window, holding its own state.
    kind: BoardWindowKind,
}

/// Every kind of window the board can show, with the data it owns.
pub enum BoardWindowKind {
    SelectTargetBlock(SelectTargetBlockData),
}

impl BoardWindow {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        pos: egui::Pos2,
        kind: BoardWindowKind,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            pos,
            connected_to: Vec::new(),
            closable: true,
            kind,
        }
    }

    pub fn connected_to(mut self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.connected_to = ids.into_iter().map(Into::into).collect();
        self
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

impl BoardWindowKind {
    fn ui(&mut self, app: &mut Firebat, ui: &mut egui::Ui) {
        match self {
            Self::SelectTargetBlock(data) => select_target_block::ui(app, data, ui),
        }
    }
}

impl BoardData {
    /// Adds a window, unless one with the same id is already open.
    pub fn add_window(&mut self, window: BoardWindow) {
        if self.windows.iter().any(|it| it.id == window.id) {
            return;
        }
        self.windows.push(window);
    }
}

impl Default for BoardData {
    fn default() -> Self {
        Self {
            scene_rect: egui::Rect::ZERO,
            windows: Vec::new(),
        }
    }
}

pub fn ui(app: &mut Firebat, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let outer_rect = ui.available_rect_before_wrap();
    if app.board.scene_rect.size() == egui::Vec2::ZERO {
        // Same size as the visible area, which means a zoom of exactly 1.
        app.board.scene_rect = egui::Rect::from_center_size(egui::Pos2::ZERO, outer_rect.size());
    }
    app.board.scene_rect = scroll_to_zoom(ui, outer_rect, app.board.scene_rect);

    let to_global = scene_transform(outer_rect, app.board.scene_rect);
    let mut scene_rect = app.board.scene_rect;
    let layers = egui::Scene::new()
        .zoom_range(ZOOM_RANGE)
        .show(ui, &mut scene_rect, |ui| {
            windows(app, ui, outer_rect, to_global)
        })
        .inner;
    app.board.scene_rect = scene_rect;

    // Panning is only known after the scene has been shown, so the windows are transformed again
    // to stay glued to the scene within the very same frame.
    let to_global = scene_transform(outer_rect, app.board.scene_rect);
    for layer in layers {
        ui.ctx().set_transform_layer(layer, to_global);
    }
}

/// Draws the scene bound windows, returning the layers they live on.
fn windows(
    app: &mut Firebat,
    ui: &mut egui::Ui,
    outer_rect: egui::Rect,
    to_global: TSTransform,
) -> Vec<egui::LayerId> {
    let scene_layer = ui.layer_id();
    let ctx = ui.ctx().clone();

    // The windows are taken out of the application, so that their content can borrow it mutably.
    let mut windows = std::mem::take(&mut app.board.windows);
    if windows.is_empty() {
        ui.label(EMPTY_HINT);
        return Vec::new();
    }

    let mut layers = Vec::with_capacity(windows.len());
    let mut rects = HashMap::with_capacity(windows.len());
    let mut closed = Vec::new();
    for it in windows.iter_mut() {
        let (layer, rect, open) = window(&ctx, scene_layer, outer_rect, to_global, app, it);
        layers.push(layer);
        rects.insert(it.id.clone(), rect);
        if !open {
            closed.push(it.id.clone());
        }
    }
    connections(ui, &windows, &rects);

    windows.retain(|it| !closed.contains(&it.id));
    windows.append(&mut app.board.windows);
    app.board.windows = windows;

    layers
}

/// Draws a single scene bound window, returning its layer, scene rect and whether it stays open.
fn window(
    ctx: &egui::Context,
    scene_layer: egui::LayerId,
    outer_rect: egui::Rect,
    to_global: TSTransform,
    app: &mut Firebat,
    window: &mut BoardWindow,
) -> (egui::LayerId, egui::Rect, bool) {
    let BoardWindow {
        id,
        title,
        pos,
        closable,
        kind,
        ..
    } = window;

    // A window lives on the middle order, which cannot become a sublayer of the background
    // ordered scene, so the window frame is drawn by hand inside an area.
    let area = egui::Area::new(egui::Id::new(("board_window", id.as_str())))
        .order(scene_layer.order)
        .fixed_pos(*pos)
        .constrain(false)
        .show(ctx, |ui| {
            // The area is clipped in screen coordinates, so the visible part of the scene has to
            // be mapped back into scene coordinates, otherwise the window gets cut off.
            ui.set_clip_rect(to_global.inverse() * outer_rect);
            egui::Frame::window(ui.style())
                .show(ui, |ui| {
                    let mut open = true;
                    ui.horizontal(|ui| {
                        let header = ui.add(
                            egui::Label::new(egui::RichText::new(title.as_str()).strong())
                                .sense(egui::Sense::drag())
                                .selectable(false),
                        );
                        *pos += header.drag_delta() / to_global.scaling;
                        if *closable {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("x").clicked() {
                                        open = false;
                                    }
                                },
                            );
                        }
                    });
                    ui.separator();
                    kind.ui(app, ui);
                    open
                })
                .inner
        });
    ctx.set_sublayer(scene_layer, area.response.layer_id);
    ctx.set_transform_layer(area.response.layer_id, to_global);
    (area.response.layer_id, area.response.rect, area.inner)
}

/// Draws an arrowed line between every connected window.
fn connections(ui: &egui::Ui, windows: &[BoardWindow], rects: &HashMap<String, egui::Rect>) {
    let painter = ui.painter();
    let stroke = egui::Stroke::new(1.5, ui.visuals().widgets.noninteractive.fg_stroke.color);

    for window in windows {
        let Some(from) = rects.get(&window.id) else {
            continue;
        };
        for target in &window.connected_to {
            let Some(to) = rects.get(target) else {
                continue;
            };
            let start = edge_point(*from, to.center());
            let end = edge_point(*to, from.center());
            painter.line_segment([start, end], stroke);

            let direction = (end - start).normalized();
            let tip = start + (end - start) / 2.0 + direction * ARROW_SIZE / 2.0;
            let normal = egui::vec2(-direction.y, direction.x);
            let base = tip - direction * ARROW_SIZE;
            painter.add(egui::Shape::convex_polygon(
                vec![
                    tip,
                    base + normal * ARROW_SIZE / 2.0,
                    base - normal * ARROW_SIZE / 2.0,
                ],
                stroke.color,
                egui::Stroke::NONE,
            ));
        }
    }
}

/// Point where a line towards the given position leaves the rect.
fn edge_point(rect: egui::Rect, towards: egui::Pos2) -> egui::Pos2 {
    let direction = towards - rect.center();
    if direction == egui::Vec2::ZERO {
        return rect.center();
    }
    let half = rect.size() / 2.0;
    let scale = (half.x / direction.x.abs()).min(half.y / direction.y.abs());
    rect.center() + direction * scale
}

/// Same transformation as the one [egui::Scene] applies to its own layer.
fn scene_transform(outer_rect: egui::Rect, scene_rect: egui::Rect) -> TSTransform {
    let scaling = scene_scale(outer_rect, scene_rect).clamp(*ZOOM_RANGE.start(), *ZOOM_RANGE.end());
    TSTransform::from_translation(
        outer_rect.center().to_vec2() - scaling * scene_rect.center().to_vec2(),
    ) * TSTransform::from_scaling(scaling)
}

fn scene_scale(outer_rect: egui::Rect, scene_rect: egui::Rect) -> f32 {
    (outer_rect.width() / scene_rect.width()).min(outer_rect.height() / scene_rect.height())
}

/// Zooms the scene in and out while scrolling, instead of panning it.
fn scroll_to_zoom(ui: &egui::Ui, outer_rect: egui::Rect, scene_rect: egui::Rect) -> egui::Rect {
    if scene_rect.size() == egui::Vec2::ZERO {
        return scene_rect;
    }
    let Some(pointer) = ui.input(|i| i.pointer.latest_pos()) else {
        return scene_rect;
    };
    if !outer_rect.contains(pointer) {
        return scene_rect;
    }
    let delta = ui.input_mut(|i| std::mem::take(&mut i.smooth_scroll_delta).y);
    if delta == 0.0 {
        return scene_rect;
    }

    let scale = scene_scale(outer_rect, scene_rect);
    let zoomed = (scale * (delta * ZOOM_SPEED).exp()).clamp(*ZOOM_RANGE.start(), *ZOOM_RANGE.end());
    let factor = scale / zoomed;
    let pointer_in_scene = scene_rect.center() + (pointer - outer_rect.center()) / scale;
    let center = pointer_in_scene + (scene_rect.center() - pointer_in_scene) * factor;

    egui::Rect::from_center_size(center, scene_rect.size() * factor)
}

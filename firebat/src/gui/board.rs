pub mod default_tabs;
pub mod display_current_ast;
pub mod select_optimization;
pub mod select_target_block;

use crate::Firebat;
use display_current_ast::DisplayCurrentAstData;
use egui::emath::TSTransform;
use fireball::{
    Fireball,
    abstract_syntax_tree::Ast,
    core::{Address, FireRaw},
};
use select_optimization::{SelectOptimizationChoice, SelectOptimizationData};
use select_target_block::SelectTargetBlockData;
use std::{
    collections::{HashMap, HashSet, VecDeque, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Ast owned by a window and reused by the ones connected to it.
pub type SharedAst = Arc<Ast>;

const ZOOM_RANGE: std::ops::RangeInclusive<f32> = 0.01..=4.0;

/// Zoom speed applied per scrolled point.
const ZOOM_SPEED: f32 = 0.005;

/// Size of the arrow head drawn in the middle of a connection.
const ARROW_SIZE: f32 = 10.0;

/// Distance between a window and the one spawned from it, in scene coordinates.
const SPAWN_OFFSET: egui::Vec2 = egui::vec2(320.0, 0.0);

/// Distance between two windows spawned from the same one, in scene coordinates.
const SPAWN_STEP: egui::Vec2 = egui::vec2(0.0, 120.0);

/// Shown while no window is open.
const EMPTY_HINT: &str = "No window is open, load a binary with File > Open.";

pub struct BoardData {
    scene_rect: egui::Rect,
    windows: Vec<BoardWindow>,
    pub pipeline: BoardPipeline,
    /// Raised by the top bar, consumed at the beginning of the next frame.
    pub decompile_requested: bool,
    /// Bumped whenever another binary is loaded, invalidating every cached ast.
    binary_generation: u64,
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
    /// Ast this window produced, reused by the windows it points to.
    ast: Option<SharedAst>,
    /// Inputs the currently held ast was built from, used to skip untouched windows.
    fingerprint: Option<BoardWindowFingerprint>,
    /// Content of the window, holding its own state.
    kind: BoardWindowKind,
}

/// Every kind of window the board can show, with the data it owns.
pub enum BoardWindowKind {
    SelectTargetBlock(SelectTargetBlockData),
    SelectOptimization(SelectOptimizationData),
    DisplayCurrentAst(DisplayCurrentAstData),
}

/// Inputs a window ast depends on, so an unchanged window can keep the ast it already holds.
#[derive(Clone, PartialEq, Eq)]
struct BoardWindowFingerprint {
    /// Identity of the ast inherited from the parent window, if any.
    parent: Option<usize>,
    /// Settings owned by the window itself.
    own: String,
}

/// Decompile sequence shared by the windows, mirroring the tui tab chain.
#[derive(Default)]
pub struct BoardPipeline {
    /// Amount of windows spawned so far, used to build unique identifiers.
    spawned: u64,
    blocks: Vec<u64>,
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
            ast: None,
            fingerprint: None,
            kind,
        }
    }

    pub fn closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        self
    }
}

impl BoardWindowKind {
    fn ui(&mut self, app: &mut Firebat, id: &str, ui: &mut egui::Ui) {
        match self {
            Self::SelectTargetBlock(data) => select_target_block::ui(app, data, ui),
            Self::SelectOptimization(data) => select_optimization::ui(app, id, data, ui),
            Self::DisplayCurrentAst(data) => display_current_ast::ui(app, id, data, ui),
        }
    }
}

impl BoardPipeline {
    /// Replaces the blocks the decompilation starts from.
    pub fn set_blocks(&mut self, blocks: Vec<u64>) {
        self.blocks = blocks;
    }

    /// Identifier for a window spawned from the given parent.
    fn spawn_id(&mut self, parent: &str, kind: &str) -> String {
        self.spawned += 1;
        format!("{parent}::{kind}::{}", self.spawned)
    }

    /// Builds the ast every chain of windows starts from.
    fn generate_ast(&self, fireball: Option<&Fireball>) -> Result<SharedAst, String> {
        let Some(fireball) = fireball else {
            return Err("no binary is loaded".to_owned());
        };
        if self.blocks.is_empty() {
            return Err("no block is selected".to_owned());
        }

        let sections = fireball.get_sections();
        let known = fireball.get_blocks();
        let mut targets = Vec::with_capacity(self.blocks.len());
        for address in &self.blocks {
            let address = Address::from_virtual_address(&sections, *address);
            let Some(block) = known.get_by_start_address(&address) else {
                continue;
            };
            targets.push(block);
        }

        fireball::ir::analyze::generate_ast_with_pre_defined_symbols(
            targets,
            fireball.get_defined(),
        )
        .map(Arc::new)
        .map_err(|error| format!("ast generation failed: {error:?}"))
    }

    /// Applies the given optimization on the ast of the parent window.
    fn optimize(
        &self,
        id: &str,
        choice: &SelectOptimizationChoice,
        ast: &SharedAst,
    ) -> Result<SharedAst, String> {
        let kind = select_optimization::choice_to_ast_optimization_kind(choice);
        ast.optimize(Some(kind.into()))
            .map(Arc::new)
            .map_err(|error| format!("optimization of {id} failed: {error:?}"))
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

    /// Points a window to another one, so the ast flows from the first to the second.
    fn connect(&mut self, from: &str, to: &str) {
        let Some(window) = self.windows.iter_mut().find(|it| it.id == from) else {
            return;
        };
        if window.connected_to.iter().any(|it| it == to) {
            return;
        }
        window.connected_to.push(to.to_owned());
    }

    /// Marks every cached ast as stale, since the binary they were built from changed.
    pub fn invalidate(&mut self) {
        self.binary_generation += 1;
    }

    /// Rebuilds the ast of the windows whose inputs changed, keeping the ones already up to date.
    fn decompile(&mut self, fireball: Option<&Fireball>) {
        let mut errors: HashMap<String, String> = HashMap::with_capacity(self.windows.len());

        for id in self.propagation_order() {
            let parent = self.parent_of(&id);
            let inherited = parent.map(|it| (it.id.clone(), it.ast.clone()));
            let (parent_id, parent_ast) = match inherited {
                Some((parent_id, ast)) => (Some(parent_id), ast),
                None => (None, None),
            };
            let choice = self.optimization_of(&id);
            let Some(index) = self.windows.iter().position(|it| it.id == id) else {
                continue;
            };

            let fingerprint = BoardWindowFingerprint {
                parent: parent_ast.as_ref().map(|it| Arc::as_ptr(it) as usize),
                own: match &self.windows[index].kind {
                    BoardWindowKind::SelectTargetBlock(_) => {
                        format!("{}:{:?}", self.binary_generation, self.pipeline.blocks)
                    }
                    BoardWindowKind::SelectOptimization(_) => match &choice {
                        Some(choice) => choice_fingerprint(choice),
                        None => String::new(),
                    },
                    BoardWindowKind::DisplayCurrentAst(_) => String::new(),
                },
            };

            // A window whose parent ast and own settings are untouched keeps the ast it holds,
            // which in turn keeps the identity its children compare themselves against.
            if self.windows[index].ast.is_some()
                && self.windows[index].fingerprint.as_ref() == Some(&fingerprint)
            {
                tracing::debug!("skipping {id}, its inputs did not change");
                continue;
            }

            let pipeline = &self.pipeline;
            let window = &mut self.windows[index];
            window.ast = None;
            window.fingerprint = None;
            tracing::debug!("decompiling {id} after {parent_id:?}");
            let result = match &mut window.kind {
                BoardWindowKind::SelectTargetBlock(_) => pipeline.generate_ast(fireball),
                BoardWindowKind::SelectOptimization(_) => match (&parent_ast, &choice) {
                    // A failing optimization is not fatal, the window simply hands the ast it
                    // received over to the windows below it.
                    (Some(ast), Some(choice)) => Ok(match pipeline.optimize(&id, choice, ast) {
                        Ok(optimized) => optimized,
                        Err(error) => {
                            tracing::warn!("{error}, keeping the ast of the parent");
                            ast.clone()
                        }
                    }),
                    (Some(ast), None) => Ok(ast.clone()),
                    (None, _) => Err(missing_ast(parent_id.as_deref(), &errors)),
                },
                BoardWindowKind::DisplayCurrentAst(data) => match &parent_ast {
                    Some(ast) => {
                        data.set_ast(ast.print(None));
                        Ok(ast.clone())
                    }
                    None => {
                        let error = missing_ast(parent_id.as_deref(), &errors);
                        data.set_ast(error.clone());
                        Err(error)
                    }
                },
            };

            match result {
                Ok(ast) => {
                    window.ast = Some(ast);
                    window.fingerprint = Some(fingerprint);
                }
                Err(error) => {
                    tracing::warn!("{id}: {error}");
                    errors.insert(id, error);
                }
            }
        }
    }

    /// Windows ordered so that every one of them comes after the window it hangs from.
    fn propagation_order(&self) -> Vec<String> {
        let mut order = Vec::with_capacity(self.windows.len());
        let mut visited = HashSet::with_capacity(self.windows.len());
        let mut queue: VecDeque<String> = self
            .windows
            .iter()
            .filter(|it| self.parent_of(&it.id).is_none())
            .map(|it| it.id.clone())
            .collect();

        while let Some(id) = queue.pop_front() {
            if !visited.insert(id.clone()) {
                continue;
            }
            let Some(window) = self.windows.iter().find(|it| it.id == id) else {
                continue;
            };
            queue.extend(window.connected_to.iter().cloned());
            order.push(id);
        }

        order
    }

    /// Optimization picked by the given window, taken from the data it owns.
    fn optimization_of(&self, id: &str) -> Option<SelectOptimizationChoice> {
        let window = self.windows.iter().find(|it| it.id == id)?;
        match &window.kind {
            BoardWindowKind::SelectOptimization(data) => Some(data.choice().clone()),
            _ => None,
        }
    }

    /// Window the given one hangs from, if any.
    fn parent_of(&self, id: &str) -> Option<&BoardWindow> {
        self.windows
            .iter()
            .find(|it| it.connected_to.iter().any(|target| target == id))
    }
}

/// Settings of an optimization window, as a value which can be compared between frames.
fn choice_fingerprint(choice: &SelectOptimizationChoice) -> String {
    // The body of the pattern is part of the fingerprint, so editing the file the path points to
    // also rebuilds the ast, not only picking another path.
    let source = if choice.selected == select_optimization::CUSTOM_PATTERN_INDEX {
        choice.custom_pattern_source()
    } else {
        String::new()
    };
    format!(
        "{}:{}:{:x}",
        choice.selected,
        choice.custom_path,
        hash_of(&source)
    )
}

/// Stable hash of a pattern body, keeping the fingerprint small.
fn hash_of(source: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    source.hash(&mut hasher);
    hasher.finish()
}

/// Reason why a window could not inherit an ast.
fn missing_ast(parent: Option<&str>, errors: &HashMap<String, String>) -> String {
    let Some(parent) = parent else {
        return "no window feeds this one".to_owned();
    };
    match errors.get(parent) {
        Some(error) => error.clone(),
        None => format!("{parent} produced no ast"),
    }
}

impl Default for BoardData {
    fn default() -> Self {
        Self {
            scene_rect: egui::Rect::ZERO,
            windows: Vec::new(),
            pipeline: BoardPipeline::default(),
            decompile_requested: false,
            binary_generation: 0,
        }
    }
}

pub fn ui(app: &mut Firebat, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    if std::mem::take(&mut app.board.decompile_requested) {
        app.board.decompile(app.fireball.as_ref());
    }

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
    let mut spawned = Vec::new();
    for it in windows.iter_mut() {
        let output = window(&ctx, scene_layer, outer_rect, to_global, app, it);
        layers.push(output.layer);
        rects.insert(it.id.clone(), output.rect);
        if !output.open {
            closed.push(it.id.clone());
        }
        spawned.extend(output.spawned);
    }
    connections(ui, &windows, &rects);

    for id in &closed {
        let children = inherited_children(&windows, &closed, id);
        for parent in windows.iter_mut() {
            let Some(index) = parent.connected_to.iter().position(|it| it == id) else {
                continue;
            };
            parent.connected_to.remove(index);
            for child in &children {
                if child == &parent.id || parent.connected_to.contains(child) {
                    continue;
                }
                parent.connected_to.push(child.clone());
            }
        }
    }
    windows.retain(|it| !closed.contains(&it.id));
    for (parent, spawn) in spawned {
        if let Some(parent) = windows.iter_mut().find(|it| it.id == parent) {
            parent.connected_to.push(spawn.id.clone());
        }
        windows.push(spawn);
    }
    windows.append(&mut app.board.windows);
    app.board.windows = windows;

    layers
}

/// Collects the surviving windows a closed window was pointing at, so its parents can adopt them.
/// Closed children are traversed as well, which keeps chains of removals connected.
fn inherited_children(windows: &[BoardWindow], closed: &[String], id: &str) -> Vec<String> {
    let mut children = Vec::new();
    let mut visited = HashSet::from([id.to_owned()]);
    let mut queue = VecDeque::from([id.to_owned()]);
    while let Some(current) = queue.pop_front() {
        let Some(window) = windows.iter().find(|it| it.id == current) else {
            continue;
        };
        for child in &window.connected_to {
            if !visited.insert(child.clone()) {
                continue;
            }
            if closed.iter().any(|it| it == child) {
                queue.push_back(child.clone());
            } else {
                children.push(child.clone());
            }
        }
    }
    children
}

/// What a single window produced during a frame.
struct BoardWindowOutput {
    layer: egui::LayerId,
    rect: egui::Rect,
    open: bool,
    /// Windows created during the frame, along with the identifier of their parent.
    spawned: Vec<(String, BoardWindow)>,
}

/// Draws a single scene bound window.
fn window(
    ctx: &egui::Context,
    scene_layer: egui::LayerId,
    outer_rect: egui::Rect,
    to_global: TSTransform,
    app: &mut Firebat,
    window: &mut BoardWindow,
) -> BoardWindowOutput {
    let BoardWindow {
        id,
        title,
        pos,
        connected_to,
        closable,
        ast: _,
        fingerprint: _,
        kind,
    } = window;
    let mut open = true;
    let mut spawned = Vec::new();

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
            egui::Frame::window(ui.style()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    let header = ui.add(
                        egui::Label::new(egui::RichText::new(title.as_str()).strong())
                            .sense(egui::Sense::drag())
                            .selectable(false),
                    );
                    *pos += header.drag_delta() / to_global.scaling;
                    let spawn_pos = *pos + SPAWN_OFFSET + SPAWN_STEP * connected_to.len() as f32;
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        header_actions(
                            app,
                            ui,
                            id,
                            spawn_pos,
                            kind,
                            *closable,
                            &mut open,
                            &mut spawned,
                        );
                    });
                });
                ui.separator();
                kind.ui(app, id, ui);
            });
        });
    ctx.set_sublayer(scene_layer, area.response.layer_id);
    ctx.set_transform_layer(area.response.layer_id, to_global);

    BoardWindowOutput {
        layer: area.response.layer_id,
        rect: area.response.rect,
        open,
        spawned,
    }
}

/// Buttons drawn at the right side of the window header.
#[allow(clippy::too_many_arguments)]
fn header_actions(
    app: &mut Firebat,
    ui: &mut egui::Ui,
    id: &str,
    spawn_pos: egui::Pos2,
    kind: &BoardWindowKind,
    closable: bool,
    open: &mut bool,
    spawned: &mut Vec<(String, BoardWindow)>,
) {
    if closable && ui.small_button("x").clicked() {
        *open = false;
    }

    // Only the windows feeding the pipeline may spawn the next step of the chain.
    if matches!(
        kind,
        BoardWindowKind::SelectTargetBlock(_) | BoardWindowKind::SelectOptimization(_)
    ) {
        if ui.small_button("AST").clicked() {
            let spawn_id = app.board.pipeline.spawn_id(id, "ast");
            spawned.push((
                id.to_owned(),
                display_current_ast::window(spawn_id, spawn_pos),
            ));
        }
        if ui.small_button("Opt").clicked() {
            let spawn_id = app.board.pipeline.spawn_id(id, "optimization");
            spawned.push((
                id.to_owned(),
                select_optimization::window(spawn_id, spawn_pos),
            ));
        }
    }
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

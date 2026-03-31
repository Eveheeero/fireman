use crate::{
    node::{Node, NodeContext, NodeId, NodePosition, NodeResponse, NodeType},
    ui::ScratchBlockRenderer,
};
use egui::{Color32, Frame, Margin, Pos2, Rect, Response, Rounding, Sense, Stroke, Ui, Vec2};

/// 2D Graph Canvas for node-based editing
pub struct GraphCanvas<'a> {
    nodes: &'a mut [Box<dyn Node>],
    selected_node: Option<NodeId>,
    dragged_node: Option<NodeId>,
    connections: &'a [(NodeId, NodeId)],
    camera_offset: Vec2,
    zoom: f32,
}

impl<'a> GraphCanvas<'a> {
    pub fn new(
        nodes: &'a mut [Box<dyn Node>],
        connections: &'a [(NodeId, NodeId)],
        selected_node: Option<NodeId>,
        dragged_node: Option<NodeId>,
    ) -> Self {
        Self {
            nodes,
            connections,
            selected_node,
            dragged_node,
            camera_offset: Vec2::ZERO,
            zoom: 1.0,
        }
    }

    pub fn with_camera(mut self, offset: Vec2, zoom: f32) -> Self {
        self.camera_offset = offset;
        self.zoom = zoom.max(0.1).min(5.0);
        self
    }

    pub fn show(self, ui: &mut Ui) -> GraphResponse {
        let available_size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(available_size, Sense::click_and_drag());

        // Fill background
        ui.painter().rect_filled(
            rect,
            0.0,
            Color32::from_rgb(30, 30, 35), // Dark background
        );

        // Draw grid
        self.draw_grid(ui, rect);

        // Process camera movement (pan)
        let mut new_camera_offset = self.camera_offset;
        if response.dragged_by(egui::PointerButton::Middle)
            || (response.dragged_by(egui::PointerButton::Primary)
                && ui.input(|i| i.key_down(egui::Key::Space)))
        {
            new_camera_offset += response.drag_delta();
        }

        // Calculate transform
        let to_screen = |pos: Pos2| -> Pos2 {
            Pos2::new(
                rect.min.x + (pos.x + new_camera_offset.x) * self.zoom,
                rect.min.y + (pos.y + new_camera_offset.y) * self.zoom,
            )
        };

        let from_screen = |pos: Pos2| -> Pos2 {
            Pos2::new(
                (pos.x - rect.min.x) / self.zoom - new_camera_offset.x,
                (pos.y - rect.min.y) / self.zoom - new_camera_offset.y,
            )
        };

        // Draw connections first (behind nodes)
        for (from_id, to_id) in self.connections {
            if let (Some(from_node), Some(to_node)) = (
                self.nodes.iter().find(|n| n.id() == *from_id),
                self.nodes.iter().find(|n| n.id() == *to_id),
            ) {
                let from_pos = to_screen(from_node.position().to_pos2());
                let to_pos = to_screen(to_node.position().to_pos2());
                let node_width = 380.0 * self.zoom;
                let center_y_offset = 50.0 * self.zoom;
                let port_offset = 10.0 * self.zoom;

                // Connect from output port (right side) to input port (left side)
                let from_pos = Pos2::new(
                    from_pos.x + node_width + port_offset, // Right edge + port offset
                    from_pos.y + center_y_offset,
                );
                let to_pos = Pos2::new(
                    to_pos.x - port_offset, // Left edge - port offset
                    to_pos.y + center_y_offset,
                );

                draw_connection(ui, from_pos, to_pos, to_node.color(), self.zoom);
            }
        }

        // Draw and interact with nodes
        let mut node_responses: Vec<(NodeId, NodeResponse)> = Vec::new();
        let mut new_dragged_node: Option<NodeId> = self.dragged_node;
        let mut new_selected_node: Option<NodeId> = self.selected_node;

        for node in self.nodes.iter_mut() {
            let node_pos = node.position().to_pos2();
            let screen_pos = to_screen(node_pos);
            // Node size: 380.0 width + 10.0 port on each side = 400.0 total
            let node_rect =
                Rect::from_min_size(screen_pos, Vec2::new(400.0 * self.zoom, 120.0 * self.zoom));

            // Check for drag
            let node_response = ui.interact(
                node_rect,
                ui.id().with(node.id().0),
                Sense::click_and_drag(),
            );

            if node_response.drag_started() {
                new_dragged_node = Some(node.id());
                new_selected_node = Some(node.id());
            }

            if node_response.dragged() {
                if let Some(dragged_id) = new_dragged_node {
                    if dragged_id == node.id() {
                        let delta = node_response.drag_delta() / self.zoom;
                        let new_pos = NodePosition::from_pos2(Pos2::new(
                            node_pos.x + delta.x,
                            node_pos.y + delta.y,
                        ));
                        node.set_position(new_pos);
                    }
                }
            }

            if node_response.drag_stopped() {
                new_dragged_node = None;
            }

            if node_response.clicked() {
                new_selected_node = Some(node.id());
            }

            // Draw node
            let ctx = NodeContext {
                is_selected: self.selected_node == Some(node.id()),
                is_dragging: self.dragged_node == Some(node.id()),
                can_delete: true,
            };

            // Render node at calculated position
            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(node_rect)
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );

            let block_response = ScratchBlockRenderer::render(&mut child_ui, node.as_mut(), &ctx);

            if block_response.inner != NodeResponse::None {
                node_responses.push((node.id(), block_response.inner));
            }
        }

        // Handle zoom with mouse wheel
        let zoom_delta = ui.input(|i| i.smooth_scroll_delta.y);
        if zoom_delta != 0.0
            && rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or(Pos2::ZERO)))
        {
            let new_zoom = (self.zoom + zoom_delta * 0.001).clamp(0.1, 5.0);
            // TODO: Zoom towards mouse pointer
        }

        GraphResponse {
            node_responses,
            camera_offset: new_camera_offset,
            zoom: self.zoom,
            dragged_node: new_dragged_node,
            selected_node: new_selected_node,
            canvas_rect: rect,
            canvas_response: response,
        }
    }

    fn draw_grid(&self, ui: &Ui, rect: Rect) {
        let grid_size = 50.0 * self.zoom;
        let offset_x = self.camera_offset.x * self.zoom % grid_size;
        let offset_y = self.camera_offset.y * self.zoom % grid_size;

        let grid_color = Color32::from_rgb(50, 50, 55);

        // Vertical lines
        let mut x = rect.min.x + offset_x;
        while x < rect.max.x {
            ui.painter().line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(1.0, grid_color),
            );
            x += grid_size;
        }

        // Horizontal lines
        let mut y = rect.min.y + offset_y;
        while y < rect.max.y {
            ui.painter().line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                Stroke::new(1.0, grid_color),
            );
            y += grid_size;
        }
    }
}

/// Response from graph canvas interaction
pub struct GraphResponse {
    pub node_responses: Vec<(NodeId, NodeResponse)>,
    pub camera_offset: Vec2,
    pub zoom: f32,
    pub dragged_node: Option<NodeId>,
    pub selected_node: Option<NodeId>,
    pub canvas_rect: Rect,
    pub canvas_response: Response,
}

/// Draw a curved connection line between two points
fn draw_connection(ui: &Ui, from: Pos2, to: Pos2, color: Color32, zoom: f32) {
    let painter = ui.painter();

    // Control points for bezier curve
    let control_offset = ((to.x - from.x) / 2.0).abs().max(50.0 * zoom);
    let control_from = Pos2::new(from.x + control_offset, from.y);
    let control_to = Pos2::new(to.x - control_offset, to.y);

    // Generate points along bezier curve
    let points: Vec<Pos2> = (0..=20)
        .map(|i| {
            let t = i as f32 / 20.0;
            let t2 = t * t;
            let t3 = t2 * t;
            let mt = 1.0 - t;
            let mt2 = mt * mt;
            let mt3 = mt2 * mt;

            Pos2::new(
                mt3 * from.x
                    + 3.0 * mt2 * t * control_from.x
                    + 3.0 * mt * t2 * control_to.x
                    + t3 * to.x,
                mt3 * from.y
                    + 3.0 * mt2 * t * control_from.y
                    + 3.0 * mt * t2 * control_to.y
                    + t3 * to.y,
            )
        })
        .collect();

    painter.add(egui::Shape::line(points, Stroke::new(3.0 * zoom, color)));

    // Draw connection dots
    painter.circle_filled(from, 6.0 * zoom, color);
    painter.circle_filled(to, 6.0 * zoom, color);
}

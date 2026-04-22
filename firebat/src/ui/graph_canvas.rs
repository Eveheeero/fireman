use crate::{
    node::{Node, NodeContext, NodeId, NodePosition, NodeResponse},
    ui::ScratchBlockRenderer,
};
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

#[derive(Clone, Copy, Debug)]
struct NodeLayout {
    node_id: NodeId,
    rect: Rect,
    input_port_pos: Option<Pos2>,
    output_port_pos: Option<Pos2>,
    color: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ConnectionEndpoints {
    from: Pos2,
    to: Pos2,
}

#[derive(Clone, Copy, Debug)]
struct ConnectionVisual {
    from_id: NodeId,
    to_id: NodeId,
    from: Pos2,
    to: Pos2,
    color: Color32,
}

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

        // Draw and interact with nodes
        let mut node_responses: Vec<(NodeId, NodeResponse)> = Vec::new();
        let mut new_dragged_node: Option<NodeId> = self.dragged_node;
        let mut new_selected_node: Option<NodeId> = self.selected_node;
        let mut node_layouts = Vec::with_capacity(self.nodes.len());
        let pointer_pos = ui.input(|input| input.pointer.interact_pos());

        for node in self.nodes.iter_mut() {
            let node_pos = node.position().to_pos2();
            let screen_pos = to_screen(node_pos);
            let max_node_rect =
                Rect::from_min_size(screen_pos, Vec2::new(420.0 * self.zoom, 720.0 * self.zoom));

            // Draw node
            let ctx = NodeContext {
                is_selected: self.selected_node == Some(node.id()),
                is_dragging: self.dragged_node == Some(node.id()),
                can_delete: true,
            };

            // Render node at calculated position
            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(max_node_rect)
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );

            let block_response = ScratchBlockRenderer::render(&mut child_ui, node.as_mut(), &ctx);
            let interactive_rect = block_response.rect.expand2(Vec2::new(20.0, 8.0));
            let node_response = ui.interact(
                interactive_rect,
                ui.id().with(("node", node.id().0)),
                Sense::click_and_drag(),
            );

            if node_response.drag_started() {
                new_dragged_node = Some(node.id());
                new_selected_node = Some(node.id());
            }

            if node_response.dragged() && new_dragged_node == Some(node.id()) {
                let delta = node_response.drag_delta() / self.zoom;
                let new_pos =
                    NodePosition::from_pos2(Pos2::new(node_pos.x + delta.x, node_pos.y + delta.y));
                node.set_position(new_pos);
            }

            if node_response.drag_stopped() {
                new_dragged_node = None;
            }

            if node_response.clicked() {
                new_selected_node = Some(node.id());
            }

            if block_response.inner != NodeResponse::None {
                node_responses.push((node.id(), block_response.inner));
            }

            node_layouts.push(NodeLayout {
                node_id: node.id(),
                rect: block_response.rect,
                input_port_pos: block_response.input_port_pos,
                output_port_pos: block_response.output_port_pos,
                color: node.color(),
            });
        }

        let connection_visuals = collect_connection_visuals(self.connections, &node_layouts);
        for visual in &connection_visuals {
            draw_connection(ui, visual.from, visual.to, visual.color, self.zoom);
        }

        let pointer_over_node = pointer_pos.is_some_and(|pointer| {
            node_layouts
                .iter()
                .any(|layout| layout.rect.expand2(Vec2::new(20.0, 8.0)).contains(pointer))
        });
        let removed_connection = if response.clicked() && !pointer_over_node {
            pointer_pos.and_then(|pointer| {
                find_connection_at_point(&connection_visuals, pointer, 10.0 * self.zoom)
            })
        } else {
            None
        };

        // Handle zoom with mouse wheel
        let zoom_delta = ui.input(|i| i.smooth_scroll_delta.y);
        if zoom_delta != 0.0
            && rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or(Pos2::ZERO)))
        {
            let _new_zoom = (self.zoom + zoom_delta * 0.001).clamp(0.1, 5.0);
            // TODO: Zoom towards mouse pointer
        }

        GraphResponse {
            node_responses,
            camera_offset: new_camera_offset,
            zoom: self.zoom,
            dragged_node: new_dragged_node,
            selected_node: new_selected_node,
            removed_connection,
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
    pub removed_connection: Option<(NodeId, NodeId)>,
    pub canvas_rect: Rect,
    pub canvas_response: Response,
}

/// Draw a curved connection line between two points
fn draw_connection(ui: &Ui, from: Pos2, to: Pos2, color: Color32, zoom: f32) {
    let painter = ui.painter();
    let points = connection_curve_points(from, to);

    painter.add(egui::Shape::line(points, Stroke::new(3.0 * zoom, color)));

    // Draw connection dots
    painter.circle_filled(from, 6.0 * zoom, color);
    painter.circle_filled(to, 6.0 * zoom, color);
}

fn collect_connection_visuals(
    connections: &[(NodeId, NodeId)],
    node_layouts: &[NodeLayout],
) -> Vec<ConnectionVisual> {
    connections
        .iter()
        .filter_map(|(from_id, to_id)| {
            let from_layout = node_layouts
                .iter()
                .find(|layout| layout.node_id == *from_id)?;
            let to_layout = node_layouts
                .iter()
                .find(|layout| layout.node_id == *to_id)?;
            let endpoints = connection_endpoints(from_layout, to_layout)?;

            Some(ConnectionVisual {
                from_id: *from_id,
                to_id: *to_id,
                from: endpoints.from,
                to: endpoints.to,
                color: to_layout.color,
            })
        })
        .collect()
}

fn connection_endpoints(
    from_layout: &NodeLayout,
    to_layout: &NodeLayout,
) -> Option<ConnectionEndpoints> {
    Some(ConnectionEndpoints {
        from: from_layout.output_port_pos?,
        to: to_layout.input_port_pos?,
    })
}

fn connection_curve_points(from: Pos2, to: Pos2) -> Vec<Pos2> {
    let control_offset = ((to.x - from.x) / 2.0).abs().max(50.0);
    let control_from = Pos2::new(from.x + control_offset, from.y);
    let control_to = Pos2::new(to.x - control_offset, to.y);

    (0..=20)
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
        .collect()
}

fn find_connection_at_point(
    connections: &[ConnectionVisual],
    point: Pos2,
    threshold: f32,
) -> Option<(NodeId, NodeId)> {
    connections
        .iter()
        .filter_map(|connection| {
            let distance = connection_hit_distance(
                point,
                &connection_curve_points(connection.from, connection.to),
            );
            (distance <= threshold).then_some((distance, (connection.from_id, connection.to_id)))
        })
        .min_by(|(left, _), (right, _)| left.total_cmp(right))
        .map(|(_, ids)| ids)
}

fn connection_hit_distance(point: Pos2, points: &[Pos2]) -> f32 {
    points
        .windows(2)
        .map(|segment| distance_to_segment(point, segment[0], segment[1]))
        .fold(f32::INFINITY, f32::min)
}

fn distance_to_segment(point: Pos2, start: Pos2, end: Pos2) -> f32 {
    let segment = end - start;
    let segment_len_sq = segment.length_sq();
    if segment_len_sq <= f32::EPSILON {
        return point.distance(start);
    }

    let point_vec = point - start;
    let t = (point_vec.dot(segment) / segment_len_sq).clamp(0.0, 1.0);
    let projection = start + segment * t;
    point.distance(projection)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::NodeId;

    #[test]
    fn connection_endpoints_follow_reported_port_positions() {
        let from_layout = NodeLayout {
            node_id: NodeId(1),
            rect: Rect::from_min_size(Pos2::new(20.0, 40.0), Vec2::new(380.0, 180.0)),
            input_port_pos: None,
            output_port_pos: Some(Pos2::new(410.0, 156.0)),
            color: Color32::WHITE,
        };
        let to_layout = NodeLayout {
            node_id: NodeId(2),
            rect: Rect::from_min_size(Pos2::new(520.0, 120.0), Vec2::new(380.0, 90.0)),
            input_port_pos: Some(Pos2::new(510.0, 172.0)),
            output_port_pos: None,
            color: Color32::WHITE,
        };

        let endpoints = connection_endpoints(&from_layout, &to_layout).expect("ports should exist");

        assert_eq!(endpoints.from, Pos2::new(410.0, 156.0));
        assert_eq!(endpoints.to, Pos2::new(510.0, 172.0));
    }

    #[test]
    fn hit_testing_finds_nearby_connection_and_ignores_distant_points() {
        let connections = [ConnectionVisual {
            from_id: NodeId(1),
            to_id: NodeId(2),
            from: Pos2::new(20.0, 40.0),
            to: Pos2::new(220.0, 40.0),
            color: Color32::WHITE,
        }];

        assert_eq!(
            find_connection_at_point(&connections, Pos2::new(120.0, 44.0), 8.0),
            Some((NodeId(1), NodeId(2)))
        );
        assert_eq!(
            find_connection_at_point(&connections, Pos2::new(120.0, 80.0), 8.0),
            None
        );
    }
}

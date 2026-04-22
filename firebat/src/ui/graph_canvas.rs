use crate::{
    node::{Connection, Node, NodeContext, NodeId, NodePosition, NodeResponse},
    ui::{OutputPortAnchor, ScratchBlockRenderer},
};
use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

#[derive(Clone, Debug)]
struct NodeLayout {
    node_id: NodeId,
    rect: Rect,
    input_port_pos: Option<Pos2>,
    output_ports: Vec<OutputPortAnchor>,
    color: Color32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ConnectionEndpoints {
    from: Pos2,
    to: Pos2,
}

#[derive(Clone, Copy, Debug)]
struct ConnectionVisual {
    connection: Connection,
    from: Pos2,
    to: Pos2,
    color: Color32,
}

/// 2D Graph Canvas for node-based editing
pub struct GraphCanvas<'a> {
    nodes: &'a mut [Box<dyn Node>],
    selected_node: Option<NodeId>,
    dragged_node: Option<NodeId>,
    connecting_from: Option<(NodeId, usize)>,
    connections: &'a [Connection],
    camera_offset: Vec2,
    zoom: f32,
}

impl<'a> GraphCanvas<'a> {
    pub fn new(
        nodes: &'a mut [Box<dyn Node>],
        connections: &'a [Connection],
        selected_node: Option<NodeId>,
        dragged_node: Option<NodeId>,
        connecting_from: Option<(NodeId, usize)>,
    ) -> Self {
        Self {
            nodes,
            connections,
            selected_node,
            dragged_node,
            connecting_from,
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

        ui.painter()
            .rect_filled(rect, 0.0, Color32::from_rgb(30, 30, 35));
        self.draw_grid(ui, rect);

        let mut new_camera_offset = self.camera_offset;
        if response.dragged_by(egui::PointerButton::Middle)
            || (response.dragged_by(egui::PointerButton::Primary)
                && ui.input(|i| i.key_down(egui::Key::Space)))
        {
            new_camera_offset += response.drag_delta() / self.zoom;
        }

        let to_screen = |pos: Pos2| -> Pos2 {
            Pos2::new(
                rect.min.x + (pos.x + new_camera_offset.x) * self.zoom,
                rect.min.y + (pos.y + new_camera_offset.y) * self.zoom,
            )
        };

        let mut node_responses: Vec<(NodeId, NodeResponse)> = Vec::new();
        let mut new_dragged_node = self.dragged_node;
        let mut new_selected_node = self.selected_node;
        let mut new_connecting_from = self.connecting_from;
        let mut node_layouts = Vec::with_capacity(self.nodes.len());
        let pointer_pos = ui.input(|input| input.pointer.interact_pos());

        for node in self.nodes.iter_mut() {
            let node_pos = node.position().to_pos2();
            let screen_pos = to_screen(node_pos);
            let max_node_rect =
                Rect::from_min_size(screen_pos, Vec2::new(420.0 * self.zoom, 720.0 * self.zoom));

            let ctx = NodeContext {
                is_selected: self.selected_node == Some(node.id()),
                is_dragging: self.dragged_node == Some(node.id()),
                can_delete: true,
            };

            let mut child_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(max_node_rect)
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );

            let block_response =
                ScratchBlockRenderer::render(&mut child_ui, node.as_mut(), &ctx, self.zoom);
            let interactive_rect = block_response
                .rect
                .expand2(Vec2::new(20.0 * self.zoom, 8.0 * self.zoom));
            let node_response = ui.interact(
                interactive_rect,
                ui.id().with(("node", node.id().0)),
                Sense::click_and_drag(),
            );

            let pointer_over_input_port =
                pointer_hits_port(pointer_pos, block_response.input_port_pos, 14.0 * self.zoom);
            let pointer_over_output_port = hovered_output_port(
                pointer_pos,
                &block_response.output_port_positions,
                14.0 * self.zoom,
            );
            let port_interaction = block_response.input_port_clicked
                || block_response.output_port_clicked.is_some()
                || block_response.input_port_drag_started
                || block_response.output_port_drag_started.is_some();
            let pointer_started_on_port =
                pointer_over_input_port || pointer_over_output_port.is_some();
            let connection_interaction = port_interaction
                || pointer_started_on_port
                || self
                    .connecting_from
                    .is_some_and(|(node_id, _)| node_id == node.id());
            let output_drag_intent = block_response.output_port_drag_started.or_else(|| {
                (node_response.drag_started())
                    .then_some(pointer_over_output_port)
                    .flatten()
            });

            if let Some(port_index) = output_drag_intent {
                new_connecting_from = Some((node.id(), port_index));
                new_dragged_node = None;
                new_selected_node = Some(node.id());
            }

            if should_start_node_drag(
                node_response.drag_started(),
                pointer_started_on_port,
                new_connecting_from.is_some_and(|(node_id, _)| node_id == node.id()),
            ) {
                new_dragged_node = Some(node.id());
                new_selected_node = Some(node.id());
            }

            if node_response.dragged()
                && !pointer_started_on_port
                && !new_connecting_from.is_some_and(|(node_id, _)| node_id == node.id())
                && new_dragged_node == Some(node.id())
            {
                let delta = node_response.drag_delta() / self.zoom;
                let new_pos =
                    NodePosition::from_pos2(Pos2::new(node_pos.x + delta.x, node_pos.y + delta.y));
                node.set_position(new_pos);
            }

            if node_response.drag_stopped() && !connection_interaction {
                new_dragged_node = None;
            }

            if node_response.clicked() && !connection_interaction {
                new_selected_node = Some(node.id());
            }

            if block_response.inner != NodeResponse::None {
                node_responses.push((node.id(), block_response.inner));
            }

            node_layouts.push(NodeLayout {
                node_id: node.id(),
                rect: block_response.rect,
                input_port_pos: block_response.input_port_pos,
                output_ports: block_response.output_port_positions,
                color: node.color(),
            });
        }

        let connection_visuals = collect_connection_visuals(self.connections, &node_layouts);
        for visual in &connection_visuals {
            draw_connection(ui, visual.from, visual.to, visual.color, self.zoom);
        }

        let hovered_target = new_connecting_from.and_then(|(source_id, _)| {
            pointer_pos.and_then(|pointer| {
                hovered_input_target(source_id, &node_layouts, pointer, 12.0 * self.zoom)
            })
        });

        if let Some((source_id, source_port)) = new_connecting_from {
            if let (Some(from), Some(pointer)) = (
                output_port_for_node(&node_layouts, source_id, source_port),
                pointer_pos,
            ) {
                let preview_target = hovered_target
                    .and_then(|target_id| input_port_for_node(&node_layouts, target_id))
                    .unwrap_or(pointer);
                let color = preview_connection_color(&node_layouts, source_id, source_port);
                draw_connection(ui, from, preview_target, color, self.zoom);
            }

            if let Some(target_id) = hovered_target {
                if let Some(target_pos) = input_port_for_node(&node_layouts, target_id) {
                    ScratchBlockRenderer::render_snap_indicator(ui, target_pos, true);
                }
            }
        }

        let hovered_node = pointer_pos.and_then(|pointer| {
            node_layouts.iter().rev().find_map(|layout| {
                layout
                    .rect
                    .expand2(Vec2::new(20.0 * self.zoom, 8.0 * self.zoom))
                    .contains(pointer)
                    .then_some(layout.node_id)
            })
        });
        let pointer_over_node = hovered_node.is_some();
        let secondary_clicked = ui
            .input(|input| input.pointer.button_clicked(egui::PointerButton::Secondary))
            && pointer_pos.is_some_and(|pointer| rect.contains(pointer));
        let deleted_node = secondary_clicked.then_some(hovered_node).flatten();
        let removed_connection = if deleted_node.is_none() {
            remove_connection_request(
                pointer_over_node,
                secondary_clicked,
                pointer_pos,
                &connection_visuals,
                10.0 * self.zoom,
            )
        } else {
            None
        };
        let pointer_released =
            ui.input(|input| input.pointer.button_released(egui::PointerButton::Primary));
        let completed_connection =
            completed_connection(new_connecting_from, pointer_released, hovered_target);
        if pointer_released {
            new_connecting_from = None;
        }

        let mut new_zoom = self.zoom;
        let zoom_delta = ui.input(|i| i.smooth_scroll_delta.y);
        if zoom_delta != 0.0 {
            if let Some(pointer) = pointer_pos.filter(|pointer| rect.contains(*pointer)) {
                new_zoom = (self.zoom + zoom_delta * 0.001).clamp(0.1, 5.0);
                let pointer_in_canvas = pointer - rect.min;
                let world_before = Vec2::new(
                    pointer_in_canvas.x / self.zoom,
                    pointer_in_canvas.y / self.zoom,
                ) - new_camera_offset;
                new_camera_offset = Vec2::new(
                    pointer_in_canvas.x / new_zoom,
                    pointer_in_canvas.y / new_zoom,
                ) - world_before;
            }
        }

        GraphResponse {
            node_responses,
            camera_offset: new_camera_offset,
            zoom: new_zoom,
            dragged_node: new_dragged_node,
            selected_node: new_selected_node,
            connecting_from: new_connecting_from,
            completed_connection,
            removed_connection,
            deleted_node,
        }
    }

    fn draw_grid(&self, ui: &Ui, rect: Rect) {
        let grid_size = 50.0 * self.zoom;
        let offset_x = self.camera_offset.x * self.zoom % grid_size;
        let offset_y = self.camera_offset.y * self.zoom % grid_size;

        let grid_color = Color32::from_rgb(50, 50, 55);

        let mut x = rect.min.x + offset_x;
        while x < rect.max.x {
            ui.painter().line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(1.0, grid_color),
            );
            x += grid_size;
        }

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
    pub connecting_from: Option<(NodeId, usize)>,
    pub completed_connection: Option<Connection>,
    pub removed_connection: Option<Connection>,
    pub deleted_node: Option<NodeId>,
}

fn draw_connection(ui: &Ui, from: Pos2, to: Pos2, color: Color32, zoom: f32) {
    let painter = ui.painter();
    let points = connection_curve_points(from, to);

    painter.add(egui::Shape::line(points, Stroke::new(3.0 * zoom, color)));
    painter.circle_filled(from, 6.0 * zoom, color);
    painter.circle_filled(to, 6.0 * zoom, color);
}

fn collect_connection_visuals(
    connections: &[Connection],
    node_layouts: &[NodeLayout],
) -> Vec<ConnectionVisual> {
    connections
        .iter()
        .filter_map(|connection| {
            let from_layout = node_layouts
                .iter()
                .find(|layout| layout.node_id == connection.from)?;
            let to_layout = node_layouts
                .iter()
                .find(|layout| layout.node_id == connection.to)?;
            let endpoints = connection_endpoints(from_layout, connection.from_port, to_layout)?;

            Some(ConnectionVisual {
                connection: *connection,
                from: endpoints.from,
                to: endpoints.to,
                color: preview_connection_color_for_layout(from_layout, connection.from_port)
                    .unwrap_or(to_layout.color),
            })
        })
        .collect()
}

fn pointer_hits_port(pointer_pos: Option<Pos2>, port_pos: Option<Pos2>, threshold: f32) -> bool {
    match (pointer_pos, port_pos) {
        (Some(pointer), Some(port)) => pointer.distance(port) <= threshold,
        _ => false,
    }
}

fn hovered_output_port(
    pointer_pos: Option<Pos2>,
    output_ports: &[OutputPortAnchor],
    threshold: f32,
) -> Option<usize> {
    let Some(pointer) = pointer_pos else {
        return None;
    };

    output_ports
        .iter()
        .filter_map(|port| {
            (port.pos.distance(pointer) <= threshold)
                .then_some((port.pos.distance(pointer), port.index))
        })
        .min_by(|(left, _), (right, _)| left.total_cmp(right))
        .map(|(_, index)| index)
}

fn should_start_node_drag(
    drag_started: bool,
    pointer_originated_on_port: bool,
    connection_active: bool,
) -> bool {
    drag_started && !pointer_originated_on_port && !connection_active
}

fn output_port_for_node(
    node_layouts: &[NodeLayout],
    node_id: NodeId,
    port_index: usize,
) -> Option<Pos2> {
    node_layouts
        .iter()
        .find(|layout| layout.node_id == node_id)
        .and_then(|layout| {
            layout
                .output_ports
                .iter()
                .find(|port| port.index == port_index)
                .map(|port| port.pos)
        })
}

fn input_port_for_node(node_layouts: &[NodeLayout], node_id: NodeId) -> Option<Pos2> {
    node_layouts
        .iter()
        .find(|layout| layout.node_id == node_id)
        .and_then(|layout| layout.input_port_pos)
}

fn hovered_input_target(
    source_id: NodeId,
    node_layouts: &[NodeLayout],
    point: Pos2,
    threshold: f32,
) -> Option<NodeId> {
    node_layouts
        .iter()
        .filter(|layout| layout.node_id != source_id)
        .filter_map(|layout| {
            let input_pos = layout.input_port_pos?;
            (input_pos.distance(point) <= threshold)
                .then_some((input_pos.distance(point), layout.node_id))
        })
        .min_by(|(left, _), (right, _)| left.total_cmp(right))
        .map(|(_, node_id)| node_id)
}

fn completed_connection(
    source: Option<(NodeId, usize)>,
    pointer_released: bool,
    hovered_target: Option<NodeId>,
) -> Option<Connection> {
    if !pointer_released {
        return None;
    }

    let (from, from_port) = source?;
    Some(Connection {
        from,
        from_port,
        to: hovered_target?,
    })
}

fn connection_endpoints(
    from_layout: &NodeLayout,
    from_port: usize,
    to_layout: &NodeLayout,
) -> Option<ConnectionEndpoints> {
    Some(ConnectionEndpoints {
        from: from_layout
            .output_ports
            .iter()
            .find(|port| port.index == from_port)?
            .pos,
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
) -> Option<Connection> {
    connections
        .iter()
        .filter_map(|connection| {
            let distance = connection_hit_distance(
                point,
                &connection_curve_points(connection.from, connection.to),
            );
            (distance <= threshold).then_some((distance, connection.connection))
        })
        .min_by(|(left, _), (right, _)| left.total_cmp(right))
        .map(|(_, connection)| connection)
}

fn remove_connection_request(
    pointer_over_node: bool,
    secondary_clicked: bool,
    pointer_pos: Option<Pos2>,
    connections: &[ConnectionVisual],
    threshold: f32,
) -> Option<Connection> {
    if pointer_over_node || !secondary_clicked {
        return None;
    }

    pointer_pos.and_then(|pointer| find_connection_at_point(connections, pointer, threshold))
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

fn preview_connection_color(
    node_layouts: &[NodeLayout],
    source_id: NodeId,
    from_port: usize,
) -> Color32 {
    node_layouts
        .iter()
        .find(|layout| layout.node_id == source_id)
        .and_then(|layout| preview_connection_color_for_layout(layout, from_port))
        .unwrap_or(Color32::WHITE)
}

fn preview_connection_color_for_layout(layout: &NodeLayout, from_port: usize) -> Option<Color32> {
    (layout.output_ports.len() > 1).then(|| match from_port {
        0 => Color32::from_rgb(0x3E, 0xA5, 0x76),
        1 => Color32::from_rgb(0xB8, 0x72, 0x34),
        _ => layout.color,
    })
}

use crate::node::{Node, NodeContext, NodeResponse, NodeType};
use egui::{Color32, Frame, Margin, Pos2, Rect, Sense, Stroke, Ui, Vec2};

/// Helper function to get darker version of a color
fn darker_color(color: Color32, factor: f32) -> Color32 {
    Color32::from_rgba_premultiplied(
        ((color.r() as f32) * (1.0 - factor)) as u8,
        ((color.g() as f32) * (1.0 - factor)) as u8,
        ((color.b() as f32) * (1.0 - factor)) as u8,
        color.a(),
    )
}

/// Helper to check if we need light or dark text based on background luminance
fn text_color_for_background(bg: Color32) -> Color32 {
    // Calculate relative luminance
    let r = bg.r() as f32 / 255.0;
    let g = bg.g() as f32 / 255.0;
    let b = bg.b() as f32 / 255.0;

    let luminance = 0.299 * r + 0.587 * g + 0.114 * b;

    if luminance > 0.5 {
        Color32::BLACK // Dark text for light backgrounds
    } else {
        Color32::WHITE // Light text for dark backgrounds
    }
}

/// Renders nodes in MIT Scratch-style blocks
pub struct ScratchBlockRenderer;

impl ScratchBlockRenderer {
    /// Get darker version of a color for ports
    fn port_color(node_color: Color32) -> Color32 {
        darker_color(node_color, 0.3)
    }

    /// Render a single block
    pub fn render(ui: &mut Ui, node: &mut dyn Node, ctx: &NodeContext) -> BlockResponse {
        let color = node.color();
        let port_color = Self::port_color(color);
        let node_type = node.node_type();

        // Block styling - use a darker inner frame for better contrast
        let frame = Frame::new()
            .fill(color)
            .corner_radius(egui::CornerRadius::same(8))
            .stroke(Stroke::new(
                if ctx.is_selected { 3.0 } else { 2.0 },
                if ctx.is_selected {
                    Color32::WHITE
                } else {
                    Color32::from_black_alpha(150)
                },
            ));

        let response = frame.show(ui, |ui| {
            ui.set_width(380.0); // Slightly smaller to accommodate ports
            ui.set_min_height(60.0);

            // Inner frame with much darker shade for content area
            // Use 0.4 (40% darker) instead of 0.1 for better contrast
            let content_color = darker_color(color, 0.4);
            let content_text_color = text_color_for_background(content_color);

            let content_frame = Frame::new()
                .fill(content_color)
                .corner_radius(egui::CornerRadius::same(4))
                .inner_margin(Margin::same(8));

            content_frame.show(ui, |ui| {
                // Set text color for the entire content area
                let mut style = ui.style_mut().clone();
                style.visuals.override_text_color = Some(content_text_color);
                ui.set_style(style);

                // Header with drag handle
                ui.horizontal(|ui| {
                    // Drag handle
                    ui.label(egui::RichText::new("::").color(content_text_color));

                    ui.label(
                        egui::RichText::new(node.name())
                            .color(content_text_color)
                            .size(16.0),
                    );
                });

                // Node content
                node.ui(ui, ctx)
            })
        });

        // Render ports OUTSIDE the block after the frame using the same UI's painter
        let block_rect = response.response.rect;
        let center_y = block_rect.center().y;
        let port_offset = 10.0; // Distance port sticks out from block
        let painter = ui.painter();

        let mut input_port_clicked = false;
        let mut output_port_clicked = false;
        let mut input_port_drag_started = false;
        let mut output_port_drag_started = false;
        let mut input_port_pos = None;
        let mut output_port_pos = None;

        // Input port on the left (for nodes that accept input: Optimization, Output)
        if matches!(node_type, NodeType::Opt | NodeType::Preview) {
            let input_pos = Pos2::new(block_rect.min.x - port_offset, center_y);
            input_port_pos = Some(input_pos);

            // Draw the port
            painter.circle_filled(input_pos, 8.0, port_color);
            painter.circle_stroke(input_pos, 8.0, Stroke::new(2.0, Color32::WHITE));

            // Make input port clickable
            let input_port_rect = Rect::from_center_size(input_pos, Vec2::new(20.0, 20.0));
            let input_response = ui.interact(
                input_port_rect,
                ui.id().with((node.id().0, "input_port")),
                Sense::click_and_drag(),
            );
            input_port_clicked = input_response.clicked();
            input_port_drag_started = input_response.drag_started();
        }

        // Output port on the right (for nodes that produce output: Input, Optimization)
        if matches!(node_type, NodeType::Input | NodeType::Opt) {
            let output_pos = Pos2::new(block_rect.max.x + port_offset, center_y);
            output_port_pos = Some(output_pos);

            // Draw the port
            painter.circle_filled(output_pos, 8.0, port_color);
            painter.circle_stroke(output_pos, 8.0, Stroke::new(2.0, Color32::WHITE));

            // Make output port clickable
            let output_port_rect = Rect::from_center_size(output_pos, Vec2::new(20.0, 20.0));
            let output_response = ui.interact(
                output_port_rect,
                ui.id().with((node.id().0, "output_port")),
                Sense::click_and_drag(),
            );
            output_port_clicked = output_response.clicked();
            output_port_drag_started = output_response.drag_started();
        }

        // Determine the response based on port clicks
        let inner_response = if input_port_clicked {
            NodeResponse::InputPortClicked
        } else if output_port_clicked {
            NodeResponse::OutputPortClicked
        } else {
            response.inner.inner
        };

        BlockResponse {
            rect: response.response.rect,
            inner: inner_response,
            input_port_clicked,
            output_port_clicked,
            input_port_drag_started,
            output_port_drag_started,
            input_port_pos,
            output_port_pos,
        }
    }

    /// Render snap indicator when dragging
    pub fn render_snap_indicator(ui: &mut Ui, pos: Pos2, is_valid: bool) {
        let color = if is_valid {
            Color32::from_rgb(0, 255, 0)
        } else {
            Color32::from_rgb(255, 0, 0)
        };

        let painter = ui.painter();
        painter.circle_stroke(pos, 6.0, Stroke::new(2.0, color));
    }
}

/// Response from rendering a block
pub struct BlockResponse {
    pub rect: Rect,
    pub inner: NodeResponse,
    pub input_port_clicked: bool,
    pub output_port_clicked: bool,
    pub input_port_drag_started: bool,
    pub output_port_drag_started: bool,
    pub input_port_pos: Option<Pos2>,
    pub output_port_pos: Option<Pos2>,
}

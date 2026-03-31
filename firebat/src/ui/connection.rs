use egui::{Color32, Pos2, Stroke, Ui};

/// Render connection lines between nodes
pub struct ConnectionRenderer;

impl ConnectionRenderer {
    /// Draw a curved connection line between two points
    pub fn render(ui: &mut Ui, from: Pos2, to: Pos2, color: Color32) {
        let painter = ui.painter();

        // Draw a curved connection using cubic bezier
        let mid_y = (from.y + to.y) / 2.0;
        let control_from = Pos2::new(from.x, mid_y);
        let control_to = Pos2::new(to.x, mid_y);

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

        painter.add(egui::Shape::line(points, Stroke::new(2.0, color)));

        // Draw connection dots
        painter.circle_filled(from, 4.0, color);
        painter.circle_filled(to, 4.0, color);
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

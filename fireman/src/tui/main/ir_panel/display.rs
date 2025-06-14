use crate::tui::FiremanCtx;
use ratatui::{
    prelude::{Constraint::*, *},
    widgets::{self, List, ListItem, ListState, Paragraph},
};

pub fn render_ir_section(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let ir_title = "IR Panel";
    frame.render_widget(
        Paragraph::new(ir_title).block(widgets::Block::bordered()),
        area,
    );
}

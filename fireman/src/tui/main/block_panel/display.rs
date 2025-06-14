use crate::tui::FiremanCtx;
use ratatui::{
    prelude::{Constraint::*, *},
    widgets::{self, List, ListItem, ListState, Paragraph},
};

pub fn render_block_section(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let block_title = "Block Panel";
    frame.render_widget(
        Paragraph::new(block_title).block(widgets::Block::bordered()),
        area,
    );
}

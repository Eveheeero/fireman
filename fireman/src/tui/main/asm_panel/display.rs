use crate::tui::FiremanCtx;
use ratatui::{
    prelude::{Constraint::*, *},
    widgets::{self, List, ListItem, ListState, Paragraph},
};

pub fn render_asm_section(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let asm_title = "ASM Panel";
    frame.render_widget(
        Paragraph::new(asm_title).block(widgets::Block::bordered()),
        area,
    );
}

use crate::tui::FiremanCtx;
use ratatui::{
    prelude::{Constraint::*, *},
    widgets::{self, List, ListItem, ListState, Paragraph},
};

pub fn render_ast_section(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let ast_title = "AST Panel";
    frame.render_widget(
        Paragraph::new(ast_title).block(widgets::Block::bordered()),
        area,
    );
}

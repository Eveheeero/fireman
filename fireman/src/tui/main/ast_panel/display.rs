use crate::tui::{
    main::{SelectedPanel, DEFAULT_STYLE, SCOPED_STYLE},
    FiremanCtx,
};
use ratatui::{
    prelude::{Constraint::*, *},
    widgets::{self, List, ListItem, ListState, Paragraph},
};

pub fn render_ast_section(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let style = if ctx.main_context.selected_panel == SelectedPanel::AstPanel {
        SCOPED_STYLE
    } else {
        DEFAULT_STYLE
    };
    frame.render_widget(widgets::Block::bordered().style(style), area);
    let block_inner_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });
    frame.render_widget(Paragraph::new("").style(DEFAULT_STYLE), block_inner_area);
    render_block_inner(frame, block_inner_area, ctx);
}

fn render_block_inner(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let temp = Paragraph::new("AST Panel");
    frame.render_widget(temp, area);
}

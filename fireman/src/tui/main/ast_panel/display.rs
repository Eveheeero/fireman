use crate::tui::{
    main::{SelectedPanel, DEFAULT_STYLE, SCOPED_STYLE},
    FiremanCtx,
};
use ratatui::{
    prelude::{Constraint::*, *},
    widgets::{self, List, ListItem, ListState, Paragraph},
};

pub fn render_ast_section(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let ast_title = "AST Panel";
    let style = if ctx.main_context.selected_panel == SelectedPanel::AstPanel {
        SCOPED_STYLE
    } else {
        DEFAULT_STYLE
    };
    frame.render_widget(
        Paragraph::new(ast_title)
            .block(widgets::Block::bordered().style(style))
            .style(DEFAULT_STYLE),
        area,
    );
}

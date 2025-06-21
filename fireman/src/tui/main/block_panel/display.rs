use crate::tui::{
    main::{SelectedPanel, DEFAULT_STYLE, SCOPED_STYLE},
    FiremanCtx,
};
use ratatui::{
    prelude::{Constraint::*, *},
    widgets::{self, List, ListItem, ListState, Paragraph},
};

pub fn render_block_section(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let block_title = "Block Panel";
    let style = if ctx.main_context.selected_panel == SelectedPanel::BlockPanel {
        SCOPED_STYLE
    } else {
        DEFAULT_STYLE
    };
    frame.render_widget(
        Paragraph::new(block_title)
            .block(widgets::Block::bordered().style(style))
            .style(DEFAULT_STYLE),
        area,
    );
}

use crate::tui::{
    main::{SelectedPanel, DEFAULT_STYLE, SCOPED_STYLE},
    FiremanCtx,
};
use ratatui::{
    prelude::{Constraint::*, *},
    widgets::{self, List, ListItem, ListState, Paragraph},
};

pub fn render_asm_section(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let asm_title = "ASM Panel";
    let style = if ctx.main_context.selected_panel == SelectedPanel::AsmPanel {
        SCOPED_STYLE
    } else {
        DEFAULT_STYLE
    };
    frame.render_widget(
        Paragraph::new(asm_title)
            .block(widgets::Block::bordered().style(style))
            .style(DEFAULT_STYLE),
        area,
    );
}

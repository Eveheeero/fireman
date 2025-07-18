use crate::tui::{
    FiremanCtx,
    main::{DEFAULT_STYLE, SCOPED_STYLE, SelectedPanel},
};
use ratatui::{prelude::*, widgets};

pub fn render_ast_section(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let style = if ctx.main_context.selected_panel == SelectedPanel::AstPanel {
        SCOPED_STYLE
    } else {
        DEFAULT_STYLE
    };
    frame.render_widget(widgets::Block::bordered().style(style).title("AST"), area);
    let block_inner_area = area.inner(Margin {
        horizontal: 1,
        vertical: 1,
    });
    frame.render_widget(
        widgets::Paragraph::new("").style(DEFAULT_STYLE),
        block_inner_area,
    );
    render_block_inner(frame, block_inner_area, ctx);
}

fn render_block_inner(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let scope_ctx = &ctx.main_context;
    let panel_ctx = &scope_ctx.ast_context;

    /* list */
    let mut list_selected = widgets::ListState::default();
    list_selected.select(panel_ctx.list_cursor);
    let list = panel_ctx.list.lock().unwrap();
    let list_items = list
        .iter()
        .map(|item| widgets::ListItem::new(item.as_str()));
    let list = widgets::List::new(list_items).highlight_style(
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_stateful_widget(list, area, &mut list_selected);
}

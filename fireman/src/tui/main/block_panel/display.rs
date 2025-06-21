use crate::tui::{
    main::{SelectedPanel, DEFAULT_STYLE, SCOPED_STYLE},
    FiremanCtx,
};
use ratatui::{prelude::*, widgets};

pub fn render_block_section(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let style = if ctx.main_context.selected_panel == SelectedPanel::BlockPanel {
        SCOPED_STYLE
    } else {
        DEFAULT_STYLE
    };
    frame.render_widget(widgets::Block::bordered().style(style).title("Block"), area);
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
    let panel_ctx = &scope_ctx.block_context;

    let [list_area, input_area] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

    /* list */
    let mut list_selected = widgets::ListState::default();
    list_selected.select(panel_ctx.list_cursor);
    let list_items = panel_ctx
        .list
        .iter()
        .map(|item| widgets::ListItem::new(item.as_str()));
    let list = widgets::List::new(list_items).highlight_style(
        Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_stateful_widget(list, list_area, &mut list_selected);

    /* input */
    let input_widget = {
        let input = &panel_ctx.input;
        if input.is_empty() {
            widgets::Paragraph::new("entry")
                .style(Style::new().add_modifier(Modifier::ITALIC).fg(Color::Gray))
        } else {
            widgets::Paragraph::new(input.as_str())
        }
    };
    frame.render_widget(input_widget, input_area);
}

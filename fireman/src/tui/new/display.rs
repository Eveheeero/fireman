use crate::tui::FiremanCtx;
use ratatui::{
    prelude::*,
    widgets::{self, List, ListItem, ListState, Paragraph},
};

pub fn display(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let main_layout = Layout::vertical([
        Constraint::Fill(1),   // File tree area
        Constraint::Length(3), // Input area
    ]);
    let [tree_area, input_area] = main_layout.areas(area);

    render_file_tree(frame, tree_area, ctx);
    render_input_box(frame, input_area, ctx);
}

fn render_file_tree(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let file_items: Vec<ListItem> = ctx
        .new_context
        .file_tree
        .iter()
        .map(|item| ListItem::new(item.as_str()))
        .collect();

    let mut state = ListState::default();
    state.select(Some(ctx.new_context.selected_index));

    let file_list = List::new(file_items)
        .block(widgets::Block::bordered().title("File Tree"))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("* ");

    frame.render_stateful_widget(file_list, area, &mut state);
}

fn render_input_box(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let input_widget = Paragraph::new(ctx.new_context.path.as_str())
        .block(widgets::Block::bordered().title("File Path"))
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(input_widget, area);
}

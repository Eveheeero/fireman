use crate::tui::TuiApp;
use crossterm::event;
use ratatui::{Frame, style, text, widgets};
use std::path::{Path, PathBuf};

pub struct TuiNavigateInputData<'list> {
    init: bool,
    pub input: String,
    input_index: usize,
    dir_content: Vec<PathBuf>,
    list: widgets::List<'list>,
    list_cursor: widgets::ListState,
}

impl<'list> Default for TuiNavigateInputData<'list> {
    fn default() -> Self {
        let current_dir = std::env::current_dir()
            .unwrap()
            .to_string_lossy()
            .to_string();
        Self {
            init: false,
            input_index: current_dir.len(),
            input: current_dir,
            dir_content: Vec::new(),
            list: widgets::List::default(),
            list_cursor: widgets::ListState::default(),
        }
    }
}

pub fn draw(app: &mut TuiApp, terminal: &mut Frame) {
    init_list(app);

    let mut list_area = terminal.area();
    list_area.height -= 1;
    terminal.render_stateful_widget(
        &app.data.navigate_input.list,
        list_area,
        &mut app.data.navigate_input.list_cursor,
    );

    let mut text_area = terminal.area();
    text_area.y = text_area.height - 1;
    text_area.x += 1;
    text_area.width -= 2;
    text_area.height = 1;
    let input_index = app.data.navigate_input.input_index;
    let input = app.data.navigate_input.input.as_str();
    let input_before: String = input.chars().take(input_index).collect();
    let input_mid: String = input.chars().nth(input_index).unwrap_or(' ').to_string();
    let input_after: String = input.chars().skip(input_index + 1).collect();
    let input = text::Line::from(
        [
            text::Span::raw(input_before),
            text::Span::styled(input_mid, style::Style::new().underlined()),
            text::Span::raw(input_after),
        ]
        .to_vec(),
    );
    let input = widgets::Paragraph::new(input);
    terminal.render_widget(input, text_area);
}
pub fn handle_event(_app: &mut TuiApp, _event: event::Event) {}

fn init_list(app: &mut TuiApp) {
    if app.data.navigate_input.init {
        return;
    }
    app.data.navigate_input.init = true;
    let path = Path::new(&app.data.navigate_input.input);
    let mut target_dir = path;
    let current_dir = std::env::current_dir().unwrap();
    loop {
        if target_dir.is_dir() {
            break;
        }
        if let Some(parent) = target_dir.parent() {
            target_dir = parent;
        } else {
            target_dir = &current_dir;
        }
    }
    let dir_content = std::fs::read_dir(target_dir).unwrap();
    let dir_content: Vec<_> = dir_content.map(|content| content.unwrap().path()).collect();
    let list = list(
        dir_content
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<String>>(),
    );
    app.data.navigate_input.list = list;
    let list_cursor = if path.is_file() {
        let mut result = 0;
        for i in 0..dir_content.len() {
            if dir_content[i] == path {
                result = i;
                break;
            }
        }
        result
    } else {
        0
    };
    app.data
        .navigate_input
        .list_cursor
        .select(Some(list_cursor));
    app.data.navigate_input.dir_content = dir_content;
    app.data.navigate_input.input_index = app.data.navigate_input.input.len();
}

fn list<'list>(entries: Vec<String>) -> widgets::List<'list> {
    widgets::List::new(entries)
        .block(widgets::Block::bordered())
        .highlight_style(style::Style::new().fg(style::Color::Cyan))
        .highlight_symbol("> ")
}

fn refresh_list(_app: &mut TuiApp) {}

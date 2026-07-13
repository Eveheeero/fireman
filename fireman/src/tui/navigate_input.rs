use crate::tui::{TuiApp, TuiState};
use crossterm::event;
use ratatui::{Frame, style, text, widgets};
use std::path::{Path, PathBuf};

pub struct TuiNavigateInputData<'list> {
    init: bool,
    previous_input: String,
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
            input_index: current_dir.chars().count(),
            previous_input: String::new(),
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
pub fn handle_event(app: &mut TuiApp, event: event::Event) {
    if !(event.is_key_press() || event.is_key_repeat()) {
        return;
    }
    let event = event.as_key_event().unwrap();
    const SPECIALS: &str = " `~!@#$%^&*()+-=[]\\;',./{}|:\"<>?";
    match event.code {
        event::KeyCode::Left | event::KeyCode::Right => {
            let is_control = event.modifiers.contains(event::KeyModifiers::CONTROL);
            let is_left = event::KeyCode::Left == event.code;
            if is_control && is_left {
                let mut is_first = true;
                while app.data.navigate_input.input_index > 0 {
                    let next = app
                        .data
                        .navigate_input
                        .input
                        .chars()
                        .skip(app.data.navigate_input.input_index - 1)
                        .next()
                        .unwrap();
                    let next_is_special = SPECIALS.contains(next);
                    if !is_first && next_is_special {
                        break;
                    }
                    app.data.navigate_input.input_index -= 1;
                    is_first = false;
                }
            } else if is_control {
                let mut is_first = true;
                while app.data.navigate_input.input_index
                    < app.data.navigate_input.input.chars().count()
                {
                    let next = app
                        .data
                        .navigate_input
                        .input
                        .chars()
                        .skip(app.data.navigate_input.input_index)
                        .next()
                        .unwrap();
                    let next_is_special = SPECIALS.contains(next);
                    if !is_first && next_is_special {
                        break;
                    }
                    app.data.navigate_input.input_index += 1;
                    is_first = false;
                }
            } else if is_left {
                if app.data.navigate_input.input_index > 0 {
                    app.data.navigate_input.input_index -= 1;
                }
            } else {
                if app.data.navigate_input.input_index
                    < app.data.navigate_input.input.chars().count()
                {
                    app.data.navigate_input.input_index += 1;
                }
            }
        }
        event::KeyCode::Up => {
            let current = app.data.navigate_input.list_cursor.selected().unwrap();
            if current > 0 {
                app.data
                    .navigate_input
                    .list_cursor
                    .select(Some(current - 1));
            }
        }
        event::KeyCode::Down => {
            let current = app.data.navigate_input.list_cursor.selected().unwrap();
            if current < app.data.navigate_input.list.len() - 1 {
                app.data
                    .navigate_input
                    .list_cursor
                    .select(Some(current + 1));
            }
        }
        event::KeyCode::PageUp => {
            let mut current = app.data.navigate_input.list_cursor.selected().unwrap();
            for _ in 0..3 {
                if current > 0 {
                    current -= 1;
                }
            }
            app.data.navigate_input.list_cursor.select(Some(current));
        }
        event::KeyCode::PageDown => {
            let mut current = app.data.navigate_input.list_cursor.selected().unwrap();
            for _ in 0..3 {
                if current < app.data.navigate_input.list.len() - 1 {
                    current += 1;
                }
            }
            app.data.navigate_input.list_cursor.select(Some(current));
            refresh_list(app);
        }
        event::KeyCode::Char(c) => {
            let byte_index = char_index_to_byte_index(
                &app.data.navigate_input.input,
                app.data.navigate_input.input_index,
            );
            app.data.navigate_input.input.insert(byte_index, c);
            app.data.navigate_input.input_index += 1;
            refresh_list(app);
        }
        event::KeyCode::Backspace => {
            if app.data.navigate_input.input_index > 0 {
                let byte_index = char_index_to_byte_index(
                    &app.data.navigate_input.input,
                    app.data.navigate_input.input_index - 1,
                );
                app.data.navigate_input.input.remove(byte_index);
                app.data.navigate_input.input_index -= 1;
                refresh_list(app);
            }
        }
        event::KeyCode::Delete => {
            if app.data.navigate_input.input_index < app.data.navigate_input.input.chars().count() {
                let byte_index = char_index_to_byte_index(
                    &app.data.navigate_input.input,
                    app.data.navigate_input.input_index,
                );
                app.data.navigate_input.input.remove(byte_index);
                refresh_list(app);
            }
        }
        event::KeyCode::Tab => {
            let current =
                app.data.navigate_input.dir_content[app.data.navigate_input.input_index].clone();
            app.data.navigate_input.input = current.to_string_lossy().to_string();
            app.data.navigate_input.input_index = app.data.navigate_input.input.chars().count();
            refresh_list(app);
        }
        event::KeyCode::Enter => {
            let current =
                app.data.navigate_input.dir_content[app.data.navigate_input.input_index].clone();
            let fireball =
                fireball::Fireball::from_path(current.to_str().unwrap()).expect("unsupported file");
            app.fireball = Some(fireball);
            app.state = TuiState::Tab(0);
        }
        _ => {}
    }
}

fn char_index_to_byte_index(input: &str, char_index: usize) -> usize {
    input
        .char_indices()
        .nth(char_index)
        .map(|(byte_index, _)| byte_index)
        .unwrap_or(input.len())
}

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
    app.data.navigate_input.previous_input = app.data.navigate_input.input.clone();
    app.data.navigate_input.input_index = app.data.navigate_input.input.chars().count();
}

fn list<'list>(entries: Vec<String>) -> widgets::List<'list> {
    widgets::List::new(entries)
        .block(widgets::Block::bordered())
        .highlight_style(style::Style::new().fg(style::Color::Cyan))
        .highlight_symbol("> ")
}

fn refresh_list(app: &mut TuiApp) {
    let previous_input = Path::new(&app.data.navigate_input.previous_input);
    let previous_input_dir = if previous_input.is_dir() {
        Some(previous_input)
    } else {
        previous_input.parent()
    };
    let input = Path::new(&app.data.navigate_input.input);
    let input_dir = if input.is_dir() {
        Some(input)
    } else {
        input.parent()
    };
    if previous_input == input {
        return;
    }

    if input_dir != previous_input_dir
        && let Some(input_dir) = input_dir
    {
        // if dir changed, refresh list
        let dir_content = std::fs::read_dir(input_dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<PathBuf>>();
        app.data.navigate_input.list = list(
            dir_content
                .iter()
                .map(|path| path.to_string_lossy().to_string())
                .collect::<Vec<String>>(),
        );
        app.data.navigate_input.list_cursor.select(Some(0));
        app.data.navigate_input.dir_content = dir_content;
    }
    // TODO change list index based on current input

    app.data.navigate_input.previous_input = app.data.navigate_input.input.clone();
}

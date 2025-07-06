use crate::tui::{FiremanScope, MutexCtx};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn handle_events(event: event::Event, ctx_: &MutexCtx) -> std::io::Result<bool> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Esc => Ok(true),
            KeyCode::Up => {
                let mut ctx = ctx_.write().unwrap();
                ctx.new_context.move_selection_up();
                Ok(false)
            }
            KeyCode::Down => {
                let mut ctx = ctx_.write().unwrap();
                ctx.new_context.move_selection_down();
                Ok(false)
            }
            KeyCode::Home => {
                let mut ctx = ctx_.write().unwrap();
                ctx.new_context.move_to_first();
                Ok(false)
            }
            KeyCode::End => {
                let mut ctx = ctx_.write().unwrap();
                ctx.new_context.move_to_last();
                Ok(false)
            }
            KeyCode::PageUp => {
                let mut ctx = ctx_.write().unwrap();
                ctx.new_context.move_page_up();
                Ok(false)
            }
            KeyCode::PageDown => {
                let mut ctx = ctx_.write().unwrap();
                ctx.new_context.move_page_down();
                Ok(false)
            }
            KeyCode::Char(c) => {
                handle_char_input(ctx_, c);
                Ok(false)
            }
            KeyCode::Backspace => {
                handle_backspace(ctx_);
                Ok(false)
            }
            KeyCode::Enter => {
                handle_enter(ctx_);
                Ok(false)
            }
            KeyCode::Tab => {
                handle_tab_completion(ctx_);
                Ok(false)
            }
            _ => Ok(false),
        },
        _ => Ok(false),
    }
}

fn handle_char_input(ctx_: &MutexCtx, c: char) {
    let mut ctx = ctx_.write().unwrap();
    ctx.new_context.path.push(c);
    ctx.top_message.clear();
    ctx.new_context.update_file_tree();

    // 문자 입력 후 첫 번째 매칭 항목으로 선택 인덱스 이동
    if !ctx.new_context.file_tree.is_empty() {
        let highlighted_index = ctx
            .new_context
            .file_tree
            .iter()
            .position(|item| item.starts_with(">>>"))
            .unwrap_or(0);
        ctx.new_context.selected_index = highlighted_index;
    }
}

fn handle_backspace(ctx_: &MutexCtx) {
    let mut ctx = ctx_.write().unwrap();
    ctx.new_context.path.pop();
    ctx.top_message.clear();
    ctx.new_context.update_file_tree();
}

fn handle_enter(ctx_: &MutexCtx) {
    let mut ctx = ctx_.write().unwrap();
    let path = ctx.new_context.path.clone();

    if path.is_empty() {
        ctx.top_message = "Please enter a file path".to_string();
    } else {
        match fs::File::open(&path) {
            Ok(_) => {
                ctx.fireball =
                    Some(fireball::Fireball::from_path(&path).expect("Failed to load Fireball"));
                ctx.scope = FiremanScope::Main;
            }
            Err(e) => {
                ctx.top_message = format!("Failed to open file: {}", e);
            }
        }
    }
}

fn handle_tab_completion(ctx_: &MutexCtx) {
    let mut ctx = ctx_.write().unwrap();
    let current_path = PathBuf::from(&ctx.new_context.path);

    // Get all matching items (highlighted ones with >>>)
    let matching_items: Vec<String> = ctx
        .new_context
        .file_tree
        .iter()
        .filter(|item| item.starts_with(">>>"))
        .cloned()
        .collect();

    // 하이라이팅된 항목이 있으면 그것을 우선 사용
    if !matching_items.is_empty() {
        if matching_items.len() == 1 {
            complete_single_match(&mut ctx, &matching_items[0], &current_path);
        } else {
            complete_common_prefix(&mut ctx, &matching_items, &current_path);
        }
    } else {
        // 하이라이팅된 항목이 없으면 현재 선택된 항목 사용
        if ctx.new_context.selected_index < ctx.new_context.file_tree.len() {
            let selected_item = ctx.new_context.file_tree[ctx.new_context.selected_index].clone();
            complete_selected_item(&mut ctx, &selected_item, &current_path);
        } else {
            // 선택된 항목도 없으면 첫 번째 항목 사용
            complete_first_item(&mut ctx, &current_path);
        }
    }

    ctx.new_context.update_file_tree();
    ctx.top_message.clear();
}

fn complete_first_item(ctx: &mut crate::tui::FiremanCtx, current_path: &Path) {
    if let Some(first_item) = ctx.new_context.file_tree.first() {
        let (is_dir, clean_name) = extract_item_info(first_item);
        if let Some(new_path) = build_completion_path(current_path, clean_name, is_dir) {
            ctx.new_context.path = new_path;
        }
    }
}

fn complete_single_match(
    ctx: &mut crate::tui::FiremanCtx,
    matched_item: &str,
    current_path: &Path,
) {
    let (is_dir, clean_name) = extract_item_info(matched_item);
    if let Some(new_path) = build_completion_path(current_path, clean_name, is_dir) {
        ctx.new_context.path = new_path;
    }
}

fn complete_common_prefix(
    ctx: &mut crate::tui::FiremanCtx,
    matching_items: &[String],
    current_path: &Path,
) {
    let clean_names: Vec<&str> = matching_items
        .iter()
        .map(|item| extract_item_info(item).1)
        .collect();

    if let Some(first_name) = clean_names.first() {
        let mut common_prefix = String::new();
        for (i, ch) in first_name.chars().enumerate() {
            if clean_names
                .iter()
                .all(|name| name.chars().nth(i).map_or(false, |c| c == ch))
            {
                common_prefix.push(ch);
            } else {
                break;
            }
        }

        if !common_prefix.is_empty() {
            if let Some(new_path) = build_completion_path(current_path, &common_prefix, false) {
                ctx.new_context.path = new_path;
            }
        }
    }
}

fn complete_selected_item(
    ctx: &mut crate::tui::FiremanCtx,
    selected_item: &str,
    current_path: &Path,
) {
    let (is_dir, clean_name) = extract_item_info(selected_item);
    if let Some(new_path) = build_completion_path(current_path, clean_name, is_dir) {
        ctx.new_context.path = new_path;
    }
}

fn extract_item_info(item: &str) -> (bool, &str) {
    // ">> " 또는 ">>> " 접두사 제거
    let item = item
        .strip_prefix(">>> ")
        .or_else(|| item.strip_prefix(">> "))
        .unwrap_or(item);
    let is_dir = item.starts_with("📁 ");
    let clean_name = item
        .strip_prefix("📁 ")
        .or_else(|| item.strip_prefix("📄 "))
        .unwrap_or("");
    (is_dir, clean_name)
}

fn build_completion_path(current_path: &Path, clean_name: &str, is_dir: bool) -> Option<String> {
    if clean_name.is_empty() {
        return None;
    }

    let base_dir = if current_path.exists() && current_path.is_dir() {
        current_path.to_path_buf()
    } else if let Some(parent) = current_path.parent() {
        if parent.as_os_str().is_empty() {
            std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf())
        } else {
            parent.to_path_buf()
        }
    } else {
        std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf())
    };

    let mut path_str = base_dir.join(clean_name).to_string_lossy().to_string();

    // Add directory separator for folders
    if is_dir && !path_str.ends_with(std::path::MAIN_SEPARATOR) {
        path_str.push(std::path::MAIN_SEPARATOR);
    }

    Some(path_str)
}

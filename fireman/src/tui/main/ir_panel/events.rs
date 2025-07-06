use crate::tui::MutexCtx;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn handle_events(event: event::Event, ctx_: &MutexCtx) -> std::io::Result<bool> {
    match event {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            _ if super::super::handle_focus_move(ctx_, key) => Ok(false),
            KeyCode::Esc => Ok(true),
            KeyCode::Up => move_cursor(ctx_, -1),
            KeyCode::Down => move_cursor(ctx_, 1),
            KeyCode::PageDown => move_cursor(ctx_, 10),
            KeyCode::PageUp => move_cursor(ctx_, -10),
            KeyCode::End => move_cursor(ctx_, i32::MAX),
            KeyCode::Home => move_cursor(ctx_, i32::MIN),
            _ => Ok(false),
        },
        _ => Ok(false),
    }
}

pub fn move_cursor(ctx_: &MutexCtx, vector: i32) -> std::io::Result<bool> {
    let mut ctx = ctx_.write().unwrap();

    let current: i32 = ctx
        .main_context
        .ir_context
        .list_cursor
        .map(|x| i32::try_from(x).unwrap())
        .unwrap_or(-1);
    let result;

    if vector != 1 && current == -1 {
        // current unselected and input is pgdown or end
        result = vector;
    } else if vector == i32::MIN {
        result = 0;
    } else {
        result = current + vector;
    }

    let list_len = ctx.main_context.ir_context.list.lock().unwrap().len();
    if result < 0 {
        ctx.main_context.ir_context.list_cursor = None;
    } else if list_len == 0 {
        ctx.main_context.ir_context.list_cursor = None;
    } else if result >= list_len as i32 {
        ctx.main_context.ir_context.list_cursor = Some(list_len - 1);
    } else {
        ctx.main_context.ir_context.list_cursor = Some(result as usize);
    }

    Ok(false)
}

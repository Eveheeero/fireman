use crate::tui::MutexCtx;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn handle_events(ctx_: &MutexCtx) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            _ if super::super::handle_focus_move(ctx_, key) => Ok(false),
            KeyCode::Esc => Ok(true),
            _ => Ok(false),
        },
        _ => Ok(false),
    }
}

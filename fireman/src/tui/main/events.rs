use crate::tui::MutexCtx;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn handle_events(ctx_: &MutexCtx) -> std::io::Result<bool> {
    match event::read()? {
        _ => Ok(false),
    }
}

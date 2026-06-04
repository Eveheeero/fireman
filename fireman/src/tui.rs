mod app;
mod input;
mod persistence;
mod prompt;
mod render;
mod types;

use crate::{model::OptimizationStore, utils::log::init_log};
use app::App;
use std::io::stdout;

#[derive(Clone, Debug)]
pub(super) struct StartupConfig {
    pub(super) input_path: Option<String>,
    pub(super) optimization_store: Option<OptimizationStore>,
}

pub(super) fn main(startup: Option<StartupConfig>) {
    init_log();

    // Enable mouse support
    crossterm::execute!(stdout(), crossterm::event::EnableMouseCapture,)
        .unwrap_or_else(|e| eprintln!("Failed to enable mouse capture: {}", e));

    let mut terminal = ratatui::init();
    let mut app = App::new(startup);
    let result = app.run(&mut terminal);

    // Disable mouse support
    crossterm::execute!(stdout(), crossterm::event::DisableMouseCapture,)
        .unwrap_or_else(|e| eprintln!("Failed to disable mouse capture: {}", e));

    ratatui::restore();
    result.unwrap();
}

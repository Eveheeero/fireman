mod app;

use crate::{model::OptimizationStore, utils::log::init_log};
use app::App;

#[derive(Clone, Debug)]
pub(super) struct StartupConfig {
    pub(super) input_path: Option<String>,
    pub(super) optimization_store: Option<OptimizationStore>,
}

pub(super) fn main(startup: Option<StartupConfig>) {
    init_log();
    let mut terminal = ratatui::init();
    let mut app = App::new(startup);
    let result = app.run(&mut terminal);
    ratatui::restore();
    result.unwrap();
}

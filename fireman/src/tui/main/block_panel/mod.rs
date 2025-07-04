mod context;
mod display;
mod events;

use crate::tui::FiremanCtx;
pub use context::Context;
pub use display::render_block_section;
pub use events::handle_events;

pub fn get_keybinding(_ctx: &FiremanCtx) -> &'static [(&'static str, &'static str)] {
    &[
        ("↑↓/Home/End/PgUp/PgDn", "cursor"),
        ("key", "input address"),
        ("space", "select block"),
        ("q", "generate AST from selected block"),
        ("enter", "analyze block"),
    ]
}

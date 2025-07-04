mod context;
mod display;
mod events;

use crate::tui::FiremanCtx;
pub use context::{Context, Data};
pub use display::render_ast_section;
pub use events::handle_events;

pub fn get_keybinding(_ctx: &FiremanCtx) -> &'static [(&'static str, &'static str)] {
    &[]
}

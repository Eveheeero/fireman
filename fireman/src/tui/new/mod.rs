mod context;
mod display;
mod events;

use crate::tui::FiremanCtx;
pub use context::Context;
pub use display::display;
pub use events::handle_events;

pub const fn get_keybinding(_ctx: &FiremanCtx) -> &'static [(&'static str, &'static str)] {
    &[
        ("↑↓/Home/End/PgUp/PgDn", "Navigate"),
        ("type", "Enter path"),
        ("enter", "Open File"),
        ("tab", "Autocomplete"),
        ("esc", "Quit"),
    ]
}

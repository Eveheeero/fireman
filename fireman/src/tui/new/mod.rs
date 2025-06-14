mod context;
mod display;
mod events;

use crate::tui::FiremanCtx;
pub use context::Context;
pub use display::display;
pub use events::handle_events;

pub fn get_keybinding(_ctx: &FiremanCtx) -> &'static [(&'static str, &'static str)] {
    &[
        ("↑↓/Home/End/Pu/Pd", "Navigate"),
        ("type", "Enter path"),
        ("enter", "Open File"),
        ("tab", "Autocomplete"),
        ("esc", "Quit"),
    ]
}

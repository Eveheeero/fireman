use crate::tui::main::MutexCtx;

pub fn get_keybinding(_ctx: &MutexCtx) -> &'static [(&'static str, &'static str)] {
    &[("q", "Quit"), ("h", "Help")]
}

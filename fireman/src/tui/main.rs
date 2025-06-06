use super::*;
use ratatui::prelude::*;

pub fn display_main(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    match ctx.scope {
        FiremanScope::New => new::display(frame, area, ctx),
    }
}
pub fn handle_events(ctx_: &MutexCtx) -> std::io::Result<bool> {
    let ctx = ctx_.read().unwrap();
    let scope = ctx.scope;
    drop(ctx);
    match scope {
        FiremanScope::New => new::handle_events(ctx_),
    }
}
pub fn get_keybinding(ctx: &FiremanCtx) -> &'static [(&'static str, &'static str)] {
    match ctx.scope {
        FiremanScope::New => new::get_keybinding(ctx),
    }
}

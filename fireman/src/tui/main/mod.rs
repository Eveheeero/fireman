mod asm_panel;
mod ast_panel;
mod block_panel;
mod ir_panel;

use crate::tui::{FiremanCtx, MutexCtx};
use ratatui::{
    prelude::{Constraint::*, Layout, Rect},
    Frame,
};

pub struct Context {
    selected_panel: SelectedPanel,

    asm_context: asm_panel::Context,
    ast_context: ast_panel::Context,
    block_context: block_panel::Context,
    ir_context: ir_panel::Context,
}

impl Context {
    pub fn new() -> Self {
        Context {
            selected_panel: SelectedPanel::BlockPanel,
            asm_context: asm_panel::Context::new(),
            ast_context: ast_panel::Context::new(),
            block_context: block_panel::Context::new(),
            ir_context: ir_panel::Context::new(),
        }
    }
}

pub fn display(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let [left_area, ir_area, ast_area] =
        Layout::horizontal([Percentage(20), Percentage(30), Percentage(50)]).areas(area);
    let [block_area, asm_area] =
        Layout::vertical([Percentage(50), Percentage(50)]).areas(left_area);

    block_panel::render_block_section(frame, block_area, ctx);
    asm_panel::render_asm_section(frame, asm_area, ctx);
    ir_panel::render_ir_section(frame, ir_area, ctx);
    ast_panel::render_ast_section(frame, ast_area, ctx);
}

pub fn handle_events(ctx_: &MutexCtx) -> std::io::Result<bool> {
    let ctx = ctx_.read().unwrap();
    let selected_panel = ctx.main_context.selected_panel;
    drop(ctx);
    match selected_panel {
        SelectedPanel::BlockPanel => block_panel::handle_events(ctx_),
        SelectedPanel::AsmPanel => asm_panel::handle_events(ctx_),
        SelectedPanel::IrPanel => ir_panel::handle_events(ctx_),
        SelectedPanel::AstPanel => ast_panel::handle_events(ctx_),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectedPanel {
    BlockPanel,
    AsmPanel,
    IrPanel,
    AstPanel,
}

pub fn get_keybinding(_ctx: &FiremanCtx) -> &'static [(&'static str, &'static str)] {
    &[
        ("↑↓/Home/End/Pu/Pd", "Navigate"),
        ("type", "Enter path"),
        ("enter", "Open File"),
        ("tab", "Autocomplete"),
        ("esc", "Quit"),
    ]
}

mod asm_panel;
mod ast_panel;
mod block_panel;
mod ir_panel;

use crate::tui::{FiremanCtx, MutexCtx};
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    prelude::*,
};

const SCOPED_STYLE: Style = Style::new().fg(Color::Yellow);
const DEFAULT_STYLE: Style = Style::new().fg(Color::White);

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
    let [left_area, ir_area, ast_area] = Layout::horizontal([
        Constraint::Percentage(20),
        Constraint::Percentage(30),
        Constraint::Percentage(50),
    ])
    .areas(area);
    let [block_area, asm_area] =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(left_area);

    block_panel::render_block_section(frame, block_area, ctx);
    asm_panel::render_asm_section(frame, asm_area, ctx);
    ir_panel::render_ir_section(frame, ir_area, ctx);
    ast_panel::render_ast_section(frame, ast_area, ctx);
}

pub fn handle_events(
    event: ratatui::crossterm::event::Event,
    ctx_: &MutexCtx,
) -> std::io::Result<bool> {
    let ctx = ctx_.read().unwrap();
    let selected_panel = ctx.main_context.selected_panel;
    drop(ctx);
    match selected_panel {
        SelectedPanel::BlockPanel => block_panel::handle_events(event, ctx_),
        SelectedPanel::AsmPanel => asm_panel::handle_events(event, ctx_),
        SelectedPanel::IrPanel => ir_panel::handle_events(event, ctx_),
        SelectedPanel::AstPanel => ast_panel::handle_events(event, ctx_),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectedPanel {
    BlockPanel,
    AsmPanel,
    IrPanel,
    AstPanel,
}

pub fn get_keybinding(ctx: &FiremanCtx) -> Vec<(&'static str, &'static str)> {
    let left = &[("ctrl↑↓←→", "Navigate panels"), ("esc", "Quit")];
    let right = match ctx.main_context.selected_panel {
        SelectedPanel::BlockPanel => block_panel::get_keybinding(ctx),
        SelectedPanel::AsmPanel => asm_panel::get_keybinding(ctx),
        SelectedPanel::IrPanel => ir_panel::get_keybinding(ctx),
        SelectedPanel::AstPanel => ast_panel::get_keybinding(ctx),
    };
    [left, right].concat()
}

/// true if handled
fn handle_focus_move(ctx_: &MutexCtx, key: KeyEvent) -> bool {
    let ctr_pressed = key.modifiers == KeyModifiers::CONTROL;
    if !ctr_pressed {
        return false;
    }

    let pressed_key = key.code;
    if !matches!(
        pressed_key,
        KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right
    ) {
        return false;
    }

    let mut ctx = ctx_.write().unwrap();
    let selected_panel = ctx.main_context.selected_panel;
    let now_selected_panel = match pressed_key {
        KeyCode::Up => match selected_panel {
            SelectedPanel::BlockPanel => selected_panel,
            SelectedPanel::AsmPanel => SelectedPanel::BlockPanel,
            SelectedPanel::IrPanel => SelectedPanel::BlockPanel,
            SelectedPanel::AstPanel => selected_panel,
        },
        KeyCode::Down => match selected_panel {
            SelectedPanel::BlockPanel => SelectedPanel::AsmPanel,
            SelectedPanel::AsmPanel => selected_panel,
            SelectedPanel::IrPanel => SelectedPanel::AsmPanel,
            SelectedPanel::AstPanel => selected_panel,
        },
        KeyCode::Left => match selected_panel {
            SelectedPanel::BlockPanel => selected_panel,
            SelectedPanel::AsmPanel => selected_panel,
            SelectedPanel::IrPanel => SelectedPanel::BlockPanel,
            SelectedPanel::AstPanel => SelectedPanel::IrPanel,
        },
        KeyCode::Right => match selected_panel {
            SelectedPanel::BlockPanel => SelectedPanel::IrPanel,
            SelectedPanel::AsmPanel => SelectedPanel::IrPanel,
            SelectedPanel::IrPanel => SelectedPanel::AstPanel,
            SelectedPanel::AstPanel => selected_panel,
        },
        _ => selected_panel,
    };
    if selected_panel != now_selected_panel {
        ctx.main_context.selected_panel = now_selected_panel;
        return true;
    }
    false
}

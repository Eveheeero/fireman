mod keybinding;
mod main;

use ratatui::{
    prelude::{
        Constraint::{Length, Min},
        *,
    },
    widgets, Frame,
};

pub(super) fn main() {
    let mut terminal = ratatui::init();
    let ctx = main::FiremanCtx::new();
    let result = run(&mut terminal, &ctx);
    ratatui::restore();
    result.unwrap();
}

fn run(terminal: &mut ratatui::DefaultTerminal, ctx: &main::MutexCtx) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, ctx))?;
        if main::handle_events(ctx)? {
            break Ok(());
        }
    }
}
fn draw(frame: &mut Frame, ctx: &main::MutexCtx) {
    let [title_area, main_area, status_area] =
        Layout::vertical([Length(1), Min(0), Length(1)]).areas(frame.area());
    display_title(frame, title_area, ctx);
    display_keybindings(frame, status_area, ctx);
    main::display_main(frame, main_area, ctx);
}

fn display_title(frame: &mut Frame, area: Rect, _ctx: &main::MutexCtx) {
    let widgets = widgets::Block::new()
        .borders(widgets::Borders::TOP)
        .title("Fireball TUI");
    // TODO Display scope
    frame.render_widget(widgets, area);
}
fn display_keybindings(frame: &mut Frame, area: Rect, ctx: &main::MutexCtx) {
    let mut widgets = widgets::Block::new()
        .borders(widgets::Borders::TOP)
        .title("Keys");
    let keybindings = keybinding::get_keybinding(ctx);
    for (k, v) in keybindings {
        let mut sb = String::new();
        sb.push('[');
        sb.push_str(k);
        sb.push_str(": ");
        sb.push_str(v);
        sb.push(']');
        widgets = widgets.title(sb);
    }
    frame.render_widget(widgets, area);
}

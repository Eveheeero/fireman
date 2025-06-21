mod main;
mod new;

use crate::utils::log::init_log;
use ratatui::{
    crossterm::event,
    prelude::{
        Constraint::{Length, Min},
        *,
    },
    widgets, Frame,
};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

pub(super) fn main() {
    init_log();
    let mut terminal = ratatui::init();
    let ctx = FiremanCtx::new();
    let result = run(&mut terminal, &ctx);
    ratatui::restore();
    result.unwrap();
}

#[derive(Clone)]
pub struct MutexCtx(pub Arc<RwLock<FiremanCtx>>);
unsafe impl Send for MutexCtx {}
unsafe impl Sync for MutexCtx {}
impl std::ops::Deref for MutexCtx {
    type Target = RwLock<FiremanCtx>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct FiremanCtx {
    /// If not none, fireball is busy
    worker: Option<std::thread::JoinHandle<()>>,
    fireball: Option<fireball::Fireball>,
    scope: FiremanScope,
    redraw_queue: VecDeque<()>,

    top_message: String,
    new_context: new::Context,
    main_context: main::Context,
}
impl FiremanCtx {
    pub fn new() -> MutexCtx {
        MutexCtx(Arc::new(RwLock::new(FiremanCtx {
            worker: None,
            fireball: None,
            scope: FiremanScope::New,
            redraw_queue: VecDeque::new(),
            top_message: String::new(),
            new_context: new::Context::new(),
            main_context: main::Context::new(),
        })))
    }
}
#[derive(PartialEq, Clone, Copy)]
pub enum FiremanScope {
    New,
    Main,
}

fn run(terminal: &mut ratatui::DefaultTerminal, ctx: &MutexCtx) -> std::io::Result<()> {
    'a: loop {
        terminal.draw(|frame| draw(frame, ctx))?;
        'b: loop {
            if event::poll(std::time::Duration::from_millis(100))? {
                break 'b;
            }
            let mut ctx = ctx.write().unwrap();
            if let Some(_) = ctx.redraw_queue.pop_front() {
                continue 'a;
            }
        }
        if handle_events(ctx)? {
            break Ok(());
        }
    }
}
fn draw(frame: &mut Frame, ctx: &MutexCtx) {
    let [title_area, main_area, status_area] =
        Layout::vertical([Length(1), Min(0), Length(1)]).areas(frame.area());
    let ctx = ctx.read().unwrap();
    display_title(frame, title_area, &ctx);
    display_keybindings(frame, status_area, &ctx);

    match ctx.scope {
        FiremanScope::New => new::display(frame, main_area, &ctx),
        FiremanScope::Main => main::display(frame, main_area, &ctx),
    }
}

fn display_title(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let widgets = widgets::Block::new()
        .borders(widgets::Borders::TOP)
        .title("Fireball TUI")
        .title(widgets::block::Title {
            content: ctx.top_message.as_str().into(),
            alignment: Some(Alignment::Right),
            position: None,
        });
    frame.render_widget(widgets, area);
}
fn display_keybindings(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let mut widgets = widgets::Block::new()
        .borders(widgets::Borders::TOP)
        .title("Keys");
    let keybindings = {
        match ctx.scope {
            FiremanScope::New => new::get_keybinding(ctx),
            FiremanScope::Main => main::get_keybinding(ctx),
        }
    };
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
pub fn handle_events(ctx_: &MutexCtx) -> std::io::Result<bool> {
    let ctx = ctx_.read().unwrap();
    let scope = ctx.scope;
    drop(ctx);
    match scope {
        FiremanScope::New => new::handle_events(ctx_),
        FiremanScope::Main => main::handle_events(ctx_),
    }
}

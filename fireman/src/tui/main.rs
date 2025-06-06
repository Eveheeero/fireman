use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::{Constraint::Fill, *},
    widgets, Frame,
};
use std::sync::Mutex;

pub struct MutexCtx(pub Mutex<FiremanCtx>);
unsafe impl Send for MutexCtx {}
unsafe impl Sync for MutexCtx {}
impl std::ops::Deref for MutexCtx {
    type Target = Mutex<FiremanCtx>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct FiremanCtx {
    /// If not none, fireball is busy
    worker: Option<std::thread::JoinHandle<()>>,
    fireball: Option<fireball::Fireball>,
    scope: FiremanScope,
}
impl FiremanCtx {
    pub fn new() -> MutexCtx {
        MutexCtx(Mutex::new(FiremanCtx {
            worker: None,
            fireball: None,
            scope: FiremanScope::New,
        }))
    }
}
#[derive(PartialEq)]
pub enum FiremanScope {
    New,
}

pub fn display_main(frame: &mut Frame, area: Rect, _ctx: &MutexCtx) {
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(area);
    frame.render_widget(widgets::Block::bordered().title("Left"), left_area);
    frame.render_widget(widgets::Block::bordered().title("Right"), right_area);
}
pub fn handle_events(_ctx: &MutexCtx) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}

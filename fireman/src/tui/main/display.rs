use crate::tui::FiremanCtx;
use ratatui::{
    prelude::{Constraint::*, *},
    widgets::{self, List, ListItem, ListState, Paragraph},
};

pub fn display(frame: &mut Frame, area: Rect, ctx: &FiremanCtx) {
    let hello = "Hello, Fireman!";

    frame.render_widget(Paragraph::new(hello), area);
}

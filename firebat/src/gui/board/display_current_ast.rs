use crate::{
    Firebat,
    gui::board::{BoardWindow, BoardWindowKind},
};

/// Shown until the first decompilation is requested.
const EMPTY_HINT: &str = "press Decompile in the top bar";

/// Width of the scrolled view, wide enough for a typical decompiled line.
const VIEW_WIDTH: f32 = 720.0;

/// Height of the scrolled view, tall enough to read a whole function at once.
const VIEW_HEIGHT: f32 = 520.0;

/// State of the window which prints the current ast.
pub struct DisplayCurrentAstData {
    /// Lines of the last decompilation, rendered lazily row by row.
    lines: Vec<String>,
    /// Longest line in characters, used to size the horizontal scroll area.
    widest: usize,
}

impl Default for DisplayCurrentAstData {
    fn default() -> Self {
        let mut data = Self {
            lines: Vec::new(),
            widest: 0,
        };
        data.set_ast(EMPTY_HINT.to_owned());
        data
    }
}

impl DisplayCurrentAstData {
    /// Stores the result of the last decompilation.
    pub fn set_ast(&mut self, ast: String) {
        self.lines = ast.lines().map(str::to_owned).collect();
        if self.lines.is_empty() {
            self.lines.push(String::new());
        }
        self.widest = self
            .lines
            .iter()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);
    }
}

/// Creates the window which prints the current ast.
pub fn window(id: impl Into<String>, pos: egui::Pos2) -> BoardWindow {
    BoardWindow::new(
        id,
        "Display Current AST",
        pos,
        BoardWindowKind::DisplayCurrentAst(DisplayCurrentAstData::default()),
    )
}

pub fn ui(_app: &mut Firebat, _id: &str, data: &mut DisplayCurrentAstData, ui: &mut egui::Ui) {
    ui.set_min_size(egui::vec2(VIEW_WIDTH, VIEW_HEIGHT));

    let font = egui::TextStyle::Monospace.resolve(ui.style());
    let char_width = ui
        .ctx()
        .fonts_mut(|fonts| fonts.glyph_width(&font, ' ').max(1.0));
    let row_height = ui.text_style_height(&egui::TextStyle::Monospace);

    ui.spacing_mut().item_spacing.y = 0.0;
    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .max_height(VIEW_HEIGHT)
        .show_rows(ui, row_height, data.lines.len(), |ui, rows| {
            ui.set_min_width(char_width * data.widest as f32);
            for line in &data.lines[rows] {
                ui.add(
                    egui::Label::new(egui::RichText::new(line).monospace())
                        .wrap_mode(egui::TextWrapMode::Extend),
                );
            }
        });
}

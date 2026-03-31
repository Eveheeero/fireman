use egui::{Frame, ScrollArea, Ui};
use fireball::abstract_syntax_tree::{Ast, AstPrintConfig};

/// Render AST as formatted C-like code
pub struct AstCodeView;

impl AstCodeView {
    pub fn show(ui: &mut Ui, ast: &Ast, config: Option<&AstPrintConfig>) {
        let cfg = config.cloned().unwrap_or_default();
        let code = ast.print(Some(cfg));

        Frame::dark_canvas(ui.style()).show(ui, |ui| {
            ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                ui.monospace(&code);
            });
        });
    }

    pub fn show_diff(ui: &mut Ui, before: &Ast, after: &Ast) {
        let before_code = before.print(Some(AstPrintConfig::default()));
        let after_code = after.print(Some(AstPrintConfig::default()));

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.strong("BEFORE:");
                Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        ui.monospace(&before_code);
                    });
                });
            });

            ui.vertical(|ui| {
                ui.strong("AFTER:");
                Frame::dark_canvas(ui.style()).show(ui, |ui| {
                    ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                        ui.monospace(&after_code);
                    });
                });
            });
        });
    }

    /// Get first N lines of code for preview
    pub fn get_preview(ast: &Ast, n: usize) -> Vec<String> {
        let code = ast.print(Some(AstPrintConfig::default()));
        code.lines().take(n).map(|s| s.to_string()).collect()
    }

    /// Get searchable summary text
    pub fn get_search_text(ast: &Ast) -> String {
        let code = ast.print(Some(AstPrintConfig::default()));
        code.lines().take(10).collect::<Vec<_>>().join(" ")
    }
}

use super::state::FirebatState;
use eframe::{
    egui,
    egui::{RichText, Stroke, WidgetText},
};
use egui_dock::TabViewer;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub(super) enum PanelTab {
    Sections,
    Optimization,
    Assembly,
    Ir,
    Ast,
}

impl PanelTab {
    pub(super) const ALL: [Self; 5] = [
        Self::Sections,
        Self::Optimization,
        Self::Assembly,
        Self::Ir,
        Self::Ast,
    ];

    pub(super) const fn title(self) -> &'static str {
        match self {
            Self::Sections => "Sections",
            Self::Optimization => "Optimization",
            Self::Assembly => "Assembly",
            Self::Ir => "IR",
            Self::Ast => "Decompiled AST",
        }
    }
}

pub(super) struct FirebatTabViewer<'a> {
    pub(super) state: &'a mut FirebatState,
}

impl TabViewer for FirebatTabViewer<'_> {
    type Tab = PanelTab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title().into()
    }

    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        let panel_fill = ui.visuals().extreme_bg_color;
        let panel_stroke = ui.visuals().widgets.noninteractive.bg_stroke.color;
        egui::Frame::group(ui.style())
            .fill(panel_fill)
            .stroke(Stroke::new(1.0, panel_stroke))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new(tab.title()).strong());
                    ui.separator();
                    match tab {
                        PanelTab::Sections => self.state.render_section_panel(ui),
                        PanelTab::Optimization => self.state.render_optimization_panel(ui),
                        PanelTab::Assembly => self.state.render_assembly_panel(ui),
                        PanelTab::Ir => self.state.render_ir_panel(ui),
                        PanelTab::Ast => self.state.render_ast_panel(ui),
                    }
                });
            });
    }
}

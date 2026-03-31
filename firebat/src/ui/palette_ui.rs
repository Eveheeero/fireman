use crate::node::NodeType;
use egui::{Ui, Window};

// Stub types for palette - these are placeholders since the palette is not currently used
#[derive(Clone, Debug)]
pub struct NodePalette {
    pub categories: Vec<PaletteCategory>,
}

impl NodePalette {
    pub fn categories(&self) -> &[PaletteCategory] {
        &self.categories
    }
}

#[derive(Clone, Debug)]
pub struct PaletteCategory {
    pub name: String,
    pub items: Vec<PaletteItem>,
}

impl PaletteCategory {
    pub fn items(&self) -> &[PaletteItem] {
        &self.items
    }
}

#[derive(Clone, Debug)]
pub struct PaletteItem {
    pub node_type: NodeType,
    pub name: String,
    pub description: String,
    pub icon: String,
}

pub struct PaletteSelection {
    pub node_type: NodeType,
    pub name: String,
}

/// UI for the node palette
pub struct PaletteUi {
    is_open: bool,
    filter: String,
}

impl PaletteUi {
    pub fn new() -> Self {
        Self {
            is_open: false,
            filter: String::new(),
        }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn show(&mut self, ctx: &egui::Context, palette: &NodePalette) -> Option<PaletteSelection> {
        let mut selection: Option<PaletteSelection> = None;

        if self.is_open {
            Window::new("Add Node")
                .collapsible(false)
                .resizable(true)
                .default_size([300.0, 400.0])
                .show(ctx, |ui: &mut Ui| {
                    // Search filter
                    ui.horizontal(|ui: &mut Ui| {
                        ui.label("Search:");
                        ui.text_edit_singleline(&mut self.filter);
                    });

                    ui.separator();

                    // Categories and items
                    egui::ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                        for category in palette.categories() {
                            ui.collapsing(&category.name, |ui: &mut Ui| {
                                let filtered_items: Vec<&PaletteItem> = category
                                    .items()
                                    .iter()
                                    .filter(|item| {
                                        item.name
                                            .to_lowercase()
                                            .contains(&self.filter.to_lowercase())
                                            || item
                                                .description
                                                .to_lowercase()
                                                .contains(&self.filter.to_lowercase())
                                    })
                                    .collect();

                                for item in filtered_items {
                                    if self.show_palette_item(ui, item) {
                                        selection = Some(PaletteSelection {
                                            node_type: item.node_type.clone(),
                                            name: item.name.to_string(),
                                        });
                                    }
                                }
                            });
                        }
                    });

                    ui.separator();

                    if ui.button("Close").clicked() {
                        self.is_open = false;
                    }
                });
        }

        selection
    }

    fn show_palette_item(&self, ui: &mut Ui, item: &PaletteItem) -> bool {
        let button = ui.button(format!("{} {}", item.icon, item.name));

        if ui.is_rect_visible(button.rect) {
            button.on_hover_text(&item.description)
        } else {
            button
        }
        .clicked()
    }
}

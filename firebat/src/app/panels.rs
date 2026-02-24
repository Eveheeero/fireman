use super::{state::FirebatState, tabs::PanelTab};
use eframe::{
    egui,
    egui::{Color32, RichText, Stroke},
};
use egui_dock::DockState;

impl FirebatState {
    pub(super) fn render_navigation(
        &mut self,
        ui: &mut egui::Ui,
        dock_state: &mut DockState<PanelTab>,
        show_perf_hud: &mut bool,
    ) {
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            ui.label(RichText::new("Firebat").strong());
            ui.add_space(12.0);
            ui.menu_button("File", |ui| {
                if ui
                    .add_enabled(!self.is_busy(), egui::Button::new("Open executable..."))
                    .clicked()
                {
                    self.open_file();
                    ui.close();
                }
            });
            ui.menu_button("View", |ui| {
                for tab in PanelTab::ALL {
                    let is_open = dock_state.find_tab(&tab).is_some();
                    let label = if is_open {
                        format!("{} (Open)", tab.title())
                    } else {
                        format!("Open {}", tab.title())
                    };
                    if ui.add_enabled(!is_open, egui::Button::new(label)).clicked() {
                        dock_state.push_to_first_leaf(tab);
                        self.log(format!("Reopened {}", tab.title()));
                        ui.close();
                    }
                }
                ui.separator();
                let perf_label = if *show_perf_hud {
                    "Hide Performance HUD"
                } else {
                    "Show Performance HUD"
                };
                if ui.button(perf_label).clicked() {
                    *show_perf_hud = !*show_perf_hud;
                    ui.close();
                }
            });
            ui.menu_button("Tools", |ui| {
                ui.add_enabled(false, egui::Button::new("No tools yet"));
            });
            if self.is_busy() {
                ui.add_space(8.0);
                ui.spinner();
                ui.label("Working...");
            }
        });
    }

    pub(super) fn render_log_bar(&mut self, ui: &mut egui::Ui) {
        let latest_log = self.logs.last().map_or("No logs yet", String::as_str);
        ui.horizontal(|ui| {
            let label = if self.log_expanded {
                "Hide logs"
            } else {
                "Show logs"
            };
            if ui.button(label).clicked() {
                self.log_expanded = !self.log_expanded;
            }
            ui.add_space(8.0);
            ui.add(egui::Label::new(latest_log).truncate());
        });

        if self.log_expanded {
            ui.separator();
            let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 12.0;
            egui::ScrollArea::vertical().max_height(180.0).show_rows(
                ui,
                row_height,
                self.logs.len(),
                |ui, row_range| {
                    for row in row_range {
                        let log_entry = &self.logs[self.logs.len() - 1 - row];
                        egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.monospace(log_entry);
                        });
                    }
                },
            );
        }
    }

    pub(super) fn render_section_panel(&mut self, ui: &mut egui::Ui) {
        let mut analyze_request: Option<String> = None;
        let mut hovered_index: Option<usize> = None;
        let hovered_parent_start = self.decompile_result.as_ref().and_then(|result| {
            self.hovered_assembly_index
                .and_then(|index| result.assembly_parent_by_index.get(&index).copied())
        });

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let controls_enabled = !self.is_busy();
                ui.add_sized(
                    [ui.available_width() - 140.0, 26.0],
                    egui::TextEdit::singleline(&mut self.analyze_target_address)
                        .hint_text("Address (empty to use entry point)"),
                );
                if ui
                    .add_enabled(controls_enabled, egui::Button::new("Analyze Address"))
                    .clicked()
                {
                    analyze_request = Some(self.analyze_target_address.clone());
                }
            });

            ui.horizontal(|ui| {
                let controls_enabled = !self.is_busy();
                if ui
                    .add_enabled(controls_enabled, egui::Button::new("Analyze All"))
                    .clicked()
                {
                    self.analyze_all();
                }
                if ui
                    .add_enabled(controls_enabled, egui::Button::new("Select All"))
                    .clicked()
                {
                    self.select_all();
                }
                if ui
                    .add_enabled(
                        controls_enabled,
                        egui::Button::new("Decompile Selected")
                            .fill(Color32::from_rgb(15, 108, 189)),
                    )
                    .clicked()
                {
                    self.decompile_selected();
                }
            });
            ui.separator();

            let section_row_height = ui.text_style_height(&egui::TextStyle::Body) + 18.0;
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show_rows(
                    ui,
                    section_row_height,
                    self.known_sections.len(),
                    |ui, row_range| {
                        for index in row_range {
                            let section_start_address =
                                self.known_sections[index].data.start_address;
                            let primary_assembly_index =
                                self.decompile_result.as_ref().and_then(|result| {
                                    result
                                        .section_primary_assembly
                                        .get(&section_start_address)
                                        .copied()
                                });
                            let is_related_hovered =
                                hovered_parent_start == Some(section_start_address);
                            let accent_color = primary_assembly_index.and_then(|assembly_index| {
                                self.decompile_result
                                    .as_ref()
                                    .and_then(|result| result.colors.get(&assembly_index).copied())
                            });
                            let panel_fill = ui.visuals().extreme_bg_color;
                            let panel_stroke = ui.visuals().widgets.noninteractive.bg_stroke.color;

                            let mut frame = egui::Frame::group(ui.style())
                                .fill(panel_fill)
                                .stroke(Stroke::new(1.0, panel_stroke));

                            if is_related_hovered {
                                frame = frame.stroke(Stroke::new(
                                    1.0,
                                    accent_color.unwrap_or(Color32::from_rgb(15, 108, 189)),
                                ));
                            }

                            let response = frame
                                .show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        let mut selected = self.known_sections[index].selected;
                                        let checkbox = ui.add_enabled(
                                            self.known_sections[index].data.analyzed,
                                            egui::Checkbox::without_text(&mut selected),
                                        );
                                        if checkbox.changed() {
                                            self.known_sections[index].selected = selected;
                                        }

                                        let end = self.known_sections[index]
                                            .data
                                            .end_address
                                            .map_or_else(
                                                || "?".to_string(),
                                                |value| format!("{value:x}"),
                                            );
                                        ui.label(format!(
                                            "0x{:x}..{end}",
                                            self.known_sections[index].data.start_address
                                        ));

                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                if self.known_sections[index].data.analyzed {
                                                    ui.label("Analyzed");
                                                } else if ui
                                                    .add_enabled(
                                                        !self.is_busy(),
                                                        egui::Button::new("Analyze Section"),
                                                    )
                                                    .clicked()
                                                {
                                                    analyze_request = Some(
                                                        self.known_sections[index]
                                                            .data
                                                            .start_address
                                                            .to_string(),
                                                    );
                                                }
                                            },
                                        );
                                    });
                                })
                                .response;

                            if response.hovered() {
                                hovered_index = primary_assembly_index;
                            }
                        }
                    },
                );
        });

        if let Some(address) = analyze_request {
            self.analyze_section_from_address(&address);
        }

        if let Some(index) = hovered_index {
            self.hover_candidate = Some(index);
        }
    }

    pub(super) fn render_assembly_panel(&mut self, ui: &mut egui::Ui) {
        let Some(result) = &self.decompile_result else {
            ui.label("No assembly data available. Select sections and run decompilation.");
            return;
        };

        let mut hovered_index: Option<usize> = None;
        let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 12.0;
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(
                ui,
                row_height,
                result.data.assembly.len(),
                |ui, row_range| {
                    for row in row_range {
                        let assembly = &result.data.assembly[row];
                        let is_hovered = self.hovered_assembly_index == Some(assembly.index);
                        let accent_color = result.colors.get(&assembly.index).copied();
                        let row_fill = accent_color
                            .map(|color| {
                                Color32::from_rgba_unmultiplied(
                                    color.r(),
                                    color.g(),
                                    color.b(),
                                    0x1A,
                                )
                            })
                            .unwrap_or(ui.visuals().extreme_bg_color);
                        let panel_stroke = ui.visuals().widgets.noninteractive.bg_stroke.color;

                        let mut frame = egui::Frame::group(ui.style())
                            .fill(row_fill)
                            .stroke(Stroke::new(1.0, panel_stroke));
                        if is_hovered {
                            frame = frame.stroke(Stroke::new(
                                1.0,
                                accent_color.unwrap_or(Color32::from_rgb(15, 108, 189)),
                            ));
                        }

                        let response = frame
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.monospace(&assembly.data);
                                });
                            })
                            .response;
                        if response.hovered() {
                            hovered_index = Some(assembly.index);
                        }
                    }
                },
            );

        if let Some(index) = hovered_index {
            self.hover_candidate = Some(index);
        }
    }

    pub(super) fn render_ir_panel(&mut self, ui: &mut egui::Ui) {
        let Some(result) = &self.decompile_result else {
            ui.label("No IR data available. Select sections and run decompilation.");
            return;
        };

        let mut hovered_index: Option<usize> = None;
        let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 12.0;
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, row_height, result.data.ir.len(), |ui, row_range| {
                for row in row_range {
                    let ir = &result.data.ir[row];
                    let is_hovered = self.hovered_assembly_index == Some(ir.parents_assembly_index);
                    let accent_color = result.colors.get(&ir.parents_assembly_index).copied();
                    let row_fill = accent_color
                        .map(|color| {
                            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 0x1A)
                        })
                        .unwrap_or(ui.visuals().extreme_bg_color);
                    let panel_stroke = ui.visuals().widgets.noninteractive.bg_stroke.color;

                    let mut frame = egui::Frame::group(ui.style())
                        .fill(row_fill)
                        .stroke(Stroke::new(1.0, panel_stroke));
                    if is_hovered {
                        frame = frame.stroke(Stroke::new(
                            1.0,
                            accent_color.unwrap_or(Color32::from_rgb(15, 108, 189)),
                        ));
                    }

                    let response = frame
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.monospace(&ir.data);
                            });
                        })
                        .response;
                    if response.hovered() {
                        hovered_index = Some(ir.parents_assembly_index);
                    }
                }
            });

        if let Some(index) = hovered_index {
            self.hover_candidate = Some(index);
        }
    }

    pub(super) fn render_ast_panel(&mut self, ui: &mut egui::Ui) {
        let Some(result) = &self.decompile_result else {
            ui.label("No decompiled code available. Select sections and run decompilation.");
            return;
        };

        ui.scope(|ui| {
            let scroll = &mut ui.style_mut().spacing.scroll;
            scroll.floating = false;
            scroll.bar_inner_margin = 0.0;
            scroll.bar_outer_margin = 0.0;

            let available = ui.available_size();
            let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 4.0;
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_width(available.x)
                .max_height(available.y)
                .min_scrolled_width(available.x)
                .show_rows(
                    ui,
                    row_height,
                    result.decompiled_line_ranges.len(),
                    |ui, row_range| {
                        for row in row_range {
                            let (start, end) = result.decompiled_line_ranges[row];
                            ui.monospace(&result.data.decompiled[start..end]);
                        }
                    },
                );
        });
    }
}

use super::{state::FirebatState, tabs::PanelTab};
use crate::model::{
    AssemblyEditorDraft, AstNodeEditorDraft, EditorLayer, EditorTarget, IrEditorDraft,
    OptimizationSettings,
};
use eframe::egui::{self, Color32, RichText, Stroke};
use egui_dock::DockState;

impl FirebatState {
    pub(super) fn render_navigation(
        &mut self,
        ui: &mut egui::Ui,
        dock_state: &mut DockState<PanelTab>,
        show_perf_hud: &mut bool,
        show_about: &mut bool,
    ) {
        ui.horizontal(|ui| {
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

            ui.menu_button("Optimization", |ui| {
                let panel_open = dock_state.find_tab(&PanelTab::Optimization).is_some();
                let open_label = if panel_open {
                    "Optimization Panel Open"
                } else {
                    "Open Optimization Panel"
                };
                if ui
                    .add_enabled(!panel_open, egui::Button::new(open_label))
                    .clicked()
                {
                    dock_state.push_to_first_leaf(PanelTab::Optimization);
                    self.log("Opened Optimization panel");
                    ui.close();
                }
                ui.separator();
                if ui
                    .add_enabled(
                        !self.is_busy(),
                        egui::Button::new("Apply Current Optimization"),
                    )
                    .clicked()
                {
                    self.apply_optimization_settings();
                    ui.close();
                }
                if ui
                    .add_enabled(!self.is_busy(), egui::Button::new("Apply Buffer Script"))
                    .clicked()
                {
                    self.apply_optimization_buffer();
                    ui.close();
                }
                if ui.button("Reset Draft To Applied").clicked() {
                    self.reset_optimization_draft();
                    ui.close();
                }
                if ui.button("Restore Defaults").clicked() {
                    self.restore_default_optimization_draft();
                    ui.close();
                }
            });

            ui.menu_button("Tools", |ui| {
                if ui
                    .add_enabled(
                        self.decompile_result.is_some() && !self.is_busy(),
                        egui::Button::new("Export edit patch"),
                    )
                    .clicked()
                {
                    self.export_patch();
                    ui.close();
                }
                if ui
                    .add_enabled(
                        self.editor_target.is_some(),
                        egui::Button::new("Reset editor"),
                    )
                    .clicked()
                {
                    self.reset_editor_draft();
                    ui.close();
                }
            });

            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    *show_about = true;
                    ui.close();
                }
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
                        let entry = &self.logs[self.logs.len() - 1 - row];
                        egui::Frame::group(ui.style()).show(ui, |ui| {
                            ui.monospace(entry);
                        });
                    }
                },
            );
        }
    }

    pub(super) fn render_section_panel(&mut self, ui: &mut egui::Ui) {
        let mut analyze_request: Option<String> = None;
        let hovered_parent_start = self.decompile_result.as_ref().and_then(|result| {
            self.hovered_assembly_index
                .and_then(|index| result.assembly_parent_by_index.get(&index).copied())
        });

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let controls_enabled = !self.is_busy();
                ui.add_sized(
                    [200.0, 24.0],
                    egui::TextEdit::singleline(&mut self.analyze_target_address)
                        .hint_text("Address (hex)"),
                );
                ui.add_space(8.0);
                if ui
                    .add_enabled(controls_enabled, egui::Button::new("Analyze"))
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
                    .add_enabled(controls_enabled, egui::Button::new("Select All Ready"))
                    .clicked()
                {
                    self.select_all();
                }
                if ui
                    .add_enabled(controls_enabled, egui::Button::new("Decompile Selected"))
                    .clicked()
                {
                    self.decompile_selected();
                }
            });

            ui.separator();
            if self.known_sections.is_empty() {
                ui.label("No sections loaded yet. Open an executable and analyze an address.");
            } else {
                let ready_count = self
                    .known_sections
                    .iter()
                    .filter(|section| section.data.analyzed)
                    .count();
                let selected_count = self
                    .known_sections
                    .iter()
                    .filter(|section| section.selected && section.data.analyzed)
                    .count();
                ui.small(format!(
                    "{selected_count} ready selected / {ready_count} ready / {} total",
                    self.known_sections.len()
                ));
                ui.add_space(4.0);

                let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 16.0;
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    self.known_sections.len(),
                    |ui, row_range| {
                        for row in row_range {
                            let section = &mut self.known_sections[row];
                            if !section.data.analyzed {
                                section.selected = false;
                            }
                            let is_hovered =
                                hovered_parent_start == Some(section.data.start_address);
                            let fill = if is_hovered {
                                Color32::from_rgba_unmultiplied(15, 108, 189, 30)
                            } else {
                                ui.visuals().extreme_bg_color
                            };
                            egui::Frame::group(ui.style())
                                .fill(fill)
                                .stroke(Stroke::new(
                                    1.0,
                                    if is_hovered {
                                        Color32::from_rgb(15, 108, 189)
                                    } else {
                                        ui.visuals().widgets.noninteractive.bg_stroke.color
                                    },
                                ))
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    ui.horizontal(|ui| {
                                        if section.data.analyzed {
                                            ui.checkbox(&mut section.selected, "");
                                        } else {
                                            let mut disabled = false;
                                            ui.add_enabled(
                                                false,
                                                egui::Checkbox::new(&mut disabled, ""),
                                            );
                                        }
                                        ui.monospace(format!("0x{:X}", section.data.start_address));
                                        if let Some(end) = section.data.end_address {
                                            ui.label(format!(".. 0x{end:X}"));
                                        }
                                        ui.add_space(8.0);
                                        ui.small(if section.data.analyzed {
                                            "Ready"
                                        } else {
                                            "Analyze to enable"
                                        });
                                    });
                                });
                        }
                    },
                );
            }
        });

        if let Some(address) = analyze_request {
            self.analyze_section_from_address(&address);
        }
    }

    pub(super) fn render_optimization_panel(&mut self, ui: &mut egui::Ui) {
        let controls_enabled = !self.is_busy();
        let mut persist_requested = false;
        let mut apply_settings = false;
        let mut apply_buffer = false;
        let mut clear_buffer = false;
        let mut reset_draft = false;
        let mut restore_defaults = false;
        let mut new_script = false;
        let mut open_script = false;
        let mut save_script = false;
        let mut save_script_as = false;
        let mut load_preset = None;
        let mut remove_preset = None;

        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                if let Some(message) = &self.optimization_status_message {
                    ui.add_space(4.0);
                    ui.colored_label(Color32::from_rgb(0xCA, 0x50, 0x10), message);
                }
                ui.add_space(8.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    persist_requested |= render_optimization_radio(
                        ui, &mut self.optimization_draft,
                    );

                    ui.add_space(8.0);
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(RichText::new("Saved Scripts").strong());
                        if self.optimization_scripts.is_empty() {
                            ui.small("No saved `.fb` presets yet.");
                        } else {
                            for (index, preset) in self.optimization_scripts.iter_mut().enumerate()
                            {
                                let mut load_clicked = false;
                                let mut remove_clicked = false;
                                egui::Frame::group(ui.style()).show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        if ui.checkbox(&mut preset.enabled, "").changed() {
                                            persist_requested = true;
                                        }
                                        ui.label(RichText::new(&preset.name).strong());
                                        if preset.applied_enabled {
                                            ui.small("Applied");
                                        }
                                        load_clicked = ui.small_button("Load").clicked();
                                        remove_clicked = ui.small_button("Remove").clicked();
                                    });
                                    ui.small(&preset.path);
                                });
                                ui.add_space(4.0);
                                if load_clicked {
                                    load_preset = Some(index);
                                }
                                if remove_clicked {
                                    remove_preset = Some(index);
                                }
                            }
                        }
                    });
                });
            });

            columns[1].vertical(|ui| {
                ui.label(RichText::new("`.fb` Script Editor").strong());
                ui.add_space(4.0);
                let path_label = self
                    .optimization_editor_path
                    .as_deref()
                    .unwrap_or("Unsaved buffer");
                ui.small(path_label);
                let applied_buffer_label = if self.optimization_applied_buffer_script.is_some() {
                    "Buffer script is currently applied."
                } else {
                    "Buffer script is not applied."
                };
                ui.small(applied_buffer_label);
                ui.add_space(8.0);

                ui.horizontal_wrapped(|ui| {
                    if ui.button("New").clicked() {
                        new_script = true;
                    }
                    if ui.button("Open").clicked() {
                        open_script = true;
                    }
                    if ui.button("Save").clicked() {
                        save_script = true;
                    }
                    if ui.button("Save As").clicked() {
                        save_script_as = true;
                    }
                });
                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .add_enabled(controls_enabled, egui::Button::new("Apply Optimization"))
                        .clicked()
                    {
                        apply_settings = true;
                    }
                    if ui
                        .add_enabled(controls_enabled, egui::Button::new("Apply Buffer"))
                        .clicked()
                    {
                        apply_buffer = true;
                    }
                    if ui
                        .add_enabled(controls_enabled, egui::Button::new("Clear Applied Buffer"))
                        .clicked()
                    {
                        clear_buffer = true;
                    }
                    if ui.button("Reset To Applied").clicked() {
                        reset_draft = true;
                    }
                    if ui.button("Restore Defaults").clicked() {
                        restore_defaults = true;
                    }
                });
                ui.add_space(8.0);
                let response = ui.add(
                    egui::TextEdit::multiline(&mut self.optimization_editor_buffer)
                        .desired_rows(28)
                        .font(egui::TextStyle::Monospace)
                        .hint_text("Write a .fb optimization script here."),
                );
                if response.changed() {
                    persist_requested = true;
                }
            });
        });

        if persist_requested {
            self.persist_optimization_state();
        }
        if let Some(index) = load_preset {
            self.load_optimization_preset_into_editor(index);
        }
        if let Some(index) = remove_preset {
            self.remove_optimization_script_preset(index);
        }
        if new_script {
            self.new_optimization_script();
        }
        if open_script {
            self.open_optimization_script();
        }
        if save_script {
            self.save_optimization_script();
        }
        if save_script_as {
            self.save_optimization_script_as();
        }
        if restore_defaults {
            self.restore_default_optimization_draft();
        }
        if reset_draft {
            self.reset_optimization_draft();
        }
        if apply_buffer {
            self.apply_optimization_buffer();
        }
        if clear_buffer {
            self.clear_applied_optimization_buffer();
        }
        if apply_settings {
            self.apply_optimization_settings();
        }
    }

    pub(super) fn render_assembly_panel(&mut self, ui: &mut egui::Ui) {
        let Some(result) = self.decompile_result.clone() else {
            ui.label("No assembly data available. Select sections and run decompilation.");
            return;
        };

        // Show keystone availability message
        #[cfg(not(feature = "keystone"))]
        {
            ui.horizontal(|ui| {
                ui.colored_label(
                    Color32::from_rgb(0xCA, 0x50, 0x10),
                    "Assembly editing disabled (build with 'keystone' feature to enable)",
                );
            });
            ui.add_space(4.0);
        }

        let mut hovered_index = None;

        let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 12.0;
        egui::ScrollArea::vertical().show_rows(
            ui,
            row_height,
            result.data.assembly.len(),
            |ui, row_range| {
                for row in row_range {
                    let assembly = &result.data.assembly[row];
                    let is_hovered = self.hovered_assembly_index == Some(assembly.index);
                    let is_selected = self.selected_assembly_row == Some(row);
                    let gutter = format!("{:>5}", assembly.index);
                    let response = render_code_row(
                        ui,
                        &gutter,
                        RichText::new(&assembly.data).monospace(),
                        Some(&assembly.data),
                        result.colors.get(&assembly.index).copied(),
                        is_hovered,
                        is_selected,
                    );
                    if response.hovered() {
                        hovered_index = Some(assembly.index);
                    }
                    #[cfg(feature = "keystone")]
                    if response.clicked() {
                        // Update selection
                        self.selected_assembly_row = Some(row);
                        self.selected_ir_row = None;
                        self.selected_ast_path = None;
                        // Update assembly editor window
                        let draft = AssemblyEditorDraft::from_display_text(&assembly.data);
                        self.assembly_editor.set_assembly(row, draft);
                    }
                }
            },
        );

        if let Some(index) = hovered_index {
            self.hover_candidate = Some(index);
        }
    }

    pub(super) fn render_ir_panel(&mut self, ui: &mut egui::Ui) {
        let Some(result) = self.decompile_result.clone() else {
            ui.label("No IR data available. Select sections and run decompilation.");
            return;
        };

        let mut hovered_index = None;

        let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 12.0;
        egui::ScrollArea::vertical().show_rows(
            ui,
            row_height,
            result.data.ir.len(),
            |ui, row_range| {
                for row in row_range {
                    let ir = &result.data.ir[row];
                    let is_hovered = self.hovered_assembly_index == Some(ir.parents_assembly_index);
                    let is_selected = self.selected_ir_row == Some(row);
                    let gutter = format!("a{:>4}", ir.parents_assembly_index);
                    let response = render_code_row(
                        ui,
                        &gutter,
                        RichText::new(&ir.data).monospace(),
                        Some(&ir.data),
                        result.colors.get(&ir.parents_assembly_index).copied(),
                        is_hovered,
                        is_selected,
                    );
                    if response.hovered() {
                        hovered_index = Some(ir.parents_assembly_index);
                    }
                    if response.clicked() {
                        // Update selection
                        self.selected_assembly_row = None;
                        self.selected_ir_row = Some(row);
                        self.selected_ast_path = None;
                        // Update IR editor window
                        let draft = IrEditorDraft::from_text(&ir.data);
                        self.ir_editor.set_ir(row, draft);
                    }
                }
            },
        );

        if let Some(index) = hovered_index {
            self.hover_candidate = Some(index);
        }
    }

    pub(super) fn render_ast_panel(&mut self, ui: &mut egui::Ui) {
        let Some(result) = self.decompile_result.clone() else {
            ui.label("No decompiled AST available. Select sections and run decompilation.");
            return;
        };

        if let Some(message) = &result.data.ast_sync_message {
            ui.colored_label(Color32::from_rgb(0xCA, 0x50, 0x10), message);
            ui.add_space(4.0);
        }

        // Show IR comments toggle
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_ir_comments, "Show IR origin comments");
        });
        ui.add_space(4.0);

        // Use the actual AST object for tree rendering
        if let Some(ref ast) = result.ast {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let styles = crate::ast_editor::styles::AstStyles::default();
                let renderer = crate::ast_editor::tree_renderer::AstTreeRenderer::new(
                    ast,
                    &styles,
                    self.selected_ast_path.as_ref(),
                    self.show_ir_comments,
                );

                if let Some(clicked_path) = renderer.render(ui) {
                    // Deselect others
                    self.selected_assembly_row = None;
                    self.selected_ir_row = None;
                    self.selected_ast_path = Some(clicked_path.clone());

                    // Create appropriate draft for the clicked node
                    let draft = create_ast_node_draft(&clicked_path, ast);
                    self.ast_editor.set_ast_node(clicked_path.clone(), draft);

                    // Also update legacy editor target for compatibility
                    // Map the path to a row number (simplified for now)
                    let row = path_to_row(&clicked_path);
                    self.select_editor_target(EditorTarget {
                        layer: EditorLayer::Ast,
                        row,
                    });
                }
            });
        } else {
            // Fallback to row-based display if AST object not available
            ui.label("AST object not available. Showing raw text:");
            let ast_lines = &result.data.ast;
            let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 8.0;
            egui::ScrollArea::vertical().show_rows(
                ui,
                row_height,
                ast_lines.len(),
                |ui, row_range| {
                    for row in row_range {
                        let ast_line = &ast_lines[row];
                        let is_selected = self
                            .editor_target
                            .as_ref()
                            .map(|t| t.layer == EditorLayer::Ast && t.row == row)
                            .unwrap_or(false);

                        let gutter = format!("{:>4}", ast_line.row + 1);
                        let response = render_code_row(
                            ui,
                            &gutter,
                            highlight_ast_line_egui(&ast_line.data, ui),
                            Some(&ast_line.data),
                            None,
                            false,
                            is_selected,
                        );

                        if response.clicked() {
                            self.select_editor_target(EditorTarget {
                                layer: EditorLayer::Ast,
                                row,
                            });
                        }
                    }
                },
            );
        }
    }

    fn render_export_preview(&mut self, ui: &mut egui::Ui) {
        let Some(json) = self.exported_patch_json.clone() else {
            return;
        };

        ui.separator();
        ui.label(RichText::new("Patch Preview").strong());
        let mut preview = json;
        ui.add(
            egui::TextEdit::multiline(&mut preview)
                .desired_rows(10)
                .font(egui::TextStyle::Monospace)
                .interactive(false),
        );
    }
}

fn render_code_row(
    ui: &mut egui::Ui,
    gutter: &str,
    text: impl Into<egui::WidgetText>,
    tooltip: Option<&str>,
    accent_color: Option<Color32>,
    is_hovered: bool,
    is_selected: bool,
) -> egui::Response {
    let fill = if is_selected {
        Color32::from_rgba_unmultiplied(15, 108, 189, 42)
    } else if is_hovered {
        accent_color
            .map(|color| Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 26))
            .unwrap_or_else(|| Color32::from_rgba_unmultiplied(15, 108, 189, 20))
    } else {
        ui.visuals().extreme_bg_color
    };
    let stroke_color = if is_selected || is_hovered {
        accent_color.unwrap_or_else(|| Color32::from_rgb(15, 108, 189))
    } else {
        ui.visuals().widgets.noninteractive.bg_stroke.color
    };
    let text = text.into();

    let response = egui::Frame::group(ui.style())
        .fill(fill)
        .stroke(Stroke::new(1.0, stroke_color))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let gutter_response = ui.add_sized(
                    [72.0, 0.0],
                    egui::Label::new(
                        RichText::new(format!("{gutter:>6} │"))
                            .monospace()
                            .color(ui.visuals().weak_text_color()),
                    )
                    .truncate(),
                );
                let text_response = ui.add_sized(
                    [ui.available_width(), 0.0],
                    egui::Label::new(text)
                        .sense(egui::Sense::click())
                        .truncate(),
                );
                gutter_response.union(text_response)
            })
            .inner
        })
        .inner;

    if let Some(tooltip) = tooltip {
        response.on_hover_text(tooltip)
    } else {
        response
    }
}

fn highlight_ast_line_egui(text: &str, ui: &egui::Ui) -> egui::text::LayoutJob {
    use egui::text::{LayoutJob, TextFormat};
    let mut job = LayoutJob::default();

    let keywords = [
        "if", "else", "while", "for", "return", "switch", "case", "default", "goto", "break",
        "continue", "void", "int", "char", "float", "double", "bool", "struct", "union", "int8_t",
        "int16_t", "int32_t", "int64_t", "uint8_t", "uint16_t", "uint32_t", "uint64_t",
    ];

    let mut current_word = String::new();
    let mut iter = text.chars().peekable();

    let normal_format = TextFormat::simple(
        egui::TextStyle::Monospace.resolve(ui.style()),
        ui.visuals().text_color(),
    );
    let keyword_format = TextFormat::simple(
        egui::TextStyle::Monospace.resolve(ui.style()),
        Color32::from_rgb(0, 200, 200),
    );
    let comment_format = TextFormat::simple(
        egui::TextStyle::Monospace.resolve(ui.style()),
        Color32::DARK_GRAY,
    );

    while let Some(c) = iter.next() {
        if c.is_alphanumeric() || c == '_' {
            current_word.push(c);
        } else {
            if !current_word.is_empty() {
                if keywords.contains(&current_word.as_str()) {
                    job.append(&current_word, 0.0, keyword_format.clone());
                } else {
                    job.append(&current_word, 0.0, normal_format.clone());
                }
                current_word.clear();
            }
            if c == '/' && iter.peek() == Some(&'/') {
                let rest: String = std::iter::once(c).chain(iter).collect();
                job.append(&rest, 0.0, comment_format.clone());
                break;
            } else if c == '/' && iter.peek() == Some(&'*') {
                let rest: String = std::iter::once(c).chain(iter).collect();
                job.append(&rest, 0.0, comment_format.clone());
                break;
            } else {
                job.append(&c.to_string(), 0.0, normal_format.clone());
            }
        }
    }
    if !current_word.is_empty() {
        if keywords.contains(&current_word.as_str()) {
            job.append(&current_word, 0.0, keyword_format.clone());
        } else {
            job.append(&current_word, 0.0, normal_format.clone());
        }
    }

    job
}

fn render_optimization_radio(
    ui: &mut egui::Ui,
    draft: &mut OptimizationSettings,
) -> bool {
    struct RadioField {
        label: &'static str,
        description: &'static str,
        get: fn(&OptimizationSettings) -> bool,
        set: fn(&mut OptimizationSettings, bool),
    }

    const FIELDS: &[RadioField] = &[
        RadioField { label: "IR analyzation", description: "Builds the IR analysis layer used by later AST passes.", get: |s| s.ir_analyzation, set: |s, v| s.ir_analyzation = v },
        RadioField { label: "Parameter analyzation", description: "Infers function parameters from recovered usage.", get: |s| s.parameter_analyzation, set: |s, v| s.parameter_analyzation = v },
        RadioField { label: "Call argument analyzation", description: "Propagates argument information into recovered calls.", get: |s| s.call_argument_analyzation, set: |s, v| s.call_argument_analyzation = v },
        RadioField { label: "Name recovery", description: "Recovers variable and helper names when possible.", get: |s| s.name_recovery, set: |s, v| s.name_recovery = v },
        RadioField { label: "Signedness inference", description: "Refines integer semantics from instruction behavior.", get: |s| s.signedness_inference, set: |s, v| s.signedness_inference = v },
        RadioField { label: "Constant folding", description: "Evaluates constant expressions during optimization.", get: |s| s.constant_folding, set: |s, v| s.constant_folding = v },
        RadioField { label: "Copy propagation", description: "Eliminates temporary copies when values can be forwarded.", get: |s| s.copy_propagation, set: |s, v| s.copy_propagation = v },
        RadioField { label: "Expression inlining", description: "Inlines short temporary expressions into their uses.", get: |s| s.expression_inlining, set: |s, v| s.expression_inlining = v },
        RadioField { label: "Dead store elimination", description: "Removes writes that never affect later behavior.", get: |s| s.dead_store_elimination, set: |s, v| s.dead_store_elimination = v },
        RadioField { label: "Collapse unused variable", description: "Drops redundant variables that do not survive analysis.", get: |s| s.collapse_unused_varaible, set: |s, v| s.collapse_unused_varaible = v },
        RadioField { label: "Lifetime scoping", description: "Shrinks recovered variable lifetimes around real usage.", get: |s| s.lifetime_scoping, set: |s, v| s.lifetime_scoping = v },
        RadioField { label: "Control flow cleanup", description: "Removes structural noise before higher-level recovery.", get: |s| s.control_flow_cleanup, set: |s, v| s.control_flow_cleanup = v },
        RadioField { label: "Loop analyzation", description: "Recovers loop constructs from CFG structure.", get: |s| s.loop_analyzation, set: |s, v| s.loop_analyzation = v },
        RadioField { label: "Ternary recovery", description: "Rebuilds ternary expressions from compact branches.", get: |s| s.ternary_recovery, set: |s, v| s.ternary_recovery = v },
        RadioField { label: "Boolean recovery", description: "Normalizes predicate-heavy code into boolean expressions.", get: |s| s.boolean_recovery, set: |s, v| s.boolean_recovery = v },
        RadioField { label: "Switch reconstruction", description: "Detects and prints switch-style control flow.", get: |s| s.switch_reconstruction, set: |s, v| s.switch_reconstruction = v },
        RadioField { label: "Early return normalization", description: "Prefers normalized early-return shapes in the AST.", get: |s| s.early_return_normalization, set: |s, v| s.early_return_normalization = v },
        RadioField { label: "Pattern matching", description: "Runs predefined and selected `.fb` pattern scripts.", get: |s| s.pattern_matching_enabled, set: |s, v| s.pattern_matching_enabled = v },
        RadioField { label: "Embedded passes", description: "Keeps built-in scripted passes enabled in the optimizer.", get: |s| s.use_embedded_passes, set: |s, v| s.use_embedded_passes = v },
        RadioField { label: "Operator canonicalization", description: "Normalizes operator ordering for consistent comparison.", get: |s| s.operator_canonicalization, set: |s, v| s.operator_canonicalization = v },
        RadioField { label: "Magic division recovery", description: "Recovers division from magic-number multiplication patterns.", get: |s| s.magic_division_recovery, set: |s, v| s.magic_division_recovery = v },
        RadioField { label: "Identity simplification", description: "Simplifies identity operations like x+0, x*1.", get: |s| s.identity_simplification, set: |s, v| s.identity_simplification = v },
        RadioField { label: "Bit trick recognition", description: "Recognizes bit manipulation idioms.", get: |s| s.bit_trick_recognition, set: |s, v| s.bit_trick_recognition = v },
        RadioField { label: "Cast minimization", description: "Removes redundant type casts.", get: |s| s.cast_minimization, set: |s, v| s.cast_minimization = v },
        RadioField { label: "Assertion recovery", description: "Recovers assertion patterns from conditional aborts.", get: |s| s.assertion_recovery, set: |s, v| s.assertion_recovery = v },
        RadioField { label: "Do-while recovery", description: "Recovers do-while loops from CFG.", get: |s| s.do_while_recovery, set: |s, v| s.do_while_recovery = v },
        RadioField { label: "Clamp recovery", description: "Recovers clamp/min/max patterns.", get: |s| s.clamp_recovery, set: |s, v| s.clamp_recovery = v },
        RadioField { label: "Loop cleanup", description: "Cleans up loop structure after recovery.", get: |s| s.loop_cleanup, set: |s, v| s.loop_cleanup = v },
        RadioField { label: "If-conversion reversal", description: "Reverses compiler if-conversion optimizations.", get: |s| s.if_conversion_reversal, set: |s, v| s.if_conversion_reversal = v },
        RadioField { label: "Anti-debug AST suppression", description: "Suppresses anti-debug code patterns in output.", get: |s| s.anti_debug_ast_suppression, set: |s, v| s.anti_debug_ast_suppression = v },
        RadioField { label: "Logging suppression", description: "Suppresses logging boilerplate in output.", get: |s| s.logging_suppression, set: |s, v| s.logging_suppression = v },
        RadioField { label: "Static guard suppression", description: "Suppresses static guard patterns in output.", get: |s| s.static_guard_suppression, set: |s, v| s.static_guard_suppression = v },
        RadioField { label: "Security scaffold suppression", description: "Suppresses security scaffold patterns in output.", get: |s| s.security_scaffold_suppression, set: |s, v| s.security_scaffold_suppression = v },
    ];

    let mut changed = false;
    for (i, field) in FIELDS.iter().enumerate() {
        let enabled = (field.get)(draft);
        if ui.radio(enabled, field.label).clicked() {
            // Single-select: disable all, then enable this one (or toggle off)
            for f in FIELDS {
                (f.set)(draft, false);
            }
            if !enabled {
                (field.set)(draft, true);
            }
            changed = true;
        }
        ui.small(field.description);
        if i < FIELDS.len() - 1 {
            ui.add_space(2.0);
        }
    }

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label("Max pass iterations");
        let response = ui.add(
            egui::DragValue::new(&mut draft.max_pass_iterations)
                .range(1..=64)
                .speed(1.0),
        );
        if response.changed() {
            changed = true;
        }
    });
    ui.small("Selected scripts are appended to the built-in pattern list.");

    changed
}

/// Creates an appropriate editor draft for the clicked AST node
fn create_ast_node_draft(
    path: &fireball::abstract_syntax_tree::AstNodePath,
    _ast: &fireball::abstract_syntax_tree::Ast,
) -> crate::model::AstNodeEditorDraft {
    use crate::model::{AstNodeDraftData, AstNodeEditType};

    // For now, create a generic statement draft
    // In the future, this could inspect the actual node and create appropriate drafts
    AstNodeEditorDraft {
        path: path.clone(),
        edit_type: AstNodeEditType::Statement,
        draft_data: AstNodeDraftData::Statement {
            statement_type: "unknown".to_string(),
            replacement: String::new(),
        },
        status_message: None,
    }
}

/// Converts an AST node path to a row number for legacy compatibility
fn path_to_row(path: &fireball::abstract_syntax_tree::AstNodePath) -> usize {
    use fireball::abstract_syntax_tree::AstNodePath;

    match path {
        AstNodePath::Function { index } => *index,
        AstNodePath::Statement {
            function_index,
            statement_path,
        } => {
            // Simple hash for row number
            *function_index * 100 + statement_path.first().copied().unwrap_or(0)
        }
        AstNodePath::Expression {
            function_index,
            statement_path,
            ..
        } => *function_index * 100 + statement_path.first().copied().unwrap_or(0),
        AstNodePath::Variable { function_index, .. } => *function_index * 100,
    }
}

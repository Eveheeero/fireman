use super::{state::FirebatState, tabs::PanelTab};
use crate::model::{EditPosition, EditorDraft, EditorLayer, EditorTarget};
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
                ui.add(
                    egui::TextEdit::singleline(&mut self.analyze_target_address)
                        .desired_width(f32::INFINITY)
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
                ui.small(
                    "Discovered sections must be analyzed before they can be selected or decompiled.",
                );
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
                ui.label(RichText::new("Optimization Settings").strong());
                ui.add_space(4.0);
                let dirty_label = if self.optimization_is_dirty() {
                    "Draft differs from applied configuration"
                } else {
                    "Draft matches applied configuration"
                };
                ui.small(dirty_label);
                if let Some(message) = &self.optimization_status_message {
                    ui.add_space(4.0);
                    ui.colored_label(Color32::from_rgb(0xCA, 0x50, 0x10), message);
                }
                ui.add_space(8.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(RichText::new("Analysis").strong());
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.ir_analyzation,
                            "IR analyzation",
                            "Builds the IR analysis layer used by later AST passes.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.parameter_analyzation,
                            "Parameter analyzation",
                            "Infers function parameters from recovered usage.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.call_argument_analyzation,
                            "Call argument analyzation",
                            "Propagates argument information into recovered calls.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.name_recovery,
                            "Name recovery",
                            "Recovers variable and helper names when possible.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.signedness_inference,
                            "Signedness inference",
                            "Refines integer semantics from instruction behavior.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.auto_comment,
                            "Auto comment",
                            "Emits automatically derived AST comments.",
                        );
                    });

                    ui.add_space(8.0);
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(RichText::new("Simplification").strong());
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.constant_folding,
                            "Constant folding",
                            "Evaluates constant expressions during optimization.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.copy_propagation,
                            "Copy propagation",
                            "Eliminates temporary copies when values can be forwarded.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.expression_inlining,
                            "Expression inlining",
                            "Inlines short temporary expressions into their uses.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.dead_store_elimination,
                            "Dead store elimination",
                            "Removes writes that never affect later behavior.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.collapse_unused_varaible,
                            "Collapse unused variable",
                            "Drops redundant variables that do not survive analysis.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.lifetime_scoping,
                            "Lifetime scoping",
                            "Shrinks recovered variable lifetimes around real usage.",
                        );
                    });

                    ui.add_space(8.0);
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(RichText::new("Structure Recovery").strong());
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.control_flow_cleanup,
                            "Control flow cleanup",
                            "Removes structural noise before higher-level recovery.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.loop_analyzation,
                            "Loop analyzation",
                            "Recovers loop constructs from CFG structure.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.ternary_recovery,
                            "Ternary recovery",
                            "Rebuilds ternary expressions from compact branches.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.boolean_recovery,
                            "Boolean recovery",
                            "Normalizes predicate-heavy code into boolean expressions.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.switch_reconstruction,
                            "Switch reconstruction",
                            "Detects and prints switch-style control flow.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.early_return_normalization,
                            "Early return normalization",
                            "Prefers normalized early-return shapes in the AST.",
                        );
                    });

                    ui.add_space(8.0);
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.label(RichText::new("Pattern Engine").strong());
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.pattern_matching_enabled,
                            "Pattern matching",
                            "Runs predefined and selected `.fb` pattern scripts.",
                        );
                        persist_requested |= render_optimization_toggle(
                            ui,
                            &mut self.optimization_draft.use_embedded_passes,
                            "Embedded passes",
                            "Keeps built-in scripted passes enabled in the optimizer.",
                        );
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label("Max pass iterations");
                            let response = ui.add(
                                egui::DragValue::new(
                                    &mut self.optimization_draft.max_pass_iterations,
                                )
                                .range(1..=64)
                                .speed(1.0),
                            );
                            if response.changed() {
                                persist_requested = true;
                            }
                        });
                        ui.small("Selected scripts are appended to the built-in pattern list.");
                    });

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

        let mut clicked_row = None;
        let mut hovered_index = None;

        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.label(RichText::new("Assembly").strong());
                ui.small("Gutter = assembly row id");
                ui.add_space(4.0);
                let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 12.0;
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    result.data.assembly.len(),
                    |ui, row_range| {
                        for row in row_range {
                            let assembly = &result.data.assembly[row];
                            let is_hovered = self.hovered_assembly_index == Some(assembly.index);
                            let is_selected = matches!(
                                self.editor_target,
                                Some(EditorTarget {
                                    layer: EditorLayer::Assembly,
                                    row: selected_row
                                }) if selected_row == row
                            );
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
                            if response.clicked() {
                                clicked_row = Some(row);
                            }
                        }
                    },
                );
            });

            columns[1].vertical(|ui| {
                self.render_assembly_editor(ui);
            });
        });

        if let Some(index) = hovered_index {
            self.hover_candidate = Some(index);
        }
        if let Some(row) = clicked_row {
            self.select_editor_target(EditorTarget {
                layer: EditorLayer::Assembly,
                row,
            });
        }
    }

    pub(super) fn render_ir_panel(&mut self, ui: &mut egui::Ui) {
        let Some(result) = self.decompile_result.clone() else {
            ui.label("No IR data available. Select sections and run decompilation.");
            return;
        };

        let mut clicked_row = None;
        let mut hovered_index = None;

        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.label(RichText::new("IR").strong());
                ui.small("Gutter = parent assembly row");
                ui.add_space(4.0);
                let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 12.0;
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    result.data.ir.len(),
                    |ui, row_range| {
                        for row in row_range {
                            let ir = &result.data.ir[row];
                            let is_hovered =
                                self.hovered_assembly_index == Some(ir.parents_assembly_index);
                            let is_selected = matches!(
                                self.editor_target,
                                Some(EditorTarget {
                                    layer: EditorLayer::Ir,
                                    row: selected_row
                                }) if selected_row == row
                            );
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
                                clicked_row = Some(row);
                            }
                        }
                    },
                );
            });

            columns[1].vertical(|ui| {
                self.render_ir_editor(ui);
            });
        });

        if let Some(index) = hovered_index {
            self.hover_candidate = Some(index);
        }
        if let Some(row) = clicked_row {
            self.select_editor_target(EditorTarget {
                layer: EditorLayer::Ir,
                row,
            });
        }
    }

    pub(super) fn render_ast_panel(&mut self, ui: &mut egui::Ui) {
        let Some(result) = self.decompile_result.clone() else {
            ui.label("No decompiled AST available. Select sections and run decompilation.");
            return;
        };

        let mut clicked_row = None;

        ui.columns(2, |columns| {
            columns[0].vertical(|ui| {
                ui.label(RichText::new("AST").strong());
                ui.small("Gutter = printed AST line number");
                if let Some(message) = &result.data.ast_sync_message {
                    ui.add_space(4.0);
                    ui.colored_label(Color32::from_rgb(0xCA, 0x50, 0x10), message);
                }
                ui.add_space(4.0);

                let row_height = ui.text_style_height(&egui::TextStyle::Monospace) + 8.0;
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    result.data.ast.len(),
                    |ui, row_range| {
                        for row in row_range {
                            let ast = &result.data.ast[row];
                            let is_selected = matches!(
                                self.editor_target,
                                Some(EditorTarget {
                                    layer: EditorLayer::Ast,
                                    row: selected_row
                                }) if selected_row == row
                            );
                            let gutter = format!("{:>5}", ast.row + 1);
                            let response = render_code_row(
                                ui,
                                &gutter,
                                highlight_ast_line_egui(&ast.data, ui),
                                Some(&ast.data),
                                None,
                                false,
                                is_selected,
                            );
                            if response.clicked() {
                                clicked_row = Some(row);
                            }
                        }
                    },
                );
            });

            columns[1].vertical(|ui| {
                self.render_ast_editor(ui);
            });
        });

        if let Some(row) = clicked_row {
            self.select_editor_target(EditorTarget {
                layer: EditorLayer::Ast,
                row,
            });
        }
    }

    fn render_assembly_editor(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("Assembly Editor").strong());
        let controls_enabled = !self.is_busy();
        let can_export = self.decompile_result.is_some();
        let selected_row = match self.editor_target {
            Some(EditorTarget {
                layer: EditorLayer::Assembly,
                row,
            }) => row,
            _ => {
                ui.add_space(8.0);
                ui.label("Click an assembly row to edit it.");
                return;
            }
        };

        let Some(EditorDraft::Assembly(draft)) = self.editor_draft.as_mut() else {
            ui.add_space(8.0);
            ui.label("No assembly draft is active.");
            return;
        };

        let mut compose_clicked = false;
        let mut apply_clicked = false;
        let mut reset_clicked = false;
        let mut export_clicked = false;

        ui.label(format!("Selected row: {selected_row}"));
        ui.add_space(4.0);
        ui.label("Raw line");
        ui.add(
            egui::TextEdit::multiline(&mut draft.raw_text)
                .desired_rows(5)
                .font(egui::TextStyle::Monospace),
        );

        ui.separator();
        ui.label("Structured fields");
        ui.horizontal(|ui| {
            ui.label("Mnemonic");
            ui.add_sized(
                [100.0, 24.0],
                egui::TextEdit::singleline(&mut draft.mnemonic).font(egui::TextStyle::Monospace),
            );
            ui.label("Operands");
            ui.add(
                egui::TextEdit::singleline(&mut draft.operands).font(egui::TextStyle::Monospace),
            );
        });
        if ui.button("Use mnemonic + operands").clicked() {
            compose_clicked = true;
        }

        if let Some(message) = &draft.status_message {
            ui.add_space(6.0);
            ui.colored_label(Color32::from_rgb(0xCA, 0x50, 0x10), message);
        }

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            apply_clicked = ui
                .add_enabled(controls_enabled, egui::Button::new("Apply"))
                .clicked();
            reset_clicked = ui.button("Reset Draft").clicked();
            export_clicked = ui
                .add_enabled(can_export, egui::Button::new("Export Patch"))
                .clicked();
        });

        if compose_clicked {
            draft.raw_text = draft.compose_line();
            draft.status_message = None;
        }
        if reset_clicked {
            self.reset_editor_draft();
        }
        if apply_clicked {
            self.apply_current_edit();
        }
        if export_clicked {
            self.export_patch();
        }

        self.render_export_preview(ui);
    }

    fn render_ir_editor(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("IR Editor").strong());
        let controls_enabled = !self.is_busy();
        let can_export = self.decompile_result.is_some();
        let selected_row = match self.editor_target {
            Some(EditorTarget {
                layer: EditorLayer::Ir,
                row,
            }) => row,
            _ => {
                ui.add_space(8.0);
                ui.label("Click an IR row to replace it or insert around it.");
                return;
            }
        };

        let Some(EditorDraft::Ir(draft)) = self.editor_draft.as_mut() else {
            ui.add_space(8.0);
            ui.label("No IR draft is active.");
            return;
        };

        let mut compose_clicked = false;
        let mut apply_clicked = false;
        let mut reset_clicked = false;
        let mut export_clicked = false;

        ui.label(format!("Selected row: {selected_row}"));
        ui.add_space(4.0);
        ui.label("Raw statement");
        ui.add(
            egui::TextEdit::multiline(&mut draft.raw_text)
                .desired_rows(5)
                .font(egui::TextStyle::Monospace),
        );

        ui.separator();
        ui.label("Structured fields");
        ui.horizontal(|ui| {
            ui.label("Opcode");
            ui.add_sized(
                [120.0, 24.0],
                egui::TextEdit::singleline(&mut draft.opcode).font(egui::TextStyle::Monospace),
            );
            ui.label("Details");
            ui.add(egui::TextEdit::singleline(&mut draft.detail).font(egui::TextStyle::Monospace));
        });
        if ui.button("Use opcode + details").clicked() {
            compose_clicked = true;
        }

        ui.add_space(6.0);
        egui::ComboBox::from_id_salt("ir-edit-position")
            .selected_text(draft.position.label())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut draft.position, EditPosition::Replace, "Replace");
                ui.selectable_value(&mut draft.position, EditPosition::Before, "Insert Before");
                ui.selectable_value(&mut draft.position, EditPosition::After, "Insert After");
            });

        if let Some(message) = &draft.status_message {
            ui.add_space(6.0);
            ui.colored_label(Color32::from_rgb(0xCA, 0x50, 0x10), message);
        }

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            apply_clicked = ui
                .add_enabled(controls_enabled, egui::Button::new("Apply"))
                .clicked();
            reset_clicked = ui.button("Reset Draft").clicked();
            export_clicked = ui
                .add_enabled(can_export, egui::Button::new("Export Patch"))
                .clicked();
        });

        if compose_clicked {
            draft.raw_text = draft.compose_line();
            draft.status_message = None;
        }
        if reset_clicked {
            self.reset_editor_draft();
        }
        if apply_clicked {
            self.apply_current_edit();
        }
        if export_clicked {
            self.export_patch();
        }

        self.render_export_preview(ui);
    }

    fn render_ast_editor(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("AST Editor").strong());
        let controls_enabled = !self.is_busy();
        let can_export = self.decompile_result.is_some();
        let selected_row = match self.editor_target {
            Some(EditorTarget {
                layer: EditorLayer::Ast,
                row,
            }) => row,
            _ => {
                ui.add_space(8.0);
                ui.label("Click an AST line to replace it or insert before/after it.");
                return;
            }
        };

        let Some(EditorDraft::Ast(draft)) = self.editor_draft.as_mut() else {
            ui.add_space(8.0);
            ui.label("No AST draft is active.");
            return;
        };

        let mut apply_clicked = false;
        let mut reset_clicked = false;
        let mut export_clicked = false;

        ui.label(format!("Selected row: {selected_row}"));
        ui.add_space(4.0);
        ui.label("Statement text");
        ui.add(
            egui::TextEdit::multiline(&mut draft.raw_text)
                .desired_rows(6)
                .font(egui::TextStyle::Monospace),
        );
        ui.add_space(6.0);
        egui::ComboBox::from_id_salt("ast-edit-position")
            .selected_text(draft.position.label())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut draft.position, EditPosition::Replace, "Replace");
                ui.selectable_value(&mut draft.position, EditPosition::Before, "Insert Before");
                ui.selectable_value(&mut draft.position, EditPosition::After, "Insert After");
            });
        ui.add_space(4.0);
        ui.small(
            "AST validation accepts the existing fireball replacement syntax, including `comment ...`, `asm ...`, `ir ...`, and `return`.",
        );

        if let Some(message) = &draft.status_message {
            ui.add_space(6.0);
            ui.colored_label(Color32::from_rgb(0xCA, 0x50, 0x10), message);
        }

        ui.add_space(8.0);
        ui.horizontal(|ui| {
            apply_clicked = ui
                .add_enabled(controls_enabled, egui::Button::new("Apply"))
                .clicked();
            reset_clicked = ui.button("Reset Draft").clicked();
            export_clicked = ui
                .add_enabled(can_export, egui::Button::new("Export Patch"))
                .clicked();
        });

        if reset_clicked {
            self.reset_editor_draft();
        }
        if apply_clicked {
            self.apply_current_edit();
        }
        if export_clicked {
            self.export_patch();
        }

        self.render_export_preview(ui);
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

fn render_optimization_toggle(
    ui: &mut egui::Ui,
    value: &mut bool,
    label: &str,
    description: &str,
) -> bool {
    let response = ui.checkbox(value, label);
    ui.small(description);
    ui.add_space(2.0);
    response.changed()
}

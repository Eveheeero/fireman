use super::{
    app::App,
    types::{OPTIMIZATION_GROUPS, OptimizationFocus, PromptState, View},
};
use crate::{
    license::{self, THIRD_PARTY_DEPS},
    model::{EditorDraft, EditorLayer},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs, Wrap},
};

impl App {
    pub(crate) fn draw(&self, frame: &mut Frame) {
        let [title_area, tabs_area, main_area, status_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .areas(frame.area());

        self.draw_title(frame, title_area);
        self.draw_tabs(frame, tabs_area);
        self.draw_body(frame, main_area);
        self.draw_status(frame, status_area);

        if self.show_license {
            self.draw_license(frame);
        } else if let Some(prompt) = &self.prompt {
            self.draw_prompt(frame, prompt);
        }
    }

    fn draw_title(&self, frame: &mut Frame, area: Rect) {
        let busy = self
            .busy_label
            .as_deref()
            .map(|label| format!("busy: {label}"))
            .unwrap_or_else(|| "idle".to_string());
        let path = self.opened_path.as_deref().unwrap_or("no binary");
        let line = Line::from(vec![
            Span::styled(
                "Fireman",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::raw(path),
            Span::raw("  "),
            Span::styled(busy, Style::default().fg(Color::Yellow)),
        ]);
        frame.render_widget(Paragraph::new(line), area);
    }

    fn draw_tabs(&self, frame: &mut Frame, area: Rect) {
        let tabs = Tabs::new([
            "1 Sections",
            "2 ASM",
            "3 IR",
            "4 AST",
            "5 Editor",
            "6 Opt",
            "7 Patch",
            "8 Logs",
        ])
        .select(self.current_view.index())
        .style(Style::default().fg(Color::Gray))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
        frame.render_widget(tabs, area);
    }

    fn draw_body(&self, frame: &mut Frame, area: Rect) {
        match self.current_view {
            View::Sections => self.draw_sections(frame, area),
            View::Assembly => self.draw_output(frame, area, EditorLayer::Assembly),
            View::Ir => self.draw_output(frame, area, EditorLayer::Ir),
            View::Ast => self.draw_output(frame, area, EditorLayer::Ast),
            View::Editor => self.draw_editor(frame, area),
            View::Optimization => self.draw_optimization(frame, area),
            View::Patch => self.draw_patch(frame, area),
            View::Logs => self.draw_logs(frame, area),
        }
    }

    fn draw_sections(&self, frame: &mut Frame, area: Rect) {
        let [list_area, detail_area] =
            Layout::horizontal([Constraint::Percentage(68), Constraint::Percentage(32)])
                .areas(area);

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
        let total_count = self.known_sections.len();
        let title = format!("Sections ({selected_count}/{ready_count} ready, {total_count} total)");

        let items: Vec<ListItem> = if self.known_sections.is_empty() {
            vec![ListItem::new(
                "No sections yet. Press `a` to analyze an address or `g` to scan everything.",
            )]
        } else {
            self.known_sections
                .iter()
                .map(|section| {
                    let is_selected = section.selected && section.data.analyzed;
                    let marker = if is_selected { "[x]" } else { "[ ]" };
                    let end = section
                        .data
                        .end_address
                        .map(|value| format!("{value:#010x}"))
                        .unwrap_or_else(|| "??????????".to_string());
                    let status_style = if section.data.analyzed {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    let item_style = if section.data.analyzed {
                        Style::default()
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    let status = if section.data.analyzed {
                        "ready"
                    } else {
                        "pending"
                    };

                    ListItem::new(Line::from(vec![
                        Span::raw(format!("{marker} ")),
                        Span::raw(format!("{:#010x}", section.data.start_address)),
                        Span::raw(" - "),
                        Span::raw(end),
                        Span::raw("  "),
                        Span::styled(status, status_style),
                    ]))
                    .style(item_style)
                })
                .collect()
        };
        let mut state = ListState::default()
            .with_selected(self.selection(Some(self.section_cursor), self.known_sections.len()));
        let list = List::new(items)
            .block(self.block(&title))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, list_area, &mut state);

        let detail = self
            .known_sections
            .get(self.section_cursor)
            .map(|section| {
                let is_selected = section.selected && section.data.analyzed;
                let state_label = if section.data.analyzed {
                    "Ready"
                } else {
                    "Pending analysis"
                };
                let mut lines = vec![
                    Line::from(format!("Start: {:#010x}", section.data.start_address)),
                    Line::from(format!(
                        "End: {}",
                        section
                            .data
                            .end_address
                            .map(|value| format!("{value:#010x}"))
                            .unwrap_or_else(|| "Unknown".to_string())
                    )),
                    Line::from(format!("State: {state_label}")),
                    Line::from(format!(
                        "Selected: {}",
                        if is_selected { "Yes" } else { "No" }
                    )),
                    Line::from(""),
                    Line::from(format!("Ready sections: {ready_count}")),
                    Line::from(format!("Selected ready sections: {selected_count}")),
                    Line::from(""),
                ];
                if section.data.analyzed {
                    lines.push(Line::from("Space/Enter toggles this section."));
                } else {
                    lines.push(Line::from(
                        "Analyze this section before selecting or decompiling it.",
                    ));
                }
                lines.push(Line::from("s/Ctrl+A toggles all ready sections."));
                lines.push(Line::from("d decompiles the ready selection."));
                lines
            })
            .unwrap_or_else(|| {
                vec![
                    Line::from("No section is focused."),
                    Line::from(""),
                    Line::from("`a` Analyze address"),
                    Line::from("`g` Analyze all"),
                    Line::from("`o` Open binary"),
                ]
            });
        frame.render_widget(
            Paragraph::new(detail)
                .block(self.block("Detail"))
                .wrap(Wrap { trim: false }),
            detail_area,
        );
    }

    fn draw_output(&self, frame: &mut Frame, area: Rect, layer: EditorLayer) {
        let Some(outputs) = &self.outputs else {
            let title = match layer {
                EditorLayer::Assembly => "Assembly",
                EditorLayer::Ir => "IR",
                EditorLayer::Ast => "AST",
            };
            frame.render_widget(
                Paragraph::new("No decompile result yet. Select sections and press `d`.")
                    .block(self.block(title)),
                area,
            );
            return;
        };

        let hovered_asm = self.hovered_assembly_index;

        let (title, lines, selected, mut info) = match layer {
            EditorLayer::Assembly => (
                format!("Assembly ({})", outputs.assembly.len()),
                outputs
                    .assembly
                    .iter()
                    .map(|row| {
                        let highlight = hovered_asm == Some(row.index);
                        let gutter_style = if highlight {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::DarkGray)
                        };
                        let text_style = if highlight {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default()
                        };
                        code_list_item(
                            format!("{:>5}", row.index),
                            row.data.clone(),
                            gutter_style,
                            text_style,
                        )
                    })
                    .collect::<Vec<_>>(),
                self.selection(Some(self.assembly_cursor), outputs.assembly.len()),
                outputs
                    .assembly
                    .get(self.assembly_cursor)
                    .map(|row| {
                        vec![
                            Line::from(format!("Assembly row: {}", row.index)),
                            Line::from(format!(
                                "Parent block: {:#010x}",
                                row.parents_start_address
                            )),
                        ]
                    })
                    .unwrap_or_else(|| vec![Line::from("No assembly rows")]),
            ),
            EditorLayer::Ir => (
                format!("IR ({})", outputs.ir.len()),
                outputs
                    .ir
                    .iter()
                    .map(|row| {
                        let highlight = hovered_asm == Some(row.parents_assembly_index);
                        let gutter_style = if highlight {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::DarkGray)
                        };
                        let text_style = if highlight {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default()
                        };
                        code_list_item(
                            format!("a{:>4}", row.parents_assembly_index),
                            row.data.clone(),
                            gutter_style,
                            text_style,
                        )
                    })
                    .collect::<Vec<_>>(),
                self.selection(Some(self.ir_cursor), outputs.ir.len()),
                outputs
                    .ir
                    .get(self.ir_cursor)
                    .map(|row| {
                        vec![
                            Line::from(format!("IR row: {}", self.ir_cursor)),
                            Line::from(format!(
                                "Parent assembly row: {}",
                                row.parents_assembly_index
                            )),
                            Line::from("Gutter: parent assembly row"),
                        ]
                    })
                    .unwrap_or_else(|| vec![Line::from("No IR rows")]),
            ),
            EditorLayer::Ast => {
                let mut selection_info = outputs
                    .ast
                    .get(self.ast_cursor)
                    .map(|row| vec![Line::from(format!("AST line: {}", row.row + 1))])
                    .unwrap_or_else(|| vec![Line::from("No AST rows")]);
                selection_info.push(Line::from("Gutter: printed AST line number"));
                if let Some(message) = &outputs.ast_sync_message {
                    selection_info.push(Line::from(""));
                    selection_info.push(Line::from(Span::styled(
                        format!("Sync: {message}"),
                        Style::default().fg(Color::Yellow),
                    )));
                }
                (
                    format!("AST ({})", outputs.ast.len()),
                    outputs
                        .ast
                        .iter()
                        .map(|row| {
                            code_list_item(
                                format!("{:>5}", row.row + 1),
                                row.data.clone(),
                                Style::default().fg(Color::DarkGray),
                                Style::default(),
                            )
                        })
                        .collect::<Vec<_>>(),
                    self.selection(Some(self.ast_cursor), outputs.ast.len()),
                    selection_info,
                )
            }
        };

        let [list_area, info_area] =
            Layout::horizontal([Constraint::Percentage(78), Constraint::Percentage(22)])
                .areas(area);
        let mut state = ListState::default().with_selected(selected);
        let list = List::new(lines)
            .block(self.block(&title))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, list_area, &mut state);

        info.push(Line::from(""));
        info.push(Line::from("Enter: load editor"));
        info.push(Line::from("e: edit line"));
        info.push(Line::from("Home/End/Pg: navigate"));
        frame.render_widget(
            Paragraph::new(info)
                .block(self.block("Selection"))
                .wrap(Wrap { trim: false }),
            info_area,
        );
    }

    fn draw_editor(&self, frame: &mut Frame, area: Rect) {
        let Some(target) = self.editor_target else {
            frame.render_widget(
                Paragraph::new("Select an ASM, IR, or AST row and press Enter or `e`.")
                    .block(self.block("Editor")),
                area,
            );
            return;
        };
        let layer_label = match target.layer {
            EditorLayer::Assembly => "Assembly",
            EditorLayer::Ir => "IR",
            EditorLayer::Ast => "AST",
        };
        let body = match &self.editor_draft {
            Some(EditorDraft::Assembly(draft)) => vec![
                Line::from(format!("Layer: {layer_label}")),
                Line::from(format!("Row: {}", target.row)),
                Line::from(""),
                Line::from(format!("Mnemonic: {}", draft.mnemonic)),
                Line::from(format!("Operands: {}", draft.operands)),
                Line::from(""),
                Line::from(format!("Composed: {}", draft.compose_line())),
            ],
            Some(EditorDraft::Ir(draft)) => vec![
                Line::from(format!("Layer: {layer_label}")),
                Line::from(format!("Row: {}", target.row)),
                Line::from(format!("Position: {}", draft.position.label())),
                Line::from(""),
                Line::from(format!("Opcode: {}", draft.opcode)),
                Line::from(format!("Detail: {}", draft.detail)),
                Line::from(""),
                Line::from(format!("Composed: {}", draft.compose_line())),
            ],
            Some(EditorDraft::Ast(draft)) => vec![
                Line::from(format!("Layer: {layer_label}")),
                Line::from(format!("Row: {}", target.row)),
                Line::from(format!("Position: {}", draft.position.label())),
                Line::from(""),
                Line::from("Text:"),
                Line::from(draft.raw_text.clone()),
            ],
            None => vec![Line::from("No editor draft loaded.")],
        };
        frame.render_widget(
            Paragraph::new(body)
                .block(self.block("Editor"))
                .wrap(Wrap { trim: false }),
            area,
        );
    }

    fn draw_optimization(&self, frame: &mut Frame, area: Rect) {
        let [settings_area, scripts_area, buffer_area] = Layout::horizontal([
            Constraint::Percentage(34),
            Constraint::Percentage(28),
            Constraint::Percentage(38),
        ])
        .areas(area);

        // Grouped settings with descriptions
        let mut settings_items: Vec<ListItem> = Vec::new();
        for group in OPTIMIZATION_GROUPS {
            settings_items.push(
                ListItem::new(Line::from(Span::styled(
                    format!("-- {} --", group.label),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )))
                .style(Style::default()),
            );
            for field in group.fields {
                let enabled = (field.get)(&self.optimization.draft_settings);
                let applied = (field.get)(&self.optimization.applied_settings);
                let dirty_marker = if enabled != applied { " *" } else { "" };
                let checkbox = if enabled { "[x]" } else { "[ ]" };
                settings_items.push(ListItem::new(vec![
                    Line::from(format!(" {checkbox} {}{dirty_marker}", field.label)),
                    Line::from(Span::styled(
                        format!("     {}", field.description),
                        Style::default().fg(Color::DarkGray),
                    )),
                ]));
            }
        }

        // Map cursor to visual index (accounting for group headers)
        let visual_index = cursor_to_visual_index(self.optimization_setting_cursor);
        let mut settings_state = ListState::default().with_selected(Some(visual_index));
        let settings_block = self.focus_block(
            "Settings",
            self.optimization_focus == OptimizationFocus::Settings,
        );
        frame.render_stateful_widget(
            List::new(settings_items)
                .block(settings_block)
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                .highlight_symbol("> "),
            settings_area,
            &mut settings_state,
        );

        let script_items: Vec<ListItem> = if self.optimization.script_presets.is_empty() {
            vec![ListItem::new("No .fb scripts registered")]
        } else {
            self.optimization
                .script_presets
                .iter()
                .map(|preset| {
                    let draft = if preset.enabled { "x" } else { " " };
                    let applied = if preset.applied_enabled { "a" } else { " " };
                    ListItem::new(format!("[{draft}/{applied}] {}", preset.name))
                })
                .collect()
        };
        let mut script_state = ListState::default().with_selected(self.selection(
            Some(self.optimization_script_cursor),
            self.optimization.script_presets.len(),
        ));
        frame.render_stateful_widget(
            List::new(script_items)
                .block(self.focus_block(
                    "Scripts",
                    self.optimization_focus == OptimizationFocus::Scripts,
                ))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                .highlight_symbol("> "),
            scripts_area,
            &mut script_state,
        );

        let buffer_path = self
            .optimization
            .editor_path
            .as_deref()
            .unwrap_or("Unsaved buffer");
        let applied = if self.optimization.applied_buffer_script.is_some() {
            "applied"
        } else {
            "not applied"
        };
        let buffer_preview = if self.optimization.editor_buffer.is_empty() {
            "Buffer is empty.".to_string()
        } else {
            self.optimization.editor_buffer.clone()
        };
        let buffer_text = Text::from(vec![
            Line::from(format!("Path: {buffer_path}")),
            Line::from(format!("Status: {applied}")),
            Line::from(""),
            Line::from(buffer_preview),
        ]);
        frame.render_widget(
            Paragraph::new(buffer_text)
                .block(self.focus_block(
                    "Buffer",
                    self.optimization_focus == OptimizationFocus::Buffer,
                ))
                .wrap(Wrap { trim: false }),
            buffer_area,
        );
    }

    fn draw_patch(&self, frame: &mut Frame, area: Rect) {
        let content = self
            .patch_preview
            .clone()
            .unwrap_or_else(|| "No exported patch preview. Press `x` to export.".to_string());
        frame.render_widget(
            Paragraph::new(content)
                .block(self.block("Patch Preview"))
                .scroll((self.patch_scroll as u16, 0))
                .wrap(Wrap { trim: false }),
            area,
        );
    }

    fn draw_logs(&self, frame: &mut Frame, area: Rect) {
        let log_count = self.logs.len();
        let title = format!("Logs ({log_count})");
        let body = if self.logs.is_empty() {
            "No logs yet.".to_string()
        } else {
            self.logs.iter().cloned().collect::<Vec<_>>().join("\n")
        };
        frame.render_widget(
            Paragraph::new(body)
                .block(self.block(&title))
                .scroll((self.log_scroll as u16, 0))
                .wrap(Wrap { trim: false }),
            area,
        );
    }

    fn draw_status(&self, frame: &mut Frame, area: Rect) {
        let text = self
            .keybindings()
            .into_iter()
            .map(|(key, label)| format!("[{key} {label}]"))
            .collect::<Vec<_>>()
            .join(" ");
        let lines = vec![Line::from(self.top_message.clone()), Line::from(text)];
        frame.render_widget(
            Paragraph::new(lines)
                .block(Block::default().borders(Borders::TOP))
                .wrap(Wrap { trim: false }),
            area,
        );
    }

    fn draw_prompt(&self, frame: &mut Frame, prompt: &PromptState) {
        let has_browser = prompt.file_browser.is_some();
        let percent_x = if has_browser {
            80
        } else if prompt.multiline {
            76
        } else {
            64
        };
        let percent_y = if has_browser {
            80
        } else if prompt.multiline {
            70
        } else {
            24
        };
        let area = centered_rect(percent_x, percent_y, frame.area());

        frame.render_widget(Clear, area);
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .title(prompt.title.as_str()),
            area,
        );

        if has_browser {
            let inner = inner_block_area(area);
            let [browser_area, input_area, help_area] = Layout::vertical([
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .areas(inner);

            self.draw_file_browser(frame, browser_area, prompt);
            self.draw_prompt_input(frame, input_area, prompt);
            frame.render_widget(Paragraph::new(prompt.help.as_str()), help_area);
        } else {
            let inner = inner_block_area(area);
            let [_title_area, body_area, help_area] = Layout::vertical([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .areas(inner);

            let body = if prompt.multiline {
                format!("{}\n", prompt.text)
            } else {
                let (head, tail) = prompt.text.split_at(prompt.cursor);
                format!("{head}|{tail}")
            };
            frame.render_widget(
                Paragraph::new(body)
                    .block(Block::default().borders(Borders::TOP))
                    .wrap(Wrap { trim: false }),
                body_area,
            );
            frame.render_widget(Paragraph::new(prompt.help.as_str()), help_area);
        }
    }

    fn draw_license(&self, frame: &mut Frame) {
        let area = frame.area();

        let mut lines = vec![
            Line::from(Span::styled(
                format!("Fireman v{}", env!("CARGO_PKG_VERSION")),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(license::PROJECT_COPYRIGHT),
            Line::from(""),
            Line::from(Span::styled(
                format!("License: {}", license::PROJECT_LICENSE),
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(license::PROJECT_URL),
            Line::from(""),
            Line::from(Span::styled(
                "Third-party libraries:",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ];
        for dep in THIRD_PARTY_DEPS {
            let label = if dep.version.is_empty() {
                dep.name.to_string()
            } else {
                format!("{} {}", dep.name, dep.version)
            };
            lines.push(Line::from(format!("  {label:<25} — {}", dep.license)));
        }
        lines.extend([
            Line::from(""),
            Line::from("See THIRD_PARTY_LICENSES for full texts."),
            Line::from(""),
            Line::from(Span::styled(
                "Press any key to close",
                Style::default().fg(Color::DarkGray),
            )),
        ]);
        // +2 for top/bottom border
        let popup_height = ((lines.len() as u16) + 2).min(area.height.saturating_sub(4));
        let popup_width = 68.min(area.width.saturating_sub(4));
        let x = (area.width.saturating_sub(popup_width)) / 2;
        let y = (area.height.saturating_sub(popup_height)) / 2;
        let popup = Rect::new(x, y, popup_width, popup_height);

        frame.render_widget(Clear, popup);

        let text = Text::from(lines);

        frame.render_widget(
            Paragraph::new(text)
                .block(
                    Block::default()
                        .title(" About / License ")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Cyan)),
                )
                .wrap(Wrap { trim: false }),
            popup,
        );
    }

    fn draw_file_browser(&self, frame: &mut Frame, area: Rect, prompt: &PromptState) {
        let Some(browser) = &prompt.file_browser else {
            return;
        };

        let items: Vec<ListItem> = browser
            .entries
            .iter()
            .map(|entry| {
                let prefix = if entry.is_dir { "dir " } else { "    " };
                let style = if entry.matched {
                    Style::default().fg(Color::Green)
                } else if entry.is_dir {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{prefix}{}", entry.name)).style(style)
            })
            .collect();

        let mut state = ListState::default().with_selected(if browser.entries.is_empty() {
            None
        } else {
            Some(browser.selected_index)
        });

        let list = List::new(items)
            .block(Block::default().borders(Borders::BOTTOM).title("Files"))
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn draw_prompt_input(&self, frame: &mut Frame, area: Rect, prompt: &PromptState) {
        let (head, tail) = prompt.text.split_at(prompt.cursor);
        let display = format!("{head}|{tail}");
        let widget = Paragraph::new(display)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Path")
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(widget, area);
    }

    pub(crate) fn block(&self, title: &str) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
    }

    pub(crate) fn focus_block(&self, title: &str, focused: bool) -> Block<'static> {
        let style = if focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };
        Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
            .border_style(style)
    }

    pub(crate) fn selection(&self, selected: Option<usize>, len: usize) -> Option<usize> {
        if len == 0 {
            None
        } else {
            Some(selected.unwrap_or(0).min(len - 1))
        }
    }
}

/// Map a flat optimization field cursor index to a visual list index
/// that accounts for group header rows.
fn cursor_to_visual_index(cursor: usize) -> usize {
    let mut visual = 0;
    let mut flat = 0;
    for group in OPTIMIZATION_GROUPS {
        visual += 1; // group header
        for _field in group.fields {
            if flat == cursor {
                return visual;
            }
            visual += 1;
            flat += 1;
        }
    }
    visual.saturating_sub(1)
}

fn code_list_item(
    gutter: impl Into<String>,
    text: impl Into<String>,
    gutter_style: Style,
    text_style: Style,
) -> ListItem<'static> {
    let gutter = gutter.into();
    let text = text.into();
    ListItem::new(Line::from(vec![
        Span::styled(gutter, gutter_style),
        Span::styled(" │ ", gutter_style),
        Span::styled(text, text_style),
    ]))
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let [outer] = Layout::horizontal([Constraint::Percentage(100)]).areas(area);
    let [_, center_x, _] = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .areas(outer);
    let [_, center_y, _] = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .areas(center_x);
    center_y
}

fn inner_block_area(area: Rect) -> Rect {
    Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    }
}

fn highlight_ast_line(line: &str) -> Line<'static> {
    let mut spans = Vec::new();
    let keywords = [
        "if", "else", "while", "for", "return", "switch", "case", "default", "goto", "break",
        "continue", "void", "int", "char", "float", "double", "bool", "struct", "union", "int8_t",
        "int16_t", "int32_t", "int64_t", "uint8_t", "uint16_t", "uint32_t", "uint64_t",
    ];

    let mut current_word = String::new();
    let mut iter = line.chars().peekable();

    while let Some(c) = iter.next() {
        if c.is_alphanumeric() || c == '_' {
            current_word.push(c);
        } else {
            if !current_word.is_empty() {
                if keywords.contains(&current_word.as_str()) {
                    spans.push(Span::styled(
                        current_word.clone(),
                        Style::default().fg(Color::Cyan),
                    ));
                } else {
                    spans.push(Span::raw(current_word.clone()));
                }
                current_word.clear();
            }
            if c == '/' && iter.peek() == Some(&'/') {
                let rest: String = std::iter::once(c).chain(iter).collect();
                spans.push(Span::styled(rest, Style::default().fg(Color::DarkGray)));
                break;
            } else if c == '/' && iter.peek() == Some(&'*') {
                let rest: String = std::iter::once(c).chain(iter).collect();
                spans.push(Span::styled(rest, Style::default().fg(Color::DarkGray)));
                break;
            } else {
                spans.push(Span::raw(c.to_string()));
            }
        }
    }
    if !current_word.is_empty() {
        if keywords.contains(&current_word.as_str()) {
            spans.push(Span::styled(current_word, Style::default().fg(Color::Cyan)));
        } else {
            spans.push(Span::raw(current_word));
        }
    }

    Line::from(spans)
}

use super::{
    app::App,
    types::{OPTIMIZATION_GROUPS, OptimizationFocus, PromptState, View},
};
use crate::model::{EditorDraft, EditorLayer};
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

        if let Some(prompt) = &self.prompt {
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

        let selected_count = self
            .known_sections
            .iter()
            .filter(|section| section.selected)
            .count();
        let total_count = self.known_sections.len();
        let title = format!("Sections ({selected_count}/{total_count} selected)");

        let items: Vec<ListItem> = if self.known_sections.is_empty() {
            vec![ListItem::new("No analyzed sections yet. Press `a` or `g`.")]
        } else {
            self.known_sections
                .iter()
                .map(|section| {
                    let status = if section.data.analyzed {
                        "analyzed"
                    } else {
                        "pending"
                    };
                    let marker = if section.selected { "[x]" } else { "[ ]" };
                    let end = section
                        .data
                        .end_address
                        .map(|value| format!("{value:#010x}"))
                        .unwrap_or_else(|| "??????????".to_string());
                    ListItem::new(format!(
                        "{marker} {:#010x} - {end}  {status}",
                        section.data.start_address
                    ))
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
                vec![
                    Line::from(format!("Start: {:#010x}", section.data.start_address)),
                    Line::from(format!(
                        "End: {}",
                        section
                            .data
                            .end_address
                            .map(|value| format!("{value:#010x}"))
                            .unwrap_or_else(|| "Unknown".to_string())
                    )),
                    Line::from(format!("Analyzed: {}", section.data.analyzed)),
                    Line::from(format!("Selected: {}", section.selected)),
                    Line::from(""),
                    Line::from(format!("Selected sections: {selected_count}")),
                    Line::from("Space: toggle | s/Ctrl+A: select all | d: decompile"),
                ]
            })
            .unwrap_or_else(|| {
                vec![
                    Line::from("No section is selected."),
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
        let title = match layer {
            EditorLayer::Assembly => "Assembly",
            EditorLayer::Ir => "IR",
            EditorLayer::Ast => "AST",
        };
        let Some(outputs) = &self.outputs else {
            frame.render_widget(
                Paragraph::new("No decompile result yet. Select sections and press `d`.")
                    .block(self.block(title)),
                area,
            );
            return;
        };

        let hovered_asm = self.hovered_assembly_index;

        let (lines, selected, footer) = match layer {
            EditorLayer::Assembly => (
                outputs
                    .assembly
                    .iter()
                    .map(|row| {
                        let highlight = hovered_asm.is_some() && hovered_asm == Some(row.index);
                        let style = if highlight {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default()
                        };
                        ListItem::new(format!("{:#010x} {}", row.parents_start_address, row.data))
                            .style(style)
                    })
                    .collect::<Vec<_>>(),
                self.selection(Some(self.assembly_cursor), outputs.assembly.len()),
                outputs
                    .assembly
                    .get(self.assembly_cursor)
                    .map(|row| {
                        format!(
                            "Row {} parent {:#010x}",
                            row.index, row.parents_start_address
                        )
                    })
                    .unwrap_or_else(|| "No assembly rows".to_string()),
            ),
            EditorLayer::Ir => (
                outputs
                    .ir
                    .iter()
                    .map(|row| {
                        let highlight = hovered_asm.is_some()
                            && hovered_asm == Some(row.parents_assembly_index);
                        let style = if highlight {
                            Style::default().fg(Color::Yellow)
                        } else {
                            Style::default()
                        };
                        ListItem::new(row.data.clone()).style(style)
                    })
                    .collect::<Vec<_>>(),
                self.selection(Some(self.ir_cursor), outputs.ir.len()),
                outputs
                    .ir
                    .get(self.ir_cursor)
                    .map(|row| format!("Parent assembly row {}", row.parents_assembly_index))
                    .unwrap_or_else(|| "No IR rows".to_string()),
            ),
            EditorLayer::Ast => (
                outputs
                    .ast
                    .iter()
                    .map(|row| ListItem::new(highlight_ast_line(&row.data)))
                    .collect::<Vec<_>>(),
                self.selection(Some(self.ast_cursor), outputs.ast.len()),
                outputs
                    .ast
                    .get(self.ast_cursor)
                    .map(|row| format!("AST row {}", row.row))
                    .unwrap_or_else(|| "No AST rows".to_string()),
            ),
        };

        let [list_area, info_area] =
            Layout::horizontal([Constraint::Percentage(76), Constraint::Percentage(24)])
                .areas(area);
        let mut state = ListState::default().with_selected(selected);
        let list = List::new(lines)
            .block(self.block(title))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, list_area, &mut state);

        let info = vec![
            Line::from(footer),
            Line::from(""),
            Line::from("Enter: load editor"),
            Line::from("e: edit line"),
            Line::from("5: editor view"),
        ];
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
        "continue", "void", "int", "char", "float", "double", "bool", "struct", "union",
        "int8_t", "int16_t", "int32_t", "int64_t", "uint8_t", "uint16_t", "uint32_t", "uint64_t",
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
            spans.push(Span::styled(
                current_word,
                Style::default().fg(Color::Cyan),
            ));
        } else {
            spans.push(Span::raw(current_word));
        }
    }

    Line::from(spans)
}

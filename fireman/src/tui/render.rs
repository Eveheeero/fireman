use super::{
    app::App,
    types::{OptimizationFocus, PromptState, TabType, OPTIMIZATION_FIELDS},
};
use crate::license::{self, THIRD_PARTY_DEPS};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Frame,
};

impl App {
    pub(crate) fn draw(&self, frame: &mut Frame) {
        let [title_area, tabs_area, main_area, status_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(3),
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
        let labels = self.tabs.labels();
        let tabs = Tabs::new(labels)
            .select(self.tabs.current_index)
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
        match self.tabs.current_tab_type() {
            Some(TabType::Input) => self.draw_sections(frame, area),
            Some(TabType::Logs) => self.draw_logs(frame, area),
            Some(TabType::Result) => self.draw_result(frame, area),
            Some(TabType::Pick) => self.draw_pick(frame, area),
            None => {}
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

    fn draw_result(&self, frame: &mut Frame, area: Rect) {
        let Some(result_state) = self.current_result_state() else {
            frame.render_widget(
                Paragraph::new("No result state for this tab.").block(self.block("Result")),
                area,
            );
            return;
        };
        let Some(outputs) = &result_state.outputs else {
            frame.render_widget(
                Paragraph::new("No decompile result yet. Select sections in the Input tab first.")
                    .block(self.block("Result")),
                area,
            );
            return;
        };

        let cursor = result_state.cursor;
        let title = format!("Result ({})", outputs.ast.len());
        let lines: Vec<ListItem> = outputs
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
            .collect();

        let [list_area, info_area] =
            Layout::horizontal([Constraint::Percentage(78), Constraint::Percentage(22)])
                .areas(area);
        let mut state =
            ListState::default().with_selected(self.selection(Some(cursor), outputs.ast.len()));
        let list = List::new(lines)
            .block(self.block(&title))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, list_area, &mut state);

        let mut info = outputs
            .ast
            .get(cursor)
            .map(|row| vec![Line::from(format!("AST line: {}", row.row + 1))])
            .unwrap_or_else(|| vec![Line::from("No AST rows")]);
        info.push(Line::from("Gutter: printed AST line number"));
        if let Some(message) = &outputs.ast_sync_message {
            info.push(Line::from(""));
            info.push(Line::from(Span::styled(
                format!("Sync: {message}"),
                Style::default().fg(Color::Yellow),
            )));
        }
        info.push(Line::from(""));
        info.push(Line::from("Home/End/Pg: navigate"));
        info.push(Line::from("d: re-decompile"));
        frame.render_widget(
            Paragraph::new(info)
                .block(self.block("Selection"))
                .wrap(Wrap { trim: false }),
            info_area,
        );
    }

    fn draw_pick(&self, frame: &mut Frame, area: Rect) {
        let Some(pick) = self.current_pick_state() else {
            frame.render_widget(
                Paragraph::new("No pick state for this tab.").block(self.block("Pick")),
                area,
            );
            return;
        };

        let [settings_area, script_area] = Layout::horizontal([
            Constraint::Percentage(40),
            Constraint::Percentage(60),
        ])
        .areas(area);

        // --- Settings panel (left) ---
        let mut settings_items: Vec<ListItem> = Vec::new();
        for field in OPTIMIZATION_FIELDS {
            let enabled = (field.get)(&pick.store.draft_settings);
            let applied = (field.get)(&pick.store.applied_settings);
            let dirty_marker = if enabled != applied { " *" } else { "" };
            let radio = if enabled { "(o)" } else { "( )" };
            settings_items.push(ListItem::new(vec![
                Line::from(format!(" {radio} {}{dirty_marker}", field.label)),
                Line::from(Span::styled(
                    format!("     {}", field.description),
                    Style::default().fg(Color::DarkGray),
                )),
            ]));
        }
        // If the buffer has content, add an "Apply .fb script" checkbox entry
        let has_buffer = !pick.store.editor_buffer.trim().is_empty();
        if has_buffer {
            let check = if pick.store.fb_script_enabled {
                "[x]"
            } else {
                "[ ]"
            };
            settings_items.push(ListItem::new(vec![
                Line::from(format!(" {check} Apply .fb script")),
                Line::from(Span::styled(
                    "     Run the editor buffer as a .fb pattern script",
                    Style::default().fg(Color::DarkGray),
                )),
            ]));
        }

        let visual_index = pick.setting_cursor;
        let mut settings_state = ListState::default().with_selected(Some(visual_index));
        let settings_block = self.focus_block("Settings", pick.focus == OptimizationFocus::Settings);
        frame.render_stateful_widget(
            List::new(settings_items)
                .block(settings_block)
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                .highlight_symbol("> "),
            settings_area,
            &mut settings_state,
        );

        // --- Script panel (right) ---
        let buffer_path = pick
            .store
            .editor_path
            .as_deref()
            .unwrap_or("Unsaved buffer");
        let applied = if pick.store.applied_buffer_script.is_some() {
            "applied"
        } else {
            "not applied"
        };

        let buf = &pick.store.editor_buffer;
        let cursor_pos = pick.script_cursor.min(buf.len());
        let (before, after) = buf.split_at(cursor_pos);
        let display_text = if pick.focus == OptimizationFocus::Script {
            format!("{before}|{after}")
        } else if buf.is_empty() {
            "Buffer is empty.".to_string()
        } else {
            buf.clone()
        };

        let script_text = Text::from(vec![
            Line::from(format!("Path: {buffer_path}")),
            Line::from(format!("Status: {applied}")),
            Line::from(""),
            Line::from(display_text),
        ]);
        frame.render_widget(
            Paragraph::new(script_text)
                .block(
                    self.focus_block("Script", pick.focus == OptimizationFocus::Script),
                )
                .wrap(Wrap { trim: false }),
            script_area,
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
        let bindings = self.keybindings();

        let mut spans = vec![];
        for (i, (key, label)) in bindings.iter().enumerate() {
            if i > 0 {
                spans.push(Span::raw("  "));
            }
            spans.push(Span::styled(
                format!("{}", key),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                format!(":{}", label),
                Style::default().fg(Color::White),
            ));
        }

        let text = Text::from(vec![
            Line::from(Span::styled(
                self.top_message.clone(),
                Style::default().fg(Color::Yellow),
            )),
            Line::from(spans),
        ]);

        frame.render_widget(
            Paragraph::new(text)
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
        Span::styled(" | ", gutter_style),
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

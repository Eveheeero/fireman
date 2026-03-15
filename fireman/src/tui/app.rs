use super::StartupConfig;
use crate::{
    model::{
        AppliedEditResult, AssemblyEditorDraft, AstEditorDraft, DecompileRequest, DecompileResult,
        EditPosition, EditRequest, EditorDraft, EditorLayer, EditorTarget, IrEditorDraft,
        KnownSection, KnownSectionData, OptimizationScriptPreset, OptimizationSettings,
        OptimizationStore,
    },
    worker::{FirebatWorker, WorkerRequest, WorkerResponse, WorkerTryRecv},
};
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Tabs, Wrap},
};
use serde::Deserialize;
use std::{
    collections::VecDeque,
    fs, io,
    path::{Path, PathBuf},
    time::Duration,
};

const LOG_LIMIT: usize = 256;

pub struct App {
    worker: FirebatWorker,
    running: bool,
    current_view: View,
    prompt: Option<PromptState>,
    busy_label: Option<String>,
    top_message: String,
    logs: VecDeque<String>,
    opened_path: Option<String>,
    pending_open_path: Option<String>,
    pending_analysis_address: Option<String>,
    known_sections: Vec<KnownSection>,
    section_cursor: usize,
    outputs: Option<DecompileResult>,
    assembly_cursor: usize,
    ir_cursor: usize,
    ast_cursor: usize,
    editor_target: Option<EditorTarget>,
    editor_draft: Option<EditorDraft>,
    optimization: OptimizationStore,
    optimization_focus: OptimizationFocus,
    optimization_setting_cursor: usize,
    optimization_script_cursor: usize,
    patch_preview: Option<String>,
    patch_scroll: usize,
    log_scroll: usize,
    last_decompile_selection: Vec<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum View {
    Sections,
    Assembly,
    Ir,
    Ast,
    Editor,
    Optimization,
    Patch,
    Logs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OptimizationFocus {
    Settings,
    Scripts,
    Buffer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PromptKind {
    OpenFile,
    AnalyzeAddress,
    EditLine(EditorTarget),
    AddScriptPath,
    LoadBufferPath,
    SaveBufferPath,
    SavePatchPath,
    EditBuffer,
}

struct PromptState {
    kind: PromptKind,
    title: String,
    text: String,
    cursor: usize,
    multiline: bool,
    help: String,
}

#[derive(Deserialize)]
struct OptimizationStoreEnvelope {
    optimization_store: OptimizationStore,
}

#[derive(Clone, Copy)]
struct OptimizationField {
    label: &'static str,
    get: fn(&OptimizationSettings) -> bool,
    set: fn(&mut OptimizationSettings, bool),
}

const OPTIMIZATION_FIELDS: [OptimizationField; 18] = [
    OptimizationField {
        label: "IR analyzation",
        get: |settings| settings.ir_analyzation,
        set: |settings, value| settings.ir_analyzation = value,
    },
    OptimizationField {
        label: "Parameter analyzation",
        get: |settings| settings.parameter_analyzation,
        set: |settings, value| settings.parameter_analyzation = value,
    },
    OptimizationField {
        label: "Call argument analyzation",
        get: |settings| settings.call_argument_analyzation,
        set: |settings, value| settings.call_argument_analyzation = value,
    },
    OptimizationField {
        label: "Constant folding",
        get: |settings| settings.constant_folding,
        set: |settings, value| settings.constant_folding = value,
    },
    OptimizationField {
        label: "Control flow cleanup",
        get: |settings| settings.control_flow_cleanup,
        set: |settings, value| settings.control_flow_cleanup = value,
    },
    OptimizationField {
        label: "Collapse unused variable",
        get: |settings| settings.collapse_unused_varaible,
        set: |settings, value| settings.collapse_unused_varaible = value,
    },
    OptimizationField {
        label: "Dead store elimination",
        get: |settings| settings.dead_store_elimination,
        set: |settings, value| settings.dead_store_elimination = value,
    },
    OptimizationField {
        label: "Pattern matching",
        get: |settings| settings.pattern_matching_enabled,
        set: |settings, value| settings.pattern_matching_enabled = value,
    },
    OptimizationField {
        label: "Loop analyzation",
        get: |settings| settings.loop_analyzation,
        set: |settings, value| settings.loop_analyzation = value,
    },
    OptimizationField {
        label: "Copy propagation",
        get: |settings| settings.copy_propagation,
        set: |settings, value| settings.copy_propagation = value,
    },
    OptimizationField {
        label: "Expression inlining",
        get: |settings| settings.expression_inlining,
        set: |settings, value| settings.expression_inlining = value,
    },
    OptimizationField {
        label: "Ternary recovery",
        get: |settings| settings.ternary_recovery,
        set: |settings, value| settings.ternary_recovery = value,
    },
    OptimizationField {
        label: "Boolean recovery",
        get: |settings| settings.boolean_recovery,
        set: |settings, value| settings.boolean_recovery = value,
    },
    OptimizationField {
        label: "Switch reconstruction",
        get: |settings| settings.switch_reconstruction,
        set: |settings, value| settings.switch_reconstruction = value,
    },
    OptimizationField {
        label: "Lifetime scoping",
        get: |settings| settings.lifetime_scoping,
        set: |settings, value| settings.lifetime_scoping = value,
    },
    OptimizationField {
        label: "Signedness inference",
        get: |settings| settings.signedness_inference,
        set: |settings, value| settings.signedness_inference = value,
    },
    OptimizationField {
        label: "Name recovery",
        get: |settings| settings.name_recovery,
        set: |settings, value| settings.name_recovery = value,
    },
    OptimizationField {
        label: "Auto comment",
        get: |settings| settings.auto_comment,
        set: |settings, value| settings.auto_comment = value,
    },
];

impl Default for App {
    fn default() -> Self {
        Self::new(None)
    }
}

impl App {
    pub(super) fn new(startup: Option<StartupConfig>) -> Self {
        let optimization = startup
            .as_ref()
            .and_then(|config| config.optimization_store.clone())
            .unwrap_or_default();
        let mut app = Self {
            worker: FirebatWorker::spawn(),
            running: true,
            current_view: View::Sections,
            prompt: None,
            busy_label: None,
            top_message: "Open a binary to start".to_string(),
            logs: VecDeque::new(),
            opened_path: None,
            pending_open_path: None,
            pending_analysis_address: None,
            known_sections: Vec::new(),
            section_cursor: 0,
            outputs: None,
            assembly_cursor: 0,
            ir_cursor: 0,
            ast_cursor: 0,
            editor_target: None,
            editor_draft: None,
            optimization,
            optimization_focus: OptimizationFocus::Settings,
            optimization_setting_cursor: 0,
            optimization_script_cursor: 0,
            patch_preview: None,
            patch_scroll: 0,
            log_scroll: 0,
            last_decompile_selection: Vec::new(),
        };
        app.log("TUI initialized");
        if let Some(startup) = startup {
            app.apply_startup_config(startup);
        }
        app
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
        while self.running {
            self.poll_worker();
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(Duration::from_millis(50))? {
                let event = event::read()?;
                self.handle_event(event);
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
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

    fn apply_startup_config(&mut self, startup: StartupConfig) {
        if startup.optimization_store.is_some() {
            self.set_status("Loaded startup optimization config");
        }

        if let Some(path) = startup.input_path {
            self.pending_open_path = Some(path.clone());
            self.send_request(WorkerRequest::OpenFile(path), "opening binary");
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
            .block(self.block("Sections"))
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, list_area, &mut state);

        let selected_count = self
            .known_sections
            .iter()
            .filter(|section| section.selected)
            .count();
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
                    Line::from("Use Space to toggle and `d` to decompile."),
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

        let (lines, selected, footer) = match layer {
            EditorLayer::Assembly => (
                outputs
                    .assembly
                    .iter()
                    .map(|row| {
                        ListItem::new(format!("{:#010x} {}", row.parents_start_address, row.data))
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
                    .map(|row| ListItem::new(row.data.clone()))
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
                    .map(|row| ListItem::new(row.data.clone()))
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

        let settings_items: Vec<ListItem> = OPTIMIZATION_FIELDS
            .iter()
            .map(|field| {
                let enabled = (field.get)(&self.optimization.draft_settings);
                let applied = (field.get)(&self.optimization.applied_settings);
                ListItem::new(format!(
                    "[{}] {}{}",
                    if enabled { "x" } else { " " },
                    field.label,
                    if enabled != applied { " *" } else { "" }
                ))
            })
            .collect();
        let mut settings_state = ListState::default().with_selected(self.selection(
            Some(self.optimization_setting_cursor),
            OPTIMIZATION_FIELDS.len(),
        ));
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
        let body = if self.logs.is_empty() {
            "No logs yet.".to_string()
        } else {
            self.logs.iter().cloned().collect::<Vec<_>>().join("\n")
        };
        frame.render_widget(
            Paragraph::new(body)
                .block(self.block("Logs"))
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
        let area = centered_rect(
            if prompt.multiline { 76 } else { 64 },
            if prompt.multiline { 70 } else { 24 },
            frame.area(),
        );
        let [title_area, body_area, help_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(area);

        frame.render_widget(Clear, area);
        frame.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .title(prompt.title.as_str()),
            area,
        );
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
        frame.render_widget(Paragraph::new(""), title_area);
    }

    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return;
            }
            if self.prompt.is_some() {
                self.handle_prompt_key(key);
                return;
            }
            self.handle_key(key);
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if self.handle_global_key(key) {
            return;
        }

        match self.current_view {
            View::Sections => self.handle_sections_key(key),
            View::Assembly => self.handle_output_key(key, EditorLayer::Assembly),
            View::Ir => self.handle_output_key(key, EditorLayer::Ir),
            View::Ast => self.handle_output_key(key, EditorLayer::Ast),
            View::Editor => self.handle_editor_key(key),
            View::Optimization => self.handle_optimization_key(key),
            View::Patch => self.handle_patch_key(key),
            View::Logs => self.handle_logs_key(key),
        }
    }

    fn handle_global_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => {
                self.running = false;
                true
            }
            KeyCode::Char('1') => {
                self.current_view = View::Sections;
                true
            }
            KeyCode::Char('2') => {
                self.current_view = View::Assembly;
                true
            }
            KeyCode::Char('3') => {
                self.current_view = View::Ir;
                true
            }
            KeyCode::Char('4') => {
                self.current_view = View::Ast;
                true
            }
            KeyCode::Char('5') => {
                self.current_view = View::Editor;
                true
            }
            KeyCode::Char('6') => {
                self.current_view = View::Optimization;
                true
            }
            KeyCode::Char('7') => {
                self.current_view = View::Patch;
                true
            }
            KeyCode::Char('8') => {
                self.current_view = View::Logs;
                true
            }
            KeyCode::Char('o') => {
                self.open_prompt(
                    PromptKind::OpenFile,
                    "Open Binary",
                    self.opened_path.clone().unwrap_or_default(),
                    false,
                    "Enter file path and press Enter. Esc cancels.",
                );
                true
            }
            KeyCode::Char('a') => {
                self.open_prompt(
                    PromptKind::AnalyzeAddress,
                    "Analyze Address",
                    self.pending_analysis_address.clone().unwrap_or_default(),
                    false,
                    "Enter an address. Leave empty to analyze the entry point.",
                );
                true
            }
            KeyCode::Char('g') => {
                self.send_request(WorkerRequest::AnalyzeAllSections, "analyzing all sections");
                true
            }
            KeyCode::Char('d') => {
                self.start_decompile();
                true
            }
            KeyCode::Char('x') => {
                self.send_request(WorkerRequest::ExportPatch, "exporting patch");
                true
            }
            _ => false,
        }
    }

    fn handle_sections_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.section_cursor = self.section_cursor.saturating_sub(1),
            KeyCode::Down => {
                if self.section_cursor + 1 < self.known_sections.len() {
                    self.section_cursor += 1;
                }
            }
            KeyCode::Char(' ') => self.toggle_section(self.section_cursor),
            KeyCode::Char('s') => self.toggle_all_sections(),
            KeyCode::Enter => {
                self.toggle_section(self.section_cursor);
            }
            _ => {}
        }
    }

    fn handle_output_key(&mut self, key: KeyEvent, layer: EditorLayer) {
        let len = self.output_len(layer);
        if len == 0 {
            return;
        }
        let cursor = self.output_cursor_mut(layer);
        match key.code {
            KeyCode::Up => *cursor = cursor.saturating_sub(1),
            KeyCode::Down => {
                if *cursor + 1 < len {
                    *cursor += 1;
                }
            }
            KeyCode::Enter => self.load_editor_from_current_row(layer),
            KeyCode::Char('e') => self.edit_current_row(layer),
            _ => {}
        }
    }

    fn handle_editor_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('e') => self.edit_loaded_draft(),
            KeyCode::Char('r') => {
                if let Some(target) = self.editor_target {
                    self.reload_editor(target);
                }
            }
            KeyCode::Char('[') => self.cycle_edit_position(false),
            KeyCode::Char(']') => self.cycle_edit_position(true),
            KeyCode::Enter => self.apply_edit(),
            _ => {}
        }
    }

    fn handle_optimization_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => self.optimization_focus = self.optimization_focus.next(),
            KeyCode::BackTab => self.optimization_focus = self.optimization_focus.previous(),
            KeyCode::Char('L') => self.load_saved_optimization_store(),
            KeyCode::Char('W') => self.save_current_optimization_store(),
            _ => match self.optimization_focus {
                OptimizationFocus::Settings => self.handle_optimization_settings_key(key),
                OptimizationFocus::Scripts => self.handle_optimization_scripts_key(key),
                OptimizationFocus::Buffer => self.handle_optimization_buffer_key(key),
            },
        }
    }

    fn handle_optimization_settings_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                self.optimization_setting_cursor =
                    self.optimization_setting_cursor.saturating_sub(1)
            }
            KeyCode::Down => {
                if self.optimization_setting_cursor + 1 < OPTIMIZATION_FIELDS.len() {
                    self.optimization_setting_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                let field = OPTIMIZATION_FIELDS[self.optimization_setting_cursor];
                let next = !(field.get)(&self.optimization.draft_settings);
                (field.set)(&mut self.optimization.draft_settings, next);
            }
            KeyCode::Char('c') => self.apply_optimization_settings(),
            KeyCode::Char('r') => {
                self.optimization.draft_settings = OptimizationSettings::default();
                self.set_status("Restored optimization draft defaults");
            }
            _ => {}
        }
    }

    fn handle_optimization_scripts_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => {
                self.optimization_script_cursor = self.optimization_script_cursor.saturating_sub(1)
            }
            KeyCode::Down => {
                if self.optimization_script_cursor + 1 < self.optimization.script_presets.len() {
                    self.optimization_script_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                if let Some(preset) = self
                    .optimization
                    .script_presets
                    .get_mut(self.optimization_script_cursor)
                {
                    preset.enabled = !preset.enabled;
                }
            }
            KeyCode::Char('n') => self.open_prompt(
                PromptKind::AddScriptPath,
                "Add Script Path",
                String::new(),
                false,
                "Enter a .fb file path to register as a script preset.",
            ),
            KeyCode::Char('x') => self.remove_script_preset(self.optimization_script_cursor),
            KeyCode::Enter => self.load_script_preset_into_buffer(self.optimization_script_cursor),
            KeyCode::Char('c') => self.apply_optimization_settings(),
            _ => {}
        }
    }

    fn handle_optimization_buffer_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('e') => self.open_prompt(
                PromptKind::EditBuffer,
                "Edit Buffer Script",
                self.optimization.editor_buffer.clone(),
                true,
                "Ctrl+S saves the buffer. Esc cancels.",
            ),
            KeyCode::Char('u') => self.apply_buffer_script(),
            KeyCode::Char('c') => self.clear_applied_buffer_script(),
            KeyCode::Char('o') => self.open_prompt(
                PromptKind::LoadBufferPath,
                "Load Buffer From Path",
                self.optimization.editor_path.clone().unwrap_or_default(),
                false,
                "Enter a .fb file path to load into the buffer.",
            ),
            KeyCode::Char('s') => {
                if let Some(path) = self.optimization.editor_path.clone() {
                    self.save_buffer_to_path(path);
                } else {
                    self.open_prompt(
                        PromptKind::SaveBufferPath,
                        "Save Buffer",
                        String::new(),
                        false,
                        "Enter a path for the buffer script.",
                    );
                }
            }
            KeyCode::Char('S') => self.open_prompt(
                PromptKind::SaveBufferPath,
                "Save Buffer As",
                self.optimization.editor_path.clone().unwrap_or_default(),
                false,
                "Enter a path for the buffer script.",
            ),
            _ => {}
        }
    }

    fn handle_patch_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.patch_scroll = self.patch_scroll.saturating_sub(1),
            KeyCode::Down => self.patch_scroll += 1,
            KeyCode::PageUp => self.patch_scroll = self.patch_scroll.saturating_sub(10),
            KeyCode::PageDown => self.patch_scroll += 10,
            KeyCode::Char('s') => self.open_prompt(
                PromptKind::SavePatchPath,
                "Save Patch Preview",
                String::new(),
                false,
                "Enter a file path for the exported patch JSON.",
            ),
            _ => {}
        }
    }

    fn handle_logs_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up => self.log_scroll = self.log_scroll.saturating_sub(1),
            KeyCode::Down => self.log_scroll += 1,
            KeyCode::PageUp => self.log_scroll = self.log_scroll.saturating_sub(10),
            KeyCode::PageDown => self.log_scroll += 10,
            _ => {}
        }
    }

    fn handle_prompt_key(&mut self, key: KeyEvent) {
        let Some(prompt) = self.prompt.as_mut() else {
            return;
        };
        match key.code {
            KeyCode::Esc => {
                self.prompt = None;
                self.set_status("Prompt cancelled");
            }
            KeyCode::Enter if !prompt.multiline => self.submit_prompt(),
            KeyCode::Enter => insert_char(&mut prompt.text, &mut prompt.cursor, '\n'),
            KeyCode::Backspace => delete_prev_char(&mut prompt.text, &mut prompt.cursor),
            KeyCode::Delete => delete_next_char(&mut prompt.text, &mut prompt.cursor),
            KeyCode::Left => move_cursor_left(&prompt.text, &mut prompt.cursor),
            KeyCode::Right => move_cursor_right(&prompt.text, &mut prompt.cursor),
            KeyCode::Home => prompt.cursor = 0,
            KeyCode::End => prompt.cursor = prompt.text.len(),
            KeyCode::Char('s')
                if prompt.multiline && key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                self.submit_prompt()
            }
            KeyCode::Char(ch) => insert_char(&mut prompt.text, &mut prompt.cursor, ch),
            _ => {}
        }
    }

    fn submit_prompt(&mut self) {
        let Some(prompt) = self.prompt.take() else {
            return;
        };
        let value = prompt.text;
        match prompt.kind {
            PromptKind::OpenFile => {
                if value.trim().is_empty() {
                    self.set_status("Binary path is required");
                } else {
                    self.pending_open_path = Some(value.clone());
                    self.send_request(WorkerRequest::OpenFile(value), "opening binary");
                }
            }
            PromptKind::AnalyzeAddress => {
                self.pending_analysis_address = Some(value.clone());
                self.send_request(WorkerRequest::AnalyzeSection(value), "analyzing section");
            }
            PromptKind::EditLine(target) => {
                self.load_editor_with_text(target, value);
                self.current_view = View::Editor;
            }
            PromptKind::AddScriptPath => self.add_script_preset(value),
            PromptKind::LoadBufferPath => self.load_buffer_from_path(value),
            PromptKind::SaveBufferPath => self.save_buffer_to_path(value),
            PromptKind::SavePatchPath => self.save_patch_preview(value),
            PromptKind::EditBuffer => {
                self.optimization.editor_buffer = value;
                self.set_status("Updated optimization buffer");
            }
        }
    }

    fn send_request(&mut self, request: WorkerRequest, label: &str) {
        if self.busy_label.is_some() {
            self.set_status("Background worker is busy");
            return;
        }
        match self.worker.send(request) {
            Ok(()) => {
                self.busy_label = Some(label.to_string());
                self.set_status(format!("Started {label}"));
            }
            Err(error) => self.set_status(error),
        }
    }

    fn poll_worker(&mut self) {
        loop {
            match self.worker.try_recv() {
                WorkerTryRecv::Message(message) => {
                    self.busy_label = None;
                    self.handle_worker_response(message);
                }
                WorkerTryRecv::Empty => break,
                WorkerTryRecv::Disconnected => {
                    self.busy_label = None;
                    self.set_status("Background worker disconnected");
                    break;
                }
            }
        }
    }

    fn handle_worker_response(&mut self, message: WorkerResponse) {
        match message {
            WorkerResponse::OpenFile(result) => match result {
                Ok(()) => {
                    self.opened_path = self.pending_open_path.take();
                    self.known_sections.clear();
                    self.outputs = None;
                    self.patch_preview = None;
                    self.editor_target = None;
                    self.editor_draft = None;
                    self.current_view = View::Sections;
                    self.set_status("Opened binary");
                }
                Err(error) => self.set_status(error),
            },
            WorkerResponse::AnalyzeSection(result) => match result {
                Ok(data) => {
                    let auto_select = self
                        .pending_analysis_address
                        .as_deref()
                        .map(|value| !value.trim().is_empty())
                        .unwrap_or(false);
                    self.pending_analysis_address = None;
                    self.merge_sections(data, auto_select);
                    self.current_view = View::Sections;
                    self.set_status("Section analysis completed");
                }
                Err(error) => self.set_status(error),
            },
            WorkerResponse::AnalyzeAllSections(result) => match result {
                Ok(data) => {
                    self.merge_sections(data, false);
                    self.current_view = View::Sections;
                    self.set_status("Analyzed all sections");
                }
                Err(error) => self.set_status(error),
            },
            WorkerResponse::DecompileSections(result) => match result {
                Ok(result) => {
                    self.outputs = Some(result);
                    self.patch_preview = None;
                    self.current_view = View::Assembly;
                    self.assembly_cursor = 0;
                    self.ir_cursor = 0;
                    self.ast_cursor = 0;
                    self.reload_editor_from_current_target();
                    self.set_status("Decompile completed");
                }
                Err(error) => self.set_status(error),
            },
            WorkerResponse::ApplyEdit(result) => match result {
                Ok(result) => self.apply_edit_result(result),
                Err(error) => self.set_status(error),
            },
            WorkerResponse::ExportPatch(result) => match result {
                Ok(json) => {
                    self.patch_preview = Some(json);
                    self.patch_scroll = 0;
                    self.current_view = View::Patch;
                    self.set_status("Patch exported");
                }
                Err(error) => self.set_status(error),
            },
        }
    }

    fn start_decompile(&mut self) {
        let start_addresses = self.selected_addresses();
        if start_addresses.is_empty() {
            self.set_status("Select at least one analyzed section before decompiling");
            return;
        }

        self.last_decompile_selection = start_addresses.clone();
        self.send_request(
            WorkerRequest::DecompileSections(DecompileRequest {
                start_addresses,
                settings: self.optimization.applied_settings.clone(),
                script_paths: self
                    .optimization
                    .script_presets
                    .iter()
                    .filter(|preset| preset.applied_enabled)
                    .map(|preset| preset.path.clone())
                    .collect(),
                buffer_script: self.optimization.applied_buffer_script.clone(),
            }),
            "decompiling sections",
        );
    }

    fn apply_edit(&mut self) {
        let Some(target) = self.editor_target else {
            self.set_status("No editor target selected");
            return;
        };
        let Some(request) = self.build_edit_request(target) else {
            self.set_status("No editor draft is loaded");
            return;
        };
        self.send_request(WorkerRequest::ApplyEdit(request), "applying edit");
    }

    fn build_edit_request(&self, target: EditorTarget) -> Option<EditRequest> {
        match self.editor_draft.as_ref()? {
            EditorDraft::Assembly(draft) => Some(EditRequest {
                layer: EditorLayer::Assembly,
                row: target.row,
                position: EditPosition::Replace,
                text: draft.compose_line(),
            }),
            EditorDraft::Ir(draft) => Some(EditRequest {
                layer: EditorLayer::Ir,
                row: target.row,
                position: draft.position,
                text: draft.compose_line(),
            }),
            EditorDraft::Ast(draft) => Some(EditRequest {
                layer: EditorLayer::Ast,
                row: target.row,
                position: draft.position,
                text: draft.raw_text.clone(),
            }),
        }
    }

    fn apply_edit_result(&mut self, result: AppliedEditResult) {
        self.outputs = Some(result.result);
        self.editor_target = Some(result.selected_target);
        self.sync_output_cursor(result.selected_target);
        self.reload_editor(result.selected_target);
        self.current_view = View::Editor;
        self.set_status("Edit applied");
    }

    fn merge_sections(&mut self, sections: Vec<KnownSectionData>, select_new: bool) {
        for data in sections {
            if let Some(existing) = self
                .known_sections
                .iter_mut()
                .find(|section| section.data.start_address == data.start_address)
            {
                existing.data = data.clone();
                if select_new {
                    existing.selected = true;
                }
            } else {
                self.known_sections.push(KnownSection {
                    selected: select_new,
                    data,
                });
            }
        }
        self.known_sections
            .sort_by_key(|section| section.data.start_address);
        if self.section_cursor >= self.known_sections.len() {
            self.section_cursor = self.known_sections.len().saturating_sub(1);
        }
    }

    fn selected_addresses(&self) -> Vec<u64> {
        self.known_sections
            .iter()
            .filter(|section| section.selected && section.data.analyzed)
            .map(|section| section.data.start_address)
            .collect()
    }

    fn toggle_section(&mut self, index: usize) {
        if let Some(section) = self.known_sections.get_mut(index) {
            section.selected = !section.selected;
        }
    }

    fn toggle_all_sections(&mut self) {
        let all_selected = self
            .known_sections
            .iter()
            .filter(|section| section.data.analyzed)
            .all(|section| section.selected);
        for section in &mut self.known_sections {
            if section.data.analyzed {
                section.selected = !all_selected;
            }
        }
    }

    fn output_len(&self, layer: EditorLayer) -> usize {
        match (layer, self.outputs.as_ref()) {
            (_, None) => 0,
            (EditorLayer::Assembly, Some(outputs)) => outputs.assembly.len(),
            (EditorLayer::Ir, Some(outputs)) => outputs.ir.len(),
            (EditorLayer::Ast, Some(outputs)) => outputs.ast.len(),
        }
    }

    fn output_cursor_mut(&mut self, layer: EditorLayer) -> &mut usize {
        match layer {
            EditorLayer::Assembly => &mut self.assembly_cursor,
            EditorLayer::Ir => &mut self.ir_cursor,
            EditorLayer::Ast => &mut self.ast_cursor,
        }
    }

    fn current_target(&self, layer: EditorLayer) -> Option<EditorTarget> {
        let row = match layer {
            EditorLayer::Assembly => {
                self.selection(Some(self.assembly_cursor), self.output_len(layer))?
            }
            EditorLayer::Ir => self.selection(Some(self.ir_cursor), self.output_len(layer))?,
            EditorLayer::Ast => self.selection(Some(self.ast_cursor), self.output_len(layer))?,
        };
        Some(EditorTarget { layer, row })
    }

    fn load_editor_from_current_row(&mut self, layer: EditorLayer) {
        let Some(target) = self.current_target(layer) else {
            self.set_status("No row is selected");
            return;
        };
        self.reload_editor(target);
        self.current_view = View::Editor;
    }

    fn edit_current_row(&mut self, layer: EditorLayer) {
        let Some(target) = self.current_target(layer) else {
            self.set_status("No row is selected");
            return;
        };
        let Some(text) = self.row_text(target) else {
            self.set_status("Selected row is unavailable");
            return;
        };
        self.open_prompt(
            PromptKind::EditLine(target),
            "Edit Row",
            text,
            false,
            "Edit the line and press Enter. The change is staged in the editor until you press Enter there.",
        );
    }

    fn edit_loaded_draft(&mut self) {
        let Some(target) = self.editor_target else {
            self.set_status("No editor target selected");
            return;
        };
        let text = match &self.editor_draft {
            Some(EditorDraft::Assembly(draft)) => draft.compose_line(),
            Some(EditorDraft::Ir(draft)) => draft.compose_line(),
            Some(EditorDraft::Ast(draft)) => draft.raw_text.clone(),
            None => {
                self.set_status("No draft loaded");
                return;
            }
        };
        self.open_prompt(
            PromptKind::EditLine(target),
            "Edit Draft",
            text,
            false,
            "Edit the staged line and press Enter.",
        );
    }

    fn reload_editor_from_current_target(&mut self) {
        if let Some(target) = self.editor_target {
            self.reload_editor(target);
        }
    }

    fn reload_editor(&mut self, target: EditorTarget) {
        self.editor_target = Some(target);
        self.editor_draft = self.row_text(target).map(|text| {
            draft_from_text(
                target.layer,
                &text,
                self.current_edit_position(target.layer),
            )
        });
    }

    fn load_editor_with_text(&mut self, target: EditorTarget, text: String) {
        let position = self.current_edit_position(target.layer);
        self.editor_target = Some(target);
        self.editor_draft = Some(draft_from_text(target.layer, &text, position));
        self.sync_output_cursor(target);
    }

    fn current_edit_position(&self, layer: EditorLayer) -> EditPosition {
        match &self.editor_draft {
            Some(EditorDraft::Ir(draft)) if layer == EditorLayer::Ir => draft.position,
            Some(EditorDraft::Ast(draft)) if layer == EditorLayer::Ast => draft.position,
            _ => EditPosition::Replace,
        }
    }

    fn cycle_edit_position(&mut self, forward: bool) {
        match self.editor_draft.as_mut() {
            Some(EditorDraft::Ir(draft)) => draft.position = next_position(draft.position, forward),
            Some(EditorDraft::Ast(draft)) => {
                draft.position = next_position(draft.position, forward)
            }
            Some(EditorDraft::Assembly(_)) => {
                self.set_status("Assembly edits always replace the current row")
            }
            None => self.set_status("No editor draft loaded"),
        }
    }

    fn row_text(&self, target: EditorTarget) -> Option<String> {
        let outputs = self.outputs.as_ref()?;
        match target.layer {
            EditorLayer::Assembly => outputs
                .assembly
                .get(target.row)
                .map(|row| format!("{:#010x} {}", row.parents_start_address, row.data)),
            EditorLayer::Ir => outputs.ir.get(target.row).map(|row| row.data.clone()),
            EditorLayer::Ast => outputs.ast.get(target.row).map(|row| row.data.clone()),
        }
    }

    fn sync_output_cursor(&mut self, target: EditorTarget) {
        match target.layer {
            EditorLayer::Assembly => self.assembly_cursor = target.row,
            EditorLayer::Ir => self.ir_cursor = target.row,
            EditorLayer::Ast => self.ast_cursor = target.row,
        }
    }

    fn apply_optimization_settings(&mut self) {
        self.optimization.applied_settings = self.optimization.draft_settings.clone();
        for preset in &mut self.optimization.script_presets {
            preset.applied_enabled = preset.enabled;
        }
        self.set_status("Applied optimization settings and script presets");
        self.redecompile_last_selection();
    }

    fn apply_buffer_script(&mut self) {
        self.optimization.applied_buffer_script =
            if self.optimization.editor_buffer.trim().is_empty() {
                None
            } else {
                Some(self.optimization.editor_buffer.clone())
            };
        self.set_status("Applied optimization buffer script");
        self.redecompile_last_selection();
    }

    fn clear_applied_buffer_script(&mut self) {
        self.optimization.applied_buffer_script = None;
        self.set_status("Cleared applied optimization buffer script");
        self.redecompile_last_selection();
    }

    fn add_script_preset(&mut self, path: String) {
        let path = path.trim();
        if path.is_empty() {
            self.set_status("Script path is required");
            return;
        }
        let path_buf = PathBuf::from(path);
        if !path_buf.exists() {
            self.set_status("Script path does not exist");
            return;
        }
        self.upsert_script_preset(&path_buf);
        self.set_status(format!("Registered script {}", path_buf.display()));
    }

    fn remove_script_preset(&mut self, index: usize) {
        if index >= self.optimization.script_presets.len() {
            return;
        }
        let removed = self.optimization.script_presets.remove(index);
        if self.optimization_script_cursor >= self.optimization.script_presets.len() {
            self.optimization_script_cursor =
                self.optimization.script_presets.len().saturating_sub(1);
        }
        self.set_status(format!("Removed script {}", removed.path));
    }

    fn load_script_preset_into_buffer(&mut self, index: usize) {
        let Some(preset) = self.optimization.script_presets.get(index) else {
            self.set_status("No script preset selected");
            return;
        };
        self.load_buffer_from_path(preset.path.clone());
    }

    fn load_buffer_from_path(&mut self, path: String) {
        let path = path.trim();
        if path.is_empty() {
            self.set_status("Buffer path is required");
            return;
        }
        match fs::read_to_string(path) {
            Ok(buffer) => {
                self.optimization.editor_buffer = buffer;
                self.optimization.editor_path = Some(path.to_string());
                self.upsert_script_preset(Path::new(path));
                self.set_status(format!("Loaded buffer from {path}"));
            }
            Err(error) => self.set_status(error.to_string()),
        }
    }

    fn save_buffer_to_path(&mut self, path: String) {
        let path = path.trim();
        if path.is_empty() {
            self.set_status("Buffer save path is required");
            return;
        }
        match fs::write(path, &self.optimization.editor_buffer) {
            Ok(()) => {
                self.optimization.editor_path = Some(path.to_string());
                self.upsert_script_preset(Path::new(path));
                self.set_status(format!("Saved buffer to {path}"));
            }
            Err(error) => self.set_status(error.to_string()),
        }
    }

    fn save_patch_preview(&mut self, path: String) {
        let Some(preview) = &self.patch_preview else {
            self.set_status("No patch preview to save");
            return;
        };
        let path = path.trim();
        if path.is_empty() {
            self.set_status("Patch save path is required");
            return;
        }
        match fs::write(path, preview) {
            Ok(()) => self.set_status(format!("Saved patch preview to {path}")),
            Err(error) => self.set_status(error.to_string()),
        }
    }

    fn redecompile_last_selection(&mut self) {
        if self.last_decompile_selection.is_empty() || self.busy_label.is_some() {
            return;
        }
        self.send_request(
            WorkerRequest::DecompileSections(DecompileRequest {
                start_addresses: self.last_decompile_selection.clone(),
                settings: self.optimization.applied_settings.clone(),
                script_paths: self
                    .optimization
                    .script_presets
                    .iter()
                    .filter(|preset| preset.applied_enabled)
                    .map(|preset| preset.path.clone())
                    .collect(),
                buffer_script: self.optimization.applied_buffer_script.clone(),
            }),
            "re-decompiling sections",
        );
    }

    fn upsert_script_preset(&mut self, path: &Path) {
        let path_string = path.to_string_lossy().to_string();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or(path_string.as_str())
            .to_string();
        if let Some(existing) = self
            .optimization
            .script_presets
            .iter_mut()
            .find(|preset| preset.path == path_string)
        {
            existing.name = name;
            return;
        }
        self.optimization
            .script_presets
            .push(OptimizationScriptPreset {
                name,
                path: path_string,
                enabled: false,
                applied_enabled: false,
            });
    }

    fn load_saved_optimization_store(&mut self) {
        match load_optimization_store() {
            Ok(store) => {
                self.optimization = store;
                self.optimization_setting_cursor = self
                    .optimization_setting_cursor
                    .min(OPTIMIZATION_FIELDS.len().saturating_sub(1));
                self.optimization_script_cursor = self
                    .optimization_script_cursor
                    .min(self.optimization.script_presets.len().saturating_sub(1));
                self.set_status("Loaded optimization settings from disk");
                self.redecompile_last_selection();
            }
            Err(error) => self.set_status(format!("Optimization load failed: {error}")),
        }
    }

    fn save_current_optimization_store(&mut self) {
        match save_optimization_store(&self.optimization) {
            Ok(()) => self.set_status("Saved optimization settings to disk"),
            Err(error) => self.set_status(format!("Optimization save failed: {error}")),
        }
    }

    fn open_prompt(
        &mut self,
        kind: PromptKind,
        title: impl Into<String>,
        text: String,
        multiline: bool,
        help: impl Into<String>,
    ) {
        let cursor = text.len();
        self.prompt = Some(PromptState {
            kind,
            title: title.into(),
            text,
            cursor,
            multiline,
            help: help.into(),
        });
    }

    fn block(&self, title: &str) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
    }

    fn focus_block(&self, title: &str, focused: bool) -> Block<'static> {
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

    fn selection(&self, selected: Option<usize>, len: usize) -> Option<usize> {
        if len == 0 {
            None
        } else {
            Some(selected.unwrap_or(0).min(len - 1))
        }
    }

    fn set_status(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.top_message = message.clone();
        self.log(message);
    }

    fn log(&mut self, message: impl Into<String>) {
        let message = message.into();
        if self.logs.len() == LOG_LIMIT {
            self.logs.pop_front();
        }
        self.logs.push_back(message);
    }

    fn keybindings(&self) -> Vec<(&'static str, &'static str)> {
        if let Some(prompt) = &self.prompt {
            return if prompt.multiline {
                vec![
                    ("Esc", "cancel"),
                    ("Ctrl+S", "save buffer"),
                    ("Enter", "newline"),
                ]
            } else {
                vec![
                    ("Esc", "cancel"),
                    ("Enter", "submit"),
                    ("Left/Right", "move cursor"),
                ]
            };
        }

        let mut keys = vec![
            ("o", "open"),
            ("a", "analyze"),
            ("g", "analyze-all"),
            ("d", "decompile"),
            ("x", "export"),
            ("1-8", "views"),
            ("q", "quit"),
        ];
        match self.current_view {
            View::Sections => keys.extend([
                ("Up/Down", "move"),
                ("Space", "toggle"),
                ("s", "toggle-all"),
            ]),
            View::Assembly | View::Ir | View::Ast => {
                keys.extend([("Up/Down", "move"), ("Enter", "load-editor"), ("e", "edit")])
            }
            View::Editor => keys.extend([
                ("e", "edit"),
                ("[ ]", "position"),
                ("Enter", "apply"),
                ("r", "reload"),
            ]),
            View::Optimization => match self.optimization_focus {
                OptimizationFocus::Settings => keys.extend([
                    ("Tab", "focus"),
                    ("Space", "toggle"),
                    ("c", "apply"),
                    ("r", "defaults"),
                    ("L/W", "load-save"),
                ]),
                OptimizationFocus::Scripts => keys.extend([
                    ("Tab", "focus"),
                    ("Space", "toggle"),
                    ("n", "add"),
                    ("x", "remove"),
                    ("Enter", "load"),
                    ("L/W", "load-save"),
                ]),
                OptimizationFocus::Buffer => keys.extend([
                    ("Tab", "focus"),
                    ("e", "edit"),
                    ("u", "apply"),
                    ("o", "load"),
                    ("s/S", "save"),
                    ("L/W", "load-save"),
                ]),
            },
            View::Patch => keys.extend([("Up/Down", "scroll"), ("s", "save")]),
            View::Logs => keys.extend([("Up/Down", "scroll")]),
        }
        keys
    }
}

impl View {
    const fn index(self) -> usize {
        match self {
            Self::Sections => 0,
            Self::Assembly => 1,
            Self::Ir => 2,
            Self::Ast => 3,
            Self::Editor => 4,
            Self::Optimization => 5,
            Self::Patch => 6,
            Self::Logs => 7,
        }
    }
}

impl OptimizationFocus {
    const fn next(self) -> Self {
        match self {
            Self::Settings => Self::Scripts,
            Self::Scripts => Self::Buffer,
            Self::Buffer => Self::Settings,
        }
    }

    const fn previous(self) -> Self {
        match self {
            Self::Settings => Self::Buffer,
            Self::Scripts => Self::Settings,
            Self::Buffer => Self::Scripts,
        }
    }
}

fn draft_from_text(layer: EditorLayer, text: &str, position: EditPosition) -> EditorDraft {
    match layer {
        EditorLayer::Assembly => {
            EditorDraft::Assembly(AssemblyEditorDraft::from_display_text(text))
        }
        EditorLayer::Ir => {
            let mut draft = IrEditorDraft::from_text(text);
            draft.position = position;
            EditorDraft::Ir(draft)
        }
        EditorLayer::Ast => {
            let mut draft = AstEditorDraft::from_text(text);
            draft.position = position;
            EditorDraft::Ast(draft)
        }
    }
}

fn next_position(position: EditPosition, forward: bool) -> EditPosition {
    match (position, forward) {
        (EditPosition::Replace, true) => EditPosition::Before,
        (EditPosition::Before, true) => EditPosition::After,
        (EditPosition::After, true) => EditPosition::Replace,
        (EditPosition::Replace, false) => EditPosition::After,
        (EditPosition::Before, false) => EditPosition::Replace,
        (EditPosition::After, false) => EditPosition::Before,
    }
}

fn insert_char(buffer: &mut String, cursor: &mut usize, ch: char) {
    buffer.insert(*cursor, ch);
    *cursor += ch.len_utf8();
}

fn delete_prev_char(buffer: &mut String, cursor: &mut usize) {
    if *cursor == 0 {
        return;
    }
    if let Some((index, ch)) = buffer[..*cursor].char_indices().last() {
        buffer.drain(index..index + ch.len_utf8());
        *cursor = index;
    }
}

fn delete_next_char(buffer: &mut String, cursor: &mut usize) {
    if *cursor >= buffer.len() {
        return;
    }
    if let Some(ch) = buffer[*cursor..].chars().next() {
        buffer.drain(*cursor..*cursor + ch.len_utf8());
    }
}

fn move_cursor_left(buffer: &str, cursor: &mut usize) {
    if *cursor == 0 {
        return;
    }
    if let Some((index, _)) = buffer[..*cursor].char_indices().last() {
        *cursor = index;
    }
}

fn move_cursor_right(buffer: &str, cursor: &mut usize) {
    if *cursor >= buffer.len() {
        return;
    }
    if let Some(ch) = buffer[*cursor..].chars().next() {
        *cursor += ch.len_utf8();
    }
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

fn optimization_store_path() -> Result<PathBuf, String> {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(config_home).join("firebat/settings.json"));
    }
    let home = std::env::var("HOME").map_err(|error| error.to_string())?;
    Ok(PathBuf::from(home).join(".config/firebat/settings.json"))
}

fn load_optimization_store() -> Result<OptimizationStore, String> {
    let path = optimization_store_path()?;
    if !path.exists() {
        return Err(format!("settings file not found at {}", path.display()));
    }
    let json = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&json)
        .or_else(|_| {
            serde_json::from_str::<OptimizationStoreEnvelope>(&json)
                .map(|config| config.optimization_store)
        })
        .map_err(|error| error.to_string())
}

fn save_optimization_store(store: &OptimizationStore) -> Result<(), String> {
    let path = optimization_store_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let json = serde_json::to_string_pretty(store).map_err(|error| error.to_string())?;
    fs::write(path, json).map_err(|error| error.to_string())
}

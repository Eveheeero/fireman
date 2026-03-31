use super::{
    StartupConfig,
    types::{
        FileBrowserState, LOG_LIMIT, PickTabState, PipelineEntry, PromptKind, PromptState,
        ResultTabState, TabManager, TabType,
    },
};
use crate::{
    model::{
        DecompileRequest, KnownSection, KnownSectionData, OptimizationSettings,
        OptimizeAstRequest,
    },
    worker::{FirebatWorker, WorkerRequest, WorkerResponse, WorkerTryRecv},
};
use ratatui::crossterm::event;
use std::{
    collections::VecDeque,
    fs, io,
    path::Path,
    time::Duration,
};

pub(crate) struct App {
    pub(crate) worker: FirebatWorker,
    pub(crate) running: bool,
    pub(crate) tabs: TabManager,
    pub(crate) prompt: Option<PromptState>,
    pub(crate) busy_label: Option<String>,
    pub(crate) top_message: String,
    pub(crate) logs: VecDeque<String>,
    pub(crate) opened_path: Option<String>,
    pub(crate) pending_open_path: Option<String>,
    pub(crate) pending_analysis_address: Option<String>,
    pub(crate) known_sections: Vec<KnownSection>,
    pub(crate) section_cursor: usize,
    pub(crate) pipeline: Vec<PipelineEntry>,
    pub(crate) log_scroll: usize,
    pub(crate) last_decompile_selection: Vec<u64>,
    pub(crate) pending_decompile_target: Option<usize>,
    pub(crate) show_license: bool,
}

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

        // Default pipeline: Result_0 only. If we have optimization config, also add Pick_0.
        let mut pipeline = vec![PipelineEntry::Result(ResultTabState::new())];
        let mut tabs = TabManager::default();

        if startup.as_ref().and_then(|c| c.optimization_store.as_ref()).is_some() {
            pipeline.push(PipelineEntry::Pick(PickTabState::new(optimization)));
            tabs.tabs.push(super::types::Tab::with_label(TabType::Pick, "Pick 0"));
        }

        let mut app = Self {
            worker: FirebatWorker::spawn(),
            running: true,
            tabs,
            prompt: None,
            busy_label: None,
            top_message: "Open a binary to start".to_string(),
            logs: VecDeque::new(),
            opened_path: None,
            pending_open_path: None,
            pending_analysis_address: None,
            known_sections: Vec::new(),
            section_cursor: 0,
            pipeline,
            log_scroll: 0,
            last_decompile_selection: Vec::new(),
            pending_decompile_target: None,
            show_license: false,
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

    fn apply_startup_config(&mut self, startup: StartupConfig) {
        if startup.optimization_store.is_some() {
            self.set_status("Loaded startup optimization config");
        }
        if let Some(path) = startup.input_path {
            self.pending_open_path = Some(path.clone());
            self.send_request(WorkerRequest::OpenFile(path), "opening binary");
        }
    }

    // ── Pipeline helpers ─────────────────────────────────────────────

    /// Get the pipeline index for the current tab (tab_index - 2), or None if Input/Logs
    pub(crate) fn pipeline_index(&self) -> Option<usize> {
        let idx = self.tabs.current_index;
        if idx >= 2 { Some(idx - 2) } else { None }
    }

    /// Get current Pick state if on a Pick tab
    pub(crate) fn current_pick_state(&self) -> Option<&PickTabState> {
        let pi = self.pipeline_index()?;
        match self.pipeline.get(pi)? {
            PipelineEntry::Pick(pick) => Some(pick),
            _ => None,
        }
    }

    pub(crate) fn current_pick_state_mut(&mut self) -> Option<&mut PickTabState> {
        let pi = self.pipeline_index()?;
        match self.pipeline.get_mut(pi)? {
            PipelineEntry::Pick(pick) => Some(pick),
            _ => None,
        }
    }

    pub(crate) fn current_result_state(&self) -> Option<&ResultTabState> {
        let pi = self.pipeline_index()?;
        match self.pipeline.get(pi)? {
            PipelineEntry::Result(res) => Some(res),
            _ => None,
        }
    }

    pub(crate) fn current_result_state_mut(&mut self) -> Option<&mut ResultTabState> {
        let pi = self.pipeline_index()?;
        match self.pipeline.get_mut(pi)? {
            PipelineEntry::Result(res) => Some(res),
            _ => None,
        }
    }

    /// Add a new pipeline stage: appends Pick + Result at end
    pub(crate) fn add_pipeline_stage(&mut self) {
        let pick_n = self
            .pipeline
            .iter()
            .filter(|e| matches!(e, PipelineEntry::Pick(_)))
            .count();
        let result_n = self
            .pipeline
            .iter()
            .filter(|e| matches!(e, PipelineEntry::Result(_)))
            .count();

        // Add Pick tab at end
        self.pipeline
            .push(PipelineEntry::Pick(PickTabState::new(Default::default())));
        self.tabs
            .tabs
            .push(super::types::Tab::with_label(TabType::Pick, format!("Pick {}", pick_n)));

        // Add Result tab at end
        self.pipeline
            .push(PipelineEntry::Result(ResultTabState::new()));
        self.tabs
            .tabs
            .push(super::types::Tab::with_label(TabType::Result, format!("Result {}", result_n)));

        // Navigate to the new Pick tab
        self.tabs.current_index = self.tabs.tabs.len() - 2;
        self.log(format!(
            "Added pipeline stage (Pick {}, Result {})",
            pick_n, result_n
        ));
    }

    /// Remove the pipeline entry at the current tab position
    pub(crate) fn remove_pipeline_entry(&mut self) {
        let Some(pi) = self.pipeline_index() else {
            return;
        };
        if pi >= self.pipeline.len() {
            return;
        }
        let tab_idx = self.tabs.current_index;
        self.pipeline.remove(pi);
        self.tabs.remove_tab(tab_idx);
        self.log("Removed pipeline entry");
    }

    /// Cascade invalidation: clear all Result entries after pipeline index `from`
    pub(crate) fn cascade_invalidate(&mut self, from: usize) {
        for entry in self.pipeline.iter_mut().skip(from + 1) {
            if let PipelineEntry::Result(res) = entry {
                res.ast = None;
                res.outputs = None;
                res.cursor = 0;
            }
        }
    }

    /// Navigate to the first tab of the given type
    pub(crate) fn navigate_to_first(&mut self, target_type: TabType) {
        if let Some(idx) = self.tabs.tabs.iter().position(|t| t.tab_type == target_type) {
            self.tabs.current_index = idx;
        }
    }

    // ── Optimization settings resolution ─────────────────────────────

    /// Get optimization settings for decompile: from current Pick tab if on one,
    /// or from the preceding Pick in the pipeline if on a Result tab.
    fn resolve_opt_settings(&self) -> (OptimizationSettings, Option<String>) {
        let pi = match self.pipeline_index() {
            Some(pi) => pi,
            None => return Default::default(),
        };
        // If on a Pick tab, use it; if on Result, walk backward to find preceding Pick
        let pick_pi = match self.pipeline.get(pi) {
            Some(PipelineEntry::Pick(_)) => Some(pi),
            Some(PipelineEntry::Result(_)) => {
                (0..pi)
                    .rev()
                    .find(|&i| matches!(self.pipeline.get(i), Some(PipelineEntry::Pick(_))))
            }
            None => None,
        };
        pick_pi
            .and_then(|i| match &self.pipeline[i] {
                PipelineEntry::Pick(pick) => {
                    let buf = if pick.store.fb_script_enabled {
                        pick.store.applied_buffer_script.clone()
                    } else {
                        None
                    };
                    Some((pick.store.applied_settings.clone(), buf))
                }
                _ => None,
            })
            .unwrap_or_default()
    }

    // ── Worker communication ─────────────────────────────────────────

    pub(crate) fn send_request(&mut self, request: WorkerRequest, label: &str) {
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
                    self.pending_analysis_address = None;
                    self.known_sections.clear();
                    // Clear all Result outputs in pipeline
                    for entry in &mut self.pipeline {
                        if let PipelineEntry::Result(res) = entry {
                            res.ast = None;
                            res.outputs = None;
                            res.cursor = 0;
                        }
                    }
                    self.section_cursor = 0;
                    self.last_decompile_selection.clear();
                    self.navigate_to_first(TabType::Input);
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
                    self.navigate_to_first(TabType::Input);
                    self.set_status("Section analysis completed");
                }
                Err(error) => self.set_status(error),
            },
            WorkerResponse::AnalyzeAllSections(result) => match result {
                Ok(data) => {
                    self.merge_sections(data, false);
                    self.navigate_to_first(TabType::Input);
                    self.set_status("Analyzed all sections");
                }
                Err(error) => self.set_status(error),
            },
            WorkerResponse::DecompileSections(result) => match result {
                Ok(dwa) => {
                    let target_pi = self.pending_decompile_target.take();
                    if let Some(pi) = target_pi {
                        if let Some(PipelineEntry::Result(res)) = self.pipeline.get_mut(pi) {
                            res.ast = Some(dwa.ast);
                            res.outputs = Some(dwa.result);
                            res.cursor = 0;
                        }
                    }
                    self.set_status("Decompile completed");
                }
                Err(error) => self.set_status(error),
            },
            WorkerResponse::OptimizeAst(result) => match result {
                Ok(dwa) => {
                    let target_pi = self.pending_decompile_target.take();
                    if let Some(pi) = target_pi {
                        if let Some(PipelineEntry::Result(res)) = self.pipeline.get_mut(pi) {
                            res.ast = Some(dwa.ast);
                            res.outputs = Some(dwa.result);
                            res.cursor = 0;
                        }
                    }
                    self.set_status("Optimization completed");
                }
                Err(error) => self.set_status(error),
            },
        }
    }

    // ── Prompts ──────────────────────────────────────────────────────

    pub(crate) fn open_prompt(
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
            file_browser: None,
        });
    }

    pub(crate) fn open_path_prompt(
        &mut self,
        kind: PromptKind,
        title: impl Into<String>,
        text: String,
        help: impl Into<String>,
    ) {
        let cursor = text.len();
        let mut browser = FileBrowserState::new();
        browser.update_from_path(&text);
        self.prompt = Some(PromptState {
            kind,
            title: title.into(),
            text,
            cursor,
            multiline: false,
            help: help.into(),
            file_browser: Some(browser),
        });
    }

    pub(crate) fn submit_prompt(&mut self) {
        if self.busy_label.is_some() {
            self.set_status("Background worker is busy");
            return;
        }
        let Some(prompt) = self.prompt.take() else {
            return;
        };
        let value = prompt.text;
        match prompt.kind {
            PromptKind::OpenFile => {
                if value.trim().is_empty() {
                    self.set_status("Binary path is required");
                    return;
                }
                match self.worker.send(WorkerRequest::OpenFile(value.clone())) {
                    Ok(()) => {
                        self.pending_open_path = Some(value);
                        self.busy_label = Some("opening binary".to_string());
                        self.set_status("Started opening binary");
                    }
                    Err(error) => self.set_status(error),
                }
            }
            PromptKind::AnalyzeAddress => {
                match self
                    .worker
                    .send(WorkerRequest::AnalyzeSection(value.clone()))
                {
                    Ok(()) => {
                        self.pending_analysis_address = Some(value);
                        self.busy_label = Some("analyzing section".to_string());
                        self.set_status("Started analyzing section");
                    }
                    Err(error) => self.set_status(error),
                }
            }
            PromptKind::LoadBufferPath => self.load_buffer_from_path(value),
            PromptKind::SaveBufferPath => self.save_buffer_to_path(value),
        }
    }

    // ── Decompile ────────────────────────────────────────────────────

    pub(crate) fn start_decompile(&mut self) {
        let start_addresses = self.selected_addresses();
        if start_addresses.is_empty() {
            self.set_status("Select at least one analyzed section before decompiling");
            return;
        }
        self.last_decompile_selection = start_addresses.clone();

        let current_type = self.tabs.current_tab_type();
        let pi = self.pipeline_index();

        // Route result to appropriate pipeline Result entry
        self.pending_decompile_target = match current_type {
            Some(TabType::Result) => pi,
            Some(TabType::Pick) => {
                // Target the next Result after this Pick
                pi.and_then(|p| {
                    self.pipeline
                        .get(p + 1)
                        .and_then(|e| matches!(e, PipelineEntry::Result(_)).then_some(p + 1))
                })
            }
            _ => {
                // Input or other: target Result_0 (pipeline index 0)
                if matches!(self.pipeline.first(), Some(PipelineEntry::Result(_))) {
                    Some(0)
                } else {
                    None
                }
            }
        };

        let (settings, buffer_script) = self.resolve_opt_settings();

        // Try incremental: if on a Pick tab, check preceding Result for a stored Ast
        let preceding_ast = pi.and_then(|p| {
            if !matches!(self.pipeline.get(p), Some(PipelineEntry::Pick(_))) {
                return None;
            }
            (0..p).rev().find_map(|i| {
                if let Some(PipelineEntry::Result(res)) = self.pipeline.get(i) {
                    res.ast.clone()
                } else {
                    None
                }
            })
        });

        if let Some(ast) = preceding_ast {
            self.send_request(
                WorkerRequest::OptimizeAst(OptimizeAstRequest {
                    ast,
                    settings,
                    script_paths: vec![],
                    buffer_script,
                }),
                "optimizing AST",
            );
        } else {
            self.send_request(
                WorkerRequest::DecompileSections(DecompileRequest {
                    start_addresses,
                    settings,
                    script_paths: vec![],
                    buffer_script,
                }),
                "decompiling sections",
            );
        }
    }

    fn merge_sections(&mut self, sections: Vec<KnownSectionData>, select_new: bool) {
        for data in sections {
            if let Some(existing) = self
                .known_sections
                .iter_mut()
                .find(|section| section.data.start_address == data.start_address)
            {
                let keep_selected = existing.selected;
                existing.data = data;
                existing.selected = (keep_selected || select_new) && existing.data.analyzed;
            } else {
                let analyzed = data.analyzed;
                self.known_sections.push(KnownSection {
                    selected: select_new && analyzed,
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

    pub(crate) fn toggle_section(&mut self, index: usize) {
        let pending_section = match self.known_sections.get_mut(index) {
            Some(section) if !section.data.analyzed => {
                section.selected = false;
                true
            }
            Some(section) => {
                section.selected = !section.selected;
                false
            }
            None => return,
        };
        if pending_section {
            self.set_status("Analyze the section before selecting it");
        }
    }

    pub(crate) fn toggle_all_sections(&mut self) {
        let analyzed_count = self
            .known_sections
            .iter()
            .filter(|section| section.data.analyzed)
            .count();
        if analyzed_count == 0 {
            self.set_status("No analyzed sections are available yet");
            return;
        }
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

    // ── Optimization settings ────────────────────────────────────────

    pub(crate) fn apply_optimization_settings(&mut self) {
        if let Some(pick) = self.current_pick_state_mut() {
            pick.store.applied_settings = pick.store.draft_settings.clone();
            if pick.store.fb_script_enabled {
                pick.store.applied_buffer_script = if pick.store.editor_buffer.trim().is_empty() {
                    None
                } else {
                    Some(pick.store.editor_buffer.clone())
                };
            } else {
                pick.store.applied_buffer_script = None;
            }
        }
        // Cascade invalidation from current pipeline index forward
        if let Some(pi) = self.pipeline_index() {
            self.cascade_invalidate(pi);
        }
        self.set_status("Applied optimization settings");
        self.redecompile_last_selection();
    }

    pub(crate) fn load_buffer_from_path(&mut self, path: String) {
        let path = path.trim();
        if path.is_empty() {
            self.set_status("Buffer path is required");
            return;
        }
        match fs::read_to_string(path) {
            Ok(buffer) => {
                if let Some(pick) = self.current_pick_state_mut() {
                    pick.store.editor_buffer = buffer;
                    pick.store.editor_path = Some(path.to_string());
                }
                self.upsert_script_preset(Path::new(path));
                self.set_status(format!("Loaded buffer from {path}"));
            }
            Err(error) => self.set_status(error.to_string()),
        }
    }

    pub(crate) fn save_buffer_to_path(&mut self, path: String) {
        let path = path.trim();
        if path.is_empty() {
            self.set_status("Buffer save path is required");
            return;
        }
        let buffer_content = self
            .current_pick_state()
            .map(|pick| pick.store.editor_buffer.clone())
            .unwrap_or_default();
        match fs::write(path, &buffer_content) {
            Ok(()) => {
                if let Some(pick) = self.current_pick_state_mut() {
                    pick.store.editor_path = Some(path.to_string());
                }
                self.upsert_script_preset(Path::new(path));
                self.set_status(format!("Saved buffer to {path}"));
            }
            Err(error) => self.set_status(error.to_string()),
        }
    }

    pub(crate) fn redecompile_last_selection(&mut self) {
        if self.last_decompile_selection.is_empty() || self.busy_label.is_some() {
            return;
        }
        let current_type = self.tabs.current_tab_type();
        let pi = self.pipeline_index();

        self.pending_decompile_target = match current_type {
            Some(TabType::Result) => pi,
            Some(TabType::Pick) => {
                pi.and_then(|p| {
                    self.pipeline
                        .get(p + 1)
                        .and_then(|e| matches!(e, PipelineEntry::Result(_)).then_some(p + 1))
                })
            }
            _ => {
                if matches!(self.pipeline.first(), Some(PipelineEntry::Result(_))) {
                    Some(0)
                } else {
                    None
                }
            }
        };

        let (settings, buffer_script) = self.resolve_opt_settings();

        // Try incremental: if on a Pick tab, check preceding Result for a stored Ast
        let preceding_ast = pi.and_then(|p| {
            if !matches!(self.pipeline.get(p), Some(PipelineEntry::Pick(_))) {
                return None;
            }
            (0..p).rev().find_map(|i| {
                if let Some(PipelineEntry::Result(res)) = self.pipeline.get(i) {
                    res.ast.clone()
                } else {
                    None
                }
            })
        });

        if let Some(ast) = preceding_ast {
            self.send_request(
                WorkerRequest::OptimizeAst(OptimizeAstRequest {
                    ast,
                    settings,
                    script_paths: vec![],
                    buffer_script,
                }),
                "re-optimizing AST",
            );
        } else {
            self.send_request(
                WorkerRequest::DecompileSections(DecompileRequest {
                    start_addresses: self.last_decompile_selection.clone(),
                    settings,
                    script_paths: vec![],
                    buffer_script,
                }),
                "re-decompiling sections",
            );
        }
    }

    fn upsert_script_preset(&mut self, path: &Path) {
        let Some(pick) = self.current_pick_state_mut() else {
            return;
        };
        let path_string = path.to_string_lossy().to_string();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or(path_string.as_str())
            .to_string();
        if let Some(existing) = pick
            .store
            .script_presets
            .iter_mut()
            .find(|preset| preset.path == path_string)
        {
            existing.name = name;
            return;
        }
        pick.store
            .script_presets
            .push(crate::model::OptimizationScriptPreset {
                name,
                path: path_string,
                enabled: false,
                applied_enabled: false,
            });
    }

    pub(crate) fn load_saved_optimization_store(&mut self) {
        match super::persistence::load_optimization_store() {
            Ok(store) => {
                if let Some(pick) = self.current_pick_state_mut() {
                    pick.store = store;
                    pick.setting_cursor = pick
                        .setting_cursor
                        .min(super::types::optimization_field_count().saturating_sub(1));
                    pick.script_cursor = pick
                        .script_cursor
                        .min(pick.store.script_presets.len().saturating_sub(1));
                }
                self.set_status("Loaded optimization settings from disk");
                self.redecompile_last_selection();
            }
            Err(error) => self.set_status(format!("Optimization load failed: {error}")),
        }
    }

    pub(crate) fn save_current_optimization_store(&mut self) {
        let store = self.current_pick_state().map(|pick| pick.store.clone());
        match store {
            Some(store) => match super::persistence::save_optimization_store(&store) {
                Ok(()) => self.set_status("Saved optimization settings to disk"),
                Err(error) => self.set_status(format!("Optimization save failed: {error}")),
            },
            None => self.set_status("No active Pick tab"),
        }
    }

    // ── Logging ──────────────────────────────────────────────────────

    pub(crate) fn set_status(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.top_message = message.clone();
        self.log(message);
    }

    pub(crate) fn log(&mut self, message: impl Into<String>) {
        let message = message.into();
        let timestamp = format_timestamp();
        let entry = format!("[{timestamp}] {message}");
        if self.logs.len() == LOG_LIMIT {
            self.logs.pop_front();
        }
        self.logs.push_back(entry);
    }
}

fn format_timestamp() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}

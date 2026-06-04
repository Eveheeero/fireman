use super::{
    StartupConfig,
    types::{
        FileBrowserState, LOG_LIMIT, OptStage, PipelineEntry, PreviewState, PromptKind,
        PromptState, TabManager, TabType,
    },
};
use crate::{
    model::{DecompileRequest, KnownSection, KnownSectionData, OptimizeAstRequest},
    worker::{FirebatWorker, WorkerRequest, WorkerResponse, WorkerTryRecv},
};
use ratatui::crossterm::event;
use std::{collections::VecDeque, fs, io, path::Path, time::Duration};

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
    /// Raw decompile result before any Opt stages
    pub(crate) base_ast: Option<fireball::abstract_syntax_tree::Ast>,
    pub(crate) base_output: Option<crate::model::DecompileResult>,
    /// Queue of pipeline indices (Opt stages) pending optimization
    pub(crate) pending_optimize_queue: VecDeque<usize>,
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

        // Default pipeline: empty. If we have optimization config, add one Opt stage.
        let mut pipeline = Vec::new();
        let mut tabs = TabManager::default();

        if startup
            .as_ref()
            .and_then(|c| c.optimization_store.as_ref())
            .is_some()
        {
            pipeline.push(PipelineEntry::Opt(OptStage::new(optimization)));
            tabs.tabs
                .push(super::types::Tab::with_label(TabType::Opt, "Opt 0"));
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
            base_ast: None,
            base_output: None,
            pending_optimize_queue: VecDeque::new(),
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

    /// Get current Opt stage if on an Opt tab
    pub(crate) fn current_opt_stage(&self) -> Option<&OptStage> {
        let pi = self.pipeline_index()?;
        match self.pipeline.get(pi)? {
            PipelineEntry::Opt(opt) => Some(opt),
            _ => None,
        }
    }

    pub(crate) fn current_opt_stage_mut(&mut self) -> Option<&mut OptStage> {
        let pi = self.pipeline_index()?;
        match self.pipeline.get_mut(pi)? {
            PipelineEntry::Opt(opt) => Some(opt),
            _ => None,
        }
    }

    pub(crate) fn current_preview_state(&self) -> Option<&PreviewState> {
        let pi = self.pipeline_index()?;
        match self.pipeline.get(pi)? {
            PipelineEntry::Preview(prev) => Some(prev),
            _ => None,
        }
    }

    pub(crate) fn current_preview_state_mut(&mut self) -> Option<&mut PreviewState> {
        let pi = self.pipeline_index()?;
        match self.pipeline.get_mut(pi)? {
            PipelineEntry::Preview(prev) => Some(prev),
            _ => None,
        }
    }

    /// Add a new Opt stage at the end of the pipeline
    pub(crate) fn add_opt_stage(&mut self) {
        let opt_n = self
            .pipeline
            .iter()
            .filter(|e| matches!(e, PipelineEntry::Opt(_)))
            .count();

        self.pipeline
            .push(PipelineEntry::Opt(OptStage::new(Default::default())));
        self.tabs.tabs.push(super::types::Tab::with_label(
            TabType::Opt,
            format!("Opt {}", opt_n),
        ));

        // Navigate to the new Opt tab
        self.tabs.current_index = self.tabs.tabs.len() - 1;
        self.log(format!("Added Opt stage {}", opt_n));
    }

    /// Insert a Preview after the current pipeline position
    pub(crate) fn add_preview(&mut self) {
        let preview_n = self
            .pipeline
            .iter()
            .filter(|e| matches!(e, PipelineEntry::Preview(_)))
            .count();

        let pi = self.pipeline_index().unwrap_or(0);
        let insert_pi = (pi + 1).min(self.pipeline.len());
        let insert_tab = insert_pi + 2; // offset for Input+Logs

        let mut preview = PreviewState::new();
        // Populate snapshot from nearest preceding Opt output or base_ast
        self.populate_preview_snapshot(&mut preview, insert_pi);

        self.pipeline
            .insert(insert_pi, PipelineEntry::Preview(preview));
        self.tabs.tabs.insert(
            insert_tab,
            super::types::Tab::with_label(TabType::Preview, format!("Preview {}", preview_n)),
        );
        self.tabs.current_index = insert_tab;
        self.log(format!("Added Preview {}", preview_n));
    }

    /// Populate a preview snapshot from the nearest preceding Opt output or base data
    fn populate_preview_snapshot(&self, preview: &mut PreviewState, before_pi: usize) {
        // Walk backward to find nearest Opt with output
        for i in (0..before_pi).rev() {
            if let PipelineEntry::Opt(opt) = &self.pipeline[i] {
                if opt.output_ast.is_some() {
                    preview.ast = opt.output_ast.clone();
                    preview.outputs = opt.output.clone();
                    return;
                }
            }
        }
        // Fall back to base
        preview.ast = self.base_ast.clone();
        preview.outputs = self.base_output.clone();
    }

    /// Remove the pipeline entry at the current tab position
    pub(crate) fn remove_pipeline_entry(&mut self) {
        let Some(pi) = self.pipeline_index() else {
            return;
        };
        if pi >= self.pipeline.len() {
            return;
        }
        let is_opt = matches!(self.pipeline.get(pi), Some(PipelineEntry::Opt(_)));
        let tab_idx = self.tabs.current_index;
        self.pipeline.remove(pi);
        self.tabs.remove_tab(tab_idx);
        // If we removed an Opt, invalidate from the removed position onward
        // After remove(pi), entries that were at pi+1.. are now at pi..
        // We need to invalidate starting from pi (inclusive)
        if is_opt && !self.pipeline.is_empty() {
            if pi == 0 {
                // Invalidate everything including position 0
                for entry in self.pipeline.iter_mut() {
                    match entry {
                        PipelineEntry::Opt(opt) => {
                            opt.output_ast = None;
                            opt.output = None;
                        }
                        PipelineEntry::Preview(prev) => {
                            prev.ast = None;
                            prev.outputs = None;
                            prev.cursor = 0;
                        }
                    }
                }
            } else {
                self.cascade_invalidate(pi.saturating_sub(1));
            }
        }
        self.log("Removed pipeline entry");
    }

    /// Cascade invalidation: clear output for all Opt stages and Preview snapshots after `from`
    pub(crate) fn cascade_invalidate(&mut self, from: usize) {
        for entry in self.pipeline.iter_mut().skip(from + 1) {
            match entry {
                PipelineEntry::Opt(opt) => {
                    opt.output_ast = None;
                    opt.output = None;
                }
                PipelineEntry::Preview(prev) => {
                    prev.ast = None;
                    prev.outputs = None;
                    prev.cursor = 0;
                }
            }
        }
    }

    /// Navigate to the first tab of the given type
    pub(crate) fn navigate_to_first(&mut self, target_type: TabType) {
        if let Some(idx) = self
            .tabs
            .tabs
            .iter()
            .position(|t| t.tab_type == target_type)
        {
            self.tabs.current_index = idx;
        }
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
                    // Clear base and all pipeline outputs
                    self.base_ast = None;
                    self.base_output = None;
                    for entry in &mut self.pipeline {
                        match entry {
                            PipelineEntry::Opt(opt) => {
                                opt.output_ast = None;
                                opt.output = None;
                            }
                            PipelineEntry::Preview(prev) => {
                                prev.ast = None;
                                prev.outputs = None;
                                prev.cursor = 0;
                            }
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
                    // Store as base result
                    self.base_ast = Some(dwa.ast.clone());
                    self.base_output = Some(dwa.result.clone());
                    // Fill any leading Previews (before first Opt) with base data
                    self.fill_previews_from_base();
                    self.set_status("Decompile completed");
                    // Start processing optimize queue
                    self.process_optimize_queue();
                }
                Err(error) => self.set_status(error),
            },
            WorkerResponse::OptimizeAst(result) => match result {
                Ok(dwa) => {
                    let target_pi = self.pending_decompile_target.take();
                    if let Some(pi) = target_pi {
                        if let Some(PipelineEntry::Opt(opt)) = self.pipeline.get_mut(pi) {
                            opt.output_ast = Some(dwa.ast.clone());
                            opt.output = Some(dwa.result.clone());
                        }
                        // Fill subsequent Previews up to next Opt
                        self.fill_previews_after_opt(pi, &dwa.ast, &dwa.result);
                    }
                    self.set_status("Optimization completed");
                    // Process next in queue
                    self.process_optimize_queue();
                }
                Err(error) => {
                    self.pending_optimize_queue.clear();
                    self.set_status(error);
                }
            },
        }
    }

    /// Fill leading Preview entries (before first Opt) with base data
    fn fill_previews_from_base(&mut self) {
        let base_ast = self.base_ast.clone();
        let base_output = self.base_output.clone();
        for entry in &mut self.pipeline {
            match entry {
                PipelineEntry::Opt(_) => break,
                PipelineEntry::Preview(prev) => {
                    prev.ast = base_ast.clone();
                    prev.outputs = base_output.clone();
                }
            }
        }
    }

    /// Fill Preview entries after an Opt stage (up to next Opt) with that Opt's output
    fn fill_previews_after_opt(
        &mut self,
        opt_pi: usize,
        ast: &fireball::abstract_syntax_tree::Ast,
        result: &crate::model::DecompileResult,
    ) {
        for entry in self.pipeline.iter_mut().skip(opt_pi + 1) {
            match entry {
                PipelineEntry::Opt(_) => break,
                PipelineEntry::Preview(prev) => {
                    prev.ast = Some(ast.clone());
                    prev.outputs = Some(result.clone());
                }
            }
        }
    }

    /// Process the next entry in the optimize queue
    fn process_optimize_queue(&mut self) {
        if self.busy_label.is_some() {
            return;
        }
        let Some(pi) = self.pending_optimize_queue.pop_front() else {
            return;
        };
        // Find input AST: from preceding Opt's output_ast, or base_ast
        let input_ast = (0..pi)
            .rev()
            .find_map(|i| {
                if let PipelineEntry::Opt(opt) = &self.pipeline[i] {
                    opt.output_ast.clone()
                } else {
                    None
                }
            })
            .or_else(|| self.base_ast.clone());

        let Some(ast) = input_ast else {
            self.set_status("No input AST available for optimization stage");
            self.pending_optimize_queue.clear();
            return;
        };

        // Get settings from this Opt stage
        let (settings, buffer_script) = match &self.pipeline[pi] {
            PipelineEntry::Opt(opt) => {
                let buf = if opt.store.fb_script_enabled {
                    opt.store.applied_buffer_script.clone()
                } else {
                    None
                };
                (opt.store.applied_settings.clone(), buf)
            }
            _ => {
                // Not an Opt stage, skip
                self.process_optimize_queue();
                return;
            }
        };

        self.pending_decompile_target = Some(pi);
        self.send_request(
            WorkerRequest::OptimizeAst(OptimizeAstRequest {
                ast,
                settings,
                script_paths: vec![],
                buffer_script,
            }),
            "optimizing AST",
        );
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

        // Always start with a raw decompile. The response handler will
        // then enqueue Opt stages for sequential processing.
        // Raw decompile always uses default (no optimization) settings
        let (settings, buffer_script): (crate::model::OptimizationSettings, Option<String>) =
            Default::default();

        self.pending_decompile_target = None;
        self.pending_optimize_queue.clear();

        // Enqueue all Opt stage indices for after raw decompile completes
        for (i, entry) in self.pipeline.iter().enumerate() {
            if matches!(entry, PipelineEntry::Opt(_)) {
                self.pending_optimize_queue.push_back(i);
            }
        }

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
        if let Some(opt) = self.current_opt_stage_mut() {
            opt.store.applied_settings = opt.store.draft_settings.clone();
            if opt.store.fb_script_enabled {
                opt.store.applied_buffer_script = if opt.store.editor_buffer.trim().is_empty() {
                    None
                } else {
                    Some(opt.store.editor_buffer.clone())
                };
            } else {
                opt.store.applied_buffer_script = None;
            }
        }
        // Clear current Opt stage's cached output and cascade forward
        if let Some(pi) = self.pipeline_index() {
            if let Some(PipelineEntry::Opt(opt)) = self.pipeline.get_mut(pi) {
                opt.output_ast = None;
                opt.output = None;
            }
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
                if let Some(opt) = self.current_opt_stage_mut() {
                    opt.store.editor_buffer = buffer;
                    opt.store.editor_path = Some(path.to_string());
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
            .current_opt_stage()
            .map(|opt| opt.store.editor_buffer.clone())
            .unwrap_or_default();
        match fs::write(path, &buffer_content) {
            Ok(()) => {
                if let Some(opt) = self.current_opt_stage_mut() {
                    opt.store.editor_path = Some(path.to_string());
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

        // Clear queue and re-enqueue all Opt stages
        self.pending_optimize_queue.clear();
        for (i, entry) in self.pipeline.iter().enumerate() {
            if matches!(entry, PipelineEntry::Opt(_)) {
                self.pending_optimize_queue.push_back(i);
            }
        }

        // Start with raw decompile
        self.pending_decompile_target = None;
        self.send_request(
            WorkerRequest::DecompileSections(DecompileRequest {
                start_addresses: self.last_decompile_selection.clone(),
                settings: Default::default(),
                script_paths: vec![],
                buffer_script: None,
            }),
            "re-decompiling sections",
        );
    }

    fn upsert_script_preset(&mut self, path: &Path) {
        let Some(opt) = self.current_opt_stage_mut() else {
            return;
        };
        let path_string = path.to_string_lossy().to_string();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or(path_string.as_str())
            .to_string();
        if let Some(existing) = opt
            .store
            .script_presets
            .iter_mut()
            .find(|preset| preset.path == path_string)
        {
            existing.name = name;
            return;
        }
        opt.store
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
                if let Some(opt) = self.current_opt_stage_mut() {
                    opt.store = store;
                    opt.setting_cursor = opt
                        .setting_cursor
                        .min(super::types::optimization_field_count().saturating_sub(1));
                    opt.script_cursor = opt
                        .script_cursor
                        .min(opt.store.script_presets.len().saturating_sub(1));
                }
                self.set_status("Loaded optimization settings from disk");
                self.redecompile_last_selection();
            }
            Err(error) => self.set_status(format!("Optimization load failed: {error}")),
        }
    }

    pub(crate) fn save_current_optimization_store(&mut self) {
        let store = self.current_opt_stage().map(|opt| opt.store.clone());
        match store {
            Some(store) => match super::persistence::save_optimization_store(&store) {
                Ok(()) => self.set_status("Saved optimization settings to disk"),
                Err(error) => self.set_status(format!("Optimization save failed: {error}")),
            },
            None => self.set_status("No active Opt tab"),
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

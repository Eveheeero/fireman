use super::{
    StartupConfig,
    types::{
        FileBrowserState, LOG_LIMIT, OptimizationFocus, PromptKind, PromptState, View,
        next_position,
    },
};
use crate::{
    model::{
        AppliedEditResult, AssemblyEditorDraft, AstEditorDraft, DecompileRequest, DecompileResult,
        EditPosition, EditRequest, EditorDraft, EditorLayer, EditorTarget, IrEditorDraft,
        KnownSection, KnownSectionData, OptimizationScriptPreset, OptimizationStore,
    },
    worker::{FirebatWorker, WorkerRequest, WorkerResponse, WorkerTryRecv},
};
use ratatui::crossterm::event;
use std::{
    collections::VecDeque,
    fs, io,
    path::{Path, PathBuf},
    time::Duration,
};

pub(crate) struct App {
    pub(crate) worker: FirebatWorker,
    pub(crate) running: bool,
    pub(crate) current_view: View,
    pub(crate) prompt: Option<PromptState>,
    pub(crate) busy_label: Option<String>,
    pub(crate) top_message: String,
    pub(crate) logs: VecDeque<String>,
    pub(crate) opened_path: Option<String>,
    pub(crate) pending_open_path: Option<String>,
    pub(crate) pending_analysis_address: Option<String>,
    pub(crate) known_sections: Vec<KnownSection>,
    pub(crate) section_cursor: usize,
    pub(crate) outputs: Option<DecompileResult>,
    pub(crate) assembly_cursor: usize,
    pub(crate) ir_cursor: usize,
    pub(crate) ast_cursor: usize,
    pub(crate) hovered_assembly_index: Option<usize>,
    pub(crate) editor_target: Option<EditorTarget>,
    pub(crate) editor_draft: Option<EditorDraft>,
    pub(crate) optimization: OptimizationStore,
    pub(crate) optimization_focus: OptimizationFocus,
    pub(crate) optimization_setting_cursor: usize,
    pub(crate) optimization_script_cursor: usize,
    pub(crate) patch_preview: Option<String>,
    pub(crate) patch_scroll: usize,
    pub(crate) log_scroll: usize,
    pub(crate) last_decompile_selection: Vec<u64>,
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
            hovered_assembly_index: None,
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

    // -- Startup --

    fn apply_startup_config(&mut self, startup: StartupConfig) {
        if startup.optimization_store.is_some() {
            self.set_status("Loaded startup optimization config");
        }

        if let Some(path) = startup.input_path {
            self.pending_open_path = Some(path.clone());
            self.send_request(WorkerRequest::OpenFile(path), "opening binary");
        }
    }

    // -- Worker communication --

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
                    self.outputs = None;
                    self.patch_preview = None;
                    self.editor_target = None;
                    self.editor_draft = None;
                    self.section_cursor = 0;
                    self.assembly_cursor = 0;
                    self.ir_cursor = 0;
                    self.ast_cursor = 0;
                    self.patch_scroll = 0;
                    self.hovered_assembly_index = None;
                    self.last_decompile_selection.clear();
                    self.set_view(View::Sections);
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
                    self.set_view(View::Sections);
                    self.set_status("Section analysis completed");
                }
                Err(error) => self.set_status(error),
            },
            WorkerResponse::AnalyzeAllSections(result) => match result {
                Ok(data) => {
                    self.merge_sections(data, false);
                    self.set_view(View::Sections);
                    self.set_status("Analyzed all sections");
                }
                Err(error) => self.set_status(error),
            },
            WorkerResponse::DecompileSections(result) => match result {
                Ok(result) => {
                    self.outputs = Some(result);
                    self.patch_preview = None;
                    self.assembly_cursor = 0;
                    self.ir_cursor = 0;
                    self.ast_cursor = 0;
                    self.set_view(View::Assembly);
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
                    self.set_view(View::Patch);
                    self.set_status("Patch exported");
                }
                Err(error) => self.set_status(error),
            },
        }
    }

    // -- Prompt management --

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

    /// Open a path-related prompt with an integrated file browser.
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
        // Check if worker is busy before consuming the prompt
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
                // Send request first, then commit state on success
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
                // Send request first, then commit state on success
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
            PromptKind::EditLine(target) => {
                self.load_editor_with_text(target, value);
                self.set_view(View::Editor);
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

    // -- Decompile --

    pub(crate) fn start_decompile(&mut self) {
        let start_addresses = self.selected_addresses();
        if start_addresses.is_empty() {
            self.set_status("Select at least one analyzed section before decompiling");
            return;
        }

        self.last_decompile_selection = start_addresses.clone();
        self.patch_preview = None;
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

    // -- Edit --

    pub(crate) fn apply_edit(&mut self) {
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
        self.set_view(View::Editor);
        self.set_status("Edit applied");
    }

    // -- Sections --

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

    // -- Output cursor --

    pub(crate) fn output_len(&self, layer: EditorLayer) -> usize {
        match (layer, self.outputs.as_ref()) {
            (_, None) => 0,
            (EditorLayer::Assembly, Some(outputs)) => outputs.assembly.len(),
            (EditorLayer::Ir, Some(outputs)) => outputs.ir.len(),
            (EditorLayer::Ast, Some(outputs)) => outputs.ast.len(),
        }
    }

    pub(crate) fn output_cursor_mut(&mut self, layer: EditorLayer) -> &mut usize {
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

    // -- Editor --

    pub(crate) fn load_editor_from_current_row(&mut self, layer: EditorLayer) {
        let Some(target) = self.current_target(layer) else {
            self.set_status("No row is selected");
            return;
        };
        self.reload_editor(target);
        self.set_view(View::Editor);
    }

    pub(crate) fn edit_current_row(&mut self, layer: EditorLayer) {
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

    pub(crate) fn edit_loaded_draft(&mut self) {
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

    pub(crate) fn reload_editor(&mut self, target: EditorTarget) {
        let draft = self.row_text(target).map(|text| {
            draft_from_text(
                target.layer,
                &text,
                self.current_edit_position(target.layer),
            )
        });
        self.editor_target = draft.as_ref().map(|_| target);
        self.editor_draft = draft;
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

    pub(crate) fn cycle_edit_position(&mut self, forward: bool) {
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
            EditorLayer::Assembly => outputs.assembly.get(target.row).map(|row| row.data.clone()),
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

    pub(crate) fn set_view(&mut self, view: View) {
        self.current_view = view;
        self.update_hover();
    }

    // -- Hover tracking --

    /// Update the hovered assembly index based on the current view and cursor.
    pub(crate) fn update_hover(&mut self) {
        self.hovered_assembly_index = match self.current_view {
            View::Assembly => self
                .outputs
                .as_ref()
                .and_then(|o| o.assembly.get(self.assembly_cursor))
                .map(|row| row.index),
            View::Ir => self
                .outputs
                .as_ref()
                .and_then(|o| o.ir.get(self.ir_cursor))
                .map(|row| row.parents_assembly_index),
            _ => None,
        };
    }

    // -- Optimization --

    pub(crate) fn apply_optimization_settings(&mut self) {
        self.optimization.applied_settings = self.optimization.draft_settings.clone();
        for preset in &mut self.optimization.script_presets {
            preset.applied_enabled = preset.enabled;
        }
        self.set_status("Applied optimization settings and script presets");
        self.redecompile_last_selection();
    }

    pub(crate) fn apply_buffer_script(&mut self) {
        self.optimization.applied_buffer_script =
            if self.optimization.editor_buffer.trim().is_empty() {
                None
            } else {
                Some(self.optimization.editor_buffer.clone())
            };
        self.set_status("Applied optimization buffer script");
        self.redecompile_last_selection();
    }

    pub(crate) fn clear_applied_buffer_script(&mut self) {
        self.optimization.applied_buffer_script = None;
        self.set_status("Cleared applied optimization buffer script");
        self.redecompile_last_selection();
    }

    pub(crate) fn add_script_preset(&mut self, path: String) {
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

    pub(crate) fn remove_script_preset(&mut self, index: usize) {
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

    pub(crate) fn load_script_preset_into_buffer(&mut self, index: usize) {
        let Some(preset) = self.optimization.script_presets.get(index) else {
            self.set_status("No script preset selected");
            return;
        };
        self.load_buffer_from_path(preset.path.clone());
    }

    pub(crate) fn load_buffer_from_path(&mut self, path: String) {
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

    pub(crate) fn save_buffer_to_path(&mut self, path: String) {
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

    pub(crate) fn save_patch_preview(&mut self, path: String) {
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
        self.patch_preview = None;
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

    pub(crate) fn load_saved_optimization_store(&mut self) {
        match super::persistence::load_optimization_store() {
            Ok(store) => {
                self.optimization = store;
                self.optimization_setting_cursor = self
                    .optimization_setting_cursor
                    .min(super::types::optimization_field_count().saturating_sub(1));
                self.optimization_script_cursor = self
                    .optimization_script_cursor
                    .min(self.optimization.script_presets.len().saturating_sub(1));
                self.set_status("Loaded optimization settings from disk");
                self.redecompile_last_selection();
            }
            Err(error) => self.set_status(format!("Optimization load failed: {error}")),
        }
    }

    pub(crate) fn save_current_optimization_store(&mut self) {
        match super::persistence::save_optimization_store(&self.optimization) {
            Ok(()) => self.set_status("Saved optimization settings to disk"),
            Err(error) => self.set_status(format!("Optimization save failed: {error}")),
        }
    }

    // -- Status and logging --

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

fn format_timestamp() -> String {
    chrono::Local::now().format("%H:%M:%S").to_string()
}

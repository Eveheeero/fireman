use crate::{
    app::editor_window::FloatingEditorWindow,
    model::{
        AssemblyEditorDraft, AstNodeDraftData, AstNodeEditType, AstNodeEditorDraft,
        DecompileRequest, DecompileResult, DecompileResultView, EditPosition, EditRequest,
        EditorDraft, EditorLayer, EditorTarget, IrEditorDraft, KnownSection, KnownSectionData,
        OptimizationScriptPreset, OptimizationSettings, OptimizationStore,
    },
    worker::{FirebatWorker, WorkerRequest, WorkerResponse, WorkerTryRecv},
};
use chrono::Local;
use eframe::egui::Color32;
use rfd::FileDialog;
use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

pub(super) struct FirebatState {
    pub(super) worker: FirebatWorker,
    pub(super) pending_requests: usize,
    pub(super) known_sections: Vec<KnownSection>,
    pub(super) decompile_result: Option<DecompileResultView>,
    pub(super) hovered_assembly_index: Option<usize>,
    pub(super) hover_candidate: Option<usize>,
    pub(super) logs: Vec<String>,
    pub(super) log_expanded: bool,
    pub(super) analyze_target_address: String,
    pub(super) editor_target: Option<EditorTarget>,
    pub(super) editor_draft: Option<EditorDraft>,
    pub(super) exported_patch_json: Option<String>,
    pub(super) optimization_draft: OptimizationSettings,
    pub(super) optimization_applied: OptimizationSettings,
    pub(super) optimization_scripts: Vec<OptimizationScriptPreset>,
    pub(super) optimization_editor_buffer: String,
    pub(super) optimization_editor_path: Option<String>,
    pub(super) optimization_applied_buffer_script: Option<String>,
    pub(super) optimization_status_message: Option<String>,
    pub(super) last_decompile_selection: Vec<u64>,
    // Floating editor windows
    pub(super) assembly_editor: FloatingEditorWindow,
    pub(super) ir_editor: FloatingEditorWindow,
    pub(super) ast_editor: FloatingEditorWindow,
    // Track selections for highlighting
    pub(super) selected_assembly_row: Option<usize>,
    pub(super) selected_ir_row: Option<usize>,
    pub(super) selected_ast_path: Option<fireball::abstract_syntax_tree::AstNodePath>,
    pub(super) show_ir_comments: bool, // Toggle for showing IR origin comments in AST
}

impl Default for FirebatState {
    fn default() -> Self {
        let optimization_store = load_optimization_store().unwrap_or_default();
        let mut assembly_editor = FloatingEditorWindow::new("Assembly Editor");
        let mut ir_editor = FloatingEditorWindow::new("IR Editor");
        let mut ast_editor = FloatingEditorWindow::new("AST Editor");

        // Position windows cascaded
        assembly_editor.position = Some(egui::pos2(50.0, 100.0));
        ir_editor.position = Some(egui::pos2(80.0, 130.0));
        ast_editor.position = Some(egui::pos2(110.0, 160.0));

        Self {
            worker: FirebatWorker::spawn(),
            pending_requests: 0,
            known_sections: Vec::new(),
            decompile_result: None,
            hovered_assembly_index: None,
            hover_candidate: None,
            logs: Vec::new(),
            log_expanded: false,
            analyze_target_address: String::new(),
            editor_target: None,
            editor_draft: None,
            exported_patch_json: None,
            optimization_draft: optimization_store.draft_settings,
            optimization_applied: optimization_store.applied_settings,
            optimization_scripts: optimization_store.script_presets,
            optimization_editor_buffer: optimization_store.editor_buffer,
            optimization_editor_path: optimization_store.editor_path,
            optimization_applied_buffer_script: optimization_store.applied_buffer_script,
            optimization_status_message: None,
            last_decompile_selection: Vec::new(),
            assembly_editor,
            ir_editor,
            ast_editor,
            selected_assembly_row: None,
            selected_ir_row: None,
            selected_ast_path: None,
            show_ir_comments: false, // Default to hidden IR comments
        }
    }
}

impl FirebatState {
    pub(super) fn log(&mut self, message: impl AsRef<str>) {
        let timestamp = Local::now().format("%H:%M:%S");
        self.logs
            .push(format!("[{timestamp}] {}", message.as_ref().trim_end()));
    }

    pub(super) fn is_busy(&self) -> bool {
        self.pending_requests > 0
    }

    pub(super) fn queue_request(&mut self, request: WorkerRequest) {
        match self.worker.send(request) {
            Ok(()) => {
                self.pending_requests = self.pending_requests.saturating_add(1);
            }
            Err(error) => self.log(error),
        }
    }

    pub(super) fn optimization_is_dirty(&self) -> bool {
        self.optimization_draft != self.optimization_applied
            || self
                .optimization_scripts
                .iter()
                .any(|preset| preset.enabled != preset.applied_enabled)
    }

    fn build_decompile_request(&self, start_addresses: Vec<u64>) -> DecompileRequest {
        DecompileRequest {
            start_addresses,
            settings: self.optimization_applied.clone(),
            script_paths: self
                .optimization_scripts
                .iter()
                .filter(|preset| preset.applied_enabled)
                .map(|preset| preset.path.clone())
                .collect(),
            buffer_script: self.optimization_applied_buffer_script.clone(),
        }
    }

    fn set_optimization_status(&mut self, message: impl Into<String>) {
        self.optimization_status_message = Some(message.into());
    }

    pub(super) fn persist_optimization_state(&mut self) {
        if let Err(error) = self.save_optimization_store() {
            self.log(format!("Optimization settings persistence failed {error}"));
            self.set_optimization_status(format!("Persistence failed: {error}"));
        }
    }

    pub(super) fn poll_worker(&mut self) {
        loop {
            match self.worker.try_recv() {
                WorkerTryRecv::Message(response) => {
                    self.pending_requests = self.pending_requests.saturating_sub(1);
                    match response {
                        WorkerResponse::OpenFile(result) => match result {
                            Ok(()) => self.log("Open success"),
                            Err(error) => self.log(format!("Open failed {error}")),
                        },
                        WorkerResponse::AnalyzeSection(result) => match result {
                            Ok(sections) => {
                                self.log(format!("Section analyzation success {}", sections.len()));
                                self.merge_known_sections(sections);
                            }
                            Err(error) => {
                                self.log(format!("Section analyzation failed {error}"));
                            }
                        },
                        WorkerResponse::AnalyzeAllSections(result) => match result {
                            Ok(sections) => {
                                self.log(format!("All sections analyzed {}", sections.len()));
                                self.merge_known_sections(sections);
                            }
                            Err(error) => {
                                self.log(format!("All sections analyzation failed {error}"));
                            }
                        },
                        WorkerResponse::DecompileSections(result) => match result {
                            Ok(result) => {
                                self.log(format!(
                                    "Decompilation ready: {} asm, {} ir, {} ast lines",
                                    result.assembly.len(),
                                    result.ir.len(),
                                    result.ast.len()
                                ));
                                self.set_decompile_result(result);
                            }
                            Err(error) => self.log(format!("Decompilation failed {error}")),
                        },
                        WorkerResponse::ApplyEdit(result) => match result {
                            Ok(result) => {
                                self.log(format!(
                                    "Applied {} edit at row {}",
                                    layer_name(result.selected_target.layer),
                                    result.selected_target.row
                                ));
                                self.set_decompile_result(result.result);
                                self.editor_target = Some(result.selected_target);
                                self.reset_editor_draft();
                            }
                            Err(error) => {
                                self.log(format!("Edit apply failed {error}"));
                                self.set_draft_status(error);
                            }
                        },
                        WorkerResponse::ExportPatch(result) => match result {
                            Ok(json) => {
                                self.log(format!("Exported patch with {} bytes", json.len()));
                                self.exported_patch_json = Some(json);
                            }
                            Err(error) => {
                                self.log(format!("Patch export failed {error}"));
                                self.set_draft_status(error);
                            }
                        },
                    }
                }
                WorkerTryRecv::Empty => break,
                WorkerTryRecv::Disconnected => {
                    if self.pending_requests > 0 {
                        self.pending_requests = 0;
                        self.log("Background worker disconnected");
                    }
                    break;
                }
            }
        }
    }

    fn merge_known_sections(&mut self, sections: Vec<KnownSectionData>) {
        for section in sections {
            if let Some(existing) = self
                .known_sections
                .iter_mut()
                .find(|known| known.data.start_address == section.start_address)
            {
                let keep_selected = existing.selected;
                existing.data = section;
                existing.selected = keep_selected && existing.data.analyzed;
            } else {
                self.known_sections.push(KnownSection {
                    selected: false,
                    data: section,
                });
            }
        }
        self.known_sections
            .sort_by_key(|section| section.data.start_address);
    }

    pub(super) fn open_file(&mut self) {
        let Some(path) = FileDialog::new().pick_file() else {
            self.log("Open canceled");
            return;
        };

        let path = path.to_string_lossy().to_string();
        self.known_sections.clear();
        self.decompile_result = None;
        self.editor_target = None;
        self.editor_draft = None;
        self.exported_patch_json = None;
        self.hovered_assembly_index = None;
        self.hover_candidate = None;
        self.last_decompile_selection.clear();
        self.log(format!("Open fireball with {path}"));
        self.queue_request(WorkerRequest::OpenFile(path));
    }

    pub(super) fn analyze_section_from_address(&mut self, start_address: &str) {
        let trimmed_address = start_address.trim().to_owned();
        if !trimmed_address.is_empty() && crate::core::parse_address(&trimmed_address).is_err() {
            self.log(format!("Invalid address {start_address}"));
            return;
        }

        if let Ok(parsed_address) = crate::core::parse_address(&trimmed_address) {
            if self.known_sections.iter().any(|section| {
                section.data.analyzed && section.data.start_address == parsed_address
            }) {
                self.log(format!("Section already known {start_address}"));
                return;
            }
        }

        self.queue_request(WorkerRequest::AnalyzeSection(trimmed_address));
    }

    pub(super) fn analyze_all(&mut self) {
        self.queue_request(WorkerRequest::AnalyzeAllSections);
    }

    pub(super) fn select_all(&mut self) {
        let analyzed_sections = self
            .known_sections
            .iter()
            .filter(|section| section.data.analyzed)
            .collect::<Vec<_>>();
        if analyzed_sections.is_empty() {
            self.log("No analyzed sections available to select");
            return;
        }
        let all_selected = analyzed_sections.iter().all(|section| section.selected);
        for section in &mut self.known_sections {
            if section.data.analyzed {
                section.selected = !all_selected;
            }
        }
    }

    pub(super) fn decompile_selected(&mut self) {
        let has_pending_selection = self
            .known_sections
            .iter()
            .any(|section| section.selected && !section.data.analyzed);
        let selected = self
            .known_sections
            .iter()
            .filter(|section| section.selected && section.data.analyzed)
            .map(|section| section.data.start_address)
            .collect::<Vec<_>>();

        if selected.is_empty() {
            if has_pending_selection {
                self.log("Selected sections are not analyzed yet; analyze them before decompiling");
            } else {
                self.log("No analyzed sections selected for decompilation");
            }
            return;
        }

        self.exported_patch_json = None;
        self.last_decompile_selection = selected.clone();
        self.log(format!("Decompiling sections {selected:?}"));
        self.queue_request(WorkerRequest::DecompileSections(
            self.build_decompile_request(selected),
        ));
    }

    pub(super) fn apply_optimization_settings(&mut self) {
        self.optimization_applied = self.optimization_draft.clone();
        for preset in &mut self.optimization_scripts {
            preset.applied_enabled = preset.enabled;
        }
        self.persist_optimization_state();
        if self.last_decompile_selection.is_empty() {
            self.set_optimization_status("Optimization settings saved for the next decompile");
            self.log("Optimization settings saved for the next decompile");
            return;
        }
        self.set_optimization_status("Reapplying optimization settings to the current selection");
        self.log("Reapplying optimization settings to the current selection");
        self.queue_request(WorkerRequest::DecompileSections(
            self.build_decompile_request(self.last_decompile_selection.clone()),
        ));
    }

    pub(super) fn apply_optimization_buffer(&mut self) {
        self.optimization_applied_buffer_script =
            if self.optimization_editor_buffer.trim().is_empty() {
                None
            } else {
                Some(self.optimization_editor_buffer.clone())
            };
        self.persist_optimization_state();
        if self.last_decompile_selection.is_empty() {
            self.set_optimization_status("Applied editor buffer for the next decompile");
            self.log("Applied optimization buffer for the next decompile");
            return;
        }
        self.set_optimization_status("Applying optimization buffer to the current selection");
        self.log("Applying optimization buffer to the current selection");
        self.queue_request(WorkerRequest::DecompileSections(
            self.build_decompile_request(self.last_decompile_selection.clone()),
        ));
    }

    pub(super) fn clear_applied_optimization_buffer(&mut self) {
        self.optimization_applied_buffer_script = None;
        self.persist_optimization_state();
        if self.last_decompile_selection.is_empty() {
            self.set_optimization_status("Cleared applied buffer script");
            self.log("Cleared applied optimization buffer");
            return;
        }
        self.set_optimization_status("Clearing applied buffer and recompiling current selection");
        self.log("Clearing applied optimization buffer and recompiling current selection");
        self.queue_request(WorkerRequest::DecompileSections(
            self.build_decompile_request(self.last_decompile_selection.clone()),
        ));
    }

    pub(super) fn restore_default_optimization_draft(&mut self) {
        self.optimization_draft = OptimizationSettings::default();
        for preset in &mut self.optimization_scripts {
            preset.enabled = false;
        }
        self.persist_optimization_state();
        self.set_optimization_status("Optimization draft restored to defaults");
    }

    pub(super) fn reset_optimization_draft(&mut self) {
        self.optimization_draft = self.optimization_applied.clone();
        for preset in &mut self.optimization_scripts {
            preset.enabled = preset.applied_enabled;
        }
        self.persist_optimization_state();
        self.set_optimization_status("Optimization draft reset to the applied state");
    }

    pub(super) fn new_optimization_script(&mut self) {
        self.optimization_editor_buffer.clear();
        self.optimization_editor_path = None;
        self.persist_optimization_state();
        self.set_optimization_status("Started a new optimization script buffer");
    }

    pub(super) fn open_optimization_script(&mut self) {
        let Some(path) = FileDialog::new()
            .add_filter("Firebat optimization", &["fb"])
            .pick_file()
        else {
            self.log("Optimization script open canceled");
            return;
        };
        match fs::read_to_string(&path) {
            Ok(buffer) => {
                self.optimization_editor_buffer = buffer;
                self.optimization_editor_path = Some(path.to_string_lossy().to_string());
                self.upsert_optimization_script_preset(&path);
                self.persist_optimization_state();
                self.set_optimization_status(format!(
                    "Opened optimization script {}",
                    path.to_string_lossy()
                ));
            }
            Err(error) => {
                self.log(format!("Optimization script open failed {error}"));
                self.set_optimization_status(format!("Open failed: {error}"));
            }
        }
    }

    pub(super) fn save_optimization_script(&mut self) {
        let Some(path) = self.optimization_editor_path.clone() else {
            self.save_optimization_script_as();
            return;
        };
        self.write_optimization_script(Path::new(&path));
    }

    pub(super) fn save_optimization_script_as(&mut self) {
        let dialog = FileDialog::new().add_filter("Firebat optimization", &["fb"]);
        let dialog = if let Some(path) = &self.optimization_editor_path {
            dialog.set_file_name(
                Path::new(path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("optimization.fb"),
            )
        } else {
            dialog.set_file_name("optimization.fb")
        };
        let Some(path) = dialog.save_file() else {
            self.log("Optimization script save canceled");
            return;
        };
        self.write_optimization_script(&path);
    }

    pub(super) fn load_optimization_preset_into_editor(&mut self, index: usize) {
        let Some(path) = self
            .optimization_scripts
            .get(index)
            .map(|preset| preset.path.clone())
        else {
            return;
        };
        match fs::read_to_string(&path) {
            Ok(buffer) => {
                self.optimization_editor_buffer = buffer;
                self.optimization_editor_path = Some(path.clone());
                self.persist_optimization_state();
                self.set_optimization_status(format!("Loaded preset {path} into the editor"));
            }
            Err(error) => {
                self.log(format!("Preset load failed {error}"));
                self.set_optimization_status(format!("Preset load failed: {error}"));
            }
        }
    }

    pub(super) fn remove_optimization_script_preset(&mut self, index: usize) {
        if index >= self.optimization_scripts.len() {
            return;
        }
        let removed = self.optimization_scripts.remove(index);
        self.persist_optimization_state();
        self.set_optimization_status(format!("Removed script preset {}", removed.name));
    }

    pub(super) fn select_editor_target(&mut self, target: EditorTarget) {
        self.editor_target = Some(target);
        self.exported_patch_json = None;
        self.reset_editor_draft();
    }

    pub(super) fn reset_editor_draft(&mut self) {
        let Some(target) = self.editor_target else {
            self.editor_draft = None;
            return;
        };
        let Some(result) = self.decompile_result.as_ref() else {
            self.editor_draft = None;
            return;
        };

        self.editor_draft = build_editor_draft(result, target);
        if self.editor_draft.is_none() {
            self.editor_target = None;
        }
    }

    pub(super) fn apply_current_edit(&mut self) {
        let Some(target) = self.editor_target else {
            self.log("Select a row before applying an edit");
            return;
        };
        let Some(request) = self.build_current_edit_request(target) else {
            self.log("No edit draft is available");
            return;
        };
        self.exported_patch_json = None;
        self.queue_request(WorkerRequest::ApplyEdit(request));
    }

    pub(super) fn export_patch(&mut self) {
        if self.decompile_result.is_none() {
            self.log("Run decompilation before exporting a patch");
            return;
        }
        self.queue_request(WorkerRequest::ExportPatch);
    }

    fn build_current_edit_request(&self, target: EditorTarget) -> Option<EditRequest> {
        let draft = self.editor_draft.as_ref()?;
        let request = match draft {
            EditorDraft::Assembly(draft) => EditRequest {
                layer: EditorLayer::Assembly,
                row: target.row,
                position: EditPosition::Replace,
                text: draft.raw_text.clone(),
            },
            EditorDraft::Ir(draft) => EditRequest {
                layer: EditorLayer::Ir,
                row: target.row,
                position: draft.position,
                text: draft.raw_text.clone(),
            },
            EditorDraft::Ast(draft) => EditRequest {
                layer: EditorLayer::Ast,
                row: target.row,
                position: draft.position,
                text: draft.raw_text.clone(),
            },
            EditorDraft::AstNode(draft) => {
                // Convert node edit to text edit
                // For now, use the replacement text from the draft
                let text = match &draft.draft_data {
                    AstNodeDraftData::Statement { replacement, .. } => replacement.clone(),
                    AstNodeDraftData::Variable { new_name, .. } => {
                        format!("// rename to {}", new_name)
                    }
                    AstNodeDraftData::Literal { new_value, .. } => new_value.clone(),
                    AstNodeDraftData::UnaryOperator {
                        new_op, operand, ..
                    } => format!("{}({})", new_op, operand),
                    AstNodeDraftData::BinaryOperator {
                        new_op,
                        left,
                        right,
                        ..
                    } => format!("{} {} {}", left, new_op, right),
                    AstNodeDraftData::Function { new_name, .. } => {
                        format!("// rename function to {}", new_name)
                    }
                };
                EditRequest {
                    layer: EditorLayer::Ast,
                    row: target.row,
                    position: EditPosition::Replace,
                    text,
                }
            }
        };
        Some(request)
    }

    fn set_decompile_result(&mut self, result: DecompileResult) {
        self.decompile_result = Some(build_decompile_view(result));
        self.exported_patch_json = None;
        self.hovered_assembly_index = None;
        self.hover_candidate = None;
        let Some(target) = self.editor_target else {
            self.editor_draft = None;
            return;
        };
        self.editor_draft = self
            .decompile_result
            .as_ref()
            .and_then(|view| build_editor_draft(view, target));
        if self.editor_draft.is_none() {
            self.editor_target = None;
        }
    }

    fn set_draft_status(&mut self, message: String) {
        if let Some(draft) = self.editor_draft.as_mut() {
            match draft {
                EditorDraft::Assembly(draft) => draft.status_message = Some(message),
                EditorDraft::Ir(draft) => draft.status_message = Some(message),
                EditorDraft::Ast(draft) => draft.status_message = Some(message),
                EditorDraft::AstNode(draft) => draft.status_message = Some(message),
            }
        }
    }

    fn save_optimization_store(&self) -> Result<(), String> {
        let path = optimization_store_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        let store = OptimizationStore {
            draft_settings: self.optimization_draft.clone(),
            applied_settings: self.optimization_applied.clone(),
            script_presets: self.optimization_scripts.clone(),
            editor_buffer: self.optimization_editor_buffer.clone(),
            editor_path: self.optimization_editor_path.clone(),
            applied_buffer_script: self.optimization_applied_buffer_script.clone(),
        };
        let json = serde_json::to_string_pretty(&store).map_err(|error| error.to_string())?;
        fs::write(path, json).map_err(|error| error.to_string())
    }

    fn write_optimization_script(&mut self, path: &Path) {
        match fs::write(path, &self.optimization_editor_buffer) {
            Ok(()) => {
                self.optimization_editor_path = Some(path.to_string_lossy().to_string());
                self.upsert_optimization_script_preset(path);
                self.persist_optimization_state();
                self.set_optimization_status(format!(
                    "Saved optimization script {}",
                    path.to_string_lossy()
                ));
            }
            Err(error) => {
                self.log(format!("Optimization script save failed {error}"));
                self.set_optimization_status(format!("Save failed: {error}"));
            }
        }
    }

    fn upsert_optimization_script_preset(&mut self, path: &Path) {
        let path_string = path.to_string_lossy().to_string();
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("script.fb")
            .to_string();
        if let Some(preset) = self
            .optimization_scripts
            .iter_mut()
            .find(|preset| preset.path == path_string)
        {
            preset.name = name;
            return;
        }
        self.optimization_scripts.push(OptimizationScriptPreset {
            name,
            path: path_string,
            enabled: false,
            applied_enabled: false,
        });
        self.optimization_scripts
            .sort_by(|left, right| left.name.cmp(&right.name));
    }
}

fn optimization_store_path() -> Result<PathBuf, String> {
    if let Ok(config_home) = env::var("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(config_home).join("firebat/settings.json"));
    }

    let home = env::var("HOME").map_err(|error| error.to_string())?;
    Ok(PathBuf::from(home).join(".config/firebat/settings.json"))
}

fn load_optimization_store() -> Result<OptimizationStore, String> {
    let path = optimization_store_path()?;
    if !path.exists() {
        return Ok(OptimizationStore::default());
    }
    let json = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&json).map_err(|error| error.to_string())
}

fn build_editor_draft(result: &DecompileResultView, target: EditorTarget) -> Option<EditorDraft> {
    match target.layer {
        EditorLayer::Assembly => result.data.assembly.get(target.row).map(|assembly| {
            EditorDraft::Assembly(AssemblyEditorDraft::from_display_text(&assembly.data))
        }),
        EditorLayer::Ir => result
            .data
            .ir
            .get(target.row)
            .map(|ir| EditorDraft::Ir(IrEditorDraft::from_text(&ir.data))),
        EditorLayer::Ast => {
            // Create new-style AST node editor draft
            result.data.ast.get(target.row).map(|ast| {
                // For now, create a simple statement editor draft
                // In the full implementation, we'd parse the AST line and determine the type
                EditorDraft::AstNode(AstNodeEditorDraft {
                    path: fireball::abstract_syntax_tree::AstNodePath::function(0),
                    edit_type: AstNodeEditType::Statement,
                    draft_data: AstNodeDraftData::Statement {
                        statement_type: "unknown".to_string(),
                        replacement: ast.data.clone(),
                    },
                    status_message: None,
                })
            })
        }
    }
}

fn build_decompile_view(result: DecompileResult) -> DecompileResultView {
    let mut colors = HashMap::new();
    let mut assembly_parent_by_index = HashMap::new();
    for assembly in &result.assembly {
        colors.insert(assembly.index, get_color_for_index(assembly.index));
        assembly_parent_by_index.insert(assembly.index, assembly.parents_start_address);
    }

    DecompileResultView {
        colors,
        assembly_parent_by_index,
        data: result.clone(),
        ast: result.ast_object.clone(), // Clone Arc reference to AST
    }
}

fn layer_name(layer: EditorLayer) -> &'static str {
    match layer {
        EditorLayer::Assembly => "assembly",
        EditorLayer::Ir => "IR",
        EditorLayer::Ast => "AST",
    }
}

const fn get_color_for_index(index: usize) -> Color32 {
    const COLORS: [Color32; 10] = [
        Color32::from_rgb(0x0F, 0x6C, 0xBD),
        Color32::from_rgb(0x11, 0x5E, 0xA3),
        Color32::from_rgb(0x00, 0x5A, 0x9C),
        Color32::from_rgb(0x00, 0x78, 0xD4),
        Color32::from_rgb(0x03, 0x83, 0x87),
        Color32::from_rgb(0x0F, 0x7B, 0x0F),
        Color32::from_rgb(0x10, 0x7C, 0x10),
        Color32::from_rgb(0x8A, 0x37, 0x00),
        Color32::from_rgb(0xCA, 0x50, 0x10),
        Color32::from_rgb(0x52, 0x52, 0x52),
    ];
    COLORS[index % COLORS.len()]
}

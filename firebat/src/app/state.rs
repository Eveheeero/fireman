use crate::{
    app::editor_window::FloatingEditorWindow,
    model::{
        AssemblyEditorDraft, AstNodeDraftData, AstNodeEditType, AstNodeEditorDraft,
        DecompileResult, DecompileResultView, EditPosition, EditRequest, EditorDraft, EditorLayer,
        EditorTarget, IrEditorDraft, KnownSection, KnownSectionData, OptimizeAstResult,
    },
    node::NodeId,
    worker::{FirebatWorker, WorkerRequest, WorkerResponse, WorkerTryRecv},
};
use chrono::Local;
use eframe::egui::{self, Color32};
use fireball::abstract_syntax_tree::Ast;
use rfd::FileDialog;
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
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
    pub(super) last_decompile_selection: Vec<u64>,
    // Floating editor windows
    pub(super) assembly_editor: FloatingEditorWindow,
    pub(super) ir_editor: FloatingEditorWindow,
    pub(super) ast_editor: FloatingEditorWindow,
    // Track selections for highlighting
    pub(super) selected_assembly_row: Option<usize>,
    pub(super) selected_ir_row: Option<usize>,
    pub(super) selected_ast_path: Option<fireball::abstract_syntax_tree::AstNodePath>,
    pub(super) show_ir_comments: bool,
    // --- Opt/Preview pipeline fields ---
    /// Base AST produced by initial decompilation (before any OptNode).
    pub(super) base_ast: Option<Arc<Ast>>,
    /// Base decompile output (assembly/ir/ast) from initial decompilation.
    pub(super) base_output: Option<DecompileResult>,
    /// Queue of OptNode IDs awaiting async optimization.
    pub(super) pending_optimize_queue: VecDeque<NodeId>,
    /// The OptNode currently being processed by the worker.
    pub(super) pending_target_node: Option<NodeId>,
    /// Completed optimization result awaiting graph wiring in shell.rs.
    pub(super) last_optimize_result: Option<(NodeId, OptimizeAstResult)>,
}

impl Default for FirebatState {
    fn default() -> Self {
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
            last_decompile_selection: Vec::new(),
            assembly_editor,
            ir_editor,
            ast_editor,
            selected_assembly_row: None,
            selected_ir_row: None,
            selected_ast_path: None,
            show_ir_comments: false,
            base_ast: None,
            base_output: None,
            pending_optimize_queue: VecDeque::new(),
            pending_target_node: None,
            last_optimize_result: None,
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
                                // Store base AST/output for pipeline
                                self.base_ast = result.ast_object.clone();
                                self.base_output = Some(result.clone());
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
                        WorkerResponse::OptimizeAst(result) => match result {
                            Ok(opt_result) => {
                                self.log(format!(
                                    "OptimizeAst ready: {} lines",
                                    opt_result.ast_lines.len()
                                ));
                                if let Some(target) = self.pending_target_node.take() {
                                    self.last_optimize_result = Some((target, opt_result));
                                }
                            }
                            Err(error) => {
                                self.log(format!("OptimizeAst failed {error}"));
                                self.pending_target_node = None;
                                self.pending_optimize_queue.clear();
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
        self.base_ast = None;
        self.base_output = None;
        self.pending_optimize_queue.clear();
        self.pending_target_node = None;
        self.last_optimize_result = None;
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

    pub(super) fn selected_addresses(&self) -> Vec<u64> {
        self.known_sections
            .iter()
            .filter(|s| s.selected && s.data.analyzed)
            .map(|s| s.data.start_address)
            .collect()
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
            result.data.ast.get(target.row).map(|ast| {
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
        ast: result.ast_object.clone(),
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

use crate::{
    model::{DecompileResult, KnownSection, KnownSectionData, OptimizeAstResult},
    node::NodeId,
    worker::{FirebatWorker, WorkerRequest, WorkerResponse, WorkerTryRecv},
};
use chrono::Local;
use fireball::abstract_syntax_tree::Ast;
use rfd::FileDialog;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::Arc,
};

pub(super) struct FirebatState {
    pub(super) worker: FirebatWorker,
    pub(super) pending_requests: usize,
    pub(super) known_sections: Vec<KnownSection>,
    pub(super) logs: Vec<String>,
    pub(super) log_expanded: bool,
    pub(super) analyze_target_address: String,
    pub(super) last_decompile_selection: Vec<u64>,
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
    pub(super) status_notice: Option<String>,
}

impl Default for FirebatState {
    fn default() -> Self {
        Self {
            worker: FirebatWorker::spawn(),
            pending_requests: 0,
            known_sections: Vec::new(),
            logs: Vec::new(),
            log_expanded: false,
            analyze_target_address: String::new(),
            last_decompile_selection: Vec::new(),
            base_ast: None,
            base_output: None,
            pending_optimize_queue: VecDeque::new(),
            pending_target_node: None,
            last_optimize_result: None,
            status_notice: None,
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

    fn set_status_notice(&mut self, message: impl Into<String>) {
        self.status_notice = Some(message.into());
    }

    pub(super) fn take_status_notice(&mut self) -> Option<String> {
        self.status_notice.take()
    }

    pub(super) fn poll_worker(&mut self) {
        loop {
            match self.worker.try_recv() {
                WorkerTryRecv::Message(response) => {
                    self.pending_requests = self.pending_requests.saturating_sub(1);
                    match response {
                        WorkerResponse::OpenFile(result) => match result {
                            Ok(()) => {
                                self.log("Open success");
                                self.set_status_notice("File opened");
                            }
                            Err(error) => {
                                self.log(format!("Open failed {error}"));
                                self.set_status_notice(format!("Open failed: {error}"));
                            }
                        },
                        WorkerResponse::AnalyzeSection(result) => match result {
                            Ok(sections) => {
                                self.log(format!("Section analyzation success {}", sections.len()));
                                self.merge_known_sections(sections);
                                self.set_status_notice("Section analysis completed");
                            }
                            Err(error) => {
                                self.log(format!("Section analyzation failed {error}"));
                                self.set_status_notice(format!("Section analysis failed: {error}"));
                            }
                        },
                        WorkerResponse::AnalyzeAllSections(result) => match result {
                            Ok(sections) => {
                                self.log(format!("All sections analyzed {}", sections.len()));
                                self.merge_known_sections(sections);
                                self.set_status_notice("Analyze all completed");
                            }
                            Err(error) => {
                                self.log(format!("All sections analyzation failed {error}"));
                                self.set_status_notice(format!("Analyze all failed: {error}"));
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
                            }
                            Err(error) => {
                                self.log(format!("Decompilation failed {error}"));
                                self.set_status_notice(format!("Pipeline failed: {error}"));
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
                                self.set_status_notice(format!("Pipeline failed: {error}"));
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

    fn reset_pipeline_state(&mut self) {
        self.known_sections.clear();
        self.last_decompile_selection.clear();
        self.base_ast = None;
        self.base_output = None;
        self.pending_optimize_queue.clear();
        self.pending_target_node = None;
        self.last_optimize_result = None;
        self.analyze_target_address.clear();
    }

    pub(super) fn clear_loaded_input_session(&mut self) {
        self.reset_pipeline_state();
    }

    pub(super) fn open_file(&mut self) -> Option<PathBuf> {
        let Some(path) = FileDialog::new().pick_file() else {
            self.log("Open canceled");
            return None;
        };

        self.open_file_path(path.clone());
        Some(path)
    }

    pub(super) fn open_file_path(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref().to_path_buf();
        let path_string = path.to_string_lossy().to_string();
        self.reset_pipeline_state();
        self.log(format!("Open fireball with {path_string}"));
        self.queue_request(WorkerRequest::OpenFile(path_string));
    }

    pub(super) fn set_section_selected(&mut self, start_address: u64, selected: bool) {
        if let Some(section) = self
            .known_sections
            .iter_mut()
            .find(|section| section.data.start_address == start_address)
        {
            if section.data.analyzed {
                section.selected = selected;
            }
        }
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
}

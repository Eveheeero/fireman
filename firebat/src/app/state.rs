use crate::{
    core::{build_line_ranges, parse_address},
    model::{DecompileResultView, KnownSection, KnownSectionData},
    worker::{FirebatWorker, WorkerRequest, WorkerResponse, WorkerTryRecv},
};
use chrono::Local;
use eframe::egui::Color32;
use rfd::FileDialog;
use std::collections::HashMap;

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
}

impl Default for FirebatState {
    fn default() -> Self {
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

    fn queue_request(&mut self, request: WorkerRequest) {
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
                            Err(error) => self.log(format!("Section analyzation failed {error}")),
                        },
                        WorkerResponse::AnalyzeAllSections(result) => match result {
                            Ok(sections) => {
                                self.log(format!("All sections analyzed {}", sections.len()));
                                self.merge_known_sections(sections);
                            }
                            Err(error) => {
                                self.log(format!("All sections analyzation failed {error}"))
                            }
                        },
                        WorkerResponse::DecompileSections(result) => match result {
                            Ok(result) => {
                                let mut colors = HashMap::new();
                                let mut section_primary_assembly = HashMap::new();
                                let mut assembly_parent_by_index = HashMap::new();
                                let decompiled_line_ranges = build_line_ranges(&result.decompiled);
                                for assembly in &result.assembly {
                                    colors.insert(
                                        assembly.index,
                                        get_color_for_index(assembly.index),
                                    );
                                    section_primary_assembly
                                        .entry(assembly.parents_start_address)
                                        .or_insert(assembly.index);
                                    assembly_parent_by_index
                                        .insert(assembly.index, assembly.parents_start_address);
                                }
                                self.decompile_result = Some(DecompileResultView {
                                    colors,
                                    section_primary_assembly,
                                    assembly_parent_by_index,
                                    decompiled_line_ranges,
                                    data: result,
                                });
                                self.log("Decompilation success");
                            }
                            Err(error) => self.log(format!("Decompilation failed {error}")),
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
        self.known_sections.retain(|section| {
            !sections
                .iter()
                .any(|new_section| new_section.start_address == section.data.start_address)
        });
        self.known_sections
            .extend(sections.into_iter().map(|section| KnownSection {
                selected: false,
                data: section,
            }));
    }

    pub(super) fn open_file(&mut self) {
        let Some(path) = FileDialog::new().pick_file() else {
            self.log("Open canceled");
            return;
        };

        let path = path.to_string_lossy().to_string();
        self.log(format!("Open fireball with {path}"));
        self.queue_request(WorkerRequest::OpenFile(path));
    }

    pub(super) fn analyze_section_from_address(&mut self, start_address: &str) {
        let trimmed_address = start_address.trim().to_owned();
        let parsed_address = parse_address(&trimmed_address);
        if !trimmed_address.is_empty() && parsed_address.is_err() {
            self.log(format!("Invalid address {start_address}"));
            return;
        }

        if let Ok(parsed_address) = parsed_address {
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
        let all_selected = analyzed_sections.iter().all(|section| section.selected);
        for section in &mut self.known_sections {
            if section.data.analyzed {
                section.selected = !all_selected;
            }
        }
    }

    pub(super) fn decompile_selected(&mut self) {
        let selected = self
            .known_sections
            .iter()
            .filter(|section| section.selected)
            .map(|section| section.data.start_address)
            .collect::<Vec<_>>();

        if selected.is_empty() {
            self.log("No sections selected for decompilation");
            return;
        }

        self.log(format!("Decompiling sections {selected:?}"));
        self.queue_request(WorkerRequest::DecompileSections(selected));
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

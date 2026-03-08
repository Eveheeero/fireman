use eframe::egui::Color32;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct KnownSectionData {
    pub(crate) start_address: u64,
    pub(crate) end_address: Option<u64>,
    pub(crate) analyzed: bool,
}

#[derive(Clone)]
pub(crate) struct KnownSection {
    pub(crate) selected: bool,
    pub(crate) data: KnownSectionData,
}

#[derive(Clone)]
pub(crate) struct Assembly {
    pub(crate) index: usize,
    pub(crate) parents_start_address: u64,
    pub(crate) data: String,
}

#[derive(Clone)]
pub(crate) struct Ir {
    pub(crate) parents_assembly_index: usize,
    pub(crate) data: String,
}

#[derive(Clone)]
pub(crate) struct DecompileResult {
    pub(crate) assembly: Vec<Assembly>,
    pub(crate) ir: Vec<Ir>,
    pub(crate) decompiled: String,
}

#[derive(Clone)]
pub(crate) struct DecompileResultView {
    pub(crate) colors: HashMap<usize, Color32>,
    pub(crate) section_primary_assembly: HashMap<u64, usize>,
    pub(crate) assembly_parent_by_index: HashMap<usize, u64>,
    pub(crate) decompiled_line_ranges: Vec<(usize, usize)>,
    pub(crate) data: DecompileResult,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum EditorLayer {
    Assembly,
    Ir,
    Ast,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct EditorTarget {
    pub(crate) layer: EditorLayer,
    pub(crate) row: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct PatchOperation {
    pub(crate) layer: EditorLayer,
    pub(crate) target: String,
    pub(crate) text: String,
}

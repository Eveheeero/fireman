//! Simple points-to analysis (Steensgaard-style unification).
//!
//! Tracks which abstract locations each pointer-typed register or stack slot
//! may point to, using union-find to maintain proper alias classes.

use crate::{
    core::Block,
    ir::{Register, data::IrData, statements::IrStatement},
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

/// An abstract memory location for alias analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbstractLocation {
    /// A machine register.
    Register(Register),
    /// An SP-relative stack slot at byte offset.
    StackSlot(i64),
    /// A fixed global address.
    Global(u64),
    /// A heap allocation site (identified by call-site ID).
    Heap(u32),
    /// Unknown / top — may alias anything.
    Unknown,
}

/// Union-find cell for Steensgaard unification.
#[derive(Debug, Clone)]
struct UnionFindEntry {
    parent: usize,
    rank: u8,
}

/// Points-to analysis using union-find for proper Steensgaard unification.
#[derive(Debug, Clone)]
pub struct PointsToSet {
    /// Map from AbstractLocation to union-find node ID.
    loc_to_id: HashMap<AbstractLocation, usize>,
    /// Union-find forest.
    entries: Vec<UnionFindEntry>,
    /// Points-to edges: node ID → set of target node IDs.
    edges: Vec<HashSet<usize>>,
}

impl Default for PointsToSet {
    fn default() -> Self {
        Self {
            loc_to_id: HashMap::new(),
            entries: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl PointsToSet {
    fn get_or_create(&mut self, loc: AbstractLocation) -> usize {
        if let Some(&id) = self.loc_to_id.get(&loc) {
            return id;
        }
        let id = self.entries.len();
        self.entries.push(UnionFindEntry {
            parent: id,
            rank: 0,
        });
        self.edges.push(HashSet::new());
        self.loc_to_id.insert(loc, id);
        id
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.entries[x].parent != x {
            self.entries[x].parent = self.entries[self.entries[x].parent].parent;
            x = self.entries[x].parent;
        }
        x
    }

    fn union(&mut self, a: usize, b: usize) -> usize {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return ra;
        }
        // Union by rank
        let (root, child) = if self.entries[ra].rank >= self.entries[rb].rank {
            (ra, rb)
        } else {
            (rb, ra)
        };
        self.entries[child].parent = root;
        if self.entries[ra].rank == self.entries[rb].rank {
            self.entries[root].rank += 1;
        }
        // Merge edges
        let child_edges: HashSet<usize> = self.edges[child].clone();
        self.edges[root].extend(child_edges);
        root
    }

    fn add_edge(&mut self, from: AbstractLocation, to: AbstractLocation) {
        let from_id = self.get_or_create(from);
        let to_id = self.get_or_create(to);
        let from_root = self.find(from_id);
        let to_root = self.find(to_id);
        self.edges[from_root].insert(to_root);
    }

    fn unify(&mut self, a: AbstractLocation, b: AbstractLocation) {
        let a_id = self.get_or_create(a);
        let b_id = self.get_or_create(b);
        self.union(a_id, b_id);
    }

    /// Check if two locations may alias (are in the same equivalence class).
    pub fn may_alias(&mut self, a: &AbstractLocation, b: &AbstractLocation) -> bool {
        if a == b {
            return true;
        }
        let Some(&a_id) = self.loc_to_id.get(a) else {
            return false;
        };
        let Some(&b_id) = self.loc_to_id.get(b) else {
            return false;
        };
        self.find(a_id) == self.find(b_id)
    }

    /// Get the set of locations that a given location may point to.
    pub fn targets_of(&mut self, loc: &AbstractLocation) -> HashSet<AbstractLocation> {
        let Some(&id) = self.loc_to_id.get(loc) else {
            return HashSet::new();
        };
        let root = self.find(id);
        let target_ids: Vec<usize> = self.edges[root].iter().copied().collect();

        // Reverse-map node IDs to locations
        let id_to_loc: HashMap<usize, AbstractLocation> =
            self.loc_to_id.iter().map(|(&loc, &id)| (id, loc)).collect();

        target_ids
            .into_iter()
            .filter_map(|tid| {
                let troot = self.find(tid);
                id_to_loc.get(&troot).copied()
            })
            .collect()
    }

    /// Total number of tracked locations.
    pub fn location_count(&self) -> usize {
        self.loc_to_id.len()
    }

    /// Total number of points-to edges.
    pub fn edge_count(&self) -> usize {
        self.edges.iter().map(|s| s.len()).sum()
    }
}

/// Run points-to analysis over function blocks.
pub fn analyze_points_to(blocks: &[Arc<Block>]) -> PointsToSet {
    let mut pts = PointsToSet::default();
    let mut call_site_id: u32 = 0;

    for block in blocks {
        let ir_block = block.get_ir();
        let Some(ir_block) = ir_block.as_ref() else {
            continue;
        };

        for ir in ir_block.ir() {
            let Some(stmts) = ir.statements else {
                continue;
            };
            for stmt in stmts {
                process_pts_statement(stmt, &mut pts, &mut call_site_id);
            }
        }
    }

    pts
}

fn process_pts_statement(stmt: &IrStatement, pts: &mut PointsToSet, call_site_id: &mut u32) {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            let src = ir_data_to_location(from);
            let dst = ir_data_to_location(to);

            if let (Some(dst_loc), Some(src_loc)) = (dst, src) {
                // dst = src → unify dst and src (Steensgaard)
                pts.unify(dst_loc, src_loc);
            }

            // If source is an address constant, dst points to that address
            if let Some(dst_loc) = ir_data_to_location(to) {
                if let IrData::Constant(addr) = from.as_ref() {
                    pts.add_edge(dst_loc, AbstractLocation::Global(*addr as u64));
                }
            }
        }
        IrStatement::JumpByCall { .. } => {
            *call_site_id += 1;
        }
        IrStatement::Condition {
            true_branch,
            false_branch,
            ..
        } => {
            for s in true_branch.iter() {
                process_pts_statement(s, pts, call_site_id);
            }
            for s in false_branch.iter() {
                process_pts_statement(s, pts, call_site_id);
            }
        }
        _ => {}
    }
}

/// Map IrData to an AbstractLocation if it represents a trackable location.
fn ir_data_to_location(data: &crate::utils::Aos<IrData>) -> Option<AbstractLocation> {
    match data.as_ref() {
        IrData::Register(reg) => Some(AbstractLocation::Register(*reg)),
        IrData::Constant(addr) => Some(AbstractLocation::Global(*addr as u64)),
        _ => None,
    }
}

/// Log points-to analysis results.
pub fn log_points_to_analysis(blocks: &[Arc<Block>]) {
    let pts = analyze_points_to(blocks);
    if pts.edge_count() > 0 {
        debug!(
            "Points-to analysis: {} locations, {} edges",
            pts.location_count(),
            pts.edge_count(),
        );
    }
}

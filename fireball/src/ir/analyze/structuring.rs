//! CFG structuring algorithm — simplified Phoenix-style region recovery.
//!
//! Converts a control flow graph into structured regions (if-then-else,
//! while, do-while, sequence) using dominator/postdominator trees and
//! natural loop information.

use crate::{
    core::Block,
    ir::analyze::{
        ControlFlowGraph, DominatorTree, FunctionControlFlowAnalysis, LoopInfo, NaturalLoop,
        PostDominatorTree, analyze_function_control_flow, infer_entry_block_id,
    },
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
};

/// A first-pass CFG interval discovered without collapsing the graph.
///
/// This stays intentionally conservative: it partitions the current CFG into
/// maximal header-led regions where every non-header member's predecessors are
/// already inside the same interval. Later interval-collapse driven
/// restructuring can build on this artifact.
#[derive(Debug, Clone)]
pub struct CfgInterval {
    pub header_block: usize,
    pub block_ids: Vec<usize>,
    pub exit_blocks: Vec<usize>,
}

type ChunkBiasMap = HashMap<usize, usize>;

impl CfgInterval {
    pub fn block_count(&self) -> usize {
        self.block_ids.len()
    }
}

/// A structured region of code recovered from the CFG.
#[derive(Debug, Clone)]
pub enum StructuredRegion {
    /// Sequential composition of regions.
    Sequence(Vec<StructuredRegion>),
    /// if (head) then_region [else else_region]
    IfThenElse {
        head_block: usize,
        then_region: Box<StructuredRegion>,
        else_region: Option<Box<StructuredRegion>>,
    },
    /// switch (head) { case values: body; ... default: body; }
    Switch {
        head_block: usize,
        cases: Vec<(Vec<i64>, StructuredRegion)>,
        default: Option<Box<StructuredRegion>>,
    },
    /// while (header) { body }
    While {
        header_block: usize,
        body: Box<StructuredRegion>,
    },
    /// do { body } while (latch)
    DoWhile {
        body: Box<StructuredRegion>,
        latch_block: usize,
    },
    /// A single basic block.
    Block(usize),
    /// Irreducible: goto target.
    Goto(usize),
    /// Label for goto target.
    Label(usize),
    /// Break out of loop.
    Break,
    /// Continue to loop header.
    Continue,
}

impl StructuredRegion {
    /// Count total basic blocks referenced.
    pub fn block_count(&self) -> usize {
        match self {
            StructuredRegion::Block(_) => 1,
            StructuredRegion::Sequence(regions) => regions.iter().map(|r| r.block_count()).sum(),
            StructuredRegion::IfThenElse {
                then_region,
                else_region,
                ..
            } => {
                1 + then_region.block_count() + else_region.as_ref().map_or(0, |r| r.block_count())
            }
            StructuredRegion::Switch { cases, default, .. } => {
                1 + cases
                    .iter()
                    .map(|(_, region)| region.block_count())
                    .sum::<usize>()
                    + default.as_ref().map_or(0, |region| region.block_count())
            }
            StructuredRegion::While { body, .. } => 1 + body.block_count(),
            StructuredRegion::DoWhile { body, .. } => body.block_count() + 1,
            StructuredRegion::Goto(_)
            | StructuredRegion::Label(_)
            | StructuredRegion::Break
            | StructuredRegion::Continue => 0,
        }
    }

    /// Count structured constructs (if/while/do-while).
    pub fn construct_count(&self) -> usize {
        match self {
            StructuredRegion::Block(_)
            | StructuredRegion::Goto(_)
            | StructuredRegion::Label(_)
            | StructuredRegion::Break
            | StructuredRegion::Continue => 0,
            StructuredRegion::Sequence(regions) => {
                regions.iter().map(|r| r.construct_count()).sum()
            }
            StructuredRegion::IfThenElse {
                then_region,
                else_region,
                ..
            } => {
                1 + then_region.construct_count()
                    + else_region.as_ref().map_or(0, |r| r.construct_count())
            }
            StructuredRegion::Switch { cases, default, .. } => {
                1 + cases
                    .iter()
                    .map(|(_, region)| region.construct_count())
                    .sum::<usize>()
                    + default
                        .as_ref()
                        .map_or(0, |region| region.construct_count())
            }
            StructuredRegion::While { body, .. } => 1 + body.construct_count(),
            StructuredRegion::DoWhile { body, .. } => 1 + body.construct_count(),
        }
    }

    /// Conservatively count goto-free structured regions that behave like
    /// single-entry/single-exit subgraphs under the current lowering model.
    pub fn sese_region_count(&self) -> usize {
        let nested = match self {
            StructuredRegion::Sequence(regions) => {
                regions.iter().map(|r| r.sese_region_count()).sum()
            }
            StructuredRegion::IfThenElse {
                then_region,
                else_region,
                ..
            } => {
                then_region.sese_region_count()
                    + else_region.as_ref().map_or(0, |r| r.sese_region_count())
            }
            StructuredRegion::Switch { cases, default, .. } => {
                cases
                    .iter()
                    .map(|(_, region)| region.sese_region_count())
                    .sum::<usize>()
                    + default
                        .as_ref()
                        .map_or(0, |region| region.sese_region_count())
            }
            StructuredRegion::While { body, .. } | StructuredRegion::DoWhile { body, .. } => {
                body.sese_region_count()
            }
            StructuredRegion::Block(_)
            | StructuredRegion::Goto(_)
            | StructuredRegion::Label(_)
            | StructuredRegion::Break
            | StructuredRegion::Continue => 0,
        };
        if self.is_sese_region() {
            nested + 1
        } else {
            nested
        }
    }

    /// A conservative SESE classifier: regions stay eligible only when they
    /// are fully structured and contain no goto/label/break/continue escapes.
    pub fn is_sese_region(&self) -> bool {
        match self {
            StructuredRegion::Block(_) => true,
            StructuredRegion::Sequence(regions) => {
                !regions.is_empty() && regions.iter().all(|region| region.is_sese_region())
            }
            StructuredRegion::IfThenElse {
                then_region,
                else_region,
                ..
            } => {
                then_region.is_sese_region()
                    && else_region
                        .as_ref()
                        .is_some_and(|region| region.is_sese_region())
            }
            StructuredRegion::Switch { cases, default, .. } => {
                !cases.is_empty()
                    && cases.iter().all(|(_, region)| region.is_sese_region())
                    && default
                        .as_ref()
                        .is_some_and(|region| region.is_sese_region())
            }
            StructuredRegion::While { body, .. } | StructuredRegion::DoWhile { body, .. } => {
                body.is_sese_region()
            }
            StructuredRegion::Goto(_)
            | StructuredRegion::Label(_)
            | StructuredRegion::Break
            | StructuredRegion::Continue => false,
        }
    }

    /// Count goto/label nodes (irreducible fallbacks).
    pub fn goto_count(&self) -> usize {
        match self {
            StructuredRegion::Goto(_) | StructuredRegion::Label(_) => 1,
            StructuredRegion::Block(_) | StructuredRegion::Break | StructuredRegion::Continue => 0,
            StructuredRegion::Sequence(regions) => regions.iter().map(|r| r.goto_count()).sum(),
            StructuredRegion::IfThenElse {
                then_region,
                else_region,
                ..
            } => then_region.goto_count() + else_region.as_ref().map_or(0, |r| r.goto_count()),
            StructuredRegion::Switch { cases, default, .. } => {
                cases
                    .iter()
                    .map(|(_, region)| region.goto_count())
                    .sum::<usize>()
                    + default.as_ref().map_or(0, |region| region.goto_count())
            }
            StructuredRegion::While { body, .. } => body.goto_count(),
            StructuredRegion::DoWhile { body, .. } => body.goto_count(),
        }
    }

    /// Count explicit labels used to anchor irreducible goto fallbacks.
    pub fn label_count(&self) -> usize {
        match self {
            StructuredRegion::Label(_) => 1,
            StructuredRegion::Goto(_)
            | StructuredRegion::Block(_)
            | StructuredRegion::Break
            | StructuredRegion::Continue => 0,
            StructuredRegion::Sequence(regions) => regions.iter().map(|r| r.label_count()).sum(),
            StructuredRegion::IfThenElse {
                then_region,
                else_region,
                ..
            } => then_region.label_count() + else_region.as_ref().map_or(0, |r| r.label_count()),
            StructuredRegion::Switch { cases, default, .. } => {
                cases
                    .iter()
                    .map(|(_, region)| region.label_count())
                    .sum::<usize>()
                    + default.as_ref().map_or(0, |region| region.label_count())
            }
            StructuredRegion::While { body, .. } | StructuredRegion::DoWhile { body, .. } => {
                body.label_count()
            }
        }
    }

    /// Count goto targets that still have no matching label node after
    /// irreducible fallback labeling.
    pub fn unresolved_goto_target_count(&self) -> usize {
        let mut goto_targets = HashSet::new();
        self.collect_goto_targets(&mut goto_targets);

        let mut label_targets = HashSet::new();
        self.collect_label_targets(&mut label_targets);

        goto_targets.difference(&label_targets).count()
    }

    /// Insert labels before blocks that are the target of fallback gotos.
    ///
    /// This is a conservative relooper-style step: it does not synthesize a
    /// dispatcher or rewrite the CFG, but it ensures shared-tail fallbacks lower
    /// into coherent goto/label pairs instead of gotos that reference a
    /// synthetic label with no definition.
    pub fn with_relooper_labels(self) -> Self {
        let mut goto_targets = HashSet::new();
        self.collect_goto_targets(&mut goto_targets);
        self.insert_labels_for_targets(&goto_targets)
    }

    /// Reorder simple sibling sequences into a more source-like order when
    /// dominator/postdominator relations clearly indicate the current order is
    /// backwards. This stays fail-closed on irreducible fallback regions.
    pub fn with_source_like_order(
        self,
        dominator_tree: &DominatorTree,
        post_dominator_tree: &PostDominatorTree,
    ) -> Self {
        match self {
            StructuredRegion::Sequence(regions) => {
                let mut reordered = regions
                    .into_iter()
                    .map(|region| {
                        region.with_source_like_order(dominator_tree, post_dominator_tree)
                    })
                    .collect::<Vec<_>>();
                reorder_sequence_regions(&mut reordered, dominator_tree, post_dominator_tree);
                StructuredRegion::Sequence(reordered)
            }
            StructuredRegion::IfThenElse {
                head_block,
                then_region,
                else_region,
            } => StructuredRegion::IfThenElse {
                head_block,
                then_region: Box::new(
                    then_region.with_source_like_order(dominator_tree, post_dominator_tree),
                ),
                else_region: else_region.map(|region| {
                    Box::new(region.with_source_like_order(dominator_tree, post_dominator_tree))
                }),
            },
            StructuredRegion::Switch {
                head_block,
                cases,
                default,
            } => StructuredRegion::Switch {
                head_block,
                cases: cases
                    .into_iter()
                    .map(|(labels, region)| {
                        (
                            labels,
                            region.with_source_like_order(dominator_tree, post_dominator_tree),
                        )
                    })
                    .collect(),
                default: default.map(|region| {
                    Box::new(region.with_source_like_order(dominator_tree, post_dominator_tree))
                }),
            },
            StructuredRegion::While { header_block, body } => StructuredRegion::While {
                header_block,
                body: Box::new(body.with_source_like_order(dominator_tree, post_dominator_tree)),
            },
            StructuredRegion::DoWhile { body, latch_block } => StructuredRegion::DoWhile {
                body: Box::new(body.with_source_like_order(dominator_tree, post_dominator_tree)),
                latch_block,
            },
            region => region,
        }
    }

    fn collect_goto_targets(&self, targets: &mut HashSet<usize>) {
        match self {
            StructuredRegion::Goto(block_id) => {
                targets.insert(*block_id);
            }
            StructuredRegion::Sequence(regions) => {
                for region in regions {
                    region.collect_goto_targets(targets);
                }
            }
            StructuredRegion::IfThenElse {
                then_region,
                else_region,
                ..
            } => {
                then_region.collect_goto_targets(targets);
                if let Some(region) = else_region.as_ref() {
                    region.collect_goto_targets(targets);
                }
            }
            StructuredRegion::Switch { cases, default, .. } => {
                for (_, region) in cases {
                    region.collect_goto_targets(targets);
                }
                if let Some(region) = default.as_ref() {
                    region.collect_goto_targets(targets);
                }
            }
            StructuredRegion::While { body, .. } | StructuredRegion::DoWhile { body, .. } => {
                body.collect_goto_targets(targets);
            }
            StructuredRegion::Block(_)
            | StructuredRegion::Label(_)
            | StructuredRegion::Break
            | StructuredRegion::Continue => {}
        }
    }

    fn collect_label_targets(&self, targets: &mut HashSet<usize>) {
        match self {
            StructuredRegion::Label(block_id) => {
                targets.insert(*block_id);
            }
            StructuredRegion::Sequence(regions) => {
                for region in regions {
                    region.collect_label_targets(targets);
                }
            }
            StructuredRegion::IfThenElse {
                then_region,
                else_region,
                ..
            } => {
                then_region.collect_label_targets(targets);
                if let Some(region) = else_region.as_ref() {
                    region.collect_label_targets(targets);
                }
            }
            StructuredRegion::Switch { cases, default, .. } => {
                for (_, region) in cases {
                    region.collect_label_targets(targets);
                }
                if let Some(region) = default.as_ref() {
                    region.collect_label_targets(targets);
                }
            }
            StructuredRegion::While { body, .. } | StructuredRegion::DoWhile { body, .. } => {
                body.collect_label_targets(targets);
            }
            StructuredRegion::Goto(_)
            | StructuredRegion::Block(_)
            | StructuredRegion::Break
            | StructuredRegion::Continue => {}
        }
    }

    fn insert_labels_for_targets(self, goto_targets: &HashSet<usize>) -> Self {
        match self {
            StructuredRegion::Block(block_id) if goto_targets.contains(&block_id) => {
                StructuredRegion::Sequence(vec![
                    StructuredRegion::Label(block_id),
                    StructuredRegion::Block(block_id),
                ])
            }
            StructuredRegion::Sequence(regions) => {
                let mut rewritten = Vec::with_capacity(regions.len());
                for region in regions {
                    match region.insert_labels_for_targets(goto_targets) {
                        StructuredRegion::Sequence(nested) => rewritten.extend(nested),
                        region => rewritten.push(region),
                    }
                }
                StructuredRegion::Sequence(rewritten)
            }
            StructuredRegion::IfThenElse {
                head_block,
                then_region,
                else_region,
            } => StructuredRegion::IfThenElse {
                head_block,
                then_region: Box::new(then_region.insert_labels_for_targets(goto_targets)),
                else_region: else_region
                    .map(|region| Box::new(region.insert_labels_for_targets(goto_targets))),
            },
            StructuredRegion::Switch {
                head_block,
                cases,
                default,
            } => StructuredRegion::Switch {
                head_block,
                cases: cases
                    .into_iter()
                    .map(|(labels, region)| {
                        (labels, region.insert_labels_for_targets(goto_targets))
                    })
                    .collect(),
                default: default
                    .map(|region| Box::new(region.insert_labels_for_targets(goto_targets))),
            },
            StructuredRegion::While { header_block, body } => StructuredRegion::While {
                header_block,
                body: Box::new(body.insert_labels_for_targets(goto_targets)),
            },
            StructuredRegion::DoWhile { body, latch_block } => StructuredRegion::DoWhile {
                body: Box::new(body.insert_labels_for_targets(goto_targets)),
                latch_block,
            },
            region => region,
        }
    }
}

fn reorder_sequence_regions(
    regions: &mut [StructuredRegion],
    dominator_tree: &DominatorTree,
    post_dominator_tree: &PostDominatorTree,
) {
    if regions.len() < 2 {
        return;
    }

    for _ in 0..regions.len() {
        let mut changed = false;
        for idx in 0..regions.len() - 1 {
            if should_swap_source_like_regions(
                &regions[idx],
                &regions[idx + 1],
                dominator_tree,
                post_dominator_tree,
            ) {
                regions.swap(idx, idx + 1);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

fn should_swap_source_like_regions(
    left: &StructuredRegion,
    right: &StructuredRegion,
    dominator_tree: &DominatorTree,
    post_dominator_tree: &PostDominatorTree,
) -> bool {
    if !left.is_sese_region()
        || !right.is_sese_region()
        || left.goto_count() > 0
        || left.label_count() > 0
        || right.goto_count() > 0
        || right.label_count() > 0
    {
        return false;
    }

    let Some(left_anchor) = region_anchor_block(left) else {
        return false;
    };
    let Some(right_anchor) = region_anchor_block(right) else {
        return false;
    };
    if left_anchor == right_anchor {
        return false;
    }

    dominator_tree.dominates(right_anchor, left_anchor)
        || post_dominator_tree.post_dominates(left_anchor, right_anchor)
}

fn region_anchor_block(region: &StructuredRegion) -> Option<usize> {
    match region {
        StructuredRegion::Block(block_id)
        | StructuredRegion::Label(block_id)
        | StructuredRegion::Goto(block_id) => Some(*block_id),
        StructuredRegion::IfThenElse { head_block, .. }
        | StructuredRegion::Switch { head_block, .. }
        | StructuredRegion::While {
            header_block: head_block,
            ..
        } => Some(*head_block),
        StructuredRegion::DoWhile { latch_block, .. } => Some(*latch_block),
        StructuredRegion::Sequence(regions) => regions.first().and_then(region_anchor_block),
        StructuredRegion::Break | StructuredRegion::Continue => None,
    }
}

/// Structure a function's CFG into nested regions.
pub fn structure_function(
    blocks: &[Arc<Block>],
    analysis: &FunctionControlFlowAnalysis,
) -> StructuredRegion {
    let cfg = analysis.cfg();
    let dom_tree = analysis.dominators();
    let postdom_tree = analysis.postdominators();
    let loops = analysis.loops();
    let chunk_bias = detect_address_gap_chunk_bias(cfg, blocks, 4096);

    // Get blocks in reverse postorder from the current CFG shape.
    let rpo = reverse_postorder(cfg, chunk_bias.as_ref());

    // Identify loop headers
    let loop_headers: HashSet<usize> = loops.loops().iter().map(|l| l.header_id).collect();

    // Processed blocks tracker
    let mut processed: HashSet<usize> = HashSet::new();

    structure_region(
        &rpo,
        cfg,
        dom_tree,
        postdom_tree,
        loops,
        &loop_headers,
        chunk_bias.as_ref(),
        &mut processed,
    )
    .with_source_like_order(dom_tree, postdom_tree)
    .with_relooper_labels()
}

/// Discover a conservative first-pass interval partition for the current CFG.
///
/// This does not collapse the CFG recursively yet; it only exposes the
/// interval headers/members/exits so later work can switch the structurer to an
/// interval-driven algorithm without recomputing the basic partition.
pub fn discover_intervals(cfg: &ControlFlowGraph) -> Vec<CfgInterval> {
    let mut remaining: HashSet<usize> = cfg.block_ids().iter().copied().collect();
    let mut pending_headers = VecDeque::from([cfg.entry_block_id()]);
    let mut queued_headers = HashSet::from([cfg.entry_block_id()]);
    let mut intervals = Vec::new();

    while !remaining.is_empty() {
        let header = next_interval_header(&mut pending_headers, &mut queued_headers, &remaining)
            .unwrap_or_else(|| {
                // Disconnected or currently unqueued residue falls back to the
                // next smallest block id so the partition stays total.
                *remaining
                    .iter()
                    .min()
                    .expect("remaining interval header set should not be empty")
            });
        let interval = build_interval(cfg, header, &remaining);

        for &block_id in &interval.block_ids {
            remaining.remove(&block_id);
        }

        for &exit_block in &interval.exit_blocks {
            if remaining.contains(&exit_block) && queued_headers.insert(exit_block) {
                pending_headers.push_back(exit_block);
            }
        }

        intervals.push(interval);
    }

    intervals
}

fn structure_region(
    rpo: &[usize],
    cfg: &crate::ir::analyze::ControlFlowGraph,
    dom_tree: &DominatorTree,
    postdom_tree: &PostDominatorTree,
    loops: &LoopInfo,
    loop_headers: &HashSet<usize>,
    chunk_bias: Option<&ChunkBiasMap>,
    processed: &mut HashSet<usize>,
) -> StructuredRegion {
    let mut sequence: Vec<StructuredRegion> = Vec::new();

    for &block_id in rpo {
        if processed.contains(&block_id) {
            continue;
        }

        if loop_headers.contains(&block_id) {
            if let Some(natural_loop) = loops.loop_for_header(block_id) {
                if natural_loop.header_id == block_id {
                    let region = structure_loop(
                        block_id,
                        natural_loop,
                        cfg,
                        dom_tree,
                        postdom_tree,
                        loops,
                        loop_headers,
                        chunk_bias,
                        processed,
                    );
                    sequence.push(region);
                    continue;
                }
            }
        }

        // Check if this is an if-then-else head
        let successors = cfg.successors_of(block_id);
        if successors.len() == 2 && !processed.contains(&block_id) {
            processed.insert(block_id);
            let then_id = successors[0];
            let else_id = successors[1];

            // Collect blocks dominated by the head that are reachable
            // exclusively through then_id or else_id (up to the merge point)
            let then_region = build_branch_region(
                then_id,
                block_id,
                rpo,
                cfg,
                dom_tree,
                postdom_tree,
                loops,
                loop_headers,
                chunk_bias,
                processed,
            );
            let else_region = if else_id != then_id {
                Some(Box::new(build_branch_region(
                    else_id,
                    block_id,
                    rpo,
                    cfg,
                    dom_tree,
                    postdom_tree,
                    loops,
                    loop_headers,
                    chunk_bias,
                    processed,
                )))
            } else {
                None
            };

            sequence.push(StructuredRegion::IfThenElse {
                head_block: block_id,
                then_region: Box::new(then_region),
                else_region,
            });
        } else {
            // Simple block
            processed.insert(block_id);
            sequence.push(StructuredRegion::Block(block_id));
        }
    }

    if sequence.len() == 1 {
        sequence.pop().unwrap()
    } else {
        StructuredRegion::Sequence(sequence)
    }
}

/// Build a structured region for a branch target, recursively collecting
/// all blocks dominated by the branch head that haven't been processed.
fn build_branch_region(
    entry: usize,
    head: usize,
    rpo: &[usize],
    cfg: &crate::ir::analyze::ControlFlowGraph,
    dom_tree: &DominatorTree,
    postdom_tree: &PostDominatorTree,
    loops: &LoopInfo,
    loop_headers: &HashSet<usize>,
    chunk_bias: Option<&ChunkBiasMap>,
    processed: &mut HashSet<usize>,
) -> StructuredRegion {
    if processed.contains(&entry) {
        return StructuredRegion::Goto(entry);
    }

    // Collect blocks in this branch: blocks dominated by `head` that are
    // reachable from `entry` without going through `head` again
    let mut branch_rpo: Vec<usize> = Vec::new();
    for &bid in rpo {
        if processed.contains(&bid) {
            continue;
        }
        // A block belongs to this branch if it's dominated by head
        // (and thus part of the if-then-else structure)
        if bid == entry || dom_tree.dominates(head, bid) {
            // But don't include the merge point (blocks with predecessors
            // from both branches). Check if all predecessors are in our set
            // or are the head itself.
            let preds = cfg.predecessors_of(bid);
            let belongs = bid == entry
                || preds
                    .iter()
                    .all(|&p| p == head || branch_rpo.contains(&p) || processed.contains(&p));
            if belongs {
                branch_rpo.push(bid);
            }
        }
    }

    if branch_rpo.is_empty() {
        return StructuredRegion::Goto(entry);
    }

    // Recursively structure this sub-region
    structure_region(
        &branch_rpo,
        cfg,
        dom_tree,
        postdom_tree,
        loops,
        loop_headers,
        chunk_bias,
        processed,
    )
}

fn structure_loop(
    header: usize,
    natural_loop: &NaturalLoop,
    cfg: &crate::ir::analyze::ControlFlowGraph,
    dom_tree: &DominatorTree,
    postdom_tree: &PostDominatorTree,
    loops: &LoopInfo,
    loop_headers: &HashSet<usize>,
    chunk_bias: Option<&ChunkBiasMap>,
    processed: &mut HashSet<usize>,
) -> StructuredRegion {
    let loop_blocks = &natural_loop.body_ids;
    let latch = natural_loop
        .latch_ids
        .iter()
        .copied()
        .find(|&latch_id| latch_id != header && cfg.successors_of(latch_id).contains(&header))
        .or_else(|| natural_loop.latch_ids.first().copied())
        .unwrap_or(header);

    // Classify: if the latch has a conditional back-edge to header,
    // it's a do-while; otherwise it's a while loop
    let latch_succs = cfg.successors_of(latch);
    let is_do_while = latch != header && latch_succs.contains(&header);

    // The control block is excluded from the body:
    // - while: header is the condition check, excluded from body
    // - do-while: latch is the condition check, excluded from body
    let excluded = if is_do_while { latch } else { header };

    // Build body RPO: all loop blocks except the excluded control block
    let rpo = reverse_postorder(cfg, chunk_bias);
    let body_rpo: Vec<usize> = rpo
        .iter()
        .copied()
        .filter(|&b| loop_blocks.contains(&b) && b != excluded)
        .collect();

    // Use a local processed set for the body, seeded with both the excluded
    // control block and the header to prevent re-entering the current loop.
    // Without inserting the header, do-while loops (where excluded == latch)
    // would see the header in body_rpo and re-enter structure_loop infinitely.
    let mut body_processed: HashSet<usize> = HashSet::new();
    body_processed.insert(excluded);
    body_processed.insert(header);

    let body = if body_rpo.is_empty() {
        StructuredRegion::Block(excluded)
    } else {
        structure_region(
            &body_rpo,
            cfg,
            dom_tree,
            postdom_tree,
            loops,
            loop_headers,
            chunk_bias,
            &mut body_processed,
        )
    };

    // Mark all loop blocks as processed in the outer set
    for &bid in loop_blocks {
        processed.insert(bid);
    }

    if is_do_while {
        StructuredRegion::DoWhile {
            body: Box::new(body),
            latch_block: latch,
        }
    } else {
        StructuredRegion::While {
            header_block: header,
            body: Box::new(body),
        }
    }
}

/// Run CFG structuring and log results.
pub fn log_structuring(blocks: &[Arc<Block>]) {
    let Some(entry_block_id) = infer_entry_block_id(blocks) else {
        return;
    };

    let analysis = analyze_function_control_flow(blocks, entry_block_id);
    let structured = structure_function(blocks, &analysis);
    let intervals = discover_intervals(analysis.cfg());
    let multi_block_intervals = intervals
        .iter()
        .filter(|interval| interval.block_count() > 1)
        .count();
    let largest_interval = intervals
        .iter()
        .map(CfgInterval::block_count)
        .max()
        .unwrap_or(0);

    debug!(
        "CFG structuring: {} blocks, {} constructs, {} SESE regions, {} intervals ({} multi-block, max {} blocks), {} gotos, {} labels, {} unresolved goto targets",
        structured.block_count(),
        structured.construct_count(),
        structured.sese_region_count(),
        intervals.len(),
        multi_block_intervals,
        largest_interval,
        structured.goto_count(),
        structured.label_count(),
        structured.unresolved_goto_target_count(),
    );
}

fn next_interval_header(
    pending_headers: &mut VecDeque<usize>,
    queued_headers: &mut HashSet<usize>,
    remaining: &HashSet<usize>,
) -> Option<usize> {
    while let Some(header) = pending_headers.pop_front() {
        queued_headers.remove(&header);
        if remaining.contains(&header) {
            return Some(header);
        }
    }

    None
}

fn build_interval(
    cfg: &ControlFlowGraph,
    header_block: usize,
    remaining: &HashSet<usize>,
) -> CfgInterval {
    let mut members = HashSet::from([header_block]);
    let mut changed = true;

    while changed {
        changed = false;

        for &candidate in cfg.block_ids() {
            if candidate == header_block
                || !remaining.contains(&candidate)
                || members.contains(&candidate)
            {
                continue;
            }

            let predecessors = cfg.predecessors_of(candidate);
            if predecessors.is_empty() {
                continue;
            }

            if predecessors.iter().all(|pred| members.contains(pred)) {
                members.insert(candidate);
                changed = true;
            }
        }
    }

    let block_ids = cfg
        .block_ids()
        .iter()
        .copied()
        .filter(|block_id| members.contains(block_id))
        .collect::<Vec<_>>();
    let mut exit_blocks = block_ids
        .iter()
        .flat_map(|block_id| cfg.successors_of(*block_id).iter().copied())
        .filter(|successor_id| !members.contains(successor_id) && remaining.contains(successor_id))
        .collect::<Vec<_>>();
    exit_blocks.sort_unstable();
    exit_blocks.dedup();

    CfgInterval {
        header_block,
        block_ids,
        exit_blocks,
    }
}

fn detect_address_gap_chunk_bias(
    cfg: &ControlFlowGraph,
    blocks: &[Arc<Block>],
    gap_threshold: u64,
) -> Option<ChunkBiasMap> {
    let chunks = cfg.detect_address_gap_chunks(blocks, gap_threshold);
    if chunks.len() <= 1 {
        return None;
    }

    let mut chunk_bias = HashMap::new();
    for (chunk_index, chunk) in chunks.into_iter().enumerate() {
        for block_id in chunk {
            chunk_bias.insert(block_id, chunk_index);
        }
    }
    Some(chunk_bias)
}

fn reverse_postorder(
    cfg: &crate::ir::analyze::ControlFlowGraph,
    chunk_bias: Option<&ChunkBiasMap>,
) -> Vec<usize> {
    fn dfs(
        block_id: usize,
        cfg: &crate::ir::analyze::ControlFlowGraph,
        chunk_bias: Option<&ChunkBiasMap>,
        visited: &mut HashSet<usize>,
        postorder: &mut Vec<usize>,
    ) {
        if !visited.insert(block_id) {
            return;
        }

        let successors = ordered_successors(block_id, cfg, chunk_bias);
        for successor_id in successors {
            dfs(successor_id, cfg, chunk_bias, visited, postorder);
        }

        postorder.push(block_id);
    }

    let mut visited = HashSet::new();
    let mut postorder = Vec::new();
    dfs(
        cfg.entry_block_id(),
        cfg,
        chunk_bias,
        &mut visited,
        &mut postorder,
    );

    let mut remaining = cfg
        .block_ids()
        .iter()
        .copied()
        .filter(|block_id| !visited.contains(block_id))
        .collect::<VecDeque<_>>();
    while let Some(block_id) = remaining.pop_front() {
        dfs(block_id, cfg, chunk_bias, &mut visited, &mut postorder);
    }

    postorder.reverse();
    postorder
}

fn ordered_successors(
    block_id: usize,
    cfg: &crate::ir::analyze::ControlFlowGraph,
    chunk_bias: Option<&ChunkBiasMap>,
) -> Vec<usize> {
    let mut successors = cfg.successors_of(block_id).to_vec();
    let Some(chunk_bias) = chunk_bias else {
        return successors;
    };
    let source_chunk = chunk_bias.get(&block_id).copied();
    successors.sort_by_key(|successor_id| {
        let successor_chunk = chunk_bias.get(successor_id).copied();
        let cross_chunk_penalty = match (source_chunk, successor_chunk) {
            (Some(source_chunk), Some(successor_chunk)) if source_chunk == successor_chunk => 0,
            (Some(_), Some(_)) => 1,
            _ => 2,
        };
        (
            cross_chunk_penalty,
            successor_chunk.unwrap_or(usize::MAX),
            *successor_id,
        )
    });
    successors
}

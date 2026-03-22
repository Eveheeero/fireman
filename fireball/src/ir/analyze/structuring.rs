//! CFG structuring algorithm — simplified Phoenix-style region recovery.
//!
//! Converts a control flow graph into structured regions (if-then-else,
//! while, do-while, sequence) using dominator/postdominator trees and
//! natural loop information.

use crate::{
    core::Block,
    ir::analyze::{
        ControlFlowGraph, DominatorTree, FunctionControlFlowAnalysis, LoopInfo,
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

    structure_region_iterative(
        rpo,
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

/// Work items for the iterative structuring algorithm.
///
/// Replaces the mutually recursive `structure_region`, `build_branch_region`,
/// and `structure_loop` functions with an explicit work stack to avoid stack
/// overflow on large/irreducible CFGs.
enum Work {
    /// Process a region (replaces `structure_region`).
    /// Each block in `rpo` is examined: loops become `ProcessLoop`, branches
    /// become `ProcessBranch`, simple blocks become `Block` leaves.
    ProcessRegion {
        rpo: Vec<usize>,
    },
    /// Process a loop body (replaces `structure_loop`).
    ProcessLoop {
        header: usize,
        loop_body_ids: Vec<usize>,
        latch_ids: Vec<usize>,
    },
    /// Process a branch arm (replaces `build_branch_region`).
    ProcessBranch {
        entry: usize,
        head: usize,
        parent_rpo: Vec<usize>,
    },
    /// Assemble N child results into a `Sequence`.
    AssembleSequence(usize),
    /// Assemble an `IfThenElse` from children on the result stack.
    AssembleIfThenElse {
        head_block: usize,
        has_else: bool,
    },
    /// Assemble a `While` from the body on the result stack.
    AssembleWhile {
        header_block: usize,
    },
    /// Assemble a `DoWhile` from the body on the result stack.
    AssembleDoWhile {
        latch_block: usize,
    },
}

fn structure_region_iterative(
    initial_rpo: Vec<usize>,
    cfg: &crate::ir::analyze::ControlFlowGraph,
    dom_tree: &DominatorTree,
    _postdom_tree: &PostDominatorTree,
    loops: &LoopInfo,
    loop_headers: &HashSet<usize>,
    chunk_bias: Option<&ChunkBiasMap>,
    processed: &mut HashSet<usize>,
) -> StructuredRegion {
    let mut work_stack: Vec<Work> = Vec::new();
    let mut result_stack: Vec<StructuredRegion> = Vec::new();

    // Guard set: loop headers currently being structured, prevents re-entry.
    let mut active_loops: HashSet<usize> = HashSet::new();

    work_stack.push(Work::ProcessRegion {
        rpo: initial_rpo,
    });

    while let Some(work) = work_stack.pop() {
        match work {
            Work::ProcessRegion { rpo } => {
                // Scan the RPO block list and emit work items for each block.
                // Because the work stack is LIFO, we push items in reverse
                // order so the first block is processed first.
                let mut child_count = 0usize;

                // Collect the work items for this region, then push in reverse.
                let mut region_work: Vec<Work> = Vec::new();

                for &block_id in &rpo {
                    if processed.contains(&block_id) {
                        continue;
                    }

                    if loop_headers.contains(&block_id) && !active_loops.contains(&block_id) {
                        if let Some(natural_loop) = loops.loop_for_header(block_id) {
                            if natural_loop.header_id == block_id {
                                region_work.push(Work::ProcessLoop {
                                    header: block_id,
                                    loop_body_ids: natural_loop.body_ids.clone(),
                                    latch_ids: natural_loop.latch_ids.clone(),
                                });
                                child_count += 1;
                                continue;
                            }
                        }
                    }

                    let successors = cfg.successors_of(block_id);
                    if successors.len() == 2 && !processed.contains(&block_id) {
                        processed.insert(block_id);
                        let then_id = successors[0];
                        let else_id = successors[1];
                        let has_else = else_id != then_id;

                        // Push assembly first (will be processed after children).
                        let assemble = Work::AssembleIfThenElse {
                            head_block: block_id,
                            has_else,
                        };

                        // Branch arms: then first, else second (if present).
                        // We collect them here and push in the right order below.
                        let mut branch_items = Vec::new();
                        branch_items.push(Work::ProcessBranch {
                            entry: then_id,
                            head: block_id,
                            parent_rpo: rpo.clone(),
                        });
                        if has_else {
                            branch_items.push(Work::ProcessBranch {
                                entry: else_id,
                                head: block_id,
                                parent_rpo: rpo.clone(),
                            });
                        }

                        // These need to be pushed as a unit: assemble after branches.
                        // We wrap them in a sub-sequence within region_work.
                        // Push order: assemble (last), else (if any), then (first).
                        // Since we'll reverse region_work, we push: assemble, branches...
                        // Actually, region_work items will be pushed in reverse onto work_stack,
                        // so within region_work the order should be:
                        //   [assemble, else_branch?, then_branch]
                        // After reverse push: then_branch is on top, processed first.
                        region_work.push(assemble);
                        if has_else {
                            region_work.push(branch_items.pop().unwrap()); // else
                        }
                        region_work.push(branch_items.pop().unwrap()); // then

                        child_count += 1;
                    } else {
                        processed.insert(block_id);
                        result_stack.push(StructuredRegion::Block(block_id));
                        child_count += 1;
                    }
                }

                // Push the assembly work for combining children into a Sequence.
                if child_count != 1 {
                    work_stack.push(Work::AssembleSequence(child_count));
                }
                // Push region_work items in reverse so first item runs first.
                for item in region_work.into_iter().rev() {
                    work_stack.push(item);
                }
            }

            Work::ProcessLoop {
                header,
                loop_body_ids,
                latch_ids,
            } => {
                // Guard against re-entry for overlapping/irreducible loops.
                if active_loops.contains(&header) || processed.contains(&header) {
                    result_stack.push(StructuredRegion::Goto(header));
                    continue;
                }

                let latch = latch_ids
                    .iter()
                    .copied()
                    .find(|&latch_id| {
                        latch_id != header
                            && cfg.successors_of(latch_id).contains(&header)
                    })
                    .or_else(|| latch_ids.first().copied())
                    .unwrap_or(header);

                let latch_succs = cfg.successors_of(latch);
                let is_do_while = latch != header && latch_succs.contains(&header);

                let excluded = if is_do_while { latch } else { header };

                // Build body RPO
                let full_rpo = reverse_postorder(cfg, chunk_bias);
                let body_rpo: Vec<usize> = full_rpo
                    .iter()
                    .copied()
                    .filter(|&b| loop_body_ids.contains(&b) && b != excluded)
                    .collect();

                // Mark the loop header and excluded block as processed so the
                // body structuring won't re-enter them.
                processed.insert(excluded);
                processed.insert(header);
                active_loops.insert(header);

                // Mark all loop blocks as processed in the outer set BEFORE
                // structuring the body so nested loops/branches see them.
                for &bid in &loop_body_ids {
                    processed.insert(bid);
                }

                if body_rpo.is_empty() {
                    result_stack.push(StructuredRegion::Block(excluded));
                    // Assemble directly.
                    if is_do_while {
                        let body = result_stack.pop().unwrap();
                        result_stack.push(StructuredRegion::DoWhile {
                            body: Box::new(body),
                            latch_block: latch,
                        });
                    } else {
                        let body = result_stack.pop().unwrap();
                        result_stack.push(StructuredRegion::While {
                            header_block: header,
                            body: Box::new(body),
                        });
                    }
                    active_loops.remove(&header);
                } else {
                    // Push assembly, then the body region processing.
                    if is_do_while {
                        work_stack.push(Work::AssembleDoWhile {
                            latch_block: latch,
                        });
                    } else {
                        work_stack.push(Work::AssembleWhile {
                            header_block: header,
                        });
                    }
                    work_stack.push(Work::ProcessRegion { rpo: body_rpo });
                }
            }

            Work::ProcessBranch {
                entry,
                head,
                parent_rpo,
            } => {
                if processed.contains(&entry) {
                    result_stack.push(StructuredRegion::Goto(entry));
                    continue;
                }

                // Collect blocks in this branch: dominated by `head`, reachable
                // from `entry` without going through `head` again.
                let mut branch_rpo: Vec<usize> = Vec::new();
                for &bid in &parent_rpo {
                    if processed.contains(&bid) {
                        continue;
                    }
                    if bid == entry || dom_tree.dominates(head, bid) {
                        let preds = cfg.predecessors_of(bid);
                        let belongs = bid == entry
                            || preds.iter().all(|&p| {
                                p == head
                                    || branch_rpo.contains(&p)
                                    || processed.contains(&p)
                            });
                        if belongs {
                            branch_rpo.push(bid);
                        }
                    }
                }

                if branch_rpo.is_empty() {
                    result_stack.push(StructuredRegion::Goto(entry));
                } else {
                    work_stack.push(Work::ProcessRegion { rpo: branch_rpo });
                }
            }

            Work::AssembleSequence(child_count) => {
                if child_count == 0 {
                    result_stack.push(StructuredRegion::Sequence(Vec::new()));
                } else {
                    let start = result_stack.len().saturating_sub(child_count);
                    let children: Vec<StructuredRegion> =
                        result_stack.drain(start..).collect();
                    if children.len() == 1 {
                        result_stack.extend(children);
                    } else {
                        result_stack.push(StructuredRegion::Sequence(children));
                    }
                }
            }

            Work::AssembleIfThenElse {
                head_block,
                has_else,
            } => {
                let else_region = if has_else {
                    Some(Box::new(result_stack.pop().unwrap_or(StructuredRegion::Sequence(Vec::new()))))
                } else {
                    None
                };
                let then_region =
                    result_stack.pop().unwrap_or(StructuredRegion::Sequence(Vec::new()));
                result_stack.push(StructuredRegion::IfThenElse {
                    head_block,
                    then_region: Box::new(then_region),
                    else_region,
                });
            }

            Work::AssembleWhile { header_block } => {
                let body =
                    result_stack.pop().unwrap_or(StructuredRegion::Sequence(Vec::new()));
                active_loops.remove(&header_block);
                result_stack.push(StructuredRegion::While {
                    header_block,
                    body: Box::new(body),
                });
            }

            Work::AssembleDoWhile { latch_block } => {
                let body =
                    result_stack.pop().unwrap_or(StructuredRegion::Sequence(Vec::new()));
                // Find the header for this latch to clear the active_loops guard.
                // The latch's successor that is in active_loops is the header.
                let latch_succs = cfg.successors_of(latch_block);
                for &succ in latch_succs {
                    active_loops.remove(&succ);
                }
                result_stack.push(StructuredRegion::DoWhile {
                    body: Box::new(body),
                    latch_block,
                });
            }
        }
    }

    result_stack.pop().unwrap_or(StructuredRegion::Sequence(Vec::new()))
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

//! CFG structuring algorithm — simplified Phoenix-style region recovery.
//!
//! Converts a control flow graph into structured regions (if-then-else,
//! while, do-while, sequence) using dominator/postdominator trees and
//! natural loop information.

use crate::{
    core::Block,
    ir::analyze::{
        DominatorTree, FunctionControlFlowAnalysis, LoopInfo, NaturalLoop,
        analyze_function_control_flow, infer_entry_block_id,
    },
    prelude::*,
};
use std::{
    collections::{HashSet, VecDeque},
    sync::Arc,
};

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
}

/// Structure a function's CFG into nested regions.
pub fn structure_function(
    _blocks: &[Arc<Block>],
    analysis: &FunctionControlFlowAnalysis,
) -> StructuredRegion {
    let cfg = analysis.cfg();
    let dom_tree = analysis.dominators();
    let loops = analysis.loops();

    // Get blocks in reverse postorder from the current CFG shape.
    let rpo = reverse_postorder(cfg);

    // Identify loop headers
    let loop_headers: HashSet<usize> = loops.loops().iter().map(|l| l.header_id).collect();

    // Processed blocks tracker
    let mut processed: HashSet<usize> = HashSet::new();

    structure_region(&rpo, cfg, dom_tree, loops, &loop_headers, &mut processed)
}

fn structure_region(
    rpo: &[usize],
    cfg: &crate::ir::analyze::ControlFlowGraph,
    dom_tree: &DominatorTree,
    loops: &LoopInfo,
    loop_headers: &HashSet<usize>,
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
                        loops,
                        loop_headers,
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
                loops,
                loop_headers,
                processed,
            );
            let else_region = if else_id != then_id {
                Some(Box::new(build_branch_region(
                    else_id,
                    block_id,
                    rpo,
                    cfg,
                    dom_tree,
                    loops,
                    loop_headers,
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
    loops: &LoopInfo,
    loop_headers: &HashSet<usize>,
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
    structure_region(&branch_rpo, cfg, dom_tree, loops, loop_headers, processed)
}

fn structure_loop(
    header: usize,
    natural_loop: &NaturalLoop,
    cfg: &crate::ir::analyze::ControlFlowGraph,
    dom_tree: &DominatorTree,
    loops: &LoopInfo,
    loop_headers: &HashSet<usize>,
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
    let rpo = reverse_postorder(cfg);
    let body_rpo: Vec<usize> = rpo
        .iter()
        .copied()
        .filter(|&b| loop_blocks.contains(&b) && b != excluded)
        .collect();

    // Use a local processed set for the body, seeded with only the excluded
    // control block to prevent re-entering the current loop's control flow
    let mut body_processed: HashSet<usize> = HashSet::new();
    body_processed.insert(excluded);

    let body = if body_rpo.is_empty() {
        StructuredRegion::Block(excluded)
    } else {
        structure_region(
            &body_rpo,
            cfg,
            dom_tree,
            loops,
            loop_headers,
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

    debug!(
        "CFG structuring: {} blocks, {} constructs, {} gotos",
        structured.block_count(),
        structured.construct_count(),
        structured.goto_count(),
    );
}

fn reverse_postorder(cfg: &crate::ir::analyze::ControlFlowGraph) -> Vec<usize> {
    fn dfs(
        block_id: usize,
        cfg: &crate::ir::analyze::ControlFlowGraph,
        visited: &mut HashSet<usize>,
        postorder: &mut Vec<usize>,
    ) {
        if !visited.insert(block_id) {
            return;
        }

        for &successor_id in cfg.successors_of(block_id) {
            dfs(successor_id, cfg, visited, postorder);
        }

        postorder.push(block_id);
    }

    let mut visited = HashSet::new();
    let mut postorder = Vec::new();
    dfs(cfg.entry_block_id(), cfg, &mut visited, &mut postorder);

    let mut remaining = cfg
        .block_ids()
        .iter()
        .copied()
        .filter(|block_id| !visited.contains(block_id))
        .collect::<VecDeque<_>>();
    while let Some(block_id) = remaining.pop_front() {
        dfs(block_id, cfg, &mut visited, &mut postorder);
    }

    postorder.reverse();
    postorder
}

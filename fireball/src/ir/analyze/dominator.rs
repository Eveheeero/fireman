use crate::core::{Address, Block, RelationType};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    entry_block_id: usize,
    block_ids: Vec<usize>,
    successors: HashMap<usize, Vec<usize>>,
    predecessors: HashMap<usize, Vec<usize>>,
    exit_block_ids: Vec<usize>,
}

impl ControlFlowGraph {
    pub fn from_blocks(blocks: &[Arc<Block>], entry_block_id: usize) -> Self {
        let mut block_ids = blocks
            .iter()
            .map(|block| block.get_id())
            .collect::<Vec<_>>();
        block_ids.sort_unstable();

        let successors = collect_cfg_successors(blocks);

        let mut predecessors = block_ids
            .iter()
            .copied()
            .map(|block_id| (block_id, Vec::new()))
            .collect::<HashMap<_, _>>();
        for (&block_id, next_ids) in &successors {
            for &next_id in next_ids {
                predecessors.entry(next_id).or_default().push(block_id);
            }
        }
        for incoming in predecessors.values_mut() {
            incoming.sort_unstable();
            incoming.dedup();
        }

        let mut exit_block_ids = block_ids
            .iter()
            .copied()
            .filter(|block_id| {
                successors
                    .get(block_id)
                    .is_none_or(|next_ids| next_ids.is_empty())
            })
            .collect::<Vec<_>>();
        exit_block_ids.sort_unstable();

        Self {
            entry_block_id,
            block_ids,
            successors,
            predecessors,
            exit_block_ids,
        }
    }

    pub fn entry_block_id(&self) -> usize {
        self.entry_block_id
    }

    pub fn block_ids(&self) -> &[usize] {
        &self.block_ids
    }

    pub fn successors_of(&self, block_id: usize) -> &[usize] {
        self.successors
            .get(&block_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn predecessors_of(&self, block_id: usize) -> &[usize] {
        self.predecessors
            .get(&block_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn exit_block_ids(&self) -> &[usize] {
        &self.exit_block_ids
    }

    /// L194: Detect blocks that are reachable from outside the function's
    /// normal entry — i.e., non-entry blocks with incoming CFG edges from
    /// blocks NOT in this CFG. These represent potential multi-entry points
    /// (shared tails). Requires the original block list to inspect raw relations.
    pub fn multi_entry_blocks(&self, blocks: &[Arc<Block>]) -> Vec<usize> {
        let block_set: HashSet<usize> = self.block_ids.iter().copied().collect();
        let mut result = Vec::new();
        for block in blocks {
            let id = block.get_id();
            if id == self.entry_block_id || !block_set.contains(&id) {
                continue;
            }
            // Check raw incoming relations for edges from blocks outside this CFG.
            let has_external_pred = block.get_connected_from().iter().any(|rel| {
                matches!(
                    rel.relation_type(),
                    RelationType::Jump | RelationType::Jcc | RelationType::Continued
                ) && !block_set.contains(&rel.from())
            });
            if has_external_pred {
                result.push(id);
            }
        }
        result
    }

    /// L192: Detect potential hot-cold split chunks — blocks that are
    /// separated from the main body by a large address gap, suggesting
    /// PGO/linker cold-path splitting.
    pub fn detect_address_gap_chunks(
        &self,
        blocks: &[Arc<Block>],
        gap_threshold: u64,
    ) -> Vec<Vec<usize>> {
        let mut block_addrs: Vec<(usize, u64)> = blocks
            .iter()
            .filter(|b| self.block_ids.contains(&b.get_id()))
            .map(|b| (b.get_id(), b.get_start_address().get_virtual_address()))
            .collect();
        block_addrs.sort_by_key(|(_, addr)| *addr);

        let mut chunks: Vec<Vec<usize>> = Vec::new();
        let mut current_chunk: Vec<usize> = Vec::new();
        let mut prev_addr: Option<u64> = None;

        for (id, addr) in &block_addrs {
            if let Some(prev) = prev_addr {
                if *addr > prev && (*addr - prev) > gap_threshold {
                    if !current_chunk.is_empty() {
                        chunks.push(std::mem::take(&mut current_chunk));
                    }
                }
            }
            current_chunk.push(*id);
            prev_addr = Some(*addr);
        }
        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }
        chunks
    }

    /// Check whether this CFG is reducible (all loops are natural loops).
    /// A CFG is reducible iff every back edge (where target dominates source)
    /// accounts for ALL retreating edges. Equivalently, after removing all
    /// back edges identified by the dominator tree, the remaining graph is a DAG.
    pub fn is_reducible(&self, dominator_tree: &DominatorTree) -> bool {
        let back_edges: HashSet<(usize, usize)> = dominator_tree.back_edges().into_iter().collect();
        // Build a graph without back edges and check for cycles (DFS).
        let mut visited = HashSet::new();
        let mut on_stack = HashSet::new();
        for &block_id in &self.block_ids {
            if !visited.contains(&block_id)
                && has_cycle_without_back_edges(
                    block_id,
                    &self.successors,
                    &back_edges,
                    &mut visited,
                    &mut on_stack,
                )
            {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct DominatorTree {
    cfg: ControlFlowGraph,
    dominators: HashMap<usize, HashSet<usize>>,
    immediate_dominators: HashMap<usize, usize>,
}

impl DominatorTree {
    pub fn compute(cfg: ControlFlowGraph) -> Self {
        let all_blocks = cfg.block_ids.iter().copied().collect::<HashSet<_>>();
        let entry_block_id = cfg.entry_block_id();

        let mut dominators = HashMap::new();
        for &block_id in cfg.block_ids() {
            let block_dominators = if block_id == entry_block_id {
                HashSet::from([block_id])
            } else {
                all_blocks.clone()
            };
            dominators.insert(block_id, block_dominators);
        }

        let mut changed = true;
        while changed {
            changed = false;
            for &block_id in cfg.block_ids() {
                if block_id == entry_block_id {
                    continue;
                }

                let predecessors = cfg.predecessors_of(block_id);
                let mut new_dominators = if predecessors.is_empty() {
                    HashSet::from([block_id])
                } else {
                    let mut iter = predecessors.iter().copied();
                    let first = iter.next().unwrap();
                    let mut intersection = dominators.get(&first).cloned().unwrap_or_default();
                    for predecessor in iter {
                        if let Some(candidate) = dominators.get(&predecessor) {
                            intersection.retain(|dominator| candidate.contains(dominator));
                        } else {
                            intersection.clear();
                            break;
                        }
                    }
                    intersection.insert(block_id);
                    intersection
                };

                if new_dominators.is_empty() {
                    new_dominators.insert(block_id);
                }

                let old_dominators = dominators.get(&block_id).cloned().unwrap_or_default();
                if old_dominators != new_dominators {
                    dominators.insert(block_id, new_dominators);
                    changed = true;
                }
            }
        }

        let immediate_dominators = build_immediate_map(&dominators, entry_block_id);

        Self {
            cfg,
            dominators,
            immediate_dominators,
        }
    }

    pub fn cfg(&self) -> &ControlFlowGraph {
        &self.cfg
    }

    pub fn dominates(&self, dominator: usize, block_id: usize) -> bool {
        self.dominators
            .get(&block_id)
            .is_some_and(|dominators| dominators.contains(&dominator))
    }

    pub fn dominators_of(&self, block_id: usize) -> Vec<usize> {
        let mut dominators = self
            .dominators
            .get(&block_id)
            .map(|items| items.iter().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        dominators.sort_unstable();
        dominators
    }

    pub fn immediate_dominator_of(&self, block_id: usize) -> Option<usize> {
        self.immediate_dominators.get(&block_id).copied()
    }

    pub fn back_edges(&self) -> Vec<(usize, usize)> {
        let mut back_edges = Vec::new();
        for &source_id in self.cfg.block_ids() {
            for &target_id in self.cfg.successors_of(source_id) {
                if self.dominates(target_id, source_id) {
                    back_edges.push((source_id, target_id));
                }
            }
        }
        back_edges.sort_unstable();
        back_edges
    }
}

#[derive(Debug, Clone)]
pub struct PostDominatorTree {
    cfg: ControlFlowGraph,
    post_dominators: HashMap<usize, HashSet<usize>>,
    immediate_post_dominators: HashMap<usize, usize>,
}

impl PostDominatorTree {
    pub fn compute(cfg: ControlFlowGraph) -> Self {
        let all_blocks = cfg.block_ids.iter().copied().collect::<HashSet<_>>();
        let exit_blocks = cfg.exit_block_ids.iter().copied().collect::<HashSet<_>>();

        let mut post_dominators = HashMap::new();
        for &block_id in cfg.block_ids() {
            let block_post_dominators = if exit_blocks.contains(&block_id) {
                HashSet::from([block_id])
            } else {
                all_blocks.clone()
            };
            post_dominators.insert(block_id, block_post_dominators);
        }

        let mut changed = true;
        while changed {
            changed = false;
            for &block_id in cfg.block_ids() {
                if exit_blocks.contains(&block_id) {
                    continue;
                }

                let successors = cfg.successors_of(block_id);
                let mut new_post_dominators = if successors.is_empty() {
                    HashSet::from([block_id])
                } else {
                    let mut iter = successors.iter().copied();
                    let first = iter.next().unwrap();
                    let mut intersection = post_dominators.get(&first).cloned().unwrap_or_default();
                    for successor in iter {
                        if let Some(candidate) = post_dominators.get(&successor) {
                            intersection
                                .retain(|post_dominator| candidate.contains(post_dominator));
                        } else {
                            intersection.clear();
                            break;
                        }
                    }
                    intersection.insert(block_id);
                    intersection
                };

                if new_post_dominators.is_empty() {
                    new_post_dominators.insert(block_id);
                }

                let old_post_dominators =
                    post_dominators.get(&block_id).cloned().unwrap_or_default();
                if old_post_dominators != new_post_dominators {
                    post_dominators.insert(block_id, new_post_dominators);
                    changed = true;
                }
            }
        }

        let immediate_post_dominators =
            build_immediate_map_for_exits(&post_dominators, &exit_blocks);

        Self {
            cfg,
            post_dominators,
            immediate_post_dominators,
        }
    }

    pub fn cfg(&self) -> &ControlFlowGraph {
        &self.cfg
    }

    pub fn post_dominates(&self, post_dominator: usize, block_id: usize) -> bool {
        self.post_dominators
            .get(&block_id)
            .is_some_and(|post_dominators| post_dominators.contains(&post_dominator))
    }

    pub fn post_dominators_of(&self, block_id: usize) -> Vec<usize> {
        let mut post_dominators = self
            .post_dominators
            .get(&block_id)
            .map(|items| items.iter().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        post_dominators.sort_unstable();
        post_dominators
    }

    pub fn immediate_post_dominator_of(&self, block_id: usize) -> Option<usize> {
        self.immediate_post_dominators.get(&block_id).copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaturalLoop {
    pub header_id: usize,
    pub latch_ids: Vec<usize>,
    pub body_ids: Vec<usize>,
    pub exit_ids: Vec<usize>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ControlDependence {
    controlling_predicates: HashMap<usize, Vec<usize>>,
    controlled_blocks: HashMap<usize, Vec<usize>>,
}

impl ControlDependence {
    pub fn compute(cfg: &ControlFlowGraph, post_dominator_tree: &PostDominatorTree) -> Self {
        let mut controlling_predicates = HashMap::<usize, HashSet<usize>>::new();
        let mut controlled_blocks = HashMap::<usize, HashSet<usize>>::new();

        for &predicate_id in cfg.block_ids() {
            let stop_id = post_dominator_tree.immediate_post_dominator_of(predicate_id);
            for &successor_id in cfg.successors_of(predicate_id) {
                if post_dominator_tree.post_dominates(successor_id, predicate_id) {
                    continue;
                }

                let mut runner = Some(successor_id);
                while let Some(block_id) = runner {
                    if Some(block_id) == stop_id {
                        break;
                    }

                    controlling_predicates
                        .entry(block_id)
                        .or_default()
                        .insert(predicate_id);
                    controlled_blocks
                        .entry(predicate_id)
                        .or_default()
                        .insert(block_id);

                    runner = post_dominator_tree.immediate_post_dominator_of(block_id);
                }
            }
        }

        Self {
            controlling_predicates: into_sorted_vec_map(controlling_predicates),
            controlled_blocks: into_sorted_vec_map(controlled_blocks),
        }
    }

    pub fn controlling_predicates_of(&self, block_id: usize) -> &[usize] {
        self.controlling_predicates
            .get(&block_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn controlled_blocks_of(&self, predicate_id: usize) -> &[usize] {
        self.controlled_blocks
            .get(&predicate_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn is_control_dependent(&self, block_id: usize, predicate_id: usize) -> bool {
        self.controlling_predicates_of(block_id)
            .contains(&predicate_id)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LoopInfo {
    loops: Vec<NaturalLoop>,
}

impl LoopInfo {
    pub fn compute(dominator_tree: &DominatorTree) -> Self {
        let cfg = dominator_tree.cfg();
        let mut loops_by_header = HashMap::<usize, NaturalLoop>::new();

        for (source_id, header_id) in dominator_tree.back_edges() {
            let loop_body = collect_natural_loop_nodes(cfg, source_id, header_id);
            let mut exit_ids = loop_body
                .iter()
                .copied()
                .flat_map(|block_id| cfg.successors_of(block_id).iter().copied())
                .filter(|successor_id| !loop_body.contains(successor_id))
                .collect::<Vec<_>>();
            exit_ids.sort_unstable();
            exit_ids.dedup();

            let entry = loops_by_header
                .entry(header_id)
                .or_insert_with(|| NaturalLoop {
                    header_id,
                    latch_ids: Vec::new(),
                    body_ids: Vec::new(),
                    exit_ids: Vec::new(),
                });

            entry.latch_ids.push(source_id);

            for block_id in loop_body {
                if !entry.body_ids.contains(&block_id) {
                    entry.body_ids.push(block_id);
                }
            }
            for exit_id in exit_ids {
                if !entry.exit_ids.contains(&exit_id) {
                    entry.exit_ids.push(exit_id);
                }
            }
        }

        let mut loops = loops_by_header.into_values().collect::<Vec<_>>();
        for loop_info in &mut loops {
            loop_info.latch_ids.sort_unstable();
            loop_info.latch_ids.dedup();
            loop_info.body_ids.sort_unstable();
            loop_info.exit_ids.sort_unstable();
        }
        loops.sort_unstable_by_key(|loop_info| loop_info.header_id);

        Self { loops }
    }

    pub fn loops(&self) -> &[NaturalLoop] {
        &self.loops
    }

    pub fn loop_for_header(&self, header_id: usize) -> Option<&NaturalLoop> {
        self.loops
            .iter()
            .find(|loop_info| loop_info.header_id == header_id)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionControlFlowAnalysis {
    cfg: ControlFlowGraph,
    dominators: DominatorTree,
    postdominators: PostDominatorTree,
    loops: LoopInfo,
    control_dependence: ControlDependence,
}

impl FunctionControlFlowAnalysis {
    pub fn compute(blocks: &[Arc<Block>], entry_block_id: usize) -> Self {
        let cfg = ControlFlowGraph::from_blocks(blocks, entry_block_id);
        let dominators = DominatorTree::compute(cfg.clone());
        let postdominators = PostDominatorTree::compute(cfg.clone());
        let loops = LoopInfo::compute(&dominators);
        let control_dependence = ControlDependence::compute(&cfg, &postdominators);
        Self {
            cfg,
            dominators,
            postdominators,
            loops,
            control_dependence,
        }
    }

    pub fn cfg(&self) -> &ControlFlowGraph {
        &self.cfg
    }

    pub fn dominators(&self) -> &DominatorTree {
        &self.dominators
    }

    pub fn postdominators(&self) -> &PostDominatorTree {
        &self.postdominators
    }

    pub fn loops(&self) -> &LoopInfo {
        &self.loops
    }

    pub fn control_dependence(&self) -> &ControlDependence {
        &self.control_dependence
    }
}

pub fn infer_entry_block_id(blocks: &[Arc<Block>]) -> Option<usize> {
    let mut candidates = blocks
        .iter()
        .map(|block| {
            let incoming_cfg_edges = block
                .get_connected_from()
                .iter()
                .filter(|relation| {
                    matches!(
                        relation.relation_type(),
                        RelationType::Jump | RelationType::Jcc | RelationType::Continued
                    )
                })
                .count();
            (
                incoming_cfg_edges,
                block.get_start_address().get_virtual_address(),
                block.get_id(),
            )
        })
        .collect::<Vec<_>>();
    candidates.sort_unstable();
    candidates.first().map(|(_, _, block_id)| *block_id)
}

pub fn analyze_dominators(blocks: &[Arc<Block>], entry_block_id: usize) -> DominatorTree {
    DominatorTree::compute(ControlFlowGraph::from_blocks(blocks, entry_block_id))
}

pub fn analyze_postdominators(blocks: &[Arc<Block>], entry_block_id: usize) -> PostDominatorTree {
    PostDominatorTree::compute(ControlFlowGraph::from_blocks(blocks, entry_block_id))
}

pub fn analyze_loops(blocks: &[Arc<Block>], entry_block_id: usize) -> LoopInfo {
    let dominator_tree = analyze_dominators(blocks, entry_block_id);
    LoopInfo::compute(&dominator_tree)
}

pub fn analyze_control_dependence(
    blocks: &[Arc<Block>],
    entry_block_id: usize,
) -> ControlDependence {
    let cfg = ControlFlowGraph::from_blocks(blocks, entry_block_id);
    let post_dominator_tree = PostDominatorTree::compute(cfg.clone());
    ControlDependence::compute(&cfg, &post_dominator_tree)
}

pub fn analyze_function_control_flow(
    blocks: &[Arc<Block>],
    entry_block_id: usize,
) -> FunctionControlFlowAnalysis {
    FunctionControlFlowAnalysis::compute(blocks, entry_block_id)
}

fn collect_cfg_successors(blocks: &[Arc<Block>]) -> HashMap<usize, Vec<usize>> {
    let known_ids = blocks
        .iter()
        .map(|block| block.get_id())
        .collect::<HashSet<_>>();
    let mut successors = known_ids
        .iter()
        .copied()
        .map(|block_id| (block_id, HashSet::new()))
        .collect::<HashMap<_, _>>();

    for block in blocks {
        for relation in block
            .get_connected_from()
            .iter()
            .chain(block.get_connected_to().iter())
        {
            if !matches!(
                relation.relation_type(),
                RelationType::Jump | RelationType::Jcc | RelationType::Continued
            ) {
                continue;
            }

            let from_id = relation.from();
            if !known_ids.contains(&from_id) {
                continue;
            }

            let Some(to_address) = relation.to() else {
                continue;
            };
            let Some(to_id) = find_block_id_by_address(blocks, &to_address) else {
                continue;
            };
            if !known_ids.contains(&to_id) {
                continue;
            }

            successors.entry(from_id).or_default().insert(to_id);
        }
    }

    successors
        .into_iter()
        .map(|(block_id, next_ids)| {
            let mut next_ids = next_ids.into_iter().collect::<Vec<_>>();
            next_ids.sort_unstable();
            (block_id, next_ids)
        })
        .collect::<HashMap<_, _>>()
}

fn find_block_id_by_address(blocks: &[Arc<Block>], address: &Address) -> Option<usize> {
    blocks
        .iter()
        .find(|block| block.contains(address))
        .map(|block| block.get_id())
}

fn build_immediate_map(
    dominators: &HashMap<usize, HashSet<usize>>,
    root_block_id: usize,
) -> HashMap<usize, usize> {
    let mut immediate_map = HashMap::new();

    for (&block_id, block_dominators) in dominators {
        if block_id == root_block_id {
            continue;
        }

        let strict_dominators = block_dominators
            .iter()
            .copied()
            .filter(|dominator| *dominator != block_id)
            .collect::<Vec<_>>();

        if let Some(immediate_dominator) = strict_dominators.iter().copied().find(|candidate| {
            strict_dominators.iter().copied().all(|other| {
                other == *candidate
                    || !dominators
                        .get(&other)
                        .is_some_and(|other_dominators| other_dominators.contains(candidate))
            })
        }) {
            immediate_map.insert(block_id, immediate_dominator);
        }
    }

    immediate_map
}

fn build_immediate_map_for_exits(
    post_dominators: &HashMap<usize, HashSet<usize>>,
    exit_blocks: &HashSet<usize>,
) -> HashMap<usize, usize> {
    let mut immediate_map = HashMap::new();

    for (&block_id, block_post_dominators) in post_dominators {
        if exit_blocks.contains(&block_id) {
            continue;
        }

        let strict_post_dominators = block_post_dominators
            .iter()
            .copied()
            .filter(|post_dominator| *post_dominator != block_id)
            .collect::<Vec<_>>();

        if let Some(immediate_post_dominator) =
            strict_post_dominators.iter().copied().find(|candidate| {
                strict_post_dominators.iter().copied().all(|other| {
                    other == *candidate
                        || !post_dominators
                            .get(&other)
                            .is_some_and(|other_post_dominators| {
                                other_post_dominators.contains(candidate)
                            })
                })
            })
        {
            immediate_map.insert(block_id, immediate_post_dominator);
        }
    }

    immediate_map
}

/// Dominance frontier: for each block B, the set of blocks where B's
/// dominance stops — i.e., blocks that have a predecessor dominated by B
/// but are NOT strictly dominated by B. Essential for SSA phi-node placement.
#[derive(Debug, Clone)]
pub struct DominanceFrontier {
    frontiers: HashMap<usize, Vec<usize>>,
}

impl DominanceFrontier {
    pub fn compute(dominator_tree: &DominatorTree) -> Self {
        let cfg = dominator_tree.cfg();
        let mut frontiers: HashMap<usize, HashSet<usize>> = HashMap::new();
        for &block_id in cfg.block_ids() {
            frontiers.insert(block_id, HashSet::new());
        }

        for &block_id in cfg.block_ids() {
            let preds = cfg.predecessors_of(block_id);
            if preds.len() < 2 {
                continue;
            }
            for &pred in preds {
                let mut runner = pred;
                let idom = dominator_tree.immediate_dominator_of(block_id);
                while Some(runner) != idom {
                    frontiers.entry(runner).or_default().insert(block_id);
                    if let Some(next) = dominator_tree.immediate_dominator_of(runner) {
                        runner = next;
                    } else {
                        break;
                    }
                }
            }
        }

        Self {
            frontiers: frontiers
                .into_iter()
                .map(|(k, v)| {
                    let mut v = v.into_iter().collect::<Vec<_>>();
                    v.sort_unstable();
                    (k, v)
                })
                .collect(),
        }
    }

    pub fn frontier_of(&self, block_id: usize) -> &[usize] {
        self.frontiers
            .get(&block_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn has_frontier(&self, block_id: usize) -> bool {
        !self.frontier_of(block_id).is_empty()
    }
}

fn collect_natural_loop_nodes(
    cfg: &ControlFlowGraph,
    source_id: usize,
    header_id: usize,
) -> HashSet<usize> {
    let mut loop_nodes = HashSet::from([header_id, source_id]);
    let mut stack = vec![source_id];

    while let Some(block_id) = stack.pop() {
        for &predecessor_id in cfg.predecessors_of(block_id) {
            if loop_nodes.insert(predecessor_id) && predecessor_id != header_id {
                stack.push(predecessor_id);
            }
        }
    }

    loop_nodes
}

fn into_sorted_vec_map(map: HashMap<usize, HashSet<usize>>) -> HashMap<usize, Vec<usize>> {
    map.into_iter()
        .map(|(block_id, related_ids)| {
            let mut related_ids = related_ids.into_iter().collect::<Vec<_>>();
            related_ids.sort_unstable();
            (block_id, related_ids)
        })
        .collect()
}

/// DFS cycle detection on the CFG with back edges removed.
fn has_cycle_without_back_edges(
    block_id: usize,
    successors: &HashMap<usize, Vec<usize>>,
    back_edges: &HashSet<(usize, usize)>,
    visited: &mut HashSet<usize>,
    on_stack: &mut HashSet<usize>,
) -> bool {
    visited.insert(block_id);
    on_stack.insert(block_id);
    if let Some(succs) = successors.get(&block_id) {
        for &succ in succs {
            if back_edges.contains(&(block_id, succ)) {
                continue;
            }
            if on_stack.contains(&succ) {
                return true;
            }
            if !visited.contains(&succ)
                && has_cycle_without_back_edges(succ, successors, back_edges, visited, on_stack)
            {
                return true;
            }
        }
    }
    on_stack.remove(&block_id);
    false
}

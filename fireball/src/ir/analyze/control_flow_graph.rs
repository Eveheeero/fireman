use crate::{
    core::{Address, Block, Relation},
    prelude::*,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct ControlFlowGraphAnalyzer {
    targets: Vec<Arc<Block>>,
    relations: Vec<Relation>,
    analyzed: Option<Vec<ControlFlowGraph>>,
}

impl ControlFlowGraphAnalyzer {
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
            analyzed: None,
            relations: Vec::new(),
        }
    }
    pub fn add_target(&mut self, target: Arc<Block>) {
        {
            let connected_from = target.get_connected_from();
            let connected_to = target.get_connected_to();

            debug!(
                "CFG analyzer target block (id: {}, start address: {}) relation from: (id){:?}, to: (addr){:?}",
                target.get_id(),
                target.get_start_address(),
                connected_from.iter().map(|r| r.from()).collect::<Vec<_>>(),
                connected_to
                    .iter()
                    .filter_map(|r| r.to())
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
            );

            for relation in connected_from.iter() {
                let block_id = relation.from();
                if self.targets.iter().any(|x| x.get_id() == block_id) {
                    debug!("Found relation with: (id){}", block_id);
                    self.relations.push(relation.clone());
                }
            }
            for relation in connected_to.iter() {
                let Some(to) = relation.to() else {
                    continue;
                };
                if self.targets.iter().any(|x| x.contains(&to)) {
                    debug!("Found relation with: (addr){}", to);
                    self.relations.push(relation.clone());
                }
            }
        }
        self.targets.push(target);
    }
    pub fn add_targets(&mut self, targets: impl IntoIterator<Item = Arc<Block>>) {
        for target in targets {
            self.add_target(target);
        }
    }
    pub fn get_targets(&self) -> &Vec<Arc<Block>> {
        &self.targets
    }
    pub fn get_analyzed(&self) -> Option<&Vec<ControlFlowGraph>> {
        self.analyzed.as_ref()
    }
    pub fn analyze(&mut self) -> &Vec<ControlFlowGraph> {
        self.analyzed = Some(analyze_control_flow_graph(&self.targets, &self.relations));
        self.analyzed.as_ref().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct LoopInfo {
    pub loop_from: Arc<Block>,
    pub loop_to: Arc<Block>,
}

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    blocks: Vec<Arc<Block>>,
    loops: Vec<LoopInfo>,
}

impl ControlFlowGraph {
    pub fn get_blocks(&self) -> &Vec<Arc<Block>> {
        &self.blocks
    }
    pub fn get_loops(&self) -> &Vec<LoopInfo> {
        &self.loops
    }
}

fn find_block_id_by_address(blocks: &[Arc<Block>], address: &Address) -> Option<usize> {
    for block in blocks.iter() {
        if block.contains(address) {
            return Some(block.get_id());
        }
    }
    None
}

fn dfs_loop_detection(
    id_to_block: &BTreeMap<usize, &Arc<Block>>,
    now_id: usize,
    componenets_relation_map: &BTreeMap<usize, Vec<usize>>,
    dfs_visited_id: &mut BTreeSet<usize>,
    stack: &mut BTreeSet<usize>,
    loops: &mut Vec<LoopInfo>,
) {
    dfs_visited_id.insert(now_id);
    stack.insert(now_id);

    if let Some(neighbors) = componenets_relation_map.get(&now_id) {
        for &neighbor_id in neighbors.iter() {
            if !dfs_visited_id.contains(&neighbor_id) {
                dfs_loop_detection(
                    id_to_block,
                    neighbor_id,
                    componenets_relation_map,
                    dfs_visited_id,
                    stack,
                    loops,
                );
            } else if stack.contains(&neighbor_id) {
                // means neighbot already visited
                loops.push(LoopInfo {
                    loop_from: (*id_to_block.get(&neighbor_id).unwrap()).clone(),
                    loop_to: (*id_to_block.get(&now_id).unwrap()).clone(),
                });
            }
        }
    }

    stack.remove(&now_id);
}

pub fn analyze_control_flow_graph(
    blocks: &[Arc<Block>],
    relations: &[Relation],
) -> Vec<ControlFlowGraph> {
    let id_to_block: BTreeMap<usize, &Arc<Block>> =
        blocks.iter().map(|block| (block.get_id(), block)).collect();

    /* Turn relations to mapped relations */
    let mut relations_map: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for &block_id in id_to_block.keys() {
        relations_map.insert(block_id, BTreeSet::new());
    }
    for relation in relations.iter() {
        let from_id = relation.from();
        if let Some(to_address) = relation.to() {
            if let Some(to_id) = find_block_id_by_address(blocks, &to_address) {
                if id_to_block.contains_key(&from_id) && id_to_block.contains_key(&to_id) {
                    relations_map.get_mut(&from_id).unwrap().insert(to_id);
                    relations_map.get_mut(&to_id).unwrap().insert(from_id);
                }
            }
        }
    }

    let mut visited_id: BTreeSet<usize> = BTreeSet::new();
    let mut cfgs: Vec<ControlFlowGraph> = Vec::new();
    for start_node_id in id_to_block.keys() {
        if !visited_id.contains(start_node_id) {
            let mut component_ids: Vec<usize> = Vec::new();
            visited_id.insert(*start_node_id);
            component_ids.push(*start_node_id);

            /* Search for related blocks */
            {
                let mut stack: Vec<usize> = Vec::new();
                stack.push(*start_node_id);
                while let Some(now_id) = stack.pop() {
                    if let Some(neighbors) = relations_map.get(&now_id) {
                        for &neighbot_id in neighbors.iter() {
                            if !visited_id.contains(&neighbot_id) {
                                visited_id.insert(neighbot_id);
                                stack.push(neighbot_id);
                                component_ids.push(neighbot_id);
                            }
                        }
                    }
                }
            }
            let component_blocks: Vec<Arc<Block>> = component_ids
                .iter()
                .filter_map(|id| id_to_block.get(id).cloned())
                .cloned()
                .collect();

            /* Analyze with component blocks */
            // Turn relations to mapped relations for this componetn
            let mut componenets_relation_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
            {
                for &block_id in component_ids.iter() {
                    componenets_relation_map.insert(block_id, Vec::new());
                }
                for relation in relations.iter() {
                    let from_id = relation.from();
                    if component_ids.contains(&from_id) {
                        if let Some(to_address) = relation.to() {
                            if let Some(to_id) = find_block_id_by_address(blocks, &to_address) {
                                if component_ids.contains(&to_id) {
                                    componenets_relation_map
                                        .get_mut(&from_id)
                                        .unwrap()
                                        .push(to_id);
                                }
                            }
                        }
                    }
                }
            }

            /* Searching for looping blocks */
            let mut component_loops: Vec<LoopInfo> = Vec::new();
            {
                let mut dfs_visited_id: BTreeSet<usize> = BTreeSet::new();
                let mut stack: BTreeSet<usize> = BTreeSet::new();
                for &start_node_id in component_ids.iter() {
                    if !dfs_visited_id.contains(&start_node_id) {
                        dfs_loop_detection(
                            &id_to_block,
                            start_node_id,
                            &componenets_relation_map,
                            &mut dfs_visited_id,
                            &mut stack,
                            &mut component_loops,
                        );
                    }
                }
            }

            cfgs.push(ControlFlowGraph {
                blocks: component_blocks,
                loops: component_loops,
            });
        }
    }

    cfgs
}

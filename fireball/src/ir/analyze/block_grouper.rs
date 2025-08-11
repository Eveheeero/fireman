use crate::{
    core::{Address, Block, Relation},
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct BlockGrouper {
    targets: Vec<Arc<Block>>,
    relations: Vec<Relation>,
    analyzed: Option<Vec<BlockGroup>>,
}

impl BlockGrouper {
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
                "Block grouper target block (id: {}, start address: {}) relation from: (id){:?}, to: (addr){:?}",
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
    pub fn get_analyzed(&self) -> Option<&Vec<BlockGroup>> {
        self.analyzed.as_ref()
    }
    pub fn analyze(&mut self) -> &Vec<BlockGroup> {
        self.analyzed = Some(analyze_block_groups(&self.targets, &self.relations));
        self.analyzed.as_ref().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct BlockGroup {
    blocks: Vec<Arc<Block>>,
}

impl BlockGroup {
    pub fn get_blocks(&self) -> &Vec<Arc<Block>> {
        &self.blocks
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

pub fn analyze_block_groups(blocks: &[Arc<Block>], relations: &[Relation]) -> Vec<BlockGroup> {
    let id_to_block: HashMap<usize, &Arc<Block>> =
        blocks.iter().map(|block| (block.get_id(), block)).collect();

    /* Turn relations to mapped relations */
    let mut relations_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for &block_id in id_to_block.keys() {
        relations_map.insert(block_id, HashSet::new());
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

    let mut visited_id: HashSet<usize> = HashSet::new();
    let mut block_groups: Vec<BlockGroup> = Vec::new();
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
                        for &neighbor_id in neighbors.iter() {
                            if !visited_id.contains(&neighbor_id) {
                                visited_id.insert(neighbor_id);
                                stack.push(neighbor_id);
                                component_ids.push(neighbor_id);
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

            block_groups.push(BlockGroup {
                blocks: component_blocks,
            });
        }
    }

    block_groups
}

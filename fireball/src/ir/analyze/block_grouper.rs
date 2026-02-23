use crate::{
    core::{Address, Block, Relation, RelationType},
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
        self.targets.push(target);
        self.refresh_relations();
    }
    pub fn add_targets(&mut self, targets: impl IntoIterator<Item = Arc<Block>>) {
        self.targets.extend(targets);
        self.refresh_relations();
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

    fn refresh_relations(&mut self) {
        let mut relation_set: HashSet<Relation> = HashSet::new();
        for target in self.targets.iter() {
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
                if self.targets.iter().any(|x| x.get_id() == relation.from()) {
                    relation_set.insert(relation.clone());
                }
            }
            for relation in connected_to.iter() {
                let Some(to) = relation.to() else {
                    continue;
                };
                if self.targets.iter().any(|x| x.contains(&to)) {
                    relation_set.insert(relation.clone());
                }
            }
        }

        let mut relations: Vec<_> = relation_set.into_iter().collect();
        relations.sort_unstable_by_key(|r| {
            (
                r.from(),
                r.to().map(|a| a.get_virtual_address()).unwrap_or(u64::MAX),
                relation_type_rank(*r.relation_type()),
            )
        });
        self.relations = relations;
    }
}

fn relation_type_rank(ty: RelationType) -> u8 {
    match ty {
        RelationType::Call => 0,
        RelationType::Halt => 1,
        RelationType::Jump => 2,
        RelationType::Jcc => 3,
        RelationType::Continued => 4,
        RelationType::Return => 5,
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
    let mut map_from_to: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut map_to_from: HashMap<usize, Vec<usize>> = HashMap::new();
    for &block_id in id_to_block.keys() {
        map_from_to.insert(block_id, Vec::new());
        map_to_from.insert(block_id, Vec::new());
    }
    for relation in relations.iter() {
        // `Call` edges connect different functions. Excluding them keeps
        // caller/callee separated while preserving intra-function flow.
        if matches!(relation.relation_type(), RelationType::Call) {
            continue;
        }
        let from_id = relation.from();
        if let Some(to_address) = relation.to()
            && let Some(to_id) = find_block_id_by_address(blocks, &to_address)
        {
            if id_to_block.contains_key(&from_id) && id_to_block.contains_key(&to_id) {
                let tos = map_from_to.get_mut(&from_id).unwrap();
                if !tos.contains(&to_id) {
                    tos.push(to_id);
                }
                let froms = map_to_from.get_mut(&to_id).unwrap();
                if !froms.contains(&from_id) {
                    froms.push(from_id);
                }
            }
        }
    }
    for tos in map_from_to.values_mut() {
        tos.sort_unstable();
    }
    for froms in map_to_from.values_mut() {
        froms.sort_unstable();
    }

    let mut start_node_ids: Vec<usize> = id_to_block.keys().copied().collect::<Vec<_>>();
    start_node_ids.sort_unstable_by_key(|id| {
        let is_root = map_to_from.get(id).is_some_and(|x| x.is_empty());
        let block = id_to_block.get(id).unwrap();
        (
            if is_root { 0u8 } else { 1u8 },
            block.get_start_address().get_virtual_address(),
            *id,
        )
    });
    let mut visited_id: HashSet<usize> = HashSet::new();
    let mut block_groups: Vec<BlockGroup> = Vec::new();
    for start_node_id in start_node_ids {
        if !visited_id.contains(&start_node_id) {
            let mut component_ids: Vec<usize> = Vec::new();
            visited_id.insert(start_node_id);
            component_ids.push(start_node_id);

            /* Search for related blocks */
            {
                let mut stack: Vec<usize> = Vec::new();

                /* to relation */
                stack.push(start_node_id);
                while let Some(now_id) = stack.pop() {
                    if let Some(tos) = map_from_to.get(&now_id) {
                        for to in tos.iter().rev() {
                            if !visited_id.contains(to) {
                                visited_id.insert(*to);
                                stack.push(*to);
                                component_ids.push(*to);
                            }
                        }
                    }
                }

                /* from relation */
                stack.push(start_node_id);
                while let Some(now_id) = stack.pop() {
                    if let Some(from) = map_to_from.get(&now_id) {
                        for from_id in from.iter().rev() {
                            if !visited_id.contains(from_id) {
                                visited_id.insert(*from_id);
                                stack.push(*from_id);
                                component_ids.push(*from_id);
                            }
                        }
                    }
                }
            }
            component_ids.sort_unstable_by_key(|id| {
                id_to_block
                    .get(id)
                    .unwrap()
                    .get_start_address()
                    .get_virtual_address()
            });
            let component_blocks: Vec<Arc<Block>> = component_ids
                .iter()
                .filter_map(|id| id_to_block.get(id).cloned())
                .cloned()
                .collect();

            debug!(
                "Generated block group: {:?}",
                component_blocks
                    .iter()
                    .map(|x| x.get_start_address().get_virtual_address())
                    .collect::<Vec<_>>()
            );
            block_groups.push(BlockGroup {
                blocks: component_blocks,
            });
        }
    }

    block_groups
}

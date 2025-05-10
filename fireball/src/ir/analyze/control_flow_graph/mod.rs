use crate::core::{Block, Relation};
use std::sync::Arc;

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
            let connected_to = target.get_connected_to();
            for relation in connected_to.iter() {
                let block_id = relation.from();
                if self.targets.iter().any(|x| x.get_id() == block_id) {
                    self.relations.push(relation.clone());
                }
            }
            let connected_from = target.get_connected_from();
            for relation in connected_from.iter() {
                let Some(to) = relation.to() else {
                    continue;
                };
                if self.targets.iter().any(|x| x.contains(&to)) {
                    self.relations.push(relation.clone());
                }
            }
        }
        self.targets.push(target);
    }
    pub fn add_targets(&mut self, targets: impl IntoIterator<Item = Arc<Block>>) {
        for target in targets {
            self.targets.push(target);
        }
    }
    pub fn get_targets(&self) -> &Vec<Arc<Block>> {
        &self.targets
    }
    pub fn get_analyzed(&self) -> Option<&Vec<ControlFlowGraph>> {
        self.analyzed.as_ref()
    }
    pub fn analyze(&mut self) {
        self.analyzed = Some(analyze_control_flow_graph(&self.targets, &self.relations));
    }
}

#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    blocks: Vec<Arc<Block>>,
}

pub fn analyze_control_flow_graph(
    blocks: &[Arc<Block>],
    relations: &[Relation],
) -> Vec<ControlFlowGraph> {
    todo!()
}

//! Complex loop analysis module for identifying and analyzing different loop patterns
#![allow(clippy::mutable_key_type)]
//!
//! This module provides advanced loop analysis capabilities including:
//! - Loop pattern recognition (for, while, do-while)
//! - Nested loop detection and analysis
//! - Loop invariant detection
//! - Iterator variable identification
//! - Range-based loop pattern matching

use crate::{
    core::Block,
    ir::{data::IrData, operator::BinaryOperator},
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use super::control_flow_graph::LoopInfo;

/// Represents different types of loop patterns
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum LoopPattern {
    /// Traditional for loop: for(init; condition; increment)
    ForLoop {
        init_block: Option<Arc<Block>>,
        condition_block: Arc<Block>,
        increment_block: Option<Arc<Block>>,
        body_blocks: Vec<Arc<Block>>,
    },
    /// While loop: while(condition)
    WhileLoop {
        condition_block: Arc<Block>,
        body_blocks: Vec<Arc<Block>>,
    },
    /// Do-while loop: do { } while(condition)
    DoWhileLoop {
        body_blocks: Vec<Arc<Block>>,
        condition_block: Arc<Block>,
    },
    /// Range-based/foreach loop
    RangeBasedLoop {
        iterator_var: LoopIterator,
        collection_ref: Option<IrData>,
        body_blocks: Vec<Arc<Block>>,
    },
    /// Generic loop that doesn't match specific patterns
    GenericLoop {
        entry_block: Arc<Block>,
        body_blocks: Vec<Arc<Block>>,
        exit_blocks: Vec<Arc<Block>>,
    },
}

/// Information about a loop iterator variable
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopIterator {
    /// The register or memory location used as iterator
    pub location: IrData,
    /// Initial value of the iterator
    pub init_value: Option<IrData>,
    /// Step/increment value
    pub step_value: Option<IrData>,
    /// Final/bound value
    pub bound_value: Option<IrData>,
    /// Comparison operator used in loop condition
    pub comparison: Option<BinaryOperator>,
}

/// Loop invariant - expressions that don't change within the loop
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopInvariant {
    /// The invariant expression
    pub expression: IrData,
    /// Blocks where this invariant is used
    pub used_in_blocks: HashSet<usize>,
}

/// Complete analysis results for a single loop
#[derive(Debug, Clone)]
pub struct AnalyzedLoop {
    /// Basic loop information from CFG
    pub loop_info: LoopInfo,
    /// Detected loop pattern
    pub pattern: LoopPattern,
    /// Detected iterator variables
    pub iterators: Vec<LoopIterator>,
    /// Loop invariants
    pub invariants: Vec<LoopInvariant>,
    /// Nested loops within this loop
    pub nested_loops: Vec<AnalyzedLoop>,
    /// Loop depth (0 for outermost loops)
    pub depth: usize,
}

/// Main loop analyzer
pub struct ComplexLoopAnalyzer {
    /// All blocks in the function
    blocks: HashMap<usize, Arc<Block>>,
    /// Control flow edges
    cfg_edges: HashMap<usize, Vec<usize>>,
    /// Reverse control flow edges
    reverse_cfg_edges: HashMap<usize, Vec<usize>>,
}

impl ComplexLoopAnalyzer {
    /// Create a new loop analyzer
    pub fn new(blocks: Vec<Arc<Block>>, relations: &[crate::core::Relation]) -> Self {
        let mut blocks_map = HashMap::new();
        let mut cfg_edges = HashMap::new();
        let mut reverse_cfg_edges = HashMap::new();

        // Build block map
        for block in blocks {
            blocks_map.insert(block.get_id(), block);
        }

        // Build CFG edges
        for relation in relations {
            let from_id = relation.from();
            if let Some(to_addr) = relation.to() {
                // Find block containing the target address
                for (block_id, block) in &blocks_map {
                    if block.contains(&to_addr) {
                        cfg_edges
                            .entry(from_id)
                            .or_insert_with(Vec::new)
                            .push(*block_id);
                        reverse_cfg_edges
                            .entry(*block_id)
                            .or_insert_with(Vec::new)
                            .push(from_id);
                        break;
                    }
                }
            }
        }

        Self {
            blocks: blocks_map,
            cfg_edges,
            reverse_cfg_edges,
        }
    }

    /// Analyze loops and detect complex patterns
    pub fn analyze_loops(&self, basic_loops: &[LoopInfo]) -> Vec<AnalyzedLoop> {
        let mut analyzed_loops = Vec::new();

        // First pass: analyze each loop independently
        for loop_info in basic_loops {
            if let Some(analyzed) = self.analyze_single_loop(loop_info) {
                analyzed_loops.push(analyzed);
            }
        }

        // Second pass: detect nested loops and set depth
        self.detect_nested_loops(&mut analyzed_loops);

        // Sort by depth (outermost first)
        analyzed_loops.sort_by_key(|l| l.depth);

        analyzed_loops
    }

    /// Analyze a single loop
    fn analyze_single_loop(&self, loop_info: &LoopInfo) -> Option<AnalyzedLoop> {
        let loop_blocks = self.get_loop_blocks(loop_info)?;
        let pattern = self.detect_loop_pattern(loop_info, &loop_blocks);
        let iterators = self.detect_iterators(&loop_blocks);
        let invariants = self.detect_invariants(&loop_blocks);

        Some(AnalyzedLoop {
            loop_info: loop_info.clone(),
            pattern,
            iterators,
            invariants,
            nested_loops: Vec::new(), // Will be filled in second pass
            depth: 0,                 // Will be set in second pass
        })
    }

    /// Get all blocks that belong to a loop
    fn get_loop_blocks(&self, loop_info: &LoopInfo) -> Option<HashSet<Arc<Block>>> {
        let mut loop_blocks = HashSet::new();
        let loop_head_id = loop_info.loop_from.get_id();
        let loop_tail_id = loop_info.loop_to.get_id();

        // Add loop head and tail
        loop_blocks.insert(loop_info.loop_from.clone());
        loop_blocks.insert(loop_info.loop_to.clone());

        // Find all blocks dominated by loop head and that can reach loop tail
        // This is a simplified version - a full implementation would use dominator analysis
        let mut visited = HashSet::new();
        let mut stack = vec![loop_head_id];

        while let Some(block_id) = stack.pop() {
            if visited.contains(&block_id) {
                continue;
            }
            visited.insert(block_id);

            if let Some(block) = self.blocks.get(&block_id) {
                loop_blocks.insert(block.clone());
            }

            if let Some(successors) = self.cfg_edges.get(&block_id) {
                for &succ_id in successors {
                    if !visited.contains(&succ_id) {
                        // Check if this successor can reach the loop tail
                        if self.can_reach(succ_id, loop_tail_id) {
                            stack.push(succ_id);
                        }
                    }
                }
            }
        }

        Some(loop_blocks)
    }

    /// Check if block `from` can reach block `to`
    fn can_reach(&self, from: usize, to: usize) -> bool {
        if from == to {
            return true;
        }

        let mut visited = HashSet::new();
        let mut stack = vec![from];

        while let Some(block_id) = stack.pop() {
            if block_id == to {
                return true;
            }

            if visited.contains(&block_id) {
                continue;
            }
            visited.insert(block_id);

            if let Some(successors) = self.cfg_edges.get(&block_id) {
                for &succ_id in successors {
                    if !visited.contains(&succ_id) {
                        stack.push(succ_id);
                    }
                }
            }
        }

        false
    }

    /// Detect the pattern of a loop
    fn detect_loop_pattern(
        &self,
        loop_info: &LoopInfo,
        loop_blocks: &HashSet<Arc<Block>>,
    ) -> LoopPattern {
        let loop_head_id = loop_info.loop_from.get_id();
        let loop_tail_id = loop_info.loop_to.get_id();

        // Check for do-while pattern: loop tail jumps back to loop head
        if self
            .cfg_edges
            .get(&loop_tail_id)
            .map(|succs| succs.contains(&loop_head_id))
            .unwrap_or(false)
        {
            let body_blocks: Vec<_> = loop_blocks
                .iter()
                .filter(|b| b.get_id() != loop_tail_id)
                .cloned()
                .collect();

            return LoopPattern::DoWhileLoop {
                body_blocks,
                condition_block: loop_info.loop_to.clone(),
            };
        }

        // Check for while/for loop pattern
        if let Some(head_block) = self.blocks.get(&loop_head_id) {
            // Look for condition check at loop head
            if self.has_conditional_jump(head_block) {
                let body_blocks: Vec<_> = loop_blocks
                    .iter()
                    .filter(|b| b.get_id() != loop_head_id)
                    .cloned()
                    .collect();

                // Try to detect for loop pattern by looking for initialization and increment
                if let Some(for_pattern) = self.try_detect_for_loop(loop_info, &body_blocks) {
                    return for_pattern;
                }

                // Otherwise it's a while loop
                return LoopPattern::WhileLoop {
                    condition_block: head_block.clone(),
                    body_blocks,
                };
            }
        }

        // Generic loop pattern
        let body_blocks: Vec<_> = loop_blocks.iter().cloned().collect();
        let exit_blocks = self.find_loop_exits(loop_blocks);

        LoopPattern::GenericLoop {
            entry_block: loop_info.loop_from.clone(),
            body_blocks,
            exit_blocks,
        }
    }

    /// Check if a block contains a conditional jump
    fn has_conditional_jump(&self, block: &Arc<Block>) -> bool {
        // This would need to examine the IR statements in the block
        // For now, we'll use a simplified check based on the number of successors
        self.cfg_edges
            .get(&block.get_id())
            .map(|succs| succs.len() == 2)
            .unwrap_or(false)
    }

    /// Try to detect a for loop pattern
    fn try_detect_for_loop(
        &self,
        _loop_info: &LoopInfo,
        _body_blocks: &[Arc<Block>],
    ) -> Option<LoopPattern> {
        // This is a placeholder - full implementation would analyze:
        // 1. Predecessor of loop head for initialization
        // 2. Loop body for increment operations
        // 3. Loop condition for iterator comparison
        None
    }

    /// Find blocks that exit the loop
    fn find_loop_exits(&self, loop_blocks: &HashSet<Arc<Block>>) -> Vec<Arc<Block>> {
        let mut exit_blocks = Vec::new();
        let loop_block_ids: HashSet<_> = loop_blocks.iter().map(|b| b.get_id()).collect();

        for block in loop_blocks {
            if let Some(successors) = self.cfg_edges.get(&block.get_id()) {
                for &succ_id in successors {
                    if !loop_block_ids.contains(&succ_id) {
                        if let Some(exit_block) = self.blocks.get(&succ_id) {
                            exit_blocks.push(exit_block.clone());
                        }
                    }
                }
            }
        }

        exit_blocks
    }

    /// Detect iterator variables in the loop
    fn detect_iterators(&self, _loop_blocks: &HashSet<Arc<Block>>) -> Vec<LoopIterator> {
        // Placeholder implementation
        // Full implementation would:
        // 1. Look for variables that are initialized before/at loop entry
        // 2. Modified in a regular pattern within the loop
        // 3. Used in loop condition
        Vec::new()
    }

    /// Detect loop invariants
    fn detect_invariants(&self, _loop_blocks: &HashSet<Arc<Block>>) -> Vec<LoopInvariant> {
        // Placeholder implementation
        // Full implementation would:
        // 1. Analyze all expressions in the loop
        // 2. Check which variables are modified within the loop
        // 3. Identify expressions that don't depend on modified variables
        Vec::new()
    }

    /// Detect nested loops and set depth levels
    fn detect_nested_loops(&self, loops: &mut [AnalyzedLoop]) {
        let n = loops.len();
        let mut nesting_graph = vec![vec![false; n]; n];

        // Build nesting relationship graph
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let loop_i_blocks: HashSet<_> = loops[i]
                        .pattern
                        .get_all_blocks()
                        .iter()
                        .map(|b| b.get_id())
                        .collect();
                    let loop_j_head = loops[j].loop_info.loop_from.get_id();

                    // Loop j is nested in loop i if loop j's head is in loop i's blocks
                    if loop_i_blocks.contains(&loop_j_head) {
                        nesting_graph[i][j] = true;
                    }
                }
            }
        }

        // Calculate depths using topological sort
        let mut depths = vec![0; n];
        let mut processed = vec![false; n];

        fn calculate_depth(
            idx: usize,
            nesting_graph: &[Vec<bool>],
            depths: &mut [usize],
            processed: &mut [bool],
        ) {
            if processed[idx] {
                return;
            }

            let mut max_parent_depth = 0;
            for i in 0..nesting_graph.len() {
                if nesting_graph[i][idx] {
                    calculate_depth(i, nesting_graph, depths, processed);
                    max_parent_depth = max_parent_depth.max(depths[i] + 1);
                }
            }

            depths[idx] = max_parent_depth;
            processed[idx] = true;
        }

        for i in 0..n {
            calculate_depth(i, &nesting_graph, &mut depths, &mut processed);
        }

        // Set depths
        for (i, loop_) in loops.iter_mut().enumerate() {
            loop_.depth = depths[i];
        }
    }
}

impl LoopPattern {
    /// Get all blocks in the loop
    pub fn get_all_blocks(&self) -> Vec<Arc<Block>> {
        match self {
            LoopPattern::ForLoop {
                init_block,
                condition_block,
                increment_block,
                body_blocks,
            } => {
                let mut blocks = vec![condition_block.clone()];
                if let Some(init) = init_block {
                    blocks.push(init.clone());
                }
                if let Some(inc) = increment_block {
                    blocks.push(inc.clone());
                }
                blocks.extend(body_blocks.clone());
                blocks
            }
            LoopPattern::WhileLoop {
                condition_block,
                body_blocks,
            } => {
                let mut blocks = vec![condition_block.clone()];
                blocks.extend(body_blocks.clone());
                blocks
            }
            LoopPattern::DoWhileLoop {
                body_blocks,
                condition_block,
            } => {
                let mut blocks = body_blocks.clone();
                blocks.push(condition_block.clone());
                blocks
            }
            LoopPattern::RangeBasedLoop { body_blocks, .. } => body_blocks.clone(),
            LoopPattern::GenericLoop {
                entry_block,
                body_blocks,
                exit_blocks,
            } => {
                let mut blocks = vec![entry_block.clone()];
                blocks.extend(body_blocks.clone());
                blocks.extend(exit_blocks.clone());
                blocks
            }
        }
    }
}

//! Pattern matching implementations for Medium IR

use super::*;
use crate::ir::low_ir;
use std::collections::{BTreeMap, BTreeSet};

/// Loop pattern detector
pub struct LoopPatternDetector;

impl LoopPatternDetector {
    /// Detect all types of loops in a function
    pub fn detect_loops(func: &low_ir::Function) -> Vec<LoopPattern> {
        let mut loops = Vec::new();

        // Find natural loops using dominator analysis
        let dominators = compute_dominators(func);
        let back_edges = find_back_edges(func, &dominators);

        for (header, tail) in back_edges {
            if let Some(loop_pattern) = Self::analyze_loop(&header, &tail, func) {
                loops.push(loop_pattern);
            }
        }

        loops
    }

    /// Analyze a single loop
    fn analyze_loop(
        header: &low_ir::BlockId,
        tail: &low_ir::BlockId,
        func: &low_ir::Function,
    ) -> Option<LoopPattern> {
        let header_block = func.blocks.get(header)?;
        let tail_block = func.blocks.get(tail)?;

        // Collect loop blocks
        let loop_blocks = Self::collect_loop_blocks(header, tail, func);

        // Identify loop type
        if let Some(for_loop) = Self::try_match_for_loop(header_block, &loop_blocks, func) {
            Some(LoopPattern::For(for_loop))
        } else if let Some(while_loop) =
            Self::try_match_while_loop(header_block, &loop_blocks, func)
        {
            Some(LoopPattern::While(while_loop))
        } else if let Some(do_while) = Self::try_match_do_while_loop(tail_block, &loop_blocks, func)
        {
            Some(LoopPattern::DoWhile(do_while))
        } else {
            None
        }
    }

    /// Try to match a for loop pattern
    fn try_match_for_loop(
        header: &low_ir::BasicBlock,
        loop_blocks: &BTreeSet<low_ir::BlockId>,
        func: &low_ir::Function,
    ) -> Option<ForLoopPattern> {
        // For loop characteristics:
        // 1. Initialization before loop
        // 2. Condition check at header
        // 3. Increment at end of body

        // Find predecessors of header
        let predecessors = Self::find_predecessors(&header.id, func);

        // Look for initialization in immediate predecessor
        let init_block = predecessors
            .iter()
            .filter(|pred_id| !loop_blocks.contains(pred_id))
            .filter_map(|pred_id| func.blocks.get(pred_id))
            .find(|block| Self::contains_initialization(block))?;

        // Extract condition from header
        let condition = Self::extract_loop_condition(header)?;

        // Find increment operation
        let increment = Self::find_increment_operation(loop_blocks, func)?;

        Some(ForLoopPattern {
            init: Self::extract_initialization(init_block),
            condition,
            increment,
            body_blocks: loop_blocks.clone(),
        })
    }

    /// Try to match a while loop pattern
    fn try_match_while_loop(
        header: &low_ir::BasicBlock,
        loop_blocks: &BTreeSet<low_ir::BlockId>,
        func: &low_ir::Function,
    ) -> Option<WhileLoopPattern> {
        // While loop: condition at header, no specific increment
        let condition = Self::extract_loop_condition(header)?;

        Some(WhileLoopPattern {
            condition,
            body_blocks: loop_blocks.clone(),
        })
    }

    /// Try to match a do-while loop pattern
    fn try_match_do_while_loop(
        tail: &low_ir::BasicBlock,
        loop_blocks: &BTreeSet<low_ir::BlockId>,
        func: &low_ir::Function,
    ) -> Option<DoWhileLoopPattern> {
        // Do-while: condition at tail
        let condition = Self::extract_loop_condition(tail)?;

        Some(DoWhileLoopPattern {
            condition,
            body_blocks: loop_blocks.clone(),
        })
    }

    /// Collect all blocks in a loop
    fn collect_loop_blocks(
        header: &low_ir::BlockId,
        tail: &low_ir::BlockId,
        func: &low_ir::Function,
    ) -> BTreeSet<low_ir::BlockId> {
        let mut loop_blocks = BTreeSet::new();
        let mut worklist = vec![tail.clone()];

        loop_blocks.insert(header.clone());

        while let Some(block_id) = worklist.pop() {
            if loop_blocks.insert(block_id.clone()) {
                // Add predecessors to worklist
                for pred in Self::find_predecessors(&block_id, func) {
                    if pred != *header && !loop_blocks.contains(&pred) {
                        worklist.push(pred);
                    }
                }
            }
        }

        loop_blocks
    }

    /// Find predecessors of a block
    fn find_predecessors(
        target: &low_ir::BlockId,
        func: &low_ir::Function,
    ) -> Vec<low_ir::BlockId> {
        let mut predecessors = Vec::new();

        for (block_id, block) in &func.blocks {
            match &block.terminator {
                low_ir::Terminator::Branch(dest) => {
                    if dest == target {
                        predecessors.push(block_id.clone());
                    }
                }
                low_ir::Terminator::CondBranch {
                    true_dest,
                    false_dest,
                    ..
                } => {
                    if true_dest == target || false_dest == target {
                        predecessors.push(block_id.clone());
                    }
                }
                low_ir::Terminator::Switch { cases, default, .. } => {
                    if default == target {
                        predecessors.push(block_id.clone());
                    }
                    for (_, dest) in cases {
                        if dest == target {
                            predecessors.push(block_id.clone());
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        predecessors
    }

    /// Check if block contains initialization pattern
    fn contains_initialization(block: &low_ir::BasicBlock) -> bool {
        // Look for assignment to loop variable
        block
            .instructions
            .iter()
            .any(|inst| matches!(inst, low_ir::Instruction::Assign { .. }))
    }

    /// Extract loop condition from block
    fn extract_loop_condition(block: &low_ir::BasicBlock) -> Option<LoopCondition> {
        match &block.terminator {
            low_ir::Terminator::CondBranch { cond, .. } => {
                Some(LoopCondition {
                    value: cond.clone(),
                    comparison: ComparisonOp::NotEqual, // TODO: Determine actual comparison
                })
            }
            _ => None,
        }
    }

    /// Extract initialization from block
    fn extract_initialization(block: &low_ir::BasicBlock) -> LoopInit {
        // Find assignment instructions
        let assignments: Vec<_> = block
            .instructions
            .iter()
            .filter_map(|inst| {
                if let low_ir::Instruction::Assign { dst, value, .. } = inst {
                    Some((dst.clone(), value.clone()))
                } else {
                    None
                }
            })
            .collect();

        LoopInit { assignments }
    }

    /// Find increment operation in loop
    fn find_increment_operation(
        loop_blocks: &BTreeSet<low_ir::BlockId>,
        func: &low_ir::Function,
    ) -> Option<LoopIncrement> {
        // Look for add/sub operations on loop variables
        for block_id in loop_blocks {
            if let Some(block) = func.blocks.get(block_id) {
                for inst in &block.instructions {
                    if let low_ir::Instruction::BinOp {
                        op, dst, lhs, rhs, ..
                    } = inst
                    {
                        match op {
                            low_ir::BinaryOp::Add | low_ir::BinaryOp::Sub => {
                                // Check if this modifies a potential loop variable
                                if let low_ir::Value::Local(local) = lhs {
                                    if local == dst {
                                        return Some(LoopIncrement {
                                            variable: dst.clone(),
                                            operation: *op,
                                            value: rhs.clone(),
                                        });
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        None
    }
}

/// Function call pattern detector
pub struct CallPatternDetector;

impl CallPatternDetector {
    /// Detect library function calls
    pub fn detect_library_calls(
        func: &low_ir::Function,
        pattern_db: &PatternDatabase,
    ) -> Vec<LibraryCallPattern> {
        let mut calls = Vec::new();

        for block in func.blocks.values() {
            for inst in &block.instructions {
                if let low_ir::Instruction::Call {
                    func: target,
                    args,
                    dst,
                    conv,
                } = inst
                {
                    if let Some(lib_call) = Self::match_library_call(target, args, pattern_db) {
                        calls.push(lib_call);
                    }
                }
            }
        }

        calls
    }

    /// Match a call against known library functions
    fn match_library_call(
        target: &low_ir::Value,
        args: &[(low_ir::Value, low_ir::Type)],
        pattern_db: &PatternDatabase,
    ) -> Option<LibraryCallPattern> {
        // TODO: Implement library function matching
        // This would involve:
        // 1. Resolving the target address
        // 2. Looking up in import tables
        // 3. Matching against known signatures

        None
    }
}

/// Array access pattern detector
pub struct ArrayPatternDetector;

impl ArrayPatternDetector {
    /// Detect array access patterns
    pub fn detect_array_accesses(func: &low_ir::Function) -> Vec<ArrayAccessPattern> {
        let mut accesses = Vec::new();

        for block in func.blocks.values() {
            for inst in &block.instructions {
                if let Some(access) = Self::match_array_access(inst) {
                    accesses.push(access);
                }
            }
        }

        accesses
    }

    /// Match an instruction as array access
    fn match_array_access(inst: &low_ir::Instruction) -> Option<ArrayAccessPattern> {
        // Pattern: base + index * element_size
        match inst {
            low_ir::Instruction::Load { ptr, ty, .. } => {
                // Check if ptr is result of address calculation
                // TODO: Trace back ptr calculation
                None
            }
            _ => None,
        }
    }
}

// Pattern data structures

#[derive(Debug, Clone)]
pub enum LoopPattern {
    For(ForLoopPattern),
    While(WhileLoopPattern),
    DoWhile(DoWhileLoopPattern),
}

#[derive(Debug, Clone)]
pub struct ForLoopPattern {
    pub init: LoopInit,
    pub condition: LoopCondition,
    pub increment: LoopIncrement,
    pub body_blocks: BTreeSet<low_ir::BlockId>,
}

#[derive(Debug, Clone)]
pub struct WhileLoopPattern {
    pub condition: LoopCondition,
    pub body_blocks: BTreeSet<low_ir::BlockId>,
}

#[derive(Debug, Clone)]
pub struct DoWhileLoopPattern {
    pub condition: LoopCondition,
    pub body_blocks: BTreeSet<low_ir::BlockId>,
}

#[derive(Debug, Clone)]
pub struct LoopInit {
    pub assignments: Vec<(low_ir::LocalId, low_ir::Value)>,
}

#[derive(Debug, Clone)]
pub struct LoopCondition {
    pub value: low_ir::Value,
    pub comparison: ComparisonOp,
}

#[derive(Debug, Clone)]
pub struct LoopIncrement {
    pub variable: low_ir::LocalId,
    pub operation: low_ir::BinaryOp,
    pub value: low_ir::Value,
}

#[derive(Debug, Clone, Copy)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone)]
pub struct LibraryCallPattern {
    pub function: String,
    pub library: String,
    pub arguments: Vec<low_ir::Value>,
    pub return_value: Option<low_ir::LocalId>,
}

#[derive(Debug, Clone)]
pub struct ArrayAccessPattern {
    pub base: low_ir::Value,
    pub index: low_ir::Value,
    pub element_size: usize,
    pub access_type: AccessType,
}

#[derive(Debug, Clone, Copy)]
pub enum AccessType {
    Read,
    Write,
}

// Dominator analysis helpers

/// Compute dominators for all blocks
fn compute_dominators(
    func: &low_ir::Function,
) -> BTreeMap<low_ir::BlockId, BTreeSet<low_ir::BlockId>> {
    let mut dominators = BTreeMap::new();

    // Entry block dominates itself
    let entry = &func.entry;
    let mut entry_doms = BTreeSet::new();
    entry_doms.insert(entry.clone());
    dominators.insert(entry.clone(), entry_doms);

    // All other blocks are initially dominated by all blocks
    let all_blocks: BTreeSet<_> = func.blocks.keys().cloned().collect();
    for block_id in func.blocks.keys() {
        if block_id != entry {
            dominators.insert(block_id.clone(), all_blocks.clone());
        }
    }

    // Iterate until fixpoint
    let mut changed = true;
    while changed {
        changed = false;

        for (block_id, _block) in &func.blocks {
            if block_id == entry {
                continue;
            }

            // Find predecessors
            let predecessors = LoopPatternDetector::find_predecessors(block_id, func);

            if !predecessors.is_empty() {
                // New dominators = intersection of predecessor dominators + self
                let mut new_doms = all_blocks.clone();

                for pred in &predecessors {
                    if let Some(pred_doms) = dominators.get(pred) {
                        new_doms = new_doms.intersection(pred_doms).cloned().collect();
                    }
                }

                new_doms.insert(block_id.clone());

                if dominators.get(block_id) != Some(&new_doms) {
                    dominators.insert(block_id.clone(), new_doms);
                    changed = true;
                }
            }
        }
    }

    dominators
}

/// Find back edges in the control flow graph
fn find_back_edges(
    func: &low_ir::Function,
    dominators: &BTreeMap<low_ir::BlockId, BTreeSet<low_ir::BlockId>>,
) -> Vec<(low_ir::BlockId, low_ir::BlockId)> {
    let mut back_edges = Vec::new();

    for (block_id, block) in &func.blocks {
        let successors = match &block.terminator {
            low_ir::Terminator::Branch(target) => vec![target.clone()],
            low_ir::Terminator::CondBranch {
                true_dest,
                false_dest,
                ..
            } => {
                vec![true_dest.clone(), false_dest.clone()]
            }
            low_ir::Terminator::Switch { cases, default, .. } => {
                let mut targets: Vec<_> = cases.values().cloned().collect();
                targets.push(default.clone());
                targets
            }
            _ => vec![],
        };

        for successor in successors {
            // A back edge is an edge where the target dominates the source
            if let Some(successor_doms) = dominators.get(&successor) {
                if successor_doms.contains(block_id) {
                    back_edges.push((successor, block_id.clone()));
                }
            }
        }
    }

    back_edges
}

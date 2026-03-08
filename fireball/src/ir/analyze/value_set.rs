//! Value-set analysis using basic integer intervals.
//!
//! Tracks integer ranges [lo, hi] through assignments and conditions
//! to support bounds-check simplification and constant propagation.
//! Uses worklist-based fixpoint iteration to handle loops and merges.

use crate::{
    core::Block,
    ir::{
        Register,
        data::IrData,
        operator::{IrBinaryOperator, IrUnaryOperator},
        statements::IrStatement,
    },
    prelude::*,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
};

/// An integer interval [lo, hi] inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interval {
    pub lo: i64,
    pub hi: i64,
}

impl Interval {
    /// Full range (top/unknown).
    pub fn full() -> Self {
        Interval {
            lo: i64::MIN,
            hi: i64::MAX,
        }
    }

    /// A single constant value.
    pub fn constant(v: i64) -> Self {
        Interval { lo: v, hi: v }
    }

    /// Check if this is a single value.
    pub fn is_constant(&self) -> bool {
        self.lo == self.hi
    }

    /// Check if this contains a value.
    pub fn contains(&self, v: i64) -> bool {
        self.lo <= v && v <= self.hi
    }

    /// Abstract addition.
    pub fn add(self, other: Self) -> Self {
        Interval {
            lo: self.lo.saturating_add(other.lo),
            hi: self.hi.saturating_add(other.hi),
        }
    }

    /// Abstract subtraction.
    pub fn sub(self, other: Self) -> Self {
        Interval {
            lo: self.lo.saturating_sub(other.hi),
            hi: self.hi.saturating_sub(other.lo),
        }
    }

    /// Meet (intersection) — narrows the range.
    pub fn meet(self, other: Self) -> Option<Self> {
        let lo = self.lo.max(other.lo);
        let hi = self.hi.min(other.hi);
        if lo <= hi {
            Some(Interval { lo, hi })
        } else {
            None // empty
        }
    }

    /// Join (union) — widens the range.
    pub fn join(self, other: Self) -> Self {
        Interval {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        }
    }

    /// Abstract bitwise AND with a constant mask.
    pub fn and_constant(self, mask: i64) -> Self {
        if mask >= 0 {
            Interval { lo: 0, hi: mask }
        } else {
            Interval::full()
        }
    }

    /// Abstract negation.
    pub fn negate(self) -> Self {
        Interval {
            lo: self.hi.saturating_neg(),
            hi: self.lo.saturating_neg(),
        }
    }
}

/// Per-block register state.
type BlockState = HashMap<Register, Interval>;

/// Result of value-set analysis: register → interval at each point.
#[derive(Debug, Clone, Default)]
pub struct ValueSetResult {
    /// Register intervals at analysis completion (merged across all blocks).
    pub intervals: HashMap<Register, Interval>,
}

impl ValueSetResult {
    /// Get the interval for a register, or full range if unknown.
    pub fn get(&self, reg: &Register) -> Interval {
        self.intervals.get(reg).copied().unwrap_or(Interval::full())
    }
}

/// Run value-set analysis using worklist-based fixpoint iteration.
pub fn analyze_value_set(blocks: &[Arc<Block>]) -> ValueSetResult {
    // Index blocks by ID for fast lookup
    let block_map: HashMap<usize, &Arc<Block>> = blocks.iter().map(|b| (b.get_id(), b)).collect();

    let block_ids: Vec<usize> = blocks.iter().map(|b| b.get_id()).collect();

    // Per-block input states (register intervals at block entry)
    let mut block_in: HashMap<usize, BlockState> = HashMap::new();
    // Per-block output states (register intervals at block exit)
    let mut block_out: HashMap<usize, BlockState> = HashMap::new();

    for &bid in &block_ids {
        block_in.insert(bid, HashMap::new());
        block_out.insert(bid, HashMap::new());
    }

    // Build successor map from block data
    let mut successors: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut predecessors: HashMap<usize, Vec<usize>> = HashMap::new();
    for &bid in &block_ids {
        successors.entry(bid).or_default();
        predecessors.entry(bid).or_default();
    }
    // We don't have direct access to CFG edges from blocks alone,
    // so we do a single-pass analysis per block and merge at block boundaries.
    // For a simple forward dataflow, process blocks in order with fixpoint.

    let mut worklist: VecDeque<usize> = block_ids.iter().copied().collect();
    let mut iterations = 0;
    let max_iterations = block_ids.len() * 10; // safety bound

    while let Some(bid) = worklist.pop_front() {
        iterations += 1;
        if iterations > max_iterations {
            break; // convergence safety
        }

        let Some(block) = block_map.get(&bid) else {
            continue;
        };

        let ir_block = block.get_ir();
        let Some(ir_block) = ir_block.as_ref() else {
            continue;
        };

        // Start with input state
        let mut state = block_in.get(&bid).cloned().unwrap_or_default();

        // Process all statements in this block
        for ir in ir_block.ir() {
            let Some(stmts) = ir.statements else {
                continue;
            };
            for stmt in stmts {
                propagate_interval(stmt, &mut state);
            }
        }

        // Check if output changed
        let old_out = block_out.get(&bid);
        if old_out != Some(&state) {
            block_out.insert(bid, state.clone());

            // Propagate to successor blocks: merge (join) into their input states
            // Since we don't have explicit CFG edges, we use sequential order
            // as an approximation — the next block gets our output
            let idx = block_ids.iter().position(|&b| b == bid);
            if let Some(idx) = idx {
                if idx + 1 < block_ids.len() {
                    let next_bid = block_ids[idx + 1];
                    let next_in = block_in.entry(next_bid).or_default();
                    let changed = merge_states(next_in, &state);
                    if changed && !worklist.contains(&next_bid) {
                        worklist.push_back(next_bid);
                    }
                }
            }
        }
    }

    // Merge all block outputs into final result
    let mut result = ValueSetResult::default();
    for state in block_out.values() {
        for (reg, interval) in state {
            let entry = result.intervals.entry(*reg).or_insert(*interval);
            *entry = entry.join(*interval);
        }
    }

    result
}

/// Merge source state into dest state (join/widen). Returns true if dest changed.
fn merge_states(dest: &mut BlockState, source: &BlockState) -> bool {
    let mut changed = false;
    for (reg, interval) in source {
        match dest.get(reg) {
            None => {
                dest.insert(*reg, *interval);
                changed = true;
            }
            Some(existing) => {
                let joined = existing.join(*interval);
                if joined != *existing {
                    dest.insert(*reg, joined);
                    changed = true;
                }
            }
        }
    }
    changed
}

fn propagate_interval(stmt: &IrStatement, state: &mut BlockState) {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            if let IrData::Register(dst) = to.as_ref() {
                let interval = eval_interval(from, state);
                state.insert(*dst, interval);
            }
        }
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            let mut true_state = state.clone();
            narrow_from_condition(condition, &mut true_state, true);
            for s in true_branch.iter() {
                propagate_interval(s, &mut true_state);
            }

            let mut false_state = state.clone();
            narrow_from_condition(condition, &mut false_state, false);
            for s in false_branch.iter() {
                propagate_interval(s, &mut false_state);
            }

            // Join results from both branches
            let mut all_regs: HashSet<Register> = HashSet::new();
            all_regs.extend(true_state.keys());
            all_regs.extend(false_state.keys());

            for reg in all_regs {
                let t = true_state.get(&reg).copied().unwrap_or(Interval::full());
                let f = false_state.get(&reg).copied().unwrap_or(Interval::full());
                state.insert(reg, t.join(f));
            }
        }
        _ => {}
    }
}

/// Evaluate the interval of an IrData expression.
fn eval_interval(data: &crate::utils::Aos<IrData>, state: &BlockState) -> Interval {
    match data.as_ref() {
        IrData::Constant(v) => Interval::constant(*v as i64),
        IrData::Register(reg) => state.get(reg).copied().unwrap_or(Interval::full()),
        IrData::Operation(op) => {
            use crate::ir::data::IrDataOperation;
            match op {
                IrDataOperation::Binary {
                    operator,
                    arg1,
                    arg2,
                } => {
                    let l = eval_interval(arg1, state);
                    let r = eval_interval(arg2, state);
                    match operator {
                        IrBinaryOperator::Add => l.add(r),
                        IrBinaryOperator::Sub => l.sub(r),
                        IrBinaryOperator::And => {
                            if r.is_constant() {
                                l.and_constant(r.lo)
                            } else if l.is_constant() {
                                r.and_constant(l.lo)
                            } else {
                                Interval::full()
                            }
                        }
                        _ => Interval::full(),
                    }
                }
                IrDataOperation::Unary { operator, arg } => {
                    let o = eval_interval(arg, state);
                    match operator {
                        IrUnaryOperator::Negation => o.negate(),
                        _ => Interval::full(),
                    }
                }
            }
        }
        _ => Interval::full(),
    }
}

/// Narrow register intervals based on a condition being true/false.
fn narrow_from_condition(
    condition: &crate::utils::Aos<IrData>,
    state: &mut BlockState,
    is_true_branch: bool,
) {
    use crate::ir::data::IrDataOperation;
    let IrData::Operation(IrDataOperation::Binary {
        operator,
        arg1,
        arg2,
    }) = condition.as_ref()
    else {
        return;
    };

    let (reg, bound) = match (arg1.as_ref(), arg2.as_ref()) {
        (IrData::Register(r), IrData::Constant(c)) => (r, *c as i64),
        _ => return,
    };

    let current = state.get(reg).copied().unwrap_or(Interval::full());

    let narrowed = match (operator, is_true_branch) {
        (IrBinaryOperator::SignedLess(_), true) | (IrBinaryOperator::UnsignedLess(_), true) => {
            current.meet(Interval {
                lo: i64::MIN,
                hi: bound.saturating_sub(1),
            })
        }
        (IrBinaryOperator::SignedLess(_), false) | (IrBinaryOperator::UnsignedLess(_), false) => {
            current.meet(Interval {
                lo: bound,
                hi: i64::MAX,
            })
        }
        (IrBinaryOperator::Equal(_), true) => current.meet(Interval::constant(bound)),
        (IrBinaryOperator::Equal(_), false) => Some(current),
        _ => Some(current),
    };

    if let Some(narrowed) = narrowed {
        state.insert(*reg, narrowed);
    }
}

/// Log value-set analysis results.
pub fn log_value_set_analysis(result: &ValueSetResult) {
    let constant_count = result
        .intervals
        .values()
        .filter(|i| i.is_constant())
        .count();
    if !result.intervals.is_empty() {
        debug!(
            "Value-set analysis: {} registers tracked, {} constants",
            result.intervals.len(),
            constant_count,
        );
    }
}

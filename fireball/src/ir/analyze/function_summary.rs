//! Function summary extraction for interprocedural analysis.
//!
//! Produces a lightweight summary of a function's observable behavior:
//! which registers it reads/writes, whether it returns, side effects, call targets.

use crate::{
    core::Block,
    ir::{
        Register,
        data::{IrData, IrDataContainable},
        statements::IrStatement,
    },
    prelude::*,
    utils::Aos,
};
use std::{collections::HashSet, sync::Arc};

/// Summary of a function's externally observable behavior.
#[derive(Debug, Clone, Default)]
pub struct FunctionSummary {
    /// Registers read before being written (inputs/parameters).
    pub reads: HashSet<Register>,
    /// Registers written (outputs/clobbers).
    pub writes: HashSet<Register>,
    /// Whether the function returns normally (contains a Halt/ret).
    pub returns: bool,
    /// Whether the function has observable side effects (memory writes, calls).
    pub has_side_effects: bool,
    /// Direct call targets (addresses).
    pub callees: Vec<u64>,
    /// Whether the function has no side effects and makes no calls.
    /// Note: this is a conservative approximation — it does NOT guarantee
    /// referential transparency (e.g., reads from globals are not tracked).
    pub is_side_effect_free: bool,
}

/// Extract a function summary from a set of blocks.
pub fn summarize_function(blocks: &[Arc<Block>]) -> FunctionSummary {
    let mut summary = FunctionSummary::default();

    // Per-block analysis: collect reads/writes per block, then merge.
    // A register is an "input" if ANY block reads it before all paths write it.
    // Conservative: we union all per-block read-before-write sets.
    let mut all_writes: HashSet<Register> = HashSet::new();

    for block in blocks {
        let ir_block = block.get_ir();
        let Some(ir_block) = ir_block.as_ref() else {
            continue;
        };

        // Per-block tracking: reads before any write within this block
        let mut block_writes: HashSet<Register> = HashSet::new();

        for ir in ir_block.ir() {
            let Some(stmts) = ir.statements else {
                continue;
            };
            for stmt in stmts {
                process_statement(stmt, &mut summary, &mut block_writes);
            }
        }

        all_writes.extend(block_writes);
    }

    summary.writes = all_writes;
    summary.is_side_effect_free = !summary.has_side_effects && summary.callees.is_empty();

    summary
}

fn process_statement(
    stmt: &IrStatement,
    summary: &mut FunctionSummary,
    block_writes: &mut HashSet<Register>,
) {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            // Collect reads from source — registers read before written in this block
            collect_reads(from, summary, block_writes);

            // Collect writes to destination
            match to.as_ref() {
                IrData::Register(reg) => {
                    block_writes.insert(*reg);
                }
                IrData::Dereference(_) => {
                    // Memory write = side effect
                    summary.has_side_effects = true;
                    collect_reads(to, summary, block_writes);
                }
                _ => {}
            }
        }
        IrStatement::JumpByCall { target } => {
            summary.has_side_effects = true;
            // Track reads from the call target (e.g. indirect call through register)
            collect_reads(target, summary, block_writes);
            if let IrData::Constant(addr) = target.as_ref() {
                summary.callees.push(*addr as u64);
            }
        }
        IrStatement::Halt => {
            summary.returns = true;
        }
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            collect_reads(condition, summary, block_writes);

            // Process branches with separate write sets, then intersect
            // (only registers written on ALL paths are truly "written before read")
            let mut true_writes = block_writes.clone();
            let mut false_writes = block_writes.clone();

            for s in true_branch.iter() {
                process_statement(s, summary, &mut true_writes);
            }
            for s in false_branch.iter() {
                process_statement(s, summary, &mut false_writes);
            }

            // After a branch, only registers written in BOTH branches
            // are guaranteed written (intersection for conservative analysis)
            let intersection: HashSet<Register> =
                true_writes.intersection(&false_writes).copied().collect();
            *block_writes = intersection;
        }
        _ => {}
    }
}

fn collect_reads(
    data: &Aos<IrData>,
    summary: &mut FunctionSummary,
    block_writes: &HashSet<Register>,
) {
    match data.as_ref() {
        IrData::Register(reg) => {
            // Only count as an input if not already written in this block
            if !block_writes.contains(reg) {
                summary.reads.insert(*reg);
            }
        }
        IrData::Dereference(inner) => {
            collect_reads(inner, summary, block_writes);
        }
        IrData::Operation(_) => {
            let mut related = Vec::new();
            data.get_related_ir_data(&mut related);
            for r in related {
                if let IrData::Register(reg) = r.as_ref() {
                    if !block_writes.contains(reg) {
                        summary.reads.insert(*reg);
                    }
                }
            }
        }
        _ => {}
    }
}

/// Log function summary analysis results.
pub fn log_function_summary(blocks: &[Arc<Block>]) {
    let summary = summarize_function(blocks);
    debug!(
        "Function summary: reads={}, writes={}, returns={}, side_effects={}, callees={}, side_effect_free={}",
        summary.reads.len(),
        summary.writes.len(),
        summary.returns,
        summary.has_side_effects,
        summary.callees.len(),
        summary.is_side_effect_free,
    );
}

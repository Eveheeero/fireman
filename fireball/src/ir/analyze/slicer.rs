//! Program slicing — backward slice extraction.
//!
//! Extracts the subset of IR statements that influence a chosen criterion
//! (e.g., a return value or specific register at a specific point).

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
use std::collections::HashSet;
use std::sync::Arc;

/// The slicing criterion: which value to slice backwards from.
#[derive(Debug, Clone)]
pub enum SliceCriterion {
    /// Slice from a specific register.
    Register(Register),
    /// Slice from return value (all Halt statements — seeds from
    /// the last assignment before each Halt).
    ReturnValue,
}

/// A program slice: the set of IR statement indices that contribute to the criterion.
#[derive(Debug, Clone)]
pub struct ProgramSlice {
    /// IR statement indices included in the slice.
    pub included: HashSet<usize>,
    /// The criterion used.
    pub criterion: SliceCriterion,
}

impl ProgramSlice {
    /// Fraction of total IR included in the slice.
    pub fn coverage(&self, total_ir: usize) -> f64 {
        if total_ir == 0 {
            0.0
        } else {
            self.included.len() as f64 / total_ir as f64
        }
    }
}

/// Compute a backward slice from the given criterion.
pub fn backward_slice(
    blocks: &[Arc<Block>],
    criterion: SliceCriterion,
) -> ProgramSlice {
    // Flatten all IR statements with globally unique IDs
    let mut all_stmts: Vec<(usize, &IrStatement)> = Vec::new();
    for block in blocks {
        let ir_block = block.get_ir();
        let Some(ir_block) = ir_block.as_ref() else {
            continue;
        };
        for ir in ir_block.ir().iter() {
            let Some(stmts) = ir.statements else {
                continue;
            };
            for stmt in stmts {
                let unique_id = all_stmts.len();
                all_stmts.push((unique_id, stmt));
            }
        }
    }

    // Find initial seed registers
    let mut worklist: Vec<Register> = Vec::new();
    let mut included: HashSet<usize> = HashSet::new();

    match &criterion {
        SliceCriterion::Register(reg) => {
            worklist.push(*reg);
        }
        SliceCriterion::ReturnValue => {
            // Find each Halt and trace backward to the preceding assignment
            // to seed with the registers that contribute to the return value
            for (i, (idx, stmt)) in all_stmts.iter().enumerate() {
                if matches!(stmt, IrStatement::Halt) {
                    included.insert(*idx);
                    // Walk backwards from this Halt to find the most recent assignment
                    for j in (0..i).rev() {
                        let (prev_idx, prev_stmt) = &all_stmts[j];
                        if let IrStatement::Assignment { from, to, .. } = prev_stmt {
                            // This is the last assignment before Halt — likely the return value
                            included.insert(*prev_idx);
                            // Seed worklist with source registers
                            collect_source_registers(from, &mut worklist);
                            // Also seed from the destination (e.g., if it's a register
                            // that was previously assigned)
                            if let IrData::Register(dst) = to.as_ref() {
                                // Don't add dst to worklist — we want its sources, not itself
                                let _ = dst;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    // Backward walk: for each register in worklist, find its definitions
    // and add their source registers to the worklist
    let mut visited_regs: HashSet<Register> = HashSet::new();

    while let Some(reg) = worklist.pop() {
        if !visited_regs.insert(reg) {
            continue;
        }

        for (idx, stmt) in &all_stmts {
            match stmt {
                IrStatement::Assignment { from, to, .. } => {
                    if let IrData::Register(dst) = to.as_ref() {
                        if *dst == reg {
                            included.insert(*idx);
                            collect_source_registers(from, &mut worklist);
                        }
                    }
                }
                IrStatement::Condition {
                    condition,
                    true_branch,
                    false_branch,
                } => {
                    let writes_reg = |stmts: &[IrStatement]| -> bool {
                        stmts.iter().any(|s| {
                            if let IrStatement::Assignment { to, .. } = s {
                                matches!(to.as_ref(), IrData::Register(r) if *r == reg)
                            } else {
                                false
                            }
                        })
                    };
                    if writes_reg(true_branch) || writes_reg(false_branch) {
                        included.insert(*idx);
                        collect_source_registers(condition, &mut worklist);
                        for inner in true_branch.iter().chain(false_branch.iter()) {
                            if let IrStatement::Assignment { from, to, .. } = inner {
                                if let IrData::Register(dst) = to.as_ref() {
                                    if *dst == reg {
                                        collect_source_registers(from, &mut worklist);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    ProgramSlice {
        included,
        criterion,
    }
}

/// Extract all registers referenced in an IrData expression.
fn collect_source_registers(data: &Aos<IrData>, worklist: &mut Vec<Register>) {
    match data.as_ref() {
        IrData::Register(reg) => {
            worklist.push(*reg);
        }
        IrData::Dereference(inner) => {
            collect_source_registers(inner, worklist);
        }
        IrData::Operation(_) => {
            let mut related = Vec::new();
            data.get_related_ir_data(&mut related);
            for r in related {
                if let IrData::Register(reg) = r.as_ref() {
                    worklist.push(*reg);
                }
            }
        }
        _ => {}
    }
}

/// Log slicing analysis results.
pub fn log_slice_analysis(blocks: &[Arc<Block>]) {
    let total_ir: usize = blocks
        .iter()
        .filter_map(|b| b.get_ir().as_ref().map(|ir| ir.ir().len()))
        .sum();

    if total_ir == 0 {
        return;
    }

    let slice = backward_slice(blocks, SliceCriterion::ReturnValue);
    if !slice.included.is_empty() {
        debug!(
            "Program slice (return value): {}/{} statements ({:.1}%)",
            slice.included.len(),
            total_ir,
            slice.coverage(total_ir) * 100.0,
        );
    }
}

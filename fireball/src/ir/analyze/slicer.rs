//! Program slicing — backward and forward slice extraction.
//!
//! Extracts the subset of IR statements that influence a chosen criterion
//! (e.g., a return value or specific register at a specific point).

use crate::{
    core::Block,
    ir::{
        Register, VirtualMachine,
        data::{IrData, IrDataContainable},
        statements::IrStatement,
        x86_64::X64Range,
    },
    prelude::*,
    utils::Aos,
};
use std::{collections::HashSet, sync::Arc};

/// The slicing criterion: which value to slice backwards from.
#[derive(Debug, Clone)]
pub enum SliceCriterion {
    /// Slice from a specific register.
    Register(Register),
    /// Slice forward from conservatively seeded parameter registers.
    Parameters,
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

impl SliceCriterion {
    fn label(&self) -> &'static str {
        match self {
            SliceCriterion::Register(_) => "register",
            SliceCriterion::Parameters => "parameters",
            SliceCriterion::ReturnValue => "return value",
        }
    }
}

/// Compute a backward slice from the given criterion.
pub fn backward_slice(blocks: &[Arc<Block>], criterion: SliceCriterion) -> ProgramSlice {
    let all_stmts = flatten_statements(blocks);

    // Find initial seed registers
    let mut worklist: Vec<Register> = Vec::new();
    let mut included: HashSet<usize> = HashSet::new();

    match &criterion {
        SliceCriterion::Register(reg) => {
            worklist.push(*reg);
        }
        SliceCriterion::Parameters => {}
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

/// Compute a forward slice from the given criterion.
pub fn forward_slice(blocks: &[Arc<Block>], criterion: SliceCriterion) -> ProgramSlice {
    let all_stmts = flatten_statements(blocks);
    let mut tracked_registers = HashSet::new();
    let mut included = HashSet::new();

    match &criterion {
        SliceCriterion::Register(reg) => {
            tracked_registers.insert(*reg);
        }
        SliceCriterion::Parameters => {
            tracked_registers.extend(conservative_parameter_registers());
        }
        SliceCriterion::ReturnValue => {
            return ProgramSlice {
                included,
                criterion,
            };
        }
    }

    for (idx, stmt) in &all_stmts {
        if propagate_forward_statement(stmt, &mut tracked_registers) {
            included.insert(*idx);
        }
    }

    ProgramSlice {
        included,
        criterion,
    }
}

fn flatten_statements(blocks: &[Arc<Block>]) -> Vec<(usize, &IrStatement)> {
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
    all_stmts
}

fn conservative_parameter_registers() -> [Register; 12] {
    [
        <VirtualMachine as X64Range>::rdi(),
        <VirtualMachine as X64Range>::edi(),
        <VirtualMachine as X64Range>::rsi(),
        <VirtualMachine as X64Range>::esi(),
        <VirtualMachine as X64Range>::rdx(),
        <VirtualMachine as X64Range>::edx(),
        <VirtualMachine as X64Range>::rcx(),
        <VirtualMachine as X64Range>::ecx(),
        <VirtualMachine as X64Range>::r8(),
        <VirtualMachine as X64Range>::r8d(),
        <VirtualMachine as X64Range>::r9(),
        <VirtualMachine as X64Range>::r9d(),
    ]
}

fn conservative_return_registers() -> [Register; 2] {
    [
        <VirtualMachine as X64Range>::rax(),
        <VirtualMachine as X64Range>::eax(),
    ]
}

fn propagate_forward_statement(
    statement: &IrStatement,
    tracked_registers: &mut HashSet<Register>,
) -> bool {
    match statement {
        IrStatement::Assignment { from, to, .. } => {
            if !data_uses_tracked_registers(from, tracked_registers) {
                return false;
            }

            if let IrData::Register(dst) = to.as_ref() {
                tracked_registers.insert(*dst);
            }

            true
        }
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            let mut branch_relevant = false;

            for inner in true_branch.iter().chain(false_branch.iter()) {
                branch_relevant |= propagate_forward_statement(inner, tracked_registers);
            }

            data_uses_tracked_registers(condition, tracked_registers) || branch_relevant
        }
        IrStatement::Jump { target } => data_uses_tracked_registers(target, tracked_registers),
        IrStatement::JumpByCall { .. } => {
            let consumes_tracked_argument = conservative_parameter_registers()
                .iter()
                .any(|reg| tracked_registers.contains(reg));

            if consumes_tracked_argument {
                tracked_registers.extend(conservative_return_registers());
            }

            consumes_tracked_argument
        }
        IrStatement::Special(special) => special_uses_tracked_registers(special, tracked_registers),
        IrStatement::Undefined | IrStatement::Exception(_) | IrStatement::Halt => false,
    }
}

fn data_uses_tracked_registers(data: &Aos<IrData>, tracked_registers: &HashSet<Register>) -> bool {
    match data.as_ref() {
        IrData::Register(reg) => tracked_registers.contains(reg),
        IrData::Dereference(inner) => data_uses_tracked_registers(inner, tracked_registers),
        IrData::Operation(_) => {
            let mut related = Vec::new();
            data.get_related_ir_data(&mut related);

            related.into_iter().any(|related| {
                matches!(related.as_ref(), IrData::Register(reg) if tracked_registers.contains(reg))
            })
        }
        _ => false,
    }
}

fn special_uses_tracked_registers(
    statement: &crate::ir::statements::IrStatementSpecial,
    tracked_registers: &HashSet<Register>,
) -> bool {
    match statement {
        crate::ir::statements::IrStatementSpecial::TypeSpecified { location, .. } => {
            data_uses_tracked_registers(location, tracked_registers)
        }
        crate::ir::statements::IrStatementSpecial::CalcFlagsAutomatically {
            operation,
            flags,
            ..
        } => {
            data_uses_tracked_registers(operation, tracked_registers)
                || flags
                    .iter()
                    .any(|flag| data_uses_tracked_registers(flag, tracked_registers))
        }
        crate::ir::statements::IrStatementSpecial::Assertion { condition } => {
            data_uses_tracked_registers(condition, tracked_registers)
        }
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
pub fn log_slice_analysis(slice: &ProgramSlice, total_statements: usize) {
    if total_statements == 0 {
        return;
    }

    if !slice.included.is_empty() {
        debug!(
            "Program slice ({}): {}/{} statements ({:.1}%)",
            slice.criterion.label(),
            slice.included.len(),
            total_statements,
            slice.coverage(total_statements) * 100.0,
        );
    }
}

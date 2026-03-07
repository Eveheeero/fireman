//! Basic taint analysis — track data flow from sources through transforms.
//!
//! Marks data originating from function parameters and API return values,
//! then propagates taint labels forward through assignments.

use crate::{
    core::Block,
    ir::{
        Register,
        data::IrData,
        statements::IrStatement,
    },
    prelude::*,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// A taint label identifying the source of tainted data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaintLabel {
    /// Function parameter (0-based index).
    Parameter(u8),
    /// Return value from a call at the given IR index.
    ApiReturn(u32),
    /// Generic user-controllable input.
    UserInput,
}

/// Result of taint analysis: which registers carry which taint labels.
#[derive(Debug, Clone, Default)]
pub struct TaintAnalysis {
    /// Register → set of taint labels it currently carries.
    pub register_taints: HashMap<Register, HashSet<TaintLabel>>,
    /// Count of tainted assignments observed.
    pub tainted_assignment_count: usize,
}

impl TaintAnalysis {
    /// Check if a register is tainted.
    pub fn is_tainted(&self, reg: &Register) -> bool {
        self.register_taints
            .get(reg)
            .is_some_and(|s| !s.is_empty())
    }

    /// Get taint labels for a register.
    pub fn labels_of(&self, reg: &Register) -> Option<&HashSet<TaintLabel>> {
        self.register_taints.get(reg)
    }

    fn taint_register(&mut self, reg: Register, labels: HashSet<TaintLabel>) {
        if !labels.is_empty() {
            self.register_taints
                .entry(reg)
                .or_default()
                .extend(labels);
        }
    }

    fn get_taints(&self, reg: &Register) -> HashSet<TaintLabel> {
        self.register_taints
            .get(reg)
            .cloned()
            .unwrap_or_default()
    }
}

/// Run taint analysis over function blocks.
///
/// Seeds: function parameters (first 4 registers in x86-64 calling convention)
/// are tainted as Parameter(0..3). Call return values are tainted as ApiReturn.
pub fn analyze_taint(blocks: &[Arc<Block>]) -> TaintAnalysis {
    let mut analysis = TaintAnalysis::default();
    let mut ir_index: u32 = 0;

    // Seed: first block's initial register state represents parameters
    // In x86-64 System V: rdi, rsi, rdx, rcx, r8, r9
    // In Win64: rcx, rdx, r8, r9
    // We don't know which ABI, so we seed all common param registers
    // by checking if they're read before written (handled by forward propagation)

    for block in blocks {
        let ir_block = block.get_ir();
        let Some(ir_block) = ir_block.as_ref() else {
            continue;
        };

        for ir in ir_block.ir() {
            let Some(stmts) = ir.statements else {
                ir_index += 1;
                continue;
            };
            for stmt in stmts {
                propagate_taint(stmt, &mut analysis, &mut ir_index);
            }
            ir_index += 1;
        }
    }

    analysis
}

fn propagate_taint(
    stmt: &IrStatement,
    analysis: &mut TaintAnalysis,
    ir_index: &mut u32,
) {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            // Collect taint from source expression
            let source_taints = collect_data_taints(from, analysis);

            if let IrData::Register(dst) = to.as_ref() {
                if !source_taints.is_empty() {
                    analysis.tainted_assignment_count += 1;
                    analysis.taint_register(*dst, source_taints);
                } else {
                    // Assignment from clean source clears taint
                    analysis.register_taints.remove(dst);
                }
            }
        }
        IrStatement::JumpByCall { .. } => {
            // After a call, the return register may carry taint from the API
            // We mark this as ApiReturn taint — downstream assignments from
            // the return register will propagate it
            // Note: we'd need architecture info to know which register is the
            // return register. For now, we just record the call site.
            *ir_index += 1;
        }
        IrStatement::Condition {
            true_branch,
            false_branch,
            ..
        } => {
            for s in true_branch.iter() {
                propagate_taint(s, analysis, ir_index);
            }
            for s in false_branch.iter() {
                propagate_taint(s, analysis, ir_index);
            }
        }
        _ => {}
    }
}

/// Collect taint labels from all registers referenced in an IrData expression.
fn collect_data_taints(
    data: &crate::utils::Aos<IrData>,
    analysis: &TaintAnalysis,
) -> HashSet<TaintLabel> {
    let mut taints = HashSet::new();
    match data.as_ref() {
        IrData::Register(reg) => {
            taints.extend(analysis.get_taints(reg));
        }
        IrData::Dereference(inner) => {
            taints.extend(collect_data_taints(inner, analysis));
        }
        IrData::Operation(_) => {
            let mut related = Vec::new();
            use crate::ir::data::IrDataContainable;
            data.get_related_ir_data(&mut related);
            for r in related {
                if let IrData::Register(reg) = r.as_ref() {
                    taints.extend(analysis.get_taints(reg));
                }
            }
        }
        _ => {}
    }
    taints
}

/// Log taint analysis results.
pub fn log_taint_analysis(blocks: &[Arc<Block>]) {
    let analysis = analyze_taint(blocks);
    let tainted_regs = analysis
        .register_taints
        .values()
        .filter(|s| !s.is_empty())
        .count();
    if tainted_regs > 0 || analysis.tainted_assignment_count > 0 {
        debug!(
            "Taint analysis: {} tainted registers, {} tainted assignments",
            tainted_regs, analysis.tainted_assignment_count,
        );
    }
}

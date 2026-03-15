//! Conservative intraprocedural pointer escape analysis.
//!
//! This is intentionally narrow: it only marks pointer-like registers as
//! escaping when they flow into common call-argument registers or are stored
//! through non-stack memory. It does not attempt interprocedural propagation
//! or full heap/ownership recovery.

use crate::{
    core::Block,
    ir::{
        Register, VirtualMachine,
        data::IrData,
        statements::{IrStatement, IrStatementSpecial},
        x86_64::X64Range,
    },
    prelude::*,
};
use std::{collections::HashSet, sync::Arc};

#[derive(Debug, Clone, Default)]
pub struct EscapeAnalysis {
    escaped_locations: HashSet<super::points_to::AbstractLocation>,
    call_escaped_locations: HashSet<super::points_to::AbstractLocation>,
    memory_escaped_locations: HashSet<super::points_to::AbstractLocation>,
    interprocedural_escaped_locations: HashSet<super::points_to::AbstractLocation>,
}

impl EscapeAnalysis {
    pub fn escaped_locations(&self) -> &HashSet<super::points_to::AbstractLocation> {
        &self.escaped_locations
    }

    pub fn escaped_count(&self) -> usize {
        self.escaped_locations.len()
    }

    pub fn call_escaped_count(&self) -> usize {
        self.call_escaped_locations.len()
    }

    pub fn memory_escaped_count(&self) -> usize {
        self.memory_escaped_locations.len()
    }

    pub fn interprocedural_escaped_count(&self) -> usize {
        self.interprocedural_escaped_locations.len()
    }

    pub fn escapes(&self, location: &super::points_to::AbstractLocation) -> bool {
        self.escaped_locations.contains(location)
    }
}

pub fn analyze_pointer_escape(
    blocks: &[Arc<Block>],
    points_to: &super::points_to::PointsToSet,
) -> EscapeAnalysis {
    let mut analysis = EscapeAnalysis::default();
    let mut points_to = points_to.clone();
    let pointer_like_locations = points_to
        .tracked_locations()
        .into_iter()
        .filter(|location| !points_to.targets_of(location).is_empty())
        .collect::<Vec<_>>();

    if pointer_like_locations.is_empty() {
        return analysis;
    }

    for block in blocks {
        let ir_block = block.get_ir();
        let Some(ir_block) = ir_block.as_ref() else {
            continue;
        };

        for ir in ir_block.ir() {
            let Some(stmts) = ir.statements.as_ref() else {
                continue;
            };
            for stmt in stmts.iter() {
                process_escape_statement(
                    stmt,
                    &pointer_like_locations,
                    &mut points_to,
                    &mut analysis,
                );
            }
        }
    }

    analysis
}

fn process_escape_statement(
    stmt: &IrStatement,
    pointer_like_locations: &[super::points_to::AbstractLocation],
    points_to: &mut super::points_to::PointsToSet,
    analysis: &mut EscapeAnalysis,
) {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            if destination_is_nonlocal_memory(to.as_ref()) {
                if let Some(source_loc) = ir_data_to_location(from.as_ref()) {
                    mark_escape(
                        source_loc,
                        pointer_like_locations,
                        points_to,
                        &mut analysis.escaped_locations,
                    );
                    mark_escape(
                        source_loc,
                        pointer_like_locations,
                        points_to,
                        &mut analysis.memory_escaped_locations,
                    );
                }
            }
        }
        IrStatement::JumpByCall { .. } => {
            for &location in pointer_like_locations {
                let super::points_to::AbstractLocation::Register(register) = location else {
                    continue;
                };
                if !is_common_pointer_argument_register(register) {
                    continue;
                }
                mark_escape(
                    location,
                    pointer_like_locations,
                    points_to,
                    &mut analysis.escaped_locations,
                );
                mark_escape(
                    location,
                    pointer_like_locations,
                    points_to,
                    &mut analysis.call_escaped_locations,
                );
            }
        }
        IrStatement::Condition {
            true_branch,
            false_branch,
            ..
        } => {
            for branch_stmt in true_branch.iter() {
                process_escape_statement(branch_stmt, pointer_like_locations, points_to, analysis);
            }
            for branch_stmt in false_branch.iter() {
                process_escape_statement(branch_stmt, pointer_like_locations, points_to, analysis);
            }
        }
        IrStatement::Special(IrStatementSpecial::Assertion { .. })
        | IrStatement::Special(IrStatementSpecial::TypeSpecified { .. })
        | IrStatement::Special(IrStatementSpecial::CalcFlagsAutomatically { .. })
        | IrStatement::Jump { .. }
        | IrStatement::Halt
        | IrStatement::Undefined
        | IrStatement::Exception(_) => {}
    }
}

fn mark_escape(
    location: super::points_to::AbstractLocation,
    pointer_like_locations: &[super::points_to::AbstractLocation],
    points_to: &mut super::points_to::PointsToSet,
    escaped: &mut HashSet<super::points_to::AbstractLocation>,
) {
    for &candidate in pointer_like_locations {
        if points_to.may_alias(&location, &candidate) {
            escaped.insert(candidate);
        }
    }
}

pub fn mark_interprocedural_register_escape(
    analysis: &mut EscapeAnalysis,
    register: Register,
    points_to: &super::points_to::PointsToSet,
) -> bool {
    let mut points_to = points_to.clone();
    let pointer_like_locations = points_to
        .tracked_locations()
        .into_iter()
        .filter(|location| !points_to.targets_of(location).is_empty())
        .collect::<Vec<_>>();

    if pointer_like_locations.is_empty() {
        return false;
    }

    let location = super::points_to::AbstractLocation::Register(register);
    let mut changed = false;
    for candidate in pointer_like_locations {
        if points_to.may_alias(&location, &candidate) {
            changed |= analysis.escaped_locations.insert(candidate);
            changed |= analysis.interprocedural_escaped_locations.insert(candidate);
        }
    }
    changed
}

fn ir_data_to_location(data: &IrData) -> Option<super::points_to::AbstractLocation> {
    match data {
        IrData::Register(register) => Some(super::points_to::AbstractLocation::Register(*register)),
        IrData::Constant(address) => {
            Some(super::points_to::AbstractLocation::Global(*address as u64))
        }
        _ => None,
    }
}

fn destination_is_nonlocal_memory(destination: &IrData) -> bool {
    let IrData::Dereference(address) = destination else {
        return false;
    };
    !address_mentions_stack(address.as_ref())
}

fn address_mentions_stack(data: &IrData) -> bool {
    match data {
        IrData::Register(register) => register.is_stack_related(),
        IrData::Dereference(inner) => address_mentions_stack(inner.as_ref()),
        IrData::Operation(operation) => match operation {
            crate::ir::data::IrDataOperation::Unary { arg, .. } => {
                address_mentions_stack(arg.as_ref())
            }
            crate::ir::data::IrDataOperation::Binary { arg1, arg2, .. } => {
                address_mentions_stack(arg1.as_ref()) || address_mentions_stack(arg2.as_ref())
            }
        },
        IrData::Intrinsic(_) | IrData::Constant(_) | IrData::Operand(_) => false,
    }
}

fn is_common_pointer_argument_register(register: Register) -> bool {
    [
        <VirtualMachine as X64Range>::rcx(),
        <VirtualMachine as X64Range>::ecx(),
        <VirtualMachine as X64Range>::rdx(),
        <VirtualMachine as X64Range>::edx(),
        <VirtualMachine as X64Range>::r8(),
        <VirtualMachine as X64Range>::r8d(),
        <VirtualMachine as X64Range>::r9(),
        <VirtualMachine as X64Range>::r9d(),
        <VirtualMachine as X64Range>::rdi(),
        <VirtualMachine as X64Range>::edi(),
        <VirtualMachine as X64Range>::rsi(),
        <VirtualMachine as X64Range>::esi(),
    ]
    .contains(&register)
}

pub fn log_escape_analysis(analysis: &EscapeAnalysis) {
    if analysis.escaped_count() > 0 {
        debug!(
            "Pointer escape analysis: {} escaped locations ({} via calls, {} via non-stack stores, {} via interprocedural projection)",
            analysis.escaped_count(),
            analysis.call_escaped_count(),
            analysis.memory_escaped_count(),
            analysis.interprocedural_escaped_count(),
        );
    }
}

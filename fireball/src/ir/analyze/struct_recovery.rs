//! Array/struct discrimination via offset pattern analysis.
//!
//! Analyzes base+offset memory access patterns to infer whether accesses
//! form an array (constant stride) or struct (varying offsets) pattern.

use crate::{
    core::Block,
    ir::{
        Register,
        data::{IrData, IrDataOperation},
        operator::IrBinaryOperator,
        statements::IrStatement,
    },
    prelude::*,
};
use std::{collections::HashMap, sync::Arc};

/// A candidate field access at a specific offset from a base.
#[derive(Debug, Clone)]
pub struct FieldCandidate {
    pub offset: i64,
    pub access_count: usize,
    pub is_read: bool,
    pub is_write: bool,
}

/// A candidate aggregate (struct or array) accessed through a base pointer.
#[derive(Debug, Clone)]
pub struct AggregateCandidate {
    pub base: Register,
    pub fields: Vec<FieldCandidate>,
    /// True if constant stride detected (likely array).
    pub likely_array: bool,
    /// Detected stride if array-like.
    pub stride: Option<i64>,
}

/// Scan IR for base+offset memory patterns and infer struct/array candidates.
pub fn recover_aggregates(blocks: &[Arc<Block>]) -> Vec<AggregateCandidate> {
    // Collect: base_register → map of offset → (read_count, write_count)
    let mut base_offsets: HashMap<Register, HashMap<i64, (usize, usize)>> = HashMap::new();

    for block in blocks {
        let ir_block = block.get_ir();
        let Some(ir_block) = ir_block.as_ref() else {
            continue;
        };

        for ir in ir_block.ir() {
            let Some(stmts) = ir.statements else {
                continue;
            };
            for stmt in stmts {
                scan_base_offset(stmt, &mut base_offsets);
            }
        }
    }

    // Convert to candidates and classify
    base_offsets
        .into_iter()
        .filter(|(_, offsets)| offsets.len() >= 2) // Need 2+ offsets to be interesting
        .map(|(base, offsets)| {
            let mut fields: Vec<FieldCandidate> = offsets
                .into_iter()
                .map(|(offset, (reads, writes))| FieldCandidate {
                    offset,
                    access_count: reads + writes,
                    is_read: reads > 0,
                    is_write: writes > 0,
                })
                .collect();
            fields.sort_by_key(|f| f.offset);

            let (likely_array, stride) = detect_stride(&fields);

            AggregateCandidate {
                base,
                fields,
                likely_array,
                stride,
            }
        })
        .collect()
}

fn scan_base_offset(
    stmt: &IrStatement,
    base_offsets: &mut HashMap<Register, HashMap<i64, (usize, usize)>>,
) {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            // Read: source is Dereference(base + offset)
            if let Some((base, offset)) = extract_base_offset(from) {
                let entry = base_offsets
                    .entry(base)
                    .or_default()
                    .entry(offset)
                    .or_default();
                entry.0 += 1; // read
            }
            // Write: dest is Dereference(base + offset)
            if let Some((base, offset)) = extract_base_offset(to) {
                let entry = base_offsets
                    .entry(base)
                    .or_default()
                    .entry(offset)
                    .or_default();
                entry.1 += 1; // write
            }
        }
        IrStatement::Condition {
            true_branch,
            false_branch,
            ..
        } => {
            for s in true_branch.iter() {
                scan_base_offset(s, base_offsets);
            }
            for s in false_branch.iter() {
                scan_base_offset(s, base_offsets);
            }
        }
        _ => {}
    }
}

/// Extract (base_register, offset) from Dereference(Operation(Binary(Add, Register, Constant))).
fn extract_base_offset(data: &crate::utils::Aos<IrData>) -> Option<(Register, i64)> {
    let IrData::Dereference(inner) = data.as_ref() else {
        return None;
    };
    let IrData::Operation(IrDataOperation::Binary {
        operator: IrBinaryOperator::Add,
        arg1,
        arg2,
    }) = inner.as_ref()
    else {
        return None;
    };

    // Pattern 1: Register + Constant
    if let (IrData::Register(reg), IrData::Constant(offset)) = (arg1.as_ref(), arg2.as_ref()) {
        return Some((reg.clone(), *offset as i64));
    }
    // Pattern 2: Constant + Register
    if let (IrData::Constant(offset), IrData::Register(reg)) = (arg1.as_ref(), arg2.as_ref()) {
        return Some((reg.clone(), *offset as i64));
    }

    None
}

/// Detect if offsets form a constant stride (array-like pattern).
fn detect_stride(fields: &[FieldCandidate]) -> (bool, Option<i64>) {
    if fields.len() < 3 {
        return (false, None);
    }

    let offsets: Vec<i64> = fields.iter().map(|f| f.offset).collect();
    let mut diffs: Vec<i64> = offsets.windows(2).map(|w| w[1] - w[0]).collect();
    diffs.sort_unstable();
    diffs.dedup();

    // If all differences are the same, it's a constant stride
    if diffs.len() == 1 && diffs[0] > 0 {
        return (true, Some(diffs[0]));
    }

    (false, None)
}

/// Log struct/array recovery results.
pub fn log_aggregate_recovery(candidates: &[AggregateCandidate]) {
    if !candidates.is_empty() {
        debug!(
            "Aggregate recovery: {} candidates ({} arrays, {} structs)",
            candidates.len(),
            candidates.iter().filter(|c| c.likely_array).count(),
            candidates.iter().filter(|c| !c.likely_array).count(),
        );
    }
}

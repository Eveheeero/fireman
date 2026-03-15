//! Global variable recovery — identify fixed-address memory accesses as globals.
//!
//! Scans IR for `Dereference(Constant(addr))` patterns where `addr` falls
//! within data sections (not code), groups by address, and counts accesses.

use crate::{
    core::{Block, Sections},
    ir::{
        data::{IrData, IrDataContainable},
        statements::IrStatement,
    },
    prelude::*,
    utils::Aos,
};
use std::{collections::HashMap, sync::Arc};

/// A recovered global variable candidate.
#[derive(Debug, Clone)]
pub struct GlobalVariable {
    pub address: u64,
    pub name: Option<String>,
    pub access_count: usize,
    pub read_count: usize,
    pub write_count: usize,
    pub likely_size: Option<u32>,
}

/// Scan all blocks for global variable candidates.
pub fn recover_globals(blocks: &[Arc<Block>], sections: &Sections) -> Vec<GlobalVariable> {
    let mut access_map: HashMap<u64, (usize, usize)> = HashMap::new(); // addr → (reads, writes)

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
                scan_statement_for_globals(stmt, sections, &mut access_map);
            }
        }
    }

    access_map
        .into_iter()
        .map(|(addr, (reads, writes))| GlobalVariable {
            address: addr,
            name: None,
            access_count: reads + writes,
            read_count: reads,
            write_count: writes,
            likely_size: None,
        })
        .collect()
}

fn scan_statement_for_globals(
    stmt: &IrStatement,
    sections: &Sections,
    access_map: &mut HashMap<u64, (usize, usize)>,
) {
    match stmt {
        IrStatement::Assignment { from, to, .. } => {
            // Reads from globals (source side)
            collect_global_addrs(from, sections, access_map, true);
            // Writes to globals (dest side — only top-level deref counts as write)
            if let Some(addr) = extract_top_level_global_addr(to, sections) {
                let entry = access_map.entry(addr).or_default();
                entry.1 += 1; // write
            }
            // Recurse into dest subexpressions for nested reads,
            // but skip the top-level deref (already counted as write above)
            if let IrData::Dereference(inner) = to.as_ref() {
                collect_global_addrs(inner, sections, access_map, true);
            }
        }
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            collect_global_addrs(condition, sections, access_map, true);
            for s in true_branch.iter() {
                scan_statement_for_globals(s, sections, access_map);
            }
            for s in false_branch.iter() {
                scan_statement_for_globals(s, sections, access_map);
            }
        }
        IrStatement::Jump { target } | IrStatement::JumpByCall { target } => {
            collect_global_addrs(target, sections, access_map, true);
        }
        IrStatement::Special(special) => {
            use crate::ir::statements::IrStatementSpecial;
            match special {
                IrStatementSpecial::TypeSpecified { location, .. } => {
                    collect_global_addrs(location, sections, access_map, true);
                }
                IrStatementSpecial::CalcFlagsAutomatically {
                    operation, flags, ..
                } => {
                    collect_global_addrs(operation, sections, access_map, true);
                    for flag in flags {
                        collect_global_addrs(flag, sections, access_map, true);
                    }
                }
                IrStatementSpecial::Assertion { condition } => {
                    collect_global_addrs(condition, sections, access_map, true);
                }
            }
        }
        _ => {}
    }
}

/// Recursively traverse IrData tree, collecting all global address references.
fn collect_global_addrs(
    data: &Aos<IrData>,
    sections: &Sections,
    access_map: &mut HashMap<u64, (usize, usize)>,
    is_read: bool,
) {
    match data.as_ref() {
        IrData::Dereference(inner) => {
            // Check if this is Dereference(Constant(addr)) — a global access
            if let IrData::Constant(addr) = inner.as_ref() {
                let addr = *addr as u64;
                if is_data_section_addr(addr, sections) {
                    let entry = access_map.entry(addr).or_default();
                    if is_read {
                        entry.0 += 1;
                    }
                }
            }
            // Recurse into the address expression
            collect_global_addrs(inner, sections, access_map, is_read);
        }
        IrData::Operation(_) => {
            let mut related = Vec::new();
            data.get_related_ir_data(&mut related);
            for r in related {
                collect_global_addrs(r, sections, access_map, is_read);
            }
        }
        _ => {}
    }
}

/// Extract top-level Dereference(Constant(addr)) for write targets.
fn extract_top_level_global_addr(data: &Aos<IrData>, sections: &Sections) -> Option<u64> {
    if let IrData::Dereference(inner) = data.as_ref() {
        if let IrData::Constant(addr) = inner.as_ref() {
            let addr = *addr as u64;
            if is_data_section_addr(addr, sections) {
                return Some(addr);
            }
        }
    }
    None
}

/// Check if an address falls within a non-executable data section.
/// Uses section characteristics flags (IMAGE_SCN_MEM_EXECUTE).
fn is_data_section_addr(addr: u64, sections: &Sections) -> bool {
    if let Some(section) = sections.from_virtual_address(addr) {
        return !section.is_executable();
    }
    false
}

/// Log global variable recovery results.
pub fn log_global_recovery(blocks: &[Arc<Block>], sections: &Sections) {
    let globals = recover_globals(blocks, sections);
    if !globals.is_empty() {
        debug!(
            "Global variable recovery: {} candidates found",
            globals.len()
        );
        for g in &globals {
            debug!(
                "  0x{:X}: {} accesses ({} reads, {} writes)",
                g.address, g.access_count, g.read_count, g.write_count
            );
        }
    }
}

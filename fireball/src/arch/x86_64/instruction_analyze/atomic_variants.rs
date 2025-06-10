//! Atomic variants of instructions that support LOCK prefix
//!
//! This module provides atomic versions of instructions that can be used with the LOCK prefix.
//! Each atomic variant wraps the memory operations with appropriate atomic ordering.

use super::shortcuts::*;
use crate::ir::statements::{IrStatement, MemoryOrdering};

/// Helper to wrap memory operations with atomic semantics
fn wrap_atomic(stmt: IrStatement, ordering: MemoryOrdering) -> IrStatement {
    match &stmt {
        // Only wrap Assignment statements that involve memory
        IrStatement::Assignment { from, to, .. } => {
            // Check if this is a memory operation (involves dereference)
            let from_str = format!("{}", from);
            let to_str = format!("{}", to);
            let involves_memory = from_str.contains("d(") || to_str.contains("d(");

            if involves_memory {
                IrStatement::Atomic {
                    statement: Box::new(stmt),
                    ordering,
                }
            } else {
                stmt
            }
        }
        _ => stmt,
    }
}

/// Atomic ADD - LOCK ADD
#[box_to_static_reference]
pub(super) fn atomic_add() -> &'static [IrStatement] {
    let base = super::a::add();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic ADC - LOCK ADC
#[box_to_static_reference]
pub(super) fn atomic_adc() -> &'static [IrStatement] {
    let base = super::a::adc();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic AND - LOCK AND
#[box_to_static_reference]
pub(super) fn atomic_and() -> &'static [IrStatement] {
    let base = super::a::and();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic BTC - LOCK BTC
#[box_to_static_reference]
pub(super) fn atomic_btc() -> &'static [IrStatement] {
    // BTC is not yet implemented in base
    [].into()
}

/// Atomic BTR - LOCK BTR
#[box_to_static_reference]
pub(super) fn atomic_btr() -> &'static [IrStatement] {
    // BTR is not yet implemented in base
    [].into()
}

/// Atomic BTS - LOCK BTS
#[box_to_static_reference]
pub(super) fn atomic_bts() -> &'static [IrStatement] {
    // BTS is not yet implemented in base
    [].into()
}

/// Atomic CMPXCHG - LOCK CMPXCHG
#[box_to_static_reference]
pub(super) fn atomic_cmpxchg() -> &'static [IrStatement] {
    let base = super::c::cmpxchg();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic DEC - LOCK DEC
#[box_to_static_reference]
pub(super) fn atomic_dec() -> &'static [IrStatement] {
    let base = super::d::dec();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic INC - LOCK INC
#[box_to_static_reference]
pub(super) fn atomic_inc() -> &'static [IrStatement] {
    let base = super::i::inc();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic NEG - LOCK NEG
#[box_to_static_reference]
pub(super) fn atomic_neg() -> &'static [IrStatement] {
    let base = super::n::neg();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic NOT - LOCK NOT
#[box_to_static_reference]
pub(super) fn atomic_not() -> &'static [IrStatement] {
    let base = super::n::not();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic OR - LOCK OR
#[box_to_static_reference]
pub(super) fn atomic_or() -> &'static [IrStatement] {
    let base = super::o::or();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic SBB - LOCK SBB
#[box_to_static_reference]
pub(super) fn atomic_sbb() -> &'static [IrStatement] {
    let base = super::s::sbb();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic SUB - LOCK SUB
#[box_to_static_reference]
pub(super) fn atomic_sub() -> &'static [IrStatement] {
    let base = super::s::sub();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic XOR - LOCK XOR
#[box_to_static_reference]
pub(super) fn atomic_xor() -> &'static [IrStatement] {
    let base = super::x::xor();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

/// Atomic XADD - LOCK XADD
#[box_to_static_reference]
pub(super) fn atomic_xadd() -> &'static [IrStatement] {
    // XADD is not yet implemented in base
    [].into()
}

/// Atomic XCHG - XCHG is implicitly atomic
#[box_to_static_reference]
pub(super) fn atomic_xchg() -> &'static [IrStatement] {
    // XCHG is always atomic, even without LOCK prefix
    let base = super::x::xchg();
    let atomic_stmts: Vec<IrStatement> = base
        .iter()
        .map(|stmt| wrap_atomic(stmt.clone(), MemoryOrdering::SeqCst))
        .collect();
    atomic_stmts.into_boxed_slice()
}

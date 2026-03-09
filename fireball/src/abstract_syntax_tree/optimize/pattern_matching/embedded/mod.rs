//! Embedded Rust implementations of optimization passes.
//!
//! These are the original Rust optimization functions that were migrated to
//! `.fb` pattern files. They are preserved here as reference implementations
//! and potential fallback for passes that cannot be fully expressed in the
//! pattern DSL.

pub(crate) mod bit_trick_recognition;
pub(crate) mod boolean_recovery;
pub(crate) mod cast_minimization;
pub(crate) mod constant_folding;
pub(crate) mod control_flow_cleanup;
pub(crate) mod early_return_normalization;
pub(crate) mod identity_simplification;
pub(crate) mod if_conversion_reversal;
pub(crate) mod magic_division_recovery;
pub(crate) mod operator_canonicalization;
pub(crate) mod ternary_recovery;
pub(crate) mod assertion_recovery;
pub(crate) mod do_while_recovery;
pub(crate) mod clamp_recovery;
pub(crate) mod loop_cleanup;
pub(crate) mod test_utils;

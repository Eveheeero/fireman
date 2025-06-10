//! X86 unified register module (placeholder)
//!
//! This module will eventually contain shared code for x86 and x86_64 registers.
//! Currently a placeholder to satisfy module resolution.

// Re-export X86Register from x86_64 module for now
pub use crate::arch::x86_64::register::X86Register;

// TODO: Implement unified X86 register handling

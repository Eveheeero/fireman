//! Atomic operation support for x86_64 architecture
//!
//! This module handles the LOCK prefix and atomic memory operations.

use crate::ir::statements::{IrStatement, MemoryOrdering};

/// Represents the LOCK prefix state for an instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockPrefix {
    /// No LOCK prefix
    None,
    /// LOCK prefix present - instruction is atomic
    Present,
}

/// Check if an instruction has the LOCK prefix
pub fn has_lock_prefix(instruction: &crate::core::Instruction) -> LockPrefix {
    // In x86_64, the LOCK prefix byte is 0xF0
    if let Some(bytes) = &instruction.inner.bytes {
        if bytes.contains(&0xF0) {
            return LockPrefix::Present;
        }
    }
    LockPrefix::None
}

/// Get the appropriate memory ordering for atomic operations
pub fn get_memory_ordering(lock_prefix: LockPrefix) -> MemoryOrdering {
    match lock_prefix {
        LockPrefix::None => MemoryOrdering::Relaxed,
        LockPrefix::Present => MemoryOrdering::SeqCst, // x86 LOCK provides sequential consistency
    }
}

/// Instructions that can be used with LOCK prefix
pub fn can_have_lock_prefix(mnemonic: &str) -> bool {
    matches!(
        mnemonic.to_lowercase().as_str(),
        "add"
            | "adc"
            | "and"
            | "btc"
            | "btr"
            | "bts"
            | "cmpxchg"
            | "cmpxchg8b"
            | "cmpxchg16b"
            | "dec"
            | "inc"
            | "neg"
            | "not"
            | "or"
            | "sbb"
            | "sub"
            | "xor"
            | "xadd"
            | "xchg"
    )
}

/// Validate LOCK prefix usage
pub fn validate_lock_prefix(instruction: &crate::core::Instruction) -> Result<(), &'static str> {
    let has_lock = has_lock_prefix(instruction) == LockPrefix::Present;

    if !has_lock {
        return Ok(());
    }

    // Get mnemonic from the statement
    let mnemonic = match &instruction.inner.statement {
        Ok(iceball::Statement::X64(stmt)) => {
            // Convert statement enum to string
            format!("{:?}", stmt).to_lowercase()
        }
        _ => return Err("Invalid instruction type"),
    };

    // Check if instruction can have LOCK prefix
    if !can_have_lock_prefix(&mnemonic) {
        return Err("LOCK prefix used with invalid instruction");
    }

    // LOCK prefix requires memory operand as destination
    // This would need more detailed operand analysis
    // For now, we assume it's valid if the instruction supports LOCK

    Ok(())
}

/// Create atomic memory operation IR statements
pub fn create_atomic_operation(
    base_statements: Vec<IrStatement>,
    lock_prefix: LockPrefix,
) -> Vec<IrStatement> {
    if lock_prefix == LockPrefix::None {
        return base_statements;
    }

    // Wrap memory operations with atomic markers
    base_statements
        .into_iter()
        .map(|stmt| {
            match stmt {
                // For memory loads/stores, add atomic ordering
                IrStatement::Assignment { .. } => {
                    // In a real implementation, we would check if this involves memory
                    // and add appropriate atomic ordering
                    stmt
                }
                // Other statements pass through unchanged
                _ => stmt,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_have_lock_prefix() {
        assert!(can_have_lock_prefix("ADD"));
        assert!(can_have_lock_prefix("cmpxchg"));
        assert!(can_have_lock_prefix("XCHG"));
        assert!(!can_have_lock_prefix("MOV"));
        assert!(!can_have_lock_prefix("JMP"));
        assert!(!can_have_lock_prefix("CALL"));
    }

    #[test]
    fn test_memory_ordering() {
        assert_eq!(
            get_memory_ordering(LockPrefix::None),
            MemoryOrdering::Relaxed
        );
        assert_eq!(
            get_memory_ordering(LockPrefix::Present),
            MemoryOrdering::SeqCst
        );
    }
}

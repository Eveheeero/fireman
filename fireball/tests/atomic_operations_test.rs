//! Test atomic operation support (LOCK prefix)

use fireball::arch::x86_64::atomic::{LockPrefix, can_have_lock_prefix, get_memory_ordering};
use fireball::ir::statements::MemoryOrdering;

#[test]
fn test_can_have_lock_prefix() {
    // Instructions that can have LOCK prefix
    assert!(can_have_lock_prefix("ADD"));
    assert!(can_have_lock_prefix("adc"));
    assert!(can_have_lock_prefix("AND"));
    assert!(can_have_lock_prefix("btc"));
    assert!(can_have_lock_prefix("BTR"));
    assert!(can_have_lock_prefix("bts"));
    assert!(can_have_lock_prefix("CMPXCHG"));
    assert!(can_have_lock_prefix("cmpxchg8b"));
    assert!(can_have_lock_prefix("CMPXCHG16B"));
    assert!(can_have_lock_prefix("dec"));
    assert!(can_have_lock_prefix("INC"));
    assert!(can_have_lock_prefix("neg"));
    assert!(can_have_lock_prefix("NOT"));
    assert!(can_have_lock_prefix("or"));
    assert!(can_have_lock_prefix("SBB"));
    assert!(can_have_lock_prefix("sub"));
    assert!(can_have_lock_prefix("XOR"));
    assert!(can_have_lock_prefix("xadd"));
    assert!(can_have_lock_prefix("XCHG"));

    // Instructions that cannot have LOCK prefix
    assert!(!can_have_lock_prefix("MOV"));
    assert!(!can_have_lock_prefix("jmp"));
    assert!(!can_have_lock_prefix("CALL"));
    assert!(!can_have_lock_prefix("ret"));
    assert!(!can_have_lock_prefix("PUSH"));
    assert!(!can_have_lock_prefix("pop"));
    assert!(!can_have_lock_prefix("LEA"));
    assert!(!can_have_lock_prefix("test"));
    assert!(!can_have_lock_prefix("CMP"));
}

#[test]
fn test_memory_ordering_mapping() {
    // No LOCK prefix = relaxed ordering
    assert_eq!(
        get_memory_ordering(LockPrefix::None),
        MemoryOrdering::Relaxed
    );

    // LOCK prefix = sequential consistency
    assert_eq!(
        get_memory_ordering(LockPrefix::Present),
        MemoryOrdering::SeqCst
    );
}

#[test]
fn test_lock_prefix_detection() {
    // This test would require actual instruction bytes
    // For now, we just test the enum
    assert_ne!(LockPrefix::None, LockPrefix::Present);
}

#[test]
fn test_atomic_instruction_combinations() {
    // Common atomic operations in x86_64
    let atomic_ops = vec![
        ("lock add", true),
        ("lock xchg", true),
        ("lock cmpxchg", true),
        ("lock inc", true),
        ("lock dec", true),
        ("lock and", true),
        ("lock or", true),
        ("lock xor", true),
        ("lock mov", false), // MOV cannot have LOCK
        ("lock jmp", false), // JMP cannot have LOCK
    ];

    for (op, expected) in atomic_ops {
        let parts: Vec<&str> = op.split_whitespace().collect();
        if parts.len() == 2 && parts[0] == "lock" {
            let can_lock = can_have_lock_prefix(parts[1]);
            assert_eq!(
                can_lock, expected,
                "Instruction '{}' lock capability mismatch",
                parts[1]
            );
        }
    }
}

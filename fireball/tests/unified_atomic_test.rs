//! Tests for unified atomic operations support across architectures
//!
//! This test suite verifies that atomic operations are correctly
//! handled across different architectures with appropriate memory ordering.

use fireball::arch::{ArchType, BaseOperation};
use fireball::ir::atomic_operations::{AtomicBuilder, AtomicOperation, AtomicRmwOp, helpers};
use fireball::ir::data::IrData;
use fireball::ir::statements::{IrStatement, MemoryOrdering};
use fireball::utils::Aos;

#[test]
fn test_atomic_builder_basic() {
    // Test building various atomic operations

    // Atomic load
    let load_op = AtomicBuilder::new(BaseOperation::Load)
        .operand(Aos::new(IrData::Constant(0x1000)))
        .size(8)
        .ordering(MemoryOrdering::Acquire)
        .build()
        .unwrap();

    match load_op {
        AtomicOperation::Load { size, ordering, .. } => {
            assert_eq!(size, 8);
            assert_eq!(ordering, MemoryOrdering::Acquire);
        }
        _ => panic!("Expected Load operation"),
    }

    // Atomic store
    let store_op = AtomicBuilder::new(BaseOperation::Store)
        .operand(Aos::new(IrData::Constant(0x2000)))
        .operand(Aos::new(IrData::Constant(42)))
        .size(4)
        .ordering(MemoryOrdering::Release)
        .build()
        .unwrap();

    match store_op {
        AtomicOperation::Store { size, ordering, .. } => {
            assert_eq!(size, 4);
            assert_eq!(ordering, MemoryOrdering::Release);
        }
        _ => panic!("Expected Store operation"),
    }
}

#[test]
fn test_atomic_rmw_operations() {
    // Test all RMW operations
    let rmw_ops = vec![
        (BaseOperation::Add, AtomicRmwOp::Add),
        (BaseOperation::Subtract, AtomicRmwOp::Sub),
        (BaseOperation::And, AtomicRmwOp::And),
        (BaseOperation::Or, AtomicRmwOp::Or),
        (BaseOperation::Xor, AtomicRmwOp::Xor),
        (BaseOperation::Exchange, AtomicRmwOp::Xchg),
    ];

    for (base_op, expected_rmw) in rmw_ops {
        let op = AtomicBuilder::new(base_op)
            .operand(Aos::new(IrData::Constant(0x3000)))
            .operand(Aos::new(IrData::Constant(1)))
            .size(8)
            .ordering(MemoryOrdering::AcqRel)
            .build()
            .unwrap();

        match op {
            AtomicOperation::Rmw {
                operation,
                size,
                ordering,
                ..
            } => {
                assert_eq!(operation, expected_rmw);
                assert_eq!(size, 8);
                assert_eq!(ordering, MemoryOrdering::AcqRel);
            }
            _ => panic!("Expected RMW operation for {:?}", base_op),
        }
    }
}

#[test]
fn test_compare_exchange() {
    let cmpxchg_op = AtomicBuilder::new(BaseOperation::CompareExchange)
        .operand(Aos::new(IrData::Constant(0x4000))) // address
        .operand(Aos::new(IrData::Constant(0))) // expected
        .operand(Aos::new(IrData::Constant(1))) // desired
        .size(8)
        .ordering(MemoryOrdering::SeqCst)
        .build()
        .unwrap();

    match cmpxchg_op {
        AtomicOperation::CompareExchange {
            size,
            success_ordering,
            failure_ordering,
            ..
        } => {
            assert_eq!(size, 8);
            assert_eq!(success_ordering, MemoryOrdering::SeqCst);
            assert_eq!(failure_ordering, MemoryOrdering::SeqCst);
        }
        _ => panic!("Expected CompareExchange operation"),
    }
}

#[test]
fn test_atomic_operation_to_ir() {
    // Test conversion to IR statements
    let add_op = AtomicBuilder::new(BaseOperation::Add)
        .operand(Aos::new(IrData::Constant(0x5000)))
        .operand(Aos::new(IrData::Constant(10)))
        .size(4)
        .ordering(MemoryOrdering::SeqCst)
        .build()
        .unwrap();

    let result = Aos::new(IrData::Constant(0x1000));
    let ir_stmts = add_op.to_ir_statements(Some(result.clone()));

    assert_eq!(ir_stmts.len(), 1);
    match &ir_stmts[0] {
        IrStatement::AtomicRmw {
            result: res,
            size,
            ordering,
            ..
        } => {
            assert_eq!(res, &result);
            assert_eq!(*size, 4);
            assert_eq!(*ordering, MemoryOrdering::SeqCst);
        }
        _ => panic!("Expected AtomicRmw IR statement"),
    }
}

#[test]
fn test_memory_fence() {
    // Test fence operations
    let fence_op = AtomicOperation::Fence {
        ordering: MemoryOrdering::SeqCst,
    };

    let ir_stmts = fence_op.to_ir_statements(None);
    assert_eq!(ir_stmts.len(), 1);

    match &ir_stmts[0] {
        IrStatement::Fence { ordering } => {
            assert_eq!(*ordering, MemoryOrdering::SeqCst);
        }
        _ => panic!("Expected Fence IR statement"),
    }
}

#[test]
fn test_invalid_atomic_operations() {
    // Test that invalid operations fail appropriately

    // Load with wrong number of operands
    let result = AtomicBuilder::new(BaseOperation::Load)
        .size(8)
        .ordering(MemoryOrdering::Acquire)
        .build();
    assert!(result.is_err());

    // Store with wrong number of operands
    let result = AtomicBuilder::new(BaseOperation::Store)
        .operand(Aos::new(IrData::Constant(0x1000)))
        .size(8)
        .ordering(MemoryOrdering::Release)
        .build();
    assert!(result.is_err());

    // Unsupported atomic operation
    let result = AtomicBuilder::new(BaseOperation::Jump)
        .operand(Aos::new(IrData::Constant(0x1000)))
        .size(8)
        .ordering(MemoryOrdering::SeqCst)
        .build();
    assert!(result.is_err());
}

#[test]
fn test_architecture_specific_ordering() {
    // Test that different architectures have appropriate default orderings
    let x86_ordering = helpers::default_ordering_for_arch(ArchType::X86);
    let x64_ordering = helpers::default_ordering_for_arch(ArchType::X86_64);
    let arm32_ordering = helpers::default_ordering_for_arch(ArchType::Arm32);
    let arm64_ordering = helpers::default_ordering_for_arch(ArchType::Arm64);

    // x86 family has strong memory model
    assert_eq!(x86_ordering, MemoryOrdering::SeqCst);
    assert_eq!(x64_ordering, MemoryOrdering::SeqCst);

    // ARM family has weaker memory model
    assert_eq!(arm32_ordering, MemoryOrdering::AcqRel);
    assert_eq!(arm64_ordering, MemoryOrdering::AcqRel);
}

#[test]
fn test_can_be_atomic_helper() {
    // Test which operations can be made atomic
    assert!(helpers::can_be_atomic(BaseOperation::Add));
    assert!(helpers::can_be_atomic(BaseOperation::Subtract));
    assert!(helpers::can_be_atomic(BaseOperation::And));
    assert!(helpers::can_be_atomic(BaseOperation::Or));
    assert!(helpers::can_be_atomic(BaseOperation::Xor));
    assert!(helpers::can_be_atomic(BaseOperation::Load));
    assert!(helpers::can_be_atomic(BaseOperation::Store));
    assert!(helpers::can_be_atomic(BaseOperation::Exchange));
    assert!(helpers::can_be_atomic(BaseOperation::CompareExchange));

    // These operations cannot be atomic
    assert!(!helpers::can_be_atomic(BaseOperation::Jump));
    assert!(!helpers::can_be_atomic(BaseOperation::Call));
    assert!(!helpers::can_be_atomic(BaseOperation::Return));
    assert!(!helpers::can_be_atomic(BaseOperation::Nop));
}

#[test]
fn test_lock_prefix_memory_ordering() {
    // x86 LOCK prefix provides sequential consistency
    assert_eq!(helpers::lock_prefix_to_ordering(), MemoryOrdering::SeqCst);

    // ARM LDAR/STLR provide acquire-release semantics
    assert_eq!(helpers::arm_ldar_stlr_to_ordering(), MemoryOrdering::AcqRel);
}

#[test]
fn test_atomic_operation_sizes() {
    // Test various sizes for atomic operations
    let sizes = vec![1, 2, 4, 8];

    for size in sizes {
        let op = AtomicBuilder::new(BaseOperation::Add)
            .operand(Aos::new(IrData::Constant(0x1000)))
            .operand(Aos::new(IrData::Constant(1)))
            .size(size)
            .ordering(MemoryOrdering::SeqCst)
            .build()
            .unwrap();

        match op {
            AtomicOperation::Rmw { size: op_size, .. } => {
                assert_eq!(op_size, size);
            }
            _ => panic!("Expected RMW operation"),
        }
    }
}

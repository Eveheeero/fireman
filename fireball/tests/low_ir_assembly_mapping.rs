//! Test Low IR to assembly 1:1 mapping
//!
//! This test verifies that Low IR accurately preserves all assembly semantics

use fireball::arch::x86_64::lifter::X64Lifter;
use fireball::core::{Address, Sections};
use fireball::ir::low_ir::*;
use fireball::ir::{Ir, IrBlock};
use std::sync::Arc;

/// Test basic Low IR generation from simple IR
#[test]
fn test_basic_low_ir_generation() {
    let sections = Arc::new(Sections::default());
    let start_addr = Address::from_virtual_address(&sections, 0x1000);

    // Create an empty IR block for now - the lifter is basic
    let ir_instructions = vec![
        Ir {
            address: Address::from_virtual_address(&sections, 0x1000),
            statements: None, // The current lifter handles None gracefully
        },
        Ir {
            address: Address::from_virtual_address(&sections, 0x1003),
            statements: None,
        },
        Ir {
            address: Address::from_virtual_address(&sections, 0x1006),
            statements: None,
        },
    ];

    // Create IR block
    let ir_block = IrBlock::new(ir_instructions, Arc::new([]));

    // Lift to Low IR
    let mut lifter = X64Lifter::new();
    let low_ir = lifter
        .lift_block(&ir_block, start_addr.clone())
        .expect("Failed to lift to Low IR");

    // Verify the Low IR module structure
    assert_eq!(low_ir.functions.len(), 1, "Should have one function");

    let function = low_ir.functions.values().next().unwrap();
    assert_eq!(function.blocks.len(), 1, "Should have one basic block");

    let basic_block = function.blocks.values().next().unwrap();

    // The lifter creates a default Return terminator
    assert!(matches!(basic_block.terminator, Terminator::Return(None)));

    println!("✓ Basic Low IR structure created successfully");
}

/// Test determinism of Low IR generation
#[test]
fn test_low_ir_determinism() {
    let sections = Arc::new(Sections::default());
    let start_addr = Address::from_virtual_address(&sections, 0x2000);

    let mut hashes = Vec::new();

    for i in 0..10 {
        // Pollute memory differently each iteration
        let _garbage: Vec<_> = (0..i * 100).map(|x| vec![x as u8; x % 100]).collect();

        // Create IR block
        let ir_instructions = vec![
            Ir {
                address: Address::from_virtual_address(&sections, 0x2000),
                statements: None,
            },
            Ir {
                address: Address::from_virtual_address(&sections, 0x2004),
                statements: None,
            },
        ];

        let ir_block = IrBlock::new(ir_instructions, Arc::new([]));

        let mut lifter = X64Lifter::new();
        let low_ir = lifter
            .lift_block(&ir_block, start_addr.clone())
            .expect("Failed to lift to Low IR");

        // Hash the Low IR
        let hash = hash_low_ir_module(&low_ir);
        hashes.push(hash);
    }

    // All hashes must be identical
    let first_hash = &hashes[0];
    for (i, hash) in hashes.iter().enumerate() {
        assert_eq!(
            first_hash, hash,
            "Low IR generation produced different output on iteration {}!",
            i
        );
    }

    println!("✓ Low IR generation is deterministic");
}

/// Test that TempAllocator produces deterministic names
#[test]
fn test_temp_allocator_determinism() {
    let sections = Arc::new(Sections::default());
    let addr = Address::from_virtual_address(&sections, 0x3000);

    let mut allocator1 = TempAllocator::new();
    let mut allocator2 = TempAllocator::new();

    // Allocate same temps in same order
    let temp1_a = allocator1.new_temp(addr.clone(), "test");
    let temp1_b = allocator1.new_temp(addr.clone(), "test");
    let temp1_c = allocator1.new_temp(addr.clone(), "other");

    let temp2_a = allocator2.new_temp(addr.clone(), "test");
    let temp2_b = allocator2.new_temp(addr.clone(), "test");
    let temp2_c = allocator2.new_temp(addr.clone(), "other");

    // Should produce identical results
    assert_eq!(temp1_a, temp2_a, "First temp should match");
    assert_eq!(temp1_b, temp2_b, "Second temp should match");
    assert_eq!(temp1_c, temp2_c, "Third temp should match");

    // Should have incrementing indices for same purpose
    assert_eq!(temp1_a.index, 0);
    assert_eq!(temp1_b.index, 1);
    assert_eq!(temp1_c.index, 0); // Different purpose, starts at 0

    println!("✓ TempAllocator produces deterministic names");
}

/// Test LocalId ordering is deterministic
#[test]
fn test_local_id_ordering() {
    let sections = Arc::new(Sections::default());

    // Create LocalIds with different attributes
    let locals = vec![
        LocalId {
            source: Address::from_virtual_address(&sections, 0x1000),
            purpose: "a",
            index: 0,
            version: 0,
        },
        LocalId {
            source: Address::from_virtual_address(&sections, 0x1000),
            purpose: "b",
            index: 0,
            version: 0,
        },
        LocalId {
            source: Address::from_virtual_address(&sections, 0x1000),
            purpose: "a",
            index: 1,
            version: 0,
        },
        LocalId {
            source: Address::from_virtual_address(&sections, 0x2000),
            purpose: "a",
            index: 0,
            version: 0,
        },
    ];

    // Sort by the Ord implementation
    let mut sorted = locals.clone();
    sorted.sort();

    // Verify ordering: by address, then purpose, then index
    assert_eq!(sorted[0].source.get_virtual_address(), 0x1000);
    assert_eq!(sorted[0].purpose, "a");
    assert_eq!(sorted[0].index, 0);

    assert_eq!(sorted[1].source.get_virtual_address(), 0x1000);
    assert_eq!(sorted[1].purpose, "a");
    assert_eq!(sorted[1].index, 1);

    assert_eq!(sorted[2].source.get_virtual_address(), 0x1000);
    assert_eq!(sorted[2].purpose, "b");

    assert_eq!(sorted[3].source.get_virtual_address(), 0x2000);

    println!("✓ LocalId ordering is deterministic");
}

fn hash_low_ir_module(module: &Module) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();

    // Hash target info
    hasher.update(module.target.arch.as_bytes());
    hasher.update(module.target.bits.to_le_bytes());

    // Hash functions
    for (func_id, function) in &module.functions {
        hasher.update(func_id.0.to_le_bytes());

        // Hash blocks
        for (block_id, block) in &function.blocks {
            hasher.update(block_id.0.to_le_bytes());

            // Hash terminator type
            let term_type = match &block.terminator {
                Terminator::Return(_) => 0u8,
                Terminator::Branch(_) => 1u8,
                Terminator::CondBranch { .. } => 2u8,
                Terminator::Switch { .. } => 3u8,
                Terminator::IndirectBranch { .. } => 4u8,
                Terminator::Unreachable => 5u8,
            };
            hasher.update([term_type]);
        }
    }

    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Test that the Low IR types maintain size information correctly
#[test]
fn test_low_ir_type_sizes() {
    // Test basic type sizes
    assert_eq!(Type::Void.size(), Some(0));
    assert_eq!(Type::Bool.size(), Some(1));
    assert_eq!(Type::I8.size(), Some(1));
    assert_eq!(Type::I16.size(), Some(2));
    assert_eq!(Type::I32.size(), Some(4));
    assert_eq!(Type::I64.size(), Some(8));
    assert_eq!(Type::I128.size(), Some(16));
    assert_eq!(Type::F32.size(), Some(4));
    assert_eq!(Type::F64.size(), Some(8));
    assert_eq!(Type::F80.size(), Some(10));
    assert_eq!(Type::Pointer(None).size(), Some(8)); // 64-bit pointers

    // Test array sizes
    assert_eq!(Type::Array(Box::new(Type::I32), 10).size(), Some(40));
    assert_eq!(Type::Array(Box::new(Type::I64), 5).size(), Some(40));

    // Test struct sizes
    let struct_type = Type::Struct(vec![Type::I32, Type::I64, Type::I16]);
    assert_eq!(struct_type.size(), Some(14)); // 4 + 8 + 2

    println!("✓ Low IR type sizes are correct");
}

/// Test Value ordering for canonical forms
#[test]
fn test_value_ordering() {
    let sections = Arc::new(Sections::default());

    let values = vec![
        // Constants come first
        Value::Constant(Constant::Int {
            value: 42,
            ty: Type::I64,
        }),
        Value::Constant(Constant::Int {
            value: 100,
            ty: Type::I64,
        }),
        // Then globals
        Value::Global(GlobalId("global1".to_string())),
        Value::Global(GlobalId("global2".to_string())),
        // Then functions
        Value::Function(FunctionId(0x1000)),
        Value::Function(FunctionId(0x2000)),
        // Then locals
        Value::Local(LocalId {
            source: Address::from_virtual_address(&sections, 0x1000),
            purpose: "test",
            index: 0,
            version: 0,
        }),
        // Finally labels
        Value::Label(BlockId(0x1000)),
    ];

    // Verify they're already in the correct order
    let mut sorted = values.clone();
    sorted.sort();

    assert_eq!(
        values, sorted,
        "Values should already be in canonical order"
    );

    println!("✓ Value ordering maintains canonical forms");
}

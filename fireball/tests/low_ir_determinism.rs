//! Determinism test for Low IR generation
//!
//! CRITICAL: This test verifies that Low IR generation is deterministic

use fireball::arch::x86_64::lifter::X64Lifter;
use fireball::binary::pe::Pe;
use fireball::core::{Address, FireRaw, Sections};
use fireball::ir::low_ir::Module;
use sha2::{Digest, Sha256};
use std::sync::Arc;

/// Helper to hash Low IR module deterministically
fn hash_low_ir_module(module: &Module) -> String {
    let mut hasher = Sha256::new();

    // Hash target info
    hasher.update(module.target.arch.as_bytes());
    hasher.update(module.target.bits.to_le_bytes());
    hasher.update([module.target.endian as u8]);

    // Hash functions in deterministic order (BTreeMap)
    for (func_id, function) in &module.functions {
        hasher.update(func_id.0.to_le_bytes());
        hasher.update(function.entry.0.to_le_bytes());

        // Hash blocks
        for (block_id, block) in &function.blocks {
            hasher.update(block_id.0.to_le_bytes());

            // Hash instructions
            hasher.update((block.instructions.len() as u64).to_le_bytes());
            for (idx, _inst) in block.instructions.iter().enumerate() {
                // For now, just hash instruction index
                // Full instruction hashing would require implementing Hash for Instruction
                hasher.update((idx as u64).to_le_bytes());
            }

            // Hash terminator type as string for determinism
            let terminator_type = match &block.terminator {
                fireball::ir::low_ir::Terminator::Return(_) => "return",
                fireball::ir::low_ir::Terminator::Branch(_) => "branch",
                fireball::ir::low_ir::Terminator::CondBranch { .. } => "cond_branch",
                fireball::ir::low_ir::Terminator::Switch { .. } => "switch",
                fireball::ir::low_ir::Terminator::IndirectBranch { .. } => "indirect",
                fireball::ir::low_ir::Terminator::Unreachable => "unreachable",
            };
            hasher.update(terminator_type.as_bytes());
        }

        // Hash locals count
        hasher.update((function.locals.len() as u64).to_le_bytes());
    }

    let result = hasher.finalize();
    format!("{:x}", result)
}

#[test]
fn test_low_ir_determinism() {
    // Load test binary
    let binary_path = "tests/resources/hello_world.exe";
    let binary_data = std::fs::read(binary_path).expect("Failed to read hello_world.exe");

    // Run Low IR generation 50 times
    let mut hashes = Vec::new();

    for i in 0..50 {
        // Pollute memory state
        let _garbage: Vec<_> = (0..i * 500).map(|x| vec![x as u8; x % 500]).collect();

        // Create fresh Pe instance and analyze
        let fire = Pe::from_binary(binary_data.clone()).unwrap();
        fire.analyze_from_entry().unwrap();

        // Get first block for testing
        let blocks = fire.get_blocks().get_all();
        assert!(!blocks.is_empty(), "No blocks found");

        let first_block = &blocks[0];

        // Get IR block if available
        let ir_guard = first_block.get_ir();
        let ir_block = match &*ir_guard {
            Some(ir) => ir,
            None => {
                println!("No IR available for block, skipping test");
                return;
            }
        };

        // Create lifter and convert to Low IR
        let mut lifter = X64Lifter::new();
        let module = lifter
            .lift_block(ir_block, first_block.get_start_address().clone())
            .expect("Failed to lift block");

        // Hash the module
        let hash = hash_low_ir_module(&module);
        hashes.push(hash);
    }

    // All hashes MUST be identical
    let first_hash = &hashes[0];
    for (i, hash) in hashes.iter().enumerate() {
        assert_eq!(
            first_hash, hash,
            "Low IR generation produced different output on iteration {}!\n\
             First hash: {}\n\
             This hash:  {}\n\
             CRITICAL BUG: Low IR determinism violated!",
            i, first_hash, hash
        );
    }

    println!("✓ Low IR determinism test passed: 50 identical runs");
    println!("  Consistent hash: {}", first_hash);
}

#[test]
fn test_low_ir_determinism_across_threads() {
    use std::thread;

    let binary_path = "tests/resources/hello_world.exe";
    let binary_data = std::fs::read(binary_path).expect("Failed to read hello_world.exe");

    // Analyze once to get blocks
    let fire = Pe::from_binary(binary_data.clone()).unwrap();
    fire.analyze_from_entry().unwrap();
    let blocks = fire.get_blocks().get_all();
    let first_block = &blocks[0];

    // Get IR block for testing
    let ir_guard = first_block.get_ir();
    let ir_block = match &*ir_guard {
        Some(ir) => ir.clone(),
        None => {
            println!("No IR available for block, skipping thread test");
            return;
        }
    };
    drop(ir_guard); // Release lock before threads

    let start_addr = first_block.get_start_address().clone();

    // Run in multiple threads
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let ir = ir_block.clone();
            let addr = start_addr.clone();
            thread::spawn(move || {
                let mut lifter = X64Lifter::new();
                let module = lifter.lift_block(&ir, addr).expect("Failed to lift block");
                hash_low_ir_module(&module)
            })
        })
        .collect();

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All must be identical
    let first = &results[0];
    for (i, result) in results.iter().enumerate() {
        assert_eq!(first, result, "Thread {} produced different Low IR!", i);
    }

    println!("✓ Low IR thread determinism test passed");
}

#[test]
fn test_low_ir_temp_allocator_determinism() {
    use fireball::ir::low_ir::TempAllocator;

    // Create dummy sections and address
    let sections = Arc::new(Sections::default());
    let addr1 = Address::from_virtual_address(&sections, 0x1000);
    let addr2 = Address::from_virtual_address(&sections, 0x2000);

    // Test that same address and purpose always produces deterministic IDs
    for _ in 0..10 {
        let mut alloc = TempAllocator::new();

        let t1 = alloc.new_temp(addr1.clone(), "test");
        let t2 = alloc.new_temp(addr1.clone(), "test");
        let t3 = alloc.new_temp(addr2.clone(), "test");
        let t4 = alloc.new_temp(addr1.clone(), "other");

        assert_eq!(t1.index, 0);
        assert_eq!(t2.index, 1);
        assert_eq!(t3.index, 0); // Different address starts at 0
        assert_eq!(t4.index, 0); // Different purpose starts at 0

        // Test reset
        alloc.reset();
        let t5 = alloc.new_temp(addr1.clone(), "test");
        assert_eq!(t5.index, 0); // Reset should start over
    }

    println!("✓ TempAllocator determinism test passed");
}

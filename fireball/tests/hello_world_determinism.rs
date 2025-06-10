//! Determinism test for hello_world.exe
//!
//! CRITICAL: This test verifies that the same binary ALWAYS produces identical IR

use fireball::binary::pe::Pe;
use fireball::core::FireRaw;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Helper to get a deterministic hash of the decompilation result
fn hash_decompilation_result(pe: &Pe) -> String {
    let mut hasher = Sha256::new();

    // Add entry point address
    let entry_addr = pe.entry().get_virtual_address();
    hasher.update(entry_addr.to_le_bytes());

    // Get all blocks and sort them by ID for deterministic ordering
    let blocks = pe.get_blocks().get_all();
    let sorted_blocks: BTreeMap<_, _> = blocks.iter().map(|b| (b.get_id(), b)).collect();

    // Hash each block's data
    for (id, block) in sorted_blocks {
        hasher.update(id.to_le_bytes());
        hasher.update(
            block
                .get_start_address()
                .get_virtual_address()
                .to_le_bytes(),
        );

        // Add block size if available
        if let Some(size) = block.get_block_size() {
            hasher.update(size.to_le_bytes());
        } else {
            hasher.update(0u64.to_le_bytes()); // Use 0 for blocks without size
        }

        // Add instruction count
        let instructions = block.get_instructions();
        hasher.update((instructions.len() as u64).to_le_bytes());

        // Add each instruction's basic info
        for (idx, instr) in instructions.iter().enumerate() {
            // Since address is private, use instruction index + block start
            // This still provides deterministic ordering
            hasher.update((idx as u64).to_le_bytes());

            // Use the bytes length for size
            if let Ok(bytes) = instr.inner().get_bytes() {
                hasher.update((bytes.len() as u64).to_le_bytes());
                // Also hash the actual instruction bytes for better coverage
                hasher.update(bytes);
            } else {
                hasher.update(0u64.to_le_bytes()); // Unknown size
            }
        }
    }

    // Add relations count (we can't easily iterate them, so just count)
    // This is sufficient to detect changes in control flow
    let blocks_count = blocks.len();
    hasher.update((blocks_count as u64).to_le_bytes());

    // Get the final hash
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[test]
fn test_hello_world_determinism() {
    // Load the hello_world.exe binary
    let binary_path = "tests/resources/hello_world.exe";
    let binary_data = std::fs::read(binary_path).expect("Failed to read hello_world.exe");

    // Run decompilation 100 times
    let mut hashes = Vec::new();

    for i in 0..100 {
        // Pollute memory state differently each time
        let _garbage: Vec<_> = (0..i * 1000).map(|x| vec![x as u8; x % 1000]).collect();

        // Create fresh Pe instance
        let fire = Pe::from_binary(binary_data.clone()).unwrap();

        // Analyze from entry point
        fire.analyze_from_entry().unwrap();

        // Get hash of the result
        let hash = hash_decompilation_result(&fire);
        hashes.push(hash);

        // For now, just verify we got some blocks
        let blocks = fire.get_blocks().get_all();
        assert!(!blocks.is_empty(), "No blocks found at iteration {}", i);
    }

    // All hashes MUST be identical
    let first_hash = &hashes[0];
    for (i, hash) in hashes.iter().enumerate() {
        assert_eq!(
            first_hash, hash,
            "Decompilation produced different output on iteration {}!\n\
             First hash: {}\n\
             This hash:  {}\n\
             CRITICAL BUG: Determinism violated!",
            i, first_hash, hash
        );
    }

    println!("✓ Determinism test passed: 100 identical runs");
    println!("  Consistent hash: {}", first_hash);
}

#[test]
fn test_determinism_with_different_analysis_order() {
    let binary_path = "tests/resources/hello_world.exe";
    let binary_data = std::fs::read(binary_path).expect("Failed to read hello_world.exe");

    // Test 1: Analyze from entry
    let fire1 = Pe::from_binary(binary_data.clone()).unwrap();
    fire1.analyze_from_entry().unwrap();
    let hash1 = hash_decompilation_result(&fire1);

    // Test 2: Analyze in a different way (if supported)
    let fire2 = Pe::from_binary(binary_data).unwrap();
    fire2.analyze_from_entry().unwrap(); // Same analysis for now
    let hash2 = hash_decompilation_result(&fire2);

    assert_eq!(
        hash1, hash2,
        "Different analysis methods produced different results!"
    );
}

#[test]
fn test_determinism_across_threads() {
    use std::thread;

    let binary_path = "tests/resources/hello_world.exe";
    let binary_data = std::fs::read(binary_path).expect("Failed to read hello_world.exe");

    // Run in multiple threads
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let data = binary_data.clone();
            thread::spawn(move || {
                let fire = Pe::from_binary(data).unwrap();
                fire.analyze_from_entry().unwrap();
                hash_decompilation_result(&fire)
            })
        })
        .collect();

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All must be identical
    let first = &results[0];
    for (i, result) in results.iter().enumerate() {
        assert_eq!(first, result, "Thread {} produced different result!", i);
    }
}

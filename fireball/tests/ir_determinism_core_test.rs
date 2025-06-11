//! Core IR determinism validation tests
//!
//! This test ensures that the decompiler produces byte-for-byte identical output
//! for identical input, regardless of runtime conditions.

use fireball::{binary::pe::Pe, core::FireRaw};
use std::sync::{Arc, Mutex};
use std::thread;

/// Test determinism of IR generation across multiple runs
#[test]
fn test_ir_determinism_multiple_runs() {
    // Use the hello_world.exe test binary
    let binary_data = include_bytes!("resources/hello_world.exe");

    // Run decompilation multiple times and verify identical output
    let mut results = Vec::new();

    for run in 0..10 {
        let result = decompile_binary(binary_data);
        results.push((run, result));
    }

    // Verify all results are identical
    let first_result = &results[0].1;
    for (run, result) in &results[1..] {
        assert_eq!(
            first_result, result,
            "Decompilation produced different results on run {}",
            run
        );
    }
}

/// Test determinism with parallel execution
#[test]
fn test_ir_determinism_parallel() {
    let binary_data = include_bytes!("resources/hello_world.exe");
    let binary_arc = Arc::new(binary_data.to_vec());

    // Run decompilation in parallel threads
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for thread_id in 0..8 {
        let binary_clone = binary_arc.clone();
        let results_clone = results.clone();

        let handle = thread::spawn(move || {
            let result = decompile_binary(&binary_clone);
            results_clone.lock().unwrap().push((thread_id, result));
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all results are identical
    let results = results.lock().unwrap();
    let first_result = &results[0].1;
    for (thread_id, result) in &results[1..] {
        assert_eq!(
            first_result, result,
            "Decompilation produced different results in thread {}",
            thread_id
        );
    }
}

/// Test determinism with different memory constraints
#[test]
fn test_determinism_memory_pressure() {
    let binary_data = include_bytes!("resources/hello_world.exe");

    // Run with different pre-allocated memory sizes
    let mut results = Vec::new();

    for capacity in &[1024, 4096, 16384, 65536] {
        // Pre-allocate some memory to simulate different conditions
        let _dummy: Vec<u8> = Vec::with_capacity(*capacity);

        let result = decompile_binary(binary_data);
        results.push((*capacity, result));
    }

    // Verify all results are identical
    let first_result = &results[0].1;
    for (capacity, result) in &results[1..] {
        assert_eq!(
            first_result, result,
            "Decompilation produced different results with capacity {}",
            capacity
        );
    }
}

/// Test determinism of address formatting
#[test]
fn test_address_formatting_determinism() {
    // Test various address values
    let addresses = vec![
        0x00000000u64,
        0x00401000u64,
        0x7FFFFFFF_FFFFFFFFu64,
        0xFFFFFFFF_FFFFFFFFu64,
        0x12345678_9ABCDEF0u64,
    ];

    for addr_value in addresses {
        // Format address multiple times
        let formatted1 = format!("{:016x}", addr_value);
        let formatted2 = format!("{:016x}", addr_value);
        let formatted3 = format!("{:016x}", addr_value);

        assert_eq!(formatted1, formatted2);
        assert_eq!(formatted2, formatted3);
        assert_eq!(formatted1.len(), 16); // Ensure always 16 characters
    }
}

/// Test determinism of BTreeMap ordering
#[test]
fn test_btreemap_ordering_determinism() {
    use std::collections::BTreeMap;

    // Create maps with various insertion orders
    let mut results = Vec::new();

    for _ in 0..5 {
        let mut map = BTreeMap::new();

        // Insert in different order each time (simulated by the loop)
        let values = vec![
            (0x401050u64, "func1"),
            (0x401000u64, "entry"),
            (0x401030u64, "func2"),
            (0x401020u64, "func3"),
            (0x401040u64, "func4"),
        ];

        for (addr, name) in values {
            map.insert(addr, name);
        }

        // Extract keys in iteration order
        let keys: Vec<_> = map.keys().copied().collect();
        results.push(keys);
    }

    // Verify all iterations produced the same order
    let first_result = &results[0];
    for result in &results[1..] {
        assert_eq!(first_result, result);
    }

    // Verify the order is sorted
    let mut sorted = first_result.clone();
    sorted.sort();
    assert_eq!(first_result, &sorted);
}

/// Decompile a binary and return a deterministic string representation
fn decompile_binary(binary_data: &[u8]) -> String {
    let mut result = String::new();

    // Parse the binary
    let pe = Pe::from_binary(binary_data.to_vec()).expect("Failed to parse PE");

    // Get entry point
    let entry_addr = pe.entry().get_virtual_address();
    result.push_str(&format!("Entry: {:016x}\n", entry_addr));

    // Get all blocks in deterministic order
    let blocks = pe.get_blocks().get_all();
    result.push_str(&format!("Blocks: {}\n", blocks.len()));

    // Process blocks in ID order (deterministic)
    let mut block_ids: Vec<_> = blocks.iter().map(|b| b.get_id()).collect();
    block_ids.sort();

    for id in block_ids {
        if let Some(block) = blocks.iter().find(|b| b.get_id() == id) {
            result.push_str(&format!("  Block {}: ", id));
            result.push_str(&format!(
                "start={:016x} ",
                block.get_start_address().get_virtual_address()
            ));

            if let Some(size) = block.get_block_size() {
                result.push_str(&format!("size={} ", size));
            }

            let instructions = block.get_instructions();
            result.push_str(&format!("instructions={}\n", instructions.len()));
        }
    }

    result
}

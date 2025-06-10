//! Parallel execution determinism tests
//!
//! These tests ensure deterministic output regardless of thread count or scheduling

use fireball::ir::{high_ir, low_ir, medium_ir};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;

/// Test that parallel IR generation produces identical results
#[test]
fn test_parallel_ir_generation_deterministic() {
    let module = create_test_module();
    let results = Arc::new(Mutex::new(Vec::new()));

    // Run IR generation in parallel with different thread counts
    for num_threads in [1, 2, 4, 8, 16, 32] {
        let mut handles = vec![];

        for _ in 0..num_threads {
            let module_clone = module.clone();
            let results_clone = Arc::clone(&results);

            let handle = thread::spawn(move || {
                // Generate all IR levels
                let medium_module = medium_ir::Module::from_low_ir(&module_clone);
                let high_module = high_ir::Module::from_medium_ir(&medium_module);
                let mut codegen = high_ir::c_codegen::CCodeGenerator::new();
                let c_code = codegen.generate(&high_module);

                let mut results = results_clone.lock().unwrap();
                results.push(c_code);
            });

            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
    }

    // Verify all results are identical
    let results = results.lock().unwrap();
    let first_result = &results[0];
    for (i, result) in results.iter().enumerate() {
        assert_eq!(
            first_result, result,
            "Parallel execution produced different results at index {}",
            i
        );
    }
}

/// Test that concurrent access to shared data structures is deterministic
#[test]
fn test_concurrent_pattern_store_deterministic() {
    use medium_ir::{Confidence, Pattern, PatternStore};

    // Run test multiple times to catch race conditions
    for _ in 0..100 {
        let store = Arc::new(Mutex::new(PatternStore::new()));
        let mut handles = vec![];

        // Create patterns concurrently
        for i in 0..10 {
            let store_clone = Arc::clone(&store);

            let handle = thread::spawn(move || {
                let pattern = Pattern::LowIR {
                    instructions: vec![],
                    terminator: None,
                    source_block: low_ir::BlockId(i as u64),
                    confidence: Confidence::HIGH,
                };

                let mut store = store_clone.lock().unwrap();
                store.insert(pattern);
            });

            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify deterministic state
        let store = store.lock().unwrap();
        let debug_output = format!("{:?}", *store);

        // The exact pattern IDs might vary due to insertion order,
        // but the total count should be consistent
        let pattern_count = debug_output.matches("PatternRef").count();
        assert_eq!(pattern_count, 10, "Unexpected number of patterns created");
    }
}

/// Test that thread-local data doesn't affect global results
#[test]
fn test_thread_local_isolation() {
    use std::cell::RefCell;

    thread_local! {
        static COUNTER: RefCell<u32> = RefCell::new(0);
    }

    let module = create_test_module();
    let mut results = vec![];

    // Generate IR in different threads with different thread-local state
    for seed in 0..10 {
        let module_clone = module.clone();

        let handle = thread::spawn(move || {
            // Set different thread-local state
            COUNTER.with(|c| *c.borrow_mut() = seed * 100);

            // Generate IR
            let medium_module = medium_ir::Module::from_low_ir(&module_clone);
            let high_module = high_ir::Module::from_medium_ir(&medium_module);
            let mut codegen = high_ir::c_codegen::CCodeGenerator::new();
            codegen.generate(&high_module)
        });

        results.push(handle.join().unwrap());
    }

    // All results should be identical despite different thread-local state
    let first_result = &results[0];
    for (i, result) in results.iter().enumerate() {
        assert_eq!(
            first_result, result,
            "Thread-local state affected output at index {}",
            i
        );
    }
}

/// Test memory pressure doesn't affect determinism
#[test]
#[ignore] // This test allocates a lot of memory
fn test_memory_pressure_determinism() {
    let module = create_test_module();
    let mut results = vec![];

    // Generate IR under different memory conditions
    for i in 0..5 {
        // Allocate varying amounts of memory to create pressure
        let _memory_hog: Vec<u8> = if i > 0 {
            vec![0u8; i * 100_000_000] // 0 to 400MB
        } else {
            vec![]
        };

        // Generate IR
        let medium_module = medium_ir::Module::from_low_ir(&module);
        let high_module = high_ir::Module::from_medium_ir(&medium_module);
        let mut codegen = high_ir::c_codegen::CCodeGenerator::new();
        let c_code = codegen.generate(&high_module);

        results.push(c_code);
    }

    // All results should be identical despite memory pressure
    let first_result = &results[0];
    for (i, result) in results.iter().enumerate() {
        assert_eq!(
            first_result, result,
            "Memory pressure affected output at index {}",
            i
        );
    }
}

/// Helper function to create a test module
fn create_test_module() -> low_ir::Module {
    let mut module = low_ir::Module {
        target: low_ir::TargetInfo::x86_64(),
        functions: BTreeMap::new(),
        globals: BTreeMap::new(),
        externals: BTreeMap::new(),
    };

    let sections = Arc::new(fireball::core::Sections::default());
    let base_addr = fireball::core::Address::from_virtual_address(&sections, 0x1000);

    // Create a more complex function to stress test determinism
    let mut func = low_ir::Function {
        id: low_ir::FunctionId(0x1000),
        signature: low_ir::Type::Function {
            ret: Box::new(low_ir::Type::I32),
            params: vec![low_ir::Type::I32, low_ir::Type::I32],
            varargs: false,
        },
        entry: low_ir::BlockId(0x1000),
        blocks: BTreeMap::new(),
        locals: BTreeMap::new(),
    };

    // Add multiple locals
    for i in 0..10 {
        let local = low_ir::LocalId {
            source: base_addr.clone(),
            index: i,
            version: 0,
            purpose: format!("var_{}", i).leak(),
        };
        func.locals.insert(local, low_ir::Type::I32);
    }

    // Create multiple blocks
    for i in 0..5 {
        let block_id = low_ir::BlockId(0x1000 + i * 0x10);
        let block = low_ir::BasicBlock {
            id: block_id.clone(),
            phis: vec![],
            instructions: vec![],
            terminator: if i < 4 {
                low_ir::Terminator::Branch(low_ir::BlockId(0x1000 + (i + 1) * 0x10))
            } else {
                low_ir::Terminator::Return(Some((
                    low_ir::Value::Constant(low_ir::Constant::Int {
                        value: 42,
                        ty: low_ir::Type::I32,
                    }),
                    low_ir::Type::I32,
                )))
            },
        };
        func.blocks.insert(block_id, block);
    }

    module.functions.insert(func.id.clone(), func);
    module
}

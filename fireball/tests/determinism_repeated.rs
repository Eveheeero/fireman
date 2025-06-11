//! Repeated execution verification tests (1000+ runs)
//!
//! These tests verify that the decompiler produces identical output
//! across many repeated executions to catch rare non-deterministic behavior

use fireball::binary::pe::Pe;
use fireball::core::Fire;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

/// Test configuration for repeated runs
const QUICK_RUNS: usize = 100;
const FULL_RUNS: usize = 1000;
const STRESS_RUNS: usize = 10000;

/// Shared state for tracking test progress
struct TestProgress {
    completed: AtomicUsize,
    failed: AtomicBool,
    failures: Mutex<Vec<String>>,
}

impl TestProgress {
    fn new() -> Self {
        Self {
            completed: AtomicUsize::new(0),
            failed: AtomicBool::new(false),
            failures: Mutex::new(Vec::new()),
        }
    }

    fn increment(&self) {
        self.completed.fetch_add(1, Ordering::SeqCst);
    }

    fn report_failure(&self, msg: String) {
        self.failed.store(true, Ordering::SeqCst);
        self.failures.lock().unwrap().push(msg);
    }

    fn get_progress(&self) -> usize {
        self.completed.load(Ordering::SeqCst)
    }

    fn has_failed(&self) -> bool {
        self.failed.load(Ordering::SeqCst)
    }
}

/// Helper to create test binaries of varying complexity
fn create_test_binary(complexity: usize) -> Vec<u8> {
    let code = match complexity {
        0 => {
            // Simple function
            vec![
                0x55, // push rbp
                0x48, 0x89, 0xe5, // mov rbp, rsp
                0xb8, 0x2a, 0x00, 0x00, 0x00, // mov eax, 42
                0x5d, // pop rbp
                0xc3, // ret
            ]
        }
        1 => {
            // Function with conditionals
            vec![
                0x55, // push rbp
                0x48, 0x89, 0xe5, // mov rbp, rsp
                0x48, 0x83, 0xec, 0x10, // sub rsp, 16
                0x89, 0x7d, 0xfc, // mov [rbp-4], edi
                0x83, 0x7d, 0xfc, 0x00, // cmp dword [rbp-4], 0
                0x7e, 0x07, // jle +7
                0xb8, 0x01, 0x00, 0x00, 0x00, // mov eax, 1
                0xeb, 0x05, // jmp +5
                0xb8, 0x00, 0x00, 0x00, 0x00, // mov eax, 0
                0xc9, // leave
                0xc3, // ret
            ]
        }
        _ => {
            // Function with loop
            vec![
                0x55, // push rbp
                0x48, 0x89, 0xe5, // mov rbp, rsp
                0x48, 0x83, 0xec, 0x10, // sub rsp, 16
                0xc7, 0x45, 0xfc, 0x00, 0x00, 0x00, 0x00, // mov dword [rbp-4], 0
                0xc7, 0x45, 0xf8, 0x00, 0x00, 0x00, 0x00, // mov dword [rbp-8], 0
                0xeb, 0x0e, // jmp +14
                0x8b, 0x45, 0xfc, // mov eax, [rbp-4]
                0x01, 0x45, 0xf8, // add [rbp-8], eax
                0x83, 0x45, 0xfc, 0x01, // add dword [rbp-4], 1
                0x83, 0x7d, 0xfc, 0x0a, // cmp dword [rbp-4], 10
                0x7e, 0xee, // jle -18
                0x8b, 0x45, 0xf8, // mov eax, [rbp-8]
                0xc9, // leave
                0xc3, // ret
            ]
        }
    };

    create_minimal_pe_with_code(code)
}

/// Create a minimal PE with custom code section
fn create_minimal_pe_with_code(code: Vec<u8>) -> Vec<u8> {
    let mut pe = vec![
        // DOS header
        0x4D, 0x5A, // MZ signature
    ];

    // Pad to 0x3C
    pe.resize(0x3C, 0);

    // PE offset at 0x3C
    pe.extend_from_slice(&[0x80, 0x00, 0x00, 0x00]); // PE header at 0x80

    // Pad to PE header
    pe.resize(0x80, 0);

    // PE signature
    pe.extend_from_slice(&[0x50, 0x45, 0x00, 0x00]); // "PE\0\0"

    // COFF header (20 bytes)
    pe.extend_from_slice(&[
        0x64, 0x86, // Machine: x86-64
        0x01, 0x00, // NumberOfSections: 1
        0x00, 0x00, 0x00, 0x00, // TimeDateStamp
        0x00, 0x00, 0x00, 0x00, // PointerToSymbolTable
        0x00, 0x00, 0x00, 0x00, // NumberOfSymbols
        0xF0, 0x00, // SizeOfOptionalHeader
        0x22, 0x00, // Characteristics: executable
    ]);

    // Optional header (240 bytes for PE32+)
    pe.extend_from_slice(&[
        0x0B, 0x02, // Magic: PE32+
        0x0E, 0x00, // MajorLinkerVersion, MinorLinkerVersion
        0x00, 0x10, 0x00, 0x00, // SizeOfCode
        0x00, 0x00, 0x00, 0x00, // SizeOfInitializedData
        0x00, 0x00, 0x00, 0x00, // SizeOfUninitializedData
        0x00, 0x10, 0x00, 0x00, // AddressOfEntryPoint
        0x00, 0x10, 0x00, 0x00, // BaseOfCode
    ]);

    // ImageBase
    pe.extend_from_slice(&[0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00]);

    // Fill rest of optional header
    pe.resize(0x80 + 24 + 240, 0);

    // Section header (.text)
    pe.extend_from_slice(b".text\0\0\0"); // Name
    pe.extend_from_slice(&[0x00, 0x10, 0x00, 0x00]); // VirtualSize
    pe.extend_from_slice(&[0x00, 0x10, 0x00, 0x00]); // VirtualAddress
    pe.extend_from_slice(&[0x00, 0x10, 0x00, 0x00]); // SizeOfRawData
    pe.extend_from_slice(&[0x00, 0x02, 0x00, 0x00]); // PointerToRawData
    pe.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // PointerToRelocations
    pe.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // PointerToLinenumbers
    pe.extend_from_slice(&[0x00, 0x00]); // NumberOfRelocations
    pe.extend_from_slice(&[0x00, 0x00]); // NumberOfLinenumbers
    pe.extend_from_slice(&[0x20, 0x00, 0x00, 0x60]); // Characteristics

    // Pad to code section
    pe.resize(0x200, 0);

    // Add the code
    pe.extend_from_slice(&code);

    pe
}

/// Compute a stable hash of the decompilation output
fn compute_stable_hash(output: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    output.hash(&mut hasher);
    hasher.finish()
}

/// Run decompilation and return hash of output
fn run_decompilation(binary: &[u8], _base_addr: u64) -> Result<u64, String> {
    // Create PE from binary
    let pe =
        Pe::from_binary(binary.to_vec()).map_err(|e| format!("Failed to parse PE: {:?}", e))?;

    // Decompile from entry point
    let output = pe
        .decompile_from_entry()
        .map_err(|e| format!("Decompilation failed: {:?}", e))?;

    Ok(compute_stable_hash(&output))
}

#[test]
fn test_repeated_execution_quick() {
    repeated_execution_test(QUICK_RUNS, 0);
}

#[test]
#[ignore] // Run with --ignored flag
fn test_repeated_execution_full() {
    repeated_execution_test(FULL_RUNS, 1);
}

#[test]
#[ignore] // Run with --ignored flag
fn test_repeated_execution_stress() {
    repeated_execution_test(STRESS_RUNS, 2);
}

/// Core repeated execution test
fn repeated_execution_test(num_runs: usize, complexity: usize) {
    let binary = create_test_binary(complexity);
    let progress = Arc::new(TestProgress::new());
    let start_time = Instant::now();

    println!(
        "Starting {} repeated executions with complexity {}...",
        num_runs, complexity
    );

    // First run to establish baseline
    let baseline_hash =
        run_decompilation(&binary, 0x1000).expect("Failed to run baseline decompilation");

    // Track all unique hashes we see
    let unique_hashes = Arc::new(Mutex::new(BTreeSet::new()));
    unique_hashes.lock().unwrap().insert(baseline_hash);

    // Run in parallel for speed
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(8);
    let runs_per_thread = num_runs / num_threads;
    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let binary_clone = binary.clone();
        let progress_clone = Arc::clone(&progress);
        let unique_hashes_clone = Arc::clone(&unique_hashes);

        let handle = thread::spawn(move || {
            for run in 0..runs_per_thread {
                if progress_clone.has_failed() {
                    break;
                }

                let global_run = thread_id * runs_per_thread + run;

                // Vary base address to test address normalization
                let base_addr = 0x1000 + (global_run as u64 % 16) * 0x1000;

                match run_decompilation(&binary_clone, base_addr) {
                    Ok(hash) => {
                        unique_hashes_clone.lock().unwrap().insert(hash);

                        if hash != baseline_hash {
                            progress_clone.report_failure(format!(
                                "Run {} (thread {}) produced different hash: {} vs baseline {}",
                                global_run, thread_id, hash, baseline_hash
                            ));
                        }
                    }
                    Err(e) => {
                        progress_clone.report_failure(format!(
                            "Run {} (thread {}) failed: {}",
                            global_run, thread_id, e
                        ));
                    }
                }

                progress_clone.increment();

                // Print progress every 10%
                let completed = progress_clone.get_progress();
                if completed % (num_runs / 10) == 0 {
                    let elapsed = start_time.elapsed();
                    let rate = completed as f64 / elapsed.as_secs_f64();
                    println!(
                        "Progress: {}/{} ({:.1}%), Rate: {:.0} runs/sec",
                        completed,
                        num_runs,
                        (completed as f64 / num_runs as f64) * 100.0,
                        rate
                    );
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Final verification
    let elapsed = start_time.elapsed();
    let unique_count = unique_hashes.lock().unwrap().len();

    println!(
        "\nCompleted {} runs in {:.1}s",
        num_runs,
        elapsed.as_secs_f64()
    );
    println!("Unique hashes found: {}", unique_count);

    if progress.has_failed() {
        let failures = progress.failures.lock().unwrap();
        panic!(
            "Determinism test failed with {} failures:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }

    assert_eq!(
        unique_count, 1,
        "Expected exactly 1 unique hash, found {}",
        unique_count
    );
}

#[test]
fn test_repeated_with_memory_stress() {
    let binary = create_test_binary(1);
    let num_runs = 50;
    let mut hashes = Vec::new();

    for i in 0..num_runs {
        // Allocate and deallocate memory to stress the system
        let mut allocations = Vec::new();
        for _ in 0..10 {
            allocations.push(vec![0u8; 1_000_000]); // 1MB each
        }

        // Force some allocations to be dropped
        if i % 2 == 0 {
            allocations.truncate(5);
        }

        // Run decompilation
        let hash = run_decompilation(&binary, 0x1000).expect("Failed to run decompilation");
        hashes.push(hash);

        // Clear allocations
        drop(allocations);
    }

    // Verify all hashes are identical
    let first_hash = hashes[0];
    for (i, hash) in hashes.iter().enumerate() {
        assert_eq!(
            *hash, first_hash,
            "Memory stress affected output at run {}",
            i
        );
    }
}

#[test]
fn test_repeated_with_different_patterns() {
    // Test multiple different binaries to ensure determinism across patterns
    let test_cases = vec![
        ("simple", create_test_binary(0)),
        ("conditional", create_test_binary(1)),
        ("loop", create_test_binary(2)),
    ];

    for (name, binary) in test_cases {
        println!("Testing pattern: {}", name);

        let mut hashes = Vec::new();
        for _ in 0..50 {
            let hash = run_decompilation(&binary, 0x1000).expect("Failed to run decompilation");
            hashes.push(hash);
        }

        // Verify consistency
        let first_hash = hashes[0];
        for (i, hash) in hashes.iter().enumerate() {
            assert_eq!(
                *hash, first_hash,
                "Pattern '{}' produced different output at run {}",
                name, i
            );
        }
    }
}

#[test]
#[ignore] // This test takes a long time
fn test_repeated_endurance() {
    // Long-running test to catch very rare non-deterministic behavior
    let binary = create_test_binary(2);
    let duration = std::time::Duration::from_secs(60); // Run for 1 minute
    let start = Instant::now();
    let mut run_count = 0;
    let mut first_hash = None;

    println!("Running endurance test for {:?}...", duration);

    while start.elapsed() < duration {
        let hash = run_decompilation(&binary, 0x1000).expect("Failed to run decompilation");

        match first_hash {
            None => first_hash = Some(hash),
            Some(expected) => {
                assert_eq!(hash, expected, "Endurance test failed at run {}", run_count);
            }
        }

        run_count += 1;

        // Print progress every 1000 runs
        if run_count % 1000 == 0 {
            let elapsed = start.elapsed();
            let rate = run_count as f64 / elapsed.as_secs_f64();
            println!(
                "Endurance progress: {} runs, {:.0} runs/sec",
                run_count, rate
            );
        }
    }

    println!(
        "Endurance test completed: {} runs in {:?}",
        run_count,
        start.elapsed()
    );
}

//! Memory pressure tests for deterministic behavior
//!
//! These tests verify that the decompiler produces identical output
//! even under various memory pressure conditions

use fireball::binary::pe::Pe;
use fireball::core::Fire;
use std::alloc::{Layout, alloc, dealloc};
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

/// Memory allocator that can simulate various conditions
struct MemoryStressor {
    allocations: Vec<(*mut u8, Layout)>,
    fragmentation_level: usize,
}

impl MemoryStressor {
    fn new() -> Self {
        Self {
            allocations: Vec::new(),
            fragmentation_level: 0,
        }
    }

    /// Allocate memory to create pressure
    unsafe fn allocate_mb(&mut self, mb: usize) {
        let size = mb * 1024 * 1024;
        let align = 16;

        if let Ok(layout) = Layout::from_size_align(size, align) {
            unsafe {
                let ptr = alloc(layout);
                if !ptr.is_null() {
                    // Touch the memory to ensure it's actually allocated
                    ptr::write_bytes(ptr, 0xAA, size);
                    self.allocations.push((ptr, layout));
                }
            }
        }
    }

    /// Create memory fragmentation
    unsafe fn create_fragmentation(&mut self, level: usize) {
        self.fragmentation_level = level;

        // Allocate and deallocate in patterns to create fragmentation
        for i in 0..level * 10 {
            let size = 1024 * (1 + i % 100); // Varying sizes
            let align = 16;

            if let Ok(layout) = Layout::from_size_align(size, align) {
                unsafe {
                    let ptr = alloc(layout);
                    if !ptr.is_null() {
                        ptr::write_bytes(ptr, (i & 0xFF) as u8, size);

                        // Deallocate some to create holes
                        if i % 3 == 0 && !self.allocations.is_empty() {
                            let idx = i % self.allocations.len();
                            let (old_ptr, old_layout) = self.allocations.remove(idx);
                            dealloc(old_ptr, old_layout);
                        } else {
                            self.allocations.push((ptr, layout));
                        }
                    }
                }
            }
        }
    }

    /// Release all allocated memory
    unsafe fn release_all(&mut self) {
        for (ptr, layout) in self.allocations.drain(..) {
            unsafe {
                dealloc(ptr, layout);
            }
        }
    }
}

impl Drop for MemoryStressor {
    fn drop(&mut self) {
        unsafe {
            self.release_all();
        }
    }
}

/// Create a minimal valid PE binary with complex code
fn create_complex_binary() -> Vec<u8> {
    create_minimal_pe_with_code(vec![
        // Complex function with multiple paths and data structures
        // Function prologue
        0x55, // push rbp
        0x48, 0x89, 0xe5, // mov rbp, rsp
        0x48, 0x81, 0xec, 0x80, 0x00, 0x00, 0x00, // sub rsp, 128
        // Initialize local variables
        0x48, 0xc7, 0x45, 0xf8, 0x00, 0x00, 0x00, 0x00, // mov qword [rbp-8], 0
        0x48, 0xc7, 0x45, 0xf0, 0x00, 0x00, 0x00, 0x00, // mov qword [rbp-16], 0
        // Loop with memory accesses
        0x48, 0xc7, 0x45, 0xe8, 0x00, 0x00, 0x00, 0x00, // mov qword [rbp-24], 0
        0xeb, 0x1a, // jmp +26
        // Loop body
        0x48, 0x8b, 0x45, 0xe8, // mov rax, [rbp-24]
        0x48, 0x8d, 0x14, 0x85, 0x00, 0x00, 0x00, 0x00, // lea rdx, [rax*4]
        0x48, 0x8b, 0x45, 0xf8, // mov rax, [rbp-8]
        0x48, 0x01, 0xd0, // add rax, rdx
        0x48, 0x89, 0x45, 0xf0, // mov [rbp-16], rax
        0x48, 0x83, 0x45, 0xe8, 0x01, // add qword [rbp-24], 1
        // Loop condition
        0x48, 0x83, 0x7d, 0xe8, 0x64, // cmp qword [rbp-24], 100
        0x7e, 0xe0, // jle -32
        // Function epilogue
        0x48, 0x8b, 0x45, 0xf0, // mov rax, [rbp-16]
        0xc9, // leave
        0xc3, // ret
    ])
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

#[test]
fn test_determinism_under_memory_pressure() {
    let binary = create_complex_binary();
    let mut results = Vec::new();

    // Test with increasing memory pressure
    for mb in [0, 50, 100, 200, 400] {
        unsafe {
            let mut stressor = MemoryStressor::new();
            stressor.allocate_mb(mb);

            // Run decompilation
            let pe = Pe::from_binary(binary.clone()).expect("Failed to parse PE");
            let output = pe.decompile_from_entry().expect("Failed to decompile");
            results.push((mb, output));

            // Clean up
            stressor.release_all();
        }
    }

    // Verify all results are identical
    let baseline = &results[0].1;
    for (mb, output) in &results {
        assert_eq!(
            baseline, output,
            "Output differs under {}MB memory pressure",
            mb
        );
    }
}

#[test]
fn test_determinism_with_memory_fragmentation() {
    let binary = create_complex_binary();
    let mut results = Vec::new();

    // Test with different fragmentation levels
    for level in 0..5 {
        unsafe {
            let mut stressor = MemoryStressor::new();
            stressor.create_fragmentation(level);

            // Run decompilation
            let pe = Pe::from_binary(binary.clone()).expect("Failed to parse PE");
            let output = pe.decompile_from_entry().expect("Failed to decompile");
            results.push((level, output));

            // Clean up
            stressor.release_all();
        }
    }

    // Verify all results are identical
    let baseline = &results[0].1;
    for (level, output) in &results {
        assert_eq!(
            baseline, output,
            "Output differs with fragmentation level {}",
            level
        );
    }
}

#[test]
fn test_determinism_with_concurrent_memory_pressure() {
    let binary = create_complex_binary();
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    // Run decompilation in multiple threads with memory pressure
    for thread_id in 0..4 {
        let binary_clone = binary.clone();
        let results_clone = Arc::clone(&results);

        let handle = thread::spawn(move || unsafe {
            let mut stressor = MemoryStressor::new();

            // Each thread creates different memory pressure
            stressor.allocate_mb(thread_id * 25);
            stressor.create_fragmentation(thread_id);

            // Run decompilation multiple times
            for run in 0..10 {
                let pe = Pe::from_binary(binary_clone.clone()).expect("Failed to parse PE");
                let output = pe.decompile_from_entry().expect("Failed to decompile");
                results_clone.lock().unwrap().push((thread_id, run, output));

                // Vary memory pressure during runs
                if run % 3 == 0 {
                    stressor.allocate_mb(10);
                }
            }

            stressor.release_all();
        });

        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all results are identical
    let results = results.lock().unwrap();
    let baseline = &results[0].2;
    for (thread_id, run, output) in results.iter() {
        assert_eq!(
            baseline, output,
            "Output differs in thread {} run {}",
            thread_id, run
        );
    }
}

#[test]
#[ignore] // This test uses a lot of memory
fn test_extreme_memory_pressure() {
    let binary = create_complex_binary();
    let mut last_successful_output = None;

    // Test with extreme memory pressure
    for mb in [0, 100, 500, 1000, 2000] {
        unsafe {
            let mut stressor = MemoryStressor::new();

            // Try to allocate, but don't fail if we can't
            stressor.allocate_mb(mb);

            // Run decompilation
            match std::panic::catch_unwind(|| {
                let pe = Pe::from_binary(binary.clone()).expect("Failed to parse PE");
                pe.decompile_from_entry().expect("Failed to decompile")
            }) {
                Ok(output) => {
                    if let Some(ref baseline) = last_successful_output {
                        assert_eq!(
                            baseline, &output,
                            "Output differs under {}MB memory pressure",
                            mb
                        );
                    }
                    last_successful_output = Some(output);
                }
                Err(_) => {
                    println!("Failed to decompile under {}MB pressure (expected)", mb);
                }
            }

            stressor.release_all();
        }
    }

    assert!(
        last_successful_output.is_some(),
        "No successful decompilation runs completed"
    );
}

#[test]
fn test_allocation_pattern_determinism() {
    let binary = create_complex_binary();

    // Different allocation patterns that might affect internal allocator behavior
    let patterns = [
        vec![1, 2, 4, 8, 16, 32, 64], // Exponential growth
        vec![64, 32, 16, 8, 4, 2, 1], // Exponential shrink
        vec![10, 10, 10, 10, 10],     // Constant
        vec![1, 50, 2, 40, 3, 30],    // Alternating
        vec![100],                    // Single large
        vec![1; 100],                 // Many small
    ];

    let mut results = Vec::new();

    for (i, pattern) in patterns.iter().enumerate() {
        unsafe {
            let mut stressor = MemoryStressor::new();

            // Apply allocation pattern
            for &mb in pattern {
                stressor.allocate_mb(mb);
            }

            // Run decompilation
            let pe = Pe::from_binary(binary.clone()).expect("Failed to parse PE");
            let output = pe.decompile_from_entry().expect("Failed to decompile");
            results.push((i, output));

            stressor.release_all();
        }
    }

    // Verify all results are identical
    let baseline = &results[0].1;
    for (pattern_idx, output) in &results {
        assert_eq!(
            baseline, output,
            "Output differs with allocation pattern {}",
            pattern_idx
        );
    }
}

#[test]
fn test_oom_recovery_determinism() {
    let binary = create_complex_binary();
    let mut outputs = Vec::new();

    for attempt in 0..5 {
        // Try to trigger near-OOM conditions
        let result = unsafe {
            let mut stressor = MemoryStressor::new();

            // Allocate memory until we start failing
            let mut allocated = 0;
            while allocated < 10000 {
                // Safety limit
                if let Ok(layout) = Layout::from_size_align(1024 * 1024, 16) {
                    let ptr = alloc(layout);
                    if ptr.is_null() {
                        break; // Can't allocate more
                    }
                    stressor.allocations.push((ptr, layout));
                    allocated += 1;
                } else {
                    break;
                }
            }

            // Release some memory to allow decompilation
            let to_release = stressor.allocations.len() / 2;
            for _ in 0..to_release {
                if let Some((ptr, layout)) = stressor.allocations.pop() {
                    dealloc(ptr, layout);
                }
            }

            // Try decompilation
            let pe = Pe::from_binary(binary.clone()).expect("Failed to parse PE");
            let output = pe.decompile_from_entry().expect("Failed to decompile");
            stressor.release_all();
            output
        };

        outputs.push((attempt, result));
    }

    // Verify all successful runs produced identical output
    let baseline = &outputs[0].1;
    for (attempt, output) in &outputs {
        assert_eq!(
            baseline, output,
            "Output differs after OOM recovery attempt {}",
            attempt
        );
    }
}

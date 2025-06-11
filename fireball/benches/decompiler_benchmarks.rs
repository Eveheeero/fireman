//! Performance benchmarks for the Fireman decompiler
//!
//! Run with: cargo bench

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use fireball::binary::pe::Pe;
use fireball::core::Fire;
use std::time::Duration;

/// Create test binaries of various sizes
fn create_test_binary(size: usize) -> Vec<u8> {
    let mut code = Vec::new();

    match size {
        1 => {
            // Tiny: Single instruction
            code.extend_from_slice(&[0xC3]); // ret
        }
        10 => {
            // Small: Simple function
            code.extend_from_slice(&[
                0x55, // push rbp
                0x48, 0x89, 0xe5, // mov rbp, rsp
                0x8b, 0xc1, // mov eax, ecx
                0x5d, // pop rbp
                0xc3, // ret
            ]);
        }
        100 => {
            // Medium: Function with loop
            code.extend_from_slice(&[
                0x55, // push rbp
                0x48, 0x89, 0xe5, // mov rbp, rsp
                0x31, 0xc0, // xor eax, eax
                0x31, 0xc9, // xor ecx, ecx
            ]);

            // Add loop body
            for _ in 0..20 {
                code.extend_from_slice(&[
                    0x01, 0xc8, // add eax, ecx
                    0x83, 0xc1, 0x01, // add ecx, 1
                    0x83, 0xf9, 0x64, // cmp ecx, 100
                    0x7c, 0xf6, // jl -10
                ]);
            }

            code.extend_from_slice(&[
                0x5d, // pop rbp
                0xc3, // ret
            ]);
        }
        1000 => {
            // Large: Multiple functions
            for i in 0..100 {
                // Function prologue
                code.extend_from_slice(&[
                    0x55, // push rbp
                    0x48, 0x89, 0xe5, // mov rbp, rsp
                ]);

                // Some operations
                for j in 0..8 {
                    let reg = (i + j) % 8;
                    code.extend_from_slice(&[
                        0x48,
                        0x83,
                        0xc0 | reg as u8,
                        0x01, // add r?x, 1
                    ]);
                }

                // Function epilogue
                code.extend_from_slice(&[
                    0x5d, // pop rbp
                    0xc3, // ret
                ]);
            }
        }
        _ => {
            // Default: medium size
            for _ in 0..size {
                code.push(0x90); // nop
            }
            code.push(0xC3); // ret
        }
    }

    create_minimal_pe_with_code(code)
}

/// Create a minimal PE with custom code
fn create_minimal_pe_with_code(code: Vec<u8>) -> Vec<u8> {
    let mut pe = vec![
        // DOS header
        0x4D, 0x5A, // MZ signature
    ];

    // Pad to 0x3C
    pe.resize(0x3C, 0);

    // PE offset
    pe.extend_from_slice(&[0x80, 0x00, 0x00, 0x00]);

    // Pad to PE header
    pe.resize(0x80, 0);

    // PE signature
    pe.extend_from_slice(&[0x50, 0x45, 0x00, 0x00]);

    // COFF header
    pe.extend_from_slice(&[
        0x64, 0x86, // Machine: x86-64
        0x01, 0x00, // NumberOfSections: 1
        0x00, 0x00, 0x00, 0x00, // TimeDateStamp
        0x00, 0x00, 0x00, 0x00, // PointerToSymbolTable
        0x00, 0x00, 0x00, 0x00, // NumberOfSymbols
        0xF0, 0x00, // SizeOfOptionalHeader
        0x22, 0x00, // Characteristics
    ]);

    // Optional header (minimal)
    pe.extend_from_slice(&[
        0x0B, 0x02, // Magic: PE32+
        0x0E, 0x00, // MajorLinkerVersion, MinorLinkerVersion
    ]);

    let code_size = ((code.len() + 0xFFF) & !0xFFF) as u32;
    pe.extend_from_slice(&code_size.to_le_bytes()); // SizeOfCode

    pe.extend_from_slice(&[
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
    pe.extend_from_slice(b".text\0\0\0");
    pe.extend_from_slice(&code_size.to_le_bytes()); // VirtualSize
    pe.extend_from_slice(&[0x00, 0x10, 0x00, 0x00]); // VirtualAddress
    pe.extend_from_slice(&code_size.to_le_bytes()); // SizeOfRawData
    pe.extend_from_slice(&[0x00, 0x02, 0x00, 0x00]); // PointerToRawData
    pe.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // PointerToRelocations
    pe.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // PointerToLinenumbers
    pe.extend_from_slice(&[0x00, 0x00]); // NumberOfRelocations
    pe.extend_from_slice(&[0x00, 0x00]); // NumberOfLinenumbers
    pe.extend_from_slice(&[0x20, 0x00, 0x00, 0x60]); // Characteristics

    // Pad to code section
    pe.resize(0x200, 0);

    // Add code
    pe.extend_from_slice(&code);

    // Pad to section alignment
    let file_size = ((pe.len() + 0x1FF) & !0x1FF);
    pe.resize(file_size, 0);

    pe
}

fn benchmark_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");
    group.measurement_time(Duration::from_secs(10));

    for size in [10, 100, 1000].iter() {
        let binary = create_test_binary(*size);

        group.bench_with_input(BenchmarkId::new("pe_parse", size), &binary, |b, binary| {
            b.iter(|| {
                let _pe = Pe::from_binary(black_box(binary.clone())).unwrap();
            });
        });
    }

    group.finish();
}

fn benchmark_full_decompilation(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_decompilation");
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(50);

    for size in [1, 10, 100].iter() {
        let binary = create_test_binary(*size);

        group.bench_with_input(BenchmarkId::new("decompile", size), &binary, |b, binary| {
            b.iter(|| {
                let pe = Pe::from_binary(binary.clone()).unwrap();
                let _c_code = black_box(pe.decompile_from_entry());
            });
        });
    }

    group.finish();
}

fn benchmark_ir_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ir_generation");
    group.measurement_time(Duration::from_secs(15));

    let binary = create_test_binary(100);
    let pe = Pe::from_binary(binary).unwrap();

    group.bench_function("analyze_entry", |b| {
        b.iter(|| {
            // This would benchmark just the IR generation phase
            // if we had access to the internal methods
            let _result = black_box(pe.decompile_from_entry());
        });
    });

    group.finish();
}

fn benchmark_pattern_matching(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_matching");

    // Create binaries with specific patterns
    let patterns = vec![
        (
            "loop",
            vec![
                0x31, 0xc9, // xor ecx, ecx
                0x83, 0xc1, 0x01, // add ecx, 1
                0x83, 0xf9, 0x64, // cmp ecx, 100
                0x7c, 0xf8, // jl -8
                0xc3, // ret
            ],
        ),
        (
            "switch",
            vec![
                0x83, 0xf9, 0x03, // cmp ecx, 3
                0x77, 0x10, // ja +16
                0xff, 0x24, 0xcd, 0x00, 0x00, 0x00, 0x00, // jmp [rcx*8]
                0xc3, // ret
            ],
        ),
        (
            "array_access",
            vec![
                0x8b, 0x04, 0x87, // mov eax, [rdi + rax*4]
                0xc3, // ret
            ],
        ),
    ];

    for (name, code) in patterns {
        let binary = create_minimal_pe_with_code(code);

        group.bench_with_input(BenchmarkId::new("pattern", name), &binary, |b, binary| {
            b.iter(|| {
                let pe = Pe::from_binary(binary.clone()).unwrap();
                let _c_code = black_box(pe.decompile_from_entry());
            });
        });
    }

    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);

    // Test memory efficiency with larger binaries
    let sizes = vec![100, 500, 1000];

    for size in sizes {
        let binary = create_test_binary(size);

        group.bench_with_input(BenchmarkId::new("memory", size), &binary, |b, binary| {
            b.iter(|| {
                let pe = Pe::from_binary(binary.clone()).unwrap();
                let _c_code = pe.decompile_from_entry();
                // Force drop to measure full lifecycle
                drop(pe);
            });
        });
    }

    group.finish();
}

fn benchmark_optimization_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimization_impact");

    // Test impact of various optimizations
    let test_cases = vec![
        (
            "no_opt",
            vec![
                0x31, 0xc0, // xor eax, eax
                0x31, 0xdb, // xor ebx, ebx
                0x01, 0xd8, // add eax, ebx
                0xc3, // ret
            ],
        ),
        (
            "const_fold",
            vec![
                0xb8, 0x05, 0x00, 0x00, 0x00, // mov eax, 5
                0xbb, 0x03, 0x00, 0x00, 0x00, // mov ebx, 3
                0x01, 0xd8, // add eax, ebx
                0xc3, // ret
            ],
        ),
        (
            "dead_code",
            vec![
                0xb8, 0x05, 0x00, 0x00, 0x00, // mov eax, 5
                0xbb, 0x03, 0x00, 0x00, 0x00, // mov ebx, 3 (dead)
                0xb8, 0x08, 0x00, 0x00, 0x00, // mov eax, 8
                0xc3, // ret
            ],
        ),
    ];

    for (name, code) in test_cases {
        let binary = create_minimal_pe_with_code(code);

        group.bench_with_input(BenchmarkId::new("optimize", name), &binary, |b, binary| {
            b.iter(|| {
                let pe = Pe::from_binary(binary.clone()).unwrap();
                let _c_code = black_box(pe.decompile_from_entry());
            });
        });
    }

    group.finish();
}

// Additional micro-benchmarks for specific components
fn benchmark_address_formatting(c: &mut Criterion) {
    c.bench_function("address_format", |b| {
        let addresses: Vec<u64> = vec![
            0x0,
            0x1000,
            0x401000,
            0xdeadbeef,
            0x7fffffffffffffff,
            0xffffffffffffffff,
        ];

        b.iter(|| {
            for &addr in &addresses {
                let _formatted = black_box(format!("{:016x}", addr));
            }
        });
    });
}

fn benchmark_type_inference(c: &mut Criterion) {
    // This would benchmark type inference if we had access to internals
    // For now, we benchmark through full decompilation
    let binary = create_test_binary(50);

    c.bench_function("type_inference", |b| {
        b.iter(|| {
            let pe = Pe::from_binary(binary.clone()).unwrap();
            let _result = black_box(pe.decompile_from_entry());
        });
    });
}

criterion_group!(
    benches,
    benchmark_parsing,
    benchmark_full_decompilation,
    benchmark_ir_generation,
    benchmark_pattern_matching,
    benchmark_memory_usage,
    benchmark_optimization_impact,
    benchmark_address_formatting,
    benchmark_type_inference
);

criterion_main!(benches);

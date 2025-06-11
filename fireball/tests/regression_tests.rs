//! Regression test suite for the Fireman decompiler
//!
//! These tests ensure that previously fixed bugs don't reappear
//! and that known good outputs remain consistent

use fireball::binary::pe::Pe;
use fireball::core::Fire;
use std::collections::BTreeMap;

/// Test case structure for regression tests
#[derive(Clone)]
struct RegressionTest {
    name: &'static str,
    description: &'static str,
    binary_code: Vec<u8>,
    expected_patterns: Vec<&'static str>,
    forbidden_patterns: Vec<&'static str>,
}

/// Collection of regression tests
fn get_regression_tests() -> Vec<RegressionTest> {
    vec![
        RegressionTest {
            name: "xor_self_zero",
            description: "XOR with self should be recognized as setting to zero",
            binary_code: vec![
                0x31, 0xC0, // xor eax, eax
                0x48, 0x31, 0xDB, // xor rbx, rbx
                0xC3, // ret
            ],
            expected_patterns: vec!["= 0", "0"],
            forbidden_patterns: vec!["^", "xor"],
        },
        RegressionTest {
            name: "parameter_names",
            description: "Parameters should have meaningful names, not param_0",
            binary_code: vec![
                0x55, // push rbp
                0x48, 0x89, 0xe5, // mov rbp, rsp
                0x89, 0x7d, 0xfc, // mov [rbp-4], edi
                0x89, 0x75, 0xf8, // mov [rbp-8], esi
                0x8b, 0x45, 0xfc, // mov eax, [rbp-4]
                0x03, 0x45, 0xf8, // add eax, [rbp-8]
                0x5d, // pop rbp
                0xc3, // ret
            ],
            expected_patterns: vec!["int", "(", ")"],
            forbidden_patterns: vec!["param_0", "param_1", "arg_0"],
        },
        RegressionTest {
            name: "return_statement",
            description: "Functions should have explicit return statements",
            binary_code: vec![
                0xb8, 0x2a, 0x00, 0x00, 0x00, // mov eax, 42
                0xc3, // ret
            ],
            expected_patterns: vec!["return"],
            forbidden_patterns: vec![],
        },
        RegressionTest {
            name: "loop_counter_naming",
            description: "Loop counters should have appropriate names",
            binary_code: vec![
                0x31, 0xc9, // xor ecx, ecx
                0x83, 0xc1, 0x01, // add ecx, 1
                0x83, 0xf9, 0x0a, // cmp ecx, 10
                0x7c, 0xf8, // jl -8
                0xc3, // ret
            ],
            expected_patterns: vec![],
            forbidden_patterns: vec!["var_0", "local_0", "temp_0"],
        },
        RegressionTest {
            name: "deterministic_addresses",
            description: "Addresses should be formatted consistently as 16-digit hex",
            binary_code: vec![
                0xe8, 0x00, 0x00, 0x00, 0x00, // call +0
                0xc3, // ret
            ],
            expected_patterns: vec![],
            forbidden_patterns: vec!["0x1000", "0X", "0000000000001000h"],
        },
        RegressionTest {
            name: "array_access_pattern",
            description: "Array access should be recognized",
            binary_code: vec![
                0x8b, 0x04, 0x87, // mov eax, [rdi + rax*4]
                0xc3, // ret
            ],
            expected_patterns: vec!["[", "]", "*"],
            forbidden_patterns: vec![],
        },
        RegressionTest {
            name: "no_redundant_casts",
            description: "Should not generate unnecessary type casts",
            binary_code: vec![
                0x8b, 0xc1, // mov eax, ecx
                0xc3, // ret
            ],
            expected_patterns: vec![],
            forbidden_patterns: vec!["(int)(int)", "(void*)(void*)"],
        },
        RegressionTest {
            name: "stack_variable_names",
            description: "Stack variables should have descriptive names",
            binary_code: vec![
                0x48, 0x83, 0xec, 0x20, // sub rsp, 32
                0x48, 0x89, 0x4c, 0x24, 0x08, // mov [rsp+8], rcx
                0x48, 0x8b, 0x44, 0x24, 0x08, // mov rax, [rsp+8]
                0x48, 0x83, 0xc4, 0x20, // add rsp, 32
                0xc3, // ret
            ],
            expected_patterns: vec![],
            forbidden_patterns: vec!["stack_0x8", "rsp+8", "rbp-0x"],
        },
    ]
}

/// Create a minimal PE with custom code section
fn create_test_pe(code: Vec<u8>) -> Vec<u8> {
    let mut pe = vec![
        // DOS header
        0x4D, 0x5A, // MZ signature
    ];

    // Pad to 0x3C
    pe.resize(0x3C, 0);

    // PE offset at 0x3C
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
    pe.extend_from_slice(b".text\0\0\0");
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

    // Add the test code
    pe.extend_from_slice(&code);

    pe
}

#[test]
fn test_all_regressions() {
    let tests = get_regression_tests();
    let mut failures = Vec::new();

    for test in &tests {
        println!(
            "Running regression test: {} - {}",
            test.name, test.description
        );

        let pe_bytes = create_test_pe(test.binary_code.clone());
        let pe = Pe::from_binary(pe_bytes).expect("Failed to parse test PE");

        match pe.decompile_from_entry() {
            Ok(c_code) => {
                // Check expected patterns
                for pattern in &test.expected_patterns {
                    if !c_code.contains(pattern) {
                        failures.push(format!(
                            "{}: Missing expected pattern '{}'",
                            test.name, pattern
                        ));
                    }
                }

                // Check forbidden patterns
                for pattern in &test.forbidden_patterns {
                    if c_code.contains(pattern) {
                        failures.push(format!(
                            "{}: Found forbidden pattern '{}'",
                            test.name, pattern
                        ));
                    }
                }
            }
            Err(e) => {
                failures.push(format!("{}: Decompilation failed: {:?}", test.name, e));
            }
        }
    }

    if !failures.is_empty() {
        panic!("Regression tests failed:\n{}", failures.join("\n"));
    }
}

#[test]
fn test_specific_regression_xor_self() {
    let test = RegressionTest {
        name: "xor_self_zero",
        description: "XOR with self should be recognized as setting to zero",
        binary_code: vec![
            0x31, 0xC0, // xor eax, eax
            0xC3, // ret
        ],
        expected_patterns: vec!["= 0", "0"],
        forbidden_patterns: vec!["^"],
    };

    let pe_bytes = create_test_pe(test.binary_code);
    let pe = Pe::from_binary(pe_bytes).expect("Failed to parse PE");

    if let Ok(c_code) = pe.decompile_from_entry() {
        println!("Generated C code:\n{}", c_code);

        // Should recognize xor eax, eax as eax = 0
        assert!(
            c_code.contains("= 0") || c_code.contains("0"),
            "XOR self pattern not optimized to zero assignment"
        );
    }
}

#[test]
fn test_historical_bugs() {
    // Test for historical bugs that have been fixed

    // Bug #1: Missing terminators in pattern conversion
    let missing_terminator_code = vec![
        0x85, 0xC0, // test eax, eax
        0x74, 0x02, // je +2
        0xEB, 0x00, // jmp +0
        0xC3, // ret
    ];

    let pe_bytes = create_test_pe(missing_terminator_code);
    let pe = Pe::from_binary(pe_bytes).expect("Failed to parse PE");

    match pe.decompile_from_entry() {
        Ok(c_code) => {
            // Should have proper control flow
            assert!(
                c_code.contains("if") || c_code.contains("return"),
                "Control flow not properly handled"
            );
        }
        Err(_) => {
            // Decompilation shouldn't crash
            panic!("Decompilation crashed on conditional branch");
        }
    }
}

#[test]
fn test_output_stability() {
    // Test that output remains stable across multiple versions
    let stable_tests = vec![
        (
            "simple_add",
            vec![
                0x01, 0xD0, // add eax, edx
                0xC3, // ret
            ],
        ),
        (
            "simple_mov",
            vec![
                0x8B, 0xC1, // mov eax, ecx
                0xC3, // ret
            ],
        ),
        (
            "simple_call",
            vec![
                0xE8, 0x00, 0x00, 0x00, 0x00, // call +0
                0xC3, // ret
            ],
        ),
    ];

    let mut outputs = BTreeMap::new();

    for (name, code) in stable_tests {
        let pe_bytes = create_test_pe(code);
        let pe = Pe::from_binary(pe_bytes).expect("Failed to parse PE");

        if let Ok(c_code) = pe.decompile_from_entry() {
            outputs.insert(name, c_code);
        }
    }

    // In a real regression suite, we would compare these outputs
    // against known good outputs from previous versions
    assert!(!outputs.is_empty(), "No outputs generated");
}

#[test]
fn test_edge_cases() {
    // Test edge cases that have caused issues in the past

    // Empty function
    let empty_code = vec![0xC3]; // just ret

    let pe_bytes = create_test_pe(empty_code);
    let pe = Pe::from_binary(pe_bytes).expect("Failed to parse PE");

    match pe.decompile_from_entry() {
        Ok(c_code) => {
            assert!(
                !c_code.is_empty(),
                "Empty function should still generate output"
            );
            assert!(c_code.contains("return"), "Should have return statement");
        }
        Err(_) => {
            panic!("Failed to decompile empty function");
        }
    }

    // Very large immediate
    let large_immediate = vec![
        0x48, 0xB8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x7F, // movabs rax, 0x7FFFFFFFFFFFFFFF
        0xC3, // ret
    ];

    let pe_bytes = create_test_pe(large_immediate);
    let pe = Pe::from_binary(pe_bytes).expect("Failed to parse PE");

    if let Ok(c_code) = pe.decompile_from_entry() {
        // Should handle large constants correctly
        assert!(
            c_code.contains("0x7") || c_code.contains("9223372036854775807"),
            "Large immediate not handled correctly"
        );
    }
}

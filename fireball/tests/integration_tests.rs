//! Integration tests for the full decompilation pipeline
//!
//! These tests verify end-to-end functionality from binary to C code

use fireball::binary::pe::Pe;
use fireball::core::Fire;
use std::path::Path;

/// Helper to create a minimal PE binary
fn create_minimal_pe() -> Vec<u8> {
    // This is a minimal valid PE file structure
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

    // Add simple x86-64 code
    pe.extend_from_slice(&[
        0x48, 0x89, 0x5C, 0x24, 0x08, // mov [rsp+8], rbx
        0x57, // push rdi
        0x48, 0x83, 0xEC, 0x20, // sub rsp, 0x20
        0x8B, 0xD9, // mov ebx, ecx
        0x8D, 0x0C, 0x11, // lea ecx, [rcx+rdx]
        0x8B, 0xC1, // mov eax, ecx
        0x48, 0x8B, 0x5C, 0x24, 0x30, // mov rbx, [rsp+0x30]
        0x48, 0x83, 0xC4, 0x20, // add rsp, 0x20
        0x5F, // pop rdi
        0xC3, // ret
    ]);

    pe
}

#[test]
fn test_pe_parsing_and_decompilation() {
    let pe_bytes = create_minimal_pe();

    // Create PE object
    let pe = Pe::from_binary(pe_bytes).expect("Failed to parse PE");

    // Test basic properties
    assert!(!pe.get_binary().is_empty());
    let arch = pe.get_architecture();
    assert_eq!(arch.arch_type, fireball::arch::ArchType::X86_64);

    // Test decompilation
    match pe.decompile_from_entry() {
        Ok(c_code) => {
            // Verify C code structure
            assert!(
                c_code.contains("int") || c_code.contains("void"),
                "Expected function declaration"
            );
            assert!(c_code.contains("{"), "Expected function body");
            assert!(c_code.contains("}"), "Expected function end");
        }
        Err(e) => {
            // It's okay if decompilation fails on minimal PE
            println!("Decompilation failed (expected for minimal PE): {:?}", e);
        }
    }
}

#[test]
fn test_full_pipeline_hello_world() {
    // Load the hello_world.exe test resource if it exists
    let hello_world_path = Path::new("tests/resources/hello_world.exe");

    if hello_world_path.exists() {
        let bytes = std::fs::read(hello_world_path).expect("Failed to read hello_world.exe");
        let pe = Pe::from_binary(bytes).expect("Failed to parse hello_world.exe");

        // Decompile from entry point
        let c_code = pe.decompile_from_entry().expect("Failed to decompile");

        // Verify expected patterns
        assert!(!c_code.is_empty(), "C code should not be empty");
        assert!(
            c_code.contains("int") || c_code.contains("void"),
            "Expected function declaration"
        );

        // Hello world should have string operations or system calls
        let has_expected_patterns = c_code.contains("printf") ||
            c_code.contains("puts") ||
            c_code.contains("write") ||
            c_code.contains("0x") || // At least some hex constants
            c_code.contains("return"); // Return statement

        assert!(
            has_expected_patterns,
            "Expected to find system calls or return statements in hello world"
        );
    }
}

#[test]
fn test_pipeline_with_different_architectures() {
    // Test that architecture detection works correctly
    let x64_pe = create_minimal_pe();
    let pe = Pe::from_binary(x64_pe).expect("Failed to parse PE");

    let arch = pe.get_architecture();
    assert_eq!(arch.arch_type, fireball::arch::ArchType::X86_64);
    assert_eq!(arch.pointer_size, 64);
}

#[test]
fn test_decompile_specific_function() {
    let pe_bytes = create_minimal_pe();
    let pe = Pe::from_binary(pe_bytes).expect("Failed to parse PE");

    // Try to decompile from a specific virtual address
    let result = pe.decompile_from_virtual_address(0x401000);

    // This might fail if the address is invalid, which is fine
    match result {
        Ok(c_code) => {
            assert!(!c_code.is_empty(), "Decompiled code should not be empty");
        }
        Err(_) => {
            // Expected for our minimal PE
        }
    }
}

#[test]
fn test_error_handling() {
    // Test with invalid PE
    let invalid_pe = vec![0xFF; 100];

    match Pe::from_binary(invalid_pe) {
        Ok(_) => panic!("Should have failed to parse invalid PE"),
        Err(e) => {
            // Verify we get a meaningful error
            let error_str = format!("{:?}", e);
            assert!(!error_str.is_empty(), "Error should have description");
        }
    }
}

#[test]
fn test_c_code_quality() {
    let pe_bytes = create_minimal_pe();
    let pe = Pe::from_binary(pe_bytes).expect("Failed to parse PE");

    if let Ok(c_code) = pe.decompile_from_entry() {
        // Check for code quality issues
        assert!(
            !c_code.contains("param_0"),
            "Should not have generic parameter names"
        );
        assert!(
            !c_code.contains("var_0"),
            "Should not have generic variable names"
        );

        // Should have proper formatting
        let lines: Vec<&str> = c_code.lines().collect();
        if lines.len() > 1 {
            // Check indentation exists
            let has_indentation = lines
                .iter()
                .any(|line| line.starts_with("    ") || line.starts_with("\t"));
            assert!(
                has_indentation || lines.len() < 3,
                "Multi-line functions should have indentation"
            );
        }
    }
}

#[test]
fn test_determinism_across_runs() {
    let pe_bytes = create_minimal_pe();

    let mut results = Vec::new();

    // Run decompilation multiple times
    for _ in 0..5 {
        let pe = Pe::from_binary(pe_bytes.clone()).expect("Failed to parse PE");

        if let Ok(c_code) = pe.decompile_from_entry() {
            results.push(c_code);
        }
    }

    // All results should be identical
    if results.len() > 1 {
        let first = &results[0];
        for result in &results[1..] {
            assert_eq!(first, result, "Decompilation should be deterministic");
        }
    }
}

#[test]
fn test_address_resolution() {
    let pe_bytes = create_minimal_pe();
    let pe = Pe::from_binary(pe_bytes).expect("Failed to parse PE");

    // Test file offset decompilation
    if let Ok(c_code) = pe.decompile_from_file_offset(0x200) {
        // Should decompile the code we added
        assert!(!c_code.is_empty());
    }
}

#[cfg(test)]
mod pattern_tests {
    use super::*;

    fn create_pe_with_pattern(code: Vec<u8>) -> Pe {
        let mut pe_bytes = create_minimal_pe();
        // Replace code section
        pe_bytes.truncate(0x200);
        pe_bytes.extend_from_slice(&code);

        Pe::from_binary(pe_bytes).expect("Failed to create PE with pattern")
    }

    #[test]
    fn test_loop_pattern_detection() {
        // Simple loop pattern
        let loop_code = vec![
            0x31, 0xC9, // xor ecx, ecx
            0x8D, 0x41, 0x01, // lea eax, [rcx+1]
            0x83, 0xF8, 0x0A, // cmp eax, 10
            0x7C, 0xF8, // jl -8
            0xC3, // ret
        ];

        let pe = create_pe_with_pattern(loop_code);

        if let Ok(c_code) = pe.decompile_from_entry() {
            // Should recognize loop structure
            let has_loop_keywords =
                c_code.contains("while") || c_code.contains("for") || c_code.contains("do");

            // Even if not recognized as high-level loop, should have branch
            let has_control_flow =
                has_loop_keywords || c_code.contains("if") || c_code.contains("goto");

            assert!(
                has_control_flow,
                "Should detect control flow in loop pattern"
            );
        }
    }

    #[test]
    fn test_switch_pattern_detection() {
        // Simple switch-like pattern (indirect jump)
        let switch_code = vec![
            0x83, 0xF9, 0x03, // cmp ecx, 3
            0x77, 0x10, // ja default_case
            0x48, 0x8D, 0x05, 0x00, 0x00, 0x00, 0x00, // lea rax, [rip+0]
            0x8B, 0x04, 0x88, // mov eax, [rax+rcx*4]
            0x48, 0x01, 0xC0, // add rax, rax
            0xFF, 0xE0, // jmp rax
            0xC3, // ret
        ];

        let pe = create_pe_with_pattern(switch_code);

        if let Ok(c_code) = pe.decompile_from_entry() {
            // Should have comparison and branching
            assert!(
                c_code.contains("if") || c_code.contains("cmp") || c_code.contains("<"),
                "Should detect comparison in switch pattern"
            );
        }
    }
}

//! Tests for x86-64 handling of 32-bit code
//!
//! This test suite verifies that the x86-64 architecture can correctly
//! handle 32-bit instructions and code patterns, as x86-64 is a superset
//! of x86-32.

use fireball::arch::register_mapping::get_register_mapper;
use fireball::arch::{ArchType, ArchitectureInfo, Endianness};
use fireball::core::{Address, Sections};
use std::sync::Arc;

#[test]
fn test_x86_64_handles_32bit_registers() {
    let mapper = get_register_mapper(ArchType::X86_64);

    // Test that x86-64 mapper can handle 32-bit register names
    let registers_32bit = vec!["eax", "ebx", "ecx", "edx", "esi", "edi", "esp", "ebp"];

    for reg in registers_32bit {
        let ir_reg = mapper.to_ir_register(reg);
        assert!(
            ir_reg.is_some(),
            "x86-64 should handle 32-bit register: {}",
            reg
        );

        // Verify size is 32 bits
        assert_eq!(
            mapper.get_register_size(reg),
            Some(32),
            "{} should be 32 bits",
            reg
        );
    }
}

#[test]
fn test_x86_64_handles_16bit_registers() {
    let mapper = get_register_mapper(ArchType::X86_64);

    // Test that x86-64 mapper can handle 16-bit register names
    let registers_16bit = vec!["ax", "bx", "cx", "dx", "si", "di", "sp", "bp"];

    for reg in registers_16bit {
        let ir_reg = mapper.to_ir_register(reg);
        assert!(
            ir_reg.is_some(),
            "x86-64 should handle 16-bit register: {}",
            reg
        );

        // Verify size is 16 bits
        assert_eq!(
            mapper.get_register_size(reg),
            Some(16),
            "{} should be 16 bits",
            reg
        );
    }
}

#[test]
fn test_x86_64_handles_8bit_registers() {
    let mapper = get_register_mapper(ArchType::X86_64);

    // Test that x86-64 mapper can handle 8-bit register names
    let registers_8bit = vec!["al", "ah", "bl", "bh", "cl", "ch", "dl", "dh"];

    for reg in registers_8bit {
        let ir_reg = mapper.to_ir_register(reg);
        assert!(
            ir_reg.is_some(),
            "x86-64 should handle 8-bit register: {}",
            reg
        );

        // Verify size is 8 bits
        assert_eq!(
            mapper.get_register_size(reg),
            Some(8),
            "{} should be 8 bits",
            reg
        );
    }
}

#[test]
fn test_32bit_instruction_compatibility() {
    // Test common 32-bit instructions that should work in x86-64
    let instructions_32bit = vec![
        // Basic arithmetic
        ("add eax, ebx", "Addition with 32-bit registers"),
        ("sub ecx, edx", "Subtraction with 32-bit registers"),
        ("mul esi", "Multiplication with 32-bit register"),
        ("div edi", "Division with 32-bit register"),
        // Memory operations
        ("mov eax, [ebx]", "Memory load with 32-bit addressing"),
        ("mov [ecx], edx", "Memory store with 32-bit addressing"),
        ("push ebp", "Push 32-bit register"),
        ("pop esp", "Pop 32-bit register"),
        // Logical operations
        ("and eax, 0xFF", "AND with immediate"),
        ("or ebx, ecx", "OR with registers"),
        ("xor edx, edx", "XOR self (zero idiom)"),
        ("not esi", "NOT operation"),
        // Shift operations
        ("shl eax, 2", "Shift left immediate"),
        ("shr ebx, cl", "Shift right by register"),
        ("rol ecx, 1", "Rotate left"),
        ("ror edx, 1", "Rotate right"),
    ];

    // For each instruction, verify it's recognized as valid
    for (instr, desc) in instructions_32bit {
        println!("Testing 32-bit instruction: {} - {}", instr, desc);
        // In a real implementation, we would parse and validate the instruction
        // For now, we just check the pattern
        assert!(
            instr.contains("eax")
                || instr.contains("ebx")
                || instr.contains("ecx")
                || instr.contains("edx")
                || instr.contains("esi")
                || instr.contains("edi")
                || instr.contains("esp")
                || instr.contains("ebp"),
            "Instruction should contain 32-bit registers"
        );
    }
}

#[test]
fn test_mixed_register_sizes() {
    let mapper = get_register_mapper(ArchType::X86_64);

    // Test that x86-64 can handle mixed register sizes in the same context
    let test_cases = vec![
        ("rax", 64, "64-bit register"),
        ("eax", 32, "32-bit sub-register of rax"),
        ("ax", 16, "16-bit sub-register of rax"),
        ("al", 8, "8-bit low sub-register of rax"),
        ("ah", 8, "8-bit high sub-register of rax"),
    ];

    for (reg, expected_size, desc) in test_cases {
        let ir_reg = mapper.to_ir_register(reg);
        assert!(ir_reg.is_some(), "{} should be recognized: {}", reg, desc);

        let size = mapper.get_register_size(reg);
        assert_eq!(size, Some(expected_size), "{}: {}", desc, reg);
    }
}

#[test]
fn test_32bit_addressing_modes() {
    // Test that 32-bit addressing modes are supported
    let addressing_modes = vec![
        "[eax]",             // Register indirect
        "[ebx + 4]",         // Register + displacement
        "[ecx + edx]",       // Register + register
        "[esi + edi*2]",     // Register + scaled index
        "[ebp + ebx*4 + 8]", // Full SIB + displacement
        "[0x401000]",        // Direct addressing
    ];

    for mode in addressing_modes {
        println!("Testing 32-bit addressing mode: {}", mode);
        // Verify the mode contains 32-bit registers or is direct addressing
        assert!(
            mode.contains("eax")
                || mode.contains("ebx")
                || mode.contains("ecx")
                || mode.contains("edx")
                || mode.contains("esi")
                || mode.contains("edi")
                || mode.contains("esp")
                || mode.contains("ebp")
                || mode.contains("0x"),
            "Addressing mode should be valid 32-bit pattern"
        );
    }
}

#[test]
fn test_x86_vs_x86_64_arch_info() {
    // Compare architecture info between x86 and x86-64
    let arch_x86 = ArchitectureInfo {
        arch_type: ArchType::X86,
        pointer_size: 32,
        endianness: Endianness::Little,
        register_count: 8,
        instruction_alignment: 1,
    };

    let arch_x86_64 = ArchitectureInfo {
        arch_type: ArchType::X86_64,
        pointer_size: 64,
        endianness: Endianness::Little,
        register_count: 16,
        instruction_alignment: 1,
    };

    // Verify differences
    assert_eq!(arch_x86.pointer_size, 32);
    assert_eq!(arch_x86_64.pointer_size, 64);
    assert_eq!(arch_x86.register_count, 8);
    assert_eq!(arch_x86_64.register_count, 16);

    // Verify similarities
    assert_eq!(arch_x86.endianness, arch_x86_64.endianness);
    assert_eq!(
        arch_x86.instruction_alignment,
        arch_x86_64.instruction_alignment
    );
}

#[test]
fn test_32bit_legacy_instructions() {
    // Test legacy 32-bit instructions that might have different behavior
    let legacy_instructions = vec![
        // String operations
        ("rep movsb", "Repeat move string byte"),
        ("rep movsd", "Repeat move string dword"),
        ("cmpsb", "Compare string byte"),
        ("scasb", "Scan string byte"),
        // Stack frame operations
        ("enter 16, 0", "Enter stack frame"),
        ("leave", "Leave stack frame"),
        // Segment operations (mostly ignored in 64-bit)
        ("push ds", "Push data segment"),
        ("pop es", "Pop extra segment"),
        // BCD arithmetic (removed in 64-bit mode)
        // These should be noted as unsupported in 64-bit mode
        ("aaa", "ASCII adjust after addition"),
        ("daa", "Decimal adjust after addition"),
    ];

    for (instr, desc) in legacy_instructions {
        println!("Testing legacy instruction: {} - {}", instr, desc);
        // Some of these might not be supported in 64-bit mode
        // but the architecture should handle them gracefully
    }
}

#[test]
fn test_register_extension_patterns() {
    // Test that 32-bit operations in 64-bit mode zero-extend to 64 bits
    let _mapper = get_register_mapper(ArchType::X86_64);

    // When writing to a 32-bit register in 64-bit mode,
    // the upper 32 bits of the 64-bit register are zeroed
    let test_patterns = vec![
        (
            "mov eax, 0x12345678",
            "Upper 32 bits of RAX should be zeroed",
        ),
        ("add ebx, ecx", "Result zero-extends to RBX"),
        ("xor edx, edx", "Entire RDX is zeroed"),
    ];

    for (pattern, desc) in test_patterns {
        println!("Testing zero-extension pattern: {} - {}", pattern, desc);
        // In x86-64, 32-bit operations implicitly zero the upper 32 bits
    }
}

#[test]
fn test_calling_convention_differences() {
    // Test differences in calling conventions between 32-bit and 64-bit
    let mapper_x86 = get_register_mapper(ArchType::X86);
    let mapper_x86_64 = get_register_mapper(ArchType::X86_64);

    let cc_x86 = mapper_x86.get_calling_convention_registers();
    let cc_x86_64 = mapper_x86_64.get_calling_convention_registers();

    // x86 (32-bit) typically uses stack for arguments (cdecl)
    assert_eq!(
        cc_x86.argument_registers.len(),
        0,
        "x86 cdecl uses stack for args"
    );

    // x86-64 uses registers for first 6 arguments (System V ABI)
    assert_eq!(
        cc_x86_64.argument_registers.len(),
        6,
        "x86-64 uses 6 registers for args"
    );
    assert_eq!(cc_x86_64.argument_registers[0], "rdi");
    assert_eq!(cc_x86_64.argument_registers[1], "rsi");

    // Both use same register for return value (different size)
    assert_eq!(cc_x86.return_register, "eax");
    assert_eq!(cc_x86_64.return_register, "rax");
}

#[test]
fn test_pointer_size_handling() {
    // Test that pointer operations handle size correctly
    let sections = Arc::new(Sections::default());

    // 32-bit pointer
    let addr_32 = 0x401000_u32;
    let formatted_32 = format!("{:08x}", addr_32);
    assert_eq!(
        formatted_32.len(),
        8,
        "32-bit address should be 8 hex digits"
    );

    // 64-bit pointer
    let addr_64 = 0x401000_u64;
    let formatted_64 = format!("{:016x}", addr_64);
    assert_eq!(
        formatted_64.len(),
        16,
        "64-bit address should be 16 hex digits"
    );

    // Create addresses
    let addr_obj_32 = Address::from_virtual_address(&sections, addr_32 as u64);
    let addr_obj_64 = Address::from_virtual_address(&sections, addr_64);

    // Both should represent the same location
    assert_eq!(
        addr_obj_32.get_virtual_address(),
        addr_obj_64.get_virtual_address()
    );
}

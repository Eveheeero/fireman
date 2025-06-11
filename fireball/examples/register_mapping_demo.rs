//! Example demonstrating the use of architecture-specific register mapping

use fireball::arch::{ArchType, get_register_mapper};

fn main() {
    println!("=== Architecture-Specific Register Mapping Demo ===\n");

    // Demonstrate x86 (32-bit) register mapping
    println!("x86 (32-bit) Architecture:");
    let x86_mapper = get_register_mapper(ArchType::X86);

    println!("  Register sizes:");
    println!(
        "    eax: {} bits",
        x86_mapper.get_register_size("eax").unwrap()
    );
    println!(
        "    ax:  {} bits",
        x86_mapper.get_register_size("ax").unwrap()
    );
    println!(
        "    al:  {} bits",
        x86_mapper.get_register_size("al").unwrap()
    );

    println!("  Special registers:");
    println!(
        "    esp is stack pointer: {}",
        x86_mapper.is_stack_pointer("esp")
    );
    println!(
        "    ebp is frame pointer: {}",
        x86_mapper.is_frame_pointer("ebp")
    );

    let x86_cc = x86_mapper.get_calling_convention_registers();
    println!("  Calling convention (cdecl):");
    println!("    Arguments: {:?}", x86_cc.argument_registers);
    println!("    Return: {}", x86_cc.return_register);
    println!("    Caller-saved: {:?}", x86_cc.caller_saved);

    // Demonstrate x86_64 (64-bit) register mapping
    println!("\nx86_64 (64-bit) Architecture:");
    let x64_mapper = get_register_mapper(ArchType::X86_64);

    println!("  Register sizes:");
    println!(
        "    rax: {} bits",
        x64_mapper.get_register_size("rax").unwrap()
    );
    println!(
        "    eax: {} bits",
        x64_mapper.get_register_size("eax").unwrap()
    );
    println!(
        "    r8:  {} bits",
        x64_mapper.get_register_size("r8").unwrap()
    );
    println!(
        "    r8d: {} bits",
        x64_mapper.get_register_size("r8d").unwrap()
    );

    let x64_cc = x64_mapper.get_calling_convention_registers();
    println!("  Calling convention (System V):");
    println!("    Arguments: {:?}", x64_cc.argument_registers);
    println!("    Return: {}", x64_cc.return_register);

    // Demonstrate ARM32 register mapping
    println!("\nARM32 Architecture:");
    let arm32_mapper = get_register_mapper(ArchType::Arm32);

    println!("  Register sizes:");
    println!(
        "    r0: {} bits",
        arm32_mapper.get_register_size("r0").unwrap()
    );
    println!(
        "    sp: {} bits",
        arm32_mapper.get_register_size("sp").unwrap()
    );

    println!("  Special registers:");
    println!(
        "    r13 is stack pointer: {}",
        arm32_mapper.is_stack_pointer("r13")
    );
    println!(
        "    sp is stack pointer: {}",
        arm32_mapper.is_stack_pointer("sp")
    );
    println!(
        "    r15 is instruction pointer: {}",
        arm32_mapper.is_instruction_pointer("r15")
    );
    println!(
        "    pc is instruction pointer: {}",
        arm32_mapper.is_instruction_pointer("pc")
    );

    let arm32_cc = arm32_mapper.get_calling_convention_registers();
    println!("  Calling convention (AAPCS):");
    println!("    Arguments: {:?}", arm32_cc.argument_registers);
    println!("    Return: {}", arm32_cc.return_register);

    // Demonstrate ARM64 register mapping
    println!("\nARM64 Architecture:");
    let arm64_mapper = get_register_mapper(ArchType::Arm64);

    println!("  Register sizes:");
    println!(
        "    x0: {} bits",
        arm64_mapper.get_register_size("x0").unwrap()
    );
    println!(
        "    w0: {} bits",
        arm64_mapper.get_register_size("w0").unwrap()
    );

    println!("  Special registers:");
    println!(
        "    sp is stack pointer: {}",
        arm64_mapper.is_stack_pointer("sp")
    );
    println!(
        "    x29 is frame pointer: {}",
        arm64_mapper.is_frame_pointer("x29")
    );
    println!(
        "    fp is frame pointer: {}",
        arm64_mapper.is_frame_pointer("fp")
    );

    let arm64_cc = arm64_mapper.get_calling_convention_registers();
    println!("  Calling convention (AAPCS64):");
    println!("    Arguments: {:?}", arm64_cc.argument_registers);
    println!("    Return: {}", arm64_cc.return_register);
    println!(
        "    Callee-saved: {} registers",
        arm64_cc.callee_saved.len()
    );

    // Demonstrate IR register conversion
    println!("\nIR Register Conversion:");
    let reg = x64_mapper.to_ir_register("rax").unwrap();
    println!(
        "  x64 'rax' -> IR Register: {} ({} bits)",
        reg.name(),
        reg.bit_len()
    );

    let reg = arm64_mapper.to_ir_register("x0").unwrap();
    println!(
        "  ARM64 'x0' -> IR Register: {} ({} bits)",
        reg.name(),
        reg.bit_len()
    );
}

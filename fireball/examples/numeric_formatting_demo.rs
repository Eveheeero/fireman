//! Example demonstrating numeric formatting features

use fireball::{
    arch::{ArchType, ArchitectureInfo, Endianness},
    ir::analyze::ir_to_c::c_abstract_syntax_tree::Literal,
    ir::analyze::ir_to_c::enhanced_ast_generator::{
        EnhancedAstConfig, EnhancedAstGenerator, NumericFormat, format_literal,
    },
};

fn main() {
    println!("=== Numeric Formatting Demo ===\n");

    // Test values
    let test_values: Vec<i64> = vec![
        10,         // Small decimal
        42,         // Another small value
        255,        // Byte boundary
        256,        // Just over byte
        0x1000,     // Page size
        0x401000,   // Typical code address
        0x7fff0000, // Stack address
        -1,         // Negative value
    ];

    // Test all numeric formats
    let formats = vec![
        ("Hexadecimal", NumericFormat::Hexadecimal),
        ("Decimal", NumericFormat::Decimal),
        ("Binary", NumericFormat::Binary),
        ("Auto", NumericFormat::Auto),
    ];

    for (format_name, format) in formats {
        println!("Format: {}", format_name);
        println!("  Value      ->  Formatted");
        println!("  ----------    ----------");

        for &value in &test_values {
            let lit = if value < 0 {
                Literal::Int(value)
            } else {
                Literal::UInt(value as u64)
            };

            println!(
                "  {:<10} -> {}",
                format!("{:#x}", value),
                format_literal(&lit, format)
            );
        }
        println!();
    }

    // Demonstrate architecture-aware address formatting
    println!("Architecture-Aware Address Formatting:");

    let arch_32 = ArchitectureInfo {
        arch_type: ArchType::X86,
        pointer_size: 32,
        endianness: Endianness::Little,
        register_count: 8,
        instruction_alignment: 1,
    };

    let arch_64 = ArchitectureInfo {
        arch_type: ArchType::X86_64,
        pointer_size: 64,
        endianness: Endianness::Little,
        register_count: 16,
        instruction_alignment: 1,
    };

    // Create generators with different architectures
    let config_32 = EnhancedAstConfig {
        architecture: Some(arch_32),
        numeric_format: NumericFormat::Hexadecimal,
        ..Default::default()
    };

    let config_64 = EnhancedAstConfig {
        architecture: Some(arch_64),
        numeric_format: NumericFormat::Hexadecimal,
        ..Default::default()
    };

    let gen_32 = EnhancedAstGenerator::new(config_32);
    let gen_64 = EnhancedAstGenerator::new(config_64);

    println!("\n32-bit Architecture:");
    println!("  0x1000     -> {}", gen_32.format_address(0x1000));
    println!("  0x401000   -> {}", gen_32.format_address(0x401000));
    println!("  0x7fff0000 -> {}", gen_32.format_address(0x7fff0000));

    println!("\n64-bit Architecture:");
    println!("  0x1000           -> {}", gen_64.format_address(0x1000));
    println!("  0x401000         -> {}", gen_64.format_address(0x401000));
    println!(
        "  0x7ffff7dd2000   -> {}",
        gen_64.format_address(0x7ffff7dd2000)
    );

    // Demonstrate Auto format heuristics
    println!("\nAuto Format Heuristics:");
    println!("(Uses decimal for small values, hex for addresses/large values)");

    let auto_config = EnhancedAstConfig {
        numeric_format: NumericFormat::Auto,
        ..Default::default()
    };
    let auto_gen = EnhancedAstGenerator::new(auto_config);

    for &value in &test_values {
        let lit = auto_gen.create_literal(value);
        println!(
            "  {:<10} -> {}",
            value,
            format_literal(&lit, NumericFormat::Auto)
        );
    }
}

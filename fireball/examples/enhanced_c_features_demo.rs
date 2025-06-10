//! Example demonstrating Enhanced C features including numeric formatting and architecture awareness

use fireball::{
    arch::{ArchType, ArchitectureInfo, Endianness},
    ir::analyze::ir_to_c::c_abstract_syntax_tree::{CType, Literal},
    ir::analyze::ir_to_c::enhanced_ast_generator::{
        EnhancedAstConfig, EnhancedAstGenerator, NumericFormat, format_literal,
    },
};

fn main() {
    println!("=== Enhanced C Features Demo ===\n");

    // Create configurations for different architectures and numeric formats
    let configs = vec![
        (
            "x86 with Hex",
            create_config(ArchType::X86, NumericFormat::Hexadecimal),
        ),
        (
            "x86 with Dec",
            create_config(ArchType::X86, NumericFormat::Decimal),
        ),
        (
            "x86_64 with Hex",
            create_config(ArchType::X86_64, NumericFormat::Hexadecimal),
        ),
        (
            "x86_64 with Binary",
            create_config(ArchType::X86_64, NumericFormat::Binary),
        ),
        (
            "ARM64 with Auto",
            create_config(ArchType::Arm64, NumericFormat::Auto),
        ),
    ];

    for (name, config) in configs {
        println!("Configuration: {}", name);
        let generator = EnhancedAstGenerator::new(config);

        // Demonstrate type sizing
        println!("  Type Sizing:");
        println!("    size_t: {:?}", generator.get_size_type());
        println!("    ptrdiff_t: {:?}", generator.get_ptrdiff_type());
        println!(
            "    register type (signed): {:?}",
            generator.get_register_type(true)
        );
        println!("    address type: {:?}", generator.get_address_type());

        // Demonstrate literal formatting
        println!("  Literal Formatting:");
        let small_val = generator.create_literal(42);
        let large_val = generator.create_literal(0x1234);
        let address_val = generator.create_literal(0x401000);

        println!(
            "    42 -> {}",
            format_literal(&small_val, generator.config.numeric_format)
        );
        println!(
            "    0x1234 -> {}",
            format_literal(&large_val, generator.config.numeric_format)
        );
        println!(
            "    0x401000 -> {}",
            format_literal(&address_val, generator.config.numeric_format)
        );

        // Demonstrate address formatting
        println!("  Address Formatting:");
        println!("    0x1000 -> {}", generator.format_address(0x1000));
        println!("    0x7FFF0000 -> {}", generator.format_address(0x7FFF0000));
        println!(
            "    0xFFFFFFFF12345678 -> {}",
            generator.format_address(0xFFFFFFFF12345678)
        );

        // Demonstrate type sizes
        println!("  Type Sizes:");
        println!(
            "    int: {} bytes",
            generator.get_type_size(&CType::Int).unwrap_or(0)
        );
        println!(
            "    pointer: {} bytes",
            generator
                .get_type_size(&CType::Pointer(Box::new(CType::Void)))
                .unwrap_or(0)
        );
        println!(
            "    int64_t: {} bytes",
            generator.get_type_size(&CType::Int64).unwrap_or(0)
        );

        println!();
    }

    // Demonstrate Auto format heuristics
    println!("Auto Format Heuristics:");
    let auto_config = create_config(ArchType::X86_64, NumericFormat::Auto);
    let auto_gen = EnhancedAstGenerator::new(auto_config);

    let test_values = vec![10, 100, 255, 256, 0x1000, 0x401000, -1];
    for val in test_values {
        let lit = auto_gen.create_literal(val);
        println!("  {} -> {}", val, format_literal(&lit, NumericFormat::Auto));
    }
}

fn create_config(arch: ArchType, format: NumericFormat) -> EnhancedAstConfig {
    EnhancedAstConfig {
        architecture: Some(ArchitectureInfo {
            arch_type: arch,
            pointer_size: match arch {
                ArchType::X86 | ArchType::Arm32 => 32,
                ArchType::X86_64 | ArchType::Arm64 => 64,
                _ => 64,
            },
            endianness: Endianness::Little,
            register_count: match arch {
                ArchType::X86 => 8,
                ArchType::X86_64 => 16,
                ArchType::Arm32 => 16,
                ArchType::Arm64 => 31,
                _ => 0,
            },
            instruction_alignment: match arch {
                ArchType::X86 | ArchType::X86_64 => 1,
                ArchType::Arm32 | ArchType::Arm64 => 4,
                _ => 1,
            },
        }),
        numeric_format: format,
        ..Default::default()
    }
}

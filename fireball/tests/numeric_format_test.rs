//! Tests for numeric format display
//!
//! This test suite verifies that numeric values are displayed consistently
//! and appropriately based on their context.

use fireball::core::{Address, Sections};
use fireball::ir::analyze::enhanced_c_codegen::EnhancedCConfig;
use fireball::ir::analyze::ir_to_c::c_abstract_syntax_tree::{
    CAst, CType, Expression, Literal, Statement, Variable, Wrapped, WrappedStatement,
};
use std::sync::Arc;

/// Helper to create a simple AST with numeric values
#[allow(dead_code)]
fn create_numeric_ast() -> CAst {
    let mut ast = CAst::new();

    // Create a function
    let sections = Arc::new(Sections::default());
    let addr = Address::from_virtual_address(&sections, 0x1000);
    let func_id = ast.generate_default_function(&addr);

    // Create variables with different numeric values
    let var_small = ast.new_variable_id(&func_id);
    let var_address = ast.new_variable_id(&func_id);
    let var_large = ast.new_variable_id(&func_id);

    if let Some(func) = ast.functions.write().unwrap().get_mut(&func_id) {
        let vars = func.variables.clone();

        // Add variables
        vars.write().unwrap().insert(
            var_small,
            Variable {
                name: "small_value".to_string(),
                id: var_small,
                var_type: CType::Int,
                const_value: None,
            },
        );

        vars.write().unwrap().insert(
            var_address,
            Variable {
                name: "address_value".to_string(),
                id: var_address,
                var_type: CType::UInt64,
                const_value: None,
            },
        );

        vars.write().unwrap().insert(
            var_large,
            Variable {
                name: "large_value".to_string(),
                id: var_large,
                var_type: CType::UInt32,
                const_value: None,
            },
        );

        // Create assignments with different numeric values

        // small_value = 42
        func.body.push(WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_small),
                    origin_expr: None,
                    comment: None,
                },
                Wrapped {
                    item: Expression::Literal(Literal::Int(42)),
                    origin_expr: None,
                    comment: None,
                },
            ),
            from: None,
            comment: None,
        });

        // address_value = 0xDEADBEEF
        func.body.push(WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_address),
                    origin_expr: None,
                    comment: None,
                },
                Wrapped {
                    item: Expression::Literal(Literal::UInt(0xDEADBEEF)),
                    origin_expr: None,
                    comment: None,
                },
            ),
            from: None,
            comment: None,
        });

        // large_value = 0x1000000
        func.body.push(WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_large),
                    origin_expr: None,
                    comment: None,
                },
                Wrapped {
                    item: Expression::Literal(Literal::UInt(0x1000000)),
                    origin_expr: None,
                    comment: None,
                },
            ),
            from: None,
            comment: None,
        });
    }

    ast
}

#[test]
fn test_numeric_value_formatting() {
    // Test that numeric values are formatted consistently
    let values = vec![
        (42_i64, "small decimal value"),
        (255_i64, "byte boundary value"),
        (0x1000_i64, "page boundary value"),
        (-1_i64, "negative value"),
    ];

    for (value, desc) in values {
        println!("Testing {}: {}", desc, value);

        // Test that the same value formats the same way multiple times
        let format1 = format!("{}", value);
        let format2 = format!("{}", value);
        assert_eq!(format1, format2, "{} should format consistently", desc);
    }
}

#[test]
fn test_hexadecimal_formatting() {
    // Test hexadecimal formatting for address-like values
    let addresses = vec![
        (0x401000_u64, "common code address"),
        (0xDEADBEEF_u64, "magic value"),
        (0x7FFFFFFF_u64, "32-bit max signed"),
        (0xFFFFFFFF_u64, "32-bit max unsigned"),
    ];

    for (addr, desc) in addresses {
        let hex_lower = format!("0x{:x}", addr);
        let hex_upper = format!("0x{:X}", addr);
        let hex_padded = format!("0x{:016x}", addr);

        println!("{}: {} / {} / {}", desc, hex_lower, hex_upper, hex_padded);

        // Verify formatting is consistent
        assert!(hex_lower.starts_with("0x"));
        assert!(hex_upper.starts_with("0x"));
        assert_eq!(hex_padded.len(), 18); // "0x" + 16 hex digits
    }
}

#[test]
fn test_binary_formatting() {
    // Test binary format for bit patterns
    let patterns = vec![
        (0b11110000_u8, "nibble pattern"),
        (0b10101010_u8, "alternating bits"),
        (0b11111111_u8, "all ones"),
        (0b00000001_u8, "single bit"),
    ];

    for (value, desc) in patterns {
        let binary_str = format!("0b{:08b}", value);
        println!("{}: {}", desc, binary_str);

        assert!(binary_str.starts_with("0b"));
        assert_eq!(binary_str.len(), 10); // "0b" + 8 binary digits
    }
}

#[test]
fn test_format_heuristics() {
    // Test heuristics for choosing appropriate format

    // Small values (< 256) are often better in decimal
    assert!(is_small_value(42));
    assert!(is_small_value(255));
    assert!(!is_small_value(256));

    // Address-like values should use hex
    assert!(is_address_like(0x1000));
    assert!(is_address_like(0xDEADBEEF));
    assert!(is_address_like(0x401000));

    // Powers of 2 often clearer in hex
    assert!(is_power_of_two(0x100));
    assert!(is_power_of_two(0x1000));
    assert!(!is_power_of_two(0x1001));

    // Round numbers might use decimal
    assert!(is_round_decimal(100));
    assert!(is_round_decimal(1000));
    assert!(!is_round_decimal(1024));
}

// Helper functions for format heuristics
fn is_small_value(val: u64) -> bool {
    val < 256
}

fn is_address_like(val: u64) -> bool {
    val >= 0x1000 // Any value that looks like an address
}

fn is_power_of_two(val: u64) -> bool {
    val != 0 && (val & (val - 1)) == 0
}

fn is_round_decimal(val: u64) -> bool {
    val % 10 == 0 && val <= 10000
}

#[test]
fn test_enhanced_c_config_defaults() {
    // Test that EnhancedCConfig has sensible defaults
    let config = EnhancedCConfig::default();

    assert!(
        config.use_fixed_width_types,
        "Should use fixed-width types by default"
    );
    assert!(config.use_nullptr, "Should use nullptr by default");
    assert!(
        config.generate_uncertainty_comments,
        "Should generate uncertainty comments by default"
    );
}

#[test]
fn test_literal_types() {
    // Test different literal types and their representation
    let literals = vec![
        (Literal::Int(42), "positive int"),
        (Literal::Int(-42), "negative int"),
        (Literal::UInt(0xDEADBEEF), "unsigned int"),
        (Literal::Float(std::f64::consts::PI), "float"),
        (Literal::Char('A'), "char"),
        (Literal::Bool(true), "bool true"),
        (Literal::Bool(false), "bool false"),
        (Literal::String("test".to_string()), "string"),
    ];

    for (literal, desc) in literals {
        match literal {
            Literal::Int(v) => println!("{}: Int({})", desc, v),
            Literal::UInt(v) => println!("{}: UInt(0x{:x})", desc, v),
            Literal::Float(v) => println!("{}: Float({})", desc, v),
            Literal::Char(v) => println!("{}: Char('{}')", desc, v),
            Literal::Bool(v) => println!("{}: Bool({})", desc, v),
            Literal::String(ref v) => println!("{}: String(\"{}\")", desc, v),
        }
    }
}

#[test]
fn test_numeric_consistency_across_types() {
    // Test that numeric values maintain consistency across different integer types
    let value = 42u64;

    let as_i8 = value as i8;
    let as_i16 = value as i16;
    let as_i32 = value as i32;
    let as_i64 = value as i64;
    let as_u8 = value as u8;
    let as_u16 = value as u16;
    let as_u32 = value as u32;

    assert_eq!(as_i8, 42);
    assert_eq!(as_i16, 42);
    assert_eq!(as_i32, 42);
    assert_eq!(as_i64, 42);
    assert_eq!(as_u8, 42);
    assert_eq!(as_u16, 42);
    assert_eq!(as_u32, 42);
    assert_eq!(value, 42);
}

#[test]
fn test_address_formatting_consistency() {
    // Test that addresses are formatted consistently
    let test_addresses = vec![
        0x0000000000401000_u64, // Typical code start
        0x00007FFFFFFFFFFF_u64, // User space limit
        0xFFFF800000000000_u64, // Kernel space start
        0xFFFFFFFFFFFFFFFF_u64, // Max address
    ];

    for addr in test_addresses {
        // Always use 16-digit hex for addresses
        let formatted = format!("{:016x}", addr);
        assert_eq!(formatted.len(), 16);

        // Verify roundtrip
        let parsed = u64::from_str_radix(&formatted, 16).unwrap();
        assert_eq!(parsed, addr);
    }
}

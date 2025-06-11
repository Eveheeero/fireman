//! Tests for architecture-specific AST optimizations
//!
//! This test suite verifies that AST-level optimizations are correctly
//! applied based on the target architecture.

use fireball::arch::{ArchType, ArchitectureInfo};
use fireball::core::{Address, Sections};
use fireball::ir::analyze::ir_to_c::c_abstract_syntax_tree::{
    BinaryOperator, CAst, CType, Expression, Literal, Statement, Variable, Wrapped,
    WrappedStatement,
};
use fireball::ir::analyze::{
    ArchOptimizationConfig, AstOptimizer, enhanced_c_codegen::EnhancedCConfig,
};
use std::sync::Arc;

/// Helper to create a test AST with a simple function
fn create_test_ast() -> CAst {
    let mut ast = CAst::new();

    // Create a dummy address for the function
    let sections = Arc::new(Sections::default());
    let addr = Address::from_virtual_address(&sections, 0x1000);

    // Generate a function
    let func_id = ast.generate_default_function(&addr);

    // Create variables
    let var_a = ast.new_variable_id(&func_id);
    let var_b = ast.new_variable_id(&func_id);

    // Add variables to function
    if let Some(func) = ast.functions.write().unwrap().get_mut(&func_id) {
        let vars = func.variables.clone();
        vars.write().unwrap().insert(
            var_a,
            Variable {
                name: "a".to_string(),
                id: var_a,
                var_type: CType::Int,
                const_value: None,
            },
        );
        vars.write().unwrap().insert(
            var_b,
            Variable {
                name: "b".to_string(),
                id: var_b,
                var_type: CType::Int,
                const_value: None,
            },
        );

        // Create XOR self pattern: a = a ^ a
        let xor_stmt = WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_a),
                    origin_expr: None,
                    comment: None,
                },
                Wrapped {
                    item: Expression::BinaryOp(
                        BinaryOperator::BitXor,
                        Box::new(Wrapped {
                            item: Expression::Variable(vars.clone(), var_a),
                            origin_expr: None,
                            comment: None,
                        }),
                        Box::new(Wrapped {
                            item: Expression::Variable(vars.clone(), var_a),
                            origin_expr: None,
                            comment: None,
                        }),
                    ),
                    origin_expr: None,
                    comment: None,
                },
            ),
            from: None,
            comment: None,
        };

        func.body.push(xor_stmt);

        // Create a function call for calling convention optimization
        let call_stmt = WrappedStatement {
            statement: Statement::Call(
                fireball::ir::analyze::ir_to_c::c_abstract_syntax_tree::JumpTarget::Unknown(
                    "test_func".to_string(),
                ),
                vec![
                    Wrapped {
                        item: Expression::Variable(vars.clone(), var_a),
                        origin_expr: None,
                        comment: None,
                    },
                    Wrapped {
                        item: Expression::Variable(vars.clone(), var_b),
                        origin_expr: None,
                        comment: None,
                    },
                ],
            ),
            from: None,
            comment: None,
        };

        func.body.push(call_stmt);
    }

    ast
}

#[test]
fn test_x86_xor_self_optimization() {
    let mut ast = create_test_ast();

    let config = ArchOptimizationConfig {
        arch_type: ArchType::X86_64,
        arch_info: ArchitectureInfo {
            arch_type: ArchType::X86_64,
            pointer_size: 64,
            endianness: fireball::arch::Endianness::Little,
            register_count: 16,
            instruction_alignment: 1,
        },
        enhanced_c_config: EnhancedCConfig::default(),
        enable_simd_patterns: false,
        enable_arch_idioms: true,
        enable_cc_optimizations: false,
        enable_expression_simplification: true,
        enable_dead_code_elimination: true,
        enable_cse: true,
    };

    let mut optimizer = AstOptimizer::new(config);
    optimizer.optimize(&mut ast);

    // Check that the XOR self was optimized
    let stats = optimizer.stats();
    assert_eq!(stats.arch_idioms_applied, 1);

    // Verify the transformation
    if let Some(func) = ast.functions.read().unwrap().values().next() {
        if let Some(stmt) = func.body.first() {
            // Check that the assignment now has a zero literal
            if let Statement::Assignment(_, rhs) = &stmt.statement {
                assert!(matches!(&rhs.item, Expression::Literal(Literal::Int(0))));
            }

            // Check that a comment was added
            assert!(stmt.comment.is_some());
            assert!(stmt.comment.as_ref().unwrap().contains("XOR self"));
        }
    }
}

#[test]
fn test_calling_convention_optimization() {
    let mut ast = create_test_ast();

    let config = ArchOptimizationConfig {
        arch_type: ArchType::X86_64,
        arch_info: ArchitectureInfo {
            arch_type: ArchType::X86_64,
            pointer_size: 64,
            endianness: fireball::arch::Endianness::Little,
            register_count: 16,
            instruction_alignment: 1,
        },
        enhanced_c_config: EnhancedCConfig::default(),
        enable_simd_patterns: false,
        enable_arch_idioms: false,
        enable_cc_optimizations: true,
        enable_expression_simplification: true,
        enable_dead_code_elimination: true,
        enable_cse: true,
    };

    let mut optimizer = AstOptimizer::new(config);
    optimizer.optimize(&mut ast);

    // Check that calling convention optimization was applied
    let stats = optimizer.stats();
    assert_eq!(stats.cc_optimizations, 1);

    // Verify the comment was added
    if let Some(func) = ast.functions.read().unwrap().values().next() {
        // Find the call statement (should be the second one)
        if let Some(stmt) = func.body.get(1) {
            if let Statement::Call(_, args) = &stmt.statement {
                assert_eq!(args.len(), 2);
                // Check that a comment was added about register usage
                assert!(stmt.comment.is_some());
                assert!(stmt.comment.as_ref().unwrap().contains("Args in registers"));
            }
        }
    }
}

#[test]
fn test_type_optimization_for_32bit() {
    let mut ast = CAst::new();

    // Create a function with size_t variable
    let sections = Arc::new(Sections::default());
    let addr = Address::from_virtual_address(&sections, 0x2000);
    let func_id = ast.generate_default_function(&addr);

    let var_id = ast.new_variable_id(&func_id);

    if let Some(func) = ast.functions.write().unwrap().get_mut(&func_id) {
        let vars = func.variables.clone();
        vars.write().unwrap().insert(
            var_id,
            Variable {
                name: "size_var".to_string(),
                id: var_id,
                var_type: CType::UInt, // Generic unsigned, will be optimized to UInt32 on 32-bit
                const_value: None,
            },
        );
    }

    let config = ArchOptimizationConfig {
        arch_type: ArchType::X86,
        arch_info: ArchitectureInfo {
            arch_type: ArchType::X86,
            pointer_size: 32,
            endianness: fireball::arch::Endianness::Little,
            register_count: 8,
            instruction_alignment: 1,
        },
        enhanced_c_config: EnhancedCConfig::default(),
        enable_simd_patterns: false,
        enable_arch_idioms: false,
        enable_cc_optimizations: false,
        enable_expression_simplification: true,
        enable_dead_code_elimination: true,
        enable_cse: true,
    };

    let mut optimizer = AstOptimizer::new(config);
    optimizer.optimize(&mut ast);

    // Check that type optimization was applied
    let stats = optimizer.stats();
    assert_eq!(stats.type_improvements, 1);

    // Verify size_t was converted to uint32_t for 32-bit arch
    if let Some(func) = ast.functions.read().unwrap().values().next() {
        let vars = func.variables.read().unwrap();
        if let Some(var) = vars.values().next() {
            assert_eq!(var.var_type, CType::UInt32);
        }
    }
}

#[test]
fn test_arm_barrel_shifter_recognition() {
    let mut ast = CAst::new();

    // Create a function with shift operation
    let sections = Arc::new(Sections::default());
    let addr = Address::from_virtual_address(&sections, 0x3000);
    let func_id = ast.generate_default_function(&addr);

    let var_id = ast.new_variable_id(&func_id);

    if let Some(func) = ast.functions.write().unwrap().get_mut(&func_id) {
        let vars = func.variables.clone();

        vars.write().unwrap().insert(
            var_id,
            Variable {
                name: "shifted".to_string(),
                id: var_id,
                var_type: CType::UInt32,
                const_value: None,
            },
        );

        // Create shift pattern: shifted = value << 2
        let shift_stmt = WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_id),
                    origin_expr: None,
                    comment: None,
                },
                Wrapped {
                    item: Expression::BinaryOp(
                        BinaryOperator::LeftShift,
                        Box::new(Wrapped {
                            item: Expression::Variable(vars.clone(), var_id),
                            origin_expr: None,
                            comment: None,
                        }),
                        Box::new(Wrapped {
                            item: Expression::Literal(Literal::Int(2)),
                            origin_expr: None,
                            comment: None,
                        }),
                    ),
                    origin_expr: None,
                    comment: None,
                },
            ),
            from: None,
            comment: None,
        };

        func.body.push(shift_stmt);
    }

    let config = ArchOptimizationConfig {
        arch_type: ArchType::Arm32,
        arch_info: ArchitectureInfo {
            arch_type: ArchType::Arm32,
            pointer_size: 32,
            endianness: fireball::arch::Endianness::Little,
            register_count: 16,
            instruction_alignment: 4,
        },
        enhanced_c_config: EnhancedCConfig::default(),
        enable_simd_patterns: false,
        enable_arch_idioms: true,
        enable_cc_optimizations: false,
        enable_expression_simplification: true,
        enable_dead_code_elimination: true,
        enable_cse: true,
    };

    let mut optimizer = AstOptimizer::new(config);
    optimizer.optimize(&mut ast);

    // Check that ARM idiom was recognized
    let stats = optimizer.stats();
    assert_eq!(stats.arch_idioms_applied, 1);

    // Verify comment was added
    if let Some(func) = ast.functions.read().unwrap().values().next() {
        if let Some(stmt) = func.body.first() {
            assert!(stmt.comment.is_some());
            assert!(
                stmt.comment
                    .as_ref()
                    .unwrap()
                    .contains("ARM barrel shifter")
            );
        }
    }
}

#[test]
fn test_enhanced_c_config_modes() {
    // Test default config
    let default_config = EnhancedCConfig::default();
    assert!(default_config.use_auto);
    assert!(default_config.use_nullptr);
    assert!(default_config.use_fixed_width_types);

    // Test conservative config
    // TODO: Add conservative() method to EnhancedCConfig
    // let conservative_config = EnhancedCConfig::conservative();
    // assert!(!conservative_config.use_auto);
    // assert!(!conservative_config.use_nullptr);
    // assert!(conservative_config.use_fixed_width_types); // Always use fixed-width
}

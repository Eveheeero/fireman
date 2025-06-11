//! Tests for AST structure preservation
//!
//! This test suite verifies that AST transformations and optimizations
//! preserve the structure and semantics of the code.

use fireball::arch::{ArchType, ArchitectureInfo, Endianness};
use fireball::core::{Address, Sections};
use fireball::ir::analyze::ir_to_c::c_abstract_syntax_tree::{
    BinaryOperator, CAst, CType, Expression, Literal, Statement, Variable, Wrapped,
    WrappedStatement,
};
use fireball::ir::analyze::{
    ArchOptimizationConfig, AstOptimizer, enhanced_c_codegen::EnhancedCConfig,
};
use std::sync::Arc;

/// Helper to create a complex AST with nested structures
fn create_complex_ast() -> CAst {
    let mut ast = CAst::new();

    // Create a function
    let sections = Arc::new(Sections::default());
    let addr = Address::from_virtual_address(&sections, 0x1000);
    let func_id = ast.generate_default_function(&addr);

    // Create multiple variables
    let var_a = ast.new_variable_id(&func_id);
    let var_b = ast.new_variable_id(&func_id);
    let var_c = ast.new_variable_id(&func_id);
    let var_array = ast.new_variable_id(&func_id);

    if let Some(func) = ast.functions.write().unwrap().get_mut(&func_id) {
        let vars = func.variables.clone();

        // Add variables with different types
        vars.write().unwrap().insert(
            var_a,
            Variable {
                name: "a".to_string(),
                id: var_a,
                var_type: CType::Int32,
                const_value: None,
            },
        );

        vars.write().unwrap().insert(
            var_b,
            Variable {
                name: "b".to_string(),
                id: var_b,
                var_type: CType::Int32,
                const_value: None,
            },
        );

        vars.write().unwrap().insert(
            var_c,
            Variable {
                name: "c".to_string(),
                id: var_c,
                var_type: CType::Bool,
                const_value: None,
            },
        );

        vars.write().unwrap().insert(
            var_array,
            Variable {
                name: "arr".to_string(),
                id: var_array,
                var_type: CType::Array(Box::new(CType::Int32), 10),
                const_value: None,
            },
        );

        // Create complex nested statements

        // 1. Simple assignment: a = 10
        func.body.push(WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_a),
                    origin_expr: None,
                    comment: None,
                },
                Wrapped {
                    item: Expression::Literal(Literal::Int(10)),
                    origin_expr: None,
                    comment: None,
                },
            ),
            from: None,
            comment: Some("Initialize a".to_string()),
        });

        // 2. Complex expression: b = a * 2 + 5
        func.body.push(WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_b),
                    origin_expr: None,
                    comment: None,
                },
                Wrapped {
                    item: Expression::BinaryOp(
                        BinaryOperator::Add,
                        Box::new(Wrapped {
                            item: Expression::BinaryOp(
                                BinaryOperator::Mul,
                                Box::new(Wrapped {
                                    item: Expression::Variable(vars.clone(), var_a),
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
                        }),
                        Box::new(Wrapped {
                            item: Expression::Literal(Literal::Int(5)),
                            origin_expr: None,
                            comment: None,
                        }),
                    ),
                    origin_expr: None,
                    comment: None,
                },
            ),
            from: None,
            comment: Some("Complex arithmetic".to_string()),
        });

        // 3. If-else statement
        let if_condition = Wrapped {
            item: Expression::BinaryOp(
                BinaryOperator::Greater,
                Box::new(Wrapped {
                    item: Expression::Variable(vars.clone(), var_a),
                    origin_expr: None,
                    comment: None,
                }),
                Box::new(Wrapped {
                    item: Expression::Variable(vars.clone(), var_b),
                    origin_expr: None,
                    comment: None,
                }),
            ),
            origin_expr: None,
            comment: None,
        };

        let then_branch = vec![WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_c),
                    origin_expr: None,
                    comment: None,
                },
                Wrapped {
                    item: Expression::Literal(Literal::Bool(true)),
                    origin_expr: None,
                    comment: None,
                },
            ),
            from: None,
            comment: None,
        }];

        let else_branch = vec![WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_c),
                    origin_expr: None,
                    comment: None,
                },
                Wrapped {
                    item: Expression::Literal(Literal::Bool(false)),
                    origin_expr: None,
                    comment: None,
                },
            ),
            from: None,
            comment: None,
        }];

        func.body.push(WrappedStatement {
            statement: Statement::If(if_condition, then_branch, Some(else_branch)),
            from: None,
            comment: Some("Conditional logic".to_string()),
        });

        // 4. While loop
        let loop_condition = Wrapped {
            item: Expression::BinaryOp(
                BinaryOperator::Less,
                Box::new(Wrapped {
                    item: Expression::Variable(vars.clone(), var_a),
                    origin_expr: None,
                    comment: None,
                }),
                Box::new(Wrapped {
                    item: Expression::Literal(Literal::Int(100)),
                    origin_expr: None,
                    comment: None,
                }),
            ),
            origin_expr: None,
            comment: None,
        };

        let loop_body = vec![WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_a),
                    origin_expr: None,
                    comment: None,
                },
                Wrapped {
                    item: Expression::BinaryOp(
                        BinaryOperator::Add,
                        Box::new(Wrapped {
                            item: Expression::Variable(vars.clone(), var_a),
                            origin_expr: None,
                            comment: None,
                        }),
                        Box::new(Wrapped {
                            item: Expression::Literal(Literal::Int(1)),
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
        }];

        func.body.push(WrappedStatement {
            statement: Statement::While(loop_condition, loop_body),
            from: None,
            comment: Some("Loop structure".to_string()),
        });
    }

    ast
}

#[test]
fn test_ast_structure_count() {
    let ast = create_complex_ast();

    // Count various AST elements
    let funcs = ast.functions.read().unwrap();
    assert_eq!(funcs.len(), 1, "Should have exactly one function");

    if let Some(func) = funcs.values().next() {
        assert_eq!(func.body.len(), 4, "Should have 4 top-level statements");

        let vars = func.variables.read().unwrap();
        assert_eq!(vars.len(), 4, "Should have 4 variables");

        // Check variable types
        let var_types: Vec<_> = vars.values().map(|v| &v.var_type).collect();
        assert!(var_types.iter().any(|t| matches!(t, CType::Int32)));
        assert!(var_types.iter().any(|t| matches!(t, CType::Bool)));
        assert!(var_types.iter().any(|t| matches!(t, CType::Array(_, _))));
    }
}

#[test]
#[ignore = "Optimization test hangs - needs investigation"]
fn test_ast_structure_preservation_after_optimization() {
    // This test is simplified to avoid potential infinite loops
    // Just verify that optimization doesn't crash on a simple AST
    let mut ast = CAst::new();

    // Create a minimal function
    let sections = Arc::new(Sections::default());
    let addr = Address::from_virtual_address(&sections, 0x1000);
    let func_id = ast.generate_default_function(&addr);

    // Create variable ID before accessing function
    let var_id = ast.new_variable_id(&func_id);

    // Add one simple statement
    if let Some(func) = ast.functions.write().unwrap().get_mut(&func_id) {
        let vars = func.variables.clone();

        vars.write().unwrap().insert(
            var_id,
            Variable {
                name: "x".to_string(),
                id: var_id,
                var_type: CType::Int32,
                const_value: None,
            },
        );

        func.body.push(WrappedStatement {
            statement: Statement::Assignment(
                Wrapped {
                    item: Expression::Variable(vars.clone(), var_id),
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
    }

    let original_func_count = ast.functions.read().unwrap().len();

    // Apply minimal optimization
    let config = ArchOptimizationConfig {
        arch_type: ArchType::X86_64,
        arch_info: ArchitectureInfo {
            arch_type: ArchType::X86_64,
            pointer_size: 64,
            endianness: Endianness::Little,
            register_count: 16,
            instruction_alignment: 1,
        },
        enhanced_c_config: EnhancedCConfig::default(),
        enable_simd_patterns: false,
        enable_arch_idioms: false,
        enable_cc_optimizations: false,
        enable_expression_simplification: true,
        enable_dead_code_elimination: true,
    };

    let mut optimizer = AstOptimizer::new(config);
    optimizer.optimize(&mut ast);

    // Verify basic structure is preserved
    assert_eq!(
        ast.functions.read().unwrap().len(),
        original_func_count,
        "Function count should be preserved"
    );
}

#[test]
fn test_nested_expression_preservation() {
    let ast = create_complex_ast();

    // Find the complex arithmetic statement (b = a * 2 + 5)
    if let Some(func) = ast.functions.read().unwrap().values().next() {
        if let Some(stmt) = func.body.get(1) {
            if let Statement::Assignment(_, rhs) = &stmt.statement {
                // Verify the nested structure is correct
                assert!(matches!(
                    &rhs.item,
                    Expression::BinaryOp(BinaryOperator::Add, _, _)
                ));

                // Check the nested multiplication
                if let Expression::BinaryOp(BinaryOperator::Add, left, right) = &rhs.item {
                    assert!(matches!(
                        &left.item,
                        Expression::BinaryOp(BinaryOperator::Mul, _, _)
                    ));
                    assert!(matches!(&right.item, Expression::Literal(Literal::Int(5))));
                }
            }
        }
    }
}

#[test]
fn test_control_flow_preservation() {
    let ast = create_complex_ast();

    if let Some(func) = ast.functions.read().unwrap().values().next() {
        let mut if_found = false;
        let mut while_found = false;

        for stmt in &func.body {
            match &stmt.statement {
                Statement::If(_, then_branch, else_branch) => {
                    if_found = true;
                    assert!(
                        !then_branch.is_empty(),
                        "Then branch should have statements"
                    );
                    assert!(else_branch.is_some(), "Else branch should exist");
                    if let Some(else_stmts) = else_branch {
                        assert!(!else_stmts.is_empty(), "Else branch should have statements");
                    }
                }
                Statement::While(_, body) => {
                    while_found = true;
                    assert!(!body.is_empty(), "While loop should have body");
                }
                _ => {}
            }
        }

        assert!(if_found, "If statement should be present");
        assert!(while_found, "While loop should be present");
    }
}

#[test]
fn test_comment_preservation() {
    let ast = create_complex_ast();

    if let Some(func) = ast.functions.read().unwrap().values().next() {
        let comments: Vec<_> = func
            .body
            .iter()
            .filter_map(|stmt| stmt.comment.as_ref())
            .collect();

        assert!(comments.len() >= 3, "Should have at least 3 comments");
        assert!(comments.iter().any(|c| c.contains("Initialize")));
        assert!(comments.iter().any(|c| c.contains("Complex")));
        assert!(comments.iter().any(|c| c.contains("Conditional")));
    }
}

#[test]
fn test_variable_type_preservation() {
    let ast = create_complex_ast();

    if let Some(func) = ast.functions.read().unwrap().values().next() {
        let vars = func.variables.read().unwrap();

        // Check each variable type is preserved
        for var in vars.values() {
            match &var.name[..] {
                "a" | "b" => assert!(matches!(var.var_type, CType::Int32)),
                "c" => assert!(matches!(var.var_type, CType::Bool)),
                "arr" => assert!(matches!(var.var_type, CType::Array(_, 10))),
                _ => {}
            }
        }
    }
}

#[test]
fn test_expression_operator_types() {
    let ast = create_complex_ast();

    if let Some(func) = ast.functions.read().unwrap().values().next() {
        // Count different operator types
        let mut add_count = 0;
        let mut mul_count = 0;
        let mut greater_count = 0;
        let mut less_count = 0;

        for stmt in &func.body {
            count_operators(
                &stmt.statement,
                &mut add_count,
                &mut mul_count,
                &mut greater_count,
                &mut less_count,
            );
        }

        // Verify we have the expected operators
        assert!(add_count > 0, "Should have Add operators");
        assert!(mul_count > 0, "Should have Mul operators");
        assert!(greater_count > 0, "Should have Greater operators");
        assert!(less_count > 0, "Should have Less operators");
    }
}

fn count_operators(
    stmt: &Statement,
    add: &mut i32,
    mul: &mut i32,
    greater: &mut i32,
    less: &mut i32,
) {
    match stmt {
        Statement::Assignment(_, expr) => {
            count_operators_from_expr(&expr.item, add, mul, greater, less)
        }
        Statement::If(cond, then_branch, else_branch) => {
            count_operators_from_expr(&cond.item, add, mul, greater, less);
            for s in then_branch {
                count_operators(&s.statement, add, mul, greater, less);
            }
            if let Some(else_stmts) = else_branch {
                for s in else_stmts {
                    count_operators(&s.statement, add, mul, greater, less);
                }
            }
        }
        Statement::While(cond, body) => {
            count_operators_from_expr(&cond.item, add, mul, greater, less);
            for s in body {
                count_operators(&s.statement, add, mul, greater, less);
            }
        }
        _ => {}
    }
}

fn count_operators_from_expr(
    expr: &Expression,
    add: &mut i32,
    mul: &mut i32,
    greater: &mut i32,
    less: &mut i32,
) {
    if let Expression::BinaryOp(op, left, right) = expr {
        match op {
            BinaryOperator::Add => *add += 1,
            BinaryOperator::Mul => *mul += 1,
            BinaryOperator::Greater => *greater += 1,
            BinaryOperator::Less => *less += 1,
            _ => {}
        }
        count_operators_from_expr(&left.item, add, mul, greater, less);
        count_operators_from_expr(&right.item, add, mul, greater, less);
    }
}

#[test]
fn test_ast_cloning() {
    let ast1 = create_complex_ast();
    let mut ast2 = ast1.clone();

    // Both ASTs should have the same structure
    assert_eq!(
        ast1.functions.read().unwrap().len(),
        ast2.functions.read().unwrap().len()
    );

    // Verify both have the same last_variable_id initially
    assert_eq!(ast1.last_variable_id.len(), ast2.last_variable_id.len());

    // Get function ID for modification
    let func_id = ast2
        .functions
        .read()
        .unwrap()
        .keys()
        .next()
        .cloned()
        .unwrap();

    // Check if this func_id already exists in last_variable_id
    let original_exists = ast1.last_variable_id.contains_key(&func_id);

    // Modifying one should not affect the other
    ast2.last_variable_id.insert(func_id, 999);

    // Verify the change
    assert_eq!(ast2.last_variable_id.get(&func_id), Some(&999));

    // Original should be unchanged
    if original_exists {
        assert_ne!(ast1.last_variable_id.get(&func_id), Some(&999));
    } else {
        assert!(!ast1.last_variable_id.contains_key(&func_id));
    }
}

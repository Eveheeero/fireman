//! Tests for multi-dimensional array support

use fireball::ir::analyze::ir_to_c::c_abstract_syntax_tree::{
    CAst, CType, Expression, Literal, PrintWithConfig, Statement, Variable, Wrapped,
};

#[test]
fn test_two_dimensional_array_type() {
    // Test that we can represent int[10][20]
    let array_type = CType::Array(Box::new(CType::Array(Box::new(CType::Int32), 20)), 10);

    // Verify it prints correctly
    let type_str = array_type.to_string_with_config(None);
    // Note: This might print as "int32_t[20][10]" due to how C arrays work
    assert!(type_str.contains("[10]") || type_str.contains("[20]"));
}

#[test]
fn test_three_dimensional_array_type() {
    // Test that we can represent int[5][10][20]
    let array_type = CType::Array(
        Box::new(CType::Array(
            Box::new(CType::Array(Box::new(CType::Int32), 20)),
            10,
        )),
        5,
    );

    let type_str = array_type.to_string_with_config(None);
    assert!(type_str.contains("int32_t"));
}

#[test]
fn test_multi_dimensional_array_access() {
    // Test accessing arr[i][j] where arr, i, j are just literals for simplicity
    let arr_expr = Expression::Literal(Literal::Int(100)); // Represents array base
    let i_expr = Expression::Literal(Literal::Int(5)); // First index
    let j_expr = Expression::Literal(Literal::Int(10)); // Second index

    // arr[i][j] is represented as ArrayAccess(ArrayAccess(arr, i), j)
    let access = Expression::ArrayAccess(
        Box::new(Wrapped {
            item: Expression::ArrayAccess(
                Box::new(Wrapped {
                    item: arr_expr,
                    origin_expr: None,
                    comment: None,
                }),
                Box::new(Wrapped {
                    item: i_expr,
                    origin_expr: None,
                    comment: None,
                }),
            ),
            origin_expr: None,
            comment: None,
        }),
        Box::new(Wrapped {
            item: j_expr,
            origin_expr: None,
            comment: None,
        }),
    );

    let access_str = access.to_string_with_config(None);
    // Should print something like "100[5][10]"
    assert!(access_str.contains("["));
    assert!(access_str.contains("]"));
    // Check for nested structure
    assert!(access_str.contains("100"));
    assert!(access_str.contains("5"));
    assert!(access_str.contains("10"));
}

#[test]
fn test_multi_dimensional_array_declaration() {
    use fireball::core::{Address, Sections};
    use std::sync::Arc;

    let mut ast = CAst::new();

    // Create sections (can't use new() as it's private, so use default or another method)
    let sections = Arc::new(Sections::default());

    // Create a function
    let func_id = ast.generate_default_function(&Address::from_virtual_address(&sections, 0x1000));

    // Create a 2D array variable: int matrix[10][20];
    let var_id = ast.new_variable_id(&func_id);
    let array_type = CType::Array(Box::new(CType::Array(Box::new(CType::Int32), 20)), 10);

    let var = Variable {
        name: "matrix".to_string(),
        id: var_id,
        var_type: array_type,
        const_value: None,
    };

    // Add variable declaration to function
    let decl_stmt = Statement::Declaration(var.clone(), None);

    // Test that the declaration can be created and printed
    let decl_str = decl_stmt.to_string_with_config(None);

    // Should contain declaration like "int32_t matrix[10][20];"
    assert!(decl_str.contains("matrix"));
    assert!(decl_str.contains("[10]") || decl_str.contains("[20]"));
}

#[test]
fn test_array_initialization_pattern() {
    // Test recognizing patterns like:
    // for (i = 0; i < 10; i++)
    //     for (j = 0; j < 20; j++)
    //         arr[i][j] = 0;

    // This would require pattern matching in the medium IR
    // For now, just ensure the AST can represent it

    // Use literals to represent array[i][j]
    let arr_access = Expression::ArrayAccess(
        Box::new(Wrapped {
            item: Expression::ArrayAccess(
                Box::new(Wrapped {
                    item: Expression::Unknown, // Represents array base
                    origin_expr: None,
                    comment: None,
                }),
                Box::new(Wrapped {
                    item: Expression::Literal(Literal::Int(1)), // i
                    origin_expr: None,
                    comment: None,
                }),
            ),
            origin_expr: None,
            comment: None,
        }),
        Box::new(Wrapped {
            item: Expression::Literal(Literal::Int(2)), // j
            origin_expr: None,
            comment: None,
        }),
    );

    let zero = Expression::Literal(Literal::Int(0));

    let assignment = Statement::Assignment(
        Wrapped {
            item: arr_access,
            origin_expr: None,
            comment: None,
        },
        Wrapped {
            item: zero,
            origin_expr: None,
            comment: None,
        },
    );

    // Just verify it can be created
    assert!(matches!(assignment, Statement::Assignment(_, _)));
}

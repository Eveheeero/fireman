use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstCall, AstExpression, AstFunction, AstFunctionId,
        AstFunctionVersion, AstLiteral, AstOptimizationConfig, AstPrintConfig, AstStatement,
        AstUnaryOperator, AstValue, AstValueType, AstVariable, AstVariableId, Wrapped,
    },
    core::{Instruction, Sections},
    ir::{Ir, analyze::IrFunction, statements::IrStatement, utils::IrStatementDescriptor},
    utils::version_map::VersionMap,
};
use hashbrown::HashMap;
use num_bigint::BigInt;
use std::sync::{Arc, RwLock};

fn wrap<T>(item: T) -> Wrapped<T> {
    Wrapped {
        item,
        comment: None,
    }
}

fn build_test_function(
    function_id: AstFunctionId,
    function_name: &str,
    body: Vec<Wrapped<AstStatement>>,
    variables: Arc<RwLock<HashMap<AstVariableId, AstVariable>>>,
) -> AstFunction {
    let instructions: Arc<[Instruction]> = Vec::<Instruction>::new().into();
    let ir = Arc::new(IrFunction::new(instructions, Vec::new(), Vec::new()));
    AstFunction {
        name: Some(function_name.to_string()),
        id: function_id,
        origin_ir: ir,
        return_type: AstValueType::Int,
        parameters: Vec::new(),
        variables,
        body,
        processed_optimizations: Vec::new(),
    }
}

#[test]
fn optimize_constant_folding_and_propagation() {
    let function_id = AstFunctionId { address: 0x1000 };
    let version = AstFunctionVersion(1);
    let var_a = AstVariableId {
        index: 1,
        parent: Some(function_id),
    };
    let var_b = AstVariableId {
        index: 2,
        parent: Some(function_id),
    };
    let variable_map = Arc::new(RwLock::new(HashMap::from([
        (
            var_a,
            AstVariable {
                name: Some("a".to_string()),
                id: var_a,
                var_type: AstValueType::Int,
                const_value: None,
                data_access_ir: None,
            },
        ),
        (
            var_b,
            AstVariable {
                name: Some("b".to_string()),
                id: var_b,
                var_type: AstValueType::Int,
                const_value: None,
                data_access_ir: None,
            },
        ),
    ])));

    let body = vec![
        wrap(AstStatement::Declaration(
            variable_map.read().unwrap().get(&var_a).unwrap().clone(),
            Some(wrap(AstExpression::Literal(AstLiteral::Int(2)))),
        )),
        wrap(AstStatement::Declaration(
            variable_map.read().unwrap().get(&var_b).unwrap().clone(),
            Some(wrap(AstExpression::BinaryOp(
                crate::abstract_syntax_tree::AstBinaryOperator::Add,
                Box::new(wrap(AstExpression::Variable(variable_map.clone(), var_a))),
                Box::new(wrap(AstExpression::Literal(AstLiteral::Int(3)))),
            ))),
        )),
        wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(variable_map.clone(), var_a)),
            wrap(AstExpression::BinaryOp(
                crate::abstract_syntax_tree::AstBinaryOperator::Mul,
                Box::new(wrap(AstExpression::Variable(variable_map.clone(), var_b))),
                Box::new(wrap(AstExpression::Literal(AstLiteral::Int(1)))),
            )),
        )),
        wrap(AstStatement::Return(Some(wrap(AstExpression::BinaryOp(
            crate::abstract_syntax_tree::AstBinaryOperator::Add,
            Box::new(wrap(AstExpression::Variable(variable_map.clone(), var_a))),
            Box::new(wrap(AstExpression::Literal(AstLiteral::Int(0)))),
        ))))),
    ];

    let function = build_test_function(function_id, "test_fn", body, variable_map.clone());
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::none()
                .constant_folding(true)
                .max_pass_iterations(2),
        ))
        .unwrap();

    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("return 5;"),
        "constant propagation/folding should simplify to return 5, got:\n{}",
        printed
    );
}

#[test]
fn optimize_control_flow_cleanup_removes_unreachable_tail() {
    let function_id = AstFunctionId { address: 0x1000 };
    let version = AstFunctionVersion(1);
    let var_a = AstVariableId {
        index: 1,
        parent: Some(function_id),
    };
    let variable_map = Arc::new(RwLock::new(HashMap::from([(
        var_a,
        AstVariable {
            name: Some("a".to_string()),
            id: var_a,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));

    let body = vec![
        wrap(AstStatement::Return(None)),
        wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(variable_map.clone(), var_a)),
            wrap(AstExpression::Literal(AstLiteral::Int(99))),
        )),
    ];

    let function = build_test_function(function_id, "test_fn", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::none().control_flow_cleanup(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));

    assert!(
        !printed.contains("99"),
        "unreachable statements after return should be removed, got:\n{}",
        printed
    );
}

#[test]
fn optimize_control_flow_cleanup_keeps_labeled_tail() {
    let function_id = AstFunctionId { address: 0x1000 };
    let version = AstFunctionVersion(1);
    let var_a = AstVariableId {
        index: 1,
        parent: Some(function_id),
    };
    let variable_map = Arc::new(RwLock::new(HashMap::from([(
        var_a,
        AstVariable {
            name: Some("a".to_string()),
            id: var_a,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));

    let body = vec![
        wrap(AstStatement::Return(None)),
        wrap(AstStatement::Label("L1".to_string())),
        wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(variable_map.clone(), var_a)),
            wrap(AstExpression::Literal(AstLiteral::Int(7))),
        )),
    ];

    let function = build_test_function(function_id, "test_fn", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::none().control_flow_cleanup(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));

    assert!(
        printed.contains("L1:"),
        "labeled tail should be preserved for potential jump targets, got:\n{}",
        printed
    );
    assert!(
        printed.contains("7"),
        "statement under reachable label should remain, got:\n{}",
        printed
    );
}

#[test]
fn optimize_control_flow_cleanup_flattens_standalone_block_without_global_pattern_matching() {
    let function_id = AstFunctionId { address: 0x1001 };
    let version = AstFunctionVersion(1);
    let var_a = AstVariableId {
        index: 1,
        parent: Some(function_id),
    };
    let variable_map = Arc::new(RwLock::new(HashMap::from([(
        var_a,
        AstVariable {
            name: Some("a".to_string()),
            id: var_a,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));

    let body = vec![
        wrap(AstStatement::Block(vec![wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(variable_map.clone(), var_a)),
            wrap(AstExpression::Literal(AstLiteral::Int(1))),
        ))])),
        wrap(AstStatement::Return(None)),
    ];

    let function = build_test_function(function_id, "test_fn", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::none().control_flow_cleanup(true),
        ))
        .unwrap();
    let body = optimized_function_body(&optimized, function_id);

    assert!(
        !body
            .iter()
            .any(|stmt| matches!(&stmt.item, AstStatement::Block(_))),
        "control-flow cleanup should still flatten standalone blocks when global pattern matching is disabled"
    );
    assert!(
        body.iter()
            .any(|stmt| matches!(&stmt.item, AstStatement::Assignment(_, _))),
        "the inner block statement should remain after flattening"
    );
}

#[test]
fn optimize_control_flow_cleanup_removes_tail_after_noreturn_function_call() {
    let caller_id = AstFunctionId { address: 0x1000 };
    let helper_id = AstFunctionId { address: 0x2000 };
    let version = AstFunctionVersion(1);
    let var_a = AstVariableId {
        index: 1,
        parent: Some(caller_id),
    };
    let caller_variable_map = Arc::new(RwLock::new(HashMap::from([(
        var_a,
        AstVariable {
            name: Some("a".to_string()),
            id: var_a,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));
    let noreturn_variable_map = Arc::new(RwLock::new(HashMap::new()));

    let caller_body = vec![
        wrap(AstStatement::Call(AstCall::Function {
            target: helper_id,
            args: Vec::new(),
        })),
        wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(caller_variable_map.clone(), var_a)),
            wrap(AstExpression::Literal(AstLiteral::Int(77))),
        )),
    ];
    let helper_body = vec![wrap(AstStatement::Call(AstCall::Unknown(
        "ext_msvcrt_dll__exit".to_string(),
        Vec::new(),
    )))];

    let caller = build_test_function(caller_id, "caller", caller_body, caller_variable_map);
    let helper = build_test_function(helper_id, "helper_nr", helper_body, noreturn_variable_map);

    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, caller));
    functions.insert(helper_id, VersionMap::new(version, helper));
    let ast = Ast {
        function_versions: HashMap::from([(caller_id, version), (helper_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize_function(
            caller_id,
            Some(AstOptimizationConfig::none().control_flow_cleanup(true)),
        )
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));

    let caller_start = printed
        .find("int caller(")
        .expect("caller function must exist");
    let caller_suffix = &printed[caller_start..];
    assert!(
        !caller_suffix.contains("77"),
        "tail after noreturn call should be removed in caller, got:\n{}",
        caller_suffix
    );
}

#[test]
fn print_aligns_local_variables_and_sorts_same_index_by_scope() {
    let caller_id = AstFunctionId { address: 0x1000 };
    let merged_scope_id = AstFunctionId { address: 0x2000 };
    let version = AstFunctionVersion(1);

    let caller_var_id = AstVariableId {
        index: 1,
        parent: Some(caller_id),
    };
    let merged_var_id = AstVariableId {
        index: 1,
        parent: Some(merged_scope_id),
    };

    let variable_map = Arc::new(RwLock::new(HashMap::from([
        (
            caller_var_id,
            AstVariable {
                name: Some("caller_var".to_string()),
                id: caller_var_id,
                var_type: AstValueType::Int,
                const_value: None,
                data_access_ir: None,
            },
        ),
        (
            merged_var_id,
            AstVariable {
                name: Some("merged_var".to_string()),
                id: merged_var_id,
                var_type: AstValueType::Int,
                const_value: Some(Wrapped {
                    item: AstValue::Num(BigInt::from(7)),
                    comment: None,
                }),
                data_access_ir: None,
            },
        ),
    ])));
    let body = vec![wrap(AstStatement::Return(None))];
    let function = build_test_function(caller_id, "caller", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(caller_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let printed = ast.print(Some(AstPrintConfig::NONE));
    let caller_pos = printed
        .find("caller_var;")
        .expect("caller variable declaration must be printed");
    let merged_pos = printed
        .find("merged_var")
        .expect("merged variable declaration must be printed");
    assert!(
        caller_pos < merged_pos,
        "caller scope variable should print before merged scope variable for same index, got:\n{}",
        printed
    );

    assert!(
        printed.contains("int caller_var;\n\n  const int merged_var = 0x7;"),
        "variables should be grouped/aligned by source function scope in print output, got:\n{}",
        printed
    );
}

#[test]
fn print_if_with_multi_statement_branch_uses_multiline_block() {
    let function_id = AstFunctionId { address: 0x3000 };
    let version = AstFunctionVersion(1);
    let var_id = AstVariableId {
        index: 1,
        parent: Some(function_id),
    };
    let variable_map = Arc::new(RwLock::new(HashMap::from([(
        var_id,
        AstVariable {
            name: Some("x".to_string()),
            id: var_id,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));

    let body = vec![wrap(AstStatement::If(
        wrap(AstExpression::Literal(AstLiteral::Bool(true))),
        vec![
            wrap(AstStatement::Assignment(
                wrap(AstExpression::Variable(variable_map.clone(), var_id)),
                wrap(AstExpression::Literal(AstLiteral::Int(1))),
            )),
            wrap(AstStatement::Assignment(
                wrap(AstExpression::Variable(variable_map.clone(), var_id)),
                wrap(AstExpression::Literal(AstLiteral::Int(2))),
            )),
        ],
        Some(vec![wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(variable_map.clone(), var_id)),
            wrap(AstExpression::Literal(AstLiteral::Int(3))),
        ))]),
    ))];

    let function = build_test_function(function_id, "if_multiline", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let printed = ast.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains(
            "if (true) {\n        x = 1;\n        x = 2;\n    } else {\n        x = 3;\n    }"
        ),
        "if true-branch with multiple statements should be printed as multiline block, got:\n{}",
        printed
    );
}

// --- Helper to build a simple test AST with N variables ---
fn build_simple_test_ast(
    _num_vars: usize,
    body: Vec<Wrapped<AstStatement>>,
    variable_map: Arc<RwLock<HashMap<AstVariableId, AstVariable>>>,
) -> Ast {
    let function_id = AstFunctionId { address: 0x9000 };
    let version = AstFunctionVersion(1);
    let function = build_test_function(function_id, "test_fn", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    }
}

fn optimized_function_body(ast: &Ast, function_id: AstFunctionId) -> Vec<Wrapped<AstStatement>> {
    let optimized_version = *ast
        .function_versions
        .get(&function_id)
        .expect("optimized function version should exist");
    let functions = ast.functions.read().unwrap();
    let versions = functions
        .get(&function_id)
        .expect("optimized function should exist");
    let function = versions
        .get(&optimized_version)
        .expect("optimized function version should exist");
    function.body.clone()
}

fn make_var_map(
    function_id: AstFunctionId,
    names: &[&str],
) -> (
    Vec<AstVariableId>,
    Arc<RwLock<HashMap<AstVariableId, AstVariable>>>,
) {
    let mut ids = Vec::new();
    let mut map = HashMap::new();
    for (i, name) in names.iter().enumerate() {
        let id = AstVariableId {
            index: (i + 1) as u32,
            parent: Some(function_id),
        };
        ids.push(id);
        map.insert(
            id,
            AstVariable {
                name: Some(name.to_string()),
                id,
                var_type: AstValueType::Int,
                const_value: None,
                data_access_ir: None,
            },
        );
    }
    (ids, Arc::new(RwLock::new(map)))
}

// ============ Phase 1 Tests: Algebraic Simplification ============

#[test]
fn optimize_same_operand_sub_to_zero() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    // return x - x; => should fold to return 0;
    let body = vec![wrap(AstStatement::Return(Some(wrap(
        AstExpression::BinaryOp(
            AstBinaryOperator::Sub,
            Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
            Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
        ),
    ))))];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("return 0;"),
        "x - x should fold to 0, got:\n{}",
        printed
    );
}

#[test]
fn optimize_same_operand_xor_to_zero() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    let body = vec![wrap(AstStatement::Return(Some(wrap(
        AstExpression::BinaryOp(
            AstBinaryOperator::BitXor,
            Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
            Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
        ),
    ))))];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("return 0;"),
        "x ^ x should fold to 0, got:\n{}",
        printed
    );
}

#[test]
fn optimize_same_operand_and_identity() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    let body = vec![wrap(AstStatement::Return(Some(wrap(
        AstExpression::BinaryOp(
            AstBinaryOperator::BitAnd,
            Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
            Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
        ),
    ))))];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("return x;"),
        "x & x should fold to x, got:\n{}",
        printed
    );
}

#[test]
fn optimize_cast_minimization_collapses_double_cast() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    let body = vec![wrap(AstStatement::Return(Some(wrap(AstExpression::Cast(
        AstValueType::Int32,
        Box::new(wrap(AstExpression::Cast(
            AstValueType::Int64,
            Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
        ))),
    )))))];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let body = optimized_function_body(&optimized, fid);
    let AstStatement::Return(Some(expr)) = &body[0].item else {
        panic!("expected optimized statement to remain a return");
    };
    let AstExpression::Cast(target_ty, inner) = &expr.item else {
        panic!("expected optimized return expression to remain a cast");
    };
    assert_eq!(*target_ty, AstValueType::Int32);
    assert!(
        matches!(inner.item, AstExpression::Variable(_, _)),
        "double cast should collapse to one cast over the original value, got {:?}",
        expr.item
    );
}

#[test]
fn optimize_cast_minimization_removes_identity_literal_cast() {
    let fid = AstFunctionId { address: 0x9000 };
    let body = vec![wrap(AstStatement::Return(Some(wrap(AstExpression::Cast(
        AstValueType::Int32,
        Box::new(wrap(AstExpression::Literal(AstLiteral::Int(42)))),
    )))))];

    let ast = build_simple_test_ast(0, body, Arc::new(RwLock::new(HashMap::new())));
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let body = optimized_function_body(&optimized, fid);
    let AstStatement::Return(Some(expr)) = &body[0].item else {
        panic!("expected optimized statement to remain a return");
    };
    assert!(
        matches!(expr.item, AstExpression::Literal(AstLiteral::Int(42))),
        "identity literal cast should be removed, got {:?}",
        expr.item
    );
}

#[test]
fn optimize_cast_minimization_collapses_double_unary_cast() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    let body = vec![wrap(AstStatement::Return(Some(wrap(
        AstExpression::UnaryOp(
            AstUnaryOperator::CastSigned,
            Box::new(wrap(AstExpression::UnaryOp(
                AstUnaryOperator::CastSigned,
                Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
            ))),
        ),
    ))))];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let body = optimized_function_body(&optimized, fid);
    let AstStatement::Return(Some(expr)) = &body[0].item else {
        panic!("expected optimized statement to remain a return");
    };
    let AstExpression::UnaryOp(operator, inner) = &expr.item else {
        panic!("expected optimized return expression to remain a unary cast");
    };
    assert!(matches!(operator, AstUnaryOperator::CastSigned));
    assert!(
        matches!(inner.item, AstExpression::Variable(_, _)),
        "double unary cast should collapse to one cast, got {:?}",
        expr.item
    );
}

#[test]
fn optimize_cast_minimization_drops_unsigned_before_signed_cast() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    let body = vec![wrap(AstStatement::Return(Some(wrap(
        AstExpression::UnaryOp(
            AstUnaryOperator::CastSigned,
            Box::new(wrap(AstExpression::UnaryOp(
                AstUnaryOperator::CastUnsigned,
                Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
            ))),
        ),
    ))))];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let body = optimized_function_body(&optimized, fid);
    let AstStatement::Return(Some(expr)) = &body[0].item else {
        panic!("expected optimized statement to remain a return");
    };
    let AstExpression::UnaryOp(operator, inner) = &expr.item else {
        panic!("expected optimized return expression to remain a unary cast");
    };
    assert!(matches!(operator, AstUnaryOperator::CastSigned));
    assert!(
        matches!(inner.item, AstExpression::Variable(_, _)),
        "signed-over-unsigned cast should keep only the outer signed cast, got {:?}",
        expr.item
    );
}

#[test]
fn optimize_same_operand_eq_to_true() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    let body = vec![wrap(AstStatement::Return(Some(wrap(
        AstExpression::BinaryOp(
            AstBinaryOperator::Equal,
            Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
            Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
        ),
    ))))];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("return true;"),
        "x == x should fold to true, got:\n{}",
        printed
    );
}

#[test]
fn optimize_double_bitnot_cancellation() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    let body = vec![wrap(AstStatement::Return(Some(wrap(
        AstExpression::UnaryOp(
            AstUnaryOperator::BitNot,
            Box::new(wrap(AstExpression::UnaryOp(
                AstUnaryOperator::BitNot,
                Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
            ))),
        ),
    ))))];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("return x;"),
        "~~x should fold to x, got:\n{}",
        printed
    );
}

#[test]
fn optimize_absorbing_mul_zero() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    let body = vec![wrap(AstStatement::Return(Some(wrap(
        AstExpression::BinaryOp(
            AstBinaryOperator::Mul,
            Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
            Box::new(wrap(AstExpression::Literal(AstLiteral::Int(0)))),
        ),
    ))))];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("return 0;"),
        "x * 0 should fold to 0, got:\n{}",
        printed
    );
}

#[test]
fn optimize_reassociation() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    // (x + 3) + 7 => x + 10
    let body = vec![wrap(AstStatement::Return(Some(wrap(
        AstExpression::BinaryOp(
            AstBinaryOperator::Add,
            Box::new(wrap(AstExpression::BinaryOp(
                AstBinaryOperator::Add,
                Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
                Box::new(wrap(AstExpression::Literal(AstLiteral::Int(3)))),
            ))),
            Box::new(wrap(AstExpression::Literal(AstLiteral::Int(7)))),
        ),
    ))))];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::none()
                .constant_folding(true)
                .pattern_matching_enabled(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("return x + 10;"),
        "(x + 3) + 7 should reassociate to x + 10, got:\n{}",
        printed
    );
}

// ============ Phase 2 Tests: Expression Inlining Improvements ============

#[test]
fn optimize_expression_inlining_wider_window() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["a", "b", "c"]);
    let (a, b, c) = (ids[0], ids[1], ids[2]);

    // a = 1; b = 2; c = a + b; return c;
    let body = vec![
        wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(vm.clone(), a)),
            wrap(AstExpression::Literal(AstLiteral::Int(1))),
        )),
        wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(vm.clone(), b)),
            wrap(AstExpression::Literal(AstLiteral::Int(2))),
        )),
        wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(vm.clone(), c)),
            wrap(AstExpression::BinaryOp(
                AstBinaryOperator::Add,
                Box::new(wrap(AstExpression::Variable(vm.clone(), a))),
                Box::new(wrap(AstExpression::Variable(vm.clone(), b))),
            )),
        )),
        wrap(AstStatement::Return(Some(wrap(AstExpression::Variable(
            vm.clone(),
            c,
        ))))),
    ];

    let ast = build_simple_test_ast(3, body, vm.clone());
    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::none()
                .expression_inlining(true)
                .constant_folding(true)
                .max_pass_iterations(2),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("return 3;"),
        "wider window inlining + folding should produce return 3, got:\n{}",
        printed
    );
}

#[test]
fn optimize_declaration_inlining() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x"]);
    let x = ids[0];

    // int x = 42; return x;
    let body = vec![
        wrap(AstStatement::Declaration(
            vm.read().unwrap().get(&x).unwrap().clone(),
            Some(wrap(AstExpression::Literal(AstLiteral::Int(42)))),
        )),
        wrap(AstStatement::Return(Some(wrap(AstExpression::Variable(
            vm.clone(),
            x,
        ))))),
    ];

    let ast = build_simple_test_ast(1, body, vm.clone());
    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::none().expression_inlining(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("return 42;"),
        "declaration should inline into return, got:\n{}",
        printed
    );
}

// ============ Phase 3 Tests: Ternary Recovery ============

#[test]
fn optimize_ternary_recovery_basic() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["cond", "result"]);
    let (cond, result) = (ids[0], ids[1]);

    // if (cond) { result = 1; } else { result = 2; }
    let body = vec![
        wrap(AstStatement::If(
            wrap(AstExpression::Variable(vm.clone(), cond)),
            vec![wrap(AstStatement::Assignment(
                wrap(AstExpression::Variable(vm.clone(), result)),
                wrap(AstExpression::Literal(AstLiteral::Int(1))),
            ))],
            Some(vec![wrap(AstStatement::Assignment(
                wrap(AstExpression::Variable(vm.clone(), result)),
                wrap(AstExpression::Literal(AstLiteral::Int(2))),
            ))]),
        )),
        wrap(AstStatement::Return(Some(wrap(AstExpression::Variable(
            vm.clone(),
            result,
        ))))),
    ];

    let ast = build_simple_test_ast(2, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().ternary_recovery(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("?") && printed.contains(":"),
        "should recover ternary operator, got:\n{}",
        printed
    );
}

#[test]
fn optimize_ternary_recovery_rejects_different_vars() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["cond", "a", "b"]);
    let (cond, a, b) = (ids[0], ids[1], ids[2]);

    // if (cond) { a = 1; } else { b = 2; } -- should NOT convert
    let body = vec![wrap(AstStatement::If(
        wrap(AstExpression::Variable(vm.clone(), cond)),
        vec![wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(vm.clone(), a)),
            wrap(AstExpression::Literal(AstLiteral::Int(1))),
        ))],
        Some(vec![wrap(AstStatement::Assignment(
            wrap(AstExpression::Variable(vm.clone(), b)),
            wrap(AstExpression::Literal(AstLiteral::Int(2))),
        ))]),
    ))];

    let ast = build_simple_test_ast(3, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().ternary_recovery(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("if"),
        "should NOT convert to ternary with different target vars, got:\n{}",
        printed
    );
}

#[test]
fn optimize_if_conversion_reversal_expands_nested_ternary_assignment() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["cond", "inner", "result"]);
    let (cond, inner, result) = (ids[0], ids[1], ids[2]);

    let body = vec![wrap(AstStatement::Assignment(
        wrap(AstExpression::Variable(vm.clone(), result)),
        wrap(AstExpression::Ternary(
            Box::new(wrap(AstExpression::Variable(vm.clone(), cond))),
            Box::new(wrap(AstExpression::Ternary(
                Box::new(wrap(AstExpression::Variable(vm.clone(), inner))),
                Box::new(wrap(AstExpression::Literal(AstLiteral::Int(1)))),
                Box::new(wrap(AstExpression::Literal(AstLiteral::Int(2)))),
            ))),
            Box::new(wrap(AstExpression::Literal(AstLiteral::Int(3)))),
        )),
    ))];

    let ast = build_simple_test_ast(3, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().constant_folding(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("if"),
        "nested ternary assignment should expand to if statements, got:\n{}",
        printed
    );
    // The outer ternary is expanded to if-else.
    // Inner simple ternaries (no further nesting) may remain as ternary
    // expressions — that is correct behavior. Only nested ternaries are expanded.
}

// ============ Phase 4 Tests: Boolean Recovery & Switch Reconstruction ============

#[test]
fn optimize_boolean_recovery_and_pattern() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["a", "b", "v"]);
    let (a, b, v) = (ids[0], ids[1], ids[2]);

    // if (a) { if (b) { v = true; } else { v = false; } } else { v = false; }
    let body = vec![
        wrap(AstStatement::If(
            wrap(AstExpression::Variable(vm.clone(), a)),
            vec![wrap(AstStatement::If(
                wrap(AstExpression::Variable(vm.clone(), b)),
                vec![wrap(AstStatement::Assignment(
                    wrap(AstExpression::Variable(vm.clone(), v)),
                    wrap(AstExpression::Literal(AstLiteral::Bool(true))),
                ))],
                Some(vec![wrap(AstStatement::Assignment(
                    wrap(AstExpression::Variable(vm.clone(), v)),
                    wrap(AstExpression::Literal(AstLiteral::Bool(false))),
                ))]),
            ))],
            Some(vec![wrap(AstStatement::Assignment(
                wrap(AstExpression::Variable(vm.clone(), v)),
                wrap(AstExpression::Literal(AstLiteral::Bool(false))),
            ))]),
        )),
        wrap(AstStatement::Return(Some(wrap(AstExpression::Variable(
            vm.clone(),
            v,
        ))))),
    ];

    let ast = build_simple_test_ast(3, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().boolean_recovery(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("&&"),
        "should recover && pattern, got:\n{}",
        printed
    );
}

#[test]
fn optimize_boolean_recovery_or_pattern() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["a", "b", "v"]);
    let (a, b, v) = (ids[0], ids[1], ids[2]);

    // if (a) { v = true; } else { if (b) { v = true; } else { v = false; } }
    let body = vec![
        wrap(AstStatement::If(
            wrap(AstExpression::Variable(vm.clone(), a)),
            vec![wrap(AstStatement::Assignment(
                wrap(AstExpression::Variable(vm.clone(), v)),
                wrap(AstExpression::Literal(AstLiteral::Bool(true))),
            ))],
            Some(vec![wrap(AstStatement::If(
                wrap(AstExpression::Variable(vm.clone(), b)),
                vec![wrap(AstStatement::Assignment(
                    wrap(AstExpression::Variable(vm.clone(), v)),
                    wrap(AstExpression::Literal(AstLiteral::Bool(true))),
                ))],
                Some(vec![wrap(AstStatement::Assignment(
                    wrap(AstExpression::Variable(vm.clone(), v)),
                    wrap(AstExpression::Literal(AstLiteral::Bool(false))),
                ))]),
            ))]),
        )),
        wrap(AstStatement::Return(Some(wrap(AstExpression::Variable(
            vm.clone(),
            v,
        ))))),
    ];

    let ast = build_simple_test_ast(3, body, vm.clone());
    let optimized = ast
        .optimize(Some(AstOptimizationConfig::none().boolean_recovery(true)))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("||"),
        "should recover || pattern, got:\n{}",
        printed
    );
}

#[test]
fn optimize_switch_reconstruction_3_cases() {
    let fid = AstFunctionId { address: 0x9000 };
    let (ids, vm) = make_var_map(fid, &["x", "r"]);
    let (x, r) = (ids[0], ids[1]);

    // if (x == 1) { r = 10; } else if (x == 2) { r = 20; } else if (x == 3) { r = 30; } else { r = 0; }
    let body = vec![
        wrap(AstStatement::If(
            wrap(AstExpression::BinaryOp(
                AstBinaryOperator::Equal,
                Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
                Box::new(wrap(AstExpression::Literal(AstLiteral::Int(1)))),
            )),
            vec![wrap(AstStatement::Assignment(
                wrap(AstExpression::Variable(vm.clone(), r)),
                wrap(AstExpression::Literal(AstLiteral::Int(10))),
            ))],
            Some(vec![wrap(AstStatement::If(
                wrap(AstExpression::BinaryOp(
                    AstBinaryOperator::Equal,
                    Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
                    Box::new(wrap(AstExpression::Literal(AstLiteral::Int(2)))),
                )),
                vec![wrap(AstStatement::Assignment(
                    wrap(AstExpression::Variable(vm.clone(), r)),
                    wrap(AstExpression::Literal(AstLiteral::Int(20))),
                ))],
                Some(vec![wrap(AstStatement::If(
                    wrap(AstExpression::BinaryOp(
                        AstBinaryOperator::Equal,
                        Box::new(wrap(AstExpression::Variable(vm.clone(), x))),
                        Box::new(wrap(AstExpression::Literal(AstLiteral::Int(3)))),
                    )),
                    vec![wrap(AstStatement::Assignment(
                        wrap(AstExpression::Variable(vm.clone(), r)),
                        wrap(AstExpression::Literal(AstLiteral::Int(30))),
                    ))],
                    Some(vec![wrap(AstStatement::Assignment(
                        wrap(AstExpression::Variable(vm.clone(), r)),
                        wrap(AstExpression::Literal(AstLiteral::Int(0))),
                    ))]),
                ))]),
            ))]),
        )),
        wrap(AstStatement::Return(Some(wrap(AstExpression::Variable(
            vm.clone(),
            r,
        ))))),
    ];

    let ast = build_simple_test_ast(2, body, vm.clone());
    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::none().switch_reconstruction(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    assert!(
        printed.contains("switch"),
        "should reconstruct switch statement, got:\n{}",
        printed
    );
    assert!(
        printed.contains("case 1:") && printed.contains("case 2:") && printed.contains("case 3:"),
        "should have all 3 cases, got:\n{}",
        printed
    );
    assert!(
        printed.contains("default:"),
        "should have default clause, got:\n{}",
        printed
    );
}

use crate::{
    abstract_syntax_tree::{
        Ast, AstCall, AstDescriptor, AstExpression, AstFunction, AstFunctionId, AstFunctionVersion,
        AstJumpTarget, AstLiteral, AstOptimizationConfig, AstPrintConfig, AstStatement,
        AstStatementOrigin, AstValue, AstValueOrigin, AstValueType, AstVariable, AstVariableId,
        Wrapped, WrappedAstStatement,
    },
    core::{Instruction, Sections},
    ir::{Ir, analyze::IrFunction, statements::IrStatement, utils::IrStatementDescriptor},
    utils::version_map::VersionMap,
};
use hashbrown::HashMap;
use num_bigint::BigInt;
use std::sync::{Arc, RwLock};

fn wrap_expression(item: AstExpression) -> Wrapped<AstExpression> {
    Wrapped {
        item,
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

fn wrap_statement(statement: AstStatement) -> WrappedAstStatement {
    WrappedAstStatement {
        statement,
        origin: AstStatementOrigin::Unknown,
        comment: None,
    }
}

fn build_test_function(
    function_id: AstFunctionId,
    function_name: &str,
    body: Vec<WrappedAstStatement>,
    variables: Arc<RwLock<HashMap<AstVariableId, AstVariable>>>,
) -> AstFunction {
    let instructions: Arc<[Instruction]> = Vec::<Instruction>::new().into();
    let ir = Arc::new(IrFunction::new(instructions, Vec::new(), Vec::new()));
    AstFunction {
        name: Some(function_name.to_string()),
        id: function_id,
        ir,
        return_type: AstValueType::Int,
        parameters: Vec::new(),
        variables,
        body,
        processed_optimizations: Vec::new(),
    }
}

fn build_ir_origin_descriptor(address: u64, byte: u8) -> AstDescriptor {
    let sections = Sections::new();
    let instruction = Instruction {
        address,
        inner: iceball::Instruction {
            statement: Err(iceball::DisassembleError::Unknown),
            arguments: Vec::new().into_boxed_slice(),
            bytes: Some(vec![byte].into_boxed_slice()),
        },
    };
    let ir_statements: &'static [IrStatement] =
        Box::leak(vec![IrStatement::Undefined].into_boxed_slice());
    let ir = Ir {
        address: crate::core::Address::from_virtual_address(&sections, address),
        statements: Some(ir_statements),
    };
    let ir_fn = Arc::new(IrFunction::new(
        vec![instruction].into(),
        vec![ir],
        Vec::new(),
    ));
    AstDescriptor::new(ir_fn, IrStatementDescriptor::new(0, Some(0)))
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
        wrap_statement(AstStatement::Declaration(
            variable_map.read().unwrap().get(&var_a).unwrap().clone(),
            Some(wrap_expression(AstExpression::Literal(AstLiteral::Int(2)))),
        )),
        wrap_statement(AstStatement::Declaration(
            variable_map.read().unwrap().get(&var_b).unwrap().clone(),
            Some(wrap_expression(AstExpression::BinaryOp(
                crate::abstract_syntax_tree::AstBinaryOperator::Add,
                Box::new(wrap_expression(AstExpression::Variable(
                    variable_map.clone(),
                    var_a,
                ))),
                Box::new(wrap_expression(AstExpression::Literal(AstLiteral::Int(3)))),
            ))),
        )),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(variable_map.clone(), var_a)),
            wrap_expression(AstExpression::BinaryOp(
                crate::abstract_syntax_tree::AstBinaryOperator::Mul,
                Box::new(wrap_expression(AstExpression::Variable(
                    variable_map.clone(),
                    var_b,
                ))),
                Box::new(wrap_expression(AstExpression::Literal(AstLiteral::Int(1)))),
            )),
        )),
        wrap_statement(AstStatement::Return(Some(wrap_expression(
            AstExpression::BinaryOp(
                crate::abstract_syntax_tree::AstBinaryOperator::Add,
                Box::new(wrap_expression(AstExpression::Variable(
                    variable_map.clone(),
                    var_a,
                ))),
                Box::new(wrap_expression(AstExpression::Literal(AstLiteral::Int(0)))),
            ),
        )))),
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
            AstOptimizationConfig::NONE
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
        wrap_statement(AstStatement::Return(None)),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(variable_map.clone(), var_a)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(99))),
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
        .optimize(Some(AstOptimizationConfig::NONE.control_flow_cleanup(true)))
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
        wrap_statement(AstStatement::Return(None)),
        wrap_statement(AstStatement::Label("L1".to_string())),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(variable_map.clone(), var_a)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(7))),
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
        .optimize(Some(AstOptimizationConfig::NONE.control_flow_cleanup(true)))
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
        wrap_statement(AstStatement::Call(AstCall::Function {
            target: helper_id,
            args: Vec::new(),
        })),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(caller_variable_map.clone(), var_a)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(77))),
        )),
    ];
    let helper_body = vec![wrap_statement(AstStatement::Call(AstCall::Unknown(
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
            Some(AstOptimizationConfig::NONE.control_flow_cleanup(true)),
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
fn optimize_call_argument_splits_branch_goto_and_merges_single_call_callees() {
    let caller_id = AstFunctionId { address: 0x1000 };
    let callee_true_id = AstFunctionId { address: 0x2000 };
    let callee_false_id = AstFunctionId { address: 0x3000 };
    let version = AstFunctionVersion(1);
    let empty_vars = Arc::new(RwLock::new(HashMap::new()));

    let caller_body = vec![wrap_statement(AstStatement::If(
        wrap_expression(AstExpression::Literal(AstLiteral::Bool(true))),
        vec![wrap_statement(AstStatement::Goto(AstJumpTarget::Unknown(
            "0x2000".to_string(),
        )))],
        Some(vec![wrap_statement(AstStatement::Goto(
            AstJumpTarget::Unknown("0x3000".to_string()),
        ))]),
    ))];
    let callee_true_body = vec![
        wrap_statement(AstStatement::Comment("callee_true".to_string())),
        wrap_statement(AstStatement::Return(None)),
    ];
    let callee_false_body = vec![
        wrap_statement(AstStatement::Comment("callee_false".to_string())),
        wrap_statement(AstStatement::Return(None)),
    ];

    let caller = build_test_function(caller_id, "caller", caller_body, empty_vars.clone());
    let callee_true = build_test_function(
        callee_true_id,
        &callee_true_id.get_default_name(),
        callee_true_body,
        empty_vars.clone(),
    );
    let callee_false = build_test_function(
        callee_false_id,
        &callee_false_id.get_default_name(),
        callee_false_body,
        empty_vars.clone(),
    );

    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, caller));
    functions.insert(callee_true_id, VersionMap::new(version, callee_true));
    functions.insert(callee_false_id, VersionMap::new(version, callee_false));
    let ast = Ast {
        function_versions: HashMap::from([
            (caller_id, version),
            (callee_true_id, version),
            (callee_false_id, version),
        ]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::NONE.call_argument_analyzation(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));

    assert!(
        printed.contains("/* callee_true */"),
        "true branch callee body should be merged into caller, got:\n{}",
        printed
    );
    assert!(
        printed.contains("/* callee_false */"),
        "false branch callee body should be merged into caller, got:\n{}",
        printed
    );
    assert!(
        !printed.contains("f2000();") && !printed.contains("f3000();"),
        "single-call callees should be inlined, got:\n{}",
        printed
    );
    assert!(
        !printed.contains("int f2000(") && !printed.contains("int f3000("),
        "single-call callee functions should be removed after merge, got:\n{}",
        printed
    );
}

#[test]
fn optimize_call_argument_keeps_multi_call_callee_split() {
    let caller_id = AstFunctionId { address: 0x1000 };
    let callee_id = AstFunctionId { address: 0x2000 };
    let version = AstFunctionVersion(1);
    let empty_vars = Arc::new(RwLock::new(HashMap::new()));

    let caller_body = vec![wrap_statement(AstStatement::If(
        wrap_expression(AstExpression::Literal(AstLiteral::Bool(true))),
        vec![wrap_statement(AstStatement::Goto(AstJumpTarget::Unknown(
            "0x2000".to_string(),
        )))],
        Some(vec![wrap_statement(AstStatement::Goto(
            AstJumpTarget::Unknown("0x2000".to_string()),
        ))]),
    ))];
    let callee_body = vec![
        wrap_statement(AstStatement::Comment("shared_callee".to_string())),
        wrap_statement(AstStatement::Return(None)),
    ];

    let caller = build_test_function(caller_id, "caller", caller_body, empty_vars.clone());
    let callee = build_test_function(
        callee_id,
        &callee_id.get_default_name(),
        callee_body,
        empty_vars.clone(),
    );

    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, caller));
    functions.insert(callee_id, VersionMap::new(version, callee));
    let ast = Ast {
        function_versions: HashMap::from([(caller_id, version), (callee_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::NONE.call_argument_analyzation(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    let call_count = printed.matches("f2000();").count();

    assert!(
        call_count == 2,
        "multi-call target should stay as calls in each branch, got:\n{}",
        printed
    );
    assert!(
        printed.contains("int f2000("),
        "multi-call callee should remain as split function, got:\n{}",
        printed
    );
}

#[test]
fn optimize_call_argument_keeps_recursive_callee_split() {
    let caller_id = AstFunctionId { address: 0x1000 };
    let recursive_id = AstFunctionId { address: 0x2000 };
    let version = AstFunctionVersion(1);
    let empty_vars = Arc::new(RwLock::new(HashMap::new()));

    let caller_body = vec![wrap_statement(AstStatement::Goto(AstJumpTarget::Unknown(
        "0x2000".to_string(),
    )))];
    let recursive_body = vec![
        wrap_statement(AstStatement::Comment("recursive_callee".to_string())),
        wrap_statement(AstStatement::Call(AstCall::Function {
            target: recursive_id,
            args: Vec::new(),
        })),
        wrap_statement(AstStatement::Return(None)),
    ];

    let caller = build_test_function(caller_id, "caller", caller_body, empty_vars.clone());
    let recursive_callee = build_test_function(
        recursive_id,
        &recursive_id.get_default_name(),
        recursive_body,
        empty_vars.clone(),
    );

    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, caller));
    functions.insert(recursive_id, VersionMap::new(version, recursive_callee));
    let ast = Ast {
        function_versions: HashMap::from([(caller_id, version), (recursive_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::NONE.call_argument_analyzation(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));
    let caller_start = printed
        .find("int caller(")
        .expect("caller function should be printed");
    let caller_end = printed[caller_start + 1..]
        .find("\nint ")
        .map(|idx| caller_start + 1 + idx)
        .unwrap_or(printed.len());
    let caller_section = &printed[caller_start..caller_end];

    assert!(
        caller_section.contains("f2000();"),
        "caller goto should be converted to call, got:\n{}",
        printed
    );
    assert!(
        !caller_section.contains("goto"),
        "caller should not keep goto after conversion, got:\n{}",
        printed
    );
    assert!(
        printed.contains("int f2000("),
        "recursive callee should remain split, got:\n{}",
        printed
    );
}

#[test]
fn optimize_call_argument_converts_branch_goto_targets_to_calls_without_split_function_ids() {
    let caller_id = AstFunctionId { address: 0x1000 };
    let version = AstFunctionVersion(1);
    let fallthrough_id = AstVariableId {
        index: 1,
        parent: Some(caller_id),
    };
    let variable_map = Arc::new(RwLock::new(HashMap::from([(
        fallthrough_id,
        AstVariable {
            name: Some("fallthrough".to_string()),
            id: fallthrough_id,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));

    let caller_body = vec![wrap_statement(AstStatement::If(
        wrap_expression(AstExpression::Literal(AstLiteral::Bool(true))),
        vec![wrap_statement(AstStatement::Goto(AstJumpTarget::Unknown(
            "0x2001".to_string(),
        )))],
        Some(vec![wrap_statement(AstStatement::Goto(
            AstJumpTarget::Variable {
                scope: caller_id,
                var_map: variable_map.clone(),
                var_id: fallthrough_id,
            },
        ))]),
    ))];

    let caller = build_test_function(caller_id, "caller", caller_body, variable_map);

    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, caller));
    let ast = Ast {
        function_versions: HashMap::from([(caller_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::NONE.call_argument_analyzation(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));

    assert!(
        printed.contains("f2001();"),
        "branch target address should become a call even without split function id, got:\n{}",
        printed
    );
    assert!(
        printed.contains("fallthrough();"),
        "branch variable target should become a variable call, got:\n{}",
        printed
    );
    assert!(
        !printed.contains("goto 0x2001"),
        "address branch should not remain goto after conversion, got:\n{}",
        printed
    );
}

#[test]
fn optimize_call_argument_renames_merged_callee_variable_name_conflicts() {
    let caller_id = AstFunctionId { address: 0x1000 };
    let callee_id = AstFunctionId { address: 0x2000 };
    let version = AstFunctionVersion(1);

    let caller_var_id = AstVariableId {
        index: 1,
        parent: Some(caller_id),
    };
    let callee_var_id = AstVariableId {
        index: 1,
        parent: Some(callee_id),
    };

    let caller_vars = Arc::new(RwLock::new(HashMap::from([(
        caller_var_id,
        AstVariable {
            name: None,
            id: caller_var_id,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));
    let callee_vars = Arc::new(RwLock::new(HashMap::from([(
        callee_var_id,
        AstVariable {
            name: None,
            id: callee_var_id,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));

    let caller_body = vec![
        wrap_statement(AstStatement::Declaration(
            caller_vars
                .read()
                .unwrap()
                .get(&caller_var_id)
                .unwrap()
                .clone(),
            None,
        )),
        wrap_statement(AstStatement::Call(AstCall::Function {
            target: callee_id,
            args: Vec::new(),
        })),
    ];
    let callee_body = vec![
        wrap_statement(AstStatement::Declaration(
            callee_vars
                .read()
                .unwrap()
                .get(&callee_var_id)
                .unwrap()
                .clone(),
            None,
        )),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(callee_vars.clone(), callee_var_id)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(2))),
        )),
        wrap_statement(AstStatement::Return(None)),
    ];

    let caller = build_test_function(caller_id, "caller", caller_body, caller_vars);
    let callee = build_test_function(
        callee_id,
        &callee_id.get_default_name(),
        callee_body,
        callee_vars,
    );

    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, caller));
    functions.insert(callee_id, VersionMap::new(version, callee));
    let ast = Ast {
        function_versions: HashMap::from([(caller_id, version), (callee_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::NONE.call_argument_analyzation(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));

    assert!(
        printed.contains("int v1;"),
        "caller variable should keep base name, got:\n{}",
        printed
    );
    assert!(
        printed.contains("int v1_1;"),
        "merged callee variable should use callee scope suffix, got:\n{}",
        printed
    );
    assert_eq!(
        printed.matches("    int v1;").count(),
        1,
        "function body should keep only caller declaration name for v1, got:\n{}",
        printed
    );
    assert!(
        printed.contains("    int v1_1;"),
        "function body should use renamed callee variable declaration, got:\n{}",
        printed
    );
    assert!(
        printed.contains("    v1_1 = 2;"),
        "function body should use renamed callee variable in expressions, got:\n{}",
        printed
    );
}

#[test]
fn optimize_call_argument_renames_callee_default_variable_without_caller_conflict() {
    let caller_id = AstFunctionId { address: 0x1100 };
    let callee_id = AstFunctionId { address: 0x2200 };
    let version = AstFunctionVersion(1);

    let caller_var_id = AstVariableId {
        index: 17,
        parent: Some(caller_id),
    };
    let callee_var_id = AstVariableId {
        index: 18,
        parent: Some(callee_id),
    };

    let caller_vars = Arc::new(RwLock::new(HashMap::from([(
        caller_var_id,
        AstVariable {
            name: None,
            id: caller_var_id,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));
    let callee_vars = Arc::new(RwLock::new(HashMap::from([(
        callee_var_id,
        AstVariable {
            name: None,
            id: callee_var_id,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));

    let caller_body = vec![
        wrap_statement(AstStatement::Declaration(
            caller_vars
                .read()
                .unwrap()
                .get(&caller_var_id)
                .unwrap()
                .clone(),
            None,
        )),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(caller_vars.clone(), caller_var_id)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
        )),
        wrap_statement(AstStatement::Call(AstCall::Function {
            target: callee_id,
            args: Vec::new(),
        })),
    ];
    let callee_body = vec![
        wrap_statement(AstStatement::Declaration(
            callee_vars
                .read()
                .unwrap()
                .get(&callee_var_id)
                .unwrap()
                .clone(),
            None,
        )),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(callee_vars.clone(), callee_var_id)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(2))),
        )),
        wrap_statement(AstStatement::Return(None)),
    ];

    let caller = build_test_function(caller_id, "caller", caller_body, caller_vars);
    let callee = build_test_function(
        callee_id,
        &callee_id.get_default_name(),
        callee_body,
        callee_vars,
    );

    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, caller));
    functions.insert(callee_id, VersionMap::new(version, callee));
    let ast = Ast {
        function_versions: HashMap::from([(caller_id, version), (callee_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::NONE.call_argument_analyzation(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));

    assert!(
        printed.contains("int v18_1;"),
        "callee default variable should use first callee suffix, got:\n{}",
        printed
    );
    assert!(
        printed.contains("    v18_1 = 2;"),
        "callee expression should use renamed variable, got:\n{}",
        printed
    );
    assert!(
        printed.contains("    v17 = 1;"),
        "caller variable should keep its original name, got:\n{}",
        printed
    );
}

#[test]
fn optimize_call_argument_renames_callee_default_variable_with_scope_suffix_when_v10_2_exists() {
    let caller_id = AstFunctionId { address: 0x1300 };
    let callee_id = AstFunctionId { address: 0x2300 };
    let version = AstFunctionVersion(1);

    let caller_var_id = AstVariableId {
        index: 1,
        parent: Some(caller_id),
    };
    let callee_var_id = AstVariableId {
        index: 10,
        parent: Some(callee_id),
    };

    let caller_vars = Arc::new(RwLock::new(HashMap::from([(
        caller_var_id,
        AstVariable {
            name: Some("v10_2".to_string()),
            id: caller_var_id,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));
    let callee_vars = Arc::new(RwLock::new(HashMap::from([(
        callee_var_id,
        AstVariable {
            name: None,
            id: callee_var_id,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));

    let caller_body = vec![
        wrap_statement(AstStatement::Declaration(
            caller_vars
                .read()
                .unwrap()
                .get(&caller_var_id)
                .unwrap()
                .clone(),
            None,
        )),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(caller_vars.clone(), caller_var_id)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
        )),
        wrap_statement(AstStatement::Call(AstCall::Function {
            target: callee_id,
            args: Vec::new(),
        })),
    ];
    let callee_body = vec![
        wrap_statement(AstStatement::Declaration(
            callee_vars
                .read()
                .unwrap()
                .get(&callee_var_id)
                .unwrap()
                .clone(),
            None,
        )),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(callee_vars.clone(), callee_var_id)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(2))),
        )),
        wrap_statement(AstStatement::Return(None)),
    ];

    let caller = build_test_function(caller_id, "caller", caller_body, caller_vars);
    let callee = build_test_function(
        callee_id,
        &callee_id.get_default_name(),
        callee_body,
        callee_vars,
    );

    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, caller));
    functions.insert(callee_id, VersionMap::new(version, callee));
    let ast = Ast {
        function_versions: HashMap::from([(caller_id, version), (callee_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::NONE.call_argument_analyzation(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));

    assert!(
        printed.contains("int v10_2;"),
        "existing caller v10_2 should stay unchanged, got:\n{}",
        printed
    );
    assert!(
        printed.contains("int v10_1;"),
        "merged callee v10 should use first callee suffix, got:\n{}",
        printed
    );
    assert!(
        printed.contains("    v10_1 = 2;"),
        "callee expression should use v10_1, got:\n{}",
        printed
    );
}

#[test]
fn optimize_call_argument_applies_incremental_scope_suffixes_for_multiple_callees() {
    let caller_id = AstFunctionId { address: 0x1400 };
    let callee1_id = AstFunctionId { address: 0x2400 };
    let callee2_id = AstFunctionId { address: 0x2500 };
    let version = AstFunctionVersion(1);

    let caller_var_ids: Vec<_> = (1..=4)
        .map(|idx| AstVariableId {
            index: idx,
            parent: Some(caller_id),
        })
        .collect();
    let caller_vars = Arc::new(RwLock::new(HashMap::from_iter(caller_var_ids.iter().map(
        |id| {
            (
                *id,
                AstVariable {
                    name: None,
                    id: *id,
                    var_type: AstValueType::Int,
                    const_value: None,
                    data_access_ir: None,
                },
            )
        },
    ))));

    let callee1_var_id = AstVariableId {
        index: 10,
        parent: Some(callee1_id),
    };
    let callee1_vars = Arc::new(RwLock::new(HashMap::from([(
        callee1_var_id,
        AstVariable {
            name: None,
            id: callee1_var_id,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));

    let callee2_var_id = AstVariableId {
        index: 20,
        parent: Some(callee2_id),
    };
    let callee2_vars = Arc::new(RwLock::new(HashMap::from([(
        callee2_var_id,
        AstVariable {
            name: None,
            id: callee2_var_id,
            var_type: AstValueType::Int,
            const_value: None,
            data_access_ir: None,
        },
    )])));

    let caller_body = vec![
        wrap_statement(AstStatement::Call(AstCall::Function {
            target: callee1_id,
            args: Vec::new(),
        })),
        wrap_statement(AstStatement::Call(AstCall::Function {
            target: callee2_id,
            args: Vec::new(),
        })),
    ];
    let callee1_body = vec![
        wrap_statement(AstStatement::Declaration(
            callee1_vars
                .read()
                .unwrap()
                .get(&callee1_var_id)
                .unwrap()
                .clone(),
            None,
        )),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(
                callee1_vars.clone(),
                callee1_var_id,
            )),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(10))),
        )),
        wrap_statement(AstStatement::Return(None)),
    ];
    let callee2_body = vec![
        wrap_statement(AstStatement::Declaration(
            callee2_vars
                .read()
                .unwrap()
                .get(&callee2_var_id)
                .unwrap()
                .clone(),
            None,
        )),
        wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(
                callee2_vars.clone(),
                callee2_var_id,
            )),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(20))),
        )),
        wrap_statement(AstStatement::Return(None)),
    ];

    let caller = build_test_function(caller_id, "caller", caller_body, caller_vars);
    let callee1 = build_test_function(
        callee1_id,
        &callee1_id.get_default_name(),
        callee1_body,
        callee1_vars,
    );
    let callee2 = build_test_function(
        callee2_id,
        &callee2_id.get_default_name(),
        callee2_body,
        callee2_vars,
    );

    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, caller));
    functions.insert(callee1_id, VersionMap::new(version, callee1));
    functions.insert(callee2_id, VersionMap::new(version, callee2));
    let ast = Ast {
        function_versions: HashMap::from([
            (caller_id, version),
            (callee1_id, version),
            (callee2_id, version),
        ]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::NONE.call_argument_analyzation(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));

    assert!(
        printed.contains("int v4;"),
        "caller variables should keep unsuffixed names up to v4, got:\n{}",
        printed
    );
    assert!(
        printed.contains("int v10_1;"),
        "first merged callee variables should use _1 suffix, got:\n{}",
        printed
    );
    assert!(
        printed.contains("int v20_2;"),
        "second merged callee variables should use _2 suffix, got:\n{}",
        printed
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
                    origin: AstValueOrigin::Unknown,
                    comment: None,
                }),
                data_access_ir: None,
            },
        ),
    ])));
    let body = vec![wrap_statement(AstStatement::Return(None))];
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

    let body = vec![wrap_statement(AstStatement::If(
        wrap_expression(AstExpression::Literal(AstLiteral::Bool(true))),
        vec![
            wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(variable_map.clone(), var_id)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            )),
            wrap_statement(AstStatement::Assignment(
                wrap_expression(AstExpression::Variable(variable_map.clone(), var_id)),
                wrap_expression(AstExpression::Literal(AstLiteral::Int(2))),
            )),
        ],
        Some(vec![wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(variable_map.clone(), var_id)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(3))),
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
        printed.contains("if (true) {\n        x = 1;\n        x = 2;\n    } else { x = 3; }"),
        "if true-branch with multiple statements should be printed as multiline block, got:\n{}",
        printed
    );
}

#[test]
fn optimize_call_argument_preserves_comments_when_inlining_blocks() {
    let caller_id = AstFunctionId { address: 0x4000 };
    let callee_id = AstFunctionId { address: 0x5000 };
    let version = AstFunctionVersion(1);
    let empty_vars = Arc::new(RwLock::new(HashMap::new()));

    let caller_body = vec![WrappedAstStatement {
        statement: AstStatement::Call(AstCall::Function {
            target: callee_id,
            args: Vec::new(),
        }),
        origin: AstStatementOrigin::Unknown,
        comment: Some("callsite comment".to_string()),
    }];
    let callee_body = vec![
        wrap_statement(AstStatement::Comment("callee block comment".to_string())),
        WrappedAstStatement {
            statement: AstStatement::Return(None),
            origin: AstStatementOrigin::Unknown,
            comment: Some("tail return comment".to_string()),
        },
    ];

    let caller = build_test_function(caller_id, "caller", caller_body, empty_vars.clone());
    let callee = build_test_function(
        callee_id,
        &callee_id.get_default_name(),
        callee_body,
        empty_vars,
    );

    let mut functions = HashMap::new();
    functions.insert(caller_id, VersionMap::new(version, caller));
    functions.insert(callee_id, VersionMap::new(version, callee));
    let ast = Ast {
        function_versions: HashMap::from([(caller_id, version), (callee_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize(Some(
            AstOptimizationConfig::NONE.call_argument_analyzation(true),
        ))
        .unwrap();
    let printed = optimized.print(Some(AstPrintConfig::NONE));

    assert!(
        printed.contains("callsite comment"),
        "callsite comment should be preserved after inlining, got:\n{}",
        printed
    );
    assert!(
        printed.contains("callee block comment"),
        "callee block comment should remain after inlining, got:\n{}",
        printed
    );
    assert!(
        printed.contains("tail return comment"),
        "trailing return comment should be preserved after stripping return, got:\n{}",
        printed
    );
}

#[test]
fn print_keeps_origin_comments_for_different_ir_sources_with_same_descriptor_index() {
    let function_id = AstFunctionId { address: 0x6100 };
    let version = AstFunctionVersion(1);
    let empty_vars = Arc::new(RwLock::new(HashMap::new()));
    let body = vec![
        WrappedAstStatement {
            statement: AstStatement::Comment("from first source".to_string()),
            origin: AstStatementOrigin::Ir(build_ir_origin_descriptor(0x401000, 0x90)),
            comment: None,
        },
        WrappedAstStatement {
            statement: AstStatement::Comment("from second source".to_string()),
            origin: AstStatementOrigin::Ir(build_ir_origin_descriptor(0x402000, 0xCC)),
            comment: None,
        },
    ];

    let function = build_test_function(function_id, "origin_print_test", body, empty_vars);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let printed = ast.print(Some(
        AstPrintConfig::NONE.print_instruction(true).print_ir(true),
    ));
    let instruction_comment_count = printed
        .lines()
        .filter(|line| line.trim_start().starts_with("// 0x"))
        .count();
    let ir_comment_count = printed.matches("/* undefined */").count();

    assert_eq!(
        instruction_comment_count, 2,
        "expected two distinct instruction comments, got:\n{}",
        printed
    );
    assert_eq!(
        ir_comment_count, 2,
        "expected two distinct IR comments, got:\n{}",
        printed
    );
    assert!(
        printed.contains("// 0x401000 90"),
        "missing first source instruction origin comment, got:\n{}",
        printed
    );
    assert!(
        printed.contains("// 0x402000 CC"),
        "missing second source instruction origin comment, got:\n{}",
        printed
    );
}

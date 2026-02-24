use crate::{
    abstract_syntax_tree::{
        Ast, AstCall, AstExpression, AstFunction, AstFunctionId, AstFunctionVersion, AstLiteral,
        AstOptimizationConfig, AstPrintConfig, AstStatement, AstStatementOrigin, AstValueOrigin,
        AstValueType, AstVariable, AstVariableId, Wrapped, WrappedAstStatement,
    },
    core::Instruction,
    ir::analyze::IrFunction,
    utils::version_map::VersionMap,
};
use hashbrown::HashMap;
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

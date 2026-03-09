use crate::{
    abstract_syntax_tree::{
        ArcAstVariableMap, Ast, AstExpression, AstFunction, AstFunctionId, AstFunctionVersion,
        AstLiteral, AstOptimizationConfig, AstStatement, AstStatementOrigin, AstValueOrigin,
        AstValueType, AstVariable, AstVariableId, Wrapped, WrappedAstStatement,
        pattern_matching::{AstPattern, AstPatternInputType},
    },
    core::Instruction,
    ir::{
        analyze::IrFunction,
        data::{IrData, IrIntrinsic},
        statements::IrStatement,
    },
    utils::version_map::VersionMap,
};
use hashbrown::HashMap;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

fn wrap_statement(statement: AstStatement) -> WrappedAstStatement {
    WrappedAstStatement {
        statement,
        origin: AstStatementOrigin::Unknown,
        comment: None,
    }
}

fn wrap_expression(item: AstExpression) -> Wrapped<AstExpression> {
    Wrapped {
        item,
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

fn parse_test_argument(op: &str) -> Option<iceball::Argument> {
    if let Ok(value) = op.parse::<u64>() {
        return Some(iceball::Argument::Constant(value));
    }

    fn try_parse(op: &str) -> Option<iceball::Argument> {
        iceball::parse_argument(iceball::Architecture::X64, op).ok()
    }
    try_parse(op)
        .or_else(|| try_parse(&op.to_ascii_uppercase()))
        .or_else(|| try_parse(&op.to_ascii_lowercase()))
}

fn parse_asm_ir_statement(asm: &str) -> IrStatement {
    let text = asm.trim();
    let mut parts = text.splitn(2, |ch: char| ch.is_whitespace());
    let mnemonic = parts
        .next()
        .expect("asm text must include a mnemonic")
        .trim();
    let operands = parts.next().unwrap_or_default().trim();
    let statement = iceball::parse_statement(iceball::Architecture::X64, mnemonic)
        .expect("mnemonic must parse in tests");
    let arguments = if operands.is_empty() {
        Vec::new()
    } else {
        operands
            .split(',')
            .map(str::trim)
            .filter(|op| !op.is_empty())
            .map(|op| parse_test_argument(op).expect("operand must parse in tests"))
            .collect::<Vec<_>>()
    };
    let instruction = Instruction {
        address: 0,
        inner: iceball::Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: None,
        },
    };
    crate::arch::x86_64::instruction_analyze::create_ir_statement(&instruction)
        .and_then(|stmts| stmts.first().cloned())
        .unwrap_or(IrStatement::Undefined)
}

fn wrap_asm_ir(asm: &str) -> WrappedAstStatement {
    wrap_statement(AstStatement::Ir(Box::new(parse_asm_ir_statement(asm))))
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

fn build_pattern_test_ast() -> (Ast, AstFunctionId) {
    let function_id = AstFunctionId { address: 0x4400 };
    let version = AstFunctionVersion(1);
    let variable_map = Arc::new(RwLock::new(HashMap::new()));

    let body = vec![
        wrap_asm_ir("mov rsp, 8"),
        wrap_asm_ir("push rbp"),
        wrap_statement(AstStatement::Ir(Box::new(IrStatement::Undefined))),
        wrap_statement(AstStatement::Comment("seed-comment".to_string())),
        wrap_statement(AstStatement::Return(None)),
    ];

    let function = build_test_function(function_id, "pattern_examples_target", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));

    (
        Ast {
            function_versions: HashMap::from([(function_id, version)]),
            functions: Arc::new(RwLock::new(functions)),
            last_variable_id: HashMap::new(),
            pre_defined_symbols: HashMap::new(),
        },
        function_id,
    )
}

fn build_asm_contains_pattern_test_ast() -> (Ast, AstFunctionId) {
    let function_id = AstFunctionId { address: 0x4410 };
    let version = AstFunctionVersion(1);
    let variable_map = Arc::new(RwLock::new(HashMap::new()));

    let body = vec![
        wrap_statement(AstStatement::Assembly("call __stack_chk_fail".to_string())),
        wrap_statement(AstStatement::Comment("replace-slot".to_string())),
        wrap_statement(AstStatement::Return(None)),
    ];

    let function = build_test_function(
        function_id,
        "pattern_asm_contains_target",
        body,
        variable_map,
    );
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));

    (
        Ast {
            function_versions: HashMap::from([(function_id, version)]),
            functions: Arc::new(RwLock::new(functions)),
            last_variable_id: HashMap::new(),
            pre_defined_symbols: HashMap::new(),
        },
        function_id,
    )
}

fn build_ast_if_pattern_test_ast() -> (Ast, AstFunctionId) {
    let function_id = AstFunctionId { address: 0x4500 };
    let version = AstFunctionVersion(1);
    let variable_map = Arc::new(RwLock::new(HashMap::new()));

    let body = vec![
        wrap_statement(AstStatement::Comment("replace-slot".to_string())),
        wrap_statement(AstStatement::If(
            wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            vec![wrap_statement(AstStatement::Comment(
                "true-branch".to_string(),
            ))],
            Some(vec![wrap_statement(AstStatement::Comment(
                "false-branch".to_string(),
            ))]),
        )),
        wrap_statement(AstStatement::Return(None)),
    ];

    let function = build_test_function(function_id, "pattern_ast_if_target", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));

    (
        Ast {
            function_versions: HashMap::from([(function_id, version)]),
            functions: Arc::new(RwLock::new(functions)),
            last_variable_id: HashMap::new(),
            pre_defined_symbols: HashMap::new(),
        },
        function_id,
    )
}

fn build_ir_if_pattern_test_ast() -> (Ast, AstFunctionId) {
    let function_id = AstFunctionId { address: 0x4600 };
    let version = AstFunctionVersion(1);
    let variable_map = Arc::new(RwLock::new(HashMap::new()));

    let body = vec![
        wrap_statement(AstStatement::Comment("replace-slot".to_string())),
        wrap_statement(AstStatement::Ir(Box::new(IrStatement::Condition {
            condition: IrData::Intrinsic(IrIntrinsic::Unknown).into(),
            true_branch: vec![IrStatement::Halt].into_boxed_slice(),
            false_branch: vec![IrStatement::Undefined].into_boxed_slice(),
        }))),
        wrap_statement(AstStatement::Return(None)),
    ];

    let function = build_test_function(function_id, "pattern_ir_if_target", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));

    (
        Ast {
            function_versions: HashMap::from([(function_id, version)]),
            functions: Arc::new(RwLock::new(functions)),
            last_variable_id: HashMap::new(),
            pre_defined_symbols: HashMap::new(),
        },
        function_id,
    )
}

fn build_predefined_pattern_test_ast() -> (Ast, AstFunctionId) {
    let function_id = AstFunctionId { address: 0x4700 };
    let version = AstFunctionVersion(1);
    let variable_map = Arc::new(RwLock::new(HashMap::new()));

    let body = vec![
        wrap_statement(AstStatement::Block(vec![wrap_statement(
            AstStatement::Undefined,
        )])),
        wrap_statement(AstStatement::Empty),
        wrap_statement(AstStatement::Block(Vec::new())),
        wrap_statement(AstStatement::If(
            wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            vec![wrap_statement(AstStatement::Comment("then".to_string()))],
            Some(Vec::new()),
        )),
        wrap_statement(AstStatement::Label("cleanup".to_string())),
        wrap_statement(AstStatement::Comment("keep".to_string())),
        wrap_statement(AstStatement::Return(None)),
    ];

    let function =
        build_test_function(function_id, "pattern_predefined_target", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));

    (
        Ast {
            function_versions: HashMap::from([(function_id, version)]),
            functions: Arc::new(RwLock::new(functions)),
            last_variable_id: HashMap::new(),
            pre_defined_symbols: HashMap::new(),
        },
        function_id,
    )
}

fn build_block_splice_pattern_test_ast() -> (Ast, AstFunctionId) {
    let function_id = AstFunctionId { address: 0x4710 };
    let version = AstFunctionVersion(1);
    let variable_map = Arc::new(RwLock::new(HashMap::new()));

    let body = vec![
        wrap_statement(AstStatement::Block(vec![wrap_statement(
            AstStatement::Comment("flattened".to_string()),
        )])),
        wrap_statement(AstStatement::Return(None)),
    ];

    let function = build_test_function(
        function_id,
        "pattern_block_splice_target",
        body,
        variable_map,
    );
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));

    (
        Ast {
            function_versions: HashMap::from([(function_id, version)]),
            functions: Arc::new(RwLock::new(functions)),
            last_variable_id: HashMap::new(),
            pre_defined_symbols: HashMap::new(),
        },
        function_id,
    )
}

fn build_ast_sequence_pattern_test_ast() -> (Ast, AstFunctionId) {
    let function_id = AstFunctionId { address: 0x4720 };
    let version = AstFunctionVersion(1);
    let variable_map = Arc::new(RwLock::new(HashMap::new()));

    let body = vec![
        wrap_statement(AstStatement::Undefined),
        wrap_statement(AstStatement::Return(None)),
    ];

    let function = build_test_function(
        function_id,
        "pattern_ast_sequence_target",
        body,
        variable_map,
    );
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));

    (
        Ast {
            function_versions: HashMap::from([(function_id, version)]),
            functions: Arc::new(RwLock::new(functions)),
            last_variable_id: HashMap::new(),
            pre_defined_symbols: HashMap::new(),
        },
        function_id,
    )
}

fn build_multi_return_pattern_test_ast() -> (Ast, AstFunctionId) {
    let function_id = AstFunctionId { address: 0x4800 };
    let version = AstFunctionVersion(1);
    let variable_map = Arc::new(RwLock::new(HashMap::new()));

    let body = vec![
        wrap_statement(AstStatement::Return(None)),
        wrap_statement(AstStatement::Comment("keep-me".to_string())),
        wrap_statement(AstStatement::Return(None)),
        wrap_statement(AstStatement::Return(None)),
    ];

    let function = build_test_function(
        function_id,
        "pattern_multi_return_target",
        body,
        variable_map,
    );
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));

    (
        Ast {
            function_versions: HashMap::from([(function_id, version)]),
            functions: Arc::new(RwLock::new(functions)),
            last_variable_id: HashMap::new(),
            pre_defined_symbols: HashMap::new(),
        },
        function_id,
    )
}

fn pattern_examples_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../patterns/examples")
        .to_path_buf()
}

fn example_pattern_path(file: &str) -> String {
    pattern_examples_dir()
        .join(file)
        .to_string_lossy()
        .to_string()
}

fn optimized_function_body(ast: &Ast, function_id: AstFunctionId) -> Vec<WrappedAstStatement> {
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

fn comment_count(body: &[WrappedAstStatement], text: &str) -> usize {
    body.iter()
        .filter(|stmt| matches!(&stmt.statement, AstStatement::Comment(comment) if comment == text))
        .count()
}

fn statement_debug_lines(body: &[WrappedAstStatement]) -> Vec<String> {
    body.iter()
        .map(|stmt| format!("{:?}", stmt.statement))
        .collect()
}

#[test]
fn pattern_matching_example_fb_files_parse_and_execute() {
    let files = [
        "if_do_asm_ir_ast.fb",
        "skip_ranges.fb",
        "skip_aliases.fb",
        "script_and_logs.fb",
        "all_syntax.fb",
        "do_del_syntax.fb",
    ];

    for file in files {
        let (ast, function_id) = build_pattern_test_ast();
        let pattern = AstPattern::from_file(example_pattern_path(file));
        let result = ast.optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        );
        if let Err(err) = result {
            panic!("example pattern file `{file}` failed: {err:?}");
        }
    }
}

#[test]
fn pattern_matching_multiple_if_do_clauses_pair_actions_with_their_group() {
    let pattern = AstPattern::new(
        "paired-if-do",
        r#"
if:
  ast comment seed-comment
do:
  ast comment first-only
if:
  ast comment missing-comment
do:
  ast comment second-only
"#,
    );

    let (ast, function_id) = build_pattern_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(2),
            ),
        )
        .expect("paired if/do clauses must optimize successfully");
    let body = optimized_function_body(&optimized, function_id);

    assert_eq!(comment_count(&body, "first-only"), 1);
    assert_eq!(comment_count(&body, "second-only"), 0);
}

#[test]
fn pattern_matching_invalid_symbol_only_asm_is_rejected() {
    let pattern = AstPattern::new(
        "invalid-asm",
        r#"
if:
  asm __stack_chk_fail
do:
  ast comment should-not-parse
"#,
    );

    let (ast, function_id) = build_pattern_test_ast();
    let result = ast.optimize_function(
        function_id,
        Some(
            AstOptimizationConfig::NONE
                .pattern_matching_enabled(true)
                .pattern_matching(vec![pattern])
                .max_pass_iterations(1),
        ),
    );

    assert!(
        result.is_err(),
        "symbol-only asm must be rejected instead of being treated as valid asm"
    );
}

#[test]
fn pattern_matching_asm_contains_matches_symbol_text_without_invalid_asm() {
    let pattern = AstPattern::new(
        "asm-contains",
        r#"
if:
  at beforeIrAnalyzation
  asm_contains __stack_chk_fail
do:
  ast comment asm-contains-hit
"#,
    );

    let (ast, function_id) = build_asm_contains_pattern_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("asm_contains pattern must optimize successfully");
    let body = optimized_function_body(&optimized, function_id);

    assert_eq!(comment_count(&body, "asm-contains-hit"), 1);
}

#[test]
fn pattern_matching_actions_example_applies_key_actions() {
    let (ast, function_id) = build_pattern_test_ast();
    let pattern = AstPattern::from_file(example_pattern_path("if_do_asm_ir_ast.fb"));
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("all syntax example must optimize successfully");

    let optimized_version = *optimized
        .function_versions
        .get(&function_id)
        .expect("optimized function version should exist");
    let functions = optimized.functions.read().unwrap();
    let versions = functions
        .get(&function_id)
        .expect("optimized function should exist");
    let function = versions
        .get(&optimized_version)
        .expect("optimized function version should exist");

    assert!(
        function.body.iter().any(|stmt| matches!(
            &stmt.statement,
            AstStatement::Ir(ir_stmt) if matches!(ir_stmt.as_ref(), IrStatement::Halt)
        )),
        "actions example must apply `do: ir halt` action"
    );
    assert!(
        function.body.iter().any(|stmt| matches!(
            &stmt.statement,
            AstStatement::Comment(text) if text == "replaced-from-01"
        )),
        "actions example must apply `do: ast comment replaced-from-01` action"
    );
}

#[test]
fn pattern_matching_label_cleanup_matches_label_statements() {
    let function_id = AstFunctionId { address: 0x4A00 };
    let version = AstFunctionVersion(1);
    let variable_map = Arc::new(RwLock::new(HashMap::new()));
    let body = vec![
        wrap_statement(AstStatement::Comment("seed-comment".to_string())),
        wrap_statement(AstStatement::Label("cleanup".to_string())),
        wrap_statement(AstStatement::Return(None)),
    ];
    let function = build_test_function(function_id, "pattern_label_target", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };
    let pattern = AstPattern::new(
        "label-cleanup",
        r#"
if:
  at afterOptimization
  ast Label(cleanup)
do:
  ast comment error cleanup handler
"#,
    );

    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("label cleanup pattern must parse and execute");
    let body = optimized_function_body(&optimized, function_id);

    assert_eq!(comment_count(&body, "error cleanup handler"), 1);
}

#[test]
fn pattern_matching_splice_block_flattens_nonempty_block() {
    let (ast, function_id) = build_block_splice_pattern_test_ast();
    let pattern = AstPattern::new(
        "splice-block",
        r#"
if:
  at afterIteration
  ast Block(...)
do:
  splice-block
"#,
    );

    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("splice-block pattern must parse and execute");
    let body = optimized_function_body(&optimized, function_id);

    assert_eq!(comment_count(&body, "flattened"), 1);
    assert!(
        !body
            .iter()
            .any(|stmt| matches!(&stmt.statement, AstStatement::Block(_))),
        "splice-block should remove matched standalone blocks from the current statement list"
    );
}

#[test]
fn pattern_matching_emit_before_and_emit_after_insert_relative_to_match() {
    let (ast, function_id) = build_pattern_test_ast();
    let pattern = AstPattern::new(
        "emit-before-after",
        r#"
if:
  at afterIteration
  script `!ast_stmts.some(|s| s.is_comment() && s.comment_text().contains("before_seed"))`
  stmt Return
do:
  emit_before Comment(before_seed)
  emit_after Comment(after_seed)
"#,
    );

    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("emit_before/emit_after pattern must parse and execute");
    let body = optimized_function_body(&optimized, function_id);

    let lines = statement_debug_lines(&body);
    let seed_index = lines
        .iter()
        .position(|line| line == "Comment(\"seed-comment\")")
        .expect("seed-comment must remain present");
    let before_index = lines
        .iter()
        .position(|line| line == "Comment(\"before_seed\")")
        .expect("emit_before must insert the new statement");
    let return_index = lines
        .iter()
        .position(|line| line == "Return(None)")
        .expect("the matched return statement must remain present");
    let after_index = lines
        .iter()
        .position(|line| line == "Comment(\"after_seed\")")
        .expect("emit_after must insert the new statement");

    assert!(
        seed_index < before_index && before_index < return_index && return_index < after_index,
        "emit_before must insert before the matched statement and emit_after must insert after it"
    );
}

#[test]
fn pattern_matching_ast_sequence_requires_contiguous_order() {
    let (ast, function_id) = build_ast_sequence_pattern_test_ast();
    let pattern = AstPattern::new(
        "ast-sequence-order",
        r#"
if:
  at afterIteration
  ast undefined; return
do:
  ast comment ast-seq-hit
if:
  at afterIteration
  ast return; undefined
do:
  ast comment ast-seq-wrong-order
"#,
    );

    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("ast sequence pattern must parse and execute");
    let body = optimized_function_body(&optimized, function_id);

    assert_eq!(comment_count(&body, "ast-seq-hit"), 1);
    assert_eq!(comment_count(&body, "ast-seq-wrong-order"), 0);
}

#[test]
fn pattern_matching_do_del_syntax_deletes_statement_based_ranges() {
    let (ast, function_id) = build_pattern_test_ast();
    let pattern = AstPattern::from_file(example_pattern_path("do_del_syntax.fb"));
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("do-del syntax example must optimize successfully");

    let optimized_version = *optimized
        .function_versions
        .get(&function_id)
        .expect("optimized function version should exist");
    let functions = optimized.functions.read().unwrap();
    let versions = functions
        .get(&function_id)
        .expect("optimized function should exist");
    let function = versions
        .get(&optimized_version)
        .expect("optimized function version should exist");

    assert!(
        function.body.len() <= 1,
        "do-del syntax should delete statements using statement indices, even when pattern matching runs repeatedly"
    );
}

#[test]
fn pattern_matching_single_iteration_reaches_fixed_point_for_repeated_matches() {
    let pattern = AstPattern::new(
        "multi-return-delete",
        r#"
if:
  ast return
do:
  del start[0]
"#,
    );

    let (ast, function_id) = build_multi_return_pattern_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("fixed-point multi-match test must optimize successfully");

    let body = optimized_function_body(&optimized, function_id);
    assert_eq!(
        body.iter()
            .filter(|stmt| matches!(&stmt.statement, AstStatement::Return(_)))
            .count(),
        0,
        "single optimizer iteration should consume all matching `ast return` statements"
    );
    assert_eq!(
        comment_count(&body, "keep-me"),
        1,
        "non-matching statements should remain after fixed-point convergence"
    );
}

#[test]
fn pattern_matching_ast_if_ellipsis_matches_nested_content() {
    let pattern = AstPattern::new(
        "ast-if-ellipsis",
        r#"
if:
  ast if(...)
do:
  ast comment ast-if-ellipsis-hit
"#,
    );

    let (ast, function_id) = build_ast_if_pattern_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("ast if(...) ellipsis pattern must optimize successfully");

    let body = optimized_function_body(&optimized, function_id);
    assert_eq!(
        comment_count(&body, "ast-if-ellipsis-hit"),
        1,
        "ast if(...) should match even when nested branch contents vary"
    );
}

#[test]
fn pattern_matching_ir_if_ellipsis_matches_nested_content() {
    let pattern = AstPattern::new(
        "ir-if-ellipsis",
        r#"
if:
  ir if(...)
do:
  ast comment ir-if-ellipsis-hit
"#,
    );

    let (ast, function_id) = build_ir_if_pattern_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("ir if(...) ellipsis pattern must optimize successfully");

    let body = optimized_function_body(&optimized, function_id);
    assert_eq!(
        comment_count(&body, "ir-if-ellipsis-hit"),
        1,
        "ir if(...) should match regardless of nested branch statements"
    );
}

#[test]
fn pattern_matching_predefined_patterns_apply_without_explicit_pattern_list() {
    let (ast, function_id) = build_predefined_pattern_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .max_pass_iterations(1),
            ),
        )
        .expect("predefined patterns must optimize successfully");

    let body = optimized_function_body(&optimized, function_id);
    assert!(
        !body
            .iter()
            .any(|stmt| matches!(&stmt.statement, AstStatement::Block(block) if block.is_empty())),
        "predefined collapse-empty-blocks should remove empty blocks"
    );
    assert!(
        body.iter()
            .any(|stmt| matches!(&stmt.statement, AstStatement::Undefined)),
        "predefined flatten-blocks should splice non-empty standalone blocks"
    );
    assert!(
        !body
            .iter()
            .any(|stmt| matches!(&stmt.statement, AstStatement::Block(_))),
        "predefined cleanup patterns should leave no standalone top-level block statements in this fixture"
    );
    assert!(
        body.iter().any(|stmt| {
            matches!(
                &stmt.statement,
                AstStatement::If(_, _, branch_false) if branch_false.is_none()
            )
        }),
        "predefined prune-empty-else should remove empty else branches"
    );
    assert_eq!(
        comment_count(&body, "error cleanup handler"),
        1,
        "predefined error-cleanup should rewrite cleanup labels to comments"
    );
}

#[test]
fn pattern_matching_predefined_registry_includes_script_bearing_examples() {
    // Example patterns are available via individual lookup but excluded from
    // the default decompilation set (predefined_patterns()).
    assert!(
        AstPattern::predefined_pattern("script_and_logs.fb").is_some(),
        "script_and_logs.fb must be available via predefined_pattern lookup"
    );
    assert!(
        AstPattern::predefined_pattern("all_syntax.fb").is_some(),
        "all_syntax.fb must be available via predefined_pattern lookup"
    );

    let predefined = AstPattern::predefined_patterns();
    assert!(
        !predefined
            .iter()
            .any(|pattern| { pattern.name() == "script_and_logs.fb" }),
        "example patterns should NOT be in the default decompilation set"
    );
    assert!(
        predefined
            .iter()
            .any(|pattern| { pattern.name() == "flatten-blocks.fb" }),
        "predefined registry must include the generic flatten-blocks rule"
    );
    assert!(
        predefined
            .iter()
            .any(|pattern| { pattern.name() == "error-cleanup.fb" }),
        "predefined registry must include the generic error-cleanup rule"
    );
}

#[test]
fn pattern_matching_new_parses_inline_pattern_and_infers_input_type() {
    let pattern = AstPattern::new(
        "inline-asm-only",
        r#"
if:
  asm push rbp
do:
  ast comment replaced-from-inline
"#,
    );
    assert_eq!(pattern.input_type(), AstPatternInputType::WithAssembly);

    let (ast, function_id) = build_pattern_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("inline pattern must optimize successfully");

    let optimized_version = *optimized
        .function_versions
        .get(&function_id)
        .expect("optimized function version should exist");
    let functions = optimized.functions.read().unwrap();
    let versions = functions
        .get(&function_id)
        .expect("optimized function should exist");
    let function = versions
        .get(&optimized_version)
        .expect("optimized function version should exist");

    assert!(
        function.body.iter().any(|stmt| matches!(
            &stmt.statement,
            AstStatement::Comment(text) if text == "replaced-from-inline"
        )),
        "inline pattern from AstPattern::new must be parsed and applied"
    );
}

#[test]
fn pattern_matching_at_before_ir_analyzation_runs_only_once() {
    let pattern = AstPattern::new(
        "at-before-ir",
        r#"
if:
  at beforeIrAnalyzation
  asm push rbp
do:
  ast comment at-before-ir
"#,
    );

    let (ast, function_id) = build_pattern_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(3),
            ),
        )
        .expect("before-ir phase pattern must optimize successfully");

    let body = optimized_function_body(&optimized, function_id);
    assert_eq!(
        comment_count(&body, "at-before-ir"),
        1,
        "at beforeIrAnalyzation should run only at the initial pre-IR phase"
    );
}

#[test]
fn pattern_matching_at_after_ir_analyzation_requires_ir_pass() {
    let pattern = AstPattern::new(
        "at-after-ir",
        r#"
if:
  at afterIrAnalyzation
  asm push rbp
do:
  ast comment at-after-ir
"#,
    );

    let (ast_without_ir, function_id_without_ir) = build_pattern_test_ast();
    let optimized_without_ir = ast_without_ir
        .optimize_function(
            function_id_without_ir,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern.clone()])
                    .max_pass_iterations(1),
            ),
        )
        .expect("pattern optimization without ir analyzation should succeed");
    let body_without_ir = optimized_function_body(&optimized_without_ir, function_id_without_ir);
    assert_eq!(
        comment_count(&body_without_ir, "at-after-ir"),
        0,
        "at afterIrAnalyzation should not run when ir_analyzation is disabled"
    );

    let (ast_with_ir, function_id_with_ir) = build_pattern_test_ast();
    let optimized_with_ir = ast_with_ir
        .optimize_function(
            function_id_with_ir,
            Some(
                AstOptimizationConfig::NONE
                    .ir_analyzation(true)
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("pattern optimization with ir analyzation should succeed");
    let body_with_ir = optimized_function_body(&optimized_with_ir, function_id_with_ir);
    assert_eq!(
        comment_count(&body_with_ir, "at-after-ir"),
        1,
        "at afterIrAnalyzation should run exactly once after ir_analyzation"
    );
}

#[test]
fn pattern_matching_at_after_iteration_reaches_fixed_point() {
    let pattern = AstPattern::new(
        "at-after-iteration",
        r#"
if:
  at afterIteration
do:
  del start[0]
"#,
    );

    let (ast, function_id) = build_pattern_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(3),
            ),
        )
        .expect("after-iteration phase pattern must optimize successfully");

    let body = optimized_function_body(&optimized, function_id);
    assert_eq!(
        body.len(),
        0,
        "at afterIteration should keep applying matching rules until the statement list reaches a fixed-point"
    );
}

#[test]
fn pattern_matching_at_any_behaves_like_no_at() {
    let no_at_pattern = AstPattern::new(
        "no-at",
        r#"
if:
  ast return
do:
  del start[0]
"#,
    );
    let at_any_pattern = AstPattern::new(
        "at-any",
        r#"
if:
  at any
  ast return
do:
  del start[0]
"#,
    );

    let (ast_no_at, function_id_no_at) = build_pattern_test_ast();
    let optimized_no_at = ast_no_at
        .optimize_function(
            function_id_no_at,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![no_at_pattern])
                    .max_pass_iterations(3),
            ),
        )
        .expect("pattern without at should optimize successfully");
    let body_no_at = optimized_function_body(&optimized_no_at, function_id_no_at);

    let (ast_at_any, function_id_at_any) = build_pattern_test_ast();
    let optimized_at_any = ast_at_any
        .optimize_function(
            function_id_at_any,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![at_any_pattern])
                    .max_pass_iterations(3),
            ),
        )
        .expect("pattern with at any should optimize successfully");
    let body_at_any = optimized_function_body(&optimized_at_any, function_id_at_any);

    assert_eq!(
        statement_debug_lines(&body_at_any),
        statement_debug_lines(&body_no_at),
        "at any must behave exactly like missing at and run with the same frequency"
    );
}

#[test]
fn pattern_matching_multiple_at_clauses_use_or_semantics() {
    let pattern = AstPattern::new(
        "at-or",
        r#"
if:
  at beforeIrAnalyzation
  at afterIteration
  ast return
do:
  ast comment at-or-hit
"#,
    );

    let (ast, function_id) = build_pattern_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(3),
            ),
        )
        .expect("pattern with multiple at clauses must optimize successfully");
    let body = optimized_function_body(&optimized, function_id);

    assert_eq!(
        comment_count(&body, "at-or-hit"),
        1,
        "multiple at clauses should be OR'ed so a match can run when either phase is active"
    );
}

// ---------------------------------------------------------------------------
// stmt / where / emit  golden-output tests
// ---------------------------------------------------------------------------

/// Build a test AST that contains `if (cond) { x = a; } else { x = b; }`
/// to validate ternary recovery via the .fb pattern.
fn build_ternary_recovery_test_ast() -> (Ast, AstFunctionId) {
    let function_id = AstFunctionId { address: 0xBB00 };
    let version = AstFunctionVersion(1);
    let variable_map: ArcAstVariableMap = Arc::new(RwLock::new(HashMap::new()));
    let var_id = AstVariableId {
        index: 42,
        parent: Some(function_id),
    };

    let cond = wrap_expression(AstExpression::Variable(
        variable_map.clone(),
        AstVariableId {
            index: 1,
            parent: Some(function_id),
        },
    ));
    let val_a = wrap_expression(AstExpression::Literal(AstLiteral::Int(10)));
    let val_b = wrap_expression(AstExpression::Literal(AstLiteral::Int(20)));
    let lhs_true = wrap_expression(AstExpression::Variable(variable_map.clone(), var_id));
    let lhs_false = wrap_expression(AstExpression::Variable(variable_map.clone(), var_id));

    let if_stmt = AstStatement::If(
        cond,
        vec![wrap_statement(AstStatement::Assignment(lhs_true, val_a))],
        Some(vec![wrap_statement(AstStatement::Assignment(
            lhs_false, val_b,
        ))]),
    );

    let body = vec![
        wrap_statement(AstStatement::Comment("before-ternary".to_string())),
        wrap_statement(if_stmt),
        wrap_statement(AstStatement::Return(None)),
    ];

    let function = build_test_function(function_id, "ternary_test", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };
    (ast, function_id)
}

#[test]
fn stmt_pattern_ternary_recovery_fb_produces_correct_output() {
    let pattern = AstPattern::new(
        "ternary-recovery",
        r#"
if:
  at afterIteration
  stmt If($cond, [Assignment(Variable($_, $v1), $a)], Some([Assignment(Variable($_, $v2), $b)]))
  where eq($v1, $v2)
do:
  emit Assignment(Variable($_, $v1), Ternary($cond, $a, $b))
"#,
    );

    let (ast, function_id) = build_ternary_recovery_test_ast();
    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("ternary recovery pattern must parse and execute");
    let body = optimized_function_body(&optimized, function_id);

    // The If statement should be replaced with an Assignment containing a Ternary
    let has_ternary = body.iter().any(|stmt| match &stmt.statement {
        AstStatement::Assignment(_, rhs) => matches!(rhs.item, AstExpression::Ternary(_, _, _)),
        _ => false,
    });
    assert!(
        has_ternary,
        "ternary recovery .fb pattern must convert if(c){{x=a}}else{{x=b}} to x=c?a:b"
    );

    // The If statement should be gone
    let has_if = body
        .iter()
        .any(|stmt| matches!(&stmt.statement, AstStatement::If(_, _, _)));
    assert!(
        !has_if,
        "ternary recovery .fb pattern must remove the original If statement"
    );

    // Other statements (comment + return) must be preserved
    assert_eq!(body.len(), 3, "surrounding statements must be preserved");
    assert!(
        matches!(&body[0].statement, AstStatement::Comment(s) if s == "before-ternary"),
        "comment before must survive"
    );
    assert!(
        matches!(&body[2].statement, AstStatement::Return(None)),
        "return after must survive"
    );
}

#[test]
fn stmt_pattern_ternary_recovery_rejects_different_variables() {
    let pattern = AstPattern::new(
        "ternary-recovery",
        r#"
if:
  at afterIteration
  stmt If($cond, [Assignment(Variable($_, $v1), $a)], Some([Assignment(Variable($_, $v2), $b)]))
  where eq($v1, $v2)
do:
  emit Assignment(Variable($_, $v1), Ternary($cond, $a, $b))
"#,
    );

    let function_id = AstFunctionId { address: 0xBC00 };
    let version = AstFunctionVersion(1);
    let variable_map: ArcAstVariableMap = Arc::new(RwLock::new(HashMap::new()));
    let var_id_x = AstVariableId {
        index: 42,
        parent: Some(function_id),
    };
    let var_id_y = AstVariableId {
        index: 99,
        parent: Some(function_id),
    };

    let cond = wrap_expression(AstExpression::Variable(
        variable_map.clone(),
        AstVariableId {
            index: 1,
            parent: Some(function_id),
        },
    ));
    let lhs_true = wrap_expression(AstExpression::Variable(variable_map.clone(), var_id_x));
    let lhs_false = wrap_expression(AstExpression::Variable(variable_map.clone(), var_id_y));
    let val_a = wrap_expression(AstExpression::Literal(AstLiteral::Int(10)));
    let val_b = wrap_expression(AstExpression::Literal(AstLiteral::Int(20)));

    let if_stmt = AstStatement::If(
        cond,
        vec![wrap_statement(AstStatement::Assignment(lhs_true, val_a))],
        Some(vec![wrap_statement(AstStatement::Assignment(
            lhs_false, val_b,
        ))]),
    );

    let body = vec![wrap_statement(if_stmt)];
    let function = build_test_function(function_id, "ternary_reject", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("pattern must execute without error");
    let body = optimized_function_body(&optimized, function_id);

    // The If should NOT be converted because v1 != v2
    let has_if = body
        .iter()
        .any(|stmt| matches!(&stmt.statement, AstStatement::If(_, _, _)));
    assert!(
        has_if,
        "where eq($v1, $v2) must reject when variables differ"
    );
}

#[test]
fn stmt_pattern_ternary_recovery_handles_nested_if() {
    let pattern = AstPattern::new(
        "ternary-recovery",
        r#"
if:
  at afterIteration
  stmt If($cond, [Assignment(Variable($_, $v1), $a)], Some([Assignment(Variable($_, $v2), $b)]))
  where eq($v1, $v2)
do:
  emit Assignment(Variable($_, $v1), Ternary($cond, $a, $b))
"#,
    );

    let function_id = AstFunctionId { address: 0xBD00 };
    let version = AstFunctionVersion(1);
    let variable_map: ArcAstVariableMap = Arc::new(RwLock::new(HashMap::new()));
    let var_id = AstVariableId {
        index: 5,
        parent: Some(function_id),
    };
    let cond_var = AstVariableId {
        index: 1,
        parent: Some(function_id),
    };

    // Build: while(true) { if(c) { x = 1 } else { x = 2 } }
    let inner_cond = wrap_expression(AstExpression::Variable(variable_map.clone(), cond_var));
    let inner_if = AstStatement::If(
        inner_cond,
        vec![wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(variable_map.clone(), var_id)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
        ))],
        Some(vec![wrap_statement(AstStatement::Assignment(
            wrap_expression(AstExpression::Variable(variable_map.clone(), var_id)),
            wrap_expression(AstExpression::Literal(AstLiteral::Int(2))),
        ))]),
    );
    let while_stmt = AstStatement::While(
        wrap_expression(AstExpression::Literal(AstLiteral::Bool(true))),
        vec![wrap_statement(inner_if)],
    );

    let body = vec![wrap_statement(while_stmt)];
    let function = build_test_function(function_id, "ternary_nested", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("nested ternary recovery must work");
    let body = optimized_function_body(&optimized, function_id);

    // The while loop should contain an Assignment with Ternary, not an If
    let while_body = match &body[0].statement {
        AstStatement::While(_, body) => body,
        other => panic!("expected While, got {other:?}"),
    };
    let has_ternary = while_body.iter().any(|stmt| match &stmt.statement {
        AstStatement::Assignment(_, rhs) => matches!(rhs.item, AstExpression::Ternary(_, _, _)),
        _ => false,
    });
    assert!(
        has_ternary,
        "ternary recovery must work inside nested while loops"
    );
}

#[test]
fn stmt_pattern_if_conversion_reversal_expands_nested_ternary() {
    let pattern = AstPattern::new(
        "if-conversion-reversal",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../patterns/recovery/after-iteration/if-conversion-reversal.fb"
        )),
    );

    let function_id = AstFunctionId { address: 0xBE00 };
    let version = AstFunctionVersion(1);
    let variable_map: ArcAstVariableMap = Arc::new(RwLock::new(HashMap::new()));
    let var_id = AstVariableId {
        index: 7,
        parent: Some(function_id),
    };
    let cond1_id = AstVariableId {
        index: 1,
        parent: Some(function_id),
    };
    let cond2_id = AstVariableId {
        index: 2,
        parent: Some(function_id),
    };

    // x = c1 ? (c2 ? 10 : 20) : 30
    let inner_ternary = AstExpression::Ternary(
        Box::new(wrap_expression(AstExpression::Variable(
            variable_map.clone(),
            cond2_id,
        ))),
        Box::new(wrap_expression(AstExpression::Literal(AstLiteral::Int(10)))),
        Box::new(wrap_expression(AstExpression::Literal(AstLiteral::Int(20)))),
    );
    let outer_ternary = AstExpression::Ternary(
        Box::new(wrap_expression(AstExpression::Variable(
            variable_map.clone(),
            cond1_id,
        ))),
        Box::new(wrap_expression(inner_ternary)),
        Box::new(wrap_expression(AstExpression::Literal(AstLiteral::Int(30)))),
    );
    let assign = AstStatement::Assignment(
        wrap_expression(AstExpression::Variable(variable_map.clone(), var_id)),
        wrap_expression(outer_ternary),
    );

    let body = vec![wrap_statement(assign)];
    let function = build_test_function(function_id, "if_conv_reversal_test", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(3),
            ),
        )
        .expect("if-conversion-reversal pattern must work");
    let body = optimized_function_body(&optimized, function_id);

    // The outer ternary should become an If statement
    let has_if = body
        .iter()
        .any(|stmt| matches!(&stmt.statement, AstStatement::If(_, _, _)));
    assert!(
        has_if,
        "if-conversion-reversal must expand nested ternary assignment to if-else"
    );
    // No top-level ternary assignment should remain
    let has_ternary_assign = body.iter().any(|stmt| match &stmt.statement {
        AstStatement::Assignment(_, rhs) => matches!(rhs.item, AstExpression::Ternary(_, _, _)),
        _ => false,
    });
    assert!(
        !has_ternary_assign,
        "nested ternary assignment should be fully expanded"
    );
}

#[test]
fn stmt_pattern_if_conversion_reversal_preserves_simple_ternary() {
    let pattern = AstPattern::new(
        "if-conversion-reversal",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../patterns/recovery/after-iteration/if-conversion-reversal.fb"
        )),
    );

    let function_id = AstFunctionId { address: 0xBF00 };
    let version = AstFunctionVersion(1);
    let variable_map: ArcAstVariableMap = Arc::new(RwLock::new(HashMap::new()));
    let var_id = AstVariableId {
        index: 7,
        parent: Some(function_id),
    };

    // x = c ? 10 : 20  (no nesting — should NOT be expanded)
    let simple_ternary = AstExpression::Ternary(
        Box::new(wrap_expression(AstExpression::Variable(
            variable_map.clone(),
            AstVariableId {
                index: 1,
                parent: Some(function_id),
            },
        ))),
        Box::new(wrap_expression(AstExpression::Literal(AstLiteral::Int(10)))),
        Box::new(wrap_expression(AstExpression::Literal(AstLiteral::Int(20)))),
    );
    let assign = AstStatement::Assignment(
        wrap_expression(AstExpression::Variable(variable_map.clone(), var_id)),
        wrap_expression(simple_ternary),
    );

    let body = vec![wrap_statement(assign)];
    let function = build_test_function(function_id, "if_conv_preserve", body, variable_map);
    let mut functions = HashMap::new();
    functions.insert(function_id, VersionMap::new(version, function));
    let ast = Ast {
        function_versions: HashMap::from([(function_id, version)]),
        functions: Arc::new(RwLock::new(functions)),
        last_variable_id: HashMap::new(),
        pre_defined_symbols: HashMap::new(),
    };

    let optimized = ast
        .optimize_function(
            function_id,
            Some(
                AstOptimizationConfig::NONE
                    .pattern_matching_enabled(true)
                    .pattern_matching(vec![pattern])
                    .max_pass_iterations(1),
            ),
        )
        .expect("simple ternary should be preserved");
    let body = optimized_function_body(&optimized, function_id);

    // Simple ternary should NOT be expanded
    let has_ternary_assign = body.iter().any(|stmt| match &stmt.statement {
        AstStatement::Assignment(_, rhs) => matches!(rhs.item, AstExpression::Ternary(_, _, _)),
        _ => false,
    });
    assert!(
        has_ternary_assign,
        "simple (non-nested) ternary must be preserved — only nested ternaries get expanded"
    );
}

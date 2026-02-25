use crate::{
    abstract_syntax_tree::{
        Ast, AstExpression, AstFunction, AstFunctionId, AstFunctionVersion, AstLiteral,
        AstOptimizationConfig, AstStatement, AstStatementOrigin, AstValueOrigin, AstValueType,
        AstVariable, AstVariableId, Wrapped, WrappedAstStatement,
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
        wrap_statement(AstStatement::Empty),
        wrap_statement(AstStatement::Block(Vec::new())),
        wrap_statement(AstStatement::If(
            wrap_expression(AstExpression::Literal(AstLiteral::Int(1))),
            Vec::new(),
            Some(Vec::new()),
        )),
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

fn pattern_examples_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../patterns/pattern_matching_examples")
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
        "01_if_do_asm_ir_ast.fb",
        "02_skip_ranges.fb",
        "03_skip_aliases.fb",
        "04_script_and_logs.fb",
        "05_all_syntax.fb",
        "06_do_del_syntax.fb",
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
fn pattern_matching_actions_example_applies_key_actions() {
    let (ast, function_id) = build_pattern_test_ast();
    let pattern = AstPattern::from_file(example_pattern_path("01_if_do_asm_ir_ast.fb"));
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
fn pattern_matching_do_del_syntax_deletes_statement_based_ranges() {
    let (ast, function_id) = build_pattern_test_ast();
    let pattern = AstPattern::from_file(example_pattern_path("06_do_del_syntax.fb"));
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
            .any(|stmt| matches!(stmt.statement, AstStatement::Empty)),
        "predefined remove-empty-statements should remove empty statements"
    );
    assert!(
        !body
            .iter()
            .any(|stmt| matches!(&stmt.statement, AstStatement::Block(block) if block.is_empty())),
        "predefined collapse-empty-blocks should remove empty blocks"
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
fn pattern_matching_at_after_iteration_runs_each_iteration() {
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
        2,
        "at afterIteration should run once per loop iteration and delete one statement each time"
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
        .expect("pattern with multiple at clauses must optimize successfully");
    let body = optimized_function_body(&optimized, function_id);

    assert_eq!(
        body.len(),
        1,
        "multiple at clauses should be OR'ed and run once for each matching phase"
    );
}

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunction, AstFunctionId, AstFunctionVersion, AstOptimizationConfig, AstStatement,
        AstStatementOrigin, AstValueType, AstVariable, AstVariableId, WrappedAstStatement,
        pattern_matching::AstPattern,
    },
    core::Instruction,
    ir::{analyze::IrFunction, statements::IrStatement},
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

#[test]
fn pattern_matching_example_fb_files_parse_and_execute() {
    let files = [
        "01_if_do_asm_ir_ast.fb",
        "02_skip_ranges.fb",
        "03_skip_aliases.fb",
        "04_script_and_logs.fb",
        "05_all_syntax.fb",
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

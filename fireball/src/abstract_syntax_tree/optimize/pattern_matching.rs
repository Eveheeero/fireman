use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, AstStatementOrigin,
        ProcessedOptimization, WrappedAstStatement,
    },
    ir::{
        analyze::datatype::DataType,
        data::{IrAccessSize, IrData, IrDataOperation, IrIntrinsic},
        operator::{IrBinaryOperator, IrUnaryOperator},
        statements::{IrStatement, IrStatementSpecial},
    },
    prelude::DecompileError,
    utils::Aos,
};
use rhai::{AST as RhaiAst, Dynamic, Engine, Scope};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
    num::NonZeroU8,
    panic::AssertUnwindSafe,
    time::SystemTime,
};
use tracing::{debug, error, info, trace, warn};

#[derive(Debug, Clone)]
pub struct AstPattern {
    pub name: String,
    pub origin: AstPatternOrigin,
    pub arg: AstPatternArgType,
    pub pattern: String,
    parsed: AstPatternParsed,
}
impl PartialEq for AstPattern {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.origin == other.origin
            && self.arg == other.arg
            && self.pattern == other.pattern
    }
}
impl Eq for AstPattern {}
impl AstPattern {
    pub const ALL: Vec<Self> = vec![];

    pub fn new(
        name: impl Into<String>,
        origin: AstPatternOrigin,
        arg: AstPatternArgType,
        pattern: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            origin,
            arg,
            pattern: pattern.into(),
            parsed: AstPatternParsed::None,
        }
    }

    pub fn from_file(path: impl Into<String>) -> Self {
        let path = path.into();
        let parsed = match load_file_pattern_rule_uncached(&path) {
            Ok(rule) => AstPatternParsed::File(rule),
            Err(err) => AstPatternParsed::ParseError(err),
        };
        Self {
            name: path.clone(),
            origin: AstPatternOrigin::File,
            arg: AstPatternArgType::WithOptimizedAst,
            pattern: path,
            parsed,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstPatternOrigin {
    PreDefined,
    UserInput,
    File,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstPatternArgType {
    WithAssembly,
    WithIr,
    WithAst,
    WithOptimizedAst,
}

#[derive(Debug, Clone)]
enum AstPatternParsed {
    None,
    File(FilePatternRule),
    ParseError(String),
}

#[derive(Debug, Clone, Default)]
struct FilePatternMatch {
    asm_statement_range: Option<(usize, usize)>,
}

#[derive(Debug, Clone, Default)]
struct FilePatternRule {
    source: String,
    in_blocks: Vec<Vec<FilePatternInBlock>>,
    out_actions: Vec<FilePatternOutAction>,
}

#[derive(Debug, Clone, Default)]
struct FilePatternInBlock {
    asm: FilePatternAsmData,
    ast: FilePatternAstData,
    ir: FilePatternIrData,
    script_conditions: FilePatternScript,
    skip_range: Option<FilePatternRange>,
    skip_asm_range: Option<FilePatternRange>,
    skip_ast_range: Option<FilePatternRange>,
    skip_ir_range: Option<FilePatternRange>,
}

#[derive(Debug, Clone)]
enum FilePatternOutAction {
    ReplaceAsm(FilePatternAsmData),
    ReplaceIr(FilePatternIrReplacement),
    ReplaceAst(AstStatement),
    Script(FilePatternScript),
    Log(FilePatternLogLevel, String),
}

#[derive(Debug, Clone, Copy)]
enum FilePatternLogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Trace,
}

#[derive(Debug, Clone)]
struct FilePatternIrReplacement {
    statement: Option<IrStatement>,
    fallback_comment: String,
}

#[derive(Debug, Clone)]
struct FilePatternAsmData {
    source: String,
    statement: AstStatement,
    enabled: bool,
}
impl FilePatternAsmData {
    fn from_text(text: &str) -> Option<Self> {
        let value = text.trim();
        if value.is_empty() {
            return None;
        }
        Some(Self {
            source: value.to_string(),
            statement: parse_asm_statement(value)
                .map(|statement| AstStatement::Ir(Box::new(statement)))
                .unwrap_or_else(|| AstStatement::Comment(format!("asm {value}"))),
            enabled: true,
        })
    }

    fn is_empty(&self) -> bool {
        !self.enabled
    }

    fn as_match_text(&self) -> &str {
        self.source.trim()
    }
}

impl Default for FilePatternAsmData {
    fn default() -> Self {
        Self {
            source: String::new(),
            statement: AstStatement::Empty,
            enabled: false,
        }
    }
}

#[derive(Debug, Clone)]
struct FilePatternAstData {
    statement: AstStatement,
    enabled: bool,
}
impl FilePatternAstData {
    fn from_text(text: &str) -> Option<Self> {
        let value = text.trim();
        if value.is_empty() {
            return None;
        }
        Some(Self {
            statement: parse_ast_replacement(value),
            enabled: true,
        })
    }

    fn is_empty(&self) -> bool {
        !self.enabled
    }
}

impl Default for FilePatternAstData {
    fn default() -> Self {
        Self {
            statement: AstStatement::Empty,
            enabled: false,
        }
    }
}

#[derive(Debug, Clone)]
struct FilePatternIrData {
    statement: IrStatement,
    enabled: bool,
}
impl FilePatternIrData {
    fn from_text(text: &str) -> Option<Self> {
        let value = text.trim();
        if value.is_empty() {
            return None;
        }
        let statement = parse_ir_statement(value)?;
        Some(Self {
            statement,
            enabled: true,
        })
    }

    fn is_empty(&self) -> bool {
        !self.enabled
    }
}

impl Default for FilePatternIrData {
    fn default() -> Self {
        Self {
            statement: IrStatement::Undefined,
            enabled: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct FilePatternScript {
    sources: String,
    compiled: Option<RhaiAst>,
}
impl FilePatternScript {
    fn single(source: String, compiled: RhaiAst) -> Self {
        Self {
            sources: source,
            compiled: Some(compiled),
        }
    }

    fn is_empty(&self) -> bool {
        self.sources.trim().is_empty()
    }

    fn source(&self) -> &str {
        self.sources.trim()
    }

    fn compiled(&self) -> Option<&RhaiAst> {
        self.compiled.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FilePatternRange {
    start: usize,
    end_exclusive: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct FilePatternNeedle(String);
impl FilePatternNeedle {
    fn from_text(text: &str) -> Option<Self> {
        let normalized = normalize_for_match(text);
        if normalized.is_empty() {
            return None;
        }
        Some(Self(normalized))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone)]
struct NormalizedAsmLine {
    stmt_index: usize,
    line: String,
}

#[derive(Debug, Clone, Copy)]
struct FilePatternScriptContext<'a> {
    source: &'a str,
    ast_debug: &'a str,
    ir_debug: &'a str,
    asm_debug: &'a str,
    statement_count: i64,
    asm_count: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileFingerprint {
    modified: Option<SystemTime>,
    len: u64,
}

#[derive(Debug, Clone)]
struct CachedRule {
    fingerprint: FileFingerprint,
    rule: FilePatternRule,
}

thread_local! {
    static FILE_PATTERN_CACHE: RefCell<HashMap<String, CachedRule>> = RefCell::new(HashMap::new());
    static RHAI_ENGINE: RefCell<Engine> = RefCell::new(build_rhai_engine());
    static RHAI_SCRIPT_CACHE: RefCell<HashMap<String, RhaiAst>> = RefCell::new(HashMap::new());
}

pub(super) fn apply_patterns(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    patterns: &[AstPattern],
) -> Result<(), DecompileError> {
    let mut body;
    let ir_debug;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        ir_debug = format!("{:?}", function.ir.get_ir());
        body = std::mem::take(&mut function.body);
    }

    apply_patterns_in_statements(&mut body, patterns);

    let file_rules = load_file_pattern_rules(patterns)?;
    if !file_rules.is_empty() {
        apply_file_pattern_rules_recursive(&mut body, &file_rules, &ir_debug);
    }

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::PatternMatching);
    }

    Ok(())
}

fn apply_patterns_in_statements(stmts: &mut Vec<WrappedAstStatement>, patterns: &[AstPattern]) {
    for stmt in stmts.iter_mut() {
        apply_patterns_in_statement(stmt, patterns);
    }

    if pattern_enabled(patterns, "remove-empty-statements") {
        stmts.retain(|stmt| !matches!(&stmt.statement, AstStatement::Empty));
    }
}

fn apply_patterns_in_statement(stmt: &mut WrappedAstStatement, patterns: &[AstPattern]) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            apply_patterns_in_statements(branch_true, patterns);
            if let Some(branch_false) = branch_false {
                apply_patterns_in_statements(branch_false, patterns);
            }
            if pattern_enabled(patterns, "prune-empty-else") {
                let remove_else =
                    matches!(branch_false.as_ref(), Some(branch) if branch.is_empty());
                if remove_else {
                    *branch_false = None;
                }
            }
        }
        AstStatement::While(_, body) => {
            apply_patterns_in_statements(body, patterns);
        }
        AstStatement::For(init, _, update, body) => {
            apply_patterns_in_statement(init, patterns);
            apply_patterns_in_statement(update, patterns);
            apply_patterns_in_statements(body, patterns);
        }
        AstStatement::Block(body) => {
            apply_patterns_in_statements(body, patterns);
            if pattern_enabled(patterns, "collapse-empty-blocks") && body.is_empty() {
                stmt.statement = AstStatement::Empty;
            }
        }
        AstStatement::Declaration(_, _)
        | AstStatement::Assignment(_, _)
        | AstStatement::Return(_)
        | AstStatement::Call(_)
        | AstStatement::Label(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Ir(_)
        | AstStatement::Empty => {}
    }
}

fn load_file_pattern_rules(
    patterns: &[AstPattern],
) -> Result<Vec<FilePatternRule>, DecompileError> {
    let mut rules = Vec::new();
    for pattern in patterns {
        match &pattern.parsed {
            AstPatternParsed::File(rule) => {
                rules.push(rule.clone());
                continue;
            }
            AstPatternParsed::ParseError(err) => {
                return Err(err.clone().into());
            }
            AstPatternParsed::None => {}
        }
        let Some(path) = pattern_file_path(pattern) else {
            continue;
        };
        let rule = load_file_pattern_rule_cached(path)?;
        rules.push(rule);
    }
    Ok(rules)
}

fn pattern_file_path(pattern: &AstPattern) -> Option<&str> {
    if pattern.origin == AstPatternOrigin::File {
        if !pattern.pattern.trim().is_empty() {
            return Some(pattern.pattern.trim());
        }
        if !pattern.name.trim().is_empty() {
            return Some(pattern.name.trim());
        }
    }
    if pattern.pattern.trim().ends_with(".fb") {
        return Some(pattern.pattern.trim());
    }
    if pattern.name.trim().ends_with(".fb") {
        return Some(pattern.name.trim());
    }
    None
}

fn apply_file_pattern_rules_recursive(
    stmts: &mut Vec<WrappedAstStatement>,
    rules: &[FilePatternRule],
    ir_debug: &str,
) {
    for rule in rules {
        apply_single_file_rule(stmts, rule, ir_debug);
    }

    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, branch_true, branch_false) => {
                apply_file_pattern_rules_recursive(branch_true, rules, ir_debug);
                if let Some(branch_false) = branch_false {
                    apply_file_pattern_rules_recursive(branch_false, rules, ir_debug);
                }
            }
            AstStatement::While(_, body) => {
                apply_file_pattern_rules_recursive(body, rules, ir_debug);
            }
            AstStatement::For(init, _, update, body) => {
                let mut init_vec = vec![(**init).clone()];
                apply_file_pattern_rules_recursive(&mut init_vec, rules, ir_debug);
                if let Some(next_init) = init_vec.into_iter().next() {
                    **init = next_init;
                }

                let mut update_vec = vec![(**update).clone()];
                apply_file_pattern_rules_recursive(&mut update_vec, rules, ir_debug);
                if let Some(next_update) = update_vec.into_iter().next() {
                    **update = next_update;
                }

                apply_file_pattern_rules_recursive(body, rules, ir_debug);
            }
            AstStatement::Block(body) => {
                apply_file_pattern_rules_recursive(body, rules, ir_debug);
            }
            AstStatement::Declaration(_, _)
            | AstStatement::Assignment(_, _)
            | AstStatement::Return(_)
            | AstStatement::Call(_)
            | AstStatement::Label(_)
            | AstStatement::Goto(_)
            | AstStatement::Assembly(_)
            | AstStatement::Undefined
            | AstStatement::Exception(_)
            | AstStatement::Comment(_)
            | AstStatement::Ir(_)
            | AstStatement::Empty => {}
        }
    }
}

fn apply_single_file_rule(
    stmts: &mut Vec<WrappedAstStatement>,
    rule: &FilePatternRule,
    ir_debug: &str,
) {
    if stmts.is_empty() {
        return;
    }

    let ast_debug = collect_ast_debug(stmts);
    let ast_statements = collect_ast_statements(stmts);
    let ir_lines = collect_ir_lines(ir_debug);
    let asm_lines = collect_asm_lines(stmts);
    let asm_debug = asm_lines
        .iter()
        .map(|line| line.line.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let script_context = FilePatternScriptContext {
        source: &rule.source,
        ast_debug: &ast_debug,
        ir_debug,
        asm_debug: &asm_debug,
        statement_count: stmts.len() as i64,
        asm_count: asm_lines.len() as i64,
    };
    let matched = rule.in_blocks.iter().find_map(|in_group| {
        in_group.iter().find_map(|block| {
            match_if_block(
                block,
                &script_context,
                &asm_lines,
                &ast_statements,
                &ir_lines,
            )
        })
    });
    let Some(matched) = matched else {
        return;
    };

    for action in &rule.out_actions {
        match action {
            FilePatternOutAction::ReplaceAsm(replacement) => {
                apply_replace_asm(stmts, &matched, replacement);
            }
            FilePatternOutAction::ReplaceIr(replacement) => {
                apply_replace_ir(stmts, &matched, replacement);
            }
            FilePatternOutAction::ReplaceAst(replacement) => {
                apply_replace_ast(stmts, &matched, replacement);
            }
            FilePatternOutAction::Script(script) => {
                if !execute_do_script(script, &script_context) {
                    break;
                }
            }
            FilePatternOutAction::Log(level, msg) => match level {
                FilePatternLogLevel::Info => info!("Pattern `{}` matched: {}", rule.source, msg),
                FilePatternLogLevel::Warn => warn!("Pattern `{}` matched: {}", rule.source, msg),
                FilePatternLogLevel::Error => {
                    error!("Pattern `{}` matched: {}", rule.source, msg)
                }
                FilePatternLogLevel::Debug => {
                    debug!("Pattern `{}` matched: {}", rule.source, msg)
                }
                FilePatternLogLevel::Trace => {
                    trace!("Pattern `{}` matched: {}", rule.source, msg)
                }
            },
        }
    }
}

fn collect_ast_statements(stmts: &[WrappedAstStatement]) -> Vec<AstStatement> {
    stmts.iter().map(|stmt| stmt.statement.clone()).collect()
}

fn collect_ir_lines(ir_debug: &str) -> Vec<String> {
    ir_debug
        .lines()
        .map(normalize_for_match)
        .filter(|line| !line.is_empty())
        .collect()
}

fn collect_asm_lines(stmts: &[WrappedAstStatement]) -> Vec<NormalizedAsmLine> {
    let mut lines = Vec::new();
    let mut seen_ir_indices = HashSet::new();

    for (idx, stmt) in stmts.iter().enumerate() {
        if let AstStatement::Assembly(text) = &stmt.statement {
            let line = normalize_for_match(text);
            if !line.is_empty() {
                lines.push(NormalizedAsmLine {
                    stmt_index: idx,
                    line,
                });
            }
            continue;
        }

        let AstStatementOrigin::Ir(desc) = &stmt.origin else {
            if let AstStatement::Ir(ir_stmt) = &stmt.statement {
                let line = normalize_for_match(&format!("{}", ir_stmt));
                if !line.is_empty() {
                    lines.push(NormalizedAsmLine {
                        stmt_index: idx,
                        line,
                    });
                }
            }
            continue;
        };
        let ir_index = desc.descriptor().ir_index();
        if !seen_ir_indices.insert(ir_index) {
            continue;
        }
        let Some(instruction) = desc.ir().get_instructions().get(ir_index as usize) else {
            continue;
        };
        let line = normalize_for_match(&instruction.inner().to_string());
        if !line.is_empty() {
            lines.push(NormalizedAsmLine {
                stmt_index: idx,
                line,
            });
        }
    }

    lines
}

fn match_if_block(
    block: &FilePatternInBlock,
    script_context: &FilePatternScriptContext<'_>,
    asm_lines: &[NormalizedAsmLine],
    ast_statements: &[AstStatement],
    ir_lines: &[String],
) -> Option<FilePatternMatch> {
    let mut has_condition = false;
    let mut matched = FilePatternMatch::default();

    if !block.asm.is_empty() {
        has_condition = true;
        let asm_skip_range = block.skip_range.or(block.skip_asm_range);
        matched.asm_statement_range = Some(find_asm_match(asm_lines, &block.asm, asm_skip_range)?);
    }
    if !block.ast.is_empty() {
        has_condition = true;
        if !sequence_matches_ast(ast_statements, &block.ast, block.skip_ast_range) {
            return None;
        }
    }
    if !block.ir.is_empty() {
        has_condition = true;
        if !sequence_matches_ir(ir_lines, &block.ir, block.skip_ir_range) {
            return None;
        }
    }
    if !block.script_conditions.is_empty() {
        has_condition = true;
        if !evaluate_if_script(&block.script_conditions, script_context) {
            trace!(
                "Pattern `{}` if-script condition failed",
                script_context.source
            );
            return None;
        }
    }

    if has_condition { Some(matched) } else { None }
}

fn collect_ast_debug(stmts: &[WrappedAstStatement]) -> String {
    stmts
        .iter()
        .map(|stmt| format!("{:?}", stmt.statement))
        .collect::<Vec<_>>()
        .join("\n")
}

fn build_rhai_engine() -> Engine {
    let mut engine = Engine::new();
    engine.on_print(move |msg| {
        debug!("Pattern script print: {}", msg);
    });
    engine.on_debug(move |msg, src, pos| {
        trace!(
            "Pattern script debug: {} (src={:?}, pos={:?})",
            msg, src, pos
        );
    });
    engine.register_fn("info", |msg: &str| info!("Pattern script: {}", msg));
    engine.register_fn("warn", |msg: &str| warn!("Pattern script: {}", msg));
    engine.register_fn("error", |msg: &str| error!("Pattern script: {}", msg));
    engine.register_fn("debug", |msg: &str| debug!("Pattern script: {}", msg));
    engine.register_fn("trace", |msg: &str| trace!("Pattern script: {}", msg));
    engine
}

fn with_rhai_engine<T>(func: impl FnOnce(&Engine) -> T) -> T {
    RHAI_ENGINE.with(|engine| {
        let engine = engine.borrow();
        func(&engine)
    })
}

fn compiled_script(script: &str) -> Result<RhaiAst, String> {
    if let Some(compiled) = RHAI_SCRIPT_CACHE.with(|cache| cache.borrow().get(script).cloned()) {
        return Ok(compiled);
    }

    let compiled = with_rhai_engine(|engine| engine.compile(script))
        .map_err(|err| format!("failed to compile rhai script: {err}"))?;

    RHAI_SCRIPT_CACHE.with(|cache| {
        cache
            .borrow_mut()
            .entry(script.to_string())
            .or_insert_with(|| compiled.clone());
    });
    Ok(compiled)
}

fn build_rhai_scope(context: &FilePatternScriptContext<'_>) -> Scope<'static> {
    let mut scope = Scope::new();
    scope.push("source", context.source.to_string());
    scope.push("ast", context.ast_debug.to_string());
    scope.push("ir", context.ir_debug.to_string());
    scope.push("asm", context.asm_debug.to_string());
    scope.push("stmt_count", context.statement_count);
    scope.push("asm_count", context.asm_count);
    scope
}

fn evaluate_if_script(script: &FilePatternScript, context: &FilePatternScriptContext<'_>) -> bool {
    let source = script.source();
    let Some(compiled) = script.compiled() else {
        error!(
            "Pattern `{}` if script has no compiled AST: {}",
            context.source, source
        );
        return false;
    };
    let mut scope = build_rhai_scope(context);
    match with_rhai_engine(|engine| engine.eval_ast_with_scope::<Dynamic>(&mut scope, compiled)) {
        Ok(value) => match value.try_cast::<bool>() {
            Some(result) => result,
            None => {
                error!(
                    "Pattern `{}` if script must return bool: {}",
                    context.source, source
                );
                false
            }
        },
        Err(err) => {
            error!(
                "Pattern `{}` if script failed: {} ({})",
                context.source, source, err
            );
            false
        }
    }
}

fn execute_do_script(script: &FilePatternScript, context: &FilePatternScriptContext<'_>) -> bool {
    let source = script.source();
    let Some(compiled) = script.compiled() else {
        error!(
            "Pattern `{}` do script has no compiled AST: {}",
            context.source, source
        );
        return false;
    };
    let mut scope = build_rhai_scope(context);
    match with_rhai_engine(|engine| engine.eval_ast_with_scope::<Dynamic>(&mut scope, compiled)) {
        Ok(value) => {
            if let Some(continue_actions) = value.try_cast::<bool>() {
                trace!(
                    "Pattern `{}` do-script returned bool={}",
                    context.source, continue_actions
                );
                if !continue_actions {
                    return false;
                }
            }
        }
        Err(err) => {
            error!(
                "Pattern `{}` do script failed: {} ({})",
                context.source, source, err
            );
            return false;
        }
    }
    true
}

fn find_asm_match(
    asm_lines: &[NormalizedAsmLine],
    asm: &FilePatternAsmData,
    skip_range: Option<FilePatternRange>,
) -> Option<(usize, usize)> {
    if asm_lines.is_empty() {
        return None;
    }

    let expected_asm = normalize_for_match(asm.as_match_text());
    let expected_ir = match &asm.statement {
        AstStatement::Ir(ir_stmt) => normalize_for_match(&format!("{}", ir_stmt)),
        _ => String::new(),
    };
    if expected_asm.is_empty() && expected_ir.is_empty() {
        return None;
    }

    let start = skip_range.map_or(0usize, |range| range.start.min(asm_lines.len()));
    let end_exclusive = skip_range.map_or(asm_lines.len(), |range| {
        range.end_exclusive.min(asm_lines.len())
    });

    if end_exclusive <= start {
        return None;
    }

    for cursor in start..end_exclusive {
        let is_match = (!expected_asm.is_empty() && asm_lines[cursor].line.contains(&expected_asm))
            || (!expected_ir.is_empty() && asm_lines[cursor].line.contains(&expected_ir));
        if is_match {
            let start_stmt = asm_lines[cursor].stmt_index;
            let end_stmt = asm_lines[cursor].stmt_index;
            return Some((start_stmt, end_stmt));
        }
    }

    None
}

fn sequence_matches_ast(
    statements: &[AstStatement],
    data: &FilePatternAstData,
    skip_range: Option<FilePatternRange>,
) -> bool {
    if statements.is_empty() {
        return false;
    }

    let expected = normalize_for_match(&format!("{:?}", data.statement));
    if expected.is_empty() {
        return false;
    }

    let start = skip_range.map_or(0usize, |range| range.start.min(statements.len()));
    let end_exclusive = skip_range.map_or(statements.len(), |range| {
        range.end_exclusive.min(statements.len())
    });
    if end_exclusive <= start {
        return false;
    }

    for statement in &statements[start..end_exclusive] {
        let line = normalize_for_match(&format!("{:?}", statement));
        if line.contains(&expected) {
            return true;
        }
    }
    false
}

fn sequence_matches_ir(
    lines: &[String],
    data: &FilePatternIrData,
    skip_range: Option<FilePatternRange>,
) -> bool {
    if lines.is_empty() {
        return false;
    }

    let expected = normalize_for_match(&format!("{}", data.statement));
    if expected.is_empty() {
        return false;
    }

    let start = skip_range.map_or(0usize, |range| range.start.min(lines.len()));
    let end_exclusive =
        skip_range.map_or(lines.len(), |range| range.end_exclusive.min(lines.len()));
    if end_exclusive <= start {
        return false;
    }

    for line in &lines[start..end_exclusive] {
        if line.contains(&expected) {
            return true;
        }
    }
    false
}

fn normalize_for_match(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn apply_replace_asm(
    stmts: &mut [WrappedAstStatement],
    matched: &FilePatternMatch,
    replacement: &FilePatternAsmData,
) {
    if replacement.is_empty() {
        return;
    }

    if let Some((start, end)) = matched.asm_statement_range {
        if let Some(first) = stmts.get_mut(start) {
            first.statement = replacement.statement.clone();
        }
        if end > start {
            for idx in (start + 1)..=end {
                if let Some(stmt) = stmts.get_mut(idx) {
                    stmt.statement = AstStatement::Empty;
                }
            }
        }
        return;
    }

    if let Some((idx, _)) = stmts.iter().enumerate().find(|(_, stmt)| {
        matches!(
            stmt.statement,
            AstStatement::Assembly(_) | AstStatement::Ir(_)
        )
    }) {
        if let Some(stmt) = stmts.get_mut(idx) {
            stmt.statement = replacement.statement.clone();
        }
    }
}

fn apply_replace_ir(
    stmts: &mut [WrappedAstStatement],
    matched: &FilePatternMatch,
    replacement: &FilePatternIrReplacement,
) {
    if let Some(ir_stmt) = &replacement.statement {
        if let Some(stmt) = stmts
            .iter_mut()
            .find(|stmt| matches!(stmt.statement, AstStatement::Ir(_)))
        {
            stmt.statement = AstStatement::Ir(Box::new(ir_stmt.clone()));
            return;
        }
    }

    if let Some((start, _)) = matched.asm_statement_range {
        if let Some(stmt) = stmts.get_mut(start) {
            stmt.statement = AstStatement::Comment(replacement.fallback_comment.clone());
            return;
        }
    }

    if let Some(stmt) = stmts.first_mut() {
        stmt.statement = AstStatement::Comment(replacement.fallback_comment.clone());
    }
}

fn apply_replace_ast(
    stmts: &mut [WrappedAstStatement],
    matched: &FilePatternMatch,
    replacement: &AstStatement,
) {
    if let Some(stmt) = stmts.iter_mut().find(|stmt| {
        matches!(
            stmt.statement,
            AstStatement::Comment(_) | AstStatement::Empty
        )
    }) {
        stmt.statement = replacement.clone();
        return;
    }

    if let Some((start, _)) = matched.asm_statement_range {
        if let Some(stmt) = stmts.get_mut(start) {
            stmt.statement = replacement.clone();
            return;
        }
    }

    if let Some(stmt) = stmts.first_mut() {
        stmt.statement = replacement.clone();
    }
}

fn parse_asm_replacement(value: &str) -> Option<FilePatternAsmData> {
    let text = value.trim();
    if text.is_empty() {
        return None;
    }
    let normalized = text.strip_prefix("asm ").unwrap_or(text).trim();
    FilePatternAsmData::from_text(normalized)
}

fn parse_asm_statement(text: &str) -> Option<IrStatement> {
    let mut raw = text.trim();
    if raw.is_empty() {
        return None;
    }
    raw = raw.strip_prefix("asm ").unwrap_or(raw).trim();
    raw = raw.split(';').next().unwrap_or(raw).trim();
    if raw.is_empty() {
        return None;
    }

    let mut parts = raw.splitn(2, |ch: char| ch.is_whitespace());
    let mnemonic = parts.next()?.trim();
    let operands = parts.next().unwrap_or_default().trim();

    let statement = iceball::parse_statement(iceball::Architecture::X64, mnemonic).ok()?;
    let arguments = parse_asm_arguments(operands)?;
    let instruction = crate::core::Instruction {
        address: 0,
        inner: iceball::Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: None,
        },
    };
    let statements = crate::arch::x86_64::instruction_analyze::create_ir_statement(&instruction)?;
    statements.first().cloned()
}

fn parse_asm_arguments(text: &str) -> Option<Vec<iceball::Argument>> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Some(Vec::new());
    }

    let mut arguments = Vec::new();
    for operand in split_top_level(trimmed, ',') {
        let operand = operand.trim();
        if operand.is_empty() {
            continue;
        }
        arguments.push(parse_asm_argument_lossy(operand)?);
    }
    Some(arguments)
}

fn parse_asm_argument_lossy(op: &str) -> Option<iceball::Argument> {
    if let Ok(value) = op.parse::<u64>() {
        return Some(iceball::Argument::Constant(value));
    }

    if let Some(arg) = parse_asm_argument_safe(op) {
        return Some(arg);
    }

    let lowered = op.to_ascii_lowercase();
    let stripped = [
        "byte ptr ",
        "word ptr ",
        "dword ptr ",
        "qword ptr ",
        "xmmword ptr ",
        "ymmword ptr ",
        "zmmword ptr ",
        "ptr ",
    ]
    .iter()
    .find_map(|prefix| lowered.strip_prefix(prefix).map(str::trim));

    stripped.and_then(parse_asm_argument_safe)
}

fn parse_asm_argument_safe(op: &str) -> Option<iceball::Argument> {
    fn try_parse(op: &str) -> Option<iceball::Argument> {
        std::panic::catch_unwind(|| iceball::parse_argument(iceball::Architecture::X64, op))
            .ok()
            .and_then(Result::ok)
    }

    try_parse(op)
        .or_else(|| try_parse(&op.to_ascii_uppercase()))
        .or_else(|| try_parse(&op.to_ascii_lowercase()))
}

fn parse_ir_replacement(replacement: &str) -> FilePatternIrReplacement {
    let statement = parse_ir_statement(replacement);
    let fallback_comment = format!("IR: {}", replacement.trim());
    FilePatternIrReplacement {
        statement,
        fallback_comment,
    }
}

fn parse_ir_statement(replacement: &str) -> Option<IrStatement> {
    let text = replacement.trim();
    if text.is_empty() {
        return None;
    }

    let normalized = normalize_for_match(text);
    if normalized == "undefined" {
        return Some(IrStatement::Undefined);
    }
    if normalized == "halt" {
        return Some(IrStatement::Halt);
    }

    if let Some(rest) = text.strip_prefix("exception ") {
        return Some(IrStatement::Exception(leak_static_str(rest.trim())));
    }
    if let Some(rest) = text.strip_prefix("jmp ") {
        return Some(IrStatement::Jump {
            target: parse_ir_data(rest),
        });
    }
    if let Some(rest) = text.strip_prefix("call ") {
        return Some(IrStatement::JumpByCall {
            target: parse_ir_data(rest),
        });
    }
    if text.starts_with("if ") {
        return parse_ir_condition_statement(text);
    }
    if let Some(rest) = text.strip_prefix("type ") {
        return parse_ir_type_special(rest).map(|special| {
            IrStatement::Special(IrStatementSpecial::TypeSpecified {
                location: special.location,
                size: special.size,
                data_type: special.data_type,
            })
        });
    }
    if let Some(rest) = text.strip_prefix("calc_flags ") {
        return parse_ir_calc_flags(rest);
    }
    if let Some(rest) = text.strip_prefix("assert ") {
        return Some(IrStatement::Special(IrStatementSpecial::Assertion {
            condition: parse_ir_data(strip_wrapping_parens(rest.trim())),
        }));
    }
    if let Some(assignment) = parse_ir_assignment_statement(text) {
        return Some(assignment);
    }

    debug!("Could not parse IR statement pattern: {}", replacement);
    None
}

#[derive(Debug)]
struct ParsedTypeSpecial {
    location: Aos<IrData>,
    size: IrAccessSize,
    data_type: DataType,
}

fn parse_ir_assignment_statement(text: &str) -> Option<IrStatement> {
    let (to_text, rhs) = text.split_once(" = ")?;
    let rhs = rhs.trim();
    if !rhs.starts_with('(') {
        return None;
    }
    let close = find_matching_delimiter(rhs, 0, '(', ')')?;
    let size_text = &rhs[1..close];
    let from_text = rhs[close + 1..].trim();
    Some(IrStatement::Assignment {
        from: parse_ir_data(from_text),
        to: parse_ir_data(to_text.trim()),
        size: parse_ir_access_size(size_text.trim()),
    })
}

fn parse_ir_type_special(text: &str) -> Option<ParsedTypeSpecial> {
    let (location_text, rhs) = text.split_once(" = ")?;
    let rhs = rhs.trim();
    let data_type_tokens = [
        ("*c", DataType::StringPointer),
        ("u", DataType::Unknown),
        ("b", DataType::Bool),
        ("i", DataType::Int),
        ("f", DataType::Float),
        ("c", DataType::Char),
        ("*", DataType::Address),
    ];

    for (token, data_type) in data_type_tokens {
        if let Some(size_text) = rhs.strip_suffix(token) {
            return Some(ParsedTypeSpecial {
                location: parse_ir_data(location_text.trim()),
                size: parse_ir_access_size(size_text.trim()),
                data_type,
            });
        }
    }
    None
}

fn parse_ir_calc_flags(text: &str) -> Option<IrStatement> {
    let trimmed = text.trim();
    let open_list = trimmed.find('[')?;
    let close_list = find_matching_delimiter(trimmed, open_list, '[', ']')?;
    let flags_text = &trimmed[open_list + 1..close_list];
    let after_flags = trimmed[close_list + 1..].trim();
    let operation_text = strip_wrapping_parens(after_flags);

    let flags = split_top_level(flags_text, ',')
        .into_iter()
        .map(parse_ir_data)
        .collect::<Vec<_>>();

    Some(IrStatement::Special(
        IrStatementSpecial::CalcFlagsAutomatically {
            operation: parse_ir_data(operation_text),
            size: IrAccessSize::ArchitectureSize,
            flags,
        },
    ))
}

fn parse_ir_condition_statement(text: &str) -> Option<IrStatement> {
    let after_if = text.strip_prefix("if ")?.trim_start();
    let open_true = after_if.find('{')?;
    let condition_text = after_if[..open_true].trim();
    let (true_block, rest) = parse_braced_block(&after_if[open_true..])?;
    let rest = rest.trim_start();

    let false_branch = if let Some(else_part) = rest.strip_prefix("else") {
        let (false_block, _) = parse_braced_block(else_part.trim_start())?;
        parse_ir_statement_sequence(&false_block)
    } else {
        Vec::new()
    };

    Some(IrStatement::Condition {
        condition: parse_ir_data(condition_text),
        true_branch: parse_ir_statement_sequence(&true_block).into_boxed_slice(),
        false_branch: false_branch.into_boxed_slice(),
    })
}

fn parse_ir_statement_sequence(text: &str) -> Vec<IrStatement> {
    split_top_level(text, ';')
        .into_iter()
        .filter_map(|segment| {
            let segment = segment.trim();
            if segment.is_empty() {
                return None;
            }
            Some(parse_ir_statement(segment).unwrap_or(IrStatement::Undefined))
        })
        .collect()
}

fn parse_ir_access_size(text: &str) -> IrAccessSize {
    let trimmed = text.trim();
    if trimmed.eq_ignore_ascii_case("arch_len") {
        return IrAccessSize::ArchitectureSize;
    }
    if trimmed.eq_ignore_ascii_case("unlimited") {
        return IrAccessSize::Unlimited;
    }
    if let Some(inner) = trimmed
        .strip_prefix("sizeof(")
        .and_then(|value| value.strip_suffix(')'))
    {
        return IrAccessSize::RelativeWith(parse_ir_data(inner.trim()));
    }
    if let Some(inner) = trimmed.strip_suffix("bit") {
        return IrAccessSize::ResultOfBit(parse_ir_data(inner.trim()));
    }
    if let Some(inner) = trimmed.strip_suffix("byte") {
        return IrAccessSize::ResultOfByte(parse_ir_data(inner.trim()));
    }
    IrAccessSize::ArchitectureSize
}

fn parse_ir_data(text: &str) -> Aos<IrData> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return ir_unknown_data();
    }

    if let Some(inner) = parse_outer_wrapped(trimmed, '(', ')') {
        return parse_ir_data(inner);
    }

    if let Some(intrinsic) = parse_ir_intrinsic(trimmed) {
        return IrData::Intrinsic(intrinsic).into();
    }
    if let Some(operation) = parse_ir_operation(trimmed) {
        return IrData::Operation(operation).into();
    }
    if let Some(constant) = parse_ir_constant(trimmed) {
        return IrData::Constant(constant).into();
    }
    if let Some(operand) = parse_ir_operand(trimmed) {
        return IrData::Operand(operand).into();
    }
    if let Some(register) = try_parse_register(trimmed) {
        return register;
    }

    ir_unknown_data()
}

fn parse_ir_operation(text: &str) -> Option<IrDataOperation> {
    let unary_ops = [
        ("sign_extend ", IrUnaryOperator::SignExtend),
        ("zero_extend ", IrUnaryOperator::ZeroExtend),
        ("! ", IrUnaryOperator::Not),
        ("- ", IrUnaryOperator::Negation),
    ];
    for (prefix, operator) in unary_ops {
        if let Some(arg_text) = text.strip_prefix(prefix) {
            return Some(IrDataOperation::Unary {
                operator,
                arg: parse_ir_data(arg_text),
            });
        }
    }

    if let Some((arg1, operator, arg2)) = parse_ir_binary_operation_parts(text) {
        return Some(IrDataOperation::Binary {
            operator,
            arg1: parse_ir_data(arg1),
            arg2: parse_ir_data(arg2),
        });
    }
    None
}

fn parse_ir_binary_operation_parts(text: &str) -> Option<(&str, IrBinaryOperator, &str)> {
    if let Some((left, right)) = split_once_top_level(text, " == ") {
        let (size, rhs) = parse_operator_sized_rhs(right)?;
        return Some((left, IrBinaryOperator::Equal(size), rhs));
    }
    if let Some((left, right)) = split_once_top_level(text, " <= ") {
        let (size, rhs) = parse_operator_sized_rhs(right)?;
        return Some((left, IrBinaryOperator::SignedLessOrEqual(size), rhs));
    }
    if let Some((left, right)) = split_once_top_level(text, " < ") {
        let (size, rhs) = parse_operator_sized_rhs(right)?;
        return Some((left, IrBinaryOperator::SignedLess(size), rhs));
    }

    let binary_tokens = [
        (" << ", IrBinaryOperator::Shl),
        (" >> ", IrBinaryOperator::Shr),
        (" sar ", IrBinaryOperator::Sar),
        (" div ", IrBinaryOperator::UnsignedDiv),
        (" rem ", IrBinaryOperator::UnsignedRem),
        (" & ", IrBinaryOperator::And),
        (" | ", IrBinaryOperator::Or),
        (" ^ ", IrBinaryOperator::Xor),
        (" + ", IrBinaryOperator::Add),
        (" - ", IrBinaryOperator::Sub),
        (" * ", IrBinaryOperator::Mul),
        (" / ", IrBinaryOperator::SignedDiv),
        (" % ", IrBinaryOperator::SignedRem),
    ];
    for (token, operator) in binary_tokens {
        if let Some((left, right)) = split_once_top_level(text, token) {
            return Some((left, operator, right));
        }
    }
    None
}

fn parse_operator_sized_rhs(text: &str) -> Option<(IrAccessSize, &str)> {
    let rhs = text.trim_start();
    if !rhs.starts_with('(') {
        return None;
    }
    let close = find_matching_delimiter(rhs, 0, '(', ')')?;
    let size = parse_ir_access_size(&rhs[1..close]);
    Some((size, rhs[close + 1..].trim_start()))
}

fn parse_ir_intrinsic(text: &str) -> Option<IrIntrinsic> {
    if text.eq_ignore_ascii_case("unknown") {
        return Some(IrIntrinsic::Unknown);
    }
    if text.eq_ignore_ascii_case("undefined") {
        return Some(IrIntrinsic::Undefined);
    }
    if text.eq_ignore_ascii_case("arch_byte_size") {
        return Some(IrIntrinsic::ArchitectureByteSize);
    }
    if text.eq_ignore_ascii_case("arch_bit_size") {
        return Some(IrIntrinsic::ArchitectureBitSize);
    }
    if text.eq_ignore_ascii_case("arch_bit_per_byte") {
        return Some(IrIntrinsic::ArchitectureBitPerByte);
    }
    if text.eq_ignore_ascii_case("instruction_byte_size") {
        return Some(IrIntrinsic::InstructionByteSize);
    }

    if let Some(size_text) = parse_function_arg(text, "signed_max") {
        return Some(IrIntrinsic::SignedMax(parse_ir_access_size(size_text)));
    }
    if let Some(size_text) = parse_function_arg(text, "signed_min") {
        return Some(IrIntrinsic::SignedMin(parse_ir_access_size(size_text)));
    }
    if let Some(size_text) = parse_function_arg(text, "unsigned_max") {
        return Some(IrIntrinsic::UnsignedMax(parse_ir_access_size(size_text)));
    }
    if let Some(size_text) = parse_function_arg(text, "unsigned_min") {
        return Some(IrIntrinsic::UnsignedMin(parse_ir_access_size(size_text)));
    }
    if let Some(size_text) = parse_function_arg(text, "bit_ones") {
        return Some(IrIntrinsic::BitOnes(parse_ir_access_size(size_text)));
    }
    if let Some(size_text) = parse_function_arg(text, "bit_zeros") {
        return Some(IrIntrinsic::BitZeros(parse_ir_access_size(size_text)));
    }
    if let Some(data_text) = parse_function_arg(text, "byte_size_of") {
        return Some(IrIntrinsic::ByteSizeOf(parse_ir_data(data_text)));
    }
    if let Some(data_text) = parse_function_arg(text, "bit_size_of") {
        return Some(IrIntrinsic::BitSizeOf(parse_ir_data(data_text)));
    }
    if let Some(arg_text) = parse_function_arg(text, "sized") {
        let args = split_top_level(arg_text, ',');
        if args.len() == 2 {
            return Some(IrIntrinsic::Sized(
                parse_ir_data(args[0]),
                parse_ir_access_size(args[1]),
            ));
        }
    }
    if let Some(value_text) = parse_function_arg(text, "operand_exists") {
        if let Ok(raw) = value_text.trim().parse::<u8>() {
            if let Some(value) = NonZeroU8::new(raw) {
                return Some(IrIntrinsic::OperandExists(value));
            }
        }
    }
    if let Some(value_text) = parse_function_arg(text, "arch_byte_size_condition") {
        return Some(IrIntrinsic::ArchitectureByteSizeCondition(
            parse_num_condition(value_text),
        ));
    }

    None
}

fn parse_num_condition(text: &str) -> crate::ir::data::NumCondition {
    let normalized = normalize_for_match(text);
    let parse_u16 = |value: &str| value.trim().parse::<u16>().ok();

    for (token, make) in [
        (
            " not in ",
            crate::ir::data::NumCondition::ExcludesRange as fn(u16, u16) -> _,
        ),
        (
            " in ",
            crate::ir::data::NumCondition::RangeInclusive as fn(u16, u16) -> _,
        ),
    ] {
        if let Some((_, rhs)) = normalized.split_once(token) {
            let rhs = rhs.trim();
            if let Some(inner) = rhs.strip_prefix('[').and_then(|v| v.strip_suffix(']')) {
                if let Some((a, b)) = inner.split_once("..") {
                    if let (Some(a), Some(b)) = (parse_u16(a), parse_u16(b)) {
                        return make(a, b);
                    }
                }
            }
        }
    }

    let comparisons = [
        (
            ">=",
            crate::ir::data::NumCondition::HigherOrEqual as fn(u16) -> _,
        ),
        (
            "<=",
            crate::ir::data::NumCondition::LowerOrEqual as fn(u16) -> _,
        ),
        (
            "!=",
            crate::ir::data::NumCondition::NotEqual as fn(u16) -> _,
        ),
        ("==", crate::ir::data::NumCondition::Equal as fn(u16) -> _),
        (">", crate::ir::data::NumCondition::Higher as fn(u16) -> _),
        ("<", crate::ir::data::NumCondition::Lower as fn(u16) -> _),
    ];
    for (token, make) in comparisons {
        if let Some((_, rhs)) = normalized.split_once(token) {
            if let Some(value) = parse_u16(rhs) {
                return make(value);
            }
        }
    }
    crate::ir::data::NumCondition::Equal(0)
}

fn parse_ir_constant(text: &str) -> Option<usize> {
    let trimmed = text.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return usize::from_str_radix(hex, 16).ok();
    }
    trimmed.parse::<usize>().ok()
}

fn parse_ir_operand(text: &str) -> Option<NonZeroU8> {
    let trimmed = text.trim();
    let raw = trimmed.strip_prefix('o')?.parse::<u8>().ok()?;
    NonZeroU8::new(raw)
}

fn try_parse_register(text: &str) -> Option<Aos<IrData>> {
    let candidate = text.trim();
    if candidate.is_empty() {
        return None;
    }
    if !candidate
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    {
        return None;
    }
    std::panic::catch_unwind(AssertUnwindSafe(|| {
        crate::arch::x86_64::str_to_x64_register(candidate)
    }))
    .ok()
}

fn parse_function_arg<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let prefix = format!("{name}(");
    let candidate = text.trim();
    if !candidate.starts_with(&prefix) || !candidate.ends_with(')') {
        return None;
    }
    let inner = &candidate[prefix.len()..candidate.len() - 1];
    Some(inner.trim())
}

fn split_once_top_level<'a>(text: &'a str, token: &str) -> Option<(&'a str, &'a str)> {
    let mut paren = 0usize;
    let mut bracket = 0usize;
    let mut brace = 0usize;
    for (idx, ch) in text.char_indices() {
        match ch {
            '(' => paren += 1,
            ')' => paren = paren.saturating_sub(1),
            '[' => bracket += 1,
            ']' => bracket = bracket.saturating_sub(1),
            '{' => brace += 1,
            '}' => brace = brace.saturating_sub(1),
            _ => {}
        }
        if paren == 0 && bracket == 0 && brace == 0 && text[idx..].starts_with(token) {
            let left = text[..idx].trim_end();
            let right = text[idx + token.len()..].trim_start();
            return Some((left, right));
        }
    }
    None
}

fn split_top_level(text: &str, delimiter: char) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0usize;
    let mut paren = 0usize;
    let mut bracket = 0usize;
    let mut brace = 0usize;
    for (idx, ch) in text.char_indices() {
        match ch {
            '(' => paren += 1,
            ')' => paren = paren.saturating_sub(1),
            '[' => bracket += 1,
            ']' => bracket = bracket.saturating_sub(1),
            '{' => brace += 1,
            '}' => brace = brace.saturating_sub(1),
            _ => {}
        }
        if ch == delimiter && paren == 0 && bracket == 0 && brace == 0 {
            out.push(text[start..idx].trim());
            start = idx + ch.len_utf8();
        }
    }
    out.push(text[start..].trim());
    out
}

fn parse_braced_block(text: &str) -> Option<(String, &str)> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with('{') {
        return None;
    }
    let close = find_matching_delimiter(trimmed, 0, '{', '}')?;
    let inner = trimmed[1..close].to_string();
    let rest = &trimmed[close + 1..];
    Some((inner, rest))
}

fn find_matching_delimiter(
    text: &str,
    open_index: usize,
    open_delim: char,
    close_delim: char,
) -> Option<usize> {
    let mut depth = 0usize;
    for (idx, ch) in text.char_indices().skip(open_index) {
        if ch == open_delim {
            depth += 1;
        } else if ch == close_delim {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(idx);
            }
        }
    }
    None
}

fn parse_outer_wrapped(text: &str, open: char, close: char) -> Option<&str> {
    if !text.starts_with(open) || !text.ends_with(close) {
        return None;
    }
    let close_idx = find_matching_delimiter(text, 0, open, close)?;
    if close_idx + close.len_utf8() != text.len() {
        return None;
    }
    Some(text[open.len_utf8()..close_idx].trim())
}

fn strip_wrapping_parens(text: &str) -> &str {
    parse_outer_wrapped(text, '(', ')').unwrap_or(text).trim()
}

fn ir_unknown_data() -> Aos<IrData> {
    IrData::Intrinsic(IrIntrinsic::Unknown).into()
}

fn leak_static_str(text: &str) -> &'static str {
    Box::leak(text.to_string().into_boxed_str())
}

fn parse_ast_replacement(replacement: &str) -> AstStatement {
    let text = replacement.trim();
    if text.eq_ignore_ascii_case("empty") {
        AstStatement::Empty
    } else if text.eq_ignore_ascii_case("undefined") {
        AstStatement::Undefined
    } else if text.eq_ignore_ascii_case("return") {
        AstStatement::Return(None)
    } else if let Some(content) = text.strip_prefix("comment ") {
        AstStatement::Comment(content.trim().to_string())
    } else if let Some(content) = text.strip_prefix("asm ") {
        parse_asm_statement(content.trim())
            .map(|stmt| AstStatement::Ir(Box::new(stmt)))
            .unwrap_or_else(|| AstStatement::Comment(text.to_string()))
    } else if let Some(content) = text.strip_prefix("ir ") {
        parse_ir_statement(content.trim())
            .map(|stmt| AstStatement::Ir(Box::new(stmt)))
            .unwrap_or_else(|| AstStatement::Comment(text.to_string()))
    } else {
        AstStatement::Comment(text.to_string())
    }
}

fn load_file_pattern_rule_cached(path: &str) -> Result<FilePatternRule, String> {
    let fingerprint = fingerprint(path)?;
    if let Some(cached) = FILE_PATTERN_CACHE.with(|cache| cache.borrow().get(path).cloned()) {
        if cached.fingerprint == fingerprint {
            return Ok(cached.rule);
        }
    }

    let rule = load_file_pattern_rule_uncached(path)?;
    FILE_PATTERN_CACHE.with(|cache| {
        cache.borrow_mut().insert(
            path.to_string(),
            CachedRule {
                fingerprint,
                rule: rule.clone(),
            },
        );
    });

    Ok(rule)
}

fn load_file_pattern_rule_uncached(path: &str) -> Result<FilePatternRule, String> {
    let content = fs::read_to_string(path)
        .map_err(|err| format!("failed to read pattern file `{path}`: {err}"))?;
    parse_pattern_file(path, &content)
}

fn fingerprint(path: &str) -> Result<FileFingerprint, String> {
    let metadata =
        fs::metadata(path).map_err(|err| format!("failed to read metadata for `{path}`: {err}"))?;
    Ok(FileFingerprint {
        modified: metadata.modified().ok(),
        len: metadata.len(),
    })
}

fn parse_pattern_file(path: &str, content: &str) -> Result<FilePatternRule, String> {
    let mut rule = FilePatternRule {
        source: path.to_string(),
        ..Default::default()
    };

    enum Section {
        None,
        If,
        Do,
    }

    let lines: Vec<&str> = content.lines().collect();
    let mut idx = 0usize;
    let mut section = Section::None;
    let mut current_in_blocks: Vec<FilePatternInBlock> = vec![FilePatternInBlock::default()];
    let mut has_current_in = false;

    while idx < lines.len() {
        let raw_line = lines[idx];
        idx += 1;
        let line = strip_inline_comment(raw_line);
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with("if:") {
            if has_current_in {
                let flushed = std::mem::take(&mut current_in_blocks);
                rule.in_blocks.push(flushed);
                current_in_blocks = vec![FilePatternInBlock::default()];
            }
            has_current_in = true;
            section = Section::If;
            continue;
        }

        if trimmed.starts_with("do:") {
            if has_current_in {
                let flushed = std::mem::take(&mut current_in_blocks);
                rule.in_blocks.push(flushed);
                current_in_blocks = vec![FilePatternInBlock::default()];
                has_current_in = false;
            }
            section = Section::Do;
            continue;
        }

        match section {
            Section::If => {
                if line.trim_start().starts_with("asm ") {
                    let value = parse_multiline_value(line.trim_start(), "asm ", &lines, &mut idx)?;
                    let sequence = split_pattern_sequence_raw(&value, true);
                    let sequence = sequence
                        .iter()
                        .filter_map(|item| FilePatternAsmData::from_text(item))
                        .collect::<Vec<_>>();
                    expand_in_blocks_for_asm(&mut current_in_blocks, &sequence);
                } else if line.trim_start().starts_with("ast ") {
                    let value = parse_multiline_value(line.trim_start(), "ast ", &lines, &mut idx)?;
                    let sequence = split_pattern_sequence_raw(&value, false);
                    let sequence = sequence
                        .iter()
                        .filter_map(|item| FilePatternAstData::from_text(item))
                        .collect::<Vec<_>>();
                    expand_in_blocks_for_ast(&mut current_in_blocks, &sequence);
                } else if line.trim_start().starts_with("ir ") {
                    let value = parse_multiline_value(line.trim_start(), "ir ", &lines, &mut idx)?;
                    let sequence = split_pattern_sequence_raw(&value, false);
                    let sequence = sequence
                        .iter()
                        .filter_map(|item| FilePatternIrData::from_text(item))
                        .collect::<Vec<_>>();
                    expand_in_blocks_for_ir(&mut current_in_blocks, &sequence);
                } else if line.trim_start().starts_with("script ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "script ", &lines, &mut idx)?;
                    let script = parse_rhai_script(&value)?;
                    expand_in_blocks_for_script(&mut current_in_blocks, &script);
                } else if line.trim_start().starts_with("skip_range ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip_range ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.skip_range = Some(range);
                    });
                } else if line.trim_start().starts_with("skip_asm_range ") {
                    let value = parse_multiline_value(
                        line.trim_start(),
                        "skip_asm_range ",
                        &lines,
                        &mut idx,
                    )?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.skip_asm_range = Some(range);
                    });
                } else if line.trim_start().starts_with("skip asm ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip asm ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.skip_asm_range = Some(range);
                    });
                } else if line.trim_start().starts_with("skip_ast_range ") {
                    let value = parse_multiline_value(
                        line.trim_start(),
                        "skip_ast_range ",
                        &lines,
                        &mut idx,
                    )?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.skip_ast_range = Some(range);
                    });
                } else if line.trim_start().starts_with("skip ast ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip ast ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.skip_ast_range = Some(range);
                    });
                } else if line.trim_start().starts_with("skip_ir_range ") {
                    let value = parse_multiline_value(
                        line.trim_start(),
                        "skip_ir_range ",
                        &lines,
                        &mut idx,
                    )?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.skip_ir_range = Some(range);
                    });
                } else if line.trim_start().starts_with("skip ir ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip ir ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.skip_ir_range = Some(range);
                    });
                } else {
                    return Err(format!(
                        "unknown `if` directive in pattern `{path}`: {}",
                        line.trim()
                    ));
                }
            }
            Section::Do => {
                let trimmed = line.trim_start();
                if trimmed.starts_with("asm ") {
                    let value = parse_multiline_value(trimmed, "asm ", &lines, &mut idx)?;
                    let replacement = parse_asm_replacement(&value).ok_or_else(|| {
                        format!(
                            "invalid asm replacement in pattern `{path}`: {}",
                            value.trim()
                        )
                    })?;
                    rule.out_actions
                        .push(FilePatternOutAction::ReplaceAsm(replacement));
                } else if trimmed.starts_with("ir ") {
                    let value = parse_multiline_value(trimmed, "ir ", &lines, &mut idx)?;
                    rule.out_actions
                        .push(FilePatternOutAction::ReplaceIr(parse_ir_replacement(
                            &value,
                        )));
                } else if trimmed.starts_with("ast ") {
                    let value = parse_multiline_value(trimmed, "ast ", &lines, &mut idx)?;
                    rule.out_actions
                        .push(FilePatternOutAction::ReplaceAst(parse_ast_replacement(
                            &value,
                        )));
                } else if trimmed.starts_with("script ") {
                    let value = parse_multiline_value(trimmed, "script ", &lines, &mut idx)?;
                    rule.out_actions
                        .push(FilePatternOutAction::Script(parse_rhai_script(&value)?));
                } else if let Some(msg) = parse_log_action(trimmed, "info") {
                    rule.out_actions
                        .push(FilePatternOutAction::Log(FilePatternLogLevel::Info, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "warn") {
                    rule.out_actions
                        .push(FilePatternOutAction::Log(FilePatternLogLevel::Warn, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "error") {
                    rule.out_actions
                        .push(FilePatternOutAction::Log(FilePatternLogLevel::Error, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "debug") {
                    rule.out_actions
                        .push(FilePatternOutAction::Log(FilePatternLogLevel::Debug, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "trace") {
                    rule.out_actions
                        .push(FilePatternOutAction::Log(FilePatternLogLevel::Trace, msg));
                } else {
                    return Err(format!(
                        "unknown `do` directive in pattern `{path}`: {}",
                        line.trim()
                    ));
                }
            }
            Section::None => {
                return Err(format!(
                    "pattern `{path}` must start with `if:` and `do:` sections"
                ));
            }
        }
    }

    if has_current_in {
        let flushed = std::mem::take(&mut current_in_blocks);
        rule.in_blocks.push(flushed);
    }

    if rule.in_blocks.is_empty() {
        return Err(format!("pattern `{path}` has no `if` blocks"));
    }
    if rule.out_actions.is_empty() {
        return Err(format!("pattern `{path}` has no `do` actions"));
    }

    Ok(rule)
}

fn strip_inline_comment(line: &str) -> String {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut in_backtick = false;
    let mut escaped = false;
    let mut out = String::with_capacity(line.len());

    for ch in line.chars() {
        if escaped {
            out.push(ch);
            escaped = false;
            continue;
        }

        match ch {
            '\\' if in_single_quote || in_double_quote || in_backtick => {
                out.push(ch);
                escaped = true;
            }
            '\'' if !in_double_quote && !in_backtick => {
                in_single_quote = !in_single_quote;
                out.push(ch);
            }
            '"' if !in_single_quote && !in_backtick => {
                in_double_quote = !in_double_quote;
                out.push(ch);
            }
            '`' if !in_single_quote && !in_double_quote => {
                in_backtick = !in_backtick;
                out.push(ch);
            }
            '#' if !in_single_quote && !in_double_quote && !in_backtick => {
                break;
            }
            _ => out.push(ch),
        }
    }

    out
}

fn parse_multiline_value(
    line: &str,
    prefix: &str,
    lines: &[&str],
    idx: &mut usize,
) -> Result<String, String> {
    let mut value = line
        .strip_prefix(prefix)
        .ok_or_else(|| format!("line `{line}` does not start with `{prefix}`"))?
        .trim();

    if value.starts_with('`') {
        value = &value[1..];
        let mut payload = String::new();
        let mut cursor = value;
        loop {
            if let Some(end_idx) = cursor.find('`') {
                payload.push_str(&cursor[..end_idx]);
                return Ok(payload.trim().to_string());
            }
            payload.push_str(cursor);
            if *idx >= lines.len() {
                return Err("unterminated backtick string in pattern file".to_string());
            }
            payload.push('\n');
            cursor = lines[*idx];
            *idx += 1;
        }
    }

    Ok(value.to_string())
}

fn split_pattern_sequence_raw(value: &str, strip_asm_prefix: bool) -> Vec<String> {
    value
        .lines()
        .flat_map(|line| line.split(';'))
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| {
            if strip_asm_prefix {
                item.strip_prefix("asm ").unwrap_or(item).trim().to_string()
            } else {
                item.to_string()
            }
        })
        .collect()
}

fn expand_in_blocks_for_asm(blocks: &mut Vec<FilePatternInBlock>, sequence: &[FilePatternAsmData]) {
    if sequence.is_empty() {
        return;
    }

    let mut expanded = Vec::with_capacity(blocks.len() * (sequence.len() + 1));
    for block in blocks.iter() {
        if !block.asm.is_empty() {
            expanded.push(block.clone());
        }
        for item in sequence {
            let mut next = block.clone();
            next.asm = item.clone();
            expanded.push(next);
        }
    }
    *blocks = expanded;
}

fn expand_in_blocks_for_ast(blocks: &mut Vec<FilePatternInBlock>, sequence: &[FilePatternAstData]) {
    if sequence.is_empty() {
        return;
    }

    let mut expanded = Vec::with_capacity(blocks.len() * (sequence.len() + 1));
    for block in blocks.iter() {
        if !block.ast.is_empty() {
            expanded.push(block.clone());
        }
        for item in sequence {
            let mut next = block.clone();
            next.ast = item.clone();
            expanded.push(next);
        }
    }
    *blocks = expanded;
}

fn expand_in_blocks_for_ir(blocks: &mut Vec<FilePatternInBlock>, sequence: &[FilePatternIrData]) {
    if sequence.is_empty() {
        return;
    }

    let mut expanded = Vec::with_capacity(blocks.len() * (sequence.len() + 1));
    for block in blocks.iter() {
        if !block.ir.is_empty() {
            expanded.push(block.clone());
        }
        for item in sequence {
            let mut next = block.clone();
            next.ir = item.clone();
            expanded.push(next);
        }
    }
    *blocks = expanded;
}

fn expand_in_blocks_for_script(blocks: &mut Vec<FilePatternInBlock>, script: &FilePatternScript) {
    if script.is_empty() {
        return;
    }

    let mut expanded = Vec::with_capacity(blocks.len() * 2);
    for block in blocks.iter() {
        if !block.script_conditions.is_empty() {
            expanded.push(block.clone());
        }
        let mut next = block.clone();
        next.script_conditions = script.clone();
        expanded.push(next);
    }
    *blocks = expanded;
}

fn update_all_in_blocks(
    blocks: &mut [FilePatternInBlock],
    mut update: impl FnMut(&mut FilePatternInBlock),
) {
    for block in blocks.iter_mut() {
        update(block);
    }
}

fn parse_skip_range(value: &str) -> Result<FilePatternRange, String> {
    let (start_raw, end_raw) = value
        .trim()
        .split_once("..")
        .ok_or_else(|| format!("invalid skip range `{value}`"))?;
    let start = start_raw
        .trim()
        .parse::<usize>()
        .map_err(|err| format!("invalid skip start `{start_raw}`: {err}"))?;
    let end = end_raw
        .trim()
        .parse::<usize>()
        .map_err(|err| format!("invalid skip end `{end_raw}`: {err}"))?;
    if start > end {
        return Err(format!("invalid skip range `{value}`: start > end"));
    }
    Ok(FilePatternRange {
        start,
        end_exclusive: end,
    })
}

fn parse_rhai_script(script: &str) -> Result<FilePatternScript, String> {
    let source = script.trim().to_string();
    if source.is_empty() {
        return Err("script must not be empty".to_string());
    }
    let compiled = compiled_script(&source)?;
    Ok(FilePatternScript::single(source, compiled))
}

fn parse_log_action(line: &str, name: &str) -> Option<String> {
    let prefix = format!("{name}(");
    if !line.starts_with(&prefix) || !line.ends_with(')') {
        return None;
    }
    let mut inner = line[prefix.len()..line.len() - 1].trim();
    if (inner.starts_with('"') && inner.ends_with('"'))
        || (inner.starts_with('\'') && inner.ends_with('\''))
    {
        inner = &inner[1..inner.len() - 1];
    }
    Some(inner.to_string())
}

fn pattern_enabled(patterns: &[AstPattern], expected: &str) -> bool {
    if patterns.is_empty() {
        return true;
    }
    patterns.iter().any(|pattern| pattern.name == expected)
}

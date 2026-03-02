//! Pattern matching constraints:
//! - Do not hardcode AST text specializations like `some([])` or `block([])`.
//! - AST/IR matching must rely on generic `...` wildcard semantics.
//! - Keep AST/IR pattern payloads typed; avoid adding `source: String` mirrors.

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
    hash::Hash,
    num::NonZeroU8,
    panic::AssertUnwindSafe,
    time::SystemTime,
};
use tracing::{debug, error, info, trace, warn};

mod hashing;
pub(super) use hashing::{Blake3StdHasher, hash_statement_list, structural_statement_hash};

const PREDEFINED_PRUNE_EMPTY_ELSE_FB: &str =
    include_str!("../../../../patterns/prune-empty-else.fb");
const PREDEFINED_COLLAPSE_EMPTY_BLOCKS_FB: &str =
    include_str!("../../../../patterns/collapse-empty-blocks.fb");

#[derive(Debug, Clone)]
pub struct AstPattern {
    name: String,
    origin: AstPatternOrigin,
    input_type: AstPatternInputType,
    pattern: String,
    parsed: AstPatternParsed,
}
impl PartialEq for AstPattern {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.origin == other.origin
            && self.input_type == other.input_type
            && self.pattern == other.pattern
    }
}
impl Eq for AstPattern {}
impl AstPattern {
    // NOTE: stable Rust cannot hold a non-empty `Vec<Self>` in a `const`.
    // Empty here acts as a sentinel, resolved to embedded predefined patterns.
    pub const ALL: Vec<Self> = vec![];

    pub fn new(name: impl Into<String>, pattern: impl Into<String>) -> Self {
        let name = name.into();
        let pattern = pattern.into();
        let pattern_trimmed = pattern.trim();
        let parse_result = if fs::metadata(pattern_trimmed).is_ok() {
            load_file_pattern_rule_uncached(pattern_trimmed)
        } else {
            parse_pattern_file(&name, &pattern)
        };
        let (parsed, input_type) = match parse_result {
            Ok(rule) => {
                let input_type = infer_input_type_from_in_blocks(&rule.in_blocks);
                (AstPatternParsed::File(rule), input_type)
            }
            Err(err) => (
                AstPatternParsed::ParseError(err),
                AstPatternInputType::Complex,
            ),
        };
        Self {
            name,
            origin: AstPatternOrigin::UserInput,
            input_type,
            pattern,
            parsed,
        }
    }

    pub fn from_file(path: impl Into<String>) -> Self {
        let path = path.into();
        let (parsed, input_type) = match load_file_pattern_rule_uncached(&path) {
            Ok(rule) => {
                let input_type = infer_input_type_from_in_blocks(&rule.in_blocks);
                (AstPatternParsed::File(rule), input_type)
            }
            Err(err) => (
                AstPatternParsed::ParseError(err),
                AstPatternInputType::Complex,
            ),
        };
        Self {
            name: path.clone(),
            origin: AstPatternOrigin::File,
            input_type,
            pattern: path,
            parsed,
        }
    }

    pub fn with_rule(mut self, rule: AstPatternRule) -> Self {
        self.input_type = infer_input_type_from_in_blocks(&rule.in_blocks);
        self.origin = AstPatternOrigin::UserInput;
        self.parsed = AstPatternParsed::File(rule);
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn origin(&self) -> AstPatternOrigin {
        self.origin
    }

    pub fn input_type(&self) -> AstPatternInputType {
        self.input_type
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    pub fn predefined_patterns() -> Vec<Self> {
        vec![
            Self::from_predefined_include(
                "collapse-empty-blocks.fb",
                PREDEFINED_COLLAPSE_EMPTY_BLOCKS_FB,
            ),
            Self::from_predefined_include("prune-empty-else.fb", PREDEFINED_PRUNE_EMPTY_ELSE_FB),
        ]
    }

    fn from_predefined_include(name: &str, source: &str) -> Self {
        let (parsed, input_type) = match parse_pattern_file(name, source) {
            Ok(rule) => {
                let input_type = infer_input_type_from_in_blocks(&rule.in_blocks);
                (AstPatternParsed::File(rule), input_type)
            }
            Err(err) => (
                AstPatternParsed::ParseError(err),
                AstPatternInputType::Complex,
            ),
        };
        Self {
            name: name.to_string(),
            origin: AstPatternOrigin::PreDefined,
            input_type,
            pattern: name.to_string(),
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
pub enum AstPatternInputType {
    WithAssembly,
    WithIr,
    WithAst,
    Complex,
}

#[derive(Debug, Clone)]
enum AstPatternParsed {
    None,
    File(AstPatternRule),
    ParseError(String),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct AstPatternMatch {
    asm_statement_range: Option<(usize, usize)>,
    ast_statement_range: Option<(usize, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstPatternApplyPhase {
    BeforeIrAnalyzation,
    AfterIrAnalyzation,
    AfterParameterAnalyzation,
    AfterCallArgumentAnalyzation,
    AfterIteration,
    AfterOptimization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstPatternApplyAt {
    Any,
    Phase(AstPatternApplyPhase),
}

#[derive(Debug, Clone, Default)]
pub struct AstPatternRule {
    pub source: String,
    pub in_blocks: Vec<Vec<AstPatternInBlock>>,
    pub out_actions: Vec<AstPatternOutAction>,
}

#[derive(Debug, Clone)]
pub enum AstPatternInBlock {
    At(AstPatternApplyAt),
    Asm(AstPatternAsmData),
    Ast(AstPatternAstData),
    Ir(AstPatternIrData),
    Script(AstPatternScript),
    SkipRange(AstPatternRange),
    SkipAsmRange(AstPatternRange),
    SkipAstRange(AstPatternRange),
    SkipIrRange(AstPatternRange),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AstPatternInBlockKind {
    At,
    Asm,
    Ast,
    Ir,
    Script,
    SkipRange,
    SkipAsmRange,
    SkipAstRange,
    SkipIrRange,
}

impl AstPatternInBlock {
    fn kind(&self) -> AstPatternInBlockKind {
        match self {
            Self::At(_) => AstPatternInBlockKind::At,
            Self::Asm(_) => AstPatternInBlockKind::Asm,
            Self::Ast(_) => AstPatternInBlockKind::Ast,
            Self::Ir(_) => AstPatternInBlockKind::Ir,
            Self::Script(_) => AstPatternInBlockKind::Script,
            Self::SkipRange(_) => AstPatternInBlockKind::SkipRange,
            Self::SkipAsmRange(_) => AstPatternInBlockKind::SkipAsmRange,
            Self::SkipAstRange(_) => AstPatternInBlockKind::SkipAstRange,
            Self::SkipIrRange(_) => AstPatternInBlockKind::SkipIrRange,
        }
    }
}

fn set_clause(clauses: &mut Vec<AstPatternInBlock>, clause: AstPatternInBlock) {
    let kind = clause.kind();
    clauses.retain(|old| old.kind() != kind);
    clauses.push(clause);
}

fn add_at_clause(clauses: &mut Vec<AstPatternInBlock>, at: AstPatternApplyAt) {
    if clauses
        .iter()
        .any(|old| matches!(old, AstPatternInBlock::At(existing) if *existing == at))
    {
        return;
    }
    clauses.push(AstPatternInBlock::At(at));
}

fn has_kind(clauses: &[AstPatternInBlock], expected: AstPatternInBlockKind) -> bool {
    clauses.iter().any(|clause| clause.kind() == expected)
}

fn infer_input_type_from_in_blocks(in_blocks: &[Vec<AstPatternInBlock>]) -> AstPatternInputType {
    let mut has_asm = false;
    let mut has_ast = false;
    let mut has_ir = false;
    let mut has_script = false;
    for clause in in_blocks.iter().flatten() {
        match clause {
            AstPatternInBlock::At(_) => {}
            AstPatternInBlock::Asm(_) => has_asm = true,
            AstPatternInBlock::Ast(_) => has_ast = true,
            AstPatternInBlock::Ir(_) => has_ir = true,
            AstPatternInBlock::Script(_) => has_script = true,
            AstPatternInBlock::SkipRange(_)
            | AstPatternInBlock::SkipAsmRange(_)
            | AstPatternInBlock::SkipAstRange(_)
            | AstPatternInBlock::SkipIrRange(_) => {}
        }
    }

    if has_script {
        return AstPatternInputType::Complex;
    }

    match (has_asm, has_ir, has_ast) {
        (true, false, false) => AstPatternInputType::WithAssembly,
        (false, true, false) => AstPatternInputType::WithIr,
        (false, false, true) => AstPatternInputType::WithAst,
        (false, false, false) => AstPatternInputType::Complex,
        _ => AstPatternInputType::Complex,
    }
}

fn has_script_in_blocks(in_blocks: &[Vec<AstPatternInBlock>]) -> bool {
    in_blocks
        .iter()
        .flatten()
        .any(|clause| matches!(clause, AstPatternInBlock::Script(_)))
}

fn block_asm(clauses: &[AstPatternInBlock]) -> Option<&AstPatternAsmData> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::Asm(value) => Some(value),
        _ => None,
    })
}

fn block_at_matches_phase(
    clauses: &[AstPatternInBlock],
    phase: AstPatternApplyPhase,
) -> (bool, bool) {
    let mut has_at = false;
    let mut matched = false;
    for clause in clauses {
        let AstPatternInBlock::At(value) = clause else {
            continue;
        };
        has_at = true;
        match value {
            AstPatternApplyAt::Any => {
                matched = true;
                break;
            }
            AstPatternApplyAt::Phase(expected) if *expected == phase => {
                matched = true;
            }
            AstPatternApplyAt::Phase(_) => {}
        }
    }
    (has_at, matched)
}

fn block_ast(clauses: &[AstPatternInBlock]) -> Option<&AstPatternAstData> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::Ast(value) => Some(value),
        _ => None,
    })
}

fn block_ir(clauses: &[AstPatternInBlock]) -> Option<&AstPatternIrData> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::Ir(value) => Some(value),
        _ => None,
    })
}

fn block_script(clauses: &[AstPatternInBlock]) -> Option<&AstPatternScript> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::Script(value) => Some(value),
        _ => None,
    })
}

fn block_skip_range(clauses: &[AstPatternInBlock]) -> Option<AstPatternRange> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::SkipRange(value) => Some(*value),
        _ => None,
    })
}

fn block_skip_asm_range(clauses: &[AstPatternInBlock]) -> Option<AstPatternRange> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::SkipAsmRange(value) => Some(*value),
        _ => None,
    })
}

fn block_skip_ast_range(clauses: &[AstPatternInBlock]) -> Option<AstPatternRange> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::SkipAstRange(value) => Some(*value),
        _ => None,
    })
}

fn block_skip_ir_range(clauses: &[AstPatternInBlock]) -> Option<AstPatternRange> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::SkipIrRange(value) => Some(*value),
        _ => None,
    })
}

#[derive(Debug, Clone)]
pub enum AstPatternOutAction {
    ReplaceAsm(AstPatternAsmData),
    ReplaceIr(AstPatternIrReplacement),
    ReplaceAst(AstStatement),
    Delete(AstPatternDeleteTarget),
    Script(AstPatternScript),
    Log(AstPatternLogLevel, String),
    PruneEmptyElse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstPatternDeleteAnchor {
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AstPatternDeleteTarget {
    Index {
        anchor: AstPatternDeleteAnchor,
        offset: isize,
    },
    Range {
        anchor: AstPatternDeleteAnchor,
        start_offset: isize,
        end_offset_exclusive: isize,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum AstPatternLogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Trace,
}

#[derive(Debug, Clone)]
pub struct AstPatternIrReplacement {
    pub statement: Option<IrStatement>,
    pub fallback_comment: String,
}
impl AstPatternIrReplacement {
    pub fn from_text(replacement: &str) -> Self {
        let statement = parse_ir_statement(replacement);
        let fallback_comment = format!("IR: {}", replacement.trim());
        Self {
            statement,
            fallback_comment,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstPatternAsmData {
    pub source: String,
    pub statement: AstStatement,
}
impl AstPatternAsmData {
    pub fn from_text(text: &str) -> Option<Self> {
        let value = text.trim();
        if value.is_empty() {
            return None;
        }
        Some(Self {
            source: value.to_string(),
            statement: parse_asm_statement(value)
                .map(|statement| AstStatement::Ir(Box::new(statement)))
                .unwrap_or_else(|| AstStatement::Comment(format!("asm {value}"))),
        })
    }

    fn is_empty(&self) -> bool {
        self.source.trim().is_empty()
    }

    fn as_match_text(&self) -> &str {
        self.source.trim()
    }
}

impl Default for AstPatternAsmData {
    fn default() -> Self {
        Self {
            source: String::new(),
            statement: AstStatement::Empty,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstPatternAstData {
    matcher: AstPatternAstMatcher,
}

#[derive(Debug, Clone)]
enum AstPatternAstMatcher {
    Empty,
    Undefined,
    ReturnAny,
    CommentContains(Box<[u8]>),
    BlockEmpty,
    SomeEmpty,
    IfAny,
    IrExact(Box<IrStatement>),
    Unsupported,
}

impl AstPatternAstData {
    pub fn from_text(text: &str) -> Option<Self> {
        let value = text.trim();
        if value.is_empty() {
            return None;
        }
        Some(Self {
            matcher: compile_ast_matcher(value),
        })
    }

    fn matches_statement(&self, statement: &AstStatement) -> bool {
        match &self.matcher {
            AstPatternAstMatcher::Empty => matches!(statement, AstStatement::Empty),
            AstPatternAstMatcher::Undefined => matches!(statement, AstStatement::Undefined),
            AstPatternAstMatcher::ReturnAny => matches!(statement, AstStatement::Return(_)),
            AstPatternAstMatcher::CommentContains(expected) => matches!(
                statement,
                AstStatement::Comment(comment)
                    if normalized_comment_contains(comment, expected.as_ref())
            ),
            AstPatternAstMatcher::BlockEmpty => {
                matches!(statement, AstStatement::Block(body) if body.is_empty())
            }
            AstPatternAstMatcher::SomeEmpty => {
                matches!(statement, AstStatement::If(_, _, Some(branch_false)) if branch_false.is_empty())
            }
            AstPatternAstMatcher::IfAny => matches!(statement, AstStatement::If(_, _, _)),
            AstPatternAstMatcher::IrExact(expected_ir) => {
                matches!(statement, AstStatement::Ir(actual_ir) if actual_ir.as_ref() == expected_ir.as_ref())
            }
            AstPatternAstMatcher::Unsupported => false,
        }
    }
}

impl Default for AstPatternAstData {
    fn default() -> Self {
        Self {
            matcher: AstPatternAstMatcher::Empty,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstPatternIrData {
    matcher: AstPatternIrMatcher,
}

#[derive(Debug, Clone)]
enum AstPatternIrMatcher {
    Exact(IrStatement),
    IfAny,
    Any,
    Unsupported,
}
impl AstPatternIrData {
    pub fn from_text(text: &str) -> Option<Self> {
        let value = text.trim();
        if value.is_empty() {
            return None;
        }
        Some(Self {
            matcher: compile_ir_matcher(value),
        })
    }

    fn matches_statement(&self, statement: &IrStatement) -> bool {
        match &self.matcher {
            AstPatternIrMatcher::Exact(expected) => expected == statement,
            AstPatternIrMatcher::IfAny => matches!(statement, IrStatement::Condition { .. }),
            AstPatternIrMatcher::Any => true,
            AstPatternIrMatcher::Unsupported => false,
        }
    }
}

impl Default for AstPatternIrData {
    fn default() -> Self {
        Self {
            matcher: AstPatternIrMatcher::Unsupported,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AstPatternScript {
    pub source: String,
    pub compiled: Option<RhaiAst>,
}
impl AstPatternScript {
    fn single(source: String, compiled: RhaiAst) -> Self {
        Self {
            source,
            compiled: Some(compiled),
        }
    }

    pub fn from_source(script: impl Into<String>) -> Result<Self, String> {
        let source = script.into().trim().to_string();
        if source.is_empty() {
            return Err("script must not be empty".to_string());
        }
        let compiled = compiled_script(&source)?;
        Ok(Self::single(source, compiled))
    }

    fn compiled_or_parse(&self) -> Result<RhaiAst, String> {
        if let Some(compiled) = &self.compiled {
            return Ok(compiled.clone());
        }
        let source = self.source();
        if source.is_empty() {
            return Err("script must not be empty".to_string());
        }
        compiled_script(source)
    }

    fn is_empty(&self) -> bool {
        self.source.trim().is_empty()
    }

    fn source(&self) -> &str {
        self.source.trim()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstPatternRange {
    pub start: usize,
    pub end_exclusive: usize,
}

fn compile_ast_matcher(text: &str) -> AstPatternAstMatcher {
    if let Some(content) = text.strip_prefix("comment ") {
        let normalized = normalize_comment_bytes(content);
        if normalized.is_empty() {
            return AstPatternAstMatcher::Unsupported;
        }
        return AstPatternAstMatcher::CommentContains(normalized.into_boxed_slice());
    }

    let strict = normalize_for_wildcard_match(text);
    if strict == "empty" {
        return AstPatternAstMatcher::Empty;
    }
    if strict == "undefined" {
        return AstPatternAstMatcher::Undefined;
    }
    if strict == "return" {
        return AstPatternAstMatcher::ReturnAny;
    }
    if strict == "block([])" {
        return AstPatternAstMatcher::BlockEmpty;
    }
    if strict == "some([])" {
        return AstPatternAstMatcher::SomeEmpty;
    }
    if normalize_for_wildcard_match_relaxed(text) == "if..." {
        return AstPatternAstMatcher::IfAny;
    }
    if let Some(ir_text) = text.strip_prefix("ir ") {
        if let Some(statement) = parse_ir_statement(ir_text.trim()) {
            return AstPatternAstMatcher::IrExact(Box::new(statement));
        }
    }
    if let Some(asm_text) = text.strip_prefix("asm ") {
        if let Some(statement) = parse_asm_statement(asm_text.trim()) {
            return AstPatternAstMatcher::IrExact(Box::new(statement));
        }
    }
    AstPatternAstMatcher::Unsupported
}

fn compile_ir_matcher(text: &str) -> AstPatternIrMatcher {
    let strict = normalize_for_wildcard_match(text);
    if strict == "..." {
        return AstPatternIrMatcher::Any;
    }
    if normalize_for_wildcard_match_relaxed(text) == "if..." {
        return AstPatternIrMatcher::IfAny;
    }
    parse_ir_statement(text)
        .map(AstPatternIrMatcher::Exact)
        .unwrap_or(AstPatternIrMatcher::Unsupported)
}

fn normalize_comment_bytes(text: &str) -> Vec<u8> {
    normalize_for_match(text).into_bytes()
}

fn normalized_comment_contains(comment: &str, expected_normalized: &[u8]) -> bool {
    if expected_normalized.is_empty() {
        return false;
    }
    let normalized = normalize_comment_bytes(comment);
    if normalized.len() < expected_normalized.len() {
        return false;
    }
    normalized
        .windows(expected_normalized.len())
        .any(|window| window == expected_normalized)
}

#[derive(Debug, Clone)]
struct AstPatternNormalizedAsmLine {
    stmt_index: usize,
    line: String,
}

#[derive(Debug, Clone, Copy)]
struct AstPatternScriptContext<'a> {
    source: &'a str,
    ast_debug: &'a str,
    ir_debug: &'a str,
    asm_debug: &'a str,
    statement_count: i64,
    asm_count: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AstPatternFileFingerprint {
    modified: Option<SystemTime>,
    len: u64,
}

#[derive(Debug, Clone)]
struct AstPatternCachedRule {
    fingerprint: AstPatternFileFingerprint,
    rule: AstPatternRule,
}

#[derive(Debug, Clone)]
struct AstPatternLoadedRule {
    rule: AstPatternRule,
    input_type: AstPatternInputType,
}

thread_local! {
    static FILE_PATTERN_CACHE: RefCell<HashMap<String, AstPatternCachedRule>> = RefCell::new(HashMap::new());
    static RHAI_ENGINE: RefCell<Engine> = RefCell::new(build_rhai_engine());
    static RHAI_SCRIPT_CACHE: RefCell<HashMap<String, RhaiAst>> = RefCell::new(HashMap::new());
}

pub(super) fn apply_patterns(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    patterns: &[AstPattern],
    phase: AstPatternApplyPhase,
) -> Result<(), DecompileError> {
    let mut body;
    let ir_debug;
    let function_ir_statements;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function_ir_statements = collect_function_ir_statements(function.ir.get_ir());
        ir_debug = format!("{:?}", function.ir.get_ir());
        body = std::mem::take(&mut function.body);
    }

    let file_rules = load_file_pattern_rules(patterns)?;
    if !file_rules.is_empty() {
        apply_file_pattern_rules_recursive(
            &mut body,
            &file_rules,
            &ir_debug,
            &function_ir_statements,
            phase,
        );
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

fn load_file_pattern_rules(
    patterns: &[AstPattern],
) -> Result<Vec<AstPatternLoadedRule>, DecompileError> {
    let resolved_patterns = resolve_patterns(patterns);
    let mut rules = Vec::new();
    for pattern in &resolved_patterns {
        match &pattern.parsed {
            AstPatternParsed::File(rule) => {
                rules.push(AstPatternLoadedRule {
                    rule: rule.clone(),
                    input_type: infer_input_type_from_in_blocks(&rule.in_blocks),
                });
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
        rules.push(AstPatternLoadedRule {
            input_type: infer_input_type_from_in_blocks(&rule.in_blocks),
            rule,
        });
    }
    Ok(rules)
}

fn resolve_patterns(patterns: &[AstPattern]) -> Vec<AstPattern> {
    if patterns.is_empty() {
        return AstPattern::predefined_patterns();
    }
    patterns.to_vec()
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
    rules: &[AstPatternLoadedRule],
    ir_debug: &str,
    function_ir_statements: &[IrStatement],
    phase: AstPatternApplyPhase,
) -> bool {
    let mut changed = false;
    let mut seen_states = HashSet::new();

    loop {
        let state_before = structural_statement_hash(stmts);
        if !seen_states.insert(state_before) {
            break;
        }

        let mut pass_changed = false;

        for loaded_rule in rules {
            pass_changed |= apply_single_file_rule(
                stmts,
                &loaded_rule.rule,
                loaded_rule.input_type,
                ir_debug,
                function_ir_statements,
                phase,
            );
        }

        for stmt in stmts.iter_mut() {
            match &mut stmt.statement {
                AstStatement::If(_, branch_true, branch_false) => {
                    pass_changed |= apply_file_pattern_rules_recursive(
                        branch_true,
                        rules,
                        ir_debug,
                        function_ir_statements,
                        phase,
                    );
                    if let Some(branch_false) = branch_false {
                        pass_changed |= apply_file_pattern_rules_recursive(
                            branch_false,
                            rules,
                            ir_debug,
                            function_ir_statements,
                            phase,
                        );
                    }
                }
                AstStatement::While(_, body) => {
                    pass_changed |= apply_file_pattern_rules_recursive(
                        body,
                        rules,
                        ir_debug,
                        function_ir_statements,
                        phase,
                    );
                }
                AstStatement::For(init, _, update, body) => {
                    let mut init_vec = vec![(**init).clone()];
                    pass_changed |= apply_file_pattern_rules_recursive(
                        &mut init_vec,
                        rules,
                        ir_debug,
                        function_ir_statements,
                        phase,
                    );
                    if let Some(next_init) = init_vec.into_iter().next() {
                        **init = next_init;
                    }

                    let mut update_vec = vec![(**update).clone()];
                    pass_changed |= apply_file_pattern_rules_recursive(
                        &mut update_vec,
                        rules,
                        ir_debug,
                        function_ir_statements,
                        phase,
                    );
                    if let Some(next_update) = update_vec.into_iter().next() {
                        **update = next_update;
                    }

                    pass_changed |= apply_file_pattern_rules_recursive(
                        body,
                        rules,
                        ir_debug,
                        function_ir_statements,
                        phase,
                    );
                }
                AstStatement::Block(body) => {
                    pass_changed |= apply_file_pattern_rules_recursive(
                        body,
                        rules,
                        ir_debug,
                        function_ir_statements,
                        phase,
                    );
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

        let state_after = structural_statement_hash(stmts);
        if state_after != state_before {
            pass_changed = true;
            changed = true;
        }

        if !pass_changed {
            break;
        }
    }

    changed
}

fn apply_single_file_rule(
    stmts: &mut Vec<WrappedAstStatement>,
    rule: &AstPatternRule,
    input_type: AstPatternInputType,
    ir_debug: &str,
    function_ir_statements: &[IrStatement],
    phase: AstPatternApplyPhase,
) -> bool {
    if stmts.is_empty() {
        return false;
    }

    let need_script = has_script_in_blocks(&rule.in_blocks);
    let (need_asm, need_ast, need_ir) = match input_type {
        AstPatternInputType::WithAssembly => (true, false, false),
        AstPatternInputType::WithAst => (false, true, false),
        AstPatternInputType::WithIr => (false, false, true),
        AstPatternInputType::Complex => (true, true, true),
    };
    let mut changed = false;
    let mut seen_states = HashSet::new();
    let mut skipped_matches = HashSet::new();

    loop {
        if stmts.is_empty() {
            break;
        }

        let state_before = structural_statement_hash(stmts);
        if !seen_states.insert(state_before) {
            break;
        }

        let ast_debug = if need_script {
            collect_ast_debug(stmts)
        } else {
            String::new()
        };
        let ast_statements = if need_ast {
            collect_ast_statements(stmts)
        } else {
            Vec::new()
        };
        let ir_statements = if need_ir {
            collect_ir_statements(function_ir_statements, stmts)
        } else {
            Vec::new()
        };
        let asm_lines = if need_asm || need_script {
            collect_asm_lines(stmts)
        } else {
            Vec::new()
        };
        let asm_debug = if need_script {
            asm_lines
                .iter()
                .map(|line| line.line.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            String::new()
        };
        let script_context = AstPatternScriptContext {
            source: &rule.source,
            ast_debug: &ast_debug,
            ir_debug,
            asm_debug: &asm_debug,
            statement_count: stmts.len() as i64,
            asm_count: asm_lines.len() as i64,
        };
        let matched = rule.in_blocks.iter().find_map(|block| {
            let matched = match_if_block(
                block,
                &script_context,
                &asm_lines,
                &ast_statements,
                &ir_statements,
                phase,
            )?;
            if skipped_matches.contains(&matched) {
                None
            } else {
                Some(matched)
            }
        });
        let Some(matched) = matched else {
            break;
        };

        for action in &rule.out_actions {
            match action {
                AstPatternOutAction::ReplaceAsm(replacement) => {
                    apply_replace_asm(stmts, &matched, replacement);
                }
                AstPatternOutAction::ReplaceIr(replacement) => {
                    apply_replace_ir(stmts, &matched, replacement);
                }
                AstPatternOutAction::ReplaceAst(replacement) => {
                    apply_replace_ast(stmts, &matched, replacement);
                }
                AstPatternOutAction::Delete(target) => {
                    apply_delete_action(stmts, &matched, target);
                }
                AstPatternOutAction::Script(script) => {
                    if !execute_do_script(script, &script_context) {
                        break;
                    }
                }
                AstPatternOutAction::Log(level, msg) => match level {
                    AstPatternLogLevel::Info => {
                        info!("Pattern `{}` matched: {}", rule.source, msg)
                    }
                    AstPatternLogLevel::Warn => {
                        warn!("Pattern `{}` matched: {}", rule.source, msg)
                    }
                    AstPatternLogLevel::Error => {
                        error!("Pattern `{}` matched: {}", rule.source, msg)
                    }
                    AstPatternLogLevel::Debug => {
                        debug!("Pattern `{}` matched: {}", rule.source, msg)
                    }
                    AstPatternLogLevel::Trace => {
                        trace!("Pattern `{}` matched: {}", rule.source, msg)
                    }
                },
                AstPatternOutAction::PruneEmptyElse => {
                    prune_empty_else_recursive(stmts);
                }
            }
        }

        let state_after = structural_statement_hash(stmts);
        if state_after != state_before {
            changed = true;
            if matched.asm_statement_range.is_some() || matched.ast_statement_range.is_some() {
                skipped_matches.clear();
            } else {
                skipped_matches.insert(matched);
            }
            continue;
        } else {
            skipped_matches.insert(matched);
        }
    }

    changed
}

fn prune_empty_else_recursive(stmts: &mut [WrappedAstStatement]) {
    for stmt in stmts.iter_mut() {
        prune_empty_else_statement_recursive(stmt);
    }
}

fn prune_empty_else_statement_recursive(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            prune_empty_else_recursive(branch_true);
            if let Some(branch_false) = branch_false {
                prune_empty_else_recursive(branch_false);
            }
            let remove_else = matches!(branch_false.as_ref(), Some(branch) if branch.is_empty());
            if remove_else {
                *branch_false = None;
            }
        }
        AstStatement::While(_, body) => {
            prune_empty_else_recursive(body);
        }
        AstStatement::For(init, _, update, body) => {
            prune_empty_else_statement_recursive(init);
            prune_empty_else_statement_recursive(update);
            prune_empty_else_recursive(body);
        }
        AstStatement::Block(body) => {
            prune_empty_else_recursive(body);
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

fn collect_ast_statements(stmts: &[WrappedAstStatement]) -> Vec<AstStatement> {
    stmts.iter().map(|stmt| stmt.statement.clone()).collect()
}

fn collect_function_ir_statements(function_ir: &[crate::ir::Ir]) -> Vec<IrStatement> {
    let mut statements = Vec::new();
    for ir in function_ir {
        if let Some(ir_statements) = ir.statements {
            statements.extend(ir_statements.iter().cloned());
        }
    }
    statements
}

fn collect_ir_statements(
    function_ir_statements: &[IrStatement],
    stmts: &[WrappedAstStatement],
) -> Vec<IrStatement> {
    let mut statements = Vec::with_capacity(function_ir_statements.len() + stmts.len());
    statements.extend(function_ir_statements.iter().cloned());
    for stmt in stmts {
        if let AstStatement::Ir(ir_stmt) = &stmt.statement {
            statements.push((**ir_stmt).clone());
        }
    }
    statements
}

fn collect_asm_lines(stmts: &[WrappedAstStatement]) -> Vec<AstPatternNormalizedAsmLine> {
    let mut lines = Vec::new();
    let mut seen_ir_indices = HashSet::new();

    for (idx, stmt) in stmts.iter().enumerate() {
        if let AstStatement::Assembly(text) = &stmt.statement {
            let line = normalize_for_match(text);
            if !line.is_empty() {
                lines.push(AstPatternNormalizedAsmLine {
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
                    lines.push(AstPatternNormalizedAsmLine {
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
            lines.push(AstPatternNormalizedAsmLine {
                stmt_index: idx,
                line,
            });
        }
    }

    lines
}

fn match_if_block(
    block: &[AstPatternInBlock],
    script_context: &AstPatternScriptContext<'_>,
    asm_lines: &[AstPatternNormalizedAsmLine],
    ast_statements: &[AstStatement],
    ir_statements: &[IrStatement],
    phase: AstPatternApplyPhase,
) -> Option<AstPatternMatch> {
    let mut has_condition = false;
    let mut matched = AstPatternMatch::default();

    let (has_at, at_matched) = block_at_matches_phase(block, phase);
    if has_at {
        has_condition = true;
        if !at_matched {
            return None;
        }
    }

    if let Some(asm) = block_asm(block) {
        has_condition = true;
        let asm_skip_range = block_skip_range(block).or(block_skip_asm_range(block));
        matched.asm_statement_range = Some(find_asm_match(asm_lines, asm, asm_skip_range)?);
    }
    if let Some(ast) = block_ast(block) {
        has_condition = true;
        matched.ast_statement_range = Some(find_ast_match(
            ast_statements,
            ast,
            block_skip_ast_range(block),
        )?);
    }
    if let Some(ir) = block_ir(block) {
        has_condition = true;
        if !sequence_matches_ir(ir_statements, ir, block_skip_ir_range(block)) {
            return None;
        }
    }
    if let Some(script_conditions) = block_script(block) {
        has_condition = true;
        if !evaluate_if_script(script_conditions, script_context) {
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

fn build_rhai_scope(context: &AstPatternScriptContext<'_>) -> Scope<'static> {
    let mut scope = Scope::new();
    scope.push("source", context.source.to_string());
    scope.push("ast", context.ast_debug.to_string());
    scope.push("ir", context.ir_debug.to_string());
    scope.push("asm", context.asm_debug.to_string());
    scope.push("stmt_count", context.statement_count);
    scope.push("asm_count", context.asm_count);
    scope
}

fn evaluate_if_script(script: &AstPatternScript, context: &AstPatternScriptContext<'_>) -> bool {
    let source = script.source();
    let compiled = match script.compiled_or_parse() {
        Ok(compiled) => compiled,
        Err(err) => {
            error!(
                "Pattern `{}` if script has no compiled AST: {} ({})",
                context.source, source, err
            );
            return false;
        }
    };
    let mut scope = build_rhai_scope(context);
    match with_rhai_engine(|engine| engine.eval_ast_with_scope::<Dynamic>(&mut scope, &compiled)) {
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

fn execute_do_script(script: &AstPatternScript, context: &AstPatternScriptContext<'_>) -> bool {
    let source = script.source();
    let compiled = match script.compiled_or_parse() {
        Ok(compiled) => compiled,
        Err(err) => {
            error!(
                "Pattern `{}` do script has no compiled AST: {} ({})",
                context.source, source, err
            );
            return false;
        }
    };
    let mut scope = build_rhai_scope(context);
    match with_rhai_engine(|engine| engine.eval_ast_with_scope::<Dynamic>(&mut scope, &compiled)) {
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
    asm_lines: &[AstPatternNormalizedAsmLine],
    asm: &AstPatternAsmData,
    skip_range: Option<AstPatternRange>,
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

fn find_ast_match(
    statements: &[AstStatement],
    data: &AstPatternAstData,
    skip_range: Option<AstPatternRange>,
) -> Option<(usize, usize)> {
    if statements.is_empty() {
        return None;
    }

    let start = skip_range.map_or(0usize, |range| range.start.min(statements.len()));
    let end_exclusive = skip_range.map_or(statements.len(), |range| {
        range.end_exclusive.min(statements.len())
    });
    if end_exclusive <= start {
        return None;
    }

    for cursor in start..end_exclusive {
        if data.matches_statement(&statements[cursor]) {
            return Some((cursor, cursor));
        }
    }

    None
}

fn sequence_matches_ir(
    statements: &[IrStatement],
    data: &AstPatternIrData,
    skip_range: Option<AstPatternRange>,
) -> bool {
    if statements.is_empty() {
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
        if data.matches_statement(statement) {
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

fn normalize_for_wildcard_match(text: &str) -> String {
    text.chars()
        .filter(|ch| !ch.is_whitespace())
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

fn normalize_for_wildcard_match_relaxed(text: &str) -> String {
    text.chars()
        .filter(|ch| {
            !ch.is_whitespace() && !matches!(ch, '(' | ')' | '[' | ']' | '{' | '}' | ',' | ';')
        })
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

fn apply_replace_asm(
    stmts: &mut [WrappedAstStatement],
    matched: &AstPatternMatch,
    replacement: &AstPatternAsmData,
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
    matched: &AstPatternMatch,
    replacement: &AstPatternIrReplacement,
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
    matched: &AstPatternMatch,
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

fn apply_delete_action(
    stmts: &mut Vec<WrappedAstStatement>,
    matched: &AstPatternMatch,
    target: &AstPatternDeleteTarget,
) {
    if stmts.is_empty() {
        return;
    }

    let len = stmts.len() as isize;
    let anchor = match target {
        AstPatternDeleteTarget::Index { anchor, .. } => *anchor,
        AstPatternDeleteTarget::Range { anchor, .. } => *anchor,
    };
    let base = resolve_delete_anchor_index(matched, anchor, stmts.len()) as isize;

    match target {
        AstPatternDeleteTarget::Index { offset, .. } => {
            let idx = base + *offset;
            if idx < 0 || idx >= len {
                return;
            }
            stmts.remove(idx as usize);
        }
        AstPatternDeleteTarget::Range {
            start_offset,
            end_offset_exclusive,
            ..
        } => {
            let mut start = (base + *start_offset).clamp(0, len) as usize;
            let mut end_exclusive = (base + *end_offset_exclusive).clamp(0, len) as usize;
            if end_exclusive < start {
                std::mem::swap(&mut start, &mut end_exclusive);
            }
            if end_exclusive <= start {
                return;
            }
            stmts.drain(start..end_exclusive);
        }
    }
}

fn resolve_delete_anchor_index(
    matched: &AstPatternMatch,
    anchor: AstPatternDeleteAnchor,
    statement_len: usize,
) -> usize {
    if let Some((start, end)) = matched.asm_statement_range {
        return match anchor {
            AstPatternDeleteAnchor::Start => start,
            AstPatternDeleteAnchor::End => end,
        };
    }
    if let Some((start, end)) = matched.ast_statement_range {
        return match anchor {
            AstPatternDeleteAnchor::Start => start,
            AstPatternDeleteAnchor::End => end,
        };
    }

    match anchor {
        AstPatternDeleteAnchor::Start => 0,
        AstPatternDeleteAnchor::End => statement_len.saturating_sub(1),
    }
}

fn parse_asm_replacement(value: &str) -> Option<AstPatternAsmData> {
    let text = value.trim();
    if text.is_empty() {
        return None;
    }
    let normalized = text.strip_prefix("asm ").unwrap_or(text).trim();
    AstPatternAsmData::from_text(normalized)
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

fn parse_ir_replacement(replacement: &str) -> AstPatternIrReplacement {
    AstPatternIrReplacement::from_text(replacement)
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
struct AstPatternParsedTypeSpecial {
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

fn parse_ir_type_special(text: &str) -> Option<AstPatternParsedTypeSpecial> {
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
            return Some(AstPatternParsedTypeSpecial {
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

fn load_file_pattern_rule_cached(path: &str) -> Result<AstPatternRule, String> {
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
            AstPatternCachedRule {
                fingerprint,
                rule: rule.clone(),
            },
        );
    });

    Ok(rule)
}

fn load_file_pattern_rule_uncached(path: &str) -> Result<AstPatternRule, String> {
    let content = fs::read_to_string(path)
        .map_err(|err| format!("failed to read pattern file `{path}`: {err}"))?;
    parse_pattern_file(path, &content)
}

fn fingerprint(path: &str) -> Result<AstPatternFileFingerprint, String> {
    let metadata =
        fs::metadata(path).map_err(|err| format!("failed to read metadata for `{path}`: {err}"))?;
    Ok(AstPatternFileFingerprint {
        modified: metadata.modified().ok(),
        len: metadata.len(),
    })
}

fn parse_pattern_file(path: &str, content: &str) -> Result<AstPatternRule, String> {
    let mut rule = AstPatternRule {
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
    let mut current_in_blocks: Vec<Vec<AstPatternInBlock>> = vec![Vec::new()];
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
                rule.in_blocks.extend(flushed);
                current_in_blocks = vec![Vec::new()];
            }
            has_current_in = true;
            section = Section::If;
            continue;
        }

        if trimmed.starts_with("do:") {
            if has_current_in {
                let flushed = std::mem::take(&mut current_in_blocks);
                rule.in_blocks.extend(flushed);
                current_in_blocks = vec![Vec::new()];
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
                        .filter_map(|item| AstPatternAsmData::from_text(item))
                        .collect::<Vec<_>>();
                    expand_in_blocks_for_asm(&mut current_in_blocks, &sequence);
                } else if line.trim_start().starts_with("ast ") {
                    let value = parse_multiline_value(line.trim_start(), "ast ", &lines, &mut idx)?;
                    let sequence = split_pattern_sequence_raw(&value, false);
                    let sequence = sequence
                        .iter()
                        .filter_map(|item| AstPatternAstData::from_text(item))
                        .collect::<Vec<_>>();
                    expand_in_blocks_for_ast(&mut current_in_blocks, &sequence);
                } else if line.trim_start().starts_with("ir ") {
                    let value = parse_multiline_value(line.trim_start(), "ir ", &lines, &mut idx)?;
                    let sequence = split_pattern_sequence_raw(&value, false);
                    let sequence = sequence
                        .iter()
                        .filter_map(|item| AstPatternIrData::from_text(item))
                        .collect::<Vec<_>>();
                    expand_in_blocks_for_ir(&mut current_in_blocks, &sequence);
                } else if line.trim_start().starts_with("script ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "script ", &lines, &mut idx)?;
                    let script = parse_rhai_script(&value)?;
                    expand_in_blocks_for_script(&mut current_in_blocks, &script);
                } else if line.trim_start().starts_with("at ") {
                    let value = parse_multiline_value(line.trim_start(), "at ", &lines, &mut idx)?;
                    let phases = parse_apply_phases(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        for phase in phases.iter().copied() {
                            add_at_clause(block, phase);
                        }
                    });
                } else if line.trim_start().starts_with("skip_range ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip_range ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(block, AstPatternInBlock::SkipRange(range));
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
                        set_clause(block, AstPatternInBlock::SkipAsmRange(range));
                    });
                } else if line.trim_start().starts_with("skip asm ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip asm ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(block, AstPatternInBlock::SkipAsmRange(range));
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
                        set_clause(block, AstPatternInBlock::SkipAstRange(range));
                    });
                } else if line.trim_start().starts_with("skip ast ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip ast ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(block, AstPatternInBlock::SkipAstRange(range));
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
                        set_clause(block, AstPatternInBlock::SkipIrRange(range));
                    });
                } else if line.trim_start().starts_with("skip ir ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip ir ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(block, AstPatternInBlock::SkipIrRange(range));
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
                        .push(AstPatternOutAction::ReplaceAsm(replacement));
                } else if trimmed.starts_with("ir ") {
                    let value = parse_multiline_value(trimmed, "ir ", &lines, &mut idx)?;
                    rule.out_actions
                        .push(AstPatternOutAction::ReplaceIr(parse_ir_replacement(&value)));
                } else if trimmed.starts_with("ast ") {
                    let value = parse_multiline_value(trimmed, "ast ", &lines, &mut idx)?;
                    rule.out_actions
                        .push(AstPatternOutAction::ReplaceAst(parse_ast_replacement(
                            &value,
                        )));
                } else if trimmed.starts_with("del ") {
                    let value = parse_multiline_value(trimmed, "del ", &lines, &mut idx)?;
                    let target = parse_do_delete_target(&value)?;
                    rule.out_actions.push(AstPatternOutAction::Delete(target));
                } else if trimmed.starts_with("script ") {
                    let value = parse_multiline_value(trimmed, "script ", &lines, &mut idx)?;
                    rule.out_actions
                        .push(AstPatternOutAction::Script(parse_rhai_script(&value)?));
                } else if let Some(msg) = parse_log_action(trimmed, "info") {
                    rule.out_actions
                        .push(AstPatternOutAction::Log(AstPatternLogLevel::Info, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "warn") {
                    rule.out_actions
                        .push(AstPatternOutAction::Log(AstPatternLogLevel::Warn, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "error") {
                    rule.out_actions
                        .push(AstPatternOutAction::Log(AstPatternLogLevel::Error, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "debug") {
                    rule.out_actions
                        .push(AstPatternOutAction::Log(AstPatternLogLevel::Debug, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "trace") {
                    rule.out_actions
                        .push(AstPatternOutAction::Log(AstPatternLogLevel::Trace, msg));
                } else if trimmed == "prune-empty-else" {
                    rule.out_actions.push(AstPatternOutAction::PruneEmptyElse);
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
        rule.in_blocks.extend(flushed);
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

fn expand_in_blocks_for_asm(
    blocks: &mut Vec<Vec<AstPatternInBlock>>,
    sequence: &[AstPatternAsmData],
) {
    if sequence.is_empty() {
        return;
    }

    let mut expanded = Vec::with_capacity(blocks.len() * (sequence.len() + 1));
    for block in blocks.iter() {
        if has_kind(block, AstPatternInBlockKind::Asm) {
            expanded.push(block.clone());
        }
        for item in sequence {
            let mut next = block.clone();
            set_clause(&mut next, AstPatternInBlock::Asm(item.clone()));
            expanded.push(next);
        }
    }
    *blocks = expanded;
}

fn expand_in_blocks_for_ast(
    blocks: &mut Vec<Vec<AstPatternInBlock>>,
    sequence: &[AstPatternAstData],
) {
    if sequence.is_empty() {
        return;
    }

    let mut expanded = Vec::with_capacity(blocks.len() * (sequence.len() + 1));
    for block in blocks.iter() {
        if has_kind(block, AstPatternInBlockKind::Ast) {
            expanded.push(block.clone());
        }
        for item in sequence {
            let mut next = block.clone();
            set_clause(&mut next, AstPatternInBlock::Ast(item.clone()));
            expanded.push(next);
        }
    }
    *blocks = expanded;
}

fn expand_in_blocks_for_ir(
    blocks: &mut Vec<Vec<AstPatternInBlock>>,
    sequence: &[AstPatternIrData],
) {
    if sequence.is_empty() {
        return;
    }

    let mut expanded = Vec::with_capacity(blocks.len() * (sequence.len() + 1));
    for block in blocks.iter() {
        if has_kind(block, AstPatternInBlockKind::Ir) {
            expanded.push(block.clone());
        }
        for item in sequence {
            let mut next = block.clone();
            set_clause(&mut next, AstPatternInBlock::Ir(item.clone()));
            expanded.push(next);
        }
    }
    *blocks = expanded;
}

fn expand_in_blocks_for_script(
    blocks: &mut Vec<Vec<AstPatternInBlock>>,
    script: &AstPatternScript,
) {
    if script.is_empty() {
        return;
    }

    let mut expanded = Vec::with_capacity(blocks.len() * 2);
    for block in blocks.iter() {
        if has_kind(block, AstPatternInBlockKind::Script) {
            expanded.push(block.clone());
        }
        let mut next = block.clone();
        set_clause(&mut next, AstPatternInBlock::Script(script.clone()));
        expanded.push(next);
    }
    *blocks = expanded;
}

fn update_all_in_blocks(
    blocks: &mut [Vec<AstPatternInBlock>],
    mut update: impl FnMut(&mut Vec<AstPatternInBlock>),
) {
    for block in blocks.iter_mut() {
        update(block);
    }
}

fn parse_skip_range(value: &str) -> Result<AstPatternRange, String> {
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
    Ok(AstPatternRange {
        start,
        end_exclusive: end,
    })
}

fn parse_rhai_script(script: &str) -> Result<AstPatternScript, String> {
    AstPatternScript::from_source(script)
}

fn parse_apply_phases(value: &str) -> Result<Vec<AstPatternApplyAt>, String> {
    let mut phases = Vec::new();
    for token in value.lines().flat_map(|line| line.split([';', ','])) {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        let phase = parse_apply_phase(token)?;
        if !phases.contains(&phase) {
            phases.push(phase);
        }
    }
    if phases.is_empty() {
        return Err(
            "invalid `at` phase: use one of any, beforeIrAnalyzation, afterIrAnalyzation, afterParameterAnalyzation, afterCallArgumentAnalyzation, afterIteration, afterOptimization".to_string(),
        );
    }
    Ok(phases)
}

fn parse_apply_phase(value: &str) -> Result<AstPatternApplyAt, String> {
    let normalized = value
        .trim()
        .chars()
        .filter(|ch| !matches!(ch, '_' | '-' | ' '))
        .collect::<String>()
        .to_ascii_lowercase();
    let at = match normalized.as_str() {
        "any" => AstPatternApplyAt::Any,
        "beforeiranalyzation" => {
            AstPatternApplyAt::Phase(AstPatternApplyPhase::BeforeIrAnalyzation)
        }
        "afteriranalyzation" => AstPatternApplyAt::Phase(AstPatternApplyPhase::AfterIrAnalyzation),
        "afterparameteranalyzation" => {
            AstPatternApplyAt::Phase(AstPatternApplyPhase::AfterParameterAnalyzation)
        }
        "aftercallargumentanalyzation" => {
            AstPatternApplyAt::Phase(AstPatternApplyPhase::AfterCallArgumentAnalyzation)
        }
        "afteriteration" => AstPatternApplyAt::Phase(AstPatternApplyPhase::AfterIteration),
        "afteroptimization" => AstPatternApplyAt::Phase(AstPatternApplyPhase::AfterOptimization),
        _ => {
            return Err(format!(
                "invalid `at` phase `{}`: use one of any, beforeIrAnalyzation, afterIrAnalyzation, afterParameterAnalyzation, afterCallArgumentAnalyzation, afterIteration, afterOptimization",
                value.trim()
            ));
        }
    };
    Ok(at)
}

fn parse_do_delete_target(value: &str) -> Result<AstPatternDeleteTarget, String> {
    let trimmed = value.trim();
    let open = trimmed
        .find('[')
        .ok_or_else(|| format!("invalid del target `{value}`"))?;
    let close = find_matching_delimiter(trimmed, open, '[', ']')
        .ok_or_else(|| format!("invalid del target `{value}`"))?;
    if close + 1 != trimmed.len() {
        return Err(format!("invalid del target `{value}`"));
    }

    let anchor = match trimmed[..open].trim() {
        "start" => AstPatternDeleteAnchor::Start,
        "end" => AstPatternDeleteAnchor::End,
        other => {
            return Err(format!(
                "invalid del anchor `{other}` in `{value}`: use `start` or `end`"
            ));
        }
    };

    let body = trimmed[open + 1..close].trim();
    if body.is_empty() {
        return Err(format!("invalid del target `{value}`"));
    }

    if let Some((start_raw, end_raw)) = body.split_once("..") {
        let start_offset = parse_signed_offset(start_raw, "range-start")?;
        let end_offset_exclusive = parse_signed_offset(end_raw, "range-end")?;
        return Ok(AstPatternDeleteTarget::Range {
            anchor,
            start_offset,
            end_offset_exclusive,
        });
    }

    let offset = parse_signed_offset(body, "index")?;
    Ok(AstPatternDeleteTarget::Index { anchor, offset })
}

fn parse_signed_offset(value: &str, name: &str) -> Result<isize, String> {
    value
        .trim()
        .parse::<isize>()
        .map_err(|err| format!("invalid del {name} `{value}`: {err}"))
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

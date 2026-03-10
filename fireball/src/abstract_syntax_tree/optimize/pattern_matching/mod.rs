//! Pattern matching constraints:
//! - Do not hardcode AST text specializations like `some([])` or `block([])`.
//! - AST/IR matching must rely on generic `...` wildcard semantics.
//! - Keep AST/IR pattern payloads typed; avoid adding `source: String` mirrors.

mod apply;
pub(crate) mod embedded;
mod fb_gz;
mod fb_parser;
mod fbz;
mod hashing;
mod ir_parser;
mod predefined_pattern;
mod rhai_types;
pub(crate) mod stmt_pattern;

use crate::{abstract_syntax_tree::AstStatement, ir::statements::IrStatement};
pub(in crate::abstract_syntax_tree::optimize) use apply::apply_patterns;
pub use fb_parser::{
    parse_editable_asm_to_ir_statements, parse_editable_ast_statement, parse_editable_ir_statement,
};
pub(super) use hashing::{Blake3StdHasher, hash_statement_list};
use rhai::AST as RhaiAst;
use std::{fs, hash::Hash, path::Path};

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
    fn from_parse_result(
        name: String,
        origin: AstPatternOrigin,
        pattern: String,
        parse_result: Result<AstPatternRule, String>,
    ) -> Self {
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
            origin,
            input_type,
            pattern,
            parsed,
        }
    }

    fn canonical_source_text(&self) -> Result<String, String> {
        let pattern_trimmed = self.pattern.trim();
        if !pattern_trimmed.is_empty() && fs::metadata(pattern_trimmed).is_ok() {
            return fb_parser::load_pattern_source_from_path(pattern_trimmed);
        }
        match self.origin {
            AstPatternOrigin::UserInput => Ok(self.pattern.clone()),
            AstPatternOrigin::File => fb_parser::load_pattern_source_from_path(pattern_trimmed),
            AstPatternOrigin::PreDefined => Err(format!(
                "pattern `{}` does not retain canonical source text for compressed export",
                self.name
            )),
        }
    }

    // NOTE: stable Rust cannot hold a non-empty `Vec<Self>` in a `const`.
    // Empty here acts as a sentinel, resolved to embedded predefined patterns.
    pub const ALL: Vec<Self> = vec![];

    pub fn new(name: impl Into<String>, pattern: impl Into<String>) -> Self {
        let name = name.into();
        let pattern = pattern.into();
        let pattern_trimmed = pattern.trim();
        let parse_result = if fs::metadata(pattern_trimmed).is_ok() {
            fb_parser::load_file_pattern_rule_uncached(pattern_trimmed)
        } else {
            fb_parser::parse_pattern_file(&name, &pattern)
        };
        Self::from_parse_result(name, AstPatternOrigin::UserInput, pattern, parse_result)
    }

    pub fn from_file(path: impl Into<String>) -> Self {
        let path = path.into();
        let parse_result = fb_parser::load_file_pattern_rule_uncached(&path);
        Self::from_parse_result(path.clone(), AstPatternOrigin::File, path, parse_result)
    }

    pub fn from_fbz_file(path: impl Into<String>) -> Self {
        Self::from_file(path)
    }

    pub fn from_fb_gz_file(path: impl Into<String>) -> Self {
        Self::from_file(path)
    }

    pub fn from_fbz_bytes(name: impl Into<String>, bytes: &[u8]) -> Self {
        let name = name.into();
        match fb_parser::load_pattern_rule_from_fbz_bytes(&name, bytes) {
            Ok((rule, source)) => {
                Self::from_parse_result(name, AstPatternOrigin::UserInput, source, Ok(rule))
            }
            Err(err) => {
                Self::from_parse_result(name, AstPatternOrigin::UserInput, String::new(), Err(err))
            }
        }
    }

    pub fn from_fb_gz_bytes(name: impl Into<String>, bytes: &[u8]) -> Self {
        let name = name.into();
        match fb_parser::load_pattern_rule_from_fb_gz_bytes(&name, bytes) {
            Ok((rule, source)) => {
                Self::from_parse_result(name, AstPatternOrigin::UserInput, source, Ok(rule))
            }
            Err(err) => {
                Self::from_parse_result(name, AstPatternOrigin::UserInput, String::new(), Err(err))
            }
        }
    }

    pub fn fbz_bytes_from_source(source: &str) -> Result<Vec<u8>, String> {
        fb_parser::encode_pattern_source_to_fbz_bytes(source)
    }

    pub fn fb_gz_bytes_from_source(source: &str) -> Result<Vec<u8>, String> {
        fb_parser::encode_pattern_source_to_fb_gz_bytes(source)
    }

    pub fn to_fbz_bytes(&self) -> Result<Vec<u8>, String> {
        let source = self.canonical_source_text()?;
        fb_parser::encode_pattern_source_to_fbz_bytes(&source)
    }

    pub fn to_fb_gz_bytes(&self) -> Result<Vec<u8>, String> {
        let source = self.canonical_source_text()?;
        fb_parser::encode_pattern_source_to_fb_gz_bytes(&source)
    }

    pub fn write_fbz_file(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let source = self.canonical_source_text()?;
        fb_parser::write_pattern_source_to_fbz_file(path.as_ref(), &source)
    }

    pub fn write_fb_gz_file(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let source = self.canonical_source_text()?;
        fb_parser::write_pattern_source_to_fb_gz_file(path.as_ref(), &source)
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
        predefined_pattern::predefined_patterns()
    }

    pub fn predefined_pattern(name: &str) -> Option<Self> {
        predefined_pattern::predefined_pattern(name)
    }

    pub(super) fn from_predefined_include(name: &str, source: &str) -> Self {
        let (parsed, input_type) = match fb_parser::parse_pattern_file(name, source) {
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
struct AstPatternSkippedMatch {
    clause_group_index: usize,
    matched: AstPatternMatch,
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
    clause_groups: Vec<AstPatternClauseGroup>,
}

#[derive(Debug, Clone, Default)]
struct AstPatternClauseGroup {
    in_blocks: Vec<Vec<AstPatternInBlock>>,
    out_actions: Vec<AstPatternOutAction>,
}

impl AstPatternClauseGroup {
    fn is_empty(&self) -> bool {
        self.in_blocks.is_empty() && self.out_actions.is_empty()
    }
}

impl AstPatternRule {
    fn push_clause_group(&mut self, group: AstPatternClauseGroup) {
        self.in_blocks.extend(group.in_blocks.iter().cloned());
        self.clause_groups.push(group);
    }
}

#[derive(Debug, Clone)]
pub enum AstPatternInBlock {
    At(AstPatternApplyAt),
    Asm(AstPatternAsmData),
    AsmContains(String),
    Ast(AstPatternAstData),
    AstSequence(Vec<AstPatternAstData>),
    Ir(AstPatternIrData),
    Stmt(stmt_pattern::PatTree, Vec<stmt_pattern::WherePredicate>),
    StmtSeq(
        Vec<stmt_pattern::PatTree>,
        Vec<stmt_pattern::WherePredicate>,
    ),
    Expr(stmt_pattern::PatTree, Vec<stmt_pattern::WherePredicate>),
    Script(AstPatternScript),
    SkipRange(AstPatternRange),
    SkipAsmRange(AstPatternRange),
    SkipAstRange(AstPatternRange),
    SkipIrRange(AstPatternRange),
    IgnoreAsm(Option<AstPatternAsmData>), // None = ignore all asm, Some = ignore specific
    IgnoreIr(Option<AstPatternIrData>),   // None = ignore all ir, Some = ignore specific
    IgnoreAst(Option<AstPatternAstData>), // None = ignore all ast, Some = ignore specific
    IgnoreComment(IgnoreCommentFilter),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AstPatternInBlockKind {
    At,
    Asm,
    AsmContains,
    Ast,
    Stmt,
    StmtSeq,
    Expr,
    Ir,
    Script,
    SkipRange,
    SkipAsmRange,
    SkipAstRange,
    SkipIrRange,
    IgnoreAsm,
    IgnoreIr,
    IgnoreAst,
    IgnoreComment,
}

impl AstPatternInBlock {
    fn kind(&self) -> AstPatternInBlockKind {
        match self {
            Self::At(_) => AstPatternInBlockKind::At,
            Self::Asm(_) => AstPatternInBlockKind::Asm,
            Self::AsmContains(_) => AstPatternInBlockKind::AsmContains,
            Self::Ast(_) => AstPatternInBlockKind::Ast,
            Self::AstSequence(_) => AstPatternInBlockKind::Ast,
            Self::Stmt(_, _) => AstPatternInBlockKind::Stmt,
            Self::StmtSeq(_, _) => AstPatternInBlockKind::StmtSeq,
            Self::Expr(_, _) => AstPatternInBlockKind::Expr,
            Self::Ir(_) => AstPatternInBlockKind::Ir,
            Self::Script(_) => AstPatternInBlockKind::Script,
            Self::SkipRange(_) => AstPatternInBlockKind::SkipRange,
            Self::SkipAsmRange(_) => AstPatternInBlockKind::SkipAsmRange,
            Self::SkipAstRange(_) => AstPatternInBlockKind::SkipAstRange,
            Self::SkipIrRange(_) => AstPatternInBlockKind::SkipIrRange,
            Self::IgnoreAsm(_) => AstPatternInBlockKind::IgnoreAsm,
            Self::IgnoreIr(_) => AstPatternInBlockKind::IgnoreIr,
            Self::IgnoreAst(_) => AstPatternInBlockKind::IgnoreAst,
            Self::IgnoreComment(_) => AstPatternInBlockKind::IgnoreComment,
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
            AstPatternInBlock::Asm(_) | AstPatternInBlock::AsmContains(_) => has_asm = true,
            AstPatternInBlock::Ast(_)
            | AstPatternInBlock::AstSequence(_)
            | AstPatternInBlock::Stmt(_, _)
            | AstPatternInBlock::StmtSeq(_, _)
            | AstPatternInBlock::Expr(_, _) => has_ast = true,
            AstPatternInBlock::Ir(_) => has_ir = true,
            AstPatternInBlock::Script(_) => has_script = true,
            AstPatternInBlock::SkipRange(_)
            | AstPatternInBlock::SkipAsmRange(_)
            | AstPatternInBlock::SkipAstRange(_)
            | AstPatternInBlock::SkipIrRange(_) => {}
            AstPatternInBlock::IgnoreAsm(_)
            | AstPatternInBlock::IgnoreIr(_)
            | AstPatternInBlock::IgnoreAst(_)
            | AstPatternInBlock::IgnoreComment(_) => {}
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

fn block_asm_contains(clauses: &[AstPatternInBlock]) -> Option<&str> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::AsmContains(value) => Some(value.as_str()),
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

fn block_ast_sequence(clauses: &[AstPatternInBlock]) -> Option<&[AstPatternAstData]> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::AstSequence(value) => Some(value.as_slice()),
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

fn block_stmt(
    clauses: &[AstPatternInBlock],
) -> Option<(&stmt_pattern::PatTree, &[stmt_pattern::WherePredicate])> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::Stmt(pat, preds) => Some((pat, preds.as_slice())),
        _ => None,
    })
}

fn block_stmt_seq(
    clauses: &[AstPatternInBlock],
) -> Option<(&[stmt_pattern::PatTree], &[stmt_pattern::WherePredicate])> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::StmtSeq(pats, preds) => Some((pats.as_slice(), preds.as_slice())),
        _ => None,
    })
}

fn block_expr(
    clauses: &[AstPatternInBlock],
) -> Option<(&stmt_pattern::PatTree, &[stmt_pattern::WherePredicate])> {
    clauses.iter().find_map(|clause| match clause {
        AstPatternInBlock::Expr(pat, preds) => Some((pat, preds.as_slice())),
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

fn block_ignore_asm_filters(clauses: &[AstPatternInBlock]) -> Vec<Option<&AstPatternAsmData>> {
    clauses
        .iter()
        .filter_map(|clause| match clause {
            AstPatternInBlock::IgnoreAsm(filter) => Some(filter.as_ref()),
            _ => None,
        })
        .collect()
}

fn block_ignore_ir_filters(clauses: &[AstPatternInBlock]) -> Vec<Option<&AstPatternIrData>> {
    clauses
        .iter()
        .filter_map(|clause| match clause {
            AstPatternInBlock::IgnoreIr(filter) => Some(filter.as_ref()),
            _ => None,
        })
        .collect()
}

fn block_ignore_ast_filters(clauses: &[AstPatternInBlock]) -> Vec<Option<&AstPatternAstData>> {
    clauses
        .iter()
        .filter_map(|clause| match clause {
            AstPatternInBlock::IgnoreAst(filter) => Some(filter.as_ref()),
            _ => None,
        })
        .collect()
}

fn block_ignore_comment_filters(clauses: &[AstPatternInBlock]) -> Vec<&IgnoreCommentFilter> {
    clauses
        .iter()
        .filter_map(|clause| match clause {
            AstPatternInBlock::IgnoreComment(filter) => Some(filter),
            _ => None,
        })
        .collect()
}

#[derive(Debug, Clone)]
pub enum IgnoreCommentFilter {
    All,
    StartsWith(String),
    EndsWith(String),
    Contains(String),
}

impl IgnoreCommentFilter {
    pub(super) fn matches_comment(&self, comment: &str) -> bool {
        let normalized = comment.trim().to_lowercase();
        match self {
            Self::All => true,
            Self::StartsWith(prefix) => normalized.starts_with(&prefix.to_lowercase()),
            Self::EndsWith(suffix) => normalized.ends_with(&suffix.to_lowercase()),
            Self::Contains(needle) => {
                let needle_lower = needle.to_lowercase();
                memchr::memmem::find(normalized.as_bytes(), needle_lower.as_bytes()).is_some()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstPatternOutAction {
    ReplaceAsm(AstPatternAsmData),
    ReplaceIr(AstPatternIrReplacement),
    ReplaceAst(AstStatement),
    Delete(AstPatternDeleteTarget),
    SpliceBlock,
    Script(AstPatternScript),
    Emit(stmt_pattern::PatTree),
    EmitBefore(stmt_pattern::PatTree),
    EmitAfter(stmt_pattern::PatTree),
    ReplaceExpr(stmt_pattern::PatTree),
    /// Built-in function applied to captures for expression replacement.
    /// Currently supports: `eval_binop($op, $a, $b)`, `eval_unary($op, $a)`.
    ReplaceExprBuiltin {
        func: String,
        args: Vec<String>,
    },
    Log(AstPatternLogLevel, String),
    PruneEmptyElse,
    ClearIgnore(ClearIgnoreTarget),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearIgnoreTarget {
    All,
    Asm,
    Ir,
    Ast,
    Comment,
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
        let statement = ir_parser::parse_ir_statement(replacement);
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
    pub fn from_text(text: &str) -> Result<Self, String> {
        let value = text.trim();
        if value.is_empty() {
            return Err("asm text cannot be empty".to_string());
        }
        let statement = ir_parser::parse_asm_statement(value)
            .ok_or_else(|| format!("unsupported asm pattern `{value}`"))?;
        Ok(Self {
            source: value.to_string(),
            statement: AstStatement::Ir(Box::new(statement)),
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
    Label(Option<Box<[u8]>>),
    BlockAny,
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
            matcher: ir_parser::compile_ast_matcher(value),
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
                    if ir_parser::normalized_comment_contains(comment, expected.as_ref())
            ),
            AstPatternAstMatcher::Label(expected) => {
                matches!(
                    statement,
                    AstStatement::Label(name)
                    if match expected {
                        None => true,
                        Some(expected_normalized) =>
                            ir_parser::normalize_for_match(name).as_bytes() == expected_normalized.as_ref(),
                    }
                )
            }
            AstPatternAstMatcher::BlockAny => matches!(statement, AstStatement::Block(_)),
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
            matcher: ir_parser::compile_ir_matcher(value),
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

#[derive(Debug, Clone)]
pub struct AstPatternScript {
    pub compiled: RhaiAst,
}
impl AstPatternScript {
    pub fn from_source(script: impl Into<String>) -> Result<Self, String> {
        let source = script.into().trim().to_string();
        if source.is_empty() {
            return Err("script must not be empty".to_string());
        }
        let compiled = apply::compiled_script(&source)?;
        Ok(Self { compiled })
    }

    fn compiled(&self) -> RhaiAst {
        self.compiled.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstPatternRange {
    pub start: usize,
    pub end_exclusive: usize,
}

#[cfg(test)]
mod tests;

#[cfg(test)]
impl AstPatternRule {
    pub fn clause_groups(&self) -> &[AstPatternClauseGroup] {
        &self.clause_groups
    }
}

#[cfg(test)]
impl AstPatternClauseGroup {
    pub fn in_blocks(&self) -> &[Vec<AstPatternInBlock>] {
        &self.in_blocks
    }
    pub fn out_actions(&self) -> &[AstPatternOutAction] {
        &self.out_actions
    }
}

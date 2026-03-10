use super::{fb_parser::parse_pattern_file, *};

fn parse(content: &str) -> AstPatternRule {
    parse_pattern_file("test", content).expect("parse failed")
}

fn parse_err(content: &str) -> String {
    parse_pattern_file("test", content).unwrap_err()
}

fn first_group(rule: &AstPatternRule) -> &AstPatternClauseGroup {
    &rule.clause_groups()[0]
}

fn first_in_block(rule: &AstPatternRule) -> &[AstPatternInBlock] {
    &first_group(rule).in_blocks()[0]
}

fn find_kind(blocks: &[AstPatternInBlock], kind: AstPatternInBlockKind) -> bool {
    blocks.iter().any(|b| b.kind() == kind)
}

// ── Phase targeting (at) ──

#[test]
fn parse_at_before_ir_analyzation() {
    let rule = parse("if:\n  at beforeIrAnalyzation\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(blocks.iter().any(|b| matches!(
        b,
        AstPatternInBlock::At(AstPatternApplyAt::Phase(
            AstPatternApplyPhase::BeforeIrAnalyzation
        ))
    )));
}

#[test]
fn parse_at_after_iteration() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(blocks.iter().any(|b| matches!(
        b,
        AstPatternInBlock::At(AstPatternApplyAt::Phase(
            AstPatternApplyPhase::AfterIteration
        ))
    )));
}

#[test]
fn parse_at_after_optimization() {
    let rule = parse("if:\n  at afterOptimization\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(blocks.iter().any(|b| matches!(
        b,
        AstPatternInBlock::At(AstPatternApplyAt::Phase(
            AstPatternApplyPhase::AfterOptimization
        ))
    )));
}

#[test]
fn parse_at_after_ir_analyzation() {
    let rule = parse("if:\n  at afterIrAnalyzation\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(blocks.iter().any(|b| matches!(
        b,
        AstPatternInBlock::At(AstPatternApplyAt::Phase(
            AstPatternApplyPhase::AfterIrAnalyzation
        ))
    )));
}

#[test]
fn parse_at_after_parameter_analyzation() {
    let rule = parse("if:\n  at afterParameterAnalyzation\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(blocks.iter().any(|b| matches!(
        b,
        AstPatternInBlock::At(AstPatternApplyAt::Phase(
            AstPatternApplyPhase::AfterParameterAnalyzation
        ))
    )));
}

#[test]
fn parse_at_after_call_argument_analyzation() {
    let rule = parse("if:\n  at afterCallArgumentAnalyzation\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(blocks.iter().any(|b| matches!(
        b,
        AstPatternInBlock::At(AstPatternApplyAt::Phase(
            AstPatternApplyPhase::AfterCallArgumentAnalyzation
        ))
    )));
}

// ── asm ──

#[test]
fn parse_asm_condition() {
    let rule = parse("if:\n  at afterIteration\n  asm push rbp\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Asm));
}

#[test]
fn parse_asm_backtick_multiline() {
    let rule = parse("if:\n  at afterIteration\n  asm `mov rsp, 8; push rbp`\ndo:\n  info(\"ok\")");
    assert!(!rule.in_blocks.is_empty());
}

// ── asm_contains ──

#[test]
fn parse_asm_contains() {
    let rule =
        parse("if:\n  at afterIteration\n  asm_contains __stack_chk_fail\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(
        blocks
            .iter()
            .any(|b| matches!(b, AstPatternInBlock::AsmContains(s) if s == "__stack_chk_fail"))
    );
}

// ── ast matchers ──

#[test]
fn parse_ast_return() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Ast));
}

#[test]
fn parse_ast_comment() {
    let rule = parse("if:\n  at afterIteration\n  ast comment test-marker\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Ast));
}

#[test]
fn parse_ast_empty() {
    let rule = parse("if:\n  at afterIteration\n  ast empty\ndo:\n  info(\"ok\")");
    assert!(!rule.in_blocks.is_empty());
}

#[test]
fn parse_ast_undefined() {
    let rule = parse("if:\n  at afterIteration\n  ast undefined\ndo:\n  info(\"ok\")");
    assert!(!rule.in_blocks.is_empty());
}

#[test]
fn parse_ast_label() {
    let rule = parse("if:\n  at afterIteration\n  ast label\ndo:\n  info(\"ok\")");
    assert!(!rule.in_blocks.is_empty());
}

#[test]
fn parse_ast_label_text() {
    let rule = parse("if:\n  at afterIteration\n  ast label test_label\ndo:\n  info(\"ok\")");
    assert!(!rule.in_blocks.is_empty());
}

#[test]
fn parse_ast_block_wildcard() {
    let rule = parse("if:\n  at afterIteration\n  ast Block(...)\ndo:\n  info(\"ok\")");
    assert!(!rule.in_blocks.is_empty());
}

#[test]
fn parse_ast_block_empty() {
    let rule = parse("if:\n  at afterIteration\n  ast Block([])\ndo:\n  info(\"ok\")");
    assert!(!rule.in_blocks.is_empty());
}

#[test]
fn parse_ast_some_empty() {
    let rule = parse("if:\n  at afterIteration\n  ast Some([])\ndo:\n  info(\"ok\")");
    assert!(!rule.in_blocks.is_empty());
}

#[test]
fn parse_ast_if_wildcard() {
    let rule = parse("if:\n  at afterIteration\n  ast If(...)\ndo:\n  info(\"ok\")");
    assert!(!rule.in_blocks.is_empty());
}

// ── ir matchers ──

#[test]
fn parse_ir_undefined() {
    let rule = parse("if:\n  at afterIteration\n  ir undefined\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Ir));
}

#[test]
fn parse_ir_halt() {
    let rule = parse("if:\n  at afterIteration\n  ir halt\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Ir));
}

// ── script ──

#[test]
fn parse_script_inline() {
    let rule = parse("if:\n  at afterIteration\n  script `stmt_count >= 1`\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Script));
}

#[test]
fn parse_script_multiline() {
    let rule = parse(
        "if:\n  at afterIteration\n  script `\nstmt_count >= 1 && asm_count >= 0\n`\ndo:\n  info(\"ok\")",
    );
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Script));
}

#[test]
fn parse_script_precompiled_no_source() {
    let script = AstPatternScript::from_source("true").unwrap();
    // compiled field is always populated, no source string stored
    let _ = script.compiled();
}

// ── stmt ──

#[test]
fn parse_stmt_pattern() {
    let rule = parse("if:\n  at afterIteration\n  stmt Return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Stmt));
}

#[test]
fn parse_stmt_with_capture() {
    let rule = parse(
        "if:\n  at afterIteration\n  stmt If(UnaryOp(Not, $cond), $then, Some($else))\ndo:\n  emit If($cond, $else, Some($then))",
    );
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Stmt));
}

// ── stmt with where predicates ──

#[test]
fn parse_stmt_where_greater_count() {
    let rule = parse(
        "if:\n  at afterIteration\n  stmt If(UnaryOp(Not, $cond), $then, Some($else))\n  where greater_count($else, $then)\ndo:\n  emit If($cond, $else, Some($then))",
    );
    let blocks = first_in_block(&rule);
    let stmt_block = blocks
        .iter()
        .find(|b| b.kind() == AstPatternInBlockKind::Stmt)
        .unwrap();
    match stmt_block {
        AstPatternInBlock::Stmt(_, preds) => assert_eq!(preds.len(), 1),
        _ => panic!("expected Stmt"),
    }
}

#[test]
fn parse_stmt_where_stmt_list_helpers() {
    let rule = parse(
        "if:\n  at afterIteration\n  stmt Block($body)\n  where is_empty_stmt_list($body)\n  where is_nonempty_stmt_list($body)\n  where ends_with_break($body)\n  where ends_with_return($body)\ndo:\n  emit Block($body)",
    );
    let blocks = first_in_block(&rule);
    let stmt_block = blocks
        .iter()
        .find(|b| b.kind() == AstPatternInBlockKind::Stmt)
        .unwrap();
    match stmt_block {
        AstPatternInBlock::Stmt(_, preds) => assert_eq!(preds.len(), 4),
        _ => panic!("expected Stmt"),
    }
}

#[test]
fn parse_stmt_where_tail_shape_helpers() {
    let rule = parse(
        "if:\n  at afterIteration\n  stmt While($cond, $body)\n  where ends_with_continue($body)\n  where is_end_if_not_cond_break($body)\n  where is_end_if_cond_else_break($body)\ndo:\n  emit While($cond, $body)",
    );
    let blocks = first_in_block(&rule);
    let stmt_block = blocks
        .iter()
        .find(|b| b.kind() == AstPatternInBlockKind::Stmt)
        .unwrap();
    match stmt_block {
        AstPatternInBlock::Stmt(_, preds) => assert_eq!(preds.len(), 3),
        _ => panic!("expected Stmt"),
    }
}

#[test]
fn parse_stmt_multiple_where() {
    let rule = parse(
        "if:\n  at afterIteration\n  expr BinaryOp(BitOr, $x, $y)\n  where structurally_equal($x, $y)\n  where is_nonzero($x)\ndo:\n  info(\"ok\")",
    );
    let blocks = first_in_block(&rule);
    let expr_block = blocks
        .iter()
        .find(|b| b.kind() == AstPatternInBlockKind::Expr)
        .unwrap();
    match expr_block {
        AstPatternInBlock::Expr(_, preds) => assert_eq!(preds.len(), 2),
        _ => panic!("expected Expr"),
    }
}

// ── expr ──

#[test]
fn parse_expr_pattern() {
    let rule = parse(
        "if:\n  at afterIteration\n  expr BinaryOp(Add, $x, Literal(Int(0)))\ndo:\n  replace_expr $x",
    );
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Expr));
}

// ── stmt_seq ──

#[test]
fn parse_stmt_seq() {
    let rule = parse(
        "if:\n  at afterIteration\n  stmt_seq [Call($f), Return(None)]\ndo:\n  emit Return(Some(Call($f)))",
    );
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::StmtSeq));
}

// ── skip ranges ──

#[test]
fn parse_skip_alias_global() {
    let rule = parse("if:\n  at afterIteration\n  skip 0..8\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    let skip = blocks
        .iter()
        .find(|b| b.kind() == AstPatternInBlockKind::SkipRange)
        .unwrap();
    match skip {
        AstPatternInBlock::SkipRange(r) => {
            assert_eq!(r.start, 0);
            assert_eq!(r.end_exclusive, 8);
        }
        _ => panic!("expected SkipRange"),
    }
}

fn parse_skip_asm_alias() {
    let rule =
        parse("if:\n  at afterIteration\n  skip asm 0..8\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::SkipAsmRange));
}

#[test]
fn parse_skip_ast_alias() {
    let rule =
        parse("if:\n  at afterIteration\n  skip ast 0..8\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::SkipAstRange));
}

#[test]
fn parse_skip_ir_alias() {
    let rule = parse("if:\n  at afterIteration\n  skip ir 0..8\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::SkipIrRange));
}

// ── ignore asm ──

#[test]
fn parse_ignore_asm_all() {
    let rule = parse("if:\n  at afterIteration\n  ignore asm\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(
        blocks
            .iter()
            .any(|b| matches!(b, AstPatternInBlock::IgnoreAsm(None)))
    );
}

#[test]
fn parse_ignore_asm_specific() {
    let rule =
        parse("if:\n  at afterIteration\n  ignore asm push rbp\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(
        blocks
            .iter()
            .any(|b| matches!(b, AstPatternInBlock::IgnoreAsm(Some(_))))
    );
}

// ── ignore ir ──

#[test]
fn parse_ignore_ir_all() {
    let rule = parse("if:\n  at afterIteration\n  ignore ir\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(
        blocks
            .iter()
            .any(|b| matches!(b, AstPatternInBlock::IgnoreIr(None)))
    );
}

// ── ignore ast ──

#[test]
fn parse_ignore_ast_all() {
    let rule = parse("if:\n  at afterIteration\n  ignore ast\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(
        blocks
            .iter()
            .any(|b| matches!(b, AstPatternInBlock::IgnoreAst(None)))
    );
}

// ── ignore comment ──

#[test]
fn parse_ignore_comment_all() {
    let rule =
        parse("if:\n  at afterIteration\n  ignore comment\n  ast return\ndo:\n  info(\"ok\")");
    let blocks = first_in_block(&rule);
    assert!(blocks.iter().any(|b| matches!(
        b,
        AstPatternInBlock::IgnoreComment(IgnoreCommentFilter::All)
    )));
}

#[test]
fn parse_ignore_commentstart() {
    let rule = parse(
        "if:\n  at afterIteration\n  ignore commentstart prefix_text\n  ast return\ndo:\n  info(\"ok\")",
    );
    let blocks = first_in_block(&rule);
    assert!(blocks.iter().any(|b| matches!(b, AstPatternInBlock::IgnoreComment(IgnoreCommentFilter::StartsWith(s)) if s == "prefix_text")));
}

#[test]
fn parse_ignore_commentend() {
    let rule = parse(
        "if:\n  at afterIteration\n  ignore commentend suffix_text\n  ast return\ndo:\n  info(\"ok\")",
    );
    let blocks = first_in_block(&rule);
    assert!(blocks.iter().any(|b| matches!(b, AstPatternInBlock::IgnoreComment(IgnoreCommentFilter::EndsWith(s)) if s == "suffix_text")));
}

#[test]
fn parse_ignore_commentcontains() {
    let rule = parse(
        "if:\n  at afterIteration\n  ignore commentcontains needle\n  ast return\ndo:\n  info(\"ok\")",
    );
    let blocks = first_in_block(&rule);
    assert!(blocks.iter().any(|b| matches!(b, AstPatternInBlock::IgnoreComment(IgnoreCommentFilter::Contains(s)) if s == "needle")));
}

// ── IgnoreCommentFilter matching ──

#[test]
fn ignore_comment_filter_all() {
    assert!(IgnoreCommentFilter::All.matches_comment("anything"));
}

#[test]
fn ignore_comment_filter_starts_with() {
    let f = IgnoreCommentFilter::StartsWith("prefix".to_string());
    assert!(f.matches_comment("Prefix text here"));
    assert!(!f.matches_comment("no prefix here"));
}

#[test]
fn ignore_comment_filter_ends_with() {
    let f = IgnoreCommentFilter::EndsWith("suffix".to_string());
    assert!(f.matches_comment("text ends with Suffix"));
    assert!(!f.matches_comment("suffix not at end!"));
}

#[test]
fn ignore_comment_filter_contains() {
    let f = IgnoreCommentFilter::Contains("needle".to_string());
    assert!(f.matches_comment("hay Needle stack"));
    assert!(!f.matches_comment("not here"));
}

// ── do: actions ──

// ── do: asm ──

#[test]
fn parse_do_asm() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  asm mov esp, 8");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ReplaceAsm(_)))
    );
}

// ── do: ir ──

#[test]
fn parse_do_ir() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  ir halt");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ReplaceIr(_)))
    );
}

// ── do: ast ──

#[test]
fn parse_do_ast() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  ast asm push rbp");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ReplaceAst(_)))
    );
}

#[test]
fn parse_do_ast_comment() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  ast comment test");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ReplaceAst(AstStatement::Comment(_))))
    );
}

// ── do: emit / emit_before / emit_after ──

#[test]
fn parse_do_emit() {
    let rule = parse("if:\n  at afterIteration\n  stmt Return\ndo:\n  emit Comment(test)");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::Emit(_)))
    );
}

#[test]
fn parse_do_emit_before() {
    let rule = parse("if:\n  at afterIteration\n  stmt Return\ndo:\n  emit_before Comment(seed)");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::EmitBefore(_)))
    );
}

#[test]
fn parse_do_emit_after() {
    let rule = parse("if:\n  at afterIteration\n  stmt Return\ndo:\n  emit_after Comment(seed)");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::EmitAfter(_)))
    );
}

// ── do: replace_expr ──

#[test]
fn parse_do_replace_expr() {
    let rule = parse(
        "if:\n  at afterIteration\n  expr BinaryOp(Add, $x, Literal(Int(0)))\ndo:\n  replace_expr $x",
    );
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ReplaceExpr(_)))
    );
}

// ── do: replace_expr_fn ──

#[test]
fn parse_do_replace_expr_fn() {
    let rule = parse(
        "if:\n  at afterIteration\n  expr BinaryOp(BitOr, $x, $y)\ndo:\n  replace_expr_fn eval_rotate_right($x, $n)",
    );
    let actions = first_group(&rule).out_actions();
    assert!(actions.iter().any(|a| matches!(a, AstPatternOutAction::ReplaceExprBuiltin { func, args } if func == "eval_rotate_right" && args.len() == 2)));
}

// ── do: script ──

#[test]
fn parse_do_script() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  script `true`");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::Script(_)))
    );
}

// ── do: splice-block ──

#[test]
fn parse_do_splice_block() {
    let rule = parse("if:\n  at afterIteration\n  ast Block(...)\ndo:\n  splice-block");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::SpliceBlock))
    );
}

// ── do: prune-empty-else ──

#[test]
fn parse_do_prune_empty_else() {
    let rule = parse("if:\n  at afterOptimization\n  ast Some([])\ndo:\n  prune-empty-else");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::PruneEmptyElse))
    );
}

// ── do: del ──

#[test]
fn parse_do_del_index() {
    let rule =
        parse("if:\n  at afterIteration\n  stmt_seq [Comment($x), Return]\ndo:\n  del start[0]");
    let actions = first_group(&rule).out_actions();
    assert!(actions.iter().any(|a| matches!(
        a,
        AstPatternOutAction::Delete(AstPatternDeleteTarget::Index {
            anchor: AstPatternDeleteAnchor::Start,
            offset: 0,
        })
    )));
}

#[test]
fn parse_do_del_range() {
    let rule =
        parse("if:\n  at afterIteration\n  stmt_seq [Comment($x), Return]\ndo:\n  del start[0..2]");
    let actions = first_group(&rule).out_actions();
    assert!(actions.iter().any(|a| matches!(
        a,
        AstPatternOutAction::Delete(AstPatternDeleteTarget::Range {
            anchor: AstPatternDeleteAnchor::Start,
            start_offset: 0,
            end_offset_exclusive: 2,
        })
    )));
}

#[test]
fn parse_do_del_end_anchor() {
    let rule =
        parse("if:\n  at afterIteration\n  stmt_seq [Comment($x), Return]\ndo:\n  del end[0]");
    let actions = first_group(&rule).out_actions();
    assert!(actions.iter().any(|a| matches!(
        a,
        AstPatternOutAction::Delete(AstPatternDeleteTarget::Index {
            anchor: AstPatternDeleteAnchor::End,
            ..
        })
    )));
}

// ── do: log actions ──

#[test]
fn parse_do_log_info() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  info(\"hello\")");
    let actions = first_group(&rule).out_actions();
    assert!(actions.iter().any(
        |a| matches!(a, AstPatternOutAction::Log(AstPatternLogLevel::Info, msg) if msg == "hello")
    ));
}

#[test]
fn parse_do_log_warn() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  warn(\"hello\")");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::Log(AstPatternLogLevel::Warn, _)))
    );
}

#[test]
fn parse_do_log_error() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  error(\"hello\")");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::Log(AstPatternLogLevel::Error, _)))
    );
}

#[test]
fn parse_do_log_debug() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  debug(\"hello\")");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::Log(AstPatternLogLevel::Debug, _)))
    );
}

#[test]
fn parse_do_log_trace() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  trace(\"hello\")");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::Log(AstPatternLogLevel::Trace, _)))
    );
}

// ── do: !ignore (clear ignore) ──

#[test]
fn parse_do_clear_ignore_all() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  !ignore\n  info(\"ok\")");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ClearIgnore(ClearIgnoreTarget::All)))
    );
}

#[test]
fn parse_do_clear_ignore_asm() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  !ignore asm\n  info(\"ok\")");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ClearIgnore(ClearIgnoreTarget::Asm)))
    );
}

#[test]
fn parse_do_clear_ignore_ir() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  !ignore ir\n  info(\"ok\")");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ClearIgnore(ClearIgnoreTarget::Ir)))
    );
}

#[test]
fn parse_do_clear_ignore_ast() {
    let rule = parse("if:\n  at afterIteration\n  ast return\ndo:\n  !ignore ast\n  info(\"ok\")");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ClearIgnore(ClearIgnoreTarget::Ast)))
    );
}

#[test]
fn parse_do_clear_ignore_comment() {
    let rule =
        parse("if:\n  at afterIteration\n  ast return\ndo:\n  !ignore comment\n  info(\"ok\")");
    let actions = first_group(&rule).out_actions();
    assert!(actions.iter().any(|a| matches!(
        a,
        AstPatternOutAction::ClearIgnore(ClearIgnoreTarget::Comment)
    )));
}

// ── Multi-clause (multiple if: before single do:) ──

#[test]
fn parse_multi_clause() {
    let content = "\
if:
  at afterIteration
  asm push rbp
if:
  at afterOptimization
  asm_contains __stack_chk_fail
do:
  info(\"multi\")";
    let rule = parse(content);
    let group = first_group(&rule);
    assert_eq!(group.in_blocks().len(), 2);
}

// ── Multiple clause groups (if:/do: then if:/do:) ──

#[test]
fn parse_multiple_clause_groups() {
    let content = "\
if:
  at afterIteration
  ast return
do:
  info(\"first\")
if:
  at afterOptimization
  ast comment marker
do:
  info(\"second\")";
    let rule = parse(content);
    assert_eq!(rule.clause_groups().len(), 2);
}

// ── Comments ──

#[test]
fn parse_comments_ignored() {
    let content = "\
# This is a comment
if:
  at afterIteration
  # another comment
  ast return
do:
  info(\"ok\")";
    let rule = parse(content);
    assert_eq!(rule.clause_groups().len(), 1);
}

// ── Error cases ──

#[test]
fn parse_err_do_before_if() {
    let err = parse_err("do:\n  info(\"oops\")");
    assert!(err.contains("do:") || err.contains("if:"));
}

#[test]
fn parse_err_empty_content() {
    let err = parse_err("");
    assert!(err.contains("no complete"));
}

#[test]
fn parse_err_unknown_if_directive() {
    let err = parse_err("if:\n  at afterIteration\n  unknown_directive value\ndo:\n  info(\"ok\")");
    assert!(err.contains("unknown"));
}

#[test]
fn parse_err_unknown_do_directive() {
    let err = parse_err("if:\n  at afterIteration\n  ast return\ndo:\n  unknown_action value");
    assert!(err.contains("unknown"));
}

#[test]
fn parse_err_invalid_phase() {
    let err = parse_err("if:\n  at badPhase\n  ast return\ndo:\n  info(\"ok\")");
    assert!(err.contains("invalid"));
}

#[test]
fn parse_err_empty_asm_contains() {
    let err = parse_err("if:\n  at afterIteration\n  asm_contains \ndo:\n  info(\"ok\")");
    assert!(err.contains("empty") || err.contains("must not"));
}

// ── Rhai script compilation ──

#[test]
fn rhai_script_from_source_valid() {
    let script = AstPatternScript::from_source("1 + 1").unwrap();
    let _ = script.compiled();
}

#[test]
fn rhai_script_from_source_empty() {
    let err = AstPatternScript::from_source("").unwrap_err();
    assert!(err.contains("empty"));
}

#[test]
fn rhai_script_from_source_whitespace() {
    let err = AstPatternScript::from_source("   \n  ").unwrap_err();
    assert!(err.contains("empty"));
}

#[test]
fn rhai_script_from_source_syntax_error() {
    let err = AstPatternScript::from_source("fn {{{").unwrap_err();
    assert!(err.contains("compile") || err.contains("failed"));
}

// ── Rhai types ──

#[test]
fn rhai_ast_stmt_kind() {
    use crate::abstract_syntax_tree::{AstStatementOrigin, WrappedAstStatement};
    let wrapped = WrappedAstStatement {
        statement: AstStatement::Comment("hello".to_string()),
        origin: AstStatementOrigin::Unknown,
        comment: None,
    };
    let rhai_stmt = rhai_types::RhaiAstStmt::from_wrapped(&wrapped);
    assert_eq!(rhai_stmt.kind, "comment");
}

#[test]
fn rhai_ast_stmt_all_kinds() {
    use crate::abstract_syntax_tree::{
        AstCall, AstExpression, AstStatementOrigin, AstValueOrigin, Wrapped, WrappedAstStatement,
    };
    let w = |s: AstStatement| -> WrappedAstStatement {
        WrappedAstStatement {
            statement: s,
            origin: AstStatementOrigin::Unknown,
            comment: None,
        }
    };
    let dummy_expr = || Wrapped {
        item: AstExpression::Unknown,
        origin: AstValueOrigin::Unknown,
        comment: None,
    };

    let cases: Vec<(AstStatement, &str)> = vec![
        (AstStatement::Empty, "empty"),
        (AstStatement::Undefined, "undefined"),
        (AstStatement::Break, "break"),
        (AstStatement::Continue, "continue"),
        (AstStatement::Comment("c".into()), "comment"),
        (AstStatement::Label("l".into()), "label"),
        (AstStatement::Return(None), "return"),
        (AstStatement::Block(vec![]), "block"),
        (
            AstStatement::Call(AstCall::Unknown("f".into(), vec![])),
            "call",
        ),
        (
            AstStatement::Assignment(dummy_expr(), dummy_expr()),
            "assignment",
        ),
        (
            AstStatement::Declaration(
                crate::abstract_syntax_tree::AstVariable {
                    name: None,
                    id: crate::abstract_syntax_tree::AstVariableId {
                        index: 0,
                        parent: None,
                    },
                    var_type: crate::abstract_syntax_tree::AstValueType::Unknown,
                    const_value: None,
                    data_access_ir: None,
                },
                None,
            ),
            "declaration",
        ),
        (AstStatement::While(dummy_expr(), vec![]), "while"),
        (
            AstStatement::For(
                Box::new(w(AstStatement::Empty)),
                dummy_expr(),
                Box::new(w(AstStatement::Empty)),
                vec![],
            ),
            "for",
        ),
        (AstStatement::DoWhile(dummy_expr(), vec![]), "dowhile"),
        (AstStatement::If(dummy_expr(), vec![], None), "if"),
        (AstStatement::Switch(dummy_expr(), vec![], None), "switch"),
    ];

    for (stmt, expected_kind) in cases {
        let rhai_stmt = rhai_types::RhaiAstStmt::from_wrapped(&w(stmt));
        assert_eq!(
            rhai_stmt.kind, expected_kind,
            "kind mismatch for {expected_kind}"
        );
    }
}

#[test]
fn rhai_ir_stmt_kinds() {
    use crate::ir::statements::IrStatement;
    let cases: Vec<(IrStatement, &str)> = vec![
        (IrStatement::Halt, "halt"),
        (IrStatement::Undefined, "undefined"),
    ];
    for (stmt, expected) in cases {
        let rhai_stmt = rhai_types::RhaiIrStmt::from_statement(&stmt);
        assert_eq!(rhai_stmt.kind, expected);
    }
}

#[test]
fn rhai_asm_line_parsing() {
    let line = rhai_types::RhaiAsmLine::from_normalized(0, "  mov eax, ebx  ");
    assert_eq!(line.mnemonic, "mov");
    assert_eq!(line.operands, "eax, ebx");
    assert_eq!(line.index, 0);
}

#[test]
fn rhai_asm_line_is_methods() {
    assert!(rhai_types::RhaiAsmLine::from_normalized(0, "call printf").mnemonic == "call");
    assert!(
        rhai_types::RhaiAsmLine::from_normalized(0, "jmp label")
            .mnemonic
            .starts_with('j')
    );
    assert!(rhai_types::RhaiAsmLine::from_normalized(0, "ret").mnemonic == "ret");
    assert!(rhai_types::RhaiAsmLine::from_normalized(0, "nop").mnemonic == "nop");
    assert!(rhai_types::RhaiAsmLine::from_normalized(0, "push rbp").mnemonic == "push");
    assert!(rhai_types::RhaiAsmLine::from_normalized(0, "pop rbp").mnemonic == "pop");
    assert!(rhai_types::RhaiAsmLine::from_normalized(0, "mov eax, 0").mnemonic == "mov");
    assert!(rhai_types::RhaiAsmLine::from_normalized(0, "movzx eax, al").mnemonic == "movzx");
}

// ── Rhai global analysis functions ──

#[test]
fn rhai_operator_classification() {
    use rhai_types::*;
    // These are module-private, test via the engine
    let engine = {
        let mut e = rhai::Engine::new();
        register_analysis_types(&mut e);
        e
    };
    assert_eq!(
        engine.eval::<bool>("is_arithmetic_op(\"Add\")").unwrap(),
        true
    );
    assert_eq!(
        engine.eval::<bool>("is_arithmetic_op(\"BitAnd\")").unwrap(),
        false
    );
    assert_eq!(
        engine.eval::<bool>("is_comparison_op(\"Less\")").unwrap(),
        true
    );
    assert_eq!(
        engine.eval::<bool>("is_comparison_op(\"Add\")").unwrap(),
        false
    );
    assert_eq!(
        engine.eval::<bool>("is_bitwise_op(\"BitXor\")").unwrap(),
        true
    );
    assert_eq!(
        engine.eval::<bool>("is_bitwise_op(\"Add\")").unwrap(),
        false
    );
    assert_eq!(
        engine.eval::<bool>("is_logical_op(\"LogicAnd\")").unwrap(),
        true
    );
    assert_eq!(
        engine.eval::<bool>("is_logical_op(\"Add\")").unwrap(),
        false
    );
}

// ── Strip inline comment ──

#[test]
fn strip_inline_comment_basic() {
    assert_eq!(fb_parser::strip_inline_comment("code # comment"), "code ");
    assert_eq!(fb_parser::strip_inline_comment("no comment"), "no comment");
    assert_eq!(
        fb_parser::strip_inline_comment("\"hash # inside\""),
        "\"hash # inside\""
    );
}

// ── Parse builtin call ──

#[test]
fn parse_builtin_call_valid() {
    let (func, args) = fb_parser::parse_builtin_call("eval_rotate_right($x, $n)").unwrap();
    assert_eq!(func, "eval_rotate_right");
    assert_eq!(args, vec!["x", "n"]);
}

#[test]
fn parse_builtin_call_three_args() {
    let (func, args) =
        fb_parser::parse_builtin_call("eval_strength_reduce_dual($x, $n, $m)").unwrap();
    assert_eq!(func, "eval_strength_reduce_dual");
    assert_eq!(args.len(), 3);
}

// ── String literal in patterns ──

#[test]
fn parse_quoted_string_in_pattern() {
    let rule = parse(
        "if:\n  at afterIteration\n  stmt If(UnaryOp(Not, $cond), [Call($f)], None)\n  where call_name_matches($f, \"abort\")\ndo:\n  emit Call(\"assert\", [$cond])",
    );
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::Emit(_)))
    );
}

#[test]
fn parse_replace_expr_call_with_string_name() {
    let rule = parse(
        "if:\n  at afterIteration\n  expr Ternary(BinaryOp(Less, $a, $b), $a2, $b2)\n  where structurally_equal($a, $a2)\n  where structurally_equal($b, $b2)\ndo:\n  replace_expr Call(\"min\", [$a, $b])",
    );
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ReplaceExpr(_)))
    );
}

// ── Suppression pattern parsing ──

#[test]
fn parse_register_spill_suppression() {
    let rule =
        parse("if:\n  at beforeIrAnalyzation\n  asm `push rbx; pop rbx`\ndo:\n  del start[0..2]");
    assert!(!rule.in_blocks.is_empty());
}

#[test]
fn parse_sanitizer_suppression() {
    let rule = parse("if:\n  at beforeIrAnalyzation\n  asm_contains __asan_\ndo:\n  del start[0]");
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::AsmContains));
}

// ── Cleanup pattern parsing ──

#[test]
fn parse_redundant_return_elimination() {
    let rule = parse(
        "if:\n  at afterOptimization\n  script `\nlet n = ast_stmts.len();\nn > 0 && ast_stmts[n - 1].is_return() && ast_stmts[n - 1].return_expr() == \"\"\n`\ndo:\n  del end[0]",
    );
    let blocks = first_in_block(&rule);
    assert!(find_kind(blocks, AstPatternInBlockKind::Script));
}

#[test]
fn parse_single_arm_if_cleanup() {
    let rule = parse(
        "if:\n  at afterOptimization\n  stmt If($cond, [], None)\n  where is_pure($cond)\ndo:\n  emit Empty",
    );
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::Emit(_)))
    );
}

#[test]
fn parse_redundant_block_unwrap() {
    let rule = parse(
        "if:\n  at afterOptimization\n  stmt If($cond, [Block($inner)], $else)\ndo:\n  emit If($cond, $inner, $else)",
    );
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::Emit(_)))
    );
}

// ── Optimization pattern parsing ──

#[test]
fn parse_redundant_cast_elimination() {
    let rule = parse(
        "if:\n  at afterIteration\n  expr Cast($t, UnaryOp(CastSigned, $x))\ndo:\n  replace_expr Cast($t, $x)",
    );
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ReplaceExpr(_)))
    );
}

#[test]
fn parse_null_check_canonicalization() {
    let rule = parse(
        "if:\n  at afterIteration\n  expr BinaryOp(NotEqual, $x, Literal(Int(0)))\n  where is_variable($x)\ndo:\n  replace_expr $x",
    );
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ReplaceExpr(_)))
    );
}

// ── Recognition pattern parsing ──

#[test]
fn parse_deref_addressof_cleanup() {
    let rule =
        parse("if:\n  at afterIteration\n  expr Deref(AddressOf($x))\ndo:\n  replace_expr $x");
    let actions = first_group(&rule).out_actions();
    assert!(
        actions
            .iter()
            .any(|a| matches!(a, AstPatternOutAction::ReplaceExpr(_)))
    );
}

// ── All predefined .fb patterns parse successfully ──

#[test]
fn all_predefined_patterns_parse() {
    let patterns = AstPattern::predefined_patterns();
    assert!(!patterns.is_empty(), "should have predefined patterns");
    for pattern in &patterns {
        let content = pattern.pattern().trim();
        if content.is_empty() || !content.contains("if:") {
            continue;
        }
        let result = parse_pattern_file(pattern.name(), pattern.pattern());
        assert!(
            result.is_ok(),
            "predefined pattern `{}` failed to parse: {}",
            pattern.name(),
            result.unwrap_err()
        );
    }
}

// ── All .fb files in patterns/ directory parse successfully ──

#[test]
fn all_pattern_files_parse() {
    let patterns_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("patterns");
    if !patterns_dir.exists() {
        return;
    }
    fn visit_dir(dir: &std::path::Path, failures: &mut Vec<String>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit_dir(&path, failures);
                } else if path.extension().is_some_and(|ext| ext == "fb") {
                    let content = std::fs::read_to_string(&path).unwrap();
                    // Skip example/reference file (contains documentation-only syntax)
                    // and known patterns with unsupported asm (endbr64, nop)
                    let fname = path.file_name().unwrap().to_str().unwrap();
                    if matches!(
                        fname,
                        "all_syntax.fb" | "cet-cfg-cleanup.fb" | "nop-padding-cleanup.fb"
                    ) {
                        continue;
                    }
                    let name = path.display().to_string();
                    if let Err(err) = parse_pattern_file(&name, &content) {
                        failures.push(format!("{name}: {err}"));
                    }
                }
            }
        }
    }
    let mut failures = Vec::new();
    visit_dir(&patterns_dir, &mut failures);
    assert!(
        failures.is_empty(),
        "pattern files failed to parse:\n{}",
        failures.join("\n")
    );
}

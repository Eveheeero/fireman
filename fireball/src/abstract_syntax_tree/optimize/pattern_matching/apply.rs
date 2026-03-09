use super::{
    AstPattern, AstPatternApplyPhase, AstPatternAsmData, AstPatternAstData, AstPatternDeleteAnchor,
    AstPatternDeleteTarget, AstPatternInBlock, AstPatternInBlockKind, AstPatternInputType,
    AstPatternIrData, AstPatternIrReplacement, AstPatternLogLevel, AstPatternMatch,
    AstPatternOrigin, AstPatternOutAction, AstPatternParsed, AstPatternRange, AstPatternRule,
    AstPatternScript, AstPatternSkippedMatch, block_asm, block_asm_contains, block_ast,
    block_ast_sequence, block_at_matches_phase, block_expr, block_ir, block_script,
    block_ignore_asm_filters, block_ignore_ast_filters, block_ignore_comment_filters,
    block_ignore_ir_filters, IgnoreCommentFilter,
    block_skip_asm_range, block_skip_ast_range, block_skip_ir_range, block_skip_range, block_stmt,
    block_stmt_seq, fb_parser::load_file_pattern_rule_cached, has_kind, has_script_in_blocks,
    hashing::structural_statement_hash, infer_input_type_from_in_blocks,
    ir_parser::normalize_for_match, stmt_pattern,
};
use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, AstStatementOrigin,
        ProcessedOptimization, WrappedAstStatement,
    },
    ir::statements::IrStatement,
    prelude::DecompileError,
};
use rhai::{AST as RhaiAst, Dynamic, Engine, Scope};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};
use tracing::{debug, error, info, trace, warn};

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

#[derive(Debug, Clone)]
struct AstPatternLoadedRule {
    rule: AstPatternRule,
    input_type: AstPatternInputType,
}

thread_local! {
    static RHAI_ENGINE: RefCell<Engine> = RefCell::new(build_rhai_engine());
    static RHAI_SCRIPT_CACHE: RefCell<HashMap<String, RhaiAst>> = RefCell::new(HashMap::new());
}

pub(in crate::abstract_syntax_tree::optimize) fn apply_patterns(
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
                AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
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
                AstStatement::Switch(_, cases, default) => {
                    for (_lit, case_body) in cases.iter_mut() {
                        pass_changed |= apply_file_pattern_rules_recursive(
                            case_body,
                            rules,
                            ir_debug,
                            function_ir_statements,
                            phase,
                        );
                    }
                    if let Some(default_body) = default {
                        pass_changed |= apply_file_pattern_rules_recursive(
                            default_body,
                            rules,
                            ir_debug,
                            function_ir_statements,
                            phase,
                        );
                    }
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
                | AstStatement::Break
                | AstStatement::Continue
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

    let need_script = rule
        .clause_groups
        .iter()
        .any(|group| has_script_in_blocks(&group.in_blocks));
    let (need_asm, need_ast, need_ir) = match input_type {
        AstPatternInputType::WithAssembly => (true, false, false),
        AstPatternInputType::WithAst => (false, true, false),
        AstPatternInputType::WithIr => (false, false, true),
        AstPatternInputType::Complex => (true, true, true),
    };
    let mut changed = false;

    // Handle expr/replace_expr clause groups separately: they transform
    // expressions in-place and don't participate in the statement-level
    // match/skip convergence loop.
    for group in &rule.clause_groups {
        // Check if this group has an Expr in-block
        let expr_match = group.in_blocks.iter().find_map(|block| block_expr(block));
        let Some((match_pat, predicates)) = expr_match else {
            continue;
        };
        // Check phase constraint
        let at_ok = group.in_blocks.iter().all(|block| {
            let (has_at, at_matched) = block_at_matches_phase(block, phase);
            !has_at || at_matched
        });
        if !at_ok {
            continue;
        }
        for action in &group.out_actions {
            match action {
                AstPatternOutAction::ReplaceExpr(replace_pat) => {
                    changed |= stmt_pattern::transform_expressions_in_stmts(
                        stmts,
                        match_pat,
                        predicates,
                        replace_pat,
                    );
                }
                AstPatternOutAction::ReplaceExprBuiltin { func, args } => {
                    changed |= stmt_pattern::transform_expressions_in_stmts_builtin(
                        stmts, match_pat, predicates, func, args,
                    );
                }
                _ => {}
            }
        }
    }

    let mut seen_states = HashSet::new();
    let mut skipped_matches = HashSet::<AstPatternSkippedMatch>::new();

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
        let matched = rule
            .clause_groups
            .iter()
            .enumerate()
            .filter(|(_, group)| {
                // Skip expr/replace_expr groups -- handled above
                !group
                    .in_blocks
                    .iter()
                    .any(|block| has_kind(block, AstPatternInBlockKind::Expr))
            })
            .find_map(|(clause_group_index, group)| {
                group.in_blocks.iter().find_map(|block| {
                    let (matched, captures) = match_if_block(
                        block,
                        &script_context,
                        &asm_lines,
                        &ast_statements,
                        &ir_statements,
                        phase,
                    )?;
                    let skipped = AstPatternSkippedMatch {
                        clause_group_index,
                        matched,
                    };
                    if skipped_matches.contains(&skipped) {
                        None
                    } else {
                        Some((clause_group_index, matched, captures))
                    }
                })
            });
        let Some((clause_group_index, matched, stmt_captures)) = matched else {
            break;
        };
        let clause_group = &rule.clause_groups[clause_group_index];

        for action in &clause_group.out_actions {
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
                AstPatternOutAction::SpliceBlock => {
                    apply_splice_block(stmts, &matched);
                }
                AstPatternOutAction::Emit(emit_pat) => {
                    if let Some((start, end)) = matched.ast_statement_range {
                        if let Some(caps) = &stmt_captures {
                            if let Some(replacement) =
                                stmt_pattern::construct_statement(emit_pat, caps)
                            {
                                stmts[start].statement = replacement;
                                // For multi-statement matches, remove the
                                // trailing statements that were part of the
                                // sequence.
                                if end > start + 1 {
                                    stmts.drain((start + 1)..end);
                                }
                            }
                        }
                    }
                }
                AstPatternOutAction::EmitBefore(emit_pat) => {
                    if let Some((start, _end)) = matched.ast_statement_range {
                        if let Some(caps) = &stmt_captures {
                            if let Some(list) =
                                stmt_pattern::construct_emit_after_list(emit_pat, caps)
                            {
                                for (j, before_stmt) in list.into_iter().enumerate() {
                                    stmts.insert(start + j, before_stmt);
                                }
                            }
                        }
                    }
                }
                AstPatternOutAction::Script(script) => {
                    if !execute_do_script_with_captures(
                        script,
                        &script_context,
                        stmt_captures.as_ref(),
                    ) {
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
                AstPatternOutAction::EmitAfter(emit_pat) => {
                    if let Some((_start, end)) = matched.ast_statement_range {
                        if let Some(caps) = &stmt_captures {
                            // EmitAfter expects a capture that holds a StmtList.
                            // Construct the list and insert after the matched statement.
                            if let Some(list) =
                                stmt_pattern::construct_emit_after_list(emit_pat, caps)
                            {
                                let insert_pos = end + 1;
                                for (j, after_stmt) in list.into_iter().enumerate() {
                                    stmts.insert(insert_pos + j, after_stmt);
                                }
                            }
                        }
                    }
                }
                AstPatternOutAction::ReplaceExpr(replace_pat) => {
                    // Find the expr pattern from the in-blocks of this clause group
                    if let Some((match_pat, predicates)) = clause_group
                        .in_blocks
                        .iter()
                        .flatten()
                        .find_map(|b| match b {
                            AstPatternInBlock::Expr(pat, preds) => Some((pat, preds.as_slice())),
                            _ => None,
                        })
                    {
                        stmt_pattern::transform_expressions_in_stmts(
                            stmts,
                            match_pat,
                            predicates,
                            replace_pat,
                        );
                    }
                }
                AstPatternOutAction::ReplaceExprBuiltin { func, args } => {
                    // Find the expr pattern from the in-blocks of this clause group
                    if let Some((match_pat, predicates)) = clause_group
                        .in_blocks
                        .iter()
                        .flatten()
                        .find_map(|b| match b {
                            AstPatternInBlock::Expr(pat, preds) => Some((pat, preds.as_slice())),
                            _ => None,
                        })
                    {
                        stmt_pattern::transform_expressions_in_stmts_builtin(
                            stmts, match_pat, predicates, func, args,
                        );
                    }
                }
                AstPatternOutAction::PruneEmptyElse => {
                    prune_empty_else_recursive(stmts);
                }
                AstPatternOutAction::ClearIgnore(_) => {
                    // Handled by the caller; no per-match action needed here.
                }
            }
        }

        let state_after = structural_statement_hash(stmts);
        if state_after != state_before {
            changed = true;
            if matched.asm_statement_range.is_some() || matched.ast_statement_range.is_some() {
                skipped_matches.clear();
            } else {
                skipped_matches.insert(AstPatternSkippedMatch {
                    clause_group_index,
                    matched,
                });
            }
            continue;
        } else {
            skipped_matches.insert(AstPatternSkippedMatch {
                clause_group_index,
                matched,
            });
        }
    }

    changed
}

fn match_if_block(
    block: &[AstPatternInBlock],
    script_context: &AstPatternScriptContext<'_>,
    asm_lines: &[AstPatternNormalizedAsmLine],
    ast_statements: &[AstStatement],
    ir_statements: &[IrStatement],
    phase: AstPatternApplyPhase,
) -> Option<(AstPatternMatch, Option<stmt_pattern::Captures>)> {
    let mut has_condition = false;
    let mut matched = AstPatternMatch::default();
    let mut stmt_captures: Option<stmt_pattern::Captures> = None;

    // Apply ignore filters to produce filtered views of asm/ir/ast data.
    let asm_ignore_filters = block_ignore_asm_filters(block);
    let filtered_asm_lines: Vec<AstPatternNormalizedAsmLine>;
    let effective_asm = if asm_ignore_filters.is_empty() {
        asm_lines
    } else {
        filtered_asm_lines = apply_ignore_asm_filters(asm_lines, &asm_ignore_filters);
        &filtered_asm_lines
    };

    let ir_ignore_filters = block_ignore_ir_filters(block);
    let filtered_ir: Vec<IrStatement>;
    let effective_ir = if ir_ignore_filters.is_empty() {
        ir_statements
    } else {
        filtered_ir = apply_ignore_ir_filters(ir_statements, &ir_ignore_filters);
        &filtered_ir
    };

    let ast_ignore_filters = block_ignore_ast_filters(block);
    let comment_ignore_filters = block_ignore_comment_filters(block);
    let ast_index_map: Vec<usize>;
    let filtered_ast: Vec<AstStatement>;
    let needs_ast_filter = !ast_ignore_filters.is_empty() || !comment_ignore_filters.is_empty();
    let (effective_ast, ast_remap) = if !needs_ast_filter {
        (ast_statements, None)
    } else {
        let (filtered, index_map) = apply_ignore_ast_and_comment_filters(
            ast_statements,
            &ast_ignore_filters,
            &comment_ignore_filters,
        );
        filtered_ast = filtered;
        ast_index_map = index_map;
        (filtered_ast.as_slice(), Some(&ast_index_map))
    };

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
        matched.asm_statement_range = Some(find_asm_match(effective_asm, asm, asm_skip_range)?);
    }
    if let Some(asm_contains) = block_asm_contains(block) {
        has_condition = true;
        let asm_skip_range = block_skip_range(block).or(block_skip_asm_range(block));
        matched.asm_statement_range = Some(find_asm_contains_match(
            effective_asm,
            asm_contains,
            asm_skip_range,
        )?);
    }
    if let Some(ast) = block_ast(block) {
        has_condition = true;
        let (start, end) = find_ast_match(
            effective_ast,
            ast,
            block_skip_ast_range(block),
        )?;
        matched.ast_statement_range = Some(remap_ast_range(start, end, ast_remap.map(|v| &**v)));
    }
    if let Some(ast_sequence) = block_ast_sequence(block) {
        has_condition = true;
        let (start, end) = find_ast_sequence_match(
            effective_ast,
            ast_sequence,
            block_skip_ast_range(block),
        )?;
        matched.ast_statement_range = Some(remap_ast_range(start, end, ast_remap.map(|v| &**v)));
    }
    // stmt structural pattern matching with captures
    if let Some((pat, predicates)) = block_stmt(block) {
        has_condition = true;
        let skip_range = block_skip_range(block).or(block_skip_ast_range(block));
        let start = skip_range.map_or(0usize, |r| r.start.min(effective_ast.len()));
        let end = skip_range.map_or(effective_ast.len(), |r| {
            r.end_exclusive.min(effective_ast.len())
        });
        let mut found = false;
        for i in start..end {
            if let Some(caps) = stmt_pattern::match_statement(pat, &effective_ast[i]) {
                let preds_ok = predicates
                    .iter()
                    .all(|pred| stmt_pattern::eval_where(pred, &caps));
                if preds_ok {
                    let (orig_start, orig_end) = remap_ast_range(i, i + 1, ast_remap.map(|v| &**v));
                    matched.ast_statement_range = Some((orig_start, orig_end));
                    stmt_captures = Some(caps);
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return None;
        }
    }
    // stmt_seq multi-statement sequence matching
    if let Some((pats, predicates)) = block_stmt_seq(block) {
        has_condition = true;
        let skip_range = block_skip_range(block).or(block_skip_ast_range(block));
        let start = skip_range.map_or(0usize, |r| r.start.min(effective_ast.len()));
        let end = skip_range.map_or(effective_ast.len(), |r| {
            r.end_exclusive.min(effective_ast.len())
        });
        let n = pats.len();
        let mut found = false;
        if n > 0 && end >= start + n {
            for window_start in start..=(end - n) {
                let mut all_matched = true;
                let mut merged_caps = stmt_pattern::Captures::new();
                for (j, pat) in pats.iter().enumerate() {
                    if let Some(caps) =
                        stmt_pattern::match_statement(pat, &effective_ast[window_start + j])
                    {
                        merged_caps.extend(caps);
                    } else {
                        all_matched = false;
                        break;
                    }
                }
                if all_matched {
                    let preds_ok = predicates
                        .iter()
                        .all(|pred| stmt_pattern::eval_where(pred, &merged_caps));
                    if preds_ok {
                        matched.ast_statement_range = Some(remap_ast_range(
                            window_start,
                            window_start + n,
                            ast_remap.map(|v| &**v),
                        ));
                        stmt_captures = Some(merged_caps);
                        found = true;
                        break;
                    }
                }
            }
        }
        if !found {
            return None;
        }
    }
    // expr structural pattern matching (expression-level, matches anywhere)
    if let Some((_pat, _predicates)) = block_expr(block) {
        // The `expr` condition is always satisfied if present -- the actual
        // matching and replacement is performed by `ReplaceExpr` in the do:
        // section which walks all expressions in-place.  We just mark
        // "has_condition = true" so the rule fires.
        has_condition = true;
    }
    if let Some(ir) = block_ir(block) {
        has_condition = true;
        if !sequence_matches_ir(effective_ir, ir, block_skip_ir_range(block)) {
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

    if has_condition {
        Some((matched, stmt_captures))
    } else {
        None
    }
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

fn find_asm_contains_match(
    asm_lines: &[AstPatternNormalizedAsmLine],
    expected: &str,
    skip_range: Option<AstPatternRange>,
) -> Option<(usize, usize)> {
    if asm_lines.is_empty() {
        return None;
    }

    let expected = normalize_for_match(expected);
    if expected.is_empty() {
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
        if asm_lines[cursor].line.contains(&expected) {
            let stmt_index = asm_lines[cursor].stmt_index;
            return Some((stmt_index, stmt_index));
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

fn find_ast_sequence_match(
    statements: &[AstStatement],
    data: &[AstPatternAstData],
    skip_range: Option<AstPatternRange>,
) -> Option<(usize, usize)> {
    if statements.is_empty() || data.is_empty() {
        return None;
    }

    let start = skip_range.map_or(0usize, |range| range.start.min(statements.len()));
    let end_exclusive = skip_range.map_or(statements.len(), |range| {
        range.end_exclusive.min(statements.len())
    });
    if end_exclusive <= start {
        return None;
    }

    let sequence_len = data.len();
    if end_exclusive - start < sequence_len {
        return None;
    }

    for cursor in start..=(end_exclusive - sequence_len) {
        if data
            .iter()
            .enumerate()
            .all(|(offset, item)| item.matches_statement(&statements[cursor + offset]))
        {
            return Some((cursor, cursor + sequence_len - 1));
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

fn apply_splice_block(stmts: &mut Vec<WrappedAstStatement>, matched: &AstPatternMatch) {
    let Some((start, end)) = matched.ast_statement_range else {
        return;
    };
    if start != end || start >= stmts.len() {
        return;
    }

    let removed = stmts.remove(start);
    if let AstStatement::Block(inner) = removed.statement {
        stmts.splice(start..start, inner);
    } else {
        stmts.insert(start, removed);
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
        AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
            prune_empty_else_recursive(body);
        }
        AstStatement::For(init, _, update, body) => {
            prune_empty_else_statement_recursive(init);
            prune_empty_else_statement_recursive(update);
            prune_empty_else_recursive(body);
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases.iter_mut() {
                prune_empty_else_recursive(case_body);
            }
            if let Some(default_body) = default {
                prune_empty_else_recursive(default_body);
            }
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
        | AstStatement::Break
        | AstStatement::Continue
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

pub(super) fn compiled_script(script: &str) -> Result<RhaiAst, String> {
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

fn execute_do_script_with_captures(
    script: &AstPatternScript,
    context: &AstPatternScriptContext<'_>,
    captures: Option<&stmt_pattern::Captures>,
) -> bool {
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
    if let Some(caps) = captures {
        stmt_pattern::inject_captures_into_rhai_scope(caps, &mut scope);
    }
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

// --- Ignore filter helpers ---

fn apply_ignore_asm_filters(
    asm_lines: &[AstPatternNormalizedAsmLine],
    filters: &[Option<&AstPatternAsmData>],
) -> Vec<AstPatternNormalizedAsmLine> {
    if filters.iter().any(|f| f.is_none()) {
        return Vec::new();
    }
    asm_lines
        .iter()
        .filter(|line| {
            !filters.iter().any(|filter| {
                if let Some(asm_data) = filter {
                    let expected = normalize_for_match(asm_data.as_match_text());
                    !expected.is_empty() && line.line.contains(&expected)
                } else {
                    true
                }
            })
        })
        .cloned()
        .collect()
}

fn apply_ignore_ir_filters(
    ir_statements: &[IrStatement],
    filters: &[Option<&AstPatternIrData>],
) -> Vec<IrStatement> {
    if filters.iter().any(|f| f.is_none()) {
        return Vec::new();
    }
    ir_statements
        .iter()
        .filter(|stmt| {
            !filters.iter().any(|filter| {
                if let Some(ir_data) = filter {
                    ir_data.matches_statement(stmt)
                } else {
                    true
                }
            })
        })
        .cloned()
        .collect()
}

/// Filter AST statements by both AST ignore filters and comment ignore filters,
/// returning the filtered list and a map from filtered indices to original indices.
fn apply_ignore_ast_and_comment_filters(
    ast_statements: &[AstStatement],
    ast_filters: &[Option<&AstPatternAstData>],
    comment_filters: &[&IgnoreCommentFilter],
) -> (Vec<AstStatement>, Vec<usize>) {
    // If any bare `ignore ast` (None) is present, ignore ALL ast statements.
    if ast_filters.iter().any(|f| f.is_none()) {
        return (Vec::new(), Vec::new());
    }
    let mut filtered = Vec::new();
    let mut index_map = Vec::new();
    for (orig_idx, stmt) in ast_statements.iter().enumerate() {
        let ignored_by_ast = ast_filters.iter().any(|filter| {
            if let Some(ast_data) = filter {
                ast_data.matches_statement(stmt)
            } else {
                true
            }
        });
        let ignored_by_comment = if let AstStatement::Comment(text) = stmt {
            comment_filters
                .iter()
                .any(|filter| filter.matches_comment(text))
        } else {
            false
        };
        if !ignored_by_ast && !ignored_by_comment {
            filtered.push(stmt.clone());
            index_map.push(orig_idx);
        }
    }
    (filtered, index_map)
}

/// Remap a (start, end_exclusive) range from filtered indices to original indices.
fn remap_ast_range(start: usize, end: usize, remap: Option<&[usize]>) -> (usize, usize) {
    match remap {
        None => (start, end),
        Some(map) => {
            let orig_start = map.get(start).copied().unwrap_or(start);
            let orig_end = if end > 0 {
                map.get(end - 1).copied().map_or(end, |e| e + 1)
            } else {
                0
            };
            (orig_start, orig_end)
        }
    }
}

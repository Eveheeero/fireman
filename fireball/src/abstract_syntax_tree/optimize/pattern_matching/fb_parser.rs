use super::{
    AstPatternApplyAt, AstPatternApplyPhase, AstPatternAsmData, AstPatternAstData,
    AstPatternClauseGroup, AstPatternDeleteAnchor, AstPatternDeleteTarget, AstPatternInBlock,
    AstPatternInBlockKind, AstPatternIrData, AstPatternIrReplacement, AstPatternLogLevel,
    AstPatternOutAction, AstPatternRange, AstPatternRule, AstPatternScript, ClearIgnoreTarget,
    IgnoreCommentFilter, add_at_clause, has_kind,
    ir_parser::{
        find_matching_delimiter, parse_asm_arguments, parse_asm_statement, parse_ir_statement,
    },
    set_clause, stmt_pattern,
};
use crate::{abstract_syntax_tree::AstStatement, ir::statements::IrStatement};
use std::{cell::RefCell, collections::HashMap, fs, time::SystemTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct AstPatternFileFingerprint {
    modified: Option<SystemTime>,
    len: u64,
}

#[derive(Debug, Clone)]
pub(super) struct AstPatternCachedRule {
    fingerprint: AstPatternFileFingerprint,
    rule: AstPatternRule,
}

thread_local! {
    static FILE_PATTERN_CACHE: RefCell<HashMap<String, AstPatternCachedRule>> = RefCell::new(HashMap::new());
}

pub(super) fn load_file_pattern_rule_cached(path: &str) -> Result<AstPatternRule, String> {
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

pub(super) fn load_file_pattern_rule_uncached(path: &str) -> Result<AstPatternRule, String> {
    let content = fs::read_to_string(path)
        .map_err(|err| format!("failed to read pattern file `{path}`: {err}"))?;
    parse_pattern_file(path, &content)
}

pub(super) fn fingerprint(path: &str) -> Result<AstPatternFileFingerprint, String> {
    let metadata =
        fs::metadata(path).map_err(|err| format!("failed to read metadata for `{path}`: {err}"))?;
    Ok(AstPatternFileFingerprint {
        modified: metadata.modified().ok(),
        len: metadata.len(),
    })
}

pub(super) fn flush_current_in_blocks(
    current_in_blocks: &mut Vec<Vec<AstPatternInBlock>>,
    clause_group: &mut AstPatternClauseGroup,
    has_current_in: &mut bool,
) {
    if !*has_current_in {
        return;
    }

    let flushed = std::mem::take(current_in_blocks);
    clause_group.in_blocks.extend(flushed);
    *current_in_blocks = vec![Vec::new()];
    *has_current_in = false;
}

pub(super) fn finalize_clause_group(
    rule: &mut AstPatternRule,
    current_in_blocks: &mut Vec<Vec<AstPatternInBlock>>,
    clause_group: &mut AstPatternClauseGroup,
    has_current_in: &mut bool,
    path: &str,
) -> Result<(), String> {
    flush_current_in_blocks(current_in_blocks, clause_group, has_current_in);
    clause_group.in_blocks.retain(|block| !block.is_empty());
    if clause_group.is_empty() {
        return Ok(());
    }
    if clause_group.in_blocks.is_empty() {
        return Err(format!(
            "pattern `{path}` has `do:` actions without matching `if:` clauses"
        ));
    }
    if clause_group.out_actions.is_empty() {
        return Err(format!(
            "pattern `{path}` has `if:` clauses without matching `do:` actions"
        ));
    }
    rule.push_clause_group(std::mem::take(clause_group));
    Ok(())
}

pub fn parse_editable_asm_to_ir_statements(text: &str) -> Result<Vec<IrStatement>, String> {
    let raw = normalize_editable_asm_text(text)
        .ok_or_else(|| "assembly statement cannot be empty".to_string())?;
    let mut parts = raw.splitn(2, |ch: char| ch.is_whitespace());
    let mnemonic = parts.next().unwrap_or_default().trim();
    let operands = parts.next().unwrap_or_default().trim();
    if mnemonic.is_empty() {
        return Err("assembly mnemonic cannot be empty".to_string());
    }

    let statement = iceball::parse_statement(iceball::Architecture::X64, mnemonic)
        .map_err(|_| format!("unknown assembly mnemonic `{mnemonic}`"))?;
    let arguments = parse_asm_arguments(operands)
        .ok_or_else(|| format!("invalid assembly operands `{operands}`"))?;
    let instruction = crate::core::Instruction {
        address: 0,
        inner: iceball::Instruction {
            statement: Ok(statement),
            arguments: arguments.into_boxed_slice(),
            bytes: None,
        },
    };

    crate::arch::x86_64::instruction_analyze::create_ir_statement(&instruction)
        .map(|statements| statements.to_vec())
        .ok_or_else(|| format!("could not analyze assembly statement `{raw}`"))
}

pub fn parse_editable_ir_statement(text: &str) -> Result<IrStatement, String> {
    parse_ir_statement(text).ok_or_else(|| format!("invalid IR statement `{}`", text.trim()))
}

pub fn parse_editable_ast_statement(text: &str) -> Result<AstStatement, String> {
    parse_ast_replacement(text)
}

pub(super) fn normalize_editable_asm_text(text: &str) -> Option<&str> {
    let mut raw = text.trim();
    if raw.is_empty() {
        return None;
    }
    raw = raw.strip_prefix("asm ").unwrap_or(raw).trim();

    if let Some((head, tail)) = raw.split_once(char::is_whitespace) {
        let is_hex_address = head.starts_with("0x")
            && head.len() > 2
            && head[2..].chars().all(|ch| ch.is_ascii_hexdigit());
        if is_hex_address {
            raw = tail.trim();
        }
    }

    raw = raw.split(';').next().unwrap_or(raw).trim();
    (!raw.is_empty()).then_some(raw)
}

pub(super) fn parse_pattern_file(path: &str, content: &str) -> Result<AstPatternRule, String> {
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
    let mut current_clause_group = AstPatternClauseGroup::default();
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
            if matches!(section, Section::Do) {
                finalize_clause_group(
                    &mut rule,
                    &mut current_in_blocks,
                    &mut current_clause_group,
                    &mut has_current_in,
                    path,
                )?;
            } else {
                flush_current_in_blocks(
                    &mut current_in_blocks,
                    &mut current_clause_group,
                    &mut has_current_in,
                );
            }
            has_current_in = true;
            section = Section::If;
            continue;
        }

        if trimmed.starts_with("do:") {
            if !has_current_in && current_clause_group.in_blocks.is_empty() {
                return Err(format!(
                    "pattern `{path}` has `do:` before any `if:` blocks"
                ));
            }
            flush_current_in_blocks(
                &mut current_in_blocks,
                &mut current_clause_group,
                &mut has_current_in,
            );
            section = Section::Do;
            continue;
        }

        match section {
            Section::If => {
                if line.trim_start().starts_with("asm_contains ") {
                    let value = parse_multiline_value(
                        line.trim_start(),
                        "asm_contains ",
                        &lines,
                        &mut idx,
                    )?;
                    let expected = value.trim();
                    if expected.is_empty() {
                        return Err(format!(
                            "invalid `asm_contains` matcher in pattern `{path}`: value must not be empty"
                        ));
                    }
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(block, AstPatternInBlock::AsmContains(expected.to_string()));
                    });
                } else if line.trim_start().starts_with("asm ") {
                    let value = parse_multiline_value(line.trim_start(), "asm ", &lines, &mut idx)?;
                    let sequence = split_pattern_sequence_raw(&value, true);
                    let sequence = sequence
                        .iter()
                        .map(|item| {
                            AstPatternAsmData::from_text(item).map_err(|err| {
                                format!(
                                    "invalid asm matcher in pattern `{path}`: {} ({err})",
                                    item.trim()
                                )
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    expand_in_blocks_for_asm(&mut current_in_blocks, &sequence);
                } else if line.trim_start().starts_with("stmt ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "stmt ", &lines, &mut idx)?;
                    let pat_tree = stmt_pattern::parse_pattern(&value)
                        .map_err(|err| format!("invalid stmt pattern in `{path}`: {err}"))?;
                    // Collect any subsequent `where` lines
                    let mut predicates = Vec::new();
                    while idx < lines.len() {
                        let next_line = strip_inline_comment(lines[idx]);
                        let next_trimmed = next_line.trim();
                        if next_trimmed.starts_with("where ") {
                            idx += 1;
                            let where_value = next_trimmed.strip_prefix("where ").unwrap();
                            let pred = stmt_pattern::parse_where(where_value).map_err(|err| {
                                format!("invalid where predicate in `{path}`: {err}")
                            })?;
                            predicates.push(pred);
                        } else {
                            break;
                        }
                    }
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(
                            block,
                            AstPatternInBlock::Stmt(pat_tree.clone(), predicates.clone()),
                        );
                    });
                } else if line.trim_start().starts_with("stmt_seq ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "stmt_seq ", &lines, &mut idx)?;
                    let pat_trees = split_stmt_seq_patterns(&value)
                        .into_iter()
                        .map(|s| {
                            stmt_pattern::parse_pattern(&s).map_err(|err| {
                                format!("invalid stmt_seq pattern in `{path}`: {err}")
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    if pat_trees.is_empty() {
                        return Err(format!(
                            "stmt_seq requires at least one pattern in `{path}`"
                        ));
                    }
                    // Collect any subsequent `where` lines
                    let mut predicates = Vec::new();
                    while idx < lines.len() {
                        let next_line = strip_inline_comment(lines[idx]);
                        let next_trimmed = next_line.trim();
                        if next_trimmed.starts_with("where ") {
                            idx += 1;
                            let where_value = next_trimmed.strip_prefix("where ").unwrap();
                            let pred = stmt_pattern::parse_where(where_value).map_err(|err| {
                                format!("invalid where predicate in `{path}`: {err}")
                            })?;
                            predicates.push(pred);
                        } else {
                            break;
                        }
                    }
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(
                            block,
                            AstPatternInBlock::StmtSeq(pat_trees.clone(), predicates.clone()),
                        );
                    });
                } else if line.trim_start().starts_with("expr ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "expr ", &lines, &mut idx)?;
                    let pat_tree = stmt_pattern::parse_pattern(&value)
                        .map_err(|err| format!("invalid expr pattern in `{path}`: {err}"))?;
                    // Collect any subsequent `where` lines
                    let mut predicates = Vec::new();
                    while idx < lines.len() {
                        let next_line = strip_inline_comment(lines[idx]);
                        let next_trimmed = next_line.trim();
                        if next_trimmed.starts_with("where ") {
                            idx += 1;
                            let where_value = next_trimmed.strip_prefix("where ").unwrap();
                            let pred = stmt_pattern::parse_where(where_value).map_err(|err| {
                                format!("invalid where predicate in `{path}`: {err}")
                            })?;
                            predicates.push(pred);
                        } else {
                            break;
                        }
                    }
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(
                            block,
                            AstPatternInBlock::Expr(pat_tree.clone(), predicates.clone()),
                        );
                    });
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
                } else if line.trim_start().starts_with("skip asm ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip asm ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(block, AstPatternInBlock::SkipAsmRange(range));
                    });
                } else if line.trim_start().starts_with("skip ast ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip ast ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(block, AstPatternInBlock::SkipAstRange(range));
                    });
                } else if line.trim_start().starts_with("skip ir ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip ir ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(block, AstPatternInBlock::SkipIrRange(range));
                    });
                } else if line.trim_start().starts_with("skip ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "skip ", &lines, &mut idx)?;
                    let range = parse_skip_range(&value)?;
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        set_clause(block, AstPatternInBlock::SkipRange(range));
                    });
                } else if line.trim_start().starts_with("ignore asm ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "ignore asm ", &lines, &mut idx)?;
                    let trimmed_val = value.trim();
                    if trimmed_val.is_empty() {
                        update_all_in_blocks(&mut current_in_blocks, |block| {
                            block.push(AstPatternInBlock::IgnoreAsm(None));
                        });
                    } else {
                        let asm_data =
                            AstPatternAsmData::from_text(trimmed_val).map_err(|err| {
                                format!(
                                    "invalid ignore asm in pattern `{path}`: {trimmed_val} ({err})"
                                )
                            })?;
                        update_all_in_blocks(&mut current_in_blocks, |block| {
                            block.push(AstPatternInBlock::IgnoreAsm(Some(asm_data.clone())));
                        });
                    }
                } else if line.trim_start() == "ignore asm" {
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.push(AstPatternInBlock::IgnoreAsm(None));
                    });
                } else if line.trim_start().starts_with("ignore ir ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "ignore ir ", &lines, &mut idx)?;
                    let trimmed_val = value.trim();
                    if trimmed_val.is_empty() {
                        update_all_in_blocks(&mut current_in_blocks, |block| {
                            block.push(AstPatternInBlock::IgnoreIr(None));
                        });
                    } else {
                        let ir_data = AstPatternIrData::from_text(trimmed_val);
                        if let Some(ir_data) = ir_data {
                            update_all_in_blocks(&mut current_in_blocks, |block| {
                                block.push(AstPatternInBlock::IgnoreIr(Some(ir_data.clone())));
                            });
                        }
                    }
                } else if line.trim_start() == "ignore ir" {
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.push(AstPatternInBlock::IgnoreIr(None));
                    });
                } else if line.trim_start().starts_with("ignore ast ") {
                    let value =
                        parse_multiline_value(line.trim_start(), "ignore ast ", &lines, &mut idx)?;
                    let trimmed_val = value.trim();
                    if trimmed_val.is_empty() {
                        update_all_in_blocks(&mut current_in_blocks, |block| {
                            block.push(AstPatternInBlock::IgnoreAst(None));
                        });
                    } else {
                        let ast_data = AstPatternAstData::from_text(trimmed_val);
                        if let Some(ast_data) = ast_data {
                            update_all_in_blocks(&mut current_in_blocks, |block| {
                                block.push(AstPatternInBlock::IgnoreAst(Some(ast_data.clone())));
                            });
                        }
                    }
                } else if line.trim_start() == "ignore ast" {
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.push(AstPatternInBlock::IgnoreAst(None));
                    });
                } else if line.trim_start() == "ignore comment" {
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.push(AstPatternInBlock::IgnoreComment(IgnoreCommentFilter::All));
                    });
                } else if line.trim_start().starts_with("ignore commentstart ") {
                    let value = parse_multiline_value(
                        line.trim_start(),
                        "ignore commentstart ",
                        &lines,
                        &mut idx,
                    )?;
                    let trimmed_val = value.trim();
                    if trimmed_val.is_empty() {
                        return Err(format!(
                            "ignore commentstart requires a value in pattern `{path}`"
                        ));
                    }
                    let text = trimmed_val.to_string();
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.push(AstPatternInBlock::IgnoreComment(
                            IgnoreCommentFilter::StartsWith(text.clone()),
                        ));
                    });
                } else if line.trim_start().starts_with("ignore commentend ") {
                    let value = parse_multiline_value(
                        line.trim_start(),
                        "ignore commentend ",
                        &lines,
                        &mut idx,
                    )?;
                    let trimmed_val = value.trim();
                    if trimmed_val.is_empty() {
                        return Err(format!(
                            "ignore commentend requires a value in pattern `{path}`"
                        ));
                    }
                    let text = trimmed_val.to_string();
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.push(AstPatternInBlock::IgnoreComment(
                            IgnoreCommentFilter::EndsWith(text.clone()),
                        ));
                    });
                } else if line.trim_start().starts_with("ignore commentcontains ") {
                    let value = parse_multiline_value(
                        line.trim_start(),
                        "ignore commentcontains ",
                        &lines,
                        &mut idx,
                    )?;
                    let trimmed_val = value.trim();
                    if trimmed_val.is_empty() {
                        return Err(format!(
                            "ignore commentcontains requires a value in pattern `{path}`"
                        ));
                    }
                    let text = trimmed_val.to_string();
                    update_all_in_blocks(&mut current_in_blocks, |block| {
                        block.push(AstPatternInBlock::IgnoreComment(
                            IgnoreCommentFilter::Contains(text.clone()),
                        ));
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
                    let replacement = parse_asm_replacement(&value).map_err(|err| {
                        format!(
                            "invalid asm replacement in pattern `{path}`: {} ({err})",
                            value.trim()
                        )
                    })?;
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::ReplaceAsm(replacement));
                } else if trimmed.starts_with("ir ") {
                    let value = parse_multiline_value(trimmed, "ir ", &lines, &mut idx)?;
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::ReplaceIr(parse_ir_replacement(&value)));
                } else if trimmed.starts_with("ast ") {
                    let value = parse_multiline_value(trimmed, "ast ", &lines, &mut idx)?;
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::ReplaceAst(parse_ast_replacement(
                            &value,
                        )?));
                } else if trimmed.starts_with("del ") {
                    let value = parse_multiline_value(trimmed, "del ", &lines, &mut idx)?;
                    let target = parse_do_delete_target(&value)?;
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::Delete(target));
                } else if trimmed == "splice-block" {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::SpliceBlock);
                } else if trimmed.starts_with("script ") {
                    let value = parse_multiline_value(trimmed, "script ", &lines, &mut idx)?;
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::Script(parse_rhai_script(&value)?));
                } else if let Some(msg) = parse_log_action(trimmed, "info") {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::Log(AstPatternLogLevel::Info, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "warn") {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::Log(AstPatternLogLevel::Warn, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "error") {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::Log(AstPatternLogLevel::Error, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "debug") {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::Log(AstPatternLogLevel::Debug, msg));
                } else if let Some(msg) = parse_log_action(trimmed, "trace") {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::Log(AstPatternLogLevel::Trace, msg));
                } else if trimmed.starts_with("emit_before ") {
                    let value = parse_multiline_value(trimmed, "emit_before ", &lines, &mut idx)?;
                    let emit_pat = stmt_pattern::parse_pattern(&value)
                        .map_err(|err| format!("invalid emit_before pattern in `{path}`: {err}"))?;
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::EmitBefore(emit_pat));
                } else if trimmed.starts_with("emit_after ") {
                    let value = parse_multiline_value(trimmed, "emit_after ", &lines, &mut idx)?;
                    let emit_pat = stmt_pattern::parse_pattern(&value)
                        .map_err(|err| format!("invalid emit_after pattern in `{path}`: {err}"))?;
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::EmitAfter(emit_pat));
                } else if trimmed.starts_with("emit ") {
                    let value = parse_multiline_value(trimmed, "emit ", &lines, &mut idx)?;
                    let emit_pat = stmt_pattern::parse_pattern(&value)
                        .map_err(|err| format!("invalid emit pattern in `{path}`: {err}"))?;
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::Emit(emit_pat));
                } else if trimmed.starts_with("replace_expr_fn ") {
                    let value =
                        parse_multiline_value(trimmed, "replace_expr_fn ", &lines, &mut idx)?;
                    let (func, args) = parse_builtin_call(&value)
                        .map_err(|err| format!("invalid replace_expr_fn in `{path}`: {err}"))?;
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::ReplaceExprBuiltin { func, args });
                } else if trimmed.starts_with("replace_expr ") {
                    let value = parse_multiline_value(trimmed, "replace_expr ", &lines, &mut idx)?;
                    let replace_pat = stmt_pattern::parse_pattern(&value).map_err(|err| {
                        format!("invalid replace_expr pattern in `{path}`: {err}")
                    })?;
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::ReplaceExpr(replace_pat));
                } else if trimmed == "prune-empty-else" {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::PruneEmptyElse);
                } else if trimmed == "!ignore" {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::ClearIgnore(ClearIgnoreTarget::All));
                } else if trimmed == "!ignore asm" {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::ClearIgnore(ClearIgnoreTarget::Asm));
                } else if trimmed == "!ignore ir" {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::ClearIgnore(ClearIgnoreTarget::Ir));
                } else if trimmed == "!ignore ast" {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::ClearIgnore(ClearIgnoreTarget::Ast));
                } else if trimmed == "!ignore comment" {
                    current_clause_group
                        .out_actions
                        .push(AstPatternOutAction::ClearIgnore(ClearIgnoreTarget::Comment));
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

    finalize_clause_group(
        &mut rule,
        &mut current_in_blocks,
        &mut current_clause_group,
        &mut has_current_in,
        path,
    )?;

    if rule.clause_groups.is_empty() {
        return Err(format!(
            "pattern `{path}` has no complete `if` / `do` clauses"
        ));
    }

    Ok(rule)
}

pub(super) fn strip_inline_comment(line: &str) -> String {
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

/// Parse a built-in function call like `eval_binop($op, $a, $b)`.
/// Returns (function_name, [capture_name_without_dollar, ...]).
pub(super) fn parse_builtin_call(input: &str) -> Result<(String, Vec<String>), String> {
    let input = input.trim();
    let paren_idx = input
        .find('(')
        .ok_or_else(|| format!("expected '(' in builtin call: {input}"))?;
    let func = input[..paren_idx].trim().to_string();
    let rest = input[paren_idx + 1..]
        .strip_suffix(')')
        .ok_or_else(|| format!("missing closing ')' in builtin call: {input}"))?;
    let args: Result<Vec<String>, String> = rest
        .split(',')
        .map(|arg| {
            let arg = arg.trim();
            arg.strip_prefix('$')
                .ok_or_else(|| {
                    format!("builtin call arguments must be captures ($name), got: {arg}")
                })
                .map(|s| s.to_string())
        })
        .collect();
    Ok((func, args?))
}

pub(super) fn parse_multiline_value(
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

pub(super) fn split_pattern_sequence_raw(value: &str, strip_asm_prefix: bool) -> Vec<String> {
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

pub(super) fn expand_in_blocks_for_asm(
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

pub(super) fn expand_in_blocks_for_ast(
    blocks: &mut Vec<Vec<AstPatternInBlock>>,
    sequence: &[AstPatternAstData],
) {
    if sequence.is_empty() {
        return;
    }

    if sequence.len() > 1 {
        for block in blocks.iter_mut() {
            set_clause(block, AstPatternInBlock::AstSequence(sequence.to_vec()));
        }
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

pub(super) fn expand_in_blocks_for_ir(
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

pub(super) fn expand_in_blocks_for_script(
    blocks: &mut Vec<Vec<AstPatternInBlock>>,
    script: &AstPatternScript,
) {
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

pub(super) fn update_all_in_blocks(
    blocks: &mut [Vec<AstPatternInBlock>],
    mut update: impl FnMut(&mut Vec<AstPatternInBlock>),
) {
    for block in blocks.iter_mut() {
        update(block);
    }
}

pub(super) fn parse_skip_range(value: &str) -> Result<AstPatternRange, String> {
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

pub(super) fn parse_rhai_script(script: &str) -> Result<AstPatternScript, String> {
    AstPatternScript::from_source(script)
}

pub(super) fn parse_apply_phases(value: &str) -> Result<Vec<AstPatternApplyAt>, String> {
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

pub(super) fn parse_apply_phase(value: &str) -> Result<AstPatternApplyAt, String> {
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

pub(super) fn parse_do_delete_target(value: &str) -> Result<AstPatternDeleteTarget, String> {
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

pub(super) fn parse_signed_offset(value: &str, name: &str) -> Result<isize, String> {
    value
        .trim()
        .parse::<isize>()
        .map_err(|err| format!("invalid del {name} `{value}`: {err}"))
}

/// Split a `[Pat1, Pat2, ...]` list by top-level commas, respecting nested
/// parentheses and brackets.
pub(super) fn split_stmt_seq_patterns(input: &str) -> Vec<String> {
    let trimmed = input.trim();
    // Strip the outer brackets
    let inner = if trimmed.starts_with('[') && trimmed.ends_with(']') {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };
    let mut results = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for ch in inner.chars() {
        match ch {
            '(' | '[' => {
                depth += 1;
                current.push(ch);
            }
            ')' | ']' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                let s = current.trim().to_string();
                if !s.is_empty() {
                    results.push(s);
                }
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }
    let s = current.trim().to_string();
    if !s.is_empty() {
        results.push(s);
    }
    results
}

pub(super) fn parse_log_action(line: &str, name: &str) -> Option<String> {
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

pub(super) fn parse_asm_replacement(value: &str) -> Result<AstPatternAsmData, String> {
    let text = value.trim();
    if text.is_empty() {
        return Err("asm replacement cannot be empty".to_string());
    }
    let normalized = text.strip_prefix("asm ").unwrap_or(text).trim();
    AstPatternAsmData::from_text(normalized)
}

pub(super) fn parse_ir_replacement(replacement: &str) -> AstPatternIrReplacement {
    AstPatternIrReplacement::from_text(replacement)
}

pub(super) fn parse_ast_replacement(replacement: &str) -> Result<AstStatement, String> {
    let text = replacement.trim();
    if text.eq_ignore_ascii_case("empty") {
        Ok(AstStatement::Empty)
    } else if text.eq_ignore_ascii_case("undefined") {
        Ok(AstStatement::Undefined)
    } else if text.eq_ignore_ascii_case("return") {
        Ok(AstStatement::Return(None))
    } else if let Some(content) = text.strip_prefix("comment ") {
        Ok(AstStatement::Comment(content.trim().to_string()))
    } else if let Some(content) = text.strip_prefix("asm ") {
        let statement = parse_asm_statement(content.trim())
            .ok_or_else(|| format!("invalid ast asm replacement `{}`", content.trim()))?;
        Ok(AstStatement::Ir(Box::new(statement)))
    } else if let Some(content) = text.strip_prefix("ir ") {
        Ok(parse_ir_statement(content.trim())
            .map(|stmt| AstStatement::Ir(Box::new(stmt)))
            .unwrap_or_else(|| AstStatement::Comment(text.to_string())))
    } else {
        Ok(AstStatement::Comment(text.to_string()))
    }
}

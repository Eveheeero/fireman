use super::*;

pub(super) fn external_symbol_identifier(name: &str) -> String {
    let demangled = demangle_symbol(name);
    let source = demangled.as_deref().unwrap_or(name);
    let mut out = String::from("ext_");
    for ch in source.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    if out.len() <= 4 {
        out.push_str("unknown");
    }
    out
}

fn demangle_symbol(name: &str) -> Option<String> {
    if let Ok(sym) = cpp_demangle::Symbol::new(name) {
        return sym.demangle().ok();
    }
    let demangled = rustc_demangle::demangle(name);
    let demangled_str = demangled.to_string();
    if demangled_str != name {
        return Some(demangled_str);
    }
    None
}

pub(super) fn external_symbol_name_from_slot(ast: &Ast, slot: u64) -> Option<String> {
    ast.pre_defined_symbols.get(&slot).cloned()
}

pub(super) fn external_slot_address_from_unknown_target(raw: &str) -> Option<u64> {
    let target = raw.trim();
    let target = target.strip_prefix('*').unwrap_or(target).trim();
    if let Some(hex) = target
        .strip_prefix("0x")
        .or_else(|| target.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16).ok()
    } else {
        target.parse::<u64>().ok()
    }
}

pub(super) fn external_slot_address_from_jump_target(jump: &AstJumpTarget) -> Option<u64> {
    match jump {
        AstJumpTarget::Unknown(raw) => external_slot_address_from_unknown_target(raw),
        AstJumpTarget::Variable {
            var_map, var_id, ..
        } => variable_constant_address(var_map, *var_id),
        _ => None,
    }
}

fn meaningful_statements(body: &[WrappedAstStatement]) -> Vec<&WrappedAstStatement> {
    body.iter()
        .filter(|stmt| {
            !matches!(
                stmt.statement,
                AstStatement::Comment(_) | AstStatement::Empty | AstStatement::Label(_)
            )
        })
        .collect()
}

pub(super) fn infer_external_import_symbol_from_body(
    ast: &Ast,
    body: &[WrappedAstStatement],
) -> Option<String> {
    let meaningful = meaningful_statements(body);
    if meaningful.len() != 1 {
        return None;
    }
    let AstStatement::Goto(jump) = &meaningful[0].statement else {
        return None;
    };
    let slot = external_slot_address_from_jump_target(jump)?;
    Some(external_symbol_name_from_slot(ast, slot).unwrap_or_else(|| format!("0x{:X}", slot)))
}

pub(super) fn collect_external_import_thunks(ast: &Ast) -> HashMap<AstFunctionId, String> {
    let mut out: HashMap<AstFunctionId, String> = HashMap::new();
    let functions = ast.functions.read().unwrap();
    let mut function_ids: Vec<_> = ast.function_versions.keys().copied().collect();
    function_ids.sort_unstable_by_key(|id| id.address);
    for function_id in function_ids {
        let Some(version) = ast.function_versions.get(&function_id).copied() else {
            continue;
        };
        let Some(function) = functions.get(&function_id).and_then(|m| m.get(&version)) else {
            continue;
        };
        if let Some(name) = infer_external_import_symbol_from_body(ast, &function.body) {
            out.insert(function_id, name);
        }
    }
    out
}

pub(super) fn rewrite_external_import_calls_recursive(
    body: &mut [WrappedAstStatement],
    external_thunks: &HashMap<AstFunctionId, String>,
) {
    for stmt in body.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, true_branch, false_branch) => {
                rewrite_external_import_calls_recursive(true_branch, external_thunks);
                if let Some(false_branch) = false_branch {
                    rewrite_external_import_calls_recursive(false_branch, external_thunks);
                }
            }
            AstStatement::While(_, block) | AstStatement::Block(block) => {
                rewrite_external_import_calls_recursive(block, external_thunks);
            }
            AstStatement::For(_, _, _, block) => {
                rewrite_external_import_calls_recursive(block, external_thunks);
            }
            AstStatement::Call(AstCall::Function { target, args }) => {
                if let Some(symbol) = external_thunks.get(target) {
                    let args = std::mem::take(args);
                    stmt.statement = AstStatement::Call(AstCall::Unknown(
                        external_symbol_identifier(symbol),
                        args,
                    ));
                }
            }
            _ => {}
        }
    }
}

pub(super) fn apply_external_import_thunk_names(
    ast: &Ast,
    external_thunks: &HashMap<AstFunctionId, String>,
) {
    if external_thunks.is_empty() {
        return;
    }
    let mut entries: Vec<_> = external_thunks.iter().collect();
    entries.sort_unstable_by_key(|(function_id, _)| function_id.address);

    let mut functions = ast.functions.write().unwrap();
    for (function_id, symbol) in entries {
        let Some(version) = ast.function_versions.get(function_id).copied() else {
            continue;
        };
        let Some(function) = functions
            .get_mut(function_id)
            .and_then(|m| m.get_mut(&version))
        else {
            continue;
        };
        function.name = Some(external_symbol_identifier(symbol));
        function.parameters.clear();
    }
}

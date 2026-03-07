use crate::{
    abstract_syntax_tree::{
        ArcAstVariableMap, Ast, AstBuiltinFunctionArgument, AstCall, AstExpression, AstFunction,
        AstFunctionId, AstFunctionVersion, AstJumpTarget, AstParameter, AstParameterLocation,
        AstStatement, AstStatementOrigin, AstValueOrigin, AstValueType, AstVariable, AstVariableId,
        ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    ir::{
        Architecture, Register, VirtualMachine,
        analyze::variables::resolve_operand,
        data::{IrData, IrDataAccessType, IrDataOperation},
        operator::{IrBinaryOperator, IrUnaryOperator},
        statements::IrStatement,
        x86_64::X64Range,
    },
    prelude::{DecompileError, *},
    utils::version_map::VersionMap,
};
use either::Either;
use hashbrown::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RegKey {
    architecture: Architecture,
    bit_start: usize,
}

impl RegKey {
    #[inline]
    fn from_register(reg: &Register) -> Self {
        Self {
            architecture: reg.architecture(),
            bit_start: reg.bit_range().start,
        }
    }

    #[inline]
    fn sort_key(&self) -> (u8, usize) {
        (architecture_sort_key(self.architecture), self.bit_start)
    }
}

#[inline]
const fn architecture_sort_key(architecture: Architecture) -> u8 {
    match architecture {
        Architecture::X64 => 0,
    }
}

type RegName = RegKey;
type RegNameToVarMap = HashMap<RegName, AstVariableId>;
type RegNameExprMap = HashMap<RegName, Wrapped<AstExpression>>;
type RegNameSet = HashSet<RegName>;

pub(super) fn analyze_call_arguments(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let variables;
    let mut body;
    let function_ir;
    let function_return_type;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();

        body = std::mem::take(&mut function.body);
        variables = function.variables.clone();
        function_ir = function.ir.clone();
        function_return_type = function.return_type.clone();
    }

    log_read_locations_for_call_arg_analysis(function_id, function_version, &variables);

    // We rebuild call nodes from IR (JumpByCall) and infer args from surrounding assignments.
    let reg_name_to_var = build_register_name_to_var_map(&variables);
    let var_id_to_reg = build_var_id_to_register_name_map(&variables);
    let _data_to_var = build_data_location_to_var_map(&variables);

    let var_id_to_rsp_offset = build_var_id_to_rsp_offset_map(&variables);
    let rsp_offset_to_var = build_rsp_offset_to_var_map(&var_id_to_rsp_offset);

    // Conservative mode:
    // Avoid collapsing IR groups into new call nodes, which can misclassify CFG jumps.
    // Instead, only enrich already-existing call nodes with inferred arguments.
    enrich_existing_call_args_recursive(
        ast,
        function_id,
        &variables,
        &mut body,
        &reg_name_to_var,
        &var_id_to_reg,
        &var_id_to_rsp_offset,
        &rsp_offset_to_var,
    );

    // Convert goto(function) forms into call(...) when safe/possible.
    rebuild_goto_as_call_if_possible(ast, function_id, &mut body);

    // Goto -> call conversion can create new call sites; enrich args again.
    enrich_existing_call_args_recursive(
        ast,
        function_id,
        &variables,
        &mut body,
        &reg_name_to_var,
        &var_id_to_reg,
        &var_id_to_rsp_offset,
        &rsp_offset_to_var,
    );

    // Rewrite external import thunk targets in calls to explicit external symbols.
    let mut external_import_thunks = collect_external_import_thunks(ast);
    if let Some(name) = infer_external_import_symbol_from_body(ast, &body) {
        external_import_thunks.insert(function_id, name);
    }
    rewrite_external_import_calls_recursive(&mut body, &external_import_thunks);
    apply_external_import_thunk_names(ast, &external_import_thunks);

    // If this function contains function-call sites with inferred arguments,
    // propagate that information back into callee signatures when they are missing.
    propagate_observed_call_args_to_callee_parameters(ast, &body);

    merge_single_non_recursive_callees(ast, function_id, &variables, &mut body);
    // Merging can introduce fresh goto-only tails from inlined callees.
    // Re-run goto->call rewriting so nested inlined branches are normalized too.
    rebuild_goto_as_call_if_possible(ast, function_id, &mut body);
    // The rewrite above can expose new zero-arg direct calls created from nested gotos.
    // Run merge once more so single-use non-recursive callees created by that rewrite
    // are still folded back into the caller.
    merge_single_non_recursive_callees(ast, function_id, &variables, &mut body);
    rebuild_goto_as_call_if_possible(ast, function_id, &mut body);

    if let Some(main_target) = detect_real_main_target_from_startup(ast, &body) {
        apply_real_main_special_case(ast, function_id, main_target, &mut body);
    }

    // Function-level CFG safety: once a top-level goto/return is emitted,
    // subsequent statements are unreachable in this linear AST view.
    let split_tail = truncate_after_terminal_control_flow(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function
            .processed_optimizations
            .push(ProcessedOptimization::CallArgumentAnalyzation);
        if let Some(name) = external_import_thunks.get(&function_id) {
            function.name = Some(external_symbol_identifier(name));
            function.parameters.clear();
        }
        function.body = body;
    }

    if let Some((target_addr, mut tail_body)) = split_tail {
        let tail_scope = AstFunctionId {
            address: target_addr,
        };
        rebuild_goto_as_call_if_possible(ast, tail_scope, &mut tail_body);
        materialize_split_tail_function(
            ast,
            target_addr,
            tail_body,
            variables,
            function_ir,
            function_return_type,
        );
    }

    Ok(())
}

fn log_read_locations_for_call_arg_analysis(
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
    variables: &ArcAstVariableMap,
) {
    trace!(
        ?function_id,
        function_version = ?function_version.0,
        "call argument analysis started"
    );

    let vars = variables.read().unwrap();
    let mut vars_sorted: Vec<_> = vars.iter().collect();
    vars_sorted.sort_unstable_by_key(|(var_id, _)| var_id.index);
    for (var_id, var) in vars_sorted {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        let mut access_map_sorted: Vec<_> = access_map.iter().collect();
        access_map_sorted.sort_unstable_by_key(|(ir_index, _)| {
            (ir_index.ir_index(), *ir_index.statement_index())
        });
        for (ir_index, accesses) in access_map_sorted {
            for access in accesses {
                if *access.access_type() != IrDataAccessType::Read {
                    continue;
                }
                trace!(
                    var_id = ?var_id.index,
                    ir_index = ?ir_index.ir_index(),
                    location = ?access.location().to_string(),
                    "-"
                );
            }
        }
    }
}

fn external_symbol_identifier(name: &str) -> String {
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

/// Try to demangle a C++ or Rust mangled symbol name.
/// Returns `Some(demangled)` if the name was successfully demangled, `None` otherwise.
fn demangle_symbol(name: &str) -> Option<String> {
    // Try C++ (Itanium ABI) demangling
    if let Ok(sym) = cpp_demangle::Symbol::new(name) {
        return sym.demangle().ok();
    }
    // Try Rust demangling as fallback
    let demangled = rustc_demangle::demangle(name);
    let demangled_str = demangled.to_string();
    if demangled_str != name {
        return Some(demangled_str);
    }
    None
}

fn external_symbol_name_from_slot(ast: &Ast, slot: u64) -> Option<String> {
    ast.pre_defined_symbols.get(&slot).cloned()
}

fn external_slot_address_from_unknown_target(raw: &str) -> Option<u64> {
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

fn external_slot_address_from_jump_target(
    jump: &crate::abstract_syntax_tree::AstJumpTarget,
) -> Option<u64> {
    match jump {
        crate::abstract_syntax_tree::AstJumpTarget::Unknown(raw) => {
            external_slot_address_from_unknown_target(raw)
        }
        crate::abstract_syntax_tree::AstJumpTarget::Variable {
            var_map, var_id, ..
        } => variable_constant_address(var_map, *var_id),
        _ => None,
    }
}

fn meaningful_statements<'a>(body: &'a [WrappedAstStatement]) -> Vec<&'a WrappedAstStatement> {
    body.iter()
        .filter(|stmt| {
            !matches!(
                stmt.statement,
                AstStatement::Comment(_) | AstStatement::Empty | AstStatement::Label(_)
            )
        })
        .collect()
}

fn infer_external_import_symbol_from_body(
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

fn collect_external_import_thunks(ast: &Ast) -> HashMap<AstFunctionId, String> {
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

fn rewrite_external_import_calls_recursive(
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

fn apply_external_import_thunk_names(ast: &Ast, external_thunks: &HashMap<AstFunctionId, String>) {
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

fn propagate_observed_call_args_to_callee_parameters(ast: &Ast, body: &[WrappedAstStatement]) {
    let mut observed: HashMap<AstFunctionId, usize> = HashMap::new();
    collect_observed_call_arg_counts(body, &mut observed);
    if observed.is_empty() {
        return;
    }

    let mut funcs = ast.functions.write().unwrap();
    for (target, argc) in observed {
        let Some(version) = ast.function_versions.get(&target).copied() else {
            continue;
        };
        let Some(func) = funcs.get_mut(&target).and_then(|m| m.get_mut(&version)) else {
            continue;
        };

        if func.parameters.len() >= argc {
            continue;
        }

        let mut params = func.parameters.clone();
        for idx in params.len()..argc {
            let location = if idx == 0 {
                AstParameterLocation::Register(
                    IrData::Register(<VirtualMachine as X64Range>::rcx()).into(),
                )
            } else if idx == 1 {
                AstParameterLocation::Register(
                    IrData::Register(<VirtualMachine as X64Range>::rdx()).into(),
                )
            } else if idx == 2 {
                AstParameterLocation::Register(
                    IrData::Register(<VirtualMachine as X64Range>::r8()).into(),
                )
            } else if idx == 3 {
                AstParameterLocation::Register(
                    IrData::Register(<VirtualMachine as X64Range>::r9()).into(),
                )
            } else {
                let off_rbp = 0x30 + ((idx - 4) as isize * 8);
                AstParameterLocation::Stack(off_rbp)
            };
            params.push(AstParameter {
                location,
                id: Either::Right(format!("p{}", idx + 1)),
            });
        }
        func.parameters = params;
    }
}

fn detect_real_main_target_from_startup(
    ast: &Ast,
    body: &[WrappedAstStatement],
) -> Option<AstFunctionId> {
    if !has_crt_startup_markers(ast, body) {
        return None;
    }

    let mut ordered_calls = Vec::new();
    collect_calls_in_order(body, &mut ordered_calls);
    let termination_boundary = ordered_calls.iter().rposition(|call| {
        call_matches_symbol(ast, call, is_termination_symbol)
            || call_matches_symbol(ast, call, is_amsg_exit_symbol)
    });
    let search_end = termination_boundary.unwrap_or(ordered_calls.len());
    let last_initterm_call = ordered_calls[..search_end]
        .iter()
        .rposition(|call| call_matches_symbol(ast, call, is_initterm_symbol))?;

    let mut fallback_target = None;
    for call in ordered_calls[last_initterm_call + 1..search_end]
        .iter()
        .rev()
    {
        let Some((target, argc)) = resolve_call_target_and_arity(ast, call) else {
            continue;
        };
        if is_runtime_startup_helper_target(ast, target) {
            continue;
        }
        if argc >= 3 {
            return Some(target);
        }
        if fallback_target.is_none() {
            fallback_target = Some(target);
        }
    }

    fallback_target
}

fn has_crt_startup_markers(ast: &Ast, body: &[WrappedAstStatement]) -> bool {
    let mut has_initterm = false;
    let mut has_local_termination = false;
    visit_calls_recursive(body, &mut |call| {
        if call_matches_symbol(ast, call, is_initterm_symbol) {
            has_initterm = true;
        }
        if call_matches_symbol(ast, call, is_termination_symbol)
            || call_matches_symbol(ast, call, is_amsg_exit_symbol)
        {
            has_local_termination = true;
        }
    });
    let has_global_termination = ast
        .pre_defined_symbols
        .values()
        .any(|name| is_termination_symbol(name) || is_amsg_exit_symbol(name));
    has_initterm && (has_local_termination || has_global_termination)
}

fn visit_calls_recursive(body: &[WrappedAstStatement], visitor: &mut impl FnMut(&AstCall)) {
    for stmt in body.iter() {
        match &stmt.statement {
            AstStatement::Call(call) => visitor(call),
            AstStatement::If(_, true_branch, false_branch) => {
                visit_calls_recursive(true_branch, visitor);
                if let Some(false_branch) = false_branch {
                    visit_calls_recursive(false_branch, visitor);
                }
            }
            AstStatement::While(_, block) | AstStatement::Block(block) => {
                visit_calls_recursive(block, visitor);
            }
            AstStatement::For(_, _, _, block) => {
                visit_calls_recursive(block, visitor);
            }
            _ => {}
        }
    }
}

fn collect_calls_in_order<'a>(body: &'a [WrappedAstStatement], out: &mut Vec<&'a AstCall>) {
    for stmt in body.iter() {
        match &stmt.statement {
            AstStatement::Call(call) => out.push(call),
            AstStatement::If(_, true_branch, false_branch) => {
                collect_calls_in_order(true_branch, out);
                if let Some(false_branch) = false_branch {
                    collect_calls_in_order(false_branch, out);
                }
            }
            AstStatement::While(_, block) | AstStatement::Block(block) => {
                collect_calls_in_order(block, out);
            }
            AstStatement::For(_, _, _, block) => {
                collect_calls_in_order(block, out);
            }
            _ => {}
        }
    }
}

fn resolve_call_target_and_arity(ast: &Ast, call: &AstCall) -> Option<(AstFunctionId, usize)> {
    match call {
        AstCall::Function { target, args } => Some((*target, args.len())),
        AstCall::Unknown(name, args) => {
            let addr = parse_default_function_name(name)?;
            let target = resolve_function_id_by_address(ast, addr)?;
            Some((target, args.len()))
        }
        _ => None,
    }
}

fn is_runtime_startup_helper_target(ast: &Ast, target: AstFunctionId) -> bool {
    let Some(name) = function_symbol_hint(ast, target) else {
        return false;
    };
    let lower = name.to_ascii_lowercase();
    lower.starts_with("ext_")
        || is_initterm_symbol(&lower)
        || is_termination_symbol(&lower)
        || is_amsg_exit_symbol(&lower)
}

fn merge_single_non_recursive_callees(
    ast: &mut Ast,
    caller_id: AstFunctionId,
    caller_variables: &ArcAstVariableMap,
    caller_body: &mut Vec<WrappedAstStatement>,
) {
    let mut active_function_ids: Vec<_> = ast.function_versions.keys().copied().collect();
    if active_function_ids.is_empty() {
        return;
    }
    active_function_ids.sort_unstable_by_key(|id| id.address);

    let caller_targets = collect_called_function_ids(caller_body);
    if caller_targets.is_empty() {
        return;
    }

    let mut function_bodies: HashMap<AstFunctionId, Vec<WrappedAstStatement>> = HashMap::new();
    {
        let functions = ast.functions.read().unwrap();
        for function_id in active_function_ids.iter().copied() {
            if function_id == caller_id {
                function_bodies.insert(function_id, caller_body.clone());
                continue;
            }
            let Some(version) = ast.function_versions.get(&function_id).copied() else {
                continue;
            };
            let Some(function) = functions.get(&function_id).and_then(|m| m.get(&version)) else {
                continue;
            };
            function_bodies.insert(function_id, function.body.clone());
        }
    }

    let active_function_set: HashSet<AstFunctionId> = active_function_ids.iter().copied().collect();
    let mut call_counts: HashMap<AstFunctionId, usize> = HashMap::new();
    let mut call_graph: HashMap<AstFunctionId, HashSet<AstFunctionId>> = HashMap::new();
    for function_id in active_function_ids.iter().copied() {
        let edges = call_graph.entry(function_id).or_default();
        let Some(body) = function_bodies.get(&function_id) else {
            continue;
        };

        let mut called_ids = Vec::new();
        collect_called_function_ids_recursive(body, &mut called_ids);
        for callee_id in called_ids.iter().copied() {
            *call_counts.entry(callee_id).or_insert(0) += 1;
            if active_function_set.contains(&callee_id) {
                edges.insert(callee_id);
            }
        }
    }

    let recursive_functions = detect_recursive_functions(&call_graph);
    let mut replacements: HashMap<AstFunctionId, Vec<WrappedAstStatement>> = HashMap::new();
    let mut callee_variable_maps: HashMap<AstFunctionId, ArcAstVariableMap> = HashMap::new();
    {
        let functions = ast.functions.read().unwrap();
        for target in caller_targets {
            if target == caller_id {
                continue;
            }
            if call_counts.get(&target).copied().unwrap_or(0) != 1 {
                continue;
            }
            if recursive_functions.contains(&target) {
                continue;
            }

            let Some(version) = ast.function_versions.get(&target).copied() else {
                continue;
            };
            let Some(callee) = functions.get(&target).and_then(|m| m.get(&version)) else {
                continue;
            };
            if !callee.parameters.is_empty() {
                continue;
            }
            if infer_external_import_symbol_from_body(ast, &callee.body).is_some() {
                continue;
            }
            replacements.insert(target, strip_trailing_void_return(callee.body.clone()));
            callee_variable_maps.insert(target, callee.variables.clone());
        }
    }

    if replacements.is_empty() {
        return;
    }

    let inlined_callees = inline_single_call_callees_recursive(caller_body, &replacements);
    if inlined_callees.is_empty() {
        return;
    }

    let mut merged_vars = Vec::new();
    for callee_id in inlined_callees.iter().copied() {
        let Some(callee_var_map) = callee_variable_maps.get(&callee_id) else {
            continue;
        };
        let callee_vars = callee_var_map.read().unwrap();
        merged_vars.extend(
            callee_vars
                .iter()
                .map(|(var_id, var)| (*var_id, var.clone())),
        );
    }
    {
        let mut caller_vars = caller_variables.write().unwrap();
        for (var_id, var) in merged_vars {
            caller_vars.entry(var_id).or_insert(var);
        }
        rename_merged_default_variables_by_scope(&mut caller_vars, caller_id);
        deduplicate_variable_names_in_scope(&mut caller_vars, caller_id);
    }
    rebind_statement_variable_maps(caller_body, &caller_variables);
    {
        let caller_vars = caller_variables.read().unwrap();
        synchronize_declaration_variable_names(caller_body, &caller_vars);
    }

    {
        let mut functions = ast.functions.write().unwrap();
        for callee_id in inlined_callees.iter().copied() {
            functions.remove(&callee_id);
        }
    }
    for callee_id in inlined_callees {
        ast.function_versions.remove(&callee_id);
    }
}

fn deduplicate_variable_names_in_scope(
    vars: &mut HashMap<AstVariableId, AstVariable>,
    caller_id: AstFunctionId,
) {
    let mut ids: Vec<_> = vars.keys().copied().collect();
    ids.sort_unstable_by_key(|id| {
        let caller_priority = if id.parent == Some(caller_id) {
            0u8
        } else {
            1u8
        };
        (
            caller_priority,
            id.index,
            id.parent.map(|x| x.address).unwrap_or(0),
        )
    });

    let mut used_names: HashSet<String> = HashSet::new();
    for id in ids {
        let Some(var) = vars.get_mut(&id) else {
            continue;
        };
        let base = var
            .name
            .clone()
            .unwrap_or_else(|| var.id.get_default_name());
        if used_names.insert(base.clone()) {
            continue;
        }

        let mut suffix = 2usize;
        let mut candidate = format!("{}_{}", base, suffix);
        while used_names.contains(&candidate) {
            suffix += 1;
            candidate = format!("{}_{}", base, suffix);
        }
        var.name = Some(candidate.clone());
        used_names.insert(candidate);
    }
}

fn rename_merged_default_variables_by_scope(
    vars: &mut HashMap<AstVariableId, AstVariable>,
    caller_id: AstFunctionId,
) {
    let mut callee_scopes: Vec<_> = vars
        .keys()
        .filter_map(|id| {
            if id.parent.is_some() && id.parent != Some(caller_id) {
                id.parent
            } else {
                None
            }
        })
        .collect();
    callee_scopes.sort_unstable_by_key(|scope| scope.address);
    callee_scopes.dedup();
    if callee_scopes.is_empty() {
        return;
    }

    let scope_suffix_map: HashMap<AstFunctionId, usize> = callee_scopes
        .into_iter()
        .enumerate()
        .map(|(idx, scope)| (scope, idx + 1))
        .collect();

    let mut used_names: HashSet<String> = vars.values().map(|var| var.name()).collect();
    let mut callee_ids: Vec<_> = vars
        .keys()
        .copied()
        .filter(|id| id.parent.is_some() && id.parent != Some(caller_id))
        .collect();
    callee_ids.sort_unstable_by_key(|id| {
        let scope_suffix = id
            .parent
            .and_then(|scope| scope_suffix_map.get(&scope).copied())
            .unwrap_or(usize::MAX);
        (scope_suffix, id.index)
    });

    for id in callee_ids {
        let Some(var) = vars.get_mut(&id) else {
            continue;
        };
        let Some(scope) = id.parent else {
            continue;
        };
        let Some(scope_suffix) = scope_suffix_map.get(&scope).copied() else {
            continue;
        };

        let default_name = id.get_default_name();
        let is_default_named = var.name.as_ref().is_none_or(|name| {
            *name == default_name || is_scope_suffixed_default_name(name, &default_name)
        });
        if !is_default_named {
            continue;
        }

        let current_name = var.name();
        used_names.remove(&current_name);

        let base_candidate = format!("{}_{}", default_name, scope_suffix);
        let mut candidate = base_candidate.clone();
        let mut extra_suffix = 2usize;
        while used_names.contains(&candidate) {
            candidate = format!("{}_{}", base_candidate, extra_suffix);
            extra_suffix += 1;
        }
        var.name = Some(candidate.clone());
        used_names.insert(candidate);
    }
}

fn is_scope_suffixed_default_name(name: &str, default_name: &str) -> bool {
    let Some(suffix) = name
        .strip_prefix(default_name)
        .and_then(|rest| rest.strip_prefix('_'))
    else {
        return false;
    };
    !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit())
}

fn synchronize_declaration_variable_names(
    body: &mut [WrappedAstStatement],
    vars: &HashMap<AstVariableId, AstVariable>,
) {
    for stmt in body.iter_mut() {
        synchronize_declaration_variable_names_in_statement(&mut stmt.statement, vars);
    }
}

fn synchronize_declaration_variable_names_in_statement(
    statement: &mut AstStatement,
    vars: &HashMap<AstVariableId, AstVariable>,
) {
    match statement {
        AstStatement::Declaration(var, _) => {
            if let Some(canonical) = vars.get(&var.id) {
                var.name = canonical.name.clone();
            }
        }
        AstStatement::If(_, then_body, else_body) => {
            synchronize_declaration_variable_names(then_body, vars);
            if let Some(else_body) = else_body {
                synchronize_declaration_variable_names(else_body, vars);
            }
        }
        AstStatement::While(_, body) | AstStatement::Block(body) => {
            synchronize_declaration_variable_names(body, vars);
        }
        AstStatement::For(init, _, step, body) => {
            synchronize_declaration_variable_names_in_statement(&mut init.statement, vars);
            synchronize_declaration_variable_names_in_statement(&mut step.statement, vars);
            synchronize_declaration_variable_names(body, vars);
        }
        _ => {}
    }
}

fn rebind_statement_variable_maps(body: &mut [WrappedAstStatement], variables: &ArcAstVariableMap) {
    for stmt in body.iter_mut() {
        rebind_statement_variable_maps_in_statement(&mut stmt.statement, variables);
    }
}

fn rebind_statement_variable_maps_in_statement(
    statement: &mut AstStatement,
    variables: &ArcAstVariableMap,
) {
    match statement {
        AstStatement::Declaration(_, init) => {
            if let Some(init) = init {
                rebind_statement_variable_maps_in_expression(init, variables);
            }
        }
        AstStatement::Assignment(lhs, rhs) => {
            rebind_statement_variable_maps_in_expression(lhs, variables);
            rebind_statement_variable_maps_in_expression(rhs, variables);
        }
        AstStatement::If(cond, then_body, else_body) => {
            rebind_statement_variable_maps_in_expression(cond, variables);
            rebind_statement_variable_maps(then_body, variables);
            if let Some(else_body) = else_body {
                rebind_statement_variable_maps(else_body, variables);
            }
        }
        AstStatement::While(cond, body) => {
            rebind_statement_variable_maps_in_expression(cond, variables);
            rebind_statement_variable_maps(body, variables);
        }
        AstStatement::For(init, cond, step, body) => {
            rebind_statement_variable_maps_in_statement(&mut init.statement, variables);
            rebind_statement_variable_maps_in_expression(cond, variables);
            rebind_statement_variable_maps_in_statement(&mut step.statement, variables);
            rebind_statement_variable_maps(body, variables);
        }
        AstStatement::Return(expr) => {
            if let Some(expr) = expr {
                rebind_statement_variable_maps_in_expression(expr, variables);
            }
        }
        AstStatement::Call(call) => {
            rebind_statement_variable_maps_in_call(call, variables);
        }
        AstStatement::Goto(AstJumpTarget::Variable { var_map, .. }) => {
            *var_map = variables.clone();
        }
        AstStatement::Block(body) => {
            rebind_statement_variable_maps(body, variables);
        }
        _ => {}
    }
}

fn rebind_statement_variable_maps_in_expression(
    expr: &mut Wrapped<AstExpression>,
    variables: &ArcAstVariableMap,
) {
    match &mut expr.item {
        AstExpression::Variable(var_map, _) => {
            *var_map = variables.clone();
        }
        AstExpression::UnaryOp(_, arg)
        | AstExpression::Cast(_, arg)
        | AstExpression::Deref(arg)
        | AstExpression::AddressOf(arg)
        | AstExpression::MemberAccess(arg, _) => {
            rebind_statement_variable_maps_in_expression(arg, variables);
        }
        AstExpression::BinaryOp(_, lhs, rhs) | AstExpression::ArrayAccess(lhs, rhs) => {
            rebind_statement_variable_maps_in_expression(lhs, variables);
            rebind_statement_variable_maps_in_expression(rhs, variables);
        }
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            rebind_statement_variable_maps_in_expression(cond, variables);
            rebind_statement_variable_maps_in_expression(true_expr, variables);
            rebind_statement_variable_maps_in_expression(false_expr, variables);
        }
        AstExpression::Call(call) => {
            rebind_statement_variable_maps_in_call(call, variables);
        }
        AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_) => {}
    }
}

fn rebind_statement_variable_maps_in_call(call: &mut AstCall, variables: &ArcAstVariableMap) {
    match call {
        AstCall::Variable { var_map, args, .. } => {
            *var_map = variables.clone();
            for arg in args.iter_mut() {
                rebind_statement_variable_maps_in_expression(arg, variables);
            }
        }
        AstCall::Function { args, .. } | AstCall::Unknown(_, args) => {
            for arg in args.iter_mut() {
                rebind_statement_variable_maps_in_expression(arg, variables);
            }
        }
        AstCall::Builtin(_, arg) => match arg.as_mut() {
            AstBuiltinFunctionArgument::None => {}
            AstBuiltinFunctionArgument::Print(args) => {
                for arg in args.iter_mut() {
                    rebind_statement_variable_maps_in_expression(arg, variables);
                }
            }
            AstBuiltinFunctionArgument::ByteSizeOf(arg)
            | AstBuiltinFunctionArgument::BitSizeOf(arg)
            | AstBuiltinFunctionArgument::OperandExists(arg)
            | AstBuiltinFunctionArgument::SignedMax(arg)
            | AstBuiltinFunctionArgument::SignedMin(arg)
            | AstBuiltinFunctionArgument::UnsignedMax(arg)
            | AstBuiltinFunctionArgument::UnsignedMin(arg)
            | AstBuiltinFunctionArgument::BitOnes(arg)
            | AstBuiltinFunctionArgument::BitZeros(arg) => {
                rebind_statement_variable_maps_in_expression(arg, variables);
            }
            AstBuiltinFunctionArgument::Sized(lhs, rhs) => {
                rebind_statement_variable_maps_in_expression(lhs, variables);
                rebind_statement_variable_maps_in_expression(rhs, variables);
            }
        },
    }
}

fn collect_called_function_ids(body: &[WrappedAstStatement]) -> HashSet<AstFunctionId> {
    let mut called_ids = Vec::new();
    collect_called_function_ids_recursive(body, &mut called_ids);
    called_ids.into_iter().collect()
}

fn collect_called_function_ids_recursive(
    body: &[WrappedAstStatement],
    called_ids: &mut Vec<AstFunctionId>,
) {
    for stmt in body.iter() {
        match &stmt.statement {
            AstStatement::Call(AstCall::Function { target, .. }) => called_ids.push(*target),
            AstStatement::If(_, branch_true, branch_false) => {
                collect_called_function_ids_recursive(branch_true, called_ids);
                if let Some(branch_false) = branch_false {
                    collect_called_function_ids_recursive(branch_false, called_ids);
                }
            }
            AstStatement::While(_, block) | AstStatement::Block(block) => {
                collect_called_function_ids_recursive(block, called_ids);
            }
            AstStatement::For(_, _, _, block) => {
                collect_called_function_ids_recursive(block, called_ids);
            }
            _ => {}
        }
    }
}

fn detect_recursive_functions(
    call_graph: &HashMap<AstFunctionId, HashSet<AstFunctionId>>,
) -> HashSet<AstFunctionId> {
    let mut recursive_functions = HashSet::new();
    let mut function_ids: Vec<_> = call_graph.keys().copied().collect();
    function_ids.sort_unstable_by_key(|id| id.address);

    for function_id in function_ids {
        if is_function_recursive(function_id, call_graph) {
            recursive_functions.insert(function_id);
        }
    }

    recursive_functions
}

fn is_function_recursive(
    start: AstFunctionId,
    call_graph: &HashMap<AstFunctionId, HashSet<AstFunctionId>>,
) -> bool {
    let mut stack: Vec<AstFunctionId> = call_graph
        .get(&start)
        .map(|targets| targets.iter().copied().collect())
        .unwrap_or_default();
    let mut visited: HashSet<AstFunctionId> = HashSet::new();

    while let Some(current) = stack.pop() {
        if current == start {
            return true;
        }
        if !visited.insert(current) {
            continue;
        }
        if let Some(next_targets) = call_graph.get(&current) {
            stack.extend(next_targets.iter().copied());
        }
    }

    false
}

fn inline_single_call_callees_recursive(
    stmts: &mut Vec<WrappedAstStatement>,
    replacements: &HashMap<AstFunctionId, Vec<WrappedAstStatement>>,
) -> HashSet<AstFunctionId> {
    let mut inlined_callees: HashSet<AstFunctionId> = HashSet::new();
    let mut rebuilt_body: Vec<WrappedAstStatement> = Vec::with_capacity(stmts.len());

    for mut stmt in std::mem::take(stmts) {
        match &mut stmt.statement {
            AstStatement::If(_, branch_true, branch_false) => {
                inlined_callees.extend(inline_single_call_callees_recursive(
                    branch_true,
                    replacements,
                ));
                if let Some(branch_false) = branch_false {
                    inlined_callees.extend(inline_single_call_callees_recursive(
                        branch_false,
                        replacements,
                    ));
                }
            }
            AstStatement::While(_, block) | AstStatement::Block(block) => {
                inlined_callees.extend(inline_single_call_callees_recursive(block, replacements));
            }
            AstStatement::For(_, _, _, block) => {
                inlined_callees.extend(inline_single_call_callees_recursive(block, replacements));
            }
            _ => {}
        }

        let AstStatement::Call(AstCall::Function { target, args }) = &stmt.statement else {
            rebuilt_body.push(stmt);
            continue;
        };
        if !args.is_empty() {
            rebuilt_body.push(stmt);
            continue;
        }
        let Some(replacement) = replacements.get(target) else {
            rebuilt_body.push(stmt);
            continue;
        };

        let mut inlined_body = replacement.clone();
        if let Some(callsite_comment) = stmt.comment.take() {
            if let Some(first) = inlined_body.first_mut() {
                if first.comment.is_none() {
                    first.comment = Some(callsite_comment);
                } else {
                    inlined_body.insert(
                        0,
                        WrappedAstStatement {
                            statement: AstStatement::Comment(callsite_comment),
                            origin: stmt.origin.clone(),
                            comment: None,
                        },
                    );
                }
            } else {
                inlined_body.push(WrappedAstStatement {
                    statement: AstStatement::Comment(callsite_comment),
                    origin: stmt.origin.clone(),
                    comment: None,
                });
            }
        }

        rebuilt_body.extend(inlined_body.into_iter());
        inlined_callees.insert(*target);
    }

    *stmts = rebuilt_body;
    inlined_callees
}

fn strip_trailing_void_return(mut body: Vec<WrappedAstStatement>) -> Vec<WrappedAstStatement> {
    if body
        .last()
        .is_some_and(|stmt| matches!(stmt.statement, AstStatement::Return(None)))
    {
        let removed = body.pop();
        if let Some(removed) = removed
            && let Some(comment) = removed.comment
        {
            body.push(WrappedAstStatement {
                statement: AstStatement::Comment(comment),
                origin: removed.origin,
                comment: None,
            });
        }
    }
    body
}

fn parse_default_function_name(name: &str) -> Option<u64> {
    let trimmed = name.trim();
    let hex = trimmed.strip_prefix('f')?;
    if hex.is_empty() || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    u64::from_str_radix(hex, 16).ok()
}

fn call_matches_symbol(ast: &Ast, call: &AstCall, matcher: fn(&str) -> bool) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => Some(name.clone()),
        AstCall::Function { target, .. } => function_symbol_hint(ast, *target),
        _ => None,
    };
    name.is_some_and(|name| matcher(&name))
}

fn function_symbol_hint(ast: &Ast, target: AstFunctionId) -> Option<String> {
    let version = ast.function_versions.get(&target).copied()?;
    let functions = ast.functions.read().unwrap();
    let function = functions.get(&target).and_then(|m| m.get(&version))?;
    if let Some(name) = &function.name {
        return Some(name.clone());
    }
    if let Some(symbol) = infer_external_import_symbol_from_body(ast, &function.body) {
        return Some(external_symbol_identifier(&symbol));
    }
    Some(target.get_default_name())
}

fn is_initterm_symbol(name: &str) -> bool {
    name.to_ascii_lowercase().contains("initterm")
}

fn is_amsg_exit_symbol(name: &str) -> bool {
    name.to_ascii_lowercase().contains("amsg_exit")
}

fn is_termination_symbol(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with("_exit")
        || lower.ends_with("__exit")
        || lower.ends_with("cexit")
        || lower == "exit"
        || lower == "_exit"
        || lower == "_cexit"
}

fn apply_real_main_special_case(
    ast: &Ast,
    startup_function_id: AstFunctionId,
    main_target: AstFunctionId,
    body: &mut Vec<WrappedAstStatement>,
) {
    {
        let mut functions = ast.functions.write().unwrap();

        if let Some(version) = ast.function_versions.get(&main_target).copied()
            && let Some(main_function) = functions
                .get_mut(&main_target)
                .and_then(|m| m.get_mut(&version))
        {
            let default_main_name = main_target.get_default_name();
            if should_apply_special_name(&main_function.name, &default_main_name) {
                main_function.name = Some("main".to_string());
            }
        }

        if let Some(version) = ast.function_versions.get(&startup_function_id).copied()
            && let Some(startup_function) = functions
                .get_mut(&startup_function_id)
                .and_then(|m| m.get_mut(&version))
        {
            let default_startup_name = startup_function_id.get_default_name();
            if should_apply_special_name(&startup_function.name, &default_startup_name) {
                startup_function.name = Some("__tmainCRTStartup".to_string());
            }
        }
    }

    rewrite_calls_to_named_target(body, main_target, "main");
    if has_named_call_recursive(body, "main") && !has_top_level_named_call(body, "main") {
        let inherited_origin =
            find_named_call_origin_recursive(body, "main").unwrap_or(AstStatementOrigin::Unknown);
        body.insert(
            0,
            WrappedAstStatement {
                statement: AstStatement::Call(AstCall::Unknown("main".to_string(), Vec::new())),
                origin: inherited_origin,
                comment: None,
            },
        );
    }
}

fn should_apply_special_name(current_name: &Option<String>, default_name: &str) -> bool {
    match current_name {
        None => true,
        Some(existing) => existing == default_name,
    }
}

fn rewrite_calls_to_named_target(
    body: &mut [WrappedAstStatement],
    target: AstFunctionId,
    replacement_name: &str,
) {
    for stmt in body.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, true_branch, false_branch) => {
                rewrite_calls_to_named_target(true_branch, target, replacement_name);
                if let Some(false_branch) = false_branch {
                    rewrite_calls_to_named_target(false_branch, target, replacement_name);
                }
            }
            AstStatement::While(_, block) | AstStatement::Block(block) => {
                rewrite_calls_to_named_target(block, target, replacement_name);
            }
            AstStatement::For(_, _, _, block) => {
                rewrite_calls_to_named_target(block, target, replacement_name);
            }
            AstStatement::Call(AstCall::Function {
                target: call_target,
                args,
            }) if *call_target == target => {
                let args = std::mem::take(args);
                stmt.statement =
                    AstStatement::Call(AstCall::Unknown(replacement_name.to_string(), args));
            }
            _ => {}
        }
    }
}

fn has_named_call_recursive(body: &[WrappedAstStatement], name: &str) -> bool {
    let mut found = false;
    visit_calls_recursive(body, &mut |call| {
        if let AstCall::Unknown(call_name, _) = call
            && call_name == name
        {
            found = true;
        }
    });
    found
}

fn find_named_call_origin_recursive(
    body: &[WrappedAstStatement],
    name: &str,
) -> Option<AstStatementOrigin> {
    for stmt in body.iter() {
        match &stmt.statement {
            AstStatement::Call(AstCall::Unknown(call_name, _)) if call_name == name => {
                return Some(stmt.origin.clone());
            }
            AstStatement::If(_, true_branch, false_branch) => {
                if let Some(origin) = find_named_call_origin_recursive(true_branch, name) {
                    return Some(origin);
                }

                if let Some(false_branch) = false_branch
                    && let Some(origin) = find_named_call_origin_recursive(false_branch, name)
                {
                    return Some(origin);
                }
            }
            AstStatement::While(_, block) | AstStatement::Block(block) => {
                if let Some(origin) = find_named_call_origin_recursive(block, name) {
                    return Some(origin);
                }
            }
            AstStatement::For(_, _, _, block) => {
                if let Some(origin) = find_named_call_origin_recursive(block, name) {
                    return Some(origin);
                }
            }
            _ => {}
        }
    }

    None
}

fn has_top_level_named_call(body: &[WrappedAstStatement], name: &str) -> bool {
    body.iter().any(|stmt| {
        matches!(
            &stmt.statement,
            AstStatement::Call(AstCall::Unknown(call_name, _)) if call_name == name
        )
    })
}

fn truncate_after_terminal_control_flow(
    body: &mut Vec<WrappedAstStatement>,
) -> Option<(u64, Vec<WrappedAstStatement>)> {
    let cut = body.iter().position(|stmt| {
        matches!(
            stmt.statement,
            AstStatement::Goto(_) | AstStatement::Return(_)
        )
    });
    let Some(idx) = cut else {
        return None;
    };

    let mut split_target = None;
    if let AstStatement::Goto(jump) = &body[idx].statement {
        split_target = jump_target_constant_address(jump);
    }

    let tail = if idx + 1 < body.len() {
        body[idx + 1..].to_vec()
    } else {
        Vec::new()
    };
    body.truncate(idx + 1);

    if let Some(addr) = split_target
        && !tail.is_empty()
    {
        return Some((addr, tail));
    }

    None
}

fn materialize_split_tail_function(
    ast: &mut Ast,
    target_addr: u64,
    tail_body: Vec<WrappedAstStatement>,
    variables: ArcAstVariableMap,
    function_ir: std::sync::Arc<crate::ir::analyze::ir_function::IrFunction>,
    function_return_type: AstValueType,
) {
    let target = AstFunctionId {
        address: target_addr,
    };
    let mut functions = ast.functions.write().unwrap();
    if functions.contains_key(&target) {
        return;
    }

    let new_func = AstFunction {
        name: None,
        id: target,
        ir: function_ir,
        return_type: function_return_type,
        parameters: Vec::new(),
        variables,
        body: tail_body,
        processed_optimizations: Vec::new(),
    };
    functions.insert(target, VersionMap::new(AstFunctionVersion(1), new_func));
    ast.function_versions.insert(target, AstFunctionVersion(1));
}

fn collect_observed_call_arg_counts(
    body: &[WrappedAstStatement],
    out: &mut HashMap<AstFunctionId, usize>,
) {
    for stmt in body.iter() {
        match &stmt.statement {
            AstStatement::Call(AstCall::Function { target, args }) => {
                if !args.is_empty() {
                    let entry = out.entry(*target).or_insert(0);
                    *entry = (*entry).max(args.len());
                }
            }
            AstStatement::If(_, t, f) => {
                collect_observed_call_arg_counts(t, out);
                if let Some(f) = f {
                    collect_observed_call_arg_counts(f, out);
                }
            }
            AstStatement::While(_, b) | AstStatement::Block(b) => {
                collect_observed_call_arg_counts(b, out);
            }
            AstStatement::For(_, _, _, b) => {
                collect_observed_call_arg_counts(b, out);
            }
            _ => {}
        }
    }
}

fn rebuild_goto_as_call_if_possible(
    ast: &Ast,
    scope: AstFunctionId,
    body: &mut Vec<WrappedAstStatement>,
) {
    rebuild_goto_as_call_if_possible_inner(ast, scope, body, 0);
}

fn rebuild_goto_as_call_if_possible_inner(
    ast: &Ast,
    scope: AstFunctionId,
    body: &mut Vec<WrappedAstStatement>,
    depth: usize,
) {
    for stmt in body.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, t, f) => {
                rebuild_goto_as_call_if_possible_inner(ast, scope, t, depth + 1);
                if let Some(f) = f {
                    rebuild_goto_as_call_if_possible_inner(ast, scope, f, depth + 1);
                }
            }
            AstStatement::While(_, b) | AstStatement::Block(b) => {
                rebuild_goto_as_call_if_possible_inner(ast, scope, b, depth + 1);
            }
            AstStatement::For(_, _, _, b) => {
                rebuild_goto_as_call_if_possible_inner(ast, scope, b, depth + 1)
            }
            _ => {}
        }
    }

    let original = std::mem::take(body);
    let total_len = original.len();
    let mut rebuilt: Vec<WrappedAstStatement> = Vec::with_capacity(total_len + 1);
    for idx in 0..total_len {
        let mut stmt = original[idx].clone();
        let mut should_stop_after = false;
        if let AstStatement::Goto(jump) = &stmt.statement {
            let tail_position = original_suffix_is_non_effectful(&original, idx + 1);
            if tail_position && let Some(call) = jump_target_to_call(ast, scope, &stmt, jump) {
                stmt.statement = AstStatement::Call(call);
                should_stop_after = true;
            }
        }

        rebuilt.push(stmt);

        if should_stop_after {
            break;
        }
    }

    *body = rebuilt;
}

fn original_suffix_is_non_effectful(body: &[WrappedAstStatement], start: usize) -> bool {
    if start >= body.len() {
        return true;
    }
    body[start..].iter().all(|x| {
        matches!(
            x.statement,
            AstStatement::Comment(_) | AstStatement::Empty | AstStatement::Label(_)
        )
    })
}

fn stmt_origin_is_unconditional_jump(stmt: &WrappedAstStatement) -> bool {
    let Some(ir_stmt) = stmt_origin_ir_statement(stmt) else {
        return false;
    };
    if !matches!(ir_stmt, IrStatement::Jump { .. }) {
        return false;
    }

    let Some(inst) = stmt_origin_instruction(stmt) else {
        return false;
    };

    inst.is_jmp() && !inst.is_jcc() && !inst.is_call()
}

fn jump_target_to_call(
    ast: &Ast,
    current_scope: AstFunctionId,
    stmt: &WrappedAstStatement,
    jump: &crate::abstract_syntax_tree::AstJumpTarget,
) -> Option<AstCall> {
    match jump {
        crate::abstract_syntax_tree::AstJumpTarget::Function { target } => {
            if *target == current_scope {
                None
            } else {
                Some(AstCall::Function {
                    target: *target,
                    args: Vec::new(),
                })
            }
        }
        crate::abstract_syntax_tree::AstJumpTarget::Unknown(raw) => {
            if let Some(addr) = external_slot_address_from_unknown_target(raw) {
                call_from_jump_address(ast, current_scope, addr)
            } else {
                Some(AstCall::Unknown(raw.clone(), Vec::new()))
            }
        }
        crate::abstract_syntax_tree::AstJumpTarget::Variable {
            scope: var_scope,
            var_map,
            var_id,
        } => {
            if let Some(addr) = variable_constant_address(var_map, *var_id) {
                call_from_jump_address(ast, current_scope, addr)
            } else if let Some(addr) = stmt_origin_fallthrough_address(stmt) {
                call_from_jump_address(ast, current_scope, addr).or_else(|| {
                    Some(AstCall::Variable {
                        scope: *var_scope,
                        var_map: var_map.clone(),
                        var_id: *var_id,
                        args: Vec::new(),
                    })
                })
            } else {
                Some(AstCall::Variable {
                    scope: *var_scope,
                    var_map: var_map.clone(),
                    var_id: *var_id,
                    args: Vec::new(),
                })
            }
        }
        crate::abstract_syntax_tree::AstJumpTarget::Instruction { .. } => None,
    }
}

fn call_from_jump_address(ast: &Ast, scope: AstFunctionId, addr: u64) -> Option<AstCall> {
    if let Some(target) = resolve_function_id_by_address(ast, addr) {
        if target == scope {
            return None;
        }
        return Some(AstCall::Function {
            target,
            args: Vec::new(),
        });
    }

    if let Some(symbol) = external_symbol_name_from_slot(ast, addr) {
        return Some(AstCall::Unknown(
            external_symbol_identifier(&symbol),
            Vec::new(),
        ));
    }

    Some(AstCall::Unknown(
        function_like_name_from_address(ast, addr),
        Vec::new(),
    ))
}

fn stmt_origin_fallthrough_address(stmt: &WrappedAstStatement) -> Option<u64> {
    let AstStatementOrigin::Ir(desc) = &stmt.origin else {
        return None;
    };
    let ir_index = desc.descriptor().ir_index() as usize;
    let ir = desc.ir().get_ir().get(ir_index)?;
    let inst = desc.ir().get_instructions().get(ir_index)?;
    let inst_len = inst
        .inner
        .bytes
        .as_ref()
        .map(|x| x.len() as u64)
        .unwrap_or(0);
    Some(ir.address.get_virtual_address() + inst_len)
}

fn jump_target_constant_address(jump: &crate::abstract_syntax_tree::AstJumpTarget) -> Option<u64> {
    match jump {
        crate::abstract_syntax_tree::AstJumpTarget::Function { target } => Some(target.address),
        // Instruction-based jump targets are often intra-function control-flow labels.
        // Do not treat them as callable function addresses here.
        crate::abstract_syntax_tree::AstJumpTarget::Instruction { .. } => None,
        crate::abstract_syntax_tree::AstJumpTarget::Unknown(s) => {
            let stripped = s.trim_start_matches("0x").trim_start_matches("0X");
            u64::from_str_radix(stripped, 16).ok()
        }
        crate::abstract_syntax_tree::AstJumpTarget::Variable {
            var_map, var_id, ..
        } => variable_constant_address(var_map, *var_id),
    }
}

fn function_like_name_from_address(ast: &Ast, addr: u64) -> String {
    if let Some(fid) = resolve_function_id_by_address(ast, addr) {
        fid.get_default_name()
    } else {
        AstFunctionId { address: addr }.get_default_name()
    }
}

fn resolve_function_id_by_address(ast: &Ast, addr: u64) -> Option<AstFunctionId> {
    let exact = AstFunctionId { address: addr };
    if ast.functions.read().unwrap().contains_key(&exact) {
        Some(exact)
    } else {
        None
    }
}

fn rebuild_call_groups_recursive(
    ast: &Ast,
    scope: AstFunctionId,
    variables: &ArcAstVariableMap,
    stmts: &mut Vec<WrappedAstStatement>,
    reg_name_to_var: &RegNameToVarMap,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    data_to_var: &HashMap<IrData, AstVariableId>,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
    rsp_offset_to_var: &HashMap<isize, AstVariableId>,
) {
    // First, recurse into nested statement lists so calls inside blocks/ifs also get rebuilt.
    for ws in stmts.iter_mut() {
        match &mut ws.statement {
            AstStatement::If(_, t, f) => {
                rebuild_call_groups_recursive(
                    ast,
                    scope,
                    variables,
                    t,
                    reg_name_to_var,
                    var_id_to_reg,
                    data_to_var,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
                if let Some(f) = f {
                    rebuild_call_groups_recursive(
                        ast,
                        scope,
                        variables,
                        f,
                        reg_name_to_var,
                        var_id_to_reg,
                        data_to_var,
                        var_id_to_rsp_offset,
                        rsp_offset_to_var,
                    );
                }
            }
            AstStatement::While(_, b) | AstStatement::Block(b) => {
                rebuild_call_groups_recursive(
                    ast,
                    scope,
                    variables,
                    b,
                    reg_name_to_var,
                    var_id_to_reg,
                    data_to_var,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
            }
            AstStatement::For(_init, _cond, _update, b) => {
                rebuild_call_groups_recursive(
                    ast,
                    scope,
                    variables,
                    b,
                    reg_name_to_var,
                    var_id_to_reg,
                    data_to_var,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
            }
            _ => {}
        }
    }

    // Then rebuild call-groups at this level.
    let body = std::mem::take(stmts);
    let call_sites = find_call_sites(&body);
    if call_sites.is_empty() {
        *stmts = body;
        enrich_existing_call_args_recursive(
            ast,
            scope,
            variables,
            stmts,
            reg_name_to_var,
            var_id_to_reg,
            var_id_to_rsp_offset,
            rsp_offset_to_var,
        );
        return;
    }

    let mut rebuilt_groups: HashMap<usize, (usize, Vec<WrappedAstStatement>)> = HashMap::new();
    for site in call_sites.into_iter() {
        let existing_call_stmt = resolve_call_target(
            ast,
            scope,
            variables,
            &body[site.call_stmt_idx],
            data_to_var,
        )
        .or_else(|| {
            // If target resolution fails, still reuse an existing AST call node so
            // we can infer arguments and collapse call-mechanics statements.
            match &body[site.call_stmt_idx].statement {
                AstStatement::Call(_) => Some(body[site.call_stmt_idx].clone()),
                _ => None,
            }
        });
        let Some(existing_call_stmt) = existing_call_stmt else {
            continue;
        };

        let args = infer_call_args(
            ast,
            variables,
            &body,
            site.group_start,
            &existing_call_stmt,
            reg_name_to_var,
            var_id_to_reg,
            var_id_to_rsp_offset,
            rsp_offset_to_var,
        );

        let call_stmt = set_call_args(existing_call_stmt, args);

        // Rebuild this instruction-group as a single call statement.
        rebuilt_groups.insert(site.group_start, (site.group_end, vec![call_stmt]));
    }

    if rebuilt_groups.is_empty() {
        *stmts = body;
        return;
    }

    let mut new_body: Vec<WrappedAstStatement> = Vec::with_capacity(body.len());
    let mut i = 0;
    while i < body.len() {
        if let Some((end, stmts_group)) = rebuilt_groups.get(&i) {
            new_body.extend(stmts_group.iter().cloned());
            i = *end;
        } else {
            new_body.push(body[i].clone());
            i += 1;
        }
    }

    *stmts = new_body;

    enrich_existing_call_args_recursive(
        ast,
        scope,
        variables,
        stmts,
        reg_name_to_var,
        var_id_to_reg,
        var_id_to_rsp_offset,
        rsp_offset_to_var,
    );
}

fn enrich_existing_call_args_recursive(
    ast: &Ast,
    scope: AstFunctionId,
    variables: &ArcAstVariableMap,
    stmts: &mut Vec<WrappedAstStatement>,
    reg_name_to_var: &RegNameToVarMap,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
    rsp_offset_to_var: &HashMap<isize, AstVariableId>,
) {
    for i in 0..stmts.len() {
        // Recurse first
        match &mut stmts[i].statement {
            AstStatement::If(_, t, f) => {
                enrich_existing_call_args_recursive(
                    ast,
                    scope,
                    variables,
                    t,
                    reg_name_to_var,
                    var_id_to_reg,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
                if let Some(f) = f {
                    enrich_existing_call_args_recursive(
                        ast,
                        scope,
                        variables,
                        f,
                        reg_name_to_var,
                        var_id_to_reg,
                        var_id_to_rsp_offset,
                        rsp_offset_to_var,
                    );
                }
            }
            AstStatement::While(_, b) | AstStatement::Block(b) => {
                enrich_existing_call_args_recursive(
                    ast,
                    scope,
                    variables,
                    b,
                    reg_name_to_var,
                    var_id_to_reg,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
            }
            AstStatement::For(_, _, _, b) => {
                enrich_existing_call_args_recursive(
                    ast,
                    scope,
                    variables,
                    b,
                    reg_name_to_var,
                    var_id_to_reg,
                    var_id_to_rsp_offset,
                    rsp_offset_to_var,
                );
            }
            _ => {}
        }

        rewrite_call_target_from_constant_value(ast, &mut stmts[i]);
        rewrite_indirect_call_target_from_nearby_assignment(ast, stmts, i);

        let should_infer = match &stmts[i].statement {
            AstStatement::Call(AstCall::Function { args, .. })
            | AstStatement::Call(AstCall::Variable { args, .. })
            | AstStatement::Call(AstCall::Unknown(_, args)) => args.is_empty(),
            _ => false,
        };
        if !should_infer {
            continue;
        }

        let inferred = {
            let call_stmt = &stmts[i];
            infer_call_args(
                ast,
                variables,
                stmts.as_slice(),
                i,
                call_stmt,
                reg_name_to_var,
                var_id_to_reg,
                var_id_to_rsp_offset,
                rsp_offset_to_var,
            )
        };
        if inferred.is_empty() {
            continue;
        }

        match &mut stmts[i].statement {
            AstStatement::Call(AstCall::Function { args, .. })
            | AstStatement::Call(AstCall::Variable { args, .. })
            | AstStatement::Call(AstCall::Unknown(_, args)) => {
                *args = inferred;
            }
            _ => {}
        }
    }
}

fn rewrite_indirect_call_target_from_nearby_assignment(
    ast: &Ast,
    stmts: &mut [WrappedAstStatement],
    call_idx: usize,
) {
    let (target_var, current_args) = match &stmts[call_idx].statement {
        AstStatement::Call(AstCall::Variable { var_id, args, .. }) => (*var_id, args.clone()),
        _ => return,
    };

    let mut j = call_idx;
    while j > 0 {
        j -= 1;
        if is_barrier(&stmts[j].statement) {
            break;
        }

        let AstStatement::Assignment(lhs, _rhs) = &stmts[j].statement else {
            continue;
        };
        let AstExpression::Variable(_, lhs_id) = lhs.item else {
            continue;
        };
        if lhs_id != target_var {
            continue;
        }

        let Some(slot) = stmt_origin_rip_relative_load_slot(&stmts[j]) else {
            break;
        };
        let external_name = external_symbol_name_from_slot(ast, slot)
            .map(|name| external_symbol_identifier(&name))
            .unwrap_or_else(|| format!("*0x{:X}", slot));
        stmts[call_idx].statement =
            AstStatement::Call(AstCall::Unknown(external_name, current_args));
        break;
    }
}

fn rewrite_call_target_from_constant_value(ast: &Ast, stmt: &mut WrappedAstStatement) {
    let (var_map, var_id, current_args) = match &stmt.statement {
        AstStatement::Call(AstCall::Variable {
            var_map,
            var_id,
            args,
            ..
        }) => (var_map.clone(), *var_id, args.clone()),
        _ => return,
    };

    let Some(addr) = variable_constant_address(&var_map, var_id) else {
        return;
    };
    if let Some(target) = resolve_function_id_by_address(ast, addr) {
        stmt.statement = AstStatement::Call(AstCall::Function {
            target,
            args: current_args,
        });
        return;
    }
    if let Some(symbol) = external_symbol_name_from_slot(ast, addr) {
        stmt.statement = AstStatement::Call(AstCall::Unknown(
            external_symbol_identifier(&symbol),
            current_args,
        ));
    }
}

fn variable_constant_address(var_map: &ArcAstVariableMap, var_id: AstVariableId) -> Option<u64> {
    let vars = var_map.read().unwrap();
    let var = vars.get(&var_id)?;
    let const_value = var.const_value.as_ref()?;
    ast_value_constant_address(&const_value.item)
}

fn ast_value_constant_address(value: &crate::abstract_syntax_tree::AstValue) -> Option<u64> {
    match value {
        crate::abstract_syntax_tree::AstValue::Pointer(p) => ast_value_constant_address(&p.item),
        crate::abstract_syntax_tree::AstValue::Num(n) => {
            let (sign, digits) = n.to_u64_digits();
            if sign == num_bigint::Sign::Minus {
                None
            } else {
                Some(*digits.first().unwrap_or(&0))
            }
        }
        _ => None,
    }
}

fn stmt_origin_rip_relative_load_slot(stmt: &WrappedAstStatement) -> Option<u64> {
    let ir_stmt = stmt_origin_ir_statement(stmt)?;
    let IrStatement::Assignment { from, .. } = ir_stmt else {
        return None;
    };
    let instruction_args = stmt_origin_instruction_args(stmt)?;
    let resolved_from = resolve_operand(from, instruction_args);

    let inst = stmt_origin_instruction(stmt)?;
    let inst_len = inst.bytes.as_ref().map(|x| x.len() as u64).unwrap_or(0);
    let AstStatementOrigin::Ir(desc) = &stmt.origin else {
        return None;
    };
    let ir_index = desc.descriptor().ir_index() as usize;
    let rip_next_ip = desc.ir().get_ir()[ir_index].address.get_virtual_address() + inst_len;

    rip_relative_deref_address(resolved_from.as_ref(), rip_next_ip)
}

fn rip_relative_deref_address(data: &IrData, rip_next_ip: u64) -> Option<u64> {
    fn signed_const(data: &IrData) -> Option<i128> {
        match data {
            IrData::Constant(v) => Some(*v as i128),
            IrData::Operation(IrDataOperation::Unary {
                operator: IrUnaryOperator::Negation,
                arg,
            }) => arg.constant().map(|v| -(v as i128)),
            _ => None,
        }
    }
    fn unwrap_cast(data: &IrData) -> &IrData {
        match data {
            IrData::Operation(IrDataOperation::Unary {
                operator: IrUnaryOperator::ZeroExtend | IrUnaryOperator::SignExtend,
                arg,
            }) => arg.as_ref(),
            _ => data,
        }
    }

    let data = unwrap_cast(data);
    let IrData::Dereference(inner) = data else {
        return None;
    };
    let inner = unwrap_cast(inner.as_ref());

    let IrData::Operation(IrDataOperation::Binary {
        operator,
        arg1,
        arg2,
    }) = inner
    else {
        return None;
    };

    let is_rip = |d: &IrData| matches!(d, IrData::Register(r) if r.is_ip());

    match operator {
        IrBinaryOperator::Add => {
            if is_rip(arg1.as_ref()) {
                let disp = signed_const(arg2.as_ref())?;
                return u64::try_from((rip_next_ip as i128) + disp).ok();
            }
            if is_rip(arg2.as_ref()) {
                let disp = signed_const(arg1.as_ref())?;
                return u64::try_from((rip_next_ip as i128) + disp).ok();
            }
            None
        }
        IrBinaryOperator::Sub => {
            if is_rip(arg1.as_ref()) {
                let disp = signed_const(arg2.as_ref())?;
                return u64::try_from((rip_next_ip as i128) - disp).ok();
            }
            None
        }
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
struct CallSite {
    group_start: usize,
    group_end: usize,
    call_stmt_idx: usize,
}

fn find_call_sites(body: &[WrappedAstStatement]) -> Vec<CallSite> {
    let mut sites = Vec::new();
    let mut idx = 0;
    while idx < body.len() {
        let Some(ir_index) = stmt_ir_index(&body[idx]) else {
            idx += 1;
            continue;
        };

        let start = find_ir_group_start_index(body, idx, ir_index);
        let end = find_ir_group_end_index(body, idx, ir_index);

        let mut call_stmt_idx: Option<usize> = None;
        for i in start..end {
            if is_origin_jump_by_call(&body[i]) {
                call_stmt_idx = Some(i);
                break;
            }
        }
        if let Some(call_stmt_idx) = call_stmt_idx {
            sites.push(CallSite {
                group_start: start,
                group_end: end,
                call_stmt_idx,
            });
        }

        idx = end;
    }
    sites
}

fn stmt_ir_index(stmt: &WrappedAstStatement) -> Option<u32> {
    match &stmt.origin {
        AstStatementOrigin::Ir(desc) => Some(desc.descriptor().ir_index()),
        _ => None,
    }
}

fn find_ir_group_start_index(body: &[WrappedAstStatement], mut idx: usize, ir_index: u32) -> usize {
    while idx > 0 {
        let Some(prev_ir_index) = stmt_ir_index(&body[idx - 1]) else {
            break;
        };
        if prev_ir_index != ir_index {
            break;
        }
        idx -= 1;
    }
    idx
}

fn find_ir_group_end_index(body: &[WrappedAstStatement], mut idx: usize, ir_index: u32) -> usize {
    while idx < body.len() {
        let Some(next_ir_index) = stmt_ir_index(&body[idx]) else {
            break;
        };
        if next_ir_index != ir_index {
            break;
        }
        idx += 1;
    }
    idx
}

fn is_origin_jump_by_call(stmt: &WrappedAstStatement) -> bool {
    let Some(ir_stmt) = stmt_origin_ir_statement(stmt) else {
        return false;
    };
    matches!(ir_stmt, IrStatement::JumpByCall { .. })
}

fn stmt_origin_ir_statement(stmt: &WrappedAstStatement) -> Option<&IrStatement> {
    let AstStatementOrigin::Ir(desc) = &stmt.origin else {
        return None;
    };
    let ir_index = desc.descriptor().ir_index() as usize;
    let stmt_index = (*desc.descriptor().statement_index()).as_ref().copied()? as usize;
    let ir_block = &desc.ir().get_ir()[ir_index];
    let stmts = ir_block.statements.as_ref()?;
    stmts.get(stmt_index)
}

fn stmt_origin_instruction_args(stmt: &WrappedAstStatement) -> Option<&[iceball::Argument]> {
    let AstStatementOrigin::Ir(desc) = &stmt.origin else {
        return None;
    };
    let ir_index = desc.descriptor().ir_index() as usize;
    Some(&desc.ir().get_instructions()[ir_index].inner.arguments)
}

fn stmt_origin_instruction(stmt: &WrappedAstStatement) -> Option<&iceball::Instruction> {
    let AstStatementOrigin::Ir(desc) = &stmt.origin else {
        return None;
    };
    let ir_index = desc.descriptor().ir_index() as usize;
    Some(&desc.ir().get_instructions()[ir_index].inner)
}

fn resolve_call_target(
    ast: &Ast,
    scope: AstFunctionId,
    variables: &ArcAstVariableMap,
    call_stmt: &WrappedAstStatement,
    data_to_var: &HashMap<IrData, AstVariableId>,
) -> Option<WrappedAstStatement> {
    // JumpByCall target in IR (resolved operands) -> AstCall
    let ir_stmt = stmt_origin_ir_statement(call_stmt)?;
    let IrStatement::JumpByCall { target } = ir_stmt else {
        return None;
    };
    let instruction_args = stmt_origin_instruction_args(call_stmt)?;
    let resolved_target = resolve_operand(target, instruction_args);

    let call = match resolved_target.as_ref() {
        IrData::Constant(c) => {
            let addr = *c as u64;
            if let Some(fid) = resolve_function_id_by_address(ast, addr) {
                AstCall::Function {
                    target: fid,
                    args: Vec::new(),
                }
            } else if let Some(symbol) = external_symbol_name_from_slot(ast, addr) {
                AstCall::Unknown(external_symbol_identifier(&symbol), Vec::new())
            } else {
                AstCall::Unknown(function_like_name_from_address(ast, addr), Vec::new())
            }
        }
        _ => {
            // Try mapping IR data location to an AST variable.
            let var_id = data_to_var.get(resolved_target.as_ref()).copied();
            if let Some(var_id) = var_id {
                AstCall::Variable {
                    scope,
                    var_map: variables.clone(),
                    var_id,
                    args: Vec::new(),
                }
            } else {
                AstCall::Unknown(resolved_target.to_string(), Vec::new())
            }
        }
    };

    let mut out = call_stmt.clone();
    out.statement = AstStatement::Call(call);

    Some(out)
}

fn set_call_args(
    mut call_stmt: WrappedAstStatement,
    args: Vec<Wrapped<AstExpression>>,
) -> WrappedAstStatement {
    if let AstStatement::Call(call) = &mut call_stmt.statement {
        match call {
            AstCall::Function { args: a, .. }
            | AstCall::Variable { args: a, .. }
            | AstCall::Unknown(_, a) => {
                *a = args;
            }
            AstCall::Builtin(_, _) => {}
        }
    }
    call_stmt
}

fn infer_call_args(
    ast: &Ast,
    variables: &ArcAstVariableMap,
    body: &[WrappedAstStatement],
    call_group_start_idx: usize,
    call_stmt: &WrappedAstStatement,
    reg_name_to_var: &RegNameToVarMap,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
    rsp_offset_to_var: &HashMap<isize, AstVariableId>,
) -> Vec<Wrapped<AstExpression>> {
    let callee_params = match &call_stmt.statement {
        AstStatement::Call(AstCall::Function { target, .. }) => {
            get_function_parameters(ast, *target)
        }
        _ => None,
    };

    let fallback_args = infer_args_by_calling_convention(
        body,
        call_group_start_idx,
        var_id_to_reg,
        var_id_to_rsp_offset,
    );

    if let Some(params) = callee_params
        && !params.is_empty()
    {
        let args = infer_args_from_callee_params(
            variables,
            body,
            call_group_start_idx,
            &params,
            reg_name_to_var,
            var_id_to_reg,
            var_id_to_rsp_offset,
            rsp_offset_to_var,
        );

        let known_count = args.iter().filter(|x| !is_unknown_expr(x)).count();
        if known_count == 0 && !fallback_args.is_empty() {
            return fallback_args;
        }
        if known_count * 2 < args.len() && !fallback_args.is_empty() {
            return fallback_args;
        }

        return args;
    }

    fallback_args
}

fn infer_args_from_callee_params(
    variables: &ArcAstVariableMap,
    body: &[WrappedAstStatement],
    call_group_start_idx: usize,
    params: &[crate::abstract_syntax_tree::AstParameter],
    reg_name_to_var: &RegNameToVarMap,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
    rsp_offset_to_var: &HashMap<isize, AstVariableId>,
) -> Vec<Wrapped<AstExpression>> {
    let mut interested: RegNameSet = HashSet::new();
    for p in params.iter() {
        let AstParameterLocation::Register(reg_loc) = &p.location else {
            continue;
        };
        if let Some(r) = ir_register(reg_loc.as_ref()) {
            interested.insert(RegKey::from_register(&r));
        }
    }
    let reg_writes =
        scan_backward_for_register_writes(body, call_group_start_idx, var_id_to_reg, &interested);

    // Stack args: map callee RBP+off to caller RSP+off (Win64: rbp = rsp after `push rbp`).
    let mut interested_stack_offsets: HashSet<isize> = HashSet::new();
    for p in params.iter() {
        let AstParameterLocation::Stack(off_rbp) = &p.location else {
            continue;
        };
        let off_rsp = *off_rbp - 0x10;
        if off_rsp >= 0 {
            interested_stack_offsets.insert(off_rsp);
        }
    }
    let stack_writes = if interested_stack_offsets.is_empty() {
        HashMap::new()
    } else {
        scan_backward_for_stack_slot_writes(
            body,
            call_group_start_idx,
            var_id_to_rsp_offset,
            var_id_to_reg,
            &interested_stack_offsets,
        )
    };

    let mut out: Vec<Wrapped<AstExpression>> = Vec::new();
    for p in params.iter() {
        match &p.location {
            AstParameterLocation::Register(reg_loc) => {
                let Some(r) = ir_register(reg_loc.as_ref()) else {
                    out.push(wrap_unknown_expr());
                    continue;
                };
                let reg_key = RegKey::from_register(&r);
                let arg = reg_writes
                    .get(&reg_key)
                    .cloned()
                    .or_else(|| {
                        reg_name_to_var
                            .get(&reg_key)
                            .copied()
                            .map(|id| wrap_var_expr(variables.clone(), id))
                    })
                    .unwrap_or_else(wrap_unknown_expr);
                out.push(arg);
            }
            AstParameterLocation::Stack(_) => {
                let AstParameterLocation::Stack(off_rbp) = &p.location else {
                    out.push(wrap_unknown_expr());
                    continue;
                };
                let off_rsp = *off_rbp - 0x10;
                if let Some(v) = stack_writes.get(&off_rsp) {
                    out.push(v.clone());
                } else if let Some(var_id) = rsp_offset_to_var.get(&off_rsp).copied() {
                    out.push(wrap_var_expr(variables.clone(), var_id));
                } else {
                    out.push(wrap_unknown_expr());
                }
            }
        }
    }

    out
}

fn infer_args_by_calling_convention(
    body: &[WrappedAstStatement],
    call_group_start_idx: usize,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
) -> Vec<Wrapped<AstExpression>> {
    // Deterministic fallback: collect all register families and stack slots written
    // before call; register key includes architecture + family bit-start.
    let interested: RegNameSet = HashSet::new();
    let reg_writes =
        scan_backward_for_register_writes(body, call_group_start_idx, var_id_to_reg, &interested);

    let mut out: Vec<Wrapped<AstExpression>> = Vec::new();
    let mut reg_keys: Vec<_> = reg_writes.keys().copied().collect();
    reg_keys.sort_unstable_by_key(|key| key.sort_key());
    for reg_key in reg_keys {
        if let Some(v) = reg_writes.get(&reg_key).cloned() {
            out.push(v);
        }
    }

    let interested_stack_offsets: HashSet<isize> = HashSet::new();
    let stack_writes = scan_backward_for_stack_slot_writes(
        body,
        call_group_start_idx,
        var_id_to_rsp_offset,
        var_id_to_reg,
        &interested_stack_offsets,
    );
    let mut stack_offsets: Vec<_> = stack_writes
        .keys()
        .copied()
        .filter(|offset| *offset >= 0)
        .collect();
    stack_offsets.sort_unstable();
    for offset in stack_offsets {
        if let Some(v) = stack_writes.get(&offset).cloned() {
            out.push(v);
        }
    }

    out
}

fn scan_backward_for_register_writes(
    body: &[WrappedAstStatement],
    call_group_start_idx: usize,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    interested: &RegNameSet,
) -> RegNameExprMap {
    let mut out: RegNameExprMap = HashMap::new();

    let mut i = call_group_start_idx;
    while i > 0 {
        i -= 1;
        let wrapped_stmt = &body[i];
        let stmt = &wrapped_stmt.statement;

        if is_barrier(stmt) {
            break;
        }

        if let Some(reg) = stmt_origin_assigned_register(wrapped_stmt) {
            let reg_key = RegKey::from_register(&reg);
            if (interested.is_empty() || interested.contains(&reg_key))
                && !out.contains_key(&reg_key)
            {
                if let Some(rhs) = stmt_assigned_expression(stmt) {
                    out.insert(reg_key, rhs);
                }
            }
        }

        match stmt {
            AstStatement::Declaration(var, init) => {
                if let Some(reg) = var_id_to_reg.get(&var.id) {
                    let reg_key = RegKey::from_register(reg);
                    if (interested.is_empty() || interested.contains(&reg_key))
                        && !out.contains_key(&reg_key)
                    {
                        if let Some(init) = init {
                            out.insert(reg_key, init.clone());
                        }
                    }
                }
            }
            AstStatement::Assignment(lhs, rhs) => {
                if let AstExpression::Variable(_, lhs_id) = lhs.item {
                    if let Some(reg) = var_id_to_reg.get(&lhs_id) {
                        let reg_key = RegKey::from_register(reg);
                        if (interested.is_empty() || interested.contains(&reg_key))
                            && !out.contains_key(&reg_key)
                        {
                            out.insert(reg_key, rhs.clone());
                        }
                    }
                }
            }
            _ => {}
        }

        if !interested.is_empty() && out.len() >= interested.len() {
            break;
        }
    }

    out
}

fn scan_backward_for_stack_slot_writes(
    body: &[WrappedAstStatement],
    call_group_start_idx: usize,
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
    interested_offsets: &HashSet<isize>,
) -> HashMap<isize, Wrapped<AstExpression>> {
    let mut out: HashMap<isize, Wrapped<AstExpression>> = HashMap::new();

    let mut i = call_group_start_idx;
    while i > 0 {
        i -= 1;
        let wrapped_stmt = &body[i];
        let stmt = &wrapped_stmt.statement;

        if is_barrier(stmt) {
            break;
        }
        if is_stack_pointer_write(wrapped_stmt, var_id_to_reg) {
            break;
        }

        if let Some(off) = stmt_origin_assigned_rsp_offset(wrapped_stmt) {
            if (interested_offsets.is_empty() || interested_offsets.contains(&off))
                && !out.contains_key(&off)
            {
                if let Some(rhs) = stmt_assigned_expression(stmt) {
                    out.insert(off, rhs);
                }
            }
        }

        match stmt {
            AstStatement::Declaration(var, init) => {
                if let Some(off) = var_id_to_rsp_offset.get(&var.id).copied() {
                    if (interested_offsets.is_empty() || interested_offsets.contains(&off))
                        && !out.contains_key(&off)
                    {
                        if let Some(init) = init {
                            out.insert(off, init.clone());
                        }
                    }
                }
            }
            AstStatement::Assignment(lhs, rhs) => {
                if let AstExpression::Variable(_, lhs_id) = lhs.item {
                    if let Some(off) = var_id_to_rsp_offset.get(&lhs_id).copied() {
                        if (interested_offsets.is_empty() || interested_offsets.contains(&off))
                            && !out.contains_key(&off)
                        {
                            out.insert(off, rhs.clone());
                        }
                    }
                }
            }
            _ => {}
        }

        if !interested_offsets.is_empty() && out.len() >= interested_offsets.len() {
            break;
        }
    }

    out
}

fn is_stack_pointer_write(
    stmt: &WrappedAstStatement,
    var_id_to_reg: &HashMap<AstVariableId, Register>,
) -> bool {
    if let Some(reg) = stmt_origin_assigned_register(stmt)
        && reg.is_sp()
    {
        return true;
    }

    match &stmt.statement {
        AstStatement::Declaration(var, _) => {
            var_id_to_reg.get(&var.id).is_some_and(Register::is_sp)
        }
        AstStatement::Assignment(lhs, _) => {
            if let AstExpression::Variable(_, id) = lhs.item {
                var_id_to_reg.get(&id).is_some_and(Register::is_sp)
            } else {
                false
            }
        }
        _ => false,
    }
}

fn stmt_assigned_expression(stmt: &AstStatement) -> Option<Wrapped<AstExpression>> {
    match stmt {
        AstStatement::Declaration(_, init) => init.clone(),
        AstStatement::Assignment(_, rhs) => Some(rhs.clone()),
        _ => None,
    }
}

fn stmt_origin_assigned_register(stmt: &WrappedAstStatement) -> Option<Register> {
    let ir_stmt = stmt_origin_ir_statement(stmt)?;
    let IrStatement::Assignment { to, .. } = ir_stmt else {
        return None;
    };
    let instruction_args = stmt_origin_instruction_args(stmt)?;
    let resolved_to = resolve_operand(to, instruction_args);
    match resolved_to.as_ref() {
        IrData::Register(reg) => Some(reg.clone()),
        _ => None,
    }
}

fn stmt_origin_assigned_rsp_offset(stmt: &WrappedAstStatement) -> Option<isize> {
    let ir_stmt = stmt_origin_ir_statement(stmt)?;
    let IrStatement::Assignment { to, .. } = ir_stmt else {
        return None;
    };
    let instruction_args = stmt_origin_instruction_args(stmt)?;
    let resolved_to = resolve_operand(to, instruction_args);
    rsp_offset_from_location(resolved_to.as_ref())
}

fn is_barrier(stmt: &AstStatement) -> bool {
    match stmt {
        AstStatement::Call(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Ir(_)
        | AstStatement::Return(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_) => true,

        AstStatement::If(_, _, _)
        | AstStatement::While(_, _)
        | AstStatement::DoWhile(_, _)
        | AstStatement::For(_, _, _, _)
        | AstStatement::Switch(_, _, _)
        | AstStatement::Block(_) => true,

        AstStatement::Declaration(_, _)
        | AstStatement::Assignment(_, _)
        | AstStatement::Label(_)
        | AstStatement::Comment(_)
        | AstStatement::Break
        | AstStatement::Continue
        | AstStatement::Empty => false,
    }
}

fn get_function_parameters(
    ast: &Ast,
    function_id: AstFunctionId,
) -> Option<Vec<crate::abstract_syntax_tree::AstParameter>> {
    let functions = ast.functions.read().unwrap();
    let version = *ast.function_versions.get(&function_id)?;
    let func = functions.get(&function_id)?.get(&version)?;
    Some(func.parameters.clone())
}

fn build_register_name_to_var_map(variables: &ArcAstVariableMap) -> RegNameToVarMap {
    let mut map: RegNameToVarMap = HashMap::new();
    let vars = variables.read().unwrap();
    let mut vars_sorted: Vec<_> = vars.iter().collect();
    vars_sorted.sort_unstable_by_key(|(var_id, _)| var_id.index);
    for (var_id, var) in vars_sorted {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        let mut access_map_sorted: Vec<_> = access_map.iter().collect();
        access_map_sorted.sort_unstable_by_key(|(ir_index, _)| {
            (ir_index.ir_index(), *ir_index.statement_index())
        });
        for (_ir_index, accesses) in access_map_sorted {
            for da in accesses.iter() {
                let IrData::Register(reg) = da.location().as_ref() else {
                    continue;
                };
                map.entry(RegKey::from_register(reg)).or_insert(*var_id);
            }
        }
    }
    map
}

fn build_var_id_to_register_name_map(
    variables: &ArcAstVariableMap,
) -> HashMap<AstVariableId, Register> {
    let vars = variables.read().unwrap();
    let mut map: HashMap<AstVariableId, Register> = HashMap::new();
    let mut vars_sorted: Vec<_> = vars.iter().collect();
    vars_sorted.sort_unstable_by_key(|(var_id, _)| var_id.index);
    for (var_id, var) in vars_sorted {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        let mut access_map_sorted: Vec<_> = access_map.iter().collect();
        access_map_sorted.sort_unstable_by_key(|(ir_index, _)| {
            (ir_index.ir_index(), *ir_index.statement_index())
        });
        let mut any_reg: Option<Register> = None;
        let mut written_reg: Option<Register> = None;
        for (_ir_index, accesses) in access_map_sorted {
            for da in accesses.iter() {
                let IrData::Register(reg) = da.location().as_ref() else {
                    continue;
                };
                if any_reg.is_none() {
                    any_reg = Some(reg.clone());
                }
                if *da.access_type() == IrDataAccessType::Write {
                    written_reg = Some(reg.clone());
                    break;
                }
            }
            if written_reg.is_some() {
                break;
            }
        }
        if let Some(reg) = written_reg.or(any_reg) {
            map.insert(*var_id, reg);
        }
    }
    map
}

fn build_data_location_to_var_map(variables: &ArcAstVariableMap) -> HashMap<IrData, AstVariableId> {
    let vars = variables.read().unwrap();
    let mut map: HashMap<IrData, AstVariableId> = HashMap::new();
    let mut vars_sorted: Vec<_> = vars.iter().collect();
    vars_sorted.sort_unstable_by_key(|(var_id, _)| var_id.index);
    for (var_id, var) in vars_sorted {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        let mut access_map_sorted: Vec<_> = access_map.iter().collect();
        access_map_sorted.sort_unstable_by_key(|(ir_index, _)| {
            (ir_index.ir_index(), *ir_index.statement_index())
        });
        for (_ir_index, accesses) in access_map_sorted {
            for da in accesses.iter() {
                map.entry(da.location().as_ref().clone()).or_insert(*var_id);
            }
        }
    }
    map
}

fn build_var_id_to_rsp_offset_map(variables: &ArcAstVariableMap) -> HashMap<AstVariableId, isize> {
    let vars = variables.read().unwrap();
    let mut map: HashMap<AstVariableId, isize> = HashMap::new();
    let mut vars_sorted: Vec<_> = vars.iter().collect();
    vars_sorted.sort_unstable_by_key(|(var_id, _)| var_id.index);
    for (var_id, var) in vars_sorted {
        let Some(access_map) = var.data_access_ir.as_ref() else {
            continue;
        };
        let mut access_map_sorted: Vec<_> = access_map.iter().collect();
        access_map_sorted.sort_unstable_by_key(|(ir_index, _)| {
            (ir_index.ir_index(), *ir_index.statement_index())
        });
        for (_ir_index, accesses) in access_map_sorted {
            let mut found = false;
            for da in accesses.iter() {
                let Some(off) = rsp_offset_from_location(da.location().as_ref()) else {
                    continue;
                };
                map.entry(*var_id).or_insert(off);
                found = true;
                break;
            }
            if found {
                break;
            }
        }
    }
    map
}

fn build_rsp_offset_to_var_map(
    var_id_to_rsp_offset: &HashMap<AstVariableId, isize>,
) -> HashMap<isize, AstVariableId> {
    let mut map: HashMap<isize, AstVariableId> = HashMap::new();
    let mut entries: Vec<_> = var_id_to_rsp_offset
        .iter()
        .map(|(var_id, off)| (*off, *var_id))
        .collect();
    entries.sort_unstable_by_key(|(off, var_id)| (*off, var_id.index));
    for (off, var_id) in entries {
        map.entry(off).or_insert(var_id);
    }
    map
}

fn rsp_offset_from_location(loc: &IrData) -> Option<isize> {
    let IrData::Dereference(inner) = loc else {
        return None;
    };
    rsp_offset_from_expr(inner.as_ref())
}

fn rsp_offset_from_expr(expr: &IrData) -> Option<isize> {
    fn is_sp_expr(data: &IrData) -> bool {
        matches!(data, IrData::Register(r) if r.is_sp())
    }
    fn signed_const(data: &IrData) -> Option<isize> {
        match data {
            IrData::Constant(v) => Some(*v as isize),
            IrData::Operation(IrDataOperation::Unary {
                operator: IrUnaryOperator::Negation,
                arg,
            }) => arg.constant().map(|v| 0 - v as isize),
            _ => None,
        }
    }

    // [rsp]
    if is_sp_expr(expr) {
        return Some(0);
    }

    // [rsp +/- const]
    match expr {
        IrData::Operation(IrDataOperation::Binary {
            operator: IrBinaryOperator::Add,
            arg1,
            arg2,
        }) if is_sp_expr(arg1.as_ref()) => signed_const(arg2.as_ref()),
        IrData::Operation(IrDataOperation::Binary {
            operator: IrBinaryOperator::Sub,
            arg1,
            arg2,
        }) if is_sp_expr(arg1.as_ref()) => signed_const(arg2.as_ref()).map(|v| 0 - v),
        _ => None,
    }
}

// NOTE: previously we tried to selectively keep some statements in the same IR-instruction group.
// In practice, the call-related groups are almost entirely call mechanics, so we collapse to 1 stmt.

fn ir_register(data: &IrData) -> Option<Register> {
    match data {
        IrData::Register(r) => Some(r.clone()),
        _ => None,
    }
}

fn wrap_var_expr(var_map: ArcAstVariableMap, var_id: AstVariableId) -> Wrapped<AstExpression> {
    Wrapped {
        item: AstExpression::Variable(var_map, var_id),
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

fn wrap_unknown_expr() -> Wrapped<AstExpression> {
    Wrapped {
        item: AstExpression::Unknown,
        origin: AstValueOrigin::Unknown,
        comment: None,
    }
}

fn is_unknown_expr(expr: &Wrapped<AstExpression>) -> bool {
    matches!(expr.item, AstExpression::Unknown)
}

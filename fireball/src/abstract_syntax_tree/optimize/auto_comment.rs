//! Auto-generate descriptive comments for variables and function calls.

use crate::{
    abstract_syntax_tree::{
        ArcAstVariableMap, Ast, AstBinaryOperator, AstBuiltinFunctionArgument, AstCall,
        AstExpression, AstFunctionId, AstFunctionVersion, AstLiteral, AstStatement,
        AstStatementOrigin, AstUnaryOperator, AstValueType, AstVariableAccessType, AstVariableId,
        GetRelatedVariables, ProcessedOptimization, Wrapped, WrappedAstStatement,
    },
    ir::{Register, analyze::AggregateCandidate, data::IrData},
    prelude::DecompileError,
};
use hashbrown::HashSet;

pub(super) fn synthesize_comments(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let mut body;
    let first_param_var_id;
    let first_param_register;
    let sret_layout_hint;
    let hidden_byref_hints: Vec<String>;
    let param_count;
    let all_param_var_ids: Vec<AstVariableId>;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        body = std::mem::take(&mut function.body);
        param_count = function.parameters.len();
        first_param_var_id = function
            .parameters
            .first()
            .and_then(|p| p.id.as_ref().left().copied());
        first_param_register = function.parameters.first().and_then(extract_param_register);
        sret_layout_hint = first_param_register
            .as_ref()
            .and_then(|reg| build_sret_layout_hint(function.ir.get_aggregates(), reg));
        hidden_byref_hints =
            build_hidden_byref_parameter_hints(&function.parameters, function.ir.get_aggregates());
        all_param_var_ids = function
            .parameters
            .iter()
            .filter_map(|p| p.id.as_ref().left().copied())
            .collect();
    }

    annotate_statement_list(&mut body, &all_param_var_ids);
    annotate_crypto_fingerprint(&mut body);
    annotate_decompression_fingerprint(&mut body);
    annotate_xor_decryption_loop(&mut body);
    annotate_integrity_check_loop(&mut body);
    annotate_loop_invariants(&mut body);
    annotate_tree_recursion_comments(&mut body, function_id);
    annotate_hash_table_patterns(&mut body);
    annotate_ring_buffer_patterns(&mut body);
    annotate_refcount_field_patterns(&mut body);
    annotate_pointer_tagging_patterns(&mut body);
    annotate_tagged_union_patterns(&mut body);
    if let Some(var_id) = first_param_var_id {
        if param_count >= 1 {
            annotate_this_or_sret_pointer(&mut body, var_id, sret_layout_hint.as_deref());
        }
    }
    annotate_hidden_byref_parameters(&mut body, &hidden_byref_hints);
    annotate_obfuscation_indicators(&mut body);
    annotate_ptr_len_pairs(&mut body, &all_param_var_ids);
    annotate_struct_field_ptr_len_pairs(&mut body);
    annotate_post_call_borrow_patterns(&mut body);
    annotate_heap_metadata_patterns(&mut body);
    annotate_format_string_types(&mut body);
    annotate_error_propagation(&mut body);
    annotate_behavioral_cluster(&mut body);
    annotate_bitfield_patterns(&mut body);
    annotate_domain_vocabulary(&mut body);
    annotate_config_string_xrefs(&mut body);
    annotate_resource_cleanup(&mut body);
    annotate_sanitizer_shadow(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::AutoComment);
    }

    Ok(())
}

fn annotate_statement_list(stmts: &mut Vec<WrappedAstStatement>, parameter_ids: &[AstVariableId]) {
    // First, recurse into nested statement lists.
    for stmt in stmts.iter_mut() {
        annotate_statement(stmt, parameter_ids);
    }

    // Collect indices where comments should be inserted (index, comment text).
    let mut insertions: Vec<(usize, String)> = Vec::new();

    for (i, stmt) in stmts.iter().enumerate() {
        match &stmt.statement {
            AstStatement::If(cond, branch_true, branch_false) => {
                // Stack canary check: if-block containing a call to stack_chk_fail.
                if statement_list_contains_stack_canary(branch_true) {
                    insertions.push((i, "// stack canary check".to_string()));
                }
                // Null pointer check: if (var == 0) or if (var != 0) where var is a pointer.
                if is_null_pointer_check(cond) {
                    insertions.push((i, "// null pointer check".to_string()));
                }
                // Assertion pattern: if (!cond) { abort/exit/assert_fail(); }
                if is_assertion_pattern(cond, branch_true, branch_false.as_deref()) {
                    insertions.push((i, "// assertion".to_string()));
                }
                // Guarded call: if (ptr) { call(ptr, ...); }
                if is_guarded_call_pattern(cond, branch_true, branch_false.as_deref()) {
                    insertions.push((i, "// guarded call".to_string()));
                }
                if detect_bounds_checked_access_comment(cond, branch_true, branch_false.as_deref())
                    .is_some()
                {
                    insertions.push((i, "// bounds-checked indexed access".to_string()));
                }
                if detect_floating_compare_comment(&cond.item).is_some() {
                    insertions.push((i, "// NaN-sensitive floating comparison".to_string()));
                }
            }
            AstStatement::Assignment(_, rhs) => {
                // Macro-like pattern: var = (a < b) ? a : b → MIN, etc.
                if let Some(macro_name) = detect_min_max_pattern(&rhs.item) {
                    insertions.push((i, format!("// {macro_name}")));
                }
                if detect_fp_exception_sensitive_comment(&rhs.item).is_some() {
                    insertions.push((
                        i,
                        "// floating-point arithmetic: denormal / FP-exception behavior may matter"
                            .to_string(),
                    ));
                }
            }
            AstStatement::Declaration(_, Some(init)) => {
                if detect_fp_exception_sensitive_comment(&init.item).is_some() {
                    insertions.push((
                        i,
                        "// floating-point arithmetic: denormal / FP-exception behavior may matter"
                            .to_string(),
                    ));
                }
            }
            AstStatement::Return(Some(expr)) => {
                if detect_fp_exception_sensitive_comment(&expr.item).is_some() {
                    insertions.push((
                        i,
                        "// floating-point arithmetic: denormal / FP-exception behavior may matter"
                            .to_string(),
                    ));
                }
            }
            AstStatement::Call(call) => {
                if call_name_matches_noreturn(call) {
                    insertions.push((i, "// does not return".to_string()));
                }
                if call_name_matches_seh(call) {
                    insertions.push((i, "// SEH setup".to_string()));
                }
                if call_name_matches_guard(call) {
                    insertions.push((i, "// static local initialization guard".to_string()));
                }
                if call_name_matches_anti_debug(call) {
                    insertions.push((i, "// anti-debug / anti-analysis check".to_string()));
                }
                if call_name_matches_instrumentation(call) {
                    insertions.push((i, "// sanitizer/coverage instrumentation".to_string()));
                }
                if call_name_matches_retpoline(call) {
                    insertions.push((i, "// retpoline indirect call thunk".to_string()));
                }
                if let Some(comment) = call_name_matches_lock(call) {
                    insertions.push((i, comment.to_string()));
                }
                if let Some(comment) = call_name_matches_refcount(call) {
                    insertions.push((i, comment.to_string()));
                }
                if call_name_matches_logging(call) {
                    insertions.push((i, "// logging / debug output".to_string()));
                }
                if let Some(comment) = call_name_matches_alloc(call) {
                    insertions.push((i, comment.to_string()));
                }
                if let Some(comment) = call_name_matches_container(call) {
                    insertions.push((i, comment.to_string()));
                }
                if call_name_matches_timing(call) {
                    insertions.push((i, "// timing / performance measurement".to_string()));
                }
                if call_name_matches_alloca(call) {
                    insertions.push((i, "// dynamic stack allocation (alloca/VLA)".to_string()));
                }
                if call_name_matches_objc(call) {
                    insertions.push((i, "// Objective-C runtime dispatch".to_string()));
                }
                if call_name_matches_safestack(call) {
                    insertions.push((i, "// safe/split stack instrumentation".to_string()));
                }
                if call_name_matches_exception(call) {
                    insertions.push((i, "// C++ exception handling runtime".to_string()));
                }
                if call_name_matches_tls(call) {
                    insertions.push((i, "// thread-local storage access".to_string()));
                }
                if call_name_matches_rust_panic(call) {
                    insertions.push((i, "// Rust panic / unwind runtime".to_string()));
                }
                if let Some(comment) = call_name_matches_errno(call) {
                    insertions.push((i, comment.to_string()));
                }
                if let Some(comment) = call_name_matches_resource_io(call) {
                    insertions.push((i, comment.to_string()));
                }
                if call_name_matches_string_op(call) {
                    insertions.push((i, "// string operation".to_string()));
                }
                if call_name_matches_math(call) {
                    insertions.push((i, "// math library call".to_string()));
                }
                if let Some(comment) = call_name_matches_process_thread(call) {
                    insertions.push((i, comment.to_string()));
                }
                if let Some(comment) = detect_callback_environment_comment(call, parameter_ids) {
                    insertions.push((i, comment));
                }
                if let Some(comment) = detect_callback_signature_comment(call, parameter_ids) {
                    insertions.push((i, comment));
                }
                if let Some(comment) = detect_callback_provenance_comment(call, parameter_ids) {
                    insertions.push((i, comment));
                }
                if let Some(comment) = call_name_matches_dynload(call) {
                    insertions.push((i, comment.to_string()));
                }
                if call_name_matches_setjmp(call) {
                    insertions.push((i, "// non-local jump (setjmp/longjmp)".to_string()));
                }
                if call_name_matches_atomic(call) {
                    insertions.push((i, "// atomic operation".to_string()));
                }
                if call_name_matches_vararg(call) {
                    insertions.push((i, "// vararg call".to_string()));
                }
            }
            AstStatement::Assembly(asm_text) => {
                let lower = asm_text.to_ascii_lowercase();
                if lower.contains("lfence") || lower.contains("mfence") || lower.contains("sfence")
                {
                    insertions.push((i, "// memory fence (speculation barrier)".to_string()));
                }
                if lower.contains("cpuid") {
                    insertions.push((i, "// serializing instruction (cpuid)".to_string()));
                }
                if lower.contains("int 3")
                    || lower.contains("int3")
                    || lower.contains("__debugbreak")
                {
                    insertions.push((i, "// breakpoint / debug trap".to_string()));
                }
                if lower.contains("ud2") {
                    insertions.push((i, "// undefined instruction (unreachable)".to_string()));
                }
            }
            AstStatement::Switch(_disc, cases, _default) => {
                // Enum inference: annotate switch with its constant set if it
                // has 3+ cases with small integer values.
                if cases.len() >= 3 {
                    let mut vals: Vec<i64> = cases
                        .iter()
                        .filter_map(|(lit, _)| match lit {
                            AstLiteral::Int(v) => Some(*v),
                            AstLiteral::UInt(v) if *v <= i64::MAX as u64 => Some(*v as i64),
                            _ => None,
                        })
                        .collect();
                    if vals.len() == cases.len() {
                        vals.sort();
                        let vals_str: Vec<String> = vals.iter().map(|v| v.to_string()).collect();
                        insertions.push((
                            i,
                            format!("// possible enum values: {{{}}}", vals_str.join(", ")),
                        ));
                    }
                }
            }
            _ => {}
        }
    }

    // Insert from back to front so earlier indices remain valid.
    for (idx, comment_text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(comment_text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn annotate_statement(stmt: &mut WrappedAstStatement, parameter_ids: &[AstVariableId]) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            annotate_statement_list(branch_true, parameter_ids);
            if let Some(branch_false) = branch_false {
                annotate_statement_list(branch_false, parameter_ids);
            }
        }
        AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
            annotate_statement_list(body, parameter_ids)
        }
        AstStatement::For(init, _, update, body) => {
            annotate_statement(init, parameter_ids);
            annotate_statement(update, parameter_ids);
            annotate_statement_list(body, parameter_ids);
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases.iter_mut() {
                annotate_statement_list(case_body, parameter_ids);
            }
            if let Some(default_body) = default {
                annotate_statement_list(default_body, parameter_ids);
            }
        }
        AstStatement::Block(body) => annotate_statement_list(body, parameter_ids),
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

/// Check whether a statement list contains a call to a stack canary function.
fn statement_list_contains_stack_canary(stmts: &[WrappedAstStatement]) -> bool {
    stmts.iter().any(|s| statement_contains_stack_canary(s))
}

fn statement_contains_stack_canary(stmt: &WrappedAstStatement) -> bool {
    match &stmt.statement {
        AstStatement::Call(call) => call_name_matches_stack_canary(call),
        AstStatement::If(_, branch_true, branch_false) => {
            statement_list_contains_stack_canary(branch_true)
                || branch_false
                    .as_ref()
                    .is_some_and(|bf| statement_list_contains_stack_canary(bf))
        }
        AstStatement::Block(body) => statement_list_contains_stack_canary(body),
        _ => false,
    }
}

fn call_name_matches_stack_canary(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    let lower = name.to_ascii_lowercase();
    lower.contains("stack_chk_fail") || lower.contains("__stack_chk")
}

fn call_name_matches_noreturn(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    let lower = name.to_ascii_lowercase();
    lower.contains("exit")
        || lower.contains("abort")
        || lower.contains("panic")
        || lower.contains("terminate")
}

fn call_name_matches_seh(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.contains("SEH") || name.contains("_except_handler")
}

/// Assertion pattern: `if (!cond) { noreturn_call(); }` with no else branch,
/// or `if (cond) { noreturn_call(); }` where the condition is a negation.
fn is_assertion_pattern(
    cond: &crate::abstract_syntax_tree::Wrapped<AstExpression>,
    branch_true: &[WrappedAstStatement],
    branch_false: Option<&[WrappedAstStatement]>,
) -> bool {
    // Must have no else branch (single-armed if).
    if branch_false.is_some() {
        return false;
    }
    // The then-branch must be exactly one noreturn call.
    if branch_true.len() != 1 {
        return false;
    }
    let call = match &branch_true[0].statement {
        AstStatement::Call(call) => call,
        _ => return false,
    };
    if !call_name_matches_noreturn(call) && !call_name_matches_assert(call) {
        return false;
    }
    // The condition should be a negation or comparison (typical assertion guard).
    matches!(
        &cond.item,
        AstExpression::UnaryOp(AstUnaryOperator::Not, _)
            | AstExpression::BinaryOp(AstBinaryOperator::Equal, _, _)
            | AstExpression::BinaryOp(AstBinaryOperator::NotEqual, _, _)
    )
}

fn call_name_matches_assert(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    let lower = name.to_ascii_lowercase();
    lower.contains("assert") || lower.contains("__assert")
}

/// Guarded call pattern: `if (var) { call(var, ...); }` with no else branch.
fn is_guarded_call_pattern(
    cond: &crate::abstract_syntax_tree::Wrapped<AstExpression>,
    branch_true: &[WrappedAstStatement],
    branch_false: Option<&[WrappedAstStatement]>,
) -> bool {
    if branch_false.is_some() {
        return false;
    }
    if branch_true.len() != 1 {
        return false;
    }
    // Condition must be a plain variable.
    let cond_var_id = match &cond.item {
        AstExpression::Variable(_, var_id) => *var_id,
        _ => return false,
    };
    // The single statement must be a call that uses the same variable.
    let call_args = match &branch_true[0].statement {
        AstStatement::Call(call) => match call {
            AstCall::Unknown(_, args) | AstCall::Function { args, .. } => args,
            AstCall::Variable { args, .. } => args,
            _ => return false,
        },
        _ => return false,
    };
    call_args
        .iter()
        .any(|arg| matches!(&arg.item, AstExpression::Variable(_, vid) if *vid == cond_var_id))
}

fn detect_bounds_checked_access_comment(
    cond: &crate::abstract_syntax_tree::Wrapped<AstExpression>,
    branch_true: &[WrappedAstStatement],
    branch_false: Option<&[WrappedAstStatement]>,
) -> Option<&'static str> {
    if branch_false.is_some() {
        return None;
    }
    let index_var = extract_in_bounds_index_var(&cond.item)?;
    let index_vars = HashSet::from([index_var]);
    branch_true
        .iter()
        .any(|stmt| statement_uses_ring_index(&stmt.statement, &index_vars))
        .then_some("// bounds-checked indexed access")
}

fn extract_in_bounds_index_var(expr: &AstExpression) -> Option<AstVariableId> {
    let AstExpression::BinaryOp(op, left, right) = expr else {
        return None;
    };

    match op {
        AstBinaryOperator::Less => match (&left.item, &right.item) {
            (AstExpression::Variable(_, var_id), rhs) if !expr_mentions_var(*var_id, rhs) => {
                Some(*var_id)
            }
            _ => None,
        },
        AstBinaryOperator::Greater => match (&left.item, &right.item) {
            (lhs, AstExpression::Variable(_, var_id)) if !expr_mentions_var(*var_id, lhs) => {
                Some(*var_id)
            }
            _ => None,
        },
        _ => None,
    }
}

fn detect_floating_compare_comment(expr: &AstExpression) -> Option<&'static str> {
    let AstExpression::BinaryOp(op, left, right) = expr else {
        return None;
    };

    match op {
        AstBinaryOperator::Equal
        | AstBinaryOperator::NotEqual
        | AstBinaryOperator::Less
        | AstBinaryOperator::LessEqual
        | AstBinaryOperator::Greater
        | AstBinaryOperator::GreaterEqual => {}
        _ => return None,
    }

    (expr_is_float_like(&left.item) || expr_is_float_like(&right.item))
        .then_some("// NaN-sensitive floating comparison")
}

fn expr_is_float_like(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Literal(AstLiteral::Float(_)) => true,
        AstExpression::Variable(var_map, var_id) => var_map.read().ok().is_some_and(|vars| {
            vars.get(var_id)
                .is_some_and(|var| is_float_value_type(&var.var_type))
        }),
        AstExpression::Cast(value_type, inner) => {
            is_float_value_type(value_type) || expr_is_float_like(&inner.item)
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner) => expr_is_float_like(&inner.item),
        AstExpression::BinaryOp(_, left, right) | AstExpression::ArrayAccess(left, right) => {
            expr_is_float_like(&left.item) || expr_is_float_like(&right.item)
        }
        AstExpression::MemberAccess(base, _) => expr_is_float_like(&base.item),
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            expr_is_float_like(&cond.item)
                || expr_is_float_like(&true_expr.item)
                || expr_is_float_like(&false_expr.item)
        }
        AstExpression::Call(_) => false,
        AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_) => false,
    }
}

fn is_float_value_type(value_type: &AstValueType) -> bool {
    matches!(value_type, AstValueType::Float | AstValueType::Double)
}

fn detect_fp_exception_sensitive_comment(expr: &AstExpression) -> Option<&'static str> {
    expr_has_fp_exception_sensitive_op(expr)
        .then_some("// floating-point arithmetic: denormal / FP-exception behavior may matter")
}

fn expr_has_fp_exception_sensitive_op(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::BinaryOp(op, left, right) => {
            let direct_match = matches!(
                op,
                AstBinaryOperator::Add
                    | AstBinaryOperator::Sub
                    | AstBinaryOperator::Mul
                    | AstBinaryOperator::Div
            ) && (expr_is_float_like(&left.item)
                || expr_is_float_like(&right.item));

            direct_match
                || expr_has_fp_exception_sensitive_op(&left.item)
                || expr_has_fp_exception_sensitive_op(&right.item)
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner) => expr_has_fp_exception_sensitive_op(&inner.item),
        AstExpression::ArrayAccess(base, index) => {
            expr_has_fp_exception_sensitive_op(&base.item)
                || expr_has_fp_exception_sensitive_op(&index.item)
        }
        AstExpression::MemberAccess(base, _) => expr_has_fp_exception_sensitive_op(&base.item),
        AstExpression::Ternary(cond, true_expr, false_expr) => {
            expr_has_fp_exception_sensitive_op(&cond.item)
                || expr_has_fp_exception_sensitive_op(&true_expr.item)
                || expr_has_fp_exception_sensitive_op(&false_expr.item)
        }
        AstExpression::Call(call) => call_has_fp_exception_sensitive_op(call),
        AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_)
        | AstExpression::Variable(_, _) => false,
    }
}

fn call_has_fp_exception_sensitive_op(call: &AstCall) -> bool {
    match call {
        AstCall::Unknown(_, args)
        | AstCall::Function { args, .. }
        | AstCall::Variable { args, .. } => args
            .iter()
            .any(|arg| expr_has_fp_exception_sensitive_op(&arg.item)),
        AstCall::Builtin(_, arg) => builtin_arg_has_fp_exception_sensitive_op(arg),
    }
}

fn builtin_arg_has_fp_exception_sensitive_op(arg: &AstBuiltinFunctionArgument) -> bool {
    match arg {
        AstBuiltinFunctionArgument::None => false,
        AstBuiltinFunctionArgument::Print(args) => args
            .iter()
            .any(|arg| expr_has_fp_exception_sensitive_op(&arg.item)),
        AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | AstBuiltinFunctionArgument::BitSizeOf(expr)
        | AstBuiltinFunctionArgument::OperandExists(expr)
        | AstBuiltinFunctionArgument::SignedMax(expr)
        | AstBuiltinFunctionArgument::SignedMin(expr)
        | AstBuiltinFunctionArgument::UnsignedMax(expr)
        | AstBuiltinFunctionArgument::UnsignedMin(expr)
        | AstBuiltinFunctionArgument::BitOnes(expr)
        | AstBuiltinFunctionArgument::BitZeros(expr) => {
            expr_has_fp_exception_sensitive_op(&expr.item)
        }
        AstBuiltinFunctionArgument::Sized(lhs, rhs) => {
            expr_has_fp_exception_sensitive_op(&lhs.item)
                || expr_has_fp_exception_sensitive_op(&rhs.item)
        }
    }
}

/// Detect MIN/MAX ternary patterns: `(a < b) ? a : b` → MIN, `(a > b) ? a : b` → MAX.
fn detect_min_max_pattern(expr: &AstExpression) -> Option<&'static str> {
    let AstExpression::Ternary(cond, true_expr, false_expr) = expr else {
        return None;
    };
    let AstExpression::BinaryOp(op, cond_lhs, cond_rhs) = &cond.item else {
        return None;
    };
    // Check if true_expr structurally matches cond_lhs and false_expr matches cond_rhs.
    let true_matches_lhs = exprs_structurally_equal(&true_expr.item, &cond_lhs.item);
    let false_matches_rhs = exprs_structurally_equal(&false_expr.item, &cond_rhs.item);
    let true_matches_rhs = exprs_structurally_equal(&true_expr.item, &cond_rhs.item);
    let false_matches_lhs = exprs_structurally_equal(&false_expr.item, &cond_lhs.item);

    match op {
        // (a < b) ? a : b → MIN(a, b)   or   (a < b) ? b : a → MAX(a, b)
        AstBinaryOperator::Less | AstBinaryOperator::LessEqual => {
            if true_matches_lhs && false_matches_rhs {
                Some("MIN")
            } else if true_matches_rhs && false_matches_lhs {
                Some("MAX")
            } else {
                None
            }
        }
        // (a > b) ? a : b → MAX(a, b)   or   (a > b) ? b : a → MIN(a, b)
        AstBinaryOperator::Greater | AstBinaryOperator::GreaterEqual => {
            if true_matches_lhs && false_matches_rhs {
                Some("MAX")
            } else if true_matches_rhs && false_matches_lhs {
                Some("MIN")
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Conservative structural equality check for expressions (variable identity only).
fn exprs_structurally_equal(a: &AstExpression, b: &AstExpression) -> bool {
    match (a, b) {
        (AstExpression::Variable(_, id_a), AstExpression::Variable(_, id_b)) => id_a == id_b,
        (AstExpression::Literal(lit_a), AstExpression::Literal(lit_b)) => lit_a == lit_b,
        _ => false,
    }
}

/// Fingerprint hash/crypto functions by checking if the function body contains
/// multiple known cryptographic initialization constants.
fn annotate_crypto_fingerprint(body: &mut Vec<WrappedAstStatement>) {
    let mut constants = std::collections::HashSet::new();
    collect_integer_literals_from_list(body, &mut constants);

    let mut labels = Vec::new();

    // MD5 init constants
    let md5_inits: &[u64] = &[0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476];
    if md5_inits.iter().filter(|c| constants.contains(c)).count() >= 3 {
        labels.push("MD5");
    }

    // SHA-256 init constants
    let sha256_inits: &[u64] = &[0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A];
    if sha256_inits
        .iter()
        .filter(|c| constants.contains(c))
        .count()
        >= 3
    {
        labels.push("SHA-256");
    }

    // CRC-32 polynomials
    if constants.contains(&0xEDB88320) || constants.contains(&0x04C11DB7) {
        labels.push("CRC-32");
    }
    if constants.contains(&0x82F63B78) {
        labels.push("CRC-32C");
    }

    // SHA-1 specific constant (beyond shared MD5/SHA-1 H0-H3)
    if constants.contains(&0xC3D2E1F0)
        && md5_inits.iter().filter(|c| constants.contains(c)).count() >= 3
    {
        // SHA-1 uses the same first 4 as MD5 plus 0xC3D2E1F0
        labels.push("SHA-1");
    }

    // AES S-box first/last entries (commonly embedded as table)
    if constants.contains(&0x63) && constants.contains(&0x7C) && constants.contains(&0x77) {
        // Too common individually, but this triple is the AES S-box header
        if constants.contains(&0x16) {
            labels.push("AES (S-box)");
        }
    }

    if !labels.is_empty() {
        let comment = format!("// likely {} routine", labels.join("/"));
        body.insert(
            0,
            WrappedAstStatement {
                statement: AstStatement::Comment(comment),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn collect_integer_literals_from_list(
    stmts: &[WrappedAstStatement],
    out: &mut std::collections::HashSet<u64>,
) {
    for stmt in stmts {
        collect_integer_literals_from_stmt(&stmt.statement, out);
    }
}

fn collect_integer_literals_from_stmt(
    stmt: &AstStatement,
    out: &mut std::collections::HashSet<u64>,
) {
    match stmt {
        AstStatement::Declaration(_, Some(init)) => {
            collect_integer_literals_from_expr(&init.item, out);
        }
        AstStatement::Assignment(lhs, rhs) => {
            collect_integer_literals_from_expr(&lhs.item, out);
            collect_integer_literals_from_expr(&rhs.item, out);
        }
        AstStatement::If(cond, bt, bf) => {
            collect_integer_literals_from_expr(&cond.item, out);
            collect_integer_literals_from_list(bt, out);
            if let Some(bf) = bf {
                collect_integer_literals_from_list(bf, out);
            }
        }
        AstStatement::While(cond, body) => {
            collect_integer_literals_from_expr(&cond.item, out);
            collect_integer_literals_from_list(body, out);
        }
        AstStatement::For(init, cond, update, body) => {
            collect_integer_literals_from_stmt(&init.statement, out);
            collect_integer_literals_from_expr(&cond.item, out);
            collect_integer_literals_from_stmt(&update.statement, out);
            collect_integer_literals_from_list(body, out);
        }
        AstStatement::Switch(disc, cases, default) => {
            collect_integer_literals_from_expr(&disc.item, out);
            for (_, case_body) in cases {
                collect_integer_literals_from_list(case_body, out);
            }
            if let Some(default_body) = default {
                collect_integer_literals_from_list(default_body, out);
            }
        }
        AstStatement::Block(body) => collect_integer_literals_from_list(body, out),
        AstStatement::Return(Some(expr)) => {
            collect_integer_literals_from_expr(&expr.item, out);
        }
        AstStatement::Call(call) => collect_integer_literals_from_call(call, out),
        _ => {}
    }
}

fn collect_integer_literals_from_expr(
    expr: &AstExpression,
    out: &mut std::collections::HashSet<u64>,
) {
    match expr {
        AstExpression::Literal(AstLiteral::Int(v)) if *v >= 0 => {
            out.insert(*v as u64);
        }
        AstExpression::Literal(AstLiteral::UInt(v)) => {
            out.insert(*v);
        }
        AstExpression::UnaryOp(_, arg) | AstExpression::Cast(_, arg) => {
            collect_integer_literals_from_expr(&arg.item, out);
        }
        AstExpression::BinaryOp(_, l, r) => {
            collect_integer_literals_from_expr(&l.item, out);
            collect_integer_literals_from_expr(&r.item, out);
        }
        AstExpression::Deref(e)
        | AstExpression::AddressOf(e)
        | AstExpression::MemberAccess(e, _) => {
            collect_integer_literals_from_expr(&e.item, out);
        }
        AstExpression::ArrayAccess(base, idx) => {
            collect_integer_literals_from_expr(&base.item, out);
            collect_integer_literals_from_expr(&idx.item, out);
        }
        AstExpression::Ternary(cond, t, f) => {
            collect_integer_literals_from_expr(&cond.item, out);
            collect_integer_literals_from_expr(&t.item, out);
            collect_integer_literals_from_expr(&f.item, out);
        }
        AstExpression::Call(call) => collect_integer_literals_from_call(call, out),
        _ => {}
    }
}

fn collect_integer_literals_from_call(call: &AstCall, out: &mut std::collections::HashSet<u64>) {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args {
                collect_integer_literals_from_expr(&arg.item, out);
            }
        }
        AstCall::Builtin(_, _) => {}
    }
}

/// Check if an expression is a null pointer check: `var == 0` or `var != 0`
/// where the variable has a pointer type.
fn is_null_pointer_check(cond: &crate::abstract_syntax_tree::Wrapped<AstExpression>) -> bool {
    let AstExpression::BinaryOp(op, lhs, rhs) = &cond.item else {
        return false;
    };

    if !matches!(op, AstBinaryOperator::Equal | AstBinaryOperator::NotEqual) {
        return false;
    }

    let (var_side, lit_side) = match (&lhs.item, &rhs.item) {
        (AstExpression::Variable(_, _), AstExpression::Literal(_)) => (&lhs.item, &rhs.item),
        (AstExpression::Literal(_), AstExpression::Variable(_, _)) => (&rhs.item, &lhs.item),
        _ => return false,
    };

    // The literal side must be 0.
    let is_zero = match lit_side {
        AstExpression::Literal(AstLiteral::Int(0)) => true,
        AstExpression::Literal(AstLiteral::UInt(0)) => true,
        _ => false,
    };
    if !is_zero {
        return false;
    }

    // The variable side must be a pointer type.
    if let AstExpression::Variable(var_map, var_id) = var_side {
        let vars = var_map.read().unwrap();
        if let Some(var) = vars.get(var_id) {
            return matches!(&var.var_type, AstValueType::Pointer(_));
        }
    }

    false
}

/// Detect C++ static local initialization guards: __cxa_guard_acquire/release.
fn call_name_matches_guard(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.contains("__cxa_guard_acquire")
        || name.contains("__cxa_guard_release")
        || name.contains("__cxa_guard_abort")
        || name.contains("_Init_thread_header")
        || name.contains("_Init_thread_footer")
}

/// Detect anti-debug / anti-analysis API calls.
fn call_name_matches_anti_debug(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    let lower = name.to_ascii_lowercase();
    lower.contains("isdebuggerpresent")
        || lower.contains("checkremotedebuggerpresent")
        || lower.contains("ntqueryinformationprocess")
        || lower.contains("outputdebugstring")
        || lower.contains("ntsetinformationthread")
        || lower.contains("ptrace")
}

/// Detect sanitizer / coverage / fuzzer instrumentation calls.
fn call_name_matches_instrumentation(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.starts_with("__asan_")
        || name.starts_with("__ubsan_")
        || name.starts_with("__tsan_")
        || name.starts_with("__msan_")
        || name.starts_with("__gcov_")
        || name.starts_with("__llvm_profile_")
        || name.starts_with("__llvm_gcov_")
        || name.starts_with("__afl_")
        || name.starts_with("__sanitizer_")
        || name.starts_with("__sancov_")
}

/// Detect retpoline / indirect call thunks.
fn call_name_matches_retpoline(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.starts_with("__x86_indirect_thunk_") || name.starts_with("__x86_return_thunk")
}

/// Detect lock/unlock synchronization API calls.
fn call_name_matches_lock(call: &AstCall) -> Option<&'static str> {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return None,
    };
    let lower = name.to_ascii_lowercase();
    if lower.contains("pthread_mutex_lock")
        || lower.contains("entercriticalsection")
        || lower.contains("acquiresrwlock")
        || lower.contains("waitforsingleobject")
        || lower.contains("pthread_spin_lock")
    {
        return Some("// lock acquire");
    }
    if lower.contains("pthread_mutex_unlock")
        || lower.contains("leavecriticalsection")
        || lower.contains("releasesrwlock")
        || lower.contains("releasemutex")
        || lower.contains("pthread_spin_unlock")
    {
        return Some("// lock release");
    }
    None
}

/// Detect reference counting API calls (COM AddRef/Release, Interlocked*).
fn call_name_matches_refcount(call: &AstCall) -> Option<&'static str> {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return None,
    };
    if name.contains("AddRef") || name.contains("InterlockedIncrement") {
        return Some("// ref count increment");
    }
    if name.contains("Release") && (name.contains("::") || name.len() <= 20) {
        return Some("// ref count decrement");
    }
    if name.contains("InterlockedDecrement") {
        return Some("// ref count decrement");
    }
    None
}

/// Detect logging / telemetry / debug output calls.
fn call_name_matches_logging(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    let lower = name.to_ascii_lowercase();
    lower == "syslog"
        || lower == "nslog"
        || lower == "__android_log_print"
        || lower == "__android_log_write"
        || lower.contains("outputdebugstring")
        || lower == "eventwrite"
        || lower == "traceloggingwrite"
        || lower.starts_with("etw")
}

/// Detect heap allocation / free calls.
fn call_name_matches_alloc(call: &AstCall) -> Option<&'static str> {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return None,
    };
    match name {
        "malloc" | "calloc" | "realloc" | "_aligned_malloc" | "HeapAlloc" | "VirtualAlloc"
        | "mmap" | "GlobalAlloc" | "LocalAlloc" => Some("// heap allocation"),
        "free" | "_aligned_free" | "HeapFree" | "VirtualFree" | "munmap" | "GlobalFree"
        | "LocalFree" => Some("// heap free"),
        _ if name.starts_with("operator new") => Some("// heap allocation"),
        _ if name.starts_with("operator delete") => Some("// heap free"),
        _ => None,
    }
}

/// Detect timing/measurement API calls used in anti-analysis.
fn call_name_matches_timing(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    let lower = name.to_ascii_lowercase();
    lower == "queryperformancecounter"
        || lower == "queryperformancefrequency"
        || lower == "gettickcount"
        || lower == "gettickcount64"
        || lower == "timegettime"
        || lower == "clock_gettime"
        || lower == "gettimeofday"
        || lower == "rdtsc"
        || lower == "__rdtsc"
}

/// Detect dynamic stack allocation calls (alloca/VLA).
fn call_name_matches_alloca(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name == "alloca"
        || name == "_alloca"
        || name == "__builtin_alloca"
        || name == "__chkstk"
        || name == "__alloca_probe"
        || name == "___chkstk_ms"
        || name == "__probestackspace"
}

/// Detect common C++/Rust managed-container APIs from preserved symbol names.
fn call_name_matches_container(call: &AstCall) -> Option<&'static str> {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return None,
    };
    let lower = name.to_ascii_lowercase();

    let vector_like = (lower.contains("vector") || lower.contains("deque"))
        && (lower.contains("push_back")
            || lower.contains("emplace_back")
            || lower.contains("reserve")
            || lower.contains("resize")
            || lower.contains("capacity")
            || lower.contains("operator[]")
            || lower.contains("at(")
            || lower.ends_with("::at")
            || lower.ends_with("::data"));
    if vector_like {
        return Some("// vector-like container operation");
    }

    let list_like = (lower.contains("list") || lower.contains("forward_list"))
        && (lower.contains("push_front")
            || lower.contains("push_back")
            || lower.contains("pop_front")
            || lower.contains("pop_back")
            || lower.contains("splice")
            || lower.contains("erase")
            || lower.contains("insert"));
    if list_like {
        return Some("// list-like container operation");
    }

    let map_like =
        (lower.contains("map") || lower.contains("unordered_map") || lower.contains("hash_map"))
            && (lower.contains("find")
                || lower.contains("insert")
                || lower.contains("erase")
                || lower.contains("emplace")
                || lower.contains("operator[]")
                || lower.contains("rehash")
                || lower.contains("bucket"));
    if map_like {
        return Some("// map-like container operation");
    }

    None
}

#[derive(Debug, Clone, Copy, Default)]
struct TreeChildUse {
    left: bool,
    right: bool,
}

impl TreeChildUse {
    fn merge(&mut self, other: Self) {
        self.left |= other.left;
        self.right |= other.right;
    }

    fn into_comment(self) -> Option<&'static str> {
        match (self.left, self.right) {
            (true, true) => Some("// likely recursive tree traversal (left/right child)"),
            (true, false) => Some("// likely recursive tree traversal (left child)"),
            (false, true) => Some("// likely recursive tree traversal (right child)"),
            (false, false) => None,
        }
    }
}

fn annotate_tree_recursion_comments(
    stmts: &mut Vec<WrappedAstStatement>,
    function_id: AstFunctionId,
) {
    let mut insertions = Vec::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        match &mut stmt.statement {
            AstStatement::If(_, t, f) => {
                annotate_tree_recursion_comments(t, function_id);
                if let Some(f) = f {
                    annotate_tree_recursion_comments(f, function_id);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => annotate_tree_recursion_comments(body, function_id),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_tree_recursion_comments(case_body, function_id);
                }
                if let Some(default) = default {
                    annotate_tree_recursion_comments(default, function_id);
                }
            }
            _ => {}
        }

        if stmt.comment.is_some() {
            continue;
        }

        if let Some(comment) = detect_tree_recursive_statement(&stmt.statement, function_id) {
            insertions.push((i, comment.to_string()));
        }
    }

    for (idx, text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn detect_tree_recursive_statement(
    stmt: &AstStatement,
    function_id: AstFunctionId,
) -> Option<&'static str> {
    match stmt {
        AstStatement::Assignment(_, rhs) => {
            detect_tree_recursive_expr(&rhs.item, function_id).into_comment()
        }
        AstStatement::Call(call) => detect_tree_recursive_call(call, function_id).into_comment(),
        AstStatement::Return(Some(expr)) => {
            detect_tree_recursive_expr(&expr.item, function_id).into_comment()
        }
        _ => None,
    }
}

fn detect_tree_recursive_expr(expr: &AstExpression, function_id: AstFunctionId) -> TreeChildUse {
    match expr {
        AstExpression::Call(call) => detect_tree_recursive_call(call, function_id),
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => {
            detect_tree_recursive_expr(&inner.item, function_id)
        }
        AstExpression::BinaryOp(_, left, right) | AstExpression::ArrayAccess(left, right) => {
            let mut usage = detect_tree_recursive_expr(&left.item, function_id);
            usage.merge(detect_tree_recursive_expr(&right.item, function_id));
            usage
        }
        AstExpression::Ternary(cond, t, f) => {
            let mut usage = detect_tree_recursive_expr(&cond.item, function_id);
            usage.merge(detect_tree_recursive_expr(&t.item, function_id));
            usage.merge(detect_tree_recursive_expr(&f.item, function_id));
            usage
        }
        AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_)
        | AstExpression::Variable(_, _) => TreeChildUse::default(),
    }
}

fn detect_tree_recursive_call(call: &AstCall, function_id: AstFunctionId) -> TreeChildUse {
    let AstCall::Function { target, args } = call else {
        return TreeChildUse::default();
    };
    if *target != function_id {
        return TreeChildUse::default();
    }

    let mut usage = TreeChildUse::default();
    for arg in args {
        usage.merge(detect_tree_child_access(&arg.item));
    }
    usage
}

fn detect_tree_child_access(expr: &AstExpression) -> TreeChildUse {
    match expr {
        AstExpression::MemberAccess(base, field) => {
            let mut usage = detect_tree_child_access(&base.item);
            let lower = field.to_ascii_lowercase();
            if matches!(lower.as_str(), "left" | "left_child" | "lchild") {
                usage.left = true;
            }
            if matches!(lower.as_str(), "right" | "right_child" | "rchild") {
                usage.right = true;
            }
            usage
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner) => detect_tree_child_access(&inner.item),
        AstExpression::BinaryOp(_, left, right) | AstExpression::ArrayAccess(left, right) => {
            let mut usage = detect_tree_child_access(&left.item);
            usage.merge(detect_tree_child_access(&right.item));
            usage
        }
        AstExpression::Call(call) => {
            let mut usage = TreeChildUse::default();
            match call {
                AstCall::Variable { args, .. }
                | AstCall::Function { args, .. }
                | AstCall::Unknown(_, args) => {
                    for arg in args {
                        usage.merge(detect_tree_child_access(&arg.item));
                    }
                }
                AstCall::Builtin(_, builtin) => match builtin.as_ref() {
                    crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Print(args) => {
                        for arg in args {
                            usage.merge(detect_tree_child_access(&arg.item));
                        }
                    }
                    crate::abstract_syntax_tree::AstBuiltinFunctionArgument::ByteSizeOf(expr)
                    | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitSizeOf(expr)
                    | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::OperandExists(
                        expr,
                    )
                    | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMax(expr)
                    | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMin(expr)
                    | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMax(expr)
                    | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMin(expr)
                    | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitOnes(expr)
                    | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitZeros(expr) => {
                        usage.merge(detect_tree_child_access(&expr.item));
                    }
                    crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Sized(
                        expr1,
                        expr2,
                    ) => {
                        usage.merge(detect_tree_child_access(&expr1.item));
                        usage.merge(detect_tree_child_access(&expr2.item));
                    }
                    crate::abstract_syntax_tree::AstBuiltinFunctionArgument::None => {}
                },
            }
            usage
        }
        AstExpression::Ternary(cond, t, f) => {
            let mut usage = detect_tree_child_access(&cond.item);
            usage.merge(detect_tree_child_access(&t.item));
            usage.merge(detect_tree_child_access(&f.item));
            usage
        }
        AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_)
        | AstExpression::Variable(_, _) => TreeChildUse::default(),
    }
}

fn annotate_hash_table_patterns(stmts: &mut Vec<WrappedAstStatement>) {
    let mut insertions = Vec::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        match &mut stmt.statement {
            AstStatement::If(_, t, f) => {
                annotate_hash_table_patterns(t);
                if let Some(f) = f {
                    annotate_hash_table_patterns(f);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => annotate_hash_table_patterns(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_hash_table_patterns(case_body);
                }
                if let Some(default) = default {
                    annotate_hash_table_patterns(default);
                }
            }
            _ => {}
        }

        if stmt.comment.is_some() {
            continue;
        }

        if let Some(comment) = detect_hash_table_statement(&stmt.statement) {
            insertions.push((i, comment.to_string()));
        }
    }

    for (idx, text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn annotate_ring_buffer_patterns(stmts: &mut Vec<WrappedAstStatement>) {
    let mut insertions = Vec::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        match &mut stmt.statement {
            AstStatement::If(_, t, f) => {
                annotate_ring_buffer_patterns(t);
                if let Some(f) = f {
                    annotate_ring_buffer_patterns(f);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => annotate_ring_buffer_patterns(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_ring_buffer_patterns(case_body);
                }
                if let Some(default) = default {
                    annotate_ring_buffer_patterns(default);
                }
            }
            _ => {}
        }

        if stmt.comment.is_some() {
            continue;
        }

        if let Some(comment) = detect_ring_buffer_statement(&stmt.statement) {
            insertions.push((i, comment.to_string()));
        }
    }

    for (idx, text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn detect_ring_buffer_statement(stmt: &AstStatement) -> Option<&'static str> {
    if detect_ring_buffer_loop(stmt) {
        return Some("// likely ring-buffer queue loop");
    }

    if detect_ring_wrap_update_var(stmt).is_some() {
        return Some("// likely ring-buffer head/tail advance");
    }

    None
}

fn annotate_refcount_field_patterns(stmts: &mut Vec<WrappedAstStatement>) {
    let mut insertions = Vec::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        match &mut stmt.statement {
            AstStatement::If(_, t, f) => {
                annotate_refcount_field_patterns(t);
                if let Some(f) = f {
                    annotate_refcount_field_patterns(f);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => annotate_refcount_field_patterns(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_refcount_field_patterns(case_body);
                }
                if let Some(default) = default {
                    annotate_refcount_field_patterns(default);
                }
            }
            _ => {}
        }

        if stmt.comment.is_some() {
            continue;
        }

        if let Some(comment) = detect_refcount_field_statement(&stmt.statement) {
            insertions.push((i, comment.to_string()));
        }
    }

    for (idx, text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn annotate_pointer_tagging_patterns(stmts: &mut Vec<WrappedAstStatement>) {
    let mut insertions = Vec::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        match &mut stmt.statement {
            AstStatement::If(_, t, f) => {
                annotate_pointer_tagging_patterns(t);
                if let Some(f) = f {
                    annotate_pointer_tagging_patterns(f);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => annotate_pointer_tagging_patterns(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_pointer_tagging_patterns(case_body);
                }
                if let Some(default) = default {
                    annotate_pointer_tagging_patterns(default);
                }
            }
            _ => {}
        }

        if stmt.comment.is_some() {
            continue;
        }

        if let Some(comment) = detect_pointer_tagging_statement(&stmt.statement) {
            insertions.push((i, comment));
        }
    }

    for (idx, text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn annotate_tagged_union_patterns(stmts: &mut Vec<WrappedAstStatement>) {
    let mut insertions = Vec::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        match &mut stmt.statement {
            AstStatement::If(_, t, f) => {
                annotate_tagged_union_patterns(t);
                if let Some(f) = f {
                    annotate_tagged_union_patterns(f);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => annotate_tagged_union_patterns(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_tagged_union_patterns(case_body);
                }
                if let Some(default) = default {
                    annotate_tagged_union_patterns(default);
                }
            }
            _ => {}
        }

        if stmt.comment.is_some() {
            continue;
        }

        if let Some(comment) = detect_tagged_union_statement(&stmt.statement) {
            insertions.push((i, comment));
        }
    }

    for (idx, text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn detect_tagged_union_statement(stmt: &AstStatement) -> Option<String> {
    match stmt {
        AstStatement::If(cond, branch_true, Some(branch_false)) => {
            detect_tagged_union_if(&cond.item, branch_true, branch_false)
        }
        AstStatement::Switch(disc, cases, default) => {
            detect_tagged_union_switch(&disc.item, cases, default.as_ref())
        }
        _ => None,
    }
}

fn detect_tagged_union_if(
    cond: &AstExpression,
    branch_true: &[WrappedAstStatement],
    branch_false: &[WrappedAstStatement],
) -> Option<String> {
    let tag_path = extract_tagged_union_if_discriminant(cond)?;
    let true_fields = collect_tagged_union_variant_fields(branch_true, &tag_path);
    let false_fields = collect_tagged_union_variant_fields(branch_false, &tag_path);

    let true_unique = unique_tagged_union_field_descriptions(&true_fields, &false_fields);
    let false_unique = unique_tagged_union_field_descriptions(&false_fields, &true_fields);

    if true_unique.is_empty() || false_unique.is_empty() {
        return None;
    }

    Some(format!(
        "// likely tagged-union variant dispatch on {}: then uses {}, else uses {}",
        describe_member_field_path(&tag_path),
        summarize_tagged_union_fields(&true_unique),
        summarize_tagged_union_fields(&false_unique),
    ))
}

fn detect_tagged_union_switch(
    disc: &AstExpression,
    cases: &[(AstLiteral, Vec<WrappedAstStatement>)],
    default: Option<&Vec<WrappedAstStatement>>,
) -> Option<String> {
    let tag_path = extract_member_field_path(disc)?;
    if tag_path.fields.is_empty() {
        return None;
    }

    let variant_fields: Vec<Vec<MemberFieldPath>> = cases
        .iter()
        .map(|(_, case_body)| collect_tagged_union_variant_fields(case_body, &tag_path))
        .chain(
            default
                .into_iter()
                .map(|default_body| collect_tagged_union_variant_fields(default_body, &tag_path)),
        )
        .collect();

    let mut unique_variants = Vec::new();
    for (idx, fields) in variant_fields.iter().enumerate() {
        let mut other_fields = Vec::new();
        for (other_idx, other) in variant_fields.iter().enumerate() {
            if idx != other_idx {
                other_fields.extend(other.iter().cloned());
            }
        }
        let unique = unique_tagged_union_field_descriptions(fields, &other_fields);
        if !unique.is_empty() {
            unique_variants.push(unique);
        }
    }

    if unique_variants.len() < 2 {
        return None;
    }

    let examples = unique_variants
        .iter()
        .take(2)
        .map(|fields| summarize_tagged_union_fields(fields))
        .collect::<Vec<_>>()
        .join("; ");

    Some(format!(
        "// likely tagged-union variant switch on {}: case bodies use {}",
        describe_member_field_path(&tag_path),
        examples,
    ))
}

fn extract_tagged_union_if_discriminant(cond: &AstExpression) -> Option<MemberFieldPath> {
    match cond {
        AstExpression::BinaryOp(
            AstBinaryOperator::Equal | AstBinaryOperator::NotEqual,
            left,
            right,
        ) => {
            let left_path = extract_member_field_path(&left.item);
            let right_path = extract_member_field_path(&right.item);
            let left_lit = expr_is_tagged_union_discriminant_literal(&left.item);
            let right_lit = expr_is_tagged_union_discriminant_literal(&right.item);

            match (left_path, right_path, left_lit, right_lit) {
                (Some(path), None, _, true) | (Some(path), None, true, false) => {
                    if !path.fields.is_empty() {
                        Some(path)
                    } else {
                        None
                    }
                }
                (None, Some(path), true, _) | (None, Some(path), false, true) => {
                    if !path.fields.is_empty() {
                        Some(path)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn expr_is_tagged_union_discriminant_literal(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Literal(
            AstLiteral::Int(_) | AstLiteral::UInt(_) | AstLiteral::Bool(_) | AstLiteral::Char(_),
        ) => true,
        AstExpression::Cast(_, inner) => expr_is_tagged_union_discriminant_literal(&inner.item),
        _ => false,
    }
}

fn collect_tagged_union_variant_fields(
    stmts: &[WrappedAstStatement],
    tag_path: &MemberFieldPath,
) -> Vec<MemberFieldPath> {
    let mut candidates = Vec::new();
    for stmt in stmts {
        collect_member_field_paths_from_statement(&stmt.statement, &mut candidates);
    }

    let mut seen = HashSet::new();
    candidates
        .into_iter()
        .filter(|path| path.root_var == tag_path.root_var)
        .filter(|path| !path.fields.is_empty())
        .filter(|path| *path != *tag_path)
        .filter(|path| seen.insert(path.clone()))
        .collect()
}

fn unique_tagged_union_field_descriptions(
    fields: &[MemberFieldPath],
    other_fields: &[MemberFieldPath],
) -> Vec<String> {
    let other = other_fields
        .iter()
        .map(describe_member_field_path)
        .collect::<HashSet<_>>();
    fields
        .iter()
        .map(describe_member_field_path)
        .filter(|field| !other.contains(field))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn summarize_tagged_union_fields(fields: &[String]) -> String {
    let mut fields = fields.to_vec();
    fields.sort();
    if fields.len() > 2 {
        format!("{}, {}...", fields[0], fields[1])
    } else {
        fields.join(", ")
    }
}

fn detect_pointer_tagging_statement(stmt: &AstStatement) -> Option<String> {
    if statement_contains_pointer_tag_untagging(stmt) {
        return Some(
            "// likely low-bit tagged pointer: clears tag bits before pointer use".to_string(),
        );
    }

    if statement_contains_pointer_tag_test(stmt) {
        return Some("// likely low-bit tagged pointer test".to_string());
    }

    None
}

fn statement_contains_pointer_tag_untagging(stmt: &AstStatement) -> bool {
    match stmt {
        AstStatement::Declaration(_, Some(init)) => expr_contains_pointer_tag_untagging(&init.item),
        AstStatement::Assignment(lhs, rhs) => {
            expr_contains_pointer_tag_untagging(&lhs.item)
                || expr_contains_pointer_tag_untagging(&rhs.item)
        }
        AstStatement::If(cond, _, _) => expr_contains_pointer_tag_untagging(&cond.item),
        AstStatement::While(cond, _) | AstStatement::DoWhile(cond, _) => {
            expr_contains_pointer_tag_untagging(&cond.item)
        }
        AstStatement::For(init, cond, step, _) => {
            statement_contains_pointer_tag_untagging(&init.statement)
                || expr_contains_pointer_tag_untagging(&cond.item)
                || statement_contains_pointer_tag_untagging(&step.statement)
        }
        AstStatement::Return(Some(expr)) => expr_contains_pointer_tag_untagging(&expr.item),
        AstStatement::Call(call) => call_contains_pointer_tag_untagging(call),
        AstStatement::Switch(expr, _, _) => expr_contains_pointer_tag_untagging(&expr.item),
        _ => false,
    }
}

fn statement_contains_pointer_tag_test(stmt: &AstStatement) -> bool {
    match stmt {
        AstStatement::If(cond, _, _)
        | AstStatement::While(cond, _)
        | AstStatement::DoWhile(cond, _) => expr_contains_pointer_tag_test(&cond.item),
        AstStatement::For(init, cond, step, _) => {
            statement_contains_pointer_tag_test(&init.statement)
                || expr_contains_pointer_tag_test(&cond.item)
                || statement_contains_pointer_tag_test(&step.statement)
        }
        _ => false,
    }
}

fn expr_contains_pointer_tag_untagging(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Deref(inner) | AstExpression::MemberAccess(inner, _) => {
            tagged_pointer_clear_mask_bits(&inner.item).is_some()
                || expr_contains_pointer_tag_untagging(&inner.item)
        }
        AstExpression::ArrayAccess(base, index) => {
            tagged_pointer_clear_mask_bits(&base.item).is_some()
                || expr_contains_pointer_tag_untagging(&base.item)
                || expr_contains_pointer_tag_untagging(&index.item)
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::AddressOf(inner) => expr_contains_pointer_tag_untagging(&inner.item),
        AstExpression::BinaryOp(_, left, right) => {
            expr_contains_pointer_tag_untagging(&left.item)
                || expr_contains_pointer_tag_untagging(&right.item)
        }
        AstExpression::Call(call) => call_contains_pointer_tag_untagging(call),
        AstExpression::Ternary(cond, t, f) => {
            expr_contains_pointer_tag_untagging(&cond.item)
                || expr_contains_pointer_tag_untagging(&t.item)
                || expr_contains_pointer_tag_untagging(&f.item)
        }
        _ => false,
    }
}

fn call_contains_pointer_tag_untagging(call: &AstCall) -> bool {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => args
            .iter()
            .any(|arg| expr_contains_pointer_tag_untagging(&arg.item)),
        AstCall::Builtin(_, arg) => builtin_arg_contains_pointer_tag_untagging(arg),
    }
}

fn builtin_arg_contains_pointer_tag_untagging(arg: &AstBuiltinFunctionArgument) -> bool {
    match arg {
        AstBuiltinFunctionArgument::None => false,
        AstBuiltinFunctionArgument::Print(args) => args
            .iter()
            .any(|arg| expr_contains_pointer_tag_untagging(&arg.item)),
        AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | AstBuiltinFunctionArgument::BitSizeOf(expr)
        | AstBuiltinFunctionArgument::OperandExists(expr)
        | AstBuiltinFunctionArgument::SignedMax(expr)
        | AstBuiltinFunctionArgument::SignedMin(expr)
        | AstBuiltinFunctionArgument::UnsignedMax(expr)
        | AstBuiltinFunctionArgument::UnsignedMin(expr)
        | AstBuiltinFunctionArgument::BitOnes(expr)
        | AstBuiltinFunctionArgument::BitZeros(expr) => {
            expr_contains_pointer_tag_untagging(&expr.item)
        }
        AstBuiltinFunctionArgument::Sized(lhs, rhs) => {
            expr_contains_pointer_tag_untagging(&lhs.item)
                || expr_contains_pointer_tag_untagging(&rhs.item)
        }
    }
}

fn expr_contains_pointer_tag_test(expr: &AstExpression) -> bool {
    if pointer_tag_test_mask_bits(expr).is_some() {
        return true;
    }

    match expr {
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => expr_contains_pointer_tag_test(&inner.item),
        AstExpression::BinaryOp(_, left, right) | AstExpression::ArrayAccess(left, right) => {
            expr_contains_pointer_tag_test(&left.item)
                || expr_contains_pointer_tag_test(&right.item)
        }
        AstExpression::Call(call) => match call {
            AstCall::Variable { args, .. }
            | AstCall::Function { args, .. }
            | AstCall::Unknown(_, args) => args
                .iter()
                .any(|arg| expr_contains_pointer_tag_test(&arg.item)),
            AstCall::Builtin(_, arg) => builtin_arg_contains_pointer_tag_test(arg),
        },
        AstExpression::Ternary(cond, t, f) => {
            expr_contains_pointer_tag_test(&cond.item)
                || expr_contains_pointer_tag_test(&t.item)
                || expr_contains_pointer_tag_test(&f.item)
        }
        _ => false,
    }
}

fn builtin_arg_contains_pointer_tag_test(arg: &AstBuiltinFunctionArgument) -> bool {
    match arg {
        AstBuiltinFunctionArgument::None => false,
        AstBuiltinFunctionArgument::Print(args) => args
            .iter()
            .any(|arg| expr_contains_pointer_tag_test(&arg.item)),
        AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | AstBuiltinFunctionArgument::BitSizeOf(expr)
        | AstBuiltinFunctionArgument::OperandExists(expr)
        | AstBuiltinFunctionArgument::SignedMax(expr)
        | AstBuiltinFunctionArgument::SignedMin(expr)
        | AstBuiltinFunctionArgument::UnsignedMax(expr)
        | AstBuiltinFunctionArgument::UnsignedMin(expr)
        | AstBuiltinFunctionArgument::BitOnes(expr)
        | AstBuiltinFunctionArgument::BitZeros(expr) => expr_contains_pointer_tag_test(&expr.item),
        AstBuiltinFunctionArgument::Sized(lhs, rhs) => {
            expr_contains_pointer_tag_test(&lhs.item) || expr_contains_pointer_tag_test(&rhs.item)
        }
    }
}

fn pointer_tag_test_mask_bits(expr: &AstExpression) -> Option<u64> {
    match expr {
        AstExpression::BinaryOp(AstBinaryOperator::BitAnd, left, right) => {
            if let Some(mask) = lowbit_tag_mask_bits(&left.item) {
                if expr_might_be_pointer(&right.item) {
                    return Some(mask);
                }
            }
            if let Some(mask) = lowbit_tag_mask_bits(&right.item) {
                if expr_might_be_pointer(&left.item) {
                    return Some(mask);
                }
            }
            None
        }
        AstExpression::BinaryOp(
            AstBinaryOperator::Equal | AstBinaryOperator::NotEqual,
            left,
            right,
        ) => {
            if is_zero_literal(&left.item) {
                pointer_tag_test_mask_bits(&right.item)
            } else if is_zero_literal(&right.item) {
                pointer_tag_test_mask_bits(&left.item)
            } else {
                None
            }
        }
        AstExpression::UnaryOp(AstUnaryOperator::Not, inner) | AstExpression::Cast(_, inner) => {
            pointer_tag_test_mask_bits(&inner.item)
        }
        _ => None,
    }
}

fn tagged_pointer_clear_mask_bits(expr: &AstExpression) -> Option<u64> {
    match expr {
        AstExpression::BinaryOp(AstBinaryOperator::BitAnd, left, right) => {
            if let Some(mask) = lowbit_clear_mask_bits(&left.item) {
                if expr_might_be_pointer(&right.item) {
                    return Some(mask);
                }
            }
            if let Some(mask) = lowbit_clear_mask_bits(&right.item) {
                if expr_might_be_pointer(&left.item) {
                    return Some(mask);
                }
            }
            None
        }
        AstExpression::Cast(_, inner) => tagged_pointer_clear_mask_bits(&inner.item),
        _ => None,
    }
}

fn expr_might_be_pointer(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Variable(_, _) => true,
        AstExpression::Deref(_) | AstExpression::AddressOf(_) => true,
        AstExpression::MemberAccess(inner, _) => expr_might_be_pointer(&inner.item),
        AstExpression::ArrayAccess(base, _) => expr_might_be_pointer(&base.item),
        AstExpression::Cast(_, inner) => expr_might_be_pointer(&inner.item),
        AstExpression::BinaryOp(AstBinaryOperator::Add | AstBinaryOperator::Sub, left, right) => {
            (expr_might_be_pointer(&left.item) && expr_is_small_int(&right.item))
                || (expr_might_be_pointer(&right.item) && expr_is_small_int(&left.item))
        }
        AstExpression::Call(_) => true,
        _ => false,
    }
}

fn expr_is_small_int(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Literal(AstLiteral::Int(value)) => value.unsigned_abs() <= 0x100,
        AstExpression::Literal(AstLiteral::UInt(value)) => *value <= 0x100,
        AstExpression::Cast(_, inner) => expr_is_small_int(&inner.item),
        _ => false,
    }
}

fn lowbit_tag_mask_bits(expr: &AstExpression) -> Option<u64> {
    match expr {
        AstExpression::Literal(AstLiteral::Int(value)) if *value > 0 => {
            let mask = *value as u64;
            is_lowbit_tag_mask(mask).then_some(mask)
        }
        AstExpression::Literal(AstLiteral::UInt(value)) => {
            is_lowbit_tag_mask(*value).then_some(*value)
        }
        AstExpression::Cast(_, inner) => lowbit_tag_mask_bits(&inner.item),
        _ => None,
    }
}

fn lowbit_clear_mask_bits(expr: &AstExpression) -> Option<u64> {
    match expr {
        AstExpression::UnaryOp(AstUnaryOperator::BitNot, inner) => {
            lowbit_tag_mask_bits(&inner.item)
        }
        AstExpression::Literal(AstLiteral::Int(value)) => {
            let cleared = !(*value as u64);
            is_lowbit_tag_mask(cleared).then_some(cleared)
        }
        AstExpression::Literal(AstLiteral::UInt(value)) => {
            let cleared = !*value;
            is_lowbit_tag_mask(cleared).then_some(cleared)
        }
        AstExpression::Cast(_, inner) => lowbit_clear_mask_bits(&inner.item),
        _ => None,
    }
}

fn is_lowbit_tag_mask(mask: u64) -> bool {
    mask != 0 && mask <= 0xff && (mask + 1).is_power_of_two()
}

fn is_zero_literal(expr: &AstExpression) -> bool {
    matches!(
        expr,
        AstExpression::Literal(AstLiteral::Int(0)) | AstExpression::Literal(AstLiteral::UInt(0))
    )
}

fn detect_refcount_field_statement(stmt: &AstStatement) -> Option<&'static str> {
    let AstStatement::Assignment(lhs, rhs) = stmt else {
        return None;
    };
    let path = extract_member_field_path(&lhs.item)?;
    let field = path.fields.last()?;

    if !is_refcount_field_name(field) {
        return None;
    }

    if expr_is_member_field_increment(&rhs.item, &path) {
        return Some("// likely refcount field increment");
    }
    if expr_is_member_field_decrement(&rhs.item, &path) {
        return Some("// likely refcount field decrement");
    }

    None
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct MemberFieldPath {
    root_var: AstVariableId,
    fields: Vec<String>,
}

fn extract_member_field_path(expr: &AstExpression) -> Option<MemberFieldPath> {
    match expr {
        AstExpression::Variable(_, var_id) => Some(MemberFieldPath {
            root_var: *var_id,
            fields: Vec::new(),
        }),
        AstExpression::MemberAccess(base, field) => {
            let mut path = extract_member_field_path(&base.item)?;
            path.fields.push(field.to_ascii_lowercase());
            Some(path)
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner) => extract_member_field_path(&inner.item),
        _ => None,
    }
}

fn is_refcount_field_name(field: &str) -> bool {
    field == "refs"
        || field.ends_with("_refs")
        || field.contains("refcount")
        || field.contains("ref_count")
        || field.contains("reference_count")
        || field.contains("refcnt")
        || field.contains("usecount")
        || field.contains("use_count")
        || field.contains("sharedcount")
        || field.contains("shared_count")
        || field.contains("strongcount")
        || field.contains("strong_count")
        || field.contains("retaincount")
        || field.contains("retain_count")
        || field.contains("retaincnt")
}

fn expr_is_member_field_increment(expr: &AstExpression, path: &MemberFieldPath) -> bool {
    match expr {
        AstExpression::BinaryOp(AstBinaryOperator::Add, left, right) => {
            (expr_matches_member_field_path(&left.item, path) && is_literal_one(&right.item))
                || (expr_matches_member_field_path(&right.item, path) && is_literal_one(&left.item))
        }
        _ => false,
    }
}

fn expr_is_member_field_decrement(expr: &AstExpression, path: &MemberFieldPath) -> bool {
    match expr {
        AstExpression::BinaryOp(AstBinaryOperator::Sub, left, right) => {
            expr_matches_member_field_path(&left.item, path) && is_literal_one(&right.item)
        }
        _ => false,
    }
}

fn expr_matches_member_field_path(expr: &AstExpression, path: &MemberFieldPath) -> bool {
    extract_member_field_path(expr).is_some_and(|candidate| candidate == *path)
}

fn is_literal_one(expr: &AstExpression) -> bool {
    matches!(
        expr,
        AstExpression::Literal(AstLiteral::Int(1)) | AstExpression::Literal(AstLiteral::UInt(1))
    )
}

fn detect_ring_buffer_loop(stmt: &AstStatement) -> bool {
    let body = match stmt {
        AstStatement::While(_, body)
        | AstStatement::DoWhile(_, body)
        | AstStatement::For(_, _, _, body) => body,
        _ => return false,
    };

    let ring_vars: HashSet<_> = body
        .iter()
        .filter_map(|stmt| detect_ring_wrap_update_var(&stmt.statement))
        .collect();

    !ring_vars.is_empty() && body_uses_ring_index(body, &ring_vars)
}

fn detect_ring_wrap_update_var(stmt: &AstStatement) -> Option<AstVariableId> {
    let AstStatement::Assignment(lhs, rhs) = stmt else {
        return None;
    };
    let AstExpression::Variable(var_map, var_id) = &lhs.item else {
        return None;
    };
    let vars = var_map.read().unwrap();
    let lower = vars.get(var_id)?.name().to_ascii_lowercase();

    if !is_ring_position_name(&lower) {
        return None;
    }

    if expr_is_ring_wrap_step_for_var(&rhs.item, *var_id) {
        return Some(*var_id);
    }

    None
}

fn is_ring_position_name(lower: &str) -> bool {
    lower.contains("head")
        || lower.contains("tail")
        || lower.contains("read_pos")
        || lower.contains("write_pos")
        || lower.contains("read_idx")
        || lower.contains("write_idx")
        || lower.contains("read_index")
        || lower.contains("write_index")
        || lower.contains("rpos")
        || lower.contains("wpos")
}

fn expr_is_ring_wrap_step_for_var(expr: &AstExpression, var_id: AstVariableId) -> bool {
    match expr {
        AstExpression::BinaryOp(AstBinaryOperator::Mod, left, _) => {
            is_increment_like_for_var(&left.item, var_id)
        }
        AstExpression::BinaryOp(AstBinaryOperator::BitAnd, left, right) => {
            (is_increment_like_for_var(&left.item, var_id)
                && is_power_of_two_minus_one_like(&right.item))
                || (is_increment_like_for_var(&right.item, var_id)
                    && is_power_of_two_minus_one_like(&left.item))
        }
        AstExpression::Cast(_, inner) => expr_is_ring_wrap_step_for_var(&inner.item, var_id),
        _ => false,
    }
}

fn body_uses_ring_index(body: &[WrappedAstStatement], ring_vars: &HashSet<AstVariableId>) -> bool {
    body.iter()
        .any(|stmt| statement_uses_ring_index(&stmt.statement, ring_vars))
}

fn statement_uses_ring_index(stmt: &AstStatement, ring_vars: &HashSet<AstVariableId>) -> bool {
    match stmt {
        AstStatement::Declaration(_, Some(rhs)) => {
            expr_has_array_access_with_index_var(&rhs.item, ring_vars)
        }
        AstStatement::Declaration(_, None) => false,
        AstStatement::Assignment(lhs, rhs) => {
            expr_has_array_access_with_index_var(&lhs.item, ring_vars)
                || expr_has_array_access_with_index_var(&rhs.item, ring_vars)
        }
        AstStatement::Call(call) => call_has_array_access_with_index_var(call, ring_vars),
        AstStatement::Return(Some(expr)) => {
            expr_has_array_access_with_index_var(&expr.item, ring_vars)
        }
        AstStatement::If(cond, t, f) => {
            expr_has_array_access_with_index_var(&cond.item, ring_vars)
                || t.iter()
                    .any(|stmt| statement_uses_ring_index(&stmt.statement, ring_vars))
                || f.as_ref().is_some_and(|f| {
                    f.iter()
                        .any(|stmt| statement_uses_ring_index(&stmt.statement, ring_vars))
                })
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            expr_has_array_access_with_index_var(&cond.item, ring_vars)
                || body
                    .iter()
                    .any(|stmt| statement_uses_ring_index(&stmt.statement, ring_vars))
        }
        AstStatement::For(init, cond, step, body) => {
            statement_uses_ring_index(&init.statement, ring_vars)
                || expr_has_array_access_with_index_var(&cond.item, ring_vars)
                || statement_uses_ring_index(&step.statement, ring_vars)
                || body
                    .iter()
                    .any(|stmt| statement_uses_ring_index(&stmt.statement, ring_vars))
        }
        AstStatement::Switch(disc, cases, default) => {
            expr_has_array_access_with_index_var(&disc.item, ring_vars)
                || cases.iter().any(|(_, body)| {
                    body.iter()
                        .any(|stmt| statement_uses_ring_index(&stmt.statement, ring_vars))
                })
                || default.as_ref().is_some_and(|body| {
                    body.iter()
                        .any(|stmt| statement_uses_ring_index(&stmt.statement, ring_vars))
                })
        }
        AstStatement::Block(body) => body
            .iter()
            .any(|stmt| statement_uses_ring_index(&stmt.statement, ring_vars)),
        _ => false,
    }
}

fn call_has_array_access_with_index_var(
    call: &AstCall,
    ring_vars: &HashSet<AstVariableId>,
) -> bool {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => args
            .iter()
            .any(|arg| expr_has_array_access_with_index_var(&arg.item, ring_vars)),
        AstCall::Builtin(_, builtin) => {
            builtin_has_array_access_with_index_var(builtin.as_ref(), ring_vars)
        }
    }
}

fn builtin_has_array_access_with_index_var(
    builtin: &crate::abstract_syntax_tree::AstBuiltinFunctionArgument,
    ring_vars: &HashSet<AstVariableId>,
) -> bool {
    match builtin {
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::None => false,
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Print(args) => args
            .iter()
            .any(|expr| expr_has_array_access_with_index_var(&expr.item, ring_vars)),
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::OperandExists(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitOnes(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitZeros(expr) => {
            expr_has_array_access_with_index_var(&expr.item, ring_vars)
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
            expr_has_array_access_with_index_var(&expr1.item, ring_vars)
                || expr_has_array_access_with_index_var(&expr2.item, ring_vars)
        }
    }
}

fn expr_has_array_access_with_index_var(
    expr: &AstExpression,
    ring_vars: &HashSet<AstVariableId>,
) -> bool {
    match expr {
        AstExpression::ArrayAccess(base, index) => {
            expr_mentions_any_var(&index.item, ring_vars)
                || expr_has_array_access_with_index_var(&base.item, ring_vars)
                || expr_has_array_access_with_index_var(&index.item, ring_vars)
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => {
            expr_has_array_access_with_index_var(&inner.item, ring_vars)
        }
        AstExpression::BinaryOp(_, left, right) => {
            expr_has_array_access_with_index_var(&left.item, ring_vars)
                || expr_has_array_access_with_index_var(&right.item, ring_vars)
        }
        AstExpression::Call(call) => call_has_array_access_with_index_var(call, ring_vars),
        AstExpression::Ternary(cond, t, f) => {
            expr_has_array_access_with_index_var(&cond.item, ring_vars)
                || expr_has_array_access_with_index_var(&t.item, ring_vars)
                || expr_has_array_access_with_index_var(&f.item, ring_vars)
        }
        _ => false,
    }
}

fn expr_mentions_any_var(expr: &AstExpression, ring_vars: &HashSet<AstVariableId>) -> bool {
    match expr {
        AstExpression::Variable(_, var_id) => ring_vars.contains(var_id),
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => expr_mentions_any_var(&inner.item, ring_vars),
        AstExpression::BinaryOp(_, left, right) | AstExpression::ArrayAccess(left, right) => {
            expr_mentions_any_var(&left.item, ring_vars)
                || expr_mentions_any_var(&right.item, ring_vars)
        }
        AstExpression::Call(call) => match call {
            AstCall::Variable { args, .. }
            | AstCall::Function { args, .. }
            | AstCall::Unknown(_, args) => args
                .iter()
                .any(|arg| expr_mentions_any_var(&arg.item, ring_vars)),
            AstCall::Builtin(_, builtin) => {
                builtin_has_array_access_with_index_var(builtin.as_ref(), ring_vars)
            }
        },
        AstExpression::Ternary(cond, t, f) => {
            expr_mentions_any_var(&cond.item, ring_vars)
                || expr_mentions_any_var(&t.item, ring_vars)
                || expr_mentions_any_var(&f.item, ring_vars)
        }
        _ => false,
    }
}

fn detect_hash_table_statement(stmt: &AstStatement) -> Option<&'static str> {
    if detect_hash_probe_loop(stmt) {
        return Some("// likely hash table probing loop");
    }

    if statement_has_hash_bucket_index(stmt) {
        return Some("// likely hash bucket indexing");
    }

    None
}

fn detect_hash_probe_loop(stmt: &AstStatement) -> bool {
    let body = match stmt {
        AstStatement::While(_, body)
        | AstStatement::DoWhile(_, body)
        | AstStatement::For(_, _, _, body) => body,
        _ => return false,
    };

    let mut updated_index = None;
    let mut has_bucket_index = false;

    for stmt in body {
        if !has_bucket_index && statement_has_hash_bucket_index(&stmt.statement) {
            has_bucket_index = true;
        }

        if updated_index.is_none() {
            updated_index = extract_probe_update_var(&stmt.statement);
        }
    }

    let Some(index_var) = updated_index else {
        return false;
    };

    has_bucket_index && body_uses_var_in_bucket_index(body, index_var)
}

fn extract_probe_update_var(stmt: &AstStatement) -> Option<AstVariableId> {
    let AstStatement::Assignment(lhs, rhs) = stmt else {
        return None;
    };
    let AstExpression::Variable(_, var_id) = &lhs.item else {
        return None;
    };
    if expr_is_probe_step_for_var(&rhs.item, *var_id) {
        return Some(*var_id);
    }
    None
}

fn expr_is_probe_step_for_var(expr: &AstExpression, var_id: AstVariableId) -> bool {
    match expr {
        AstExpression::BinaryOp(AstBinaryOperator::Mod, left, _modulus) => {
            is_increment_like_for_var(&left.item, var_id)
        }
        AstExpression::BinaryOp(AstBinaryOperator::BitAnd, left, right) => {
            (is_increment_like_for_var(&left.item, var_id)
                && is_power_of_two_minus_one_like(&right.item))
                || (is_increment_like_for_var(&right.item, var_id)
                    && is_power_of_two_minus_one_like(&left.item))
        }
        _ => false,
    }
}

fn is_increment_like_for_var(expr: &AstExpression, var_id: AstVariableId) -> bool {
    match expr {
        AstExpression::BinaryOp(AstBinaryOperator::Add, left, right) => {
            (matches!(&left.item, AstExpression::Variable(_, v) if *v == var_id)
                && is_small_positive_int(&right.item))
                || (matches!(&right.item, AstExpression::Variable(_, v) if *v == var_id)
                    && is_small_positive_int(&left.item))
        }
        AstExpression::Variable(_, v) => *v == var_id,
        _ => false,
    }
}

fn is_small_positive_int(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Literal(AstLiteral::Int(v)) => *v > 0 && *v <= 8,
        AstExpression::Literal(AstLiteral::UInt(v)) => *v > 0 && *v <= 8,
        _ => false,
    }
}

fn is_power_of_two_minus_one_like(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Literal(AstLiteral::Int(v)) if *v > 0 => {
            let as_u64 = *v as u64;
            (as_u64 & (as_u64 + 1)) == 0
        }
        AstExpression::Literal(AstLiteral::UInt(v)) if *v > 0 => (*v & (*v + 1)) == 0,
        _ => false,
    }
}

fn statement_has_hash_bucket_index(stmt: &AstStatement) -> bool {
    match stmt {
        AstStatement::Declaration(_, Some(rhs)) => expr_has_hash_bucket_index(&rhs.item),
        AstStatement::Declaration(_, None) => false,
        AstStatement::Assignment(lhs, rhs) => {
            expr_has_hash_bucket_index(&lhs.item) || expr_has_hash_bucket_index(&rhs.item)
        }
        AstStatement::Call(call) => call_has_hash_bucket_index(call),
        AstStatement::Return(Some(expr)) => expr_has_hash_bucket_index(&expr.item),
        AstStatement::If(cond, t, f) => {
            expr_has_hash_bucket_index(&cond.item)
                || t.iter()
                    .any(|stmt| statement_has_hash_bucket_index(&stmt.statement))
                || f.as_ref().is_some_and(|f| {
                    f.iter()
                        .any(|stmt| statement_has_hash_bucket_index(&stmt.statement))
                })
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            expr_has_hash_bucket_index(&cond.item)
                || body
                    .iter()
                    .any(|stmt| statement_has_hash_bucket_index(&stmt.statement))
        }
        AstStatement::For(init, cond, step, body) => {
            statement_has_hash_bucket_index(&init.statement)
                || expr_has_hash_bucket_index(&cond.item)
                || statement_has_hash_bucket_index(&step.statement)
                || body
                    .iter()
                    .any(|stmt| statement_has_hash_bucket_index(&stmt.statement))
        }
        AstStatement::Switch(disc, cases, default) => {
            expr_has_hash_bucket_index(&disc.item)
                || cases.iter().any(|(_, body)| {
                    body.iter()
                        .any(|stmt| statement_has_hash_bucket_index(&stmt.statement))
                })
                || default.as_ref().is_some_and(|body| {
                    body.iter()
                        .any(|stmt| statement_has_hash_bucket_index(&stmt.statement))
                })
        }
        AstStatement::Block(body) => body
            .iter()
            .any(|stmt| statement_has_hash_bucket_index(&stmt.statement)),
        _ => false,
    }
}

fn body_uses_var_in_bucket_index(body: &[WrappedAstStatement], var_id: AstVariableId) -> bool {
    body.iter()
        .any(|stmt| statement_uses_var_in_bucket_index(&stmt.statement, var_id))
}

fn statement_uses_var_in_bucket_index(stmt: &AstStatement, var_id: AstVariableId) -> bool {
    match stmt {
        AstStatement::Declaration(_, Some(rhs)) => expr_uses_var_in_bucket_index(&rhs.item, var_id),
        AstStatement::Declaration(_, None) => false,
        AstStatement::Assignment(lhs, rhs) => {
            expr_uses_var_in_bucket_index(&lhs.item, var_id)
                || expr_uses_var_in_bucket_index(&rhs.item, var_id)
        }
        AstStatement::Call(call) => call_uses_var_in_bucket_index(call, var_id),
        AstStatement::Return(Some(expr)) => expr_uses_var_in_bucket_index(&expr.item, var_id),
        AstStatement::If(cond, t, f) => {
            expr_uses_var_in_bucket_index(&cond.item, var_id)
                || t.iter()
                    .any(|stmt| statement_uses_var_in_bucket_index(&stmt.statement, var_id))
                || f.as_ref().is_some_and(|f| {
                    f.iter()
                        .any(|stmt| statement_uses_var_in_bucket_index(&stmt.statement, var_id))
                })
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            expr_uses_var_in_bucket_index(&cond.item, var_id)
                || body
                    .iter()
                    .any(|stmt| statement_uses_var_in_bucket_index(&stmt.statement, var_id))
        }
        AstStatement::For(init, cond, step, body) => {
            statement_uses_var_in_bucket_index(&init.statement, var_id)
                || expr_uses_var_in_bucket_index(&cond.item, var_id)
                || statement_uses_var_in_bucket_index(&step.statement, var_id)
                || body
                    .iter()
                    .any(|stmt| statement_uses_var_in_bucket_index(&stmt.statement, var_id))
        }
        AstStatement::Switch(disc, cases, default) => {
            expr_uses_var_in_bucket_index(&disc.item, var_id)
                || cases.iter().any(|(_, body)| {
                    body.iter()
                        .any(|stmt| statement_uses_var_in_bucket_index(&stmt.statement, var_id))
                })
                || default.as_ref().is_some_and(|body| {
                    body.iter()
                        .any(|stmt| statement_uses_var_in_bucket_index(&stmt.statement, var_id))
                })
        }
        AstStatement::Block(body) => body
            .iter()
            .any(|stmt| statement_uses_var_in_bucket_index(&stmt.statement, var_id)),
        _ => false,
    }
}

fn call_has_hash_bucket_index(call: &AstCall) -> bool {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => args.iter().any(|arg| expr_has_hash_bucket_index(&arg.item)),
        AstCall::Builtin(_, builtin) => builtin_has_hash_bucket_index(builtin.as_ref()),
    }
}

fn call_uses_var_in_bucket_index(call: &AstCall, var_id: AstVariableId) -> bool {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => args
            .iter()
            .any(|arg| expr_uses_var_in_bucket_index(&arg.item, var_id)),
        AstCall::Builtin(_, builtin) => builtin_uses_var_in_bucket_index(builtin.as_ref(), var_id),
    }
}

fn builtin_has_hash_bucket_index(
    builtin: &crate::abstract_syntax_tree::AstBuiltinFunctionArgument,
) -> bool {
    match builtin {
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::None => false,
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Print(args) => {
            args.iter().any(|arg| expr_has_hash_bucket_index(&arg.item))
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::OperandExists(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitOnes(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitZeros(expr) => {
            expr_has_hash_bucket_index(&expr.item)
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
            expr_has_hash_bucket_index(&expr1.item) || expr_has_hash_bucket_index(&expr2.item)
        }
    }
}

fn builtin_uses_var_in_bucket_index(
    builtin: &crate::abstract_syntax_tree::AstBuiltinFunctionArgument,
    var_id: AstVariableId,
) -> bool {
    match builtin {
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::None => false,
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Print(args) => args
            .iter()
            .any(|arg| expr_uses_var_in_bucket_index(&arg.item, var_id)),
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::OperandExists(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitOnes(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitZeros(expr) => {
            expr_uses_var_in_bucket_index(&expr.item, var_id)
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
            expr_uses_var_in_bucket_index(&expr1.item, var_id)
                || expr_uses_var_in_bucket_index(&expr2.item, var_id)
        }
    }
}

fn expr_has_hash_bucket_index(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::ArrayAccess(base, index) => {
            index_is_hash_bucket_index(&index.item) || expr_has_hash_bucket_index(&base.item)
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => expr_has_hash_bucket_index(&inner.item),
        AstExpression::BinaryOp(_, left, right) => {
            expr_has_hash_bucket_index(&left.item) || expr_has_hash_bucket_index(&right.item)
        }
        AstExpression::Call(call) => call_has_hash_bucket_index(call),
        AstExpression::Ternary(cond, t, f) => {
            expr_has_hash_bucket_index(&cond.item)
                || expr_has_hash_bucket_index(&t.item)
                || expr_has_hash_bucket_index(&f.item)
        }
        AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_)
        | AstExpression::Variable(_, _) => false,
    }
}

fn expr_uses_var_in_bucket_index(expr: &AstExpression, var_id: AstVariableId) -> bool {
    match expr {
        AstExpression::ArrayAccess(base, index) => {
            (index_is_hash_bucket_index(&index.item) && expr_mentions_var(var_id, &index.item))
                || expr_uses_var_in_bucket_index(&base.item, var_id)
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => {
            expr_uses_var_in_bucket_index(&inner.item, var_id)
        }
        AstExpression::BinaryOp(_, left, right) => {
            expr_uses_var_in_bucket_index(&left.item, var_id)
                || expr_uses_var_in_bucket_index(&right.item, var_id)
        }
        AstExpression::Call(call) => call_uses_var_in_bucket_index(call, var_id),
        AstExpression::Ternary(cond, t, f) => {
            expr_uses_var_in_bucket_index(&cond.item, var_id)
                || expr_uses_var_in_bucket_index(&t.item, var_id)
                || expr_uses_var_in_bucket_index(&f.item, var_id)
        }
        AstExpression::Unknown
        | AstExpression::Undefined
        | AstExpression::ArchitectureBitSize
        | AstExpression::ArchitectureByteSize
        | AstExpression::Literal(_)
        | AstExpression::Variable(_, _) => false,
    }
}

fn index_is_hash_bucket_index(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::BinaryOp(AstBinaryOperator::Mod, _, _) => true,
        AstExpression::BinaryOp(AstBinaryOperator::BitAnd, left, right) => {
            is_power_of_two_minus_one_like(&left.item)
                || is_power_of_two_minus_one_like(&right.item)
        }
        _ => false,
    }
}

/// Detect Objective-C runtime dispatch calls.
fn call_name_matches_objc(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.starts_with("objc_msgSend")
        || name == "objc_alloc"
        || name == "objc_alloc_init"
        || name == "objc_release"
        || name == "objc_retain"
        || name == "objc_autoreleasePoolPush"
        || name == "objc_autoreleasePoolPop"
}

/// Detect decompression-routine fingerprint by checking for known compression
/// constants (zlib window size, Huffman table sizes, DEFLATE markers).
fn annotate_decompression_fingerprint(body: &mut Vec<WrappedAstStatement>) {
    let mut literals = std::collections::HashSet::new();
    collect_integer_literals_from_list(body, &mut literals);

    // zlib/DEFLATE constants
    let zlib_markers: &[u64] = &[
        0x8000, // 32768 window size
        288,    // # of literal/length codes
        320,    // total literal+distance codes
        30,     // # of distance codes
        286,    // end of literal codes
        15,     // max code length
        0x78,   // zlib default header byte
    ];
    let zlib_hits = zlib_markers.iter().filter(|c| literals.contains(c)).count();
    if zlib_hits >= 3 {
        body.insert(
            0,
            WrappedAstStatement {
                statement: AstStatement::Comment(
                    "// likely zlib/DEFLATE decompression routine".to_string(),
                ),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
        return;
    }

    // LZ77/LZ4/LZSS markers
    let lz_markers: &[u64] = &[
        0x1000, // 4096 ring buffer
        0x1FFF, // ring buffer mask
        0xFEE,  // LZSS initial fill position
        0x0F,   // LZ4 token nibble mask
        0xF0,   // LZ4 token nibble mask
        0x7F,   // length encoding
    ];
    let lz_hits = lz_markers.iter().filter(|c| literals.contains(c)).count();
    if lz_hits >= 3 {
        body.insert(
            0,
            WrappedAstStatement {
                statement: AstStatement::Comment(
                    "// likely LZ-family decompression routine".to_string(),
                ),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

/// Detect XOR-based decryption loops: a loop whose body contains an assignment
/// with XOR on a dereferenced/array-accessed value (e.g., `*ptr ^= key`).
/// Detect SafeStack/split-stack runtime calls.
fn call_name_matches_safestack(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.starts_with("__safestack_") || name.starts_with("__splitstack_") || name == "__morestack"
}

/// Detect C++ exception handling runtime calls.
fn call_name_matches_exception(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.starts_with("__cxa_throw")
        || name.starts_with("__cxa_begin_catch")
        || name.starts_with("__cxa_end_catch")
        || name.starts_with("__cxa_allocate_exception")
        || name.starts_with("__cxa_rethrow")
        || name == "_Unwind_Resume"
        || name == "__gxx_personality_v0"
        || name.starts_with("__CxxFrameHandler")
        || name.starts_with("_CxxThrowException")
}

/// Detect thread-local storage runtime calls.
fn call_name_matches_tls(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.starts_with("__tls_init")
        || name.starts_with("__emutls_")
        || name == "TlsAlloc"
        || name == "TlsGetValue"
        || name == "TlsSetValue"
        || name == "TlsFree"
        || name == "pthread_getspecific"
        || name == "pthread_setspecific"
        || name == "pthread_key_create"
}

/// Detect Rust panic/unwind runtime calls.
/// Detect process/thread management API calls.
fn call_name_matches_process_thread(call: &AstCall) -> Option<&'static str> {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return None,
    };
    match name {
        "CreateProcessA" | "CreateProcessW" | "CreateProcess" | "fork" | "vfork" | "execve"
        | "execvp" | "posix_spawn" => Some("// process creation"),
        "ExitProcess" | "_exit" | "_Exit" | "exit" | "quick_exit" => Some("// process termination"),
        "CreateThread" | "CreateRemoteThread" | "pthread_create" | "_beginthreadex"
        | "_beginthread" => Some("// thread creation"),
        "WaitForSingleObject" | "WaitForMultipleObjects" | "pthread_join" | "waitpid" | "wait" => {
            Some("// wait / synchronization")
        }
        "VirtualProtect" | "VirtualProtectEx" | "mprotect" => Some("// memory protection change"),
        "VirtualAlloc" | "VirtualAllocEx" | "VirtualFree" => Some("// virtual memory management"),
        _ => None,
    }
}

fn detect_callback_environment_comment(
    call: &AstCall,
    parameter_ids: &[AstVariableId],
) -> Option<String> {
    let (name, args, callback_arg_index, context_arg_index, _) = callback_api_pattern(call)?;

    let callback_arg = args.get(callback_arg_index)?;
    let context_arg = args.get(context_arg_index)?;
    let callback_desc = describe_callback_target_expr(&callback_arg.item, parameter_ids)?;
    let context_target = extract_ownership_target_from_expr(&context_arg.item)?;
    let context_desc = describe_ownership_target(&context_target);

    Some(format!(
        "// likely callback environment: {} receives {} via \"{}\"",
        callback_desc, context_desc, name
    ))
}

fn detect_callback_signature_comment(
    call: &AstCall,
    parameter_ids: &[AstVariableId],
) -> Option<String> {
    let (name, args, callback_arg_index, _, signature) = callback_api_pattern(call)?;
    let callback_arg = args.get(callback_arg_index)?;
    let callback_desc = describe_callback_target_expr(&callback_arg.item, parameter_ids)?;
    Some(format!(
        "// likely callback signature: {} matches {} for \"{}\"",
        callback_desc, signature, name
    ))
}

fn detect_callback_provenance_comment(
    call: &AstCall,
    parameter_ids: &[AstVariableId],
) -> Option<String> {
    let (name, args, callback_arg_index, _, _) = callback_api_pattern(call)?;
    let callback_arg = args.get(callback_arg_index)?;
    let provenance = describe_callback_provenance(&callback_arg.item, parameter_ids)?;
    Some(format!(
        "// likely callback provenance: {} via \"{}\"",
        provenance, name
    ))
}

fn callback_api_pattern(
    call: &AstCall,
) -> Option<(&str, &[Wrapped<AstExpression>], usize, usize, &'static str)> {
    let AstCall::Unknown(name, args) = call else {
        return None;
    };

    match name.as_str() {
        "CreateThread" | "CreateRemoteThread" => Some((
            name.as_str(),
            args.as_slice(),
            2usize,
            3usize,
            "LPTHREAD_START_ROUTINE(void*) -> DWORD",
        )),
        "_beginthreadex" => Some((
            name.as_str(),
            args.as_slice(),
            2usize,
            3usize,
            "unsigned (__stdcall *)(void*)",
        )),
        "_beginthread" => Some((
            name.as_str(),
            args.as_slice(),
            0usize,
            2usize,
            "void (__cdecl *)(void*)",
        )),
        "pthread_create" => Some((
            name.as_str(),
            args.as_slice(),
            2usize,
            3usize,
            "void *(*)(void *)",
        )),
        "EnumWindows" | "EnumChildWindows" | "EnumThreadWindows" => Some((
            name.as_str(),
            args.as_slice(),
            0usize,
            1usize,
            "BOOL callback(HWND, LPARAM)",
        )),
        _ => None,
    }
}

fn describe_callback_target_expr(
    expr: &AstExpression,
    parameter_ids: &[AstVariableId],
) -> Option<String> {
    match expr {
        AstExpression::Variable(var_map, var_id) => {
            let name = get_ast_variable_name(var_map, *var_id);
            let source = callback_root_source(*var_id, parameter_ids, Some(&name));
            Some(format!("callback {} ({})", name, source))
        }
        AstExpression::ArrayAccess(base, _) => {
            describe_callback_table_source(&base.item, parameter_ids)
                .map(|source| format!("callback table slot from {}", source))
        }
        AstExpression::AddressOf(inner)
        | AstExpression::Deref(inner)
        | AstExpression::Cast(_, inner) => {
            describe_callback_target_expr(&inner.item, parameter_ids)
        }
        _ => extract_member_field_path(expr)
            .filter(|path| !path.fields.is_empty())
            .map(|path| {
                let kind = callback_path_kind(&path);
                let source = callback_path_source(&path, parameter_ids, kind);
                format!(
                    "callback {} ({})",
                    describe_member_field_path(&path),
                    source
                )
            }),
    }
}

fn describe_callback_provenance(
    expr: &AstExpression,
    parameter_ids: &[AstVariableId],
) -> Option<String> {
    match expr {
        AstExpression::Variable(var_map, var_id) => {
            let name = get_ast_variable_name(var_map, *var_id);
            let source = callback_root_source(*var_id, parameter_ids, Some(&name));
            Some(format!("callback {} comes from {}", name, source))
        }
        AstExpression::ArrayAccess(base, _) => {
            describe_callback_table_source(&base.item, parameter_ids)
                .map(|source| format!("callback comes from {} table slot", source))
        }
        AstExpression::AddressOf(inner)
        | AstExpression::Deref(inner)
        | AstExpression::Cast(_, inner) => describe_callback_provenance(&inner.item, parameter_ids),
        _ => extract_member_field_path(expr)
            .filter(|path| !path.fields.is_empty())
            .map(|path| {
                let kind = callback_path_kind(&path);
                let source = callback_path_source(&path, parameter_ids, kind);
                format!(
                    "callback {} comes from {}",
                    describe_member_field_path(&path),
                    source
                )
            }),
    }
}

fn describe_callback_table_source(
    expr: &AstExpression,
    parameter_ids: &[AstVariableId],
) -> Option<String> {
    match expr {
        AstExpression::Variable(var_map, var_id) => {
            let name = get_ast_variable_name(var_map, *var_id);
            let source = callback_root_source(*var_id, parameter_ids, Some(&name));
            Some(format!("{} table {}", source, name))
        }
        AstExpression::AddressOf(inner)
        | AstExpression::Deref(inner)
        | AstExpression::Cast(_, inner) => {
            describe_callback_table_source(&inner.item, parameter_ids)
        }
        _ => extract_member_field_path(expr)
            .filter(|path| !path.fields.is_empty())
            .map(|path| {
                let kind = callback_path_kind(&path);
                let source = callback_path_source(&path, parameter_ids, kind);
                format!("{} {}", source, describe_member_field_path(&path))
            }),
    }
}

fn get_ast_variable_name(var_map: &ArcAstVariableMap, var_id: AstVariableId) -> String {
    var_map
        .read()
        .ok()
        .and_then(|vars| vars.get(&var_id).map(|var| var.name()))
        .unwrap_or_else(|| "callback".to_string())
}

fn callback_root_source(
    var_id: AstVariableId,
    parameter_ids: &[AstVariableId],
    name: Option<&str>,
) -> &'static str {
    if parameter_ids.contains(&var_id) {
        "parameter"
    } else if var_id.parent.is_none() || name.is_some_and(|value| value.starts_with("global_")) {
        "global"
    } else {
        "local variable"
    }
}

fn callback_path_kind(path: &MemberFieldPath) -> &'static str {
    if path
        .fields
        .iter()
        .any(|field| matches!(field.as_str(), "vtable" | "vfptr" | "vptr" | "__vftable"))
    {
        "vtable-like field"
    } else {
        "field"
    }
}

fn callback_path_source(
    path: &MemberFieldPath,
    parameter_ids: &[AstVariableId],
    kind: &'static str,
) -> String {
    match callback_root_source(path.root_var, parameter_ids, None) {
        "parameter" => format!("parameter {}", kind),
        "global" => format!("global {}", kind),
        _ => kind.to_string(),
    }
}

/// Detect dynamic library loading calls.
fn call_name_matches_dynload(call: &AstCall) -> Option<&'static str> {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return None,
    };
    match name {
        "LoadLibraryA" | "LoadLibraryW" | "LoadLibraryExA" | "LoadLibraryExW" | "dlopen" => {
            Some("// dynamic library load")
        }
        "GetProcAddress" | "dlsym" => Some("// dynamic symbol resolve"),
        "FreeLibrary" | "dlclose" => Some("// dynamic library unload"),
        _ => None,
    }
}

/// Detect setjmp/longjmp non-local jump calls.
fn call_name_matches_setjmp(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name == "setjmp"
        || name == "_setjmp"
        || name == "sigsetjmp"
        || name == "longjmp"
        || name == "_longjmp"
        || name == "siglongjmp"
}

/// Detect atomic/interlocked operation calls.
fn call_name_matches_atomic(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.starts_with("__atomic_")
        || name.starts_with("__sync_")
        || name.starts_with("InterlockedCompareExchange")
        || name.starts_with("InterlockedExchange")
        || name.starts_with("InterlockedAdd")
        || name.starts_with("InterlockedOr")
        || name.starts_with("InterlockedAnd")
        || name.starts_with("InterlockedXor")
        || name == "atomic_load"
        || name == "atomic_store"
        || name == "atomic_compare_exchange_strong"
        || name == "atomic_compare_exchange_weak"
        || name == "atomic_fetch_add"
        || name == "atomic_fetch_sub"
}

fn call_name_matches_rust_panic(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    name.contains("rust_begin_unwind")
        || name.contains("_rust_panic")
        || name.contains("core::panicking")
        || name.contains("std::panicking")
        || name.contains("rust_panic")
        || name.contains("panic_bounds_check")
        || name.contains("panic_fmt")
        || name.contains("begin_panic")
}

/// Detect errno / last-error access functions.
fn call_name_matches_errno(call: &AstCall) -> Option<&'static str> {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return None,
    };
    match name {
        "GetLastError" | "__errno_location" | "__errno" | "_errno" | "errno" | "___error" => {
            Some("// read last error code")
        }
        "SetLastError" | "WSASetLastError" => Some("// set last error code"),
        _ => None,
    }
}

/// Detect file/socket/handle resource I/O calls.
fn call_name_matches_resource_io(call: &AstCall) -> Option<&'static str> {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return None,
    };
    match name {
        "fopen" | "fopen64" | "_wfopen" | "CreateFileA" | "CreateFileW" | "CreateFile" | "open"
        | "open64" | "_open" => Some("// file open"),
        "fclose" | "close" | "_close" | "CloseHandle" => Some("// resource close"),
        "fread" | "fwrite" | "read" | "write" | "ReadFile" | "WriteFile" => Some("// file I/O"),
        "socket" | "WSASocketA" | "WSASocketW" => Some("// socket creation"),
        "connect" | "WSAConnect" => Some("// socket connect"),
        "send" | "recv" | "sendto" | "recvfrom" | "WSASend" | "WSARecv" => Some("// network I/O"),
        "bind" | "listen" | "accept" | "WSAAccept" => Some("// socket server"),
        "RegOpenKeyExA" | "RegOpenKeyExW" | "RegOpenKey" => Some("// registry open"),
        "RegQueryValueExA" | "RegQueryValueExW" => Some("// registry read"),
        "RegSetValueExA" | "RegSetValueExW" => Some("// registry write"),
        "RegCloseKey" => Some("// registry close"),
        _ => None,
    }
}

/// Detect common C string manipulation calls.
fn call_name_matches_string_op(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    matches!(
        name,
        "strcpy"
            | "strncpy"
            | "strcat"
            | "strncat"
            | "strcmp"
            | "strncmp"
            | "strlen"
            | "strtok"
            | "strstr"
            | "strchr"
            | "strrchr"
            | "wcscpy"
            | "wcsncpy"
            | "wcscat"
            | "wcsncat"
            | "wcscmp"
            | "wcsncmp"
            | "wcslen"
            | "wcsstr"
            | "wcschr"
            | "memcpy"
            | "memmove"
            | "memset"
            | "memcmp"
            | "lstrcpyA"
            | "lstrcpyW"
            | "lstrcmpA"
            | "lstrcmpW"
            | "lstrlenA"
            | "lstrlenW"
            | "sprintf"
            | "snprintf"
            | "swprintf"
            | "_snwprintf"
            | "sscanf"
            | "swscanf"
    )
}

/// Detect standard math library calls.
fn call_name_matches_math(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    matches!(
        name,
        "sin"
            | "cos"
            | "tan"
            | "asin"
            | "acos"
            | "atan"
            | "atan2"
            | "sinf"
            | "cosf"
            | "tanf"
            | "asinf"
            | "acosf"
            | "atanf"
            | "atan2f"
            | "sqrt"
            | "sqrtf"
            | "cbrt"
            | "cbrtf"
            | "pow"
            | "powf"
            | "exp"
            | "expf"
            | "exp2"
            | "exp2f"
            | "log"
            | "logf"
            | "log2"
            | "log2f"
            | "log10"
            | "log10f"
            | "floor"
            | "floorf"
            | "ceil"
            | "ceilf"
            | "round"
            | "roundf"
            | "fabs"
            | "fabsf"
            | "fmod"
            | "fmodf"
    )
}

/// Detect well-known variadic (vararg) function calls.
fn call_name_matches_vararg(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return false,
    };
    matches!(
        name,
        "printf"
            | "fprintf"
            | "sprintf"
            | "snprintf"
            | "dprintf"
            | "wprintf"
            | "fwprintf"
            | "swprintf"
            | "_snwprintf"
            | "scanf"
            | "fscanf"
            | "sscanf"
            | "swscanf"
            | "vprintf"
            | "vfprintf"
            | "vsprintf"
            | "vsnprintf"
            | "vasprintf"
            | "vwprintf"
            | "vswprintf"
            | "vscanf"
            | "vfscanf"
            | "vsscanf"
            | "ioctl"
            | "fcntl"
            | "open"
            | "execl"
            | "execlp"
            | "execle"
    )
}

fn annotate_xor_decryption_loop(body: &mut Vec<WrappedAstStatement>) {
    for stmt in body.iter_mut() {
        // Recurse first
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                annotate_xor_decryption_loop(bt);
                if let Some(bf) = bf {
                    annotate_xor_decryption_loop(bf);
                }
            }
            AstStatement::Block(b) => annotate_xor_decryption_loop(b),
            AstStatement::Switch(_, cases, def) => {
                for (_, cb) in cases.iter_mut() {
                    annotate_xor_decryption_loop(cb);
                }
                if let Some(d) = def {
                    annotate_xor_decryption_loop(d);
                }
            }
            _ => {}
        }

        let loop_body = match &stmt.statement {
            AstStatement::While(_, body) | AstStatement::For(_, _, _, body) => body,
            _ => continue,
        };

        if stmt.comment.is_some() {
            continue;
        }

        // Check if the loop body contains a XOR assignment on a memory access
        if loop_body_has_xor_on_memory(loop_body) {
            stmt.comment = Some("likely XOR decryption/encoding loop".to_string());
        }
    }
}

fn loop_body_has_xor_on_memory(stmts: &[WrappedAstStatement]) -> bool {
    for s in stmts {
        match &s.statement {
            AstStatement::Assignment(lhs, rhs) => {
                let lhs_is_mem = matches!(
                    &lhs.item,
                    AstExpression::Deref(_) | AstExpression::ArrayAccess(_, _)
                );
                if lhs_is_mem && expr_contains_xor(&rhs.item) {
                    return true;
                }
            }
            AstStatement::If(_, bt, bf) => {
                if loop_body_has_xor_on_memory(bt) {
                    return true;
                }
                if let Some(bf) = bf {
                    if loop_body_has_xor_on_memory(bf) {
                        return true;
                    }
                }
            }
            AstStatement::Block(b) => {
                if loop_body_has_xor_on_memory(b) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn expr_contains_xor(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::BinaryOp(AstBinaryOperator::BitXor, _, _) => true,
        AstExpression::BinaryOp(_, l, r) => {
            expr_contains_xor(&l.item) || expr_contains_xor(&r.item)
        }
        AstExpression::UnaryOp(_, arg) | AstExpression::Cast(_, arg) => {
            expr_contains_xor(&arg.item)
        }
        _ => false,
    }
}

/// Detect integrity-check loops: a loop that accumulates a value using
/// ADD/XOR/ROL-like operations over a memory range, suggesting CRC/hash/checksum.
fn annotate_integrity_check_loop(body: &mut Vec<WrappedAstStatement>) {
    for stmt in body.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                annotate_integrity_check_loop(bt);
                if let Some(bf) = bf {
                    annotate_integrity_check_loop(bf);
                }
            }
            AstStatement::Block(b) => annotate_integrity_check_loop(b),
            AstStatement::Switch(_, cases, def) => {
                for (_, cb) in cases.iter_mut() {
                    annotate_integrity_check_loop(cb);
                }
                if let Some(d) = def {
                    annotate_integrity_check_loop(d);
                }
            }
            _ => {}
        }

        let loop_body = match &stmt.statement {
            AstStatement::While(_, body) | AstStatement::For(_, _, _, body) => body,
            _ => continue,
        };

        if stmt.comment.is_some() {
            continue;
        }

        // Look for accumulator pattern: var = var OP memory_read
        // where OP is XOR, ADD, or bitwise combination
        if loop_body_has_accumulator_on_memory(loop_body) {
            stmt.comment = Some("likely checksum/integrity verification loop".to_string());
        }
    }
}

fn loop_body_has_accumulator_on_memory(stmts: &[WrappedAstStatement]) -> bool {
    for s in stmts {
        if let AstStatement::Assignment(lhs, rhs) = &s.statement {
            // lhs must be a simple variable
            let AstExpression::Variable(lhs_map, lhs_id) = &lhs.item else {
                continue;
            };
            // rhs must be a binary op where one side is the SAME variable as lhs
            // and the other side accesses memory
            if let AstExpression::BinaryOp(op, left, right) = &rhs.item {
                if !matches!(
                    op,
                    AstBinaryOperator::BitXor
                        | AstBinaryOperator::Add
                        | AstBinaryOperator::BitOr
                        | AstBinaryOperator::LeftShift
                        | AstBinaryOperator::RightShift
                ) {
                    continue;
                }
                let left_is_same_var = matches!(
                    &left.item,
                    AstExpression::Variable(m, id) if std::ptr::eq(
                        &**m as *const _ as *const u8,
                        &**lhs_map as *const _ as *const u8
                    ) && id == lhs_id
                );
                let right_is_same_var = matches!(
                    &right.item,
                    AstExpression::Variable(m, id) if std::ptr::eq(
                        &**m as *const _ as *const u8,
                        &**lhs_map as *const _ as *const u8
                    ) && id == lhs_id
                );
                let left_is_mem = expr_accesses_memory(&left.item);
                let right_is_mem = expr_accesses_memory(&right.item);

                // Pattern: acc = acc OP mem_read  or  acc = mem_read OP acc
                if (left_is_same_var && right_is_mem) || (right_is_same_var && left_is_mem) {
                    return true;
                }
            }
        }
    }
    false
}

fn expr_accesses_memory(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Deref(_) | AstExpression::ArrayAccess(_, _) => true,
        AstExpression::Cast(_, inner) => expr_accesses_memory(&inner.item),
        AstExpression::BinaryOp(_, l, r) => {
            expr_accesses_memory(&l.item) || expr_accesses_memory(&r.item)
        }
        AstExpression::UnaryOp(_, arg) => expr_accesses_memory(&arg.item),
        _ => false,
    }
}

/// Collect all variable IDs that are written (assigned/declared) anywhere in a
/// statement list, recursing into nested control flow.
fn collect_written_variables(stmts: &[WrappedAstStatement], out: &mut HashSet<AstVariableId>) {
    for stmt in stmts {
        for (access, var_id) in stmt.statement.get_related_variables() {
            if access == AstVariableAccessType::Write {
                out.insert(var_id);
            }
        }
    }
}

/// Check whether an expression only reads variables that are NOT in `written`
/// and has no side effects (pure + no memory writes).
fn is_loop_invariant_expr(expr: &AstExpression, written: &HashSet<AstVariableId>) -> bool {
    use super::opt_utils::is_pure_expression;
    if !is_pure_expression(expr) {
        return false;
    }
    let mut read_vars = HashSet::new();
    super::opt_utils::collect_expr_variables(expr, &mut read_vars);
    read_vars.is_disjoint(written)
}

/// Annotate assignments inside loop bodies whose RHS is loop-invariant (all
/// referenced variables are never written inside the loop and the expression
/// has no side effects). This is annotation-only — no code motion is performed.
fn annotate_loop_invariants(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        annotate_loop_invariants_in_stmt(stmt);
    }
}

fn annotate_loop_invariants_in_stmt(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::While(_, body) => {
            annotate_loop_body_invariants(body);
            annotate_loop_invariants(body);
        }
        AstStatement::For(init, _, update, body) => {
            annotate_loop_invariants_in_stmt(init);
            annotate_loop_invariants_in_stmt(update);
            annotate_loop_body_invariants(body);
            annotate_loop_invariants(body);
        }
        AstStatement::Block(body) => {
            annotate_loop_invariants(body);
        }
        AstStatement::If(_, branch_true, branch_false) => {
            annotate_loop_invariants(branch_true);
            if let Some(branch_false) = branch_false {
                annotate_loop_invariants(branch_false);
            }
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases.iter_mut() {
                annotate_loop_invariants(case_body);
            }
            if let Some(default_body) = default {
                annotate_loop_invariants(default_body);
            }
        }
        _ => {}
    }
}

fn annotate_loop_body_invariants(body: &mut Vec<WrappedAstStatement>) {
    let mut written = HashSet::new();
    collect_written_variables(body, &mut written);

    if written.is_empty() {
        return;
    }

    let mut insertions: Vec<(usize, String)> = Vec::new();

    for (i, stmt) in body.iter().enumerate() {
        if let AstStatement::Assignment(_, rhs) = &stmt.statement {
            if is_loop_invariant_expr(&rhs.item, &written) {
                insertions.push((i, "// loop-invariant".to_string()));
            }
        }
        if let AstStatement::Declaration(_, Some(init)) = &stmt.statement {
            if is_loop_invariant_expr(&init.item, &written) {
                insertions.push((i, "// loop-invariant".to_string()));
            }
        }
    }

    for (idx, comment_text) in insertions.into_iter().rev() {
        body.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(comment_text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

// ---------------------------------------------------------------------------
// "this" pointer / SRet hidden parameter inference (L84/L86)
// ---------------------------------------------------------------------------

/// Detect if the first parameter is used primarily as a deref target (sret)
/// or as base+offset member access pattern ("this" pointer).
fn annotate_this_or_sret_pointer(
    body: &mut [WrappedAstStatement],
    first_param: AstVariableId,
    sret_layout_hint: Option<&str>,
) {
    let mut store_through_count = 0usize;
    let mut member_access_count = 0usize;
    let mut other_use_count = 0usize;
    classify_first_param_usage(
        body,
        first_param,
        &mut store_through_count,
        &mut member_access_count,
        &mut other_use_count,
    );

    let total = store_through_count + member_access_count + other_use_count;
    if total < 2 {
        return;
    }

    // SRet: primarily stored through (first param is a hidden return pointer)
    if store_through_count > 0 && store_through_count >= member_access_count + other_use_count {
        if let Some(first) = body.first_mut() {
            if first.comment.is_none() {
                let mut comment = "first parameter likely sret (hidden return pointer)".to_string();
                if let Some(layout_hint) = sret_layout_hint {
                    comment.push_str("; ");
                    comment.push_str(layout_hint);
                }
                first.comment = Some(comment);
            }
        }
        return;
    }

    // "this" pointer: primarily used as base+offset for member access
    if member_access_count >= 2 && member_access_count > store_through_count {
        // Refine: check for constructor/destructor patterns (L134)
        let label = if has_direct_deref_store(body, first_param) {
            "likely constructor ('this' pointer + vptr/field init)"
        } else if body_calls_free_or_delete(body) {
            "likely destructor ('this' pointer + free/delete)"
        } else {
            "first parameter likely 'this' pointer (member access pattern)"
        };
        if let Some(first) = body.first_mut() {
            if first.comment.is_none() {
                first.comment = Some(label.to_string());
            }
        }
    }
}

fn extract_param_register(param: &crate::abstract_syntax_tree::AstParameter) -> Option<Register> {
    let crate::abstract_syntax_tree::AstParameterLocation::Register(data) = &param.location else {
        return None;
    };

    match data.as_ref() {
        IrData::Register(register) => Some(register.clone()),
        _ => None,
    }
}

fn build_hidden_byref_parameter_hints(
    parameters: &[crate::abstract_syntax_tree::AstParameter],
    aggregates: Option<&[AggregateCandidate]>,
) -> Vec<String> {
    let Some(aggregates) = aggregates else {
        return Vec::new();
    };

    parameters
        .iter()
        .enumerate()
        .skip(1)
        .filter_map(|(param_index, param)| {
            let register = extract_param_register(param)?;
            let abi = classify_hidden_byref_abi(&register)?;
            let aggregate = aggregates
                .iter()
                .find(|aggregate| aggregate.base == register)?;

            let mut offsets: Vec<i64> = aggregate.fields.iter().map(|field| field.offset).collect();
            offsets.sort_unstable();
            offsets.dedup();
            if offsets.len() < 2 {
                return None;
            }

            let read_count = aggregate
                .fields
                .iter()
                .filter(|field| field.is_read)
                .count();
            let write_count = aggregate
                .fields
                .iter()
                .filter(|field| field.is_write)
                .count();
            if read_count < 2 || read_count <= write_count {
                return None;
            }

            let offsets = offsets
                .iter()
                .map(|offset| format!("0x{offset:x}"))
                .collect::<Vec<_>>()
                .join(", ");
            let layout_hint = if aggregate.likely_array {
                if let Some(stride) = aggregate.stride {
                    format!("array-like access stride 0x{stride:x}, offsets [{offsets}]")
                } else {
                    format!("array-like access offsets [{offsets}]")
                }
            } else {
                format!("read-dominant offsets [{offsets}]")
            };

            Some(format!(
                "parameter {} in {} {} likely hidden by-reference aggregate argument; {}",
                param_index + 1,
                abi,
                register.name(),
                layout_hint
            ))
        })
        .collect()
}

fn classify_hidden_byref_abi(register: &Register) -> Option<&'static str> {
    match register.name() {
        "rcx" | "rdx" | "r8" | "r9" => Some("Win64"),
        "x0" | "x1" | "x2" | "x3" | "x4" | "x5" | "x6" | "x7" => Some("AArch64"),
        _ => None,
    }
}

fn annotate_hidden_byref_parameters(body: &mut Vec<WrappedAstStatement>, hints: &[String]) {
    let Some((insert_at, _)) = next_non_comment_statement(body, 0) else {
        return;
    };

    let mut insertions = Vec::new();
    for hint in hints {
        let text = format!("// {hint}");
        if preceded_by_same_comment(body, insert_at + insertions.len(), &text) {
            continue;
        }
        insertions.push(text);
    }

    for (offset, text) in insertions.into_iter().enumerate() {
        body.insert(
            insert_at + offset,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn build_sret_layout_hint(
    aggregates: Option<&[AggregateCandidate]>,
    first_param_register: &Register,
) -> Option<String> {
    let aggregate = aggregates?
        .iter()
        .find(|aggregate| aggregate.base == *first_param_register)?;

    let mut write_offsets: Vec<i64> = aggregate
        .fields
        .iter()
        .filter(|field| field.is_write)
        .map(|field| field.offset)
        .collect();

    write_offsets.sort_unstable();
    write_offsets.dedup();

    if write_offsets.len() < 2 {
        return None;
    }

    let offsets = write_offsets
        .iter()
        .map(|offset| format!("0x{offset:x}"))
        .collect::<Vec<_>>()
        .join(", ");

    if aggregate.likely_array {
        if let Some(stride) = aggregate.stride {
            Some(format!(
                "layout hint: array-like return buffer stride 0x{stride:x}, write offsets [{offsets}]"
            ))
        } else {
            Some(format!(
                "layout hint: array-like return buffer write offsets [{offsets}]"
            ))
        }
    } else {
        Some(format!(
            "layout hint: written return-buffer offsets [{offsets}]"
        ))
    }
}

/// Check if there's a store to *param or param[0] (vptr initialization pattern).
fn has_direct_deref_store(stmts: &[WrappedAstStatement], var_id: AstVariableId) -> bool {
    for stmt in stmts {
        if let AstStatement::Assignment(lhs, _) = &stmt.statement {
            match &lhs.item {
                AstExpression::Deref(inner) => {
                    if matches!(&inner.item, AstExpression::Variable(_, id) if *id == var_id) {
                        return true;
                    }
                }
                AstExpression::ArrayAccess(base, idx) => {
                    if matches!(&base.item, AstExpression::Variable(_, id) if *id == var_id)
                        && matches!(
                            &idx.item,
                            AstExpression::Literal(AstLiteral::Int(0) | AstLiteral::UInt(0))
                        )
                    {
                        return true;
                    }
                }
                _ => {}
            }
        }
    }
    false
}

/// Check if the function body calls free/delete/operator delete (destructor indicator).
fn body_calls_free_or_delete(stmts: &[WrappedAstStatement]) -> bool {
    for stmt in stmts {
        match &stmt.statement {
            AstStatement::Call(call) => {
                let name = match call {
                    AstCall::Unknown(name, _) => name.as_str(),
                    _ => continue,
                };
                if matches!(
                    name,
                    "free"
                        | "_free"
                        | "operator delete"
                        | "operator delete[]"
                        | "_ZdlPv"
                        | "_ZdaPv"
                        | "HeapFree"
                        | "VirtualFree"
                        | "GlobalFree"
                        | "LocalFree"
                ) {
                    return true;
                }
            }
            AstStatement::If(_, bt, bf) => {
                if body_calls_free_or_delete(bt) {
                    return true;
                }
                if let Some(bf) = bf {
                    if body_calls_free_or_delete(bf) {
                        return true;
                    }
                }
            }
            AstStatement::While(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => {
                if body_calls_free_or_delete(body) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn classify_first_param_usage(
    stmts: &[WrappedAstStatement],
    var_id: AstVariableId,
    stores: &mut usize,
    member_reads: &mut usize,
    other: &mut usize,
) {
    for stmt in stmts {
        match &stmt.statement {
            AstStatement::Assignment(lhs, rhs) => {
                // Store through first param: *(param + offset) = value
                if is_deref_of_param(var_id, &lhs.item) {
                    *stores += 1;
                } else if expr_uses_var_as_base(var_id, &rhs.item) {
                    *member_reads += 1;
                } else if expr_mentions_var(var_id, &lhs.item)
                    || expr_mentions_var(var_id, &rhs.item)
                {
                    *other += 1;
                }
            }
            AstStatement::If(cond, bt, bf) => {
                if expr_mentions_var(var_id, &cond.item) {
                    *other += 1;
                }
                classify_first_param_usage(bt, var_id, stores, member_reads, other);
                if let Some(bf) = bf {
                    classify_first_param_usage(bf, var_id, stores, member_reads, other);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => {
                classify_first_param_usage(body, var_id, stores, member_reads, other);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    classify_first_param_usage(case_body, var_id, stores, member_reads, other);
                }
                if let Some(default_body) = default {
                    classify_first_param_usage(default_body, var_id, stores, member_reads, other);
                }
            }
            AstStatement::Return(Some(expr)) => {
                if expr_mentions_var(var_id, &expr.item) {
                    *other += 1;
                }
            }
            AstStatement::Call(call) => {
                if call_mentions_var(var_id, call) {
                    *other += 1;
                }
            }
            _ => {}
        }
    }
}

/// Check if expression is *(var + ...) or var[...]
fn is_deref_of_param(var_id: AstVariableId, expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Deref(inner) => expr_mentions_var(var_id, &inner.item),
        AstExpression::ArrayAccess(base, _) => {
            matches!(&base.item, AstExpression::Variable(_, id) if *id == var_id)
        }
        _ => false,
    }
}

/// Check if expression uses var as a base in deref/array context (member read)
fn expr_uses_var_as_base(var_id: AstVariableId, expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Deref(inner) => expr_mentions_var(var_id, &inner.item),
        AstExpression::ArrayAccess(base, _) => {
            matches!(&base.item, AstExpression::Variable(_, id) if *id == var_id)
        }
        AstExpression::BinaryOp(_, left, right) => {
            expr_uses_var_as_base(var_id, &left.item) || expr_uses_var_as_base(var_id, &right.item)
        }
        _ => false,
    }
}

fn expr_mentions_var(var_id: AstVariableId, expr: &AstExpression) -> bool {
    let mut vars = HashSet::new();
    super::opt_utils::collect_expr_variables(expr, &mut vars);
    vars.contains(&var_id)
}

fn call_mentions_var(var_id: AstVariableId, call: &AstCall) -> bool {
    match call {
        AstCall::Unknown(_, args) => args.iter().any(|a| expr_mentions_var(var_id, &a.item)),
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Obfuscation pattern detection (L154)
// ---------------------------------------------------------------------------

/// Detect signs of obfuscation: high goto density or excessive nesting.
fn annotate_obfuscation_indicators(body: &mut Vec<WrappedAstStatement>) {
    let mut goto_count = 0usize;
    let mut stmt_count = 0usize;
    let mut max_depth = 0usize;
    measure_complexity(body, 0, &mut goto_count, &mut stmt_count, &mut max_depth);

    if stmt_count < 10 {
        return;
    }

    let goto_ratio = goto_count as f64 / stmt_count as f64;
    // Heuristic: >30% goto density or nesting depth >10 suggests obfuscation/flattening
    if goto_ratio > 0.3 || max_depth > 10 {
        if let Some(first) = body.first_mut() {
            if first.comment.is_none() {
                first.comment = Some(format!(
                    "possible obfuscation ({} gotos / {} stmts, max depth {})",
                    goto_count, stmt_count, max_depth
                ));
            }
        }
    }
}

fn measure_complexity(
    stmts: &[WrappedAstStatement],
    depth: usize,
    gotos: &mut usize,
    total: &mut usize,
    max_depth: &mut usize,
) {
    if depth > *max_depth {
        *max_depth = depth;
    }
    for stmt in stmts {
        *total += 1;
        match &stmt.statement {
            AstStatement::Goto(_) => {
                *gotos += 1;
            }
            AstStatement::If(_, bt, bf) => {
                measure_complexity(bt, depth + 1, gotos, total, max_depth);
                if let Some(bf) = bf {
                    measure_complexity(bf, depth + 1, gotos, total, max_depth);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => {
                measure_complexity(body, depth + 1, gotos, total, max_depth);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    measure_complexity(case_body, depth + 1, gotos, total, max_depth);
                }
                if let Some(default_body) = default {
                    measure_complexity(default_body, depth + 1, gotos, total, max_depth);
                }
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// "ptr+len" pairing detection (L242)
// ---------------------------------------------------------------------------

/// Detect consecutive parameter pairs where one is used in deref contexts (pointer)
/// and the next is used in comparison/loop-bound contexts (length).
fn annotate_ptr_len_pairs(body: &mut Vec<WrappedAstStatement>, params: &[AstVariableId]) {
    if params.len() < 2 {
        return;
    }

    for i in 0..params.len() - 1 {
        let ptr_var = params[i];
        let len_var = params[i + 1];

        let ptr_deref = count_deref_uses(body, ptr_var);
        let len_compare = count_compare_or_bound_uses(body, len_var);

        // Heuristic: ptr used in 1+ deref, len used in 1+ comparison → likely ptr+len pair
        if ptr_deref >= 1 && len_compare >= 1 {
            let note = format!(
                "likely (ptr, len) parameter pair at positions {}, {}",
                i,
                i + 1
            );
            // Check we haven't already inserted this note
            let already = body.iter().any(
                |s| matches!(&s.statement, AstStatement::Comment(c) if c.contains("ptr, len")),
            );
            if !already {
                body.insert(
                    0,
                    WrappedAstStatement {
                        statement: AstStatement::Comment(note),
                        origin: AstStatementOrigin::Unknown,
                        comment: None,
                    },
                );
            }
        }
    }
}

fn annotate_struct_field_ptr_len_pairs(body: &mut Vec<WrappedAstStatement>) {
    for stmt in body.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                annotate_struct_field_ptr_len_pairs(bt);
                if let Some(bf) = bf {
                    annotate_struct_field_ptr_len_pairs(bf);
                }
            }
            AstStatement::While(_, nested)
            | AstStatement::DoWhile(_, nested)
            | AstStatement::For(_, _, _, nested)
            | AstStatement::Block(nested) => annotate_struct_field_ptr_len_pairs(nested),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_struct_field_ptr_len_pairs(case_body);
                }
                if let Some(default) = default {
                    annotate_struct_field_ptr_len_pairs(default);
                }
            }
            _ => {}
        }
    }

    let Some((buf_path, len_path)) = detect_struct_field_ptr_len_pair(body) else {
        return;
    };

    let already = body.iter().any(
        |stmt| matches!(&stmt.statement, AstStatement::Comment(c) if c.contains("buf, len/cap) field pair")),
    );
    if already {
        return;
    }

    let note = format!(
        "likely (buf, len/cap) field pair: {} + {}",
        describe_member_field_path(&buf_path),
        describe_member_field_path(&len_path)
    );
    body.insert(
        0,
        WrappedAstStatement {
            statement: AstStatement::Comment(note),
            origin: AstStatementOrigin::Unknown,
            comment: None,
        },
    );
}

fn detect_struct_field_ptr_len_pair(
    stmts: &[WrappedAstStatement],
) -> Option<(MemberFieldPath, MemberFieldPath)> {
    let mut candidates = Vec::new();
    for stmt in stmts {
        collect_member_field_paths_from_statement(&stmt.statement, &mut candidates);
    }

    for buf_path in candidates.iter() {
        let Some(buf_field) = buf_path.fields.last() else {
            continue;
        };
        if !is_buffer_like_field_name(buf_field)
            || count_member_field_buffer_uses(stmts, buf_path) == 0
        {
            continue;
        }

        for len_path in candidates.iter() {
            if buf_path == len_path {
                continue;
            }

            let Some(len_field) = len_path.fields.last() else {
                continue;
            };
            if !is_length_like_field_name(len_field)
                || !member_field_paths_share_object_prefix(buf_path, len_path)
                || count_member_field_bound_uses(stmts, len_path) == 0
            {
                continue;
            }

            return Some((buf_path.clone(), len_path.clone()));
        }
    }

    None
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum OwnershipTarget {
    Variable(AstVariableId),
    Field(MemberFieldPath),
}

fn annotate_post_call_borrow_patterns(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, branch_true, branch_false) => {
                annotate_post_call_borrow_patterns(branch_true);
                if let Some(branch_false) = branch_false {
                    annotate_post_call_borrow_patterns(branch_false);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => annotate_post_call_borrow_patterns(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_post_call_borrow_patterns(case_body);
                }
                if let Some(default) = default {
                    annotate_post_call_borrow_patterns(default);
                }
            }
            _ => {}
        }
    }

    let mut insertions = Vec::new();
    for i in 0..stmts.len() {
        let Some((next_idx, cleanup_stmt)) = next_non_comment_statement(stmts, i + 1) else {
            continue;
        };
        let AstStatement::Call(call) = &stmts[i].statement else {
            continue;
        };
        if let Some(comment) = detect_post_call_borrow_comment(call, cleanup_stmt) {
            if !preceded_by_same_comment(stmts, i, &comment) {
                insertions.push((i, comment));
            }
            if next_idx > i + 1 {
                continue;
            }
        }
    }

    for (idx, text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn annotate_heap_metadata_patterns(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, branch_true, branch_false) => {
                annotate_heap_metadata_patterns(branch_true);
                if let Some(branch_false) = branch_false {
                    annotate_heap_metadata_patterns(branch_false);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => annotate_heap_metadata_patterns(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_heap_metadata_patterns(case_body);
                }
                if let Some(default) = default {
                    annotate_heap_metadata_patterns(default);
                }
            }
            _ => {}
        }
    }

    if stmts.iter().any(|stmt| {
        matches!(
            &stmt.statement,
            AstStatement::Comment(text)
                if text.contains("allocator bookkeeping / heap metadata access")
        )
    }) {
        return;
    }

    if !body_has_heap_allocator_activity(stmts) {
        return;
    }

    let Some(insert_idx) = stmts.iter().enumerate().find_map(|(idx, stmt)| {
        if stmt.comment.is_some() {
            return None;
        }
        statement_looks_like_heap_metadata_access(&stmt.statement).then_some(idx)
    }) else {
        return;
    };

    stmts.insert(
        insert_idx,
        WrappedAstStatement {
            statement: AstStatement::Comment(
                "// likely allocator bookkeeping / heap metadata access; avoid typing nearby offsets as user fields"
                    .to_string(),
            ),
            origin: AstStatementOrigin::Unknown,
            comment: None,
        },
    );
}

fn body_has_heap_allocator_activity(stmts: &[WrappedAstStatement]) -> bool {
    stmts
        .iter()
        .any(|stmt| statement_contains_allocator_activity(&stmt.statement))
}

fn statement_contains_allocator_activity(stmt: &AstStatement) -> bool {
    match stmt {
        AstStatement::Declaration(_, Some(init)) => expr_contains_allocator_activity(&init.item),
        AstStatement::Assignment(lhs, rhs) => {
            expr_contains_allocator_activity(&lhs.item)
                || expr_contains_allocator_activity(&rhs.item)
        }
        AstStatement::If(cond, _, _) => expr_contains_allocator_activity(&cond.item),
        AstStatement::While(cond, _) | AstStatement::DoWhile(cond, _) => {
            expr_contains_allocator_activity(&cond.item)
        }
        AstStatement::For(init, cond, step, _) => {
            statement_contains_allocator_activity(&init.statement)
                || expr_contains_allocator_activity(&cond.item)
                || statement_contains_allocator_activity(&step.statement)
        }
        AstStatement::Return(Some(expr)) => expr_contains_allocator_activity(&expr.item),
        AstStatement::Call(call) => call_contains_allocator_activity(call),
        AstStatement::Switch(expr, _, _) => expr_contains_allocator_activity(&expr.item),
        _ => false,
    }
}

fn expr_contains_allocator_activity(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => expr_contains_allocator_activity(&inner.item),
        AstExpression::BinaryOp(_, left, right) | AstExpression::ArrayAccess(left, right) => {
            expr_contains_allocator_activity(&left.item)
                || expr_contains_allocator_activity(&right.item)
        }
        AstExpression::Call(call) => call_contains_allocator_activity(call),
        AstExpression::Ternary(cond, t, f) => {
            expr_contains_allocator_activity(&cond.item)
                || expr_contains_allocator_activity(&t.item)
                || expr_contains_allocator_activity(&f.item)
        }
        _ => false,
    }
}

fn call_contains_allocator_activity(call: &AstCall) -> bool {
    if call_name_matches_alloc(call).is_some() {
        return true;
    }

    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => args
            .iter()
            .any(|arg| expr_contains_allocator_activity(&arg.item)),
        AstCall::Builtin(_, arg) => builtin_argument_contains_allocator_activity(arg),
    }
}

fn builtin_argument_contains_allocator_activity(arg: &AstBuiltinFunctionArgument) -> bool {
    match arg {
        AstBuiltinFunctionArgument::None => false,
        AstBuiltinFunctionArgument::Print(args) => args
            .iter()
            .any(|arg| expr_contains_allocator_activity(&arg.item)),
        AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | AstBuiltinFunctionArgument::BitSizeOf(expr)
        | AstBuiltinFunctionArgument::OperandExists(expr)
        | AstBuiltinFunctionArgument::SignedMax(expr)
        | AstBuiltinFunctionArgument::SignedMin(expr)
        | AstBuiltinFunctionArgument::UnsignedMax(expr)
        | AstBuiltinFunctionArgument::UnsignedMin(expr)
        | AstBuiltinFunctionArgument::BitOnes(expr)
        | AstBuiltinFunctionArgument::BitZeros(expr) => {
            expr_contains_allocator_activity(&expr.item)
        }
        AstBuiltinFunctionArgument::Sized(left, right) => {
            expr_contains_allocator_activity(&left.item)
                || expr_contains_allocator_activity(&right.item)
        }
    }
}

fn statement_looks_like_heap_metadata_access(stmt: &AstStatement) -> bool {
    match stmt {
        AstStatement::Declaration(_, Some(init)) => {
            expr_looks_like_heap_metadata_access(&init.item)
        }
        AstStatement::Assignment(lhs, rhs) => {
            expr_looks_like_heap_metadata_access(&lhs.item)
                || expr_looks_like_heap_metadata_access(&rhs.item)
        }
        AstStatement::If(cond, _, _) => expr_looks_like_heap_metadata_access(&cond.item),
        AstStatement::While(cond, _) | AstStatement::DoWhile(cond, _) => {
            expr_looks_like_heap_metadata_access(&cond.item)
        }
        AstStatement::For(init, cond, step, _) => {
            statement_looks_like_heap_metadata_access(&init.statement)
                || expr_looks_like_heap_metadata_access(&cond.item)
                || statement_looks_like_heap_metadata_access(&step.statement)
        }
        AstStatement::Return(Some(expr)) => expr_looks_like_heap_metadata_access(&expr.item),
        AstStatement::Call(call) => call_looks_like_heap_metadata_access(call),
        AstStatement::Switch(expr, _, _) => expr_looks_like_heap_metadata_access(&expr.item),
        _ => false,
    }
}

fn expr_looks_like_heap_metadata_access(expr: &AstExpression) -> bool {
    if extract_member_field_path(expr)
        .map(|path| {
            path.fields
                .iter()
                .any(|field| is_heap_metadata_field_name(field))
        })
        .unwrap_or(false)
    {
        return true;
    }

    match expr {
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::AddressOf(inner) => expr_looks_like_heap_metadata_access(&inner.item),
        AstExpression::Deref(inner) => {
            expr_is_heap_metadata_offset_access(&inner.item)
                || expr_looks_like_heap_metadata_access(&inner.item)
        }
        AstExpression::MemberAccess(base, field) => {
            is_heap_metadata_field_name(field) || expr_looks_like_heap_metadata_access(&base.item)
        }
        AstExpression::BinaryOp(_, left, right) => {
            expr_is_heap_metadata_offset_access(expr)
                || expr_looks_like_heap_metadata_access(&left.item)
                || expr_looks_like_heap_metadata_access(&right.item)
        }
        AstExpression::ArrayAccess(base, index) => {
            expr_is_heap_metadata_offset_access(&base.item)
                || expr_looks_like_heap_metadata_access(&base.item)
                || expr_looks_like_heap_metadata_access(&index.item)
        }
        AstExpression::Call(call) => call_looks_like_heap_metadata_access(call),
        AstExpression::Ternary(cond, t, f) => {
            expr_looks_like_heap_metadata_access(&cond.item)
                || expr_looks_like_heap_metadata_access(&t.item)
                || expr_looks_like_heap_metadata_access(&f.item)
        }
        _ => false,
    }
}

fn call_looks_like_heap_metadata_access(call: &AstCall) -> bool {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => args
            .iter()
            .any(|arg| expr_looks_like_heap_metadata_access(&arg.item)),
        AstCall::Builtin(_, arg) => builtin_argument_looks_like_heap_metadata_access(arg),
    }
}

fn builtin_argument_looks_like_heap_metadata_access(arg: &AstBuiltinFunctionArgument) -> bool {
    match arg {
        AstBuiltinFunctionArgument::None => false,
        AstBuiltinFunctionArgument::Print(args) => args
            .iter()
            .any(|arg| expr_looks_like_heap_metadata_access(&arg.item)),
        AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | AstBuiltinFunctionArgument::BitSizeOf(expr)
        | AstBuiltinFunctionArgument::OperandExists(expr)
        | AstBuiltinFunctionArgument::SignedMax(expr)
        | AstBuiltinFunctionArgument::SignedMin(expr)
        | AstBuiltinFunctionArgument::UnsignedMax(expr)
        | AstBuiltinFunctionArgument::UnsignedMin(expr)
        | AstBuiltinFunctionArgument::BitOnes(expr)
        | AstBuiltinFunctionArgument::BitZeros(expr) => {
            expr_looks_like_heap_metadata_access(&expr.item)
        }
        AstBuiltinFunctionArgument::Sized(left, right) => {
            expr_looks_like_heap_metadata_access(&left.item)
                || expr_looks_like_heap_metadata_access(&right.item)
        }
    }
}

fn expr_is_heap_metadata_offset_access(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::BinaryOp(AstBinaryOperator::Add | AstBinaryOperator::Sub, left, right) => {
            (expr_is_heap_metadata_offset_literal(&left.item)
                && expr_could_be_pointer_base(&right.item))
                || (expr_is_heap_metadata_offset_literal(&right.item)
                    && expr_could_be_pointer_base(&left.item))
                || expr_is_heap_metadata_offset_access(&left.item)
                || expr_is_heap_metadata_offset_access(&right.item)
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => expr_is_heap_metadata_offset_access(&inner.item),
        AstExpression::BinaryOp(_, left, right) | AstExpression::ArrayAccess(left, right) => {
            expr_is_heap_metadata_offset_access(&left.item)
                || expr_is_heap_metadata_offset_access(&right.item)
        }
        AstExpression::Call(call) => call_looks_like_heap_metadata_access(call),
        AstExpression::Ternary(cond, t, f) => {
            expr_is_heap_metadata_offset_access(&cond.item)
                || expr_is_heap_metadata_offset_access(&t.item)
                || expr_is_heap_metadata_offset_access(&f.item)
        }
        _ => false,
    }
}

fn expr_is_heap_metadata_offset_literal(expr: &AstExpression) -> bool {
    matches!(
        expr,
        AstExpression::Literal(AstLiteral::Int(8 | 16 | 24 | 32))
            | AstExpression::Literal(AstLiteral::UInt(8 | 16 | 24 | 32))
    )
}

fn expr_could_be_pointer_base(expr: &AstExpression) -> bool {
    !matches!(
        expr,
        AstExpression::Literal(AstLiteral::Int(_))
            | AstExpression::Literal(AstLiteral::UInt(_))
            | AstExpression::Literal(AstLiteral::Float(_))
            | AstExpression::Literal(AstLiteral::Bool(_))
            | AstExpression::Literal(AstLiteral::Char(_))
            | AstExpression::Literal(AstLiteral::String(_))
    )
}

fn is_heap_metadata_field_name(field: &str) -> bool {
    let lower = field.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "fd" | "bk" | "flink" | "blink" | "arena" | "chunk" | "header" | "footer"
    ) || lower == "prev_size"
        || lower == "next_free"
        || lower == "prev_free"
        || lower == "nextchunk"
        || lower == "prevchunk"
        || lower.ends_with("_chunk")
        || lower.ends_with("_header")
        || lower.ends_with("_footer")
        || lower.contains("heap_header")
        || lower.contains("chunk_size")
}

fn detect_post_call_borrow_comment(call: &AstCall, cleanup_stmt: &AstStatement) -> Option<String> {
    let cleanup_target = extract_cleanup_target(cleanup_stmt)?;
    let call_name = describe_call_for_ownership(call);
    let is_cleanup_like_call = matches!(
        call_name.as_deref(),
        Some(name) if is_cleanup_like_name(name)
    );
    if is_cleanup_like_call {
        return None;
    }

    let arg_targets = collect_call_argument_targets(call);
    if !arg_targets.contains(&cleanup_target) {
        return None;
    }

    let target_desc = describe_ownership_target(&cleanup_target);
    Some(match call_name {
        Some(name) => format!(
            "// likely borrow-only call: caller releases {} after {}",
            target_desc, name
        ),
        None => format!(
            "// likely borrow-only call: caller releases {} after call",
            target_desc
        ),
    })
}

fn next_non_comment_statement(
    stmts: &[WrappedAstStatement],
    start: usize,
) -> Option<(usize, &AstStatement)> {
    for (idx, stmt) in stmts.iter().enumerate().skip(start) {
        if matches!(stmt.statement, AstStatement::Comment(_)) {
            continue;
        }
        return Some((idx, &stmt.statement));
    }
    None
}

fn preceded_by_same_comment(stmts: &[WrappedAstStatement], idx: usize, text: &str) -> bool {
    idx.checked_sub(1)
        .and_then(|prev| stmts.get(prev))
        .is_some_and(
            |stmt| matches!(&stmt.statement, AstStatement::Comment(existing) if existing == text),
        )
}

fn extract_cleanup_target(stmt: &AstStatement) -> Option<OwnershipTarget> {
    let AstStatement::Call(AstCall::Unknown(name, args)) = stmt else {
        return None;
    };
    if !is_cleanup_like_name(name) {
        return None;
    }
    args.first()
        .and_then(|arg| extract_ownership_target_from_expr(&arg.item))
}

fn is_cleanup_like_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower == "free"
        || lower.contains("close")
        || lower.contains("release")
        || lower.contains("destroy")
        || lower.contains("delete")
        || lower.contains("unref")
        || lower.contains("dealloc")
        || lower.contains("munmap")
        || lower.contains("freeaddrinfo")
        || lower.contains("closehandle")
        || lower.contains("regclosekey")
}

fn describe_call_for_ownership(call: &AstCall) -> Option<String> {
    match call {
        AstCall::Unknown(name, _) => Some(format!("\"{}\"", name)),
        _ => None,
    }
}

fn collect_call_argument_targets(call: &AstCall) -> Vec<OwnershipTarget> {
    let mut targets = Vec::new();
    let args = match call {
        AstCall::Unknown(_, args) => Some(args.as_slice()),
        AstCall::Variable { args, .. } | AstCall::Function { args, .. } => Some(args.as_slice()),
        AstCall::Builtin(_, _) => None,
    };
    if let Some(args) = args {
        for arg in args {
            if let Some(target) = extract_ownership_target_from_expr(&arg.item) {
                if !targets.contains(&target) {
                    targets.push(target);
                }
            }
        }
    }
    targets
}

fn extract_ownership_target_from_expr(expr: &AstExpression) -> Option<OwnershipTarget> {
    match expr {
        AstExpression::Variable(_, id) => Some(OwnershipTarget::Variable(*id)),
        AstExpression::AddressOf(inner)
        | AstExpression::Deref(inner)
        | AstExpression::Cast(_, inner) => extract_ownership_target_from_expr(&inner.item),
        _ => extract_member_field_path(expr)
            .filter(|path| !path.fields.is_empty())
            .map(OwnershipTarget::Field),
    }
}

fn describe_ownership_target(target: &OwnershipTarget) -> String {
    match target {
        OwnershipTarget::Variable(_) => "argument".to_string(),
        OwnershipTarget::Field(path) => format!("resource {}", describe_member_field_path(path)),
    }
}

fn collect_member_field_paths_from_statement(stmt: &AstStatement, out: &mut Vec<MemberFieldPath>) {
    match stmt {
        AstStatement::Declaration(_, Some(rhs)) => {
            collect_member_field_paths_from_expr(&rhs.item, out)
        }
        AstStatement::Assignment(lhs, rhs) => {
            collect_member_field_paths_from_expr(&lhs.item, out);
            collect_member_field_paths_from_expr(&rhs.item, out);
        }
        AstStatement::Call(call) => collect_member_field_paths_from_call(call, out),
        AstStatement::Return(Some(expr)) => collect_member_field_paths_from_expr(&expr.item, out),
        AstStatement::If(cond, bt, bf) => {
            collect_member_field_paths_from_expr(&cond.item, out);
            for stmt in bt {
                collect_member_field_paths_from_statement(&stmt.statement, out);
            }
            if let Some(bf) = bf {
                for stmt in bf {
                    collect_member_field_paths_from_statement(&stmt.statement, out);
                }
            }
        }
        AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
            collect_member_field_paths_from_expr(&cond.item, out);
            for stmt in body {
                collect_member_field_paths_from_statement(&stmt.statement, out);
            }
        }
        AstStatement::For(init, cond, step, body) => {
            collect_member_field_paths_from_statement(&init.statement, out);
            collect_member_field_paths_from_expr(&cond.item, out);
            collect_member_field_paths_from_statement(&step.statement, out);
            for stmt in body {
                collect_member_field_paths_from_statement(&stmt.statement, out);
            }
        }
        AstStatement::Switch(disc, cases, default) => {
            collect_member_field_paths_from_expr(&disc.item, out);
            for (_, case_body) in cases {
                for stmt in case_body {
                    collect_member_field_paths_from_statement(&stmt.statement, out);
                }
            }
            if let Some(default) = default {
                for stmt in default {
                    collect_member_field_paths_from_statement(&stmt.statement, out);
                }
            }
        }
        AstStatement::Block(body) => {
            for stmt in body {
                collect_member_field_paths_from_statement(&stmt.statement, out);
            }
        }
        _ => {}
    }
}

fn collect_member_field_paths_from_call(call: &AstCall, out: &mut Vec<MemberFieldPath>) {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => {
            for arg in args {
                collect_member_field_paths_from_expr(&arg.item, out);
            }
        }
        AstCall::Builtin(_, builtin) => {
            collect_member_field_paths_from_builtin_arg(builtin.as_ref(), out)
        }
    }
}

fn collect_member_field_paths_from_builtin_arg(
    builtin: &crate::abstract_syntax_tree::AstBuiltinFunctionArgument,
    out: &mut Vec<MemberFieldPath>,
) {
    match builtin {
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Print(args) => {
            for arg in args {
                collect_member_field_paths_from_expr(&arg.item, out);
            }
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
            collect_member_field_paths_from_expr(&expr1.item, out);
            collect_member_field_paths_from_expr(&expr2.item, out);
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::OperandExists(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitOnes(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitZeros(expr) => {
            collect_member_field_paths_from_expr(&expr.item, out)
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::None => {}
    }
}

fn collect_member_field_paths_from_expr(expr: &AstExpression, out: &mut Vec<MemberFieldPath>) {
    if let Some(path) = extract_member_field_path(expr) {
        if !path.fields.is_empty() && !out.contains(&path) {
            out.push(path);
        }
    }

    match expr {
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => {
            collect_member_field_paths_from_expr(&inner.item, out)
        }
        AstExpression::BinaryOp(_, left, right) | AstExpression::ArrayAccess(left, right) => {
            collect_member_field_paths_from_expr(&left.item, out);
            collect_member_field_paths_from_expr(&right.item, out);
        }
        AstExpression::Call(call) => collect_member_field_paths_from_call(call, out),
        AstExpression::Ternary(cond, t, f) => {
            collect_member_field_paths_from_expr(&cond.item, out);
            collect_member_field_paths_from_expr(&t.item, out);
            collect_member_field_paths_from_expr(&f.item, out);
        }
        _ => {}
    }
}

fn is_buffer_like_field_name(field: &str) -> bool {
    field == "buf"
        || field.ends_with("_buf")
        || field.contains("buffer")
        || field == "data"
        || field.ends_with("_data")
        || field.contains("bytes")
        || field == "ptr"
        || field.ends_with("_ptr")
        || field.contains("pointer")
        || field == "str"
        || field.ends_with("_str")
        || field.contains("string")
        || field.contains("payload")
}

fn is_length_like_field_name(field: &str) -> bool {
    field == "len"
        || field.ends_with("_len")
        || field.contains("length")
        || field == "size"
        || field.ends_with("_size")
        || field == "cap"
        || field.ends_with("_cap")
        || field.contains("capacity")
        || field.contains("nbytes")
        || field.contains("byte_len")
        || field.contains("byte_count")
}

fn member_field_paths_share_object_prefix(a: &MemberFieldPath, b: &MemberFieldPath) -> bool {
    a.root_var == b.root_var
        && a.fields.len() >= 1
        && b.fields.len() >= 1
        && a.fields[..a.fields.len() - 1] == b.fields[..b.fields.len() - 1]
}

fn describe_member_field_path(path: &MemberFieldPath) -> String {
    path.fields.join(".")
}

fn count_member_field_buffer_uses(stmts: &[WrappedAstStatement], path: &MemberFieldPath) -> usize {
    let mut count = 0;
    for stmt in stmts {
        match &stmt.statement {
            AstStatement::Declaration(_, Some(rhs)) => {
                if expr_uses_member_field_as_buffer_base(&rhs.item, path) {
                    count += 1;
                }
            }
            AstStatement::Assignment(lhs, rhs) => {
                if expr_uses_member_field_as_buffer_base(&lhs.item, path)
                    || expr_uses_member_field_as_buffer_base(&rhs.item, path)
                {
                    count += 1;
                }
            }
            AstStatement::Call(call) => {
                if call_uses_member_field_as_buffer_base(call, path) {
                    count += 1;
                }
            }
            AstStatement::Return(Some(expr)) => {
                if expr_uses_member_field_as_buffer_base(&expr.item, path) {
                    count += 1;
                }
            }
            AstStatement::If(cond, bt, bf) => {
                if expr_uses_member_field_as_buffer_base(&cond.item, path) {
                    count += 1;
                }
                count += count_member_field_buffer_uses(bt, path);
                if let Some(bf) = bf {
                    count += count_member_field_buffer_uses(bf, path);
                }
            }
            AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
                if expr_uses_member_field_as_buffer_base(&cond.item, path) {
                    count += 1;
                }
                count += count_member_field_buffer_uses(body, path);
            }
            AstStatement::For(init, cond, step, body) => {
                count += count_member_field_buffer_uses(
                    &[WrappedAstStatement {
                        statement: init.statement.clone(),
                        origin: AstStatementOrigin::Unknown,
                        comment: None,
                    }],
                    path,
                );
                if expr_uses_member_field_as_buffer_base(&cond.item, path) {
                    count += 1;
                }
                count += count_member_field_buffer_uses(
                    &[WrappedAstStatement {
                        statement: step.statement.clone(),
                        origin: AstStatementOrigin::Unknown,
                        comment: None,
                    }],
                    path,
                );
                count += count_member_field_buffer_uses(body, path);
            }
            AstStatement::Block(body) => count += count_member_field_buffer_uses(body, path),
            AstStatement::Switch(disc, cases, default) => {
                if expr_uses_member_field_as_buffer_base(&disc.item, path) {
                    count += 1;
                }
                for (_, case_body) in cases {
                    count += count_member_field_buffer_uses(case_body, path);
                }
                if let Some(default) = default {
                    count += count_member_field_buffer_uses(default, path);
                }
            }
            _ => {}
        }
    }
    count
}

fn count_member_field_bound_uses(stmts: &[WrappedAstStatement], path: &MemberFieldPath) -> usize {
    let mut count = 0;
    for stmt in stmts {
        match &stmt.statement {
            AstStatement::If(cond, bt, bf) => {
                if is_comparison_involving_member_field(path, &cond.item) {
                    count += 1;
                }
                count += count_member_field_bound_uses(bt, path);
                if let Some(bf) = bf {
                    count += count_member_field_bound_uses(bf, path);
                }
            }
            AstStatement::While(cond, body) | AstStatement::DoWhile(cond, body) => {
                if is_comparison_involving_member_field(path, &cond.item) {
                    count += 1;
                }
                count += count_member_field_bound_uses(body, path);
            }
            AstStatement::For(_, cond, _, body) => {
                if is_comparison_involving_member_field(path, &cond.item) {
                    count += 1;
                }
                count += count_member_field_bound_uses(body, path);
            }
            AstStatement::Block(body) => count += count_member_field_bound_uses(body, path),
            AstStatement::Switch(disc, cases, default) => {
                if is_comparison_involving_member_field(path, &disc.item) {
                    count += 1;
                }
                for (_, case_body) in cases {
                    count += count_member_field_bound_uses(case_body, path);
                }
                if let Some(default) = default {
                    count += count_member_field_bound_uses(default, path);
                }
            }
            _ => {}
        }
    }
    count
}

fn expr_uses_member_field_as_buffer_base(expr: &AstExpression, path: &MemberFieldPath) -> bool {
    match expr {
        AstExpression::Deref(inner) => {
            expr_matches_member_field_path(&inner.item, path)
                || expr_uses_member_field_as_buffer_base(&inner.item, path)
        }
        AstExpression::ArrayAccess(base, index) => {
            expr_matches_member_field_path(&base.item, path)
                || expr_uses_member_field_as_buffer_base(&base.item, path)
                || expr_uses_member_field_as_buffer_base(&index.item, path)
        }
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => {
            expr_uses_member_field_as_buffer_base(&inner.item, path)
        }
        AstExpression::BinaryOp(_, left, right) => {
            expr_uses_member_field_as_buffer_base(&left.item, path)
                || expr_uses_member_field_as_buffer_base(&right.item, path)
        }
        AstExpression::Call(call) => call_uses_member_field_as_buffer_base(call, path),
        AstExpression::Ternary(cond, t, f) => {
            expr_uses_member_field_as_buffer_base(&cond.item, path)
                || expr_uses_member_field_as_buffer_base(&t.item, path)
                || expr_uses_member_field_as_buffer_base(&f.item, path)
        }
        _ => false,
    }
}

fn call_uses_member_field_as_buffer_base(call: &AstCall, path: &MemberFieldPath) -> bool {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => args
            .iter()
            .any(|arg| expr_uses_member_field_as_buffer_base(&arg.item, path)),
        AstCall::Builtin(_, builtin) => {
            builtin_arg_uses_member_field_as_buffer_base(builtin.as_ref(), path)
        }
    }
}

fn builtin_arg_uses_member_field_as_buffer_base(
    builtin: &crate::abstract_syntax_tree::AstBuiltinFunctionArgument,
    path: &MemberFieldPath,
) -> bool {
    match builtin {
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Print(args) => args
            .iter()
            .any(|arg| expr_uses_member_field_as_buffer_base(&arg.item, path)),
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
            expr_uses_member_field_as_buffer_base(&expr1.item, path)
                || expr_uses_member_field_as_buffer_base(&expr2.item, path)
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::OperandExists(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitOnes(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitZeros(expr) => {
            expr_uses_member_field_as_buffer_base(&expr.item, path)
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::None => false,
    }
}

fn is_comparison_involving_member_field(path: &MemberFieldPath, expr: &AstExpression) -> bool {
    if let AstExpression::BinaryOp(op, left, right) = expr {
        if matches!(
            op,
            AstBinaryOperator::Less
                | AstBinaryOperator::LessEqual
                | AstBinaryOperator::Greater
                | AstBinaryOperator::GreaterEqual
                | AstBinaryOperator::Equal
                | AstBinaryOperator::NotEqual
        ) {
            return expr_mentions_member_field_path(&left.item, path)
                || expr_mentions_member_field_path(&right.item, path);
        }
    }
    false
}

fn expr_mentions_member_field_path(expr: &AstExpression, path: &MemberFieldPath) -> bool {
    if expr_matches_member_field_path(expr, path) {
        return true;
    }

    match expr {
        AstExpression::UnaryOp(_, inner)
        | AstExpression::Cast(_, inner)
        | AstExpression::Deref(inner)
        | AstExpression::AddressOf(inner)
        | AstExpression::MemberAccess(inner, _) => {
            expr_mentions_member_field_path(&inner.item, path)
        }
        AstExpression::BinaryOp(_, left, right) | AstExpression::ArrayAccess(left, right) => {
            expr_mentions_member_field_path(&left.item, path)
                || expr_mentions_member_field_path(&right.item, path)
        }
        AstExpression::Call(call) => call_mentions_member_field_path(call, path),
        AstExpression::Ternary(cond, t, f) => {
            expr_mentions_member_field_path(&cond.item, path)
                || expr_mentions_member_field_path(&t.item, path)
                || expr_mentions_member_field_path(&f.item, path)
        }
        _ => false,
    }
}

fn call_mentions_member_field_path(call: &AstCall, path: &MemberFieldPath) -> bool {
    match call {
        AstCall::Variable { args, .. }
        | AstCall::Function { args, .. }
        | AstCall::Unknown(_, args) => args
            .iter()
            .any(|arg| expr_mentions_member_field_path(&arg.item, path)),
        AstCall::Builtin(_, builtin) => {
            builtin_arg_mentions_member_field_path(builtin.as_ref(), path)
        }
    }
}

fn builtin_arg_mentions_member_field_path(
    builtin: &crate::abstract_syntax_tree::AstBuiltinFunctionArgument,
    path: &MemberFieldPath,
) -> bool {
    match builtin {
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Print(args) => args
            .iter()
            .any(|arg| expr_mentions_member_field_path(&arg.item, path)),
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::Sized(expr1, expr2) => {
            expr_mentions_member_field_path(&expr1.item, path)
                || expr_mentions_member_field_path(&expr2.item, path)
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::ByteSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitSizeOf(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::OperandExists(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::SignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMax(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::UnsignedMin(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitOnes(expr)
        | crate::abstract_syntax_tree::AstBuiltinFunctionArgument::BitZeros(expr) => {
            expr_mentions_member_field_path(&expr.item, path)
        }
        crate::abstract_syntax_tree::AstBuiltinFunctionArgument::None => false,
    }
}

fn count_deref_uses(stmts: &[WrappedAstStatement], var_id: AstVariableId) -> usize {
    let mut count = 0;
    for stmt in stmts {
        match &stmt.statement {
            AstStatement::Assignment(lhs, rhs) => {
                if is_deref_of_param(var_id, &lhs.item) || is_deref_of_param(var_id, &rhs.item) {
                    count += 1;
                }
            }
            AstStatement::If(_, bt, bf) => {
                count += count_deref_uses(bt, var_id);
                if let Some(bf) = bf {
                    count += count_deref_uses(bf, var_id);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::For(_, _, _, body)
            | AstStatement::Block(body) => {
                count += count_deref_uses(body, var_id);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    count += count_deref_uses(case_body, var_id);
                }
                if let Some(default_body) = default {
                    count += count_deref_uses(default_body, var_id);
                }
            }
            _ => {}
        }
    }
    count
}

fn count_compare_or_bound_uses(stmts: &[WrappedAstStatement], var_id: AstVariableId) -> usize {
    let mut count = 0;
    for stmt in stmts {
        match &stmt.statement {
            AstStatement::If(cond, bt, bf) => {
                if is_comparison_involving(var_id, &cond.item) {
                    count += 1;
                }
                count += count_compare_or_bound_uses(bt, var_id);
                if let Some(bf) = bf {
                    count += count_compare_or_bound_uses(bf, var_id);
                }
            }
            AstStatement::While(cond, body) => {
                if is_comparison_involving(var_id, &cond.item) {
                    count += 1;
                }
                count += count_compare_or_bound_uses(body, var_id);
            }
            AstStatement::For(_, cond, _, body) => {
                if is_comparison_involving(var_id, &cond.item) {
                    count += 1;
                }
                count += count_compare_or_bound_uses(body, var_id);
            }
            AstStatement::Block(body) => {
                count += count_compare_or_bound_uses(body, var_id);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    count += count_compare_or_bound_uses(case_body, var_id);
                }
                if let Some(default_body) = default {
                    count += count_compare_or_bound_uses(default_body, var_id);
                }
            }
            _ => {}
        }
    }
    count
}

fn is_comparison_involving(var_id: AstVariableId, expr: &AstExpression) -> bool {
    if let AstExpression::BinaryOp(op, left, right) = expr {
        if matches!(
            op,
            AstBinaryOperator::Less
                | AstBinaryOperator::LessEqual
                | AstBinaryOperator::Greater
                | AstBinaryOperator::GreaterEqual
                | AstBinaryOperator::Equal
                | AstBinaryOperator::NotEqual
        ) {
            return expr_mentions_var(var_id, &left.item) || expr_mentions_var(var_id, &right.item);
        }
    }
    false
}

// ---------------------------------------------------------------------------
// L116: Format-string driven typing — annotate printf/scanf calls with
// expected argument types parsed from the format string literal.
// ---------------------------------------------------------------------------

fn annotate_format_string_types(stmts: &mut Vec<WrappedAstStatement>) {
    let mut insertions: Vec<(usize, String)> = Vec::new();
    for (i, stmt) in stmts.iter().enumerate() {
        if let AstStatement::Call(call) = &stmt.statement {
            if let Some(comment) = detect_format_string_types(call) {
                insertions.push((i, comment));
            }
        }
    }
    for (idx, text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn detect_format_string_types(call: &AstCall) -> Option<String> {
    let (name, args) = match call {
        AstCall::Unknown(name, args) => (name.as_str(), args),
        _ => return None,
    };
    let lower = name.to_ascii_lowercase();
    let is_printf_family = lower.contains("printf");
    let is_scanf_family = lower.contains("scanf");
    if !is_printf_family && !is_scanf_family {
        return None;
    }
    // Find the format string literal among the arguments.
    let fmt_str = args.iter().find_map(|arg| match &arg.item {
        AstExpression::Literal(AstLiteral::String(s)) if s.contains('%') => Some(s.as_str()),
        _ => None,
    })?;
    let types = parse_format_specifiers(fmt_str);
    if types.is_empty() {
        return None;
    }
    let types_str = types.join(", ");
    Some(format!(
        "// format string expects: [{types_str}] (from \"{name}\")"
    ))
}

fn parse_format_specifiers(fmt: &str) -> Vec<&'static str> {
    let mut types = Vec::new();
    let bytes = fmt.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            i += 1;
            // Skip flags: -, +, 0, space, #
            while i < bytes.len() && matches!(bytes[i], b'-' | b'+' | b'0' | b' ' | b'#') {
                i += 1;
            }
            // Skip width (digits or *)
            while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'*') {
                i += 1;
            }
            // Skip precision (.digits or .*)
            if i < bytes.len() && bytes[i] == b'.' {
                i += 1;
                while i < bytes.len() && (bytes[i].is_ascii_digit() || bytes[i] == b'*') {
                    i += 1;
                }
            }
            // Length modifiers
            let mut length = "";
            if i < bytes.len() {
                match bytes[i] {
                    b'l' => {
                        i += 1;
                        if i < bytes.len() && bytes[i] == b'l' {
                            length = "ll";
                            i += 1;
                        } else {
                            length = "l";
                        }
                    }
                    b'h' => {
                        i += 1;
                        if i < bytes.len() && bytes[i] == b'h' {
                            length = "hh";
                            i += 1;
                        } else {
                            length = "h";
                        }
                    }
                    b'z' | b'j' | b't' => {
                        length = match bytes[i] {
                            b'z' => "z",
                            b'j' => "j",
                            _ => "t",
                        };
                        i += 1;
                    }
                    _ => {}
                }
            }
            // Conversion specifier
            if i < bytes.len() {
                let ty = match bytes[i] {
                    b'd' | b'i' => match length {
                        "ll" => "long long",
                        "l" => "long",
                        "h" => "short",
                        "hh" => "char",
                        "z" => "ssize_t",
                        _ => "int",
                    },
                    b'u' => match length {
                        "ll" => "unsigned long long",
                        "l" => "unsigned long",
                        "z" => "size_t",
                        _ => "unsigned int",
                    },
                    b'x' | b'X' | b'o' => match length {
                        "ll" => "unsigned long long",
                        "l" => "unsigned long",
                        _ => "unsigned int",
                    },
                    b'f' | b'e' | b'g' | b'F' | b'E' | b'G' => {
                        if length == "l" {
                            "double"
                        } else {
                            "double"
                        }
                    }
                    b's' => {
                        if length == "l" {
                            "wchar_t*"
                        } else {
                            "char*"
                        }
                    }
                    b'c' => {
                        if length == "l" {
                            "wint_t"
                        } else {
                            "char"
                        }
                    }
                    b'p' => "void*",
                    b'n' => "int*",
                    b'%' => {
                        i += 1;
                        continue;
                    }
                    _ => {
                        i += 1;
                        continue;
                    }
                };
                types.push(ty);
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    types
}

// ---------------------------------------------------------------------------
// L858: Error-propagation modeling — detect "ret = call(); if (ret < 0)
// return ret;" patterns and annotate as error-propagation.
// ---------------------------------------------------------------------------

fn annotate_error_propagation(stmts: &mut Vec<WrappedAstStatement>) {
    let mut insertions: Vec<(usize, String)> = Vec::new();
    // Look for: Assignment(var, Call(...)) followed by If(var < 0, [Return(var)])
    for i in 0..stmts.len().saturating_sub(1) {
        let assigned_var = match &stmts[i].statement {
            AstStatement::Assignment(lhs, rhs) => {
                if matches!(rhs.item, AstExpression::Call(_)) {
                    if let AstExpression::Variable(_, var_id) = &lhs.item {
                        Some(*var_id)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            AstStatement::Declaration(decl_var, Some(rhs)) => {
                if matches!(rhs.item, AstExpression::Call(_)) {
                    Some(decl_var.id)
                } else {
                    None
                }
            }
            _ => None,
        };
        let Some(var_id) = assigned_var else {
            continue;
        };
        // Check if next statement is: if (var < 0) { return var; }
        // or: if (var == 0) { return ...; } or: if (var != 0) { return var; }
        if let AstStatement::If(cond, then_branch, _) = &stmts[i + 1].statement {
            if is_error_check_on_var(var_id, &cond.item)
                && then_branch_returns_or_propagates(then_branch, var_id)
            {
                insertions.push((i, "// error propagation pattern".to_string()));
            }
        }
    }
    for (idx, text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
}

fn is_error_check_on_var(var_id: AstVariableId, cond: &AstExpression) -> bool {
    if let AstExpression::BinaryOp(op, left, right) = cond {
        let is_check_op = matches!(
            op,
            AstBinaryOperator::Less
                | AstBinaryOperator::Equal
                | AstBinaryOperator::NotEqual
                | AstBinaryOperator::LessEqual
        );
        if !is_check_op {
            return false;
        }
        let has_var = matches!(&left.item, AstExpression::Variable(_, v) if *v == var_id)
            || matches!(&right.item, AstExpression::Variable(_, v) if *v == var_id);
        let has_zero_or_neg =
            expr_is_small_constant(&left.item) || expr_is_small_constant(&right.item);
        return has_var && has_zero_or_neg;
    }
    false
}

fn expr_is_small_constant(expr: &AstExpression) -> bool {
    matches!(
        expr,
        AstExpression::Literal(AstLiteral::Int(v)) if *v <= 0 && *v >= -4096
    ) || matches!(expr, AstExpression::Literal(AstLiteral::UInt(0)))
        || matches!(expr, AstExpression::Literal(AstLiteral::Int(0)))
}

fn then_branch_returns_or_propagates(
    branch: &[WrappedAstStatement],
    var_id: AstVariableId,
) -> bool {
    // The then-branch should contain a Return statement (possibly with the var).
    branch.iter().any(|s| match &s.statement {
        AstStatement::Return(Some(expr)) => {
            matches!(&expr.item, AstExpression::Variable(_, v) if *v == var_id)
                || expr_is_small_constant(&expr.item)
        }
        AstStatement::Return(None) => true,
        _ => false,
    })
}

// ---------------------------------------------------------------------------
// L918: Behavioral clustering for naming — detect dominant API call
// category in a function body and annotate the function accordingly.
// ---------------------------------------------------------------------------

fn annotate_behavioral_cluster(stmts: &mut Vec<WrappedAstStatement>) {
    let mut counts = [0u32; 8]; // crypto, io, string, math, memory, network, thread, ui
    collect_call_categories(stmts, &mut counts);
    let total: u32 = counts.iter().sum();
    if total < 3 {
        return;
    }
    let categories = [
        "crypto/hashing",
        "file/IO",
        "string manipulation",
        "math/numeric",
        "memory management",
        "network/socket",
        "threading/sync",
        "UI/windowing",
    ];
    let max_idx = counts
        .iter()
        .enumerate()
        .max_by_key(|(_, c)| *c)
        .map(|(i, _)| i)
        .unwrap_or(0);
    // Dominant category must be >= 40% of calls and at least 2 occurrences.
    if counts[max_idx] >= 2 && (counts[max_idx] as f64 / total as f64) >= 0.4 {
        let already = stmts.iter().any(
            |s| matches!(&s.statement, AstStatement::Comment(c) if c.contains("dominant API category")),
        );
        if !already {
            stmts.insert(
                0,
                WrappedAstStatement {
                    statement: AstStatement::Comment(format!(
                        "// dominant API category: {} ({}/{} calls)",
                        categories[max_idx], counts[max_idx], total
                    )),
                    origin: AstStatementOrigin::Unknown,
                    comment: None,
                },
            );
        }
    }
}

fn collect_call_categories(stmts: &[WrappedAstStatement], counts: &mut [u32; 8]) {
    for stmt in stmts {
        match &stmt.statement {
            AstStatement::Call(call) => categorize_call(call, counts),
            AstStatement::Assignment(_, rhs) => {
                if let AstExpression::Call(call) = &rhs.item {
                    categorize_call(call, counts);
                }
            }
            AstStatement::Declaration(_, Some(rhs)) => {
                if let AstExpression::Call(call) = &rhs.item {
                    categorize_call(call, counts);
                }
            }
            AstStatement::If(_, t, f) => {
                collect_call_categories(t, counts);
                if let Some(f) = f {
                    collect_call_categories(f, counts);
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                collect_call_categories(body, counts)
            }
            AstStatement::For(_, _, _, body) => collect_call_categories(body, counts),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    collect_call_categories(case_body, counts);
                }
                if let Some(d) = default {
                    collect_call_categories(d, counts);
                }
            }
            _ => {}
        }
    }
}

fn categorize_call(call: &AstCall, counts: &mut [u32; 8]) {
    let name = match call {
        AstCall::Unknown(name, _) => name.to_ascii_lowercase(),
        _ => return,
    };
    // 0: crypto/hashing
    if name.contains("aes")
        || name.contains("sha")
        || name.contains("md5")
        || name.contains("hmac")
        || name.contains("crypt")
        || name.contains("cipher")
        || name.contains("hash")
        || name.contains("pbkdf")
        || name.contains("chacha")
        || name.contains("poly1305")
    {
        counts[0] += 1;
    }
    // 1: file/IO
    if name.contains("fopen")
        || name.contains("fclose")
        || name.contains("fread")
        || name.contains("fwrite")
        || name.contains("fseek")
        || name.contains("ftell")
        || name.starts_with("read")
        || name.starts_with("write")
        || name.starts_with("open")
        || name.starts_with("close")
        || name.contains("ioctl")
        || name.contains("fcntl")
    {
        counts[1] += 1;
    }
    // 2: string manipulation
    if name.starts_with("str")
        || name.starts_with("wcs")
        || name.contains("sprintf")
        || name.contains("sscanf")
        || name.contains("memcpy")
        || name.contains("memmove")
        || name.contains("memset")
        || name.contains("memcmp")
    {
        counts[2] += 1;
    }
    // 3: math/numeric
    if name.starts_with("sin")
        || name.starts_with("cos")
        || name.starts_with("tan")
        || name.starts_with("sqrt")
        || name.starts_with("pow")
        || name.starts_with("log")
        || name.starts_with("exp")
        || name.starts_with("ceil")
        || name.starts_with("floor")
        || name.starts_with("fabs")
        || name.starts_with("fmod")
    {
        counts[3] += 1;
    }
    // 4: memory management
    if name.contains("malloc")
        || name.contains("calloc")
        || name.contains("realloc")
        || name == "free"
        || name.contains("alloc")
        || name.contains("mmap")
        || name.contains("munmap")
        || name.contains("virtualalloc")
        || name.contains("virtualfree")
        || name.contains("heapalloc")
        || name.contains("heapfree")
    {
        counts[4] += 1;
    }
    // 5: network/socket
    if name.contains("socket")
        || name.contains("connect")
        || name.contains("bind")
        || name.contains("listen")
        || name.contains("accept")
        || name.starts_with("send")
        || name.starts_with("recv")
        || name.contains("getaddrinfo")
        || name.contains("gethostby")
        || name.contains("select")
        || name.contains("poll")
        || name.contains("epoll")
    {
        counts[5] += 1;
    }
    // 6: threading/sync
    if name.contains("pthread")
        || name.contains("mutex")
        || name.contains("semaphore")
        || name.contains("critical_section")
        || name.contains("createthread")
        || name.contains("waitfor")
        || name.contains("signal")
        || name.contains("condvar")
    {
        counts[6] += 1;
    }
    // 7: UI/windowing
    if name.contains("createwindow")
        || name.contains("showwindow")
        || name.contains("messagebox")
        || name.contains("getmessage")
        || name.contains("dispatchmessage")
        || name.contains("defwindowproc")
        || name.contains("postmessage")
        || name.contains("sendmessage")
    {
        counts[7] += 1;
    }
}

// ---------------------------------------------------------------------------
// L485: Bitfield pack/unpack reconstruction — detect multiple mask/shift
// extractions from the same variable, suggesting bitfield access patterns.
// ---------------------------------------------------------------------------

fn annotate_bitfield_patterns(stmts: &mut Vec<WrappedAstStatement>) {
    // Collect (var_id, mask_or_shift_count) pairs across the function body.
    let mut var_extractions: hashbrown::HashMap<AstVariableId, u32> = hashbrown::HashMap::new();
    count_bitfield_extractions(stmts, &mut var_extractions);
    // If any variable has 3+ distinct mask/shift extractions, annotate.
    let candidates: Vec<AstVariableId> = var_extractions
        .into_iter()
        .filter(|(_, count)| *count >= 3)
        .map(|(var_id, _)| var_id)
        .collect();
    if candidates.is_empty() {
        return;
    }
    let already = stmts
        .iter()
        .any(|s| matches!(&s.statement, AstStatement::Comment(c) if c.contains("bitfield")));
    if already {
        return;
    }
    stmts.insert(
        0,
        WrappedAstStatement {
            statement: AstStatement::Comment(format!(
                "// likely bitfield access pattern ({} variable(s) with repeated mask/shift extraction)",
                candidates.len()
            )),
            origin: AstStatementOrigin::Unknown,
            comment: None,
        },
    );
}

fn count_bitfield_extractions(
    stmts: &[WrappedAstStatement],
    map: &mut hashbrown::HashMap<AstVariableId, u32>,
) {
    for stmt in stmts {
        match &stmt.statement {
            AstStatement::Assignment(_, rhs) => {
                check_bitfield_expr(&rhs.item, map);
            }
            AstStatement::Declaration(_, Some(rhs)) => {
                check_bitfield_expr(&rhs.item, map);
            }
            AstStatement::If(cond, t, f) => {
                check_bitfield_expr(&cond.item, map);
                count_bitfield_extractions(t, map);
                if let Some(f) = f {
                    count_bitfield_extractions(f, map);
                }
            }
            AstStatement::While(cond, body) => {
                check_bitfield_expr(&cond.item, map);
                count_bitfield_extractions(body, map);
            }
            AstStatement::For(_, cond, _, body) => {
                check_bitfield_expr(&cond.item, map);
                count_bitfield_extractions(body, map);
            }
            AstStatement::Block(body) => count_bitfield_extractions(body, map),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    count_bitfield_extractions(case_body, map);
                }
                if let Some(d) = default {
                    count_bitfield_extractions(d, map);
                }
            }
            _ => {}
        }
    }
}

/// Detect patterns like `(var >> N) & M` or `(var & M) >> N` — bitfield extraction.
fn check_bitfield_expr(expr: &AstExpression, map: &mut hashbrown::HashMap<AstVariableId, u32>) {
    match expr {
        // (something) & mask — check if `something` involves a shift of a variable
        AstExpression::BinaryOp(AstBinaryOperator::BitAnd, left, right) => {
            // Either side could be the mask constant
            if let Some(var_id) = extract_shifted_var(&left.item) {
                *map.entry(var_id).or_insert(0) += 1;
            } else if let Some(var_id) = extract_shifted_var(&right.item) {
                *map.entry(var_id).or_insert(0) += 1;
            }
            // Also: var & MASK (no shift) counts if MASK is not all-ones
            if is_mask_constant(&right.item) {
                if let AstExpression::Variable(_, var_id) = &left.item {
                    *map.entry(*var_id).or_insert(0) += 1;
                }
            }
            if is_mask_constant(&left.item) {
                if let AstExpression::Variable(_, var_id) = &right.item {
                    *map.entry(*var_id).or_insert(0) += 1;
                }
            }
        }
        // (var >> N) without mask — still a potential bitfield extraction
        AstExpression::BinaryOp(AstBinaryOperator::RightShift, inner, _) => {
            if let AstExpression::Variable(_, var_id) = &inner.item {
                *map.entry(*var_id).or_insert(0) += 1;
            }
        }
        _ => {}
    }
}

fn extract_shifted_var(expr: &AstExpression) -> Option<AstVariableId> {
    if let AstExpression::BinaryOp(AstBinaryOperator::RightShift, inner, _) = expr {
        if let AstExpression::Variable(_, var_id) = &inner.item {
            return Some(*var_id);
        }
    }
    None
}

fn is_mask_constant(expr: &AstExpression) -> bool {
    match expr {
        AstExpression::Literal(AstLiteral::Int(v)) => {
            let v = *v as u64;
            // Common masks: 0x1, 0x3, 0x7, 0xF, 0x1F, 0x3F, 0x7F, 0xFF, 0xFFFF, etc.
            v > 0 && v < u64::MAX && (v & (v + 1)) == 0
        }
        AstExpression::Literal(AstLiteral::UInt(v)) => {
            *v > 0 && *v < u64::MAX && (*v & (*v + 1)) == 0
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// L916: Domain vocabulary seeding — extract string literals from function
// body and annotate likely domain/purpose based on keywords.
// ---------------------------------------------------------------------------

fn annotate_domain_vocabulary(stmts: &mut Vec<WrappedAstStatement>) {
    let mut strings: Vec<String> = Vec::new();
    collect_string_literals(stmts, &mut strings);
    if strings.is_empty() {
        return;
    }
    let mut domains: Vec<&str> = Vec::new();
    for s in &strings {
        let lower = s.to_ascii_lowercase();
        if lower.contains("http")
            || lower.contains("url")
            || lower.contains("uri")
            || lower.contains("://")
        {
            if !domains.contains(&"network/URL") {
                domains.push("network/URL");
            }
        }
        if lower.contains("password")
            || lower.contains("token")
            || lower.contains("secret")
            || lower.contains("apikey")
            || lower.contains("api_key")
            || lower.contains("credential")
        {
            if !domains.contains(&"authentication/secrets") {
                domains.push("authentication/secrets");
            }
        }
        if lower.contains("encrypt")
            || lower.contains("decrypt")
            || lower.contains("cipher")
            || lower.contains("aes")
            || lower.contains("rsa")
            || lower.contains("sha")
        {
            if !domains.contains(&"cryptography") {
                domains.push("cryptography");
            }
        }
        if lower.contains("sql")
            || lower.contains("select ")
            || lower.contains("insert ")
            || lower.contains("database")
            || lower.contains("query")
        {
            if !domains.contains(&"database/SQL") {
                domains.push("database/SQL");
            }
        }
        if lower.contains("cookie")
            || lower.contains("session")
            || lower.contains("header")
            || lower.contains("content-type")
            || lower.contains("user-agent")
        {
            if !domains.contains(&"HTTP/web") {
                domains.push("HTTP/web");
            }
        }
        if lower.contains("registry")
            || lower.contains("hkey_")
            || lower.contains("regopen")
            || lower.contains("regedit")
        {
            if !domains.contains(&"Windows registry") {
                domains.push("Windows registry");
            }
        }
        if (lower.contains('/')
            && (lower.contains("/etc/")
                || lower.contains("/usr/")
                || lower.contains("/tmp/")
                || lower.contains("/var/")))
            || lower.contains("c:\\")
            || lower.contains("c:/")
        {
            if !domains.contains(&"filesystem paths") {
                domains.push("filesystem paths");
            }
        }
    }
    if domains.is_empty() {
        return;
    }
    let already = stmts.iter().any(
        |s| matches!(&s.statement, AstStatement::Comment(c) if c.contains("domain vocabulary")),
    );
    if already {
        return;
    }
    stmts.insert(
        0,
        WrappedAstStatement {
            statement: AstStatement::Comment(format!(
                "// domain vocabulary hints: {}",
                domains.join(", ")
            )),
            origin: AstStatementOrigin::Unknown,
            comment: None,
        },
    );
}

fn collect_string_literals(stmts: &[WrappedAstStatement], out: &mut Vec<String>) {
    for stmt in stmts {
        collect_string_literals_from_stmt(stmt, out);
    }
}

fn collect_string_literals_from_stmt(stmt: &WrappedAstStatement, out: &mut Vec<String>) {
    match &stmt.statement {
        AstStatement::Assignment(_, rhs) => collect_string_literals_from_expr(&rhs.item, out),
        AstStatement::Declaration(_, Some(rhs)) => {
            collect_string_literals_from_expr(&rhs.item, out)
        }
        AstStatement::Call(call) => collect_string_literals_from_call(call, out),
        AstStatement::If(cond, t, f) => {
            collect_string_literals_from_expr(&cond.item, out);
            collect_string_literals(t, out);
            if let Some(f) = f {
                collect_string_literals(f, out);
            }
        }
        AstStatement::While(cond, body) => {
            collect_string_literals_from_expr(&cond.item, out);
            collect_string_literals(body, out);
        }
        AstStatement::For(_, cond, _, body) => {
            collect_string_literals_from_expr(&cond.item, out);
            collect_string_literals(body, out);
        }
        AstStatement::Block(body) => collect_string_literals(body, out),
        AstStatement::Switch(_, cases, default) => {
            for (_, case_body) in cases {
                collect_string_literals(case_body, out);
            }
            if let Some(d) = default {
                collect_string_literals(d, out);
            }
        }
        AstStatement::Return(Some(expr)) => collect_string_literals_from_expr(&expr.item, out),
        _ => {}
    }
}

fn collect_string_literals_from_expr(expr: &AstExpression, out: &mut Vec<String>) {
    match expr {
        AstExpression::Literal(AstLiteral::String(s)) => out.push(s.clone()),
        AstExpression::Call(call) => collect_string_literals_from_call(call, out),
        AstExpression::BinaryOp(_, left, right) => {
            collect_string_literals_from_expr(&left.item, out);
            collect_string_literals_from_expr(&right.item, out);
        }
        AstExpression::UnaryOp(_, inner) => {
            collect_string_literals_from_expr(&inner.item, out);
        }
        AstExpression::Cast(_, inner) => {
            collect_string_literals_from_expr(&inner.item, out);
        }
        _ => {}
    }
}

fn collect_string_literals_from_call(call: &AstCall, out: &mut Vec<String>) {
    let args = match call {
        AstCall::Unknown(_, args) => args,
        AstCall::Function { args, .. } => args,
        AstCall::Variable { args, .. } => args,
        _ => return,
    };
    for arg in args {
        collect_string_literals_from_expr(&arg.item, out);
    }
}

// ---------------------------------------------------------------------------
// L375: Config/string xref mining — extract likely config keys, paths, URLs
// from string literals and annotate their usage context.
// ---------------------------------------------------------------------------

fn annotate_config_string_xrefs(stmts: &mut Vec<WrappedAstStatement>) {
    let mut config_strings: Vec<String> = Vec::new();
    find_config_strings(stmts, &mut config_strings);
    if config_strings.is_empty() {
        return;
    }
    let already = stmts.iter().any(
        |s| matches!(&s.statement, AstStatement::Comment(c) if c.contains("config/string references")),
    );
    if already {
        return;
    }
    // Show up to 5 config strings.
    let display: Vec<&str> = config_strings.iter().map(|s| s.as_str()).take(5).collect();
    let suffix = if config_strings.len() > 5 {
        format!(" (+{} more)", config_strings.len() - 5)
    } else {
        String::new()
    };
    stmts.insert(
        0,
        WrappedAstStatement {
            statement: AstStatement::Comment(format!(
                "// config/string references: [{}]{}",
                display.join(", "),
                suffix
            )),
            origin: AstStatementOrigin::Unknown,
            comment: None,
        },
    );
}

fn find_config_strings(stmts: &[WrappedAstStatement], out: &mut Vec<String>) {
    let mut all_strings: Vec<String> = Vec::new();
    collect_string_literals(stmts, &mut all_strings);
    for s in all_strings {
        if is_config_like_string(&s) && !out.contains(&s) {
            out.push(s);
        }
    }
}

fn is_config_like_string(s: &str) -> bool {
    // Config keys: KEY=value, key.subkey, key_name
    if s.contains("://") {
        return true; // URL
    }
    if s.starts_with('/') && s.len() > 2 && s.chars().filter(|c| *c == '/').count() >= 2 {
        return true; // Unix path
    }
    if s.len() >= 2 && s.chars().nth(1) == Some(':') && (s.starts_with('C') || s.starts_with('c')) {
        return true; // Windows path
    }
    // Environment variable or config key patterns
    if s.contains('=') && s.len() < 200 && !s.contains(' ') {
        return true;
    }
    // Dotted config key: "app.setting.name"
    if s.len() > 3
        && s.len() < 100
        && !s.contains(' ')
        && s.chars().filter(|c| *c == '.').count() >= 2
    {
        return true;
    }
    false
}

// ---------------------------------------------------------------------------
// L718: Resource cleanup normalization — detect sequences of resource-release
// calls (free/close/release/delete) and annotate as cleanup block.
// ---------------------------------------------------------------------------

fn annotate_resource_cleanup(stmts: &mut Vec<WrappedAstStatement>) {
    // Look for sequences of 2+ consecutive cleanup calls near the end of the
    // statement list (or before a return).
    let len = stmts.len();
    if len < 2 {
        return;
    }
    let mut insertions: Vec<(usize, String)> = Vec::new();
    let mut i = 0;
    while i < len {
        let mut run = 0;
        let start = i;
        while i < len && is_cleanup_call(&stmts[i].statement) {
            run += 1;
            i += 1;
        }
        if run >= 2 {
            insertions.push((start, format!("// resource cleanup ({run} release calls)")));
        }
        i += 1;
    }
    for (idx, text) in insertions.into_iter().rev() {
        stmts.insert(
            idx,
            WrappedAstStatement {
                statement: AstStatement::Comment(text),
                origin: AstStatementOrigin::Unknown,
                comment: None,
            },
        );
    }
    // Also recurse into sub-blocks.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, t, f) => {
                annotate_resource_cleanup(t);
                if let Some(f) = f {
                    annotate_resource_cleanup(f);
                }
            }
            AstStatement::While(_, body) | AstStatement::Block(body) => {
                annotate_resource_cleanup(body);
            }
            AstStatement::For(_, _, _, body) => annotate_resource_cleanup(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    annotate_resource_cleanup(case_body);
                }
                if let Some(d) = default {
                    annotate_resource_cleanup(d);
                }
            }
            _ => {}
        }
    }
}

fn is_cleanup_call(stmt: &AstStatement) -> bool {
    let call = match stmt {
        AstStatement::Call(call) => call,
        _ => return false,
    };
    let name = match call {
        AstCall::Unknown(name, _) => name.to_ascii_lowercase(),
        _ => return false,
    };
    name == "free"
        || name.contains("close")
        || name.contains("release")
        || name.contains("destroy")
        || name.contains("delete")
        || name.contains("unref")
        || name.contains("dealloc")
        || name.contains("munmap")
        || name.contains("freeaddrinfo")
        || name.contains("closehandle")
        || name.contains("regclosekey")
}

// ---------------------------------------------------------------------------
// L908: Sanitizer shadow-memory modeling — detect ASan/TSan/MSan shadow
// address computation patterns and annotate.
// ---------------------------------------------------------------------------

fn annotate_sanitizer_shadow(stmts: &mut Vec<WrappedAstStatement>) {
    let mut found = false;
    check_sanitizer_patterns(stmts, &mut found);
    if !found {
        return;
    }
    let already = stmts.iter().any(
        |s| matches!(&s.statement, AstStatement::Comment(c) if c.contains("sanitizer shadow")),
    );
    if already {
        return;
    }
    stmts.insert(
        0,
        WrappedAstStatement {
            statement: AstStatement::Comment(
                "// contains sanitizer shadow-memory access pattern (likely ASan/MSan instrumentation)"
                    .to_string(),
            ),
            origin: AstStatementOrigin::Unknown,
            comment: None,
        },
    );
}

fn check_sanitizer_patterns(stmts: &[WrappedAstStatement], found: &mut bool) {
    if *found {
        return;
    }
    for stmt in stmts {
        if *found {
            return;
        }
        match &stmt.statement {
            AstStatement::Assignment(_, rhs) => {
                if expr_has_shadow_pattern(&rhs.item) {
                    *found = true;
                }
            }
            AstStatement::Declaration(_, Some(rhs)) => {
                if expr_has_shadow_pattern(&rhs.item) {
                    *found = true;
                }
            }
            AstStatement::If(cond, t, f) => {
                if expr_has_shadow_pattern(&cond.item) {
                    *found = true;
                }
                check_sanitizer_patterns(t, found);
                if let Some(f) = f {
                    check_sanitizer_patterns(f, found);
                }
            }
            AstStatement::While(cond, body) => {
                if expr_has_shadow_pattern(&cond.item) {
                    *found = true;
                }
                check_sanitizer_patterns(body, found);
            }
            AstStatement::For(_, cond, _, body) => {
                if expr_has_shadow_pattern(&cond.item) {
                    *found = true;
                }
                check_sanitizer_patterns(body, found);
            }
            AstStatement::Block(body) => check_sanitizer_patterns(body, found),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    check_sanitizer_patterns(case_body, found);
                }
                if let Some(d) = default {
                    check_sanitizer_patterns(d, found);
                }
            }
            _ => {}
        }
    }
}

/// Detect ASan-like shadow address pattern: `*(addr >> 3) + SHADOW_OFFSET`
/// or `addr >> 3` combined with a large constant offset (0x7fff8000 etc.)
fn expr_has_shadow_pattern(expr: &AstExpression) -> bool {
    match expr {
        // *(expr >> 3 + offset) or deref of shifted address
        AstExpression::Deref(inner) => {
            if let AstExpression::BinaryOp(AstBinaryOperator::Add, left, right) = &inner.item {
                let has_shift = is_shadow_shift(&left.item) || is_shadow_shift(&right.item);
                let has_large_const = is_shadow_offset(&left.item) || is_shadow_offset(&right.item);
                return has_shift && has_large_const;
            }
            is_shadow_shift(&inner.item)
        }
        AstExpression::BinaryOp(AstBinaryOperator::Add, left, right) => {
            let has_shift = is_shadow_shift(&left.item) || is_shadow_shift(&right.item);
            let has_large_const = is_shadow_offset(&left.item) || is_shadow_offset(&right.item);
            has_shift && has_large_const
        }
        _ => false,
    }
}

fn is_shadow_shift(expr: &AstExpression) -> bool {
    // addr >> 3 (ASan) or addr >> 1 (MSan character-level)
    if let AstExpression::BinaryOp(AstBinaryOperator::RightShift, _, shift_amt) = expr {
        if let AstExpression::Literal(AstLiteral::Int(n)) = &shift_amt.item {
            return *n == 3 || *n == 1;
        }
        if let AstExpression::Literal(AstLiteral::UInt(n)) = &shift_amt.item {
            return *n == 3 || *n == 1;
        }
    }
    false
}

fn is_shadow_offset(expr: &AstExpression) -> bool {
    // Common ASan shadow offsets: 0x7fff8000, 0x20000000, 0x100000000000, etc.
    match expr {
        AstExpression::Literal(AstLiteral::Int(v)) => {
            let v = *v as u64;
            v >= 0x1000_0000 && (v & 0xFFF) == 0
        }
        AstExpression::Literal(AstLiteral::UInt(v)) => *v >= 0x1000_0000 && (*v & 0xFFF) == 0,
        _ => false,
    }
}

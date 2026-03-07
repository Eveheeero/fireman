use crate::{
    abstract_syntax_tree::{
        Ast, AstBinaryOperator, AstCall, AstExpression, AstFunctionId, AstFunctionVersion,
        AstLiteral, AstStatement, AstStatementOrigin, AstUnaryOperator, AstValueType,
        AstVariableAccessType, AstVariableId, GetRelatedVariables, ProcessedOptimization,
        WrappedAstStatement,
    },
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
        all_param_var_ids = function
            .parameters
            .iter()
            .filter_map(|p| p.id.as_ref().left().copied())
            .collect();
    }

    annotate_statement_list(&mut body);
    annotate_crypto_fingerprint(&mut body);
    annotate_decompression_fingerprint(&mut body);
    annotate_xor_decryption_loop(&mut body);
    annotate_integrity_check_loop(&mut body);
    annotate_loop_invariants(&mut body);
    if let Some(var_id) = first_param_var_id {
        if param_count >= 1 {
            annotate_this_or_sret_pointer(&mut body, var_id);
        }
    }
    annotate_obfuscation_indicators(&mut body);
    annotate_ptr_len_pairs(&mut body, &all_param_var_ids);
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

fn annotate_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    // First, recurse into nested statement lists.
    for stmt in stmts.iter_mut() {
        annotate_statement(stmt);
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
            }
            AstStatement::Assignment(_, rhs) => {
                // Macro-like pattern: var = (a < b) ? a : b → MIN, etc.
                if let Some(macro_name) = detect_min_max_pattern(&rhs.item) {
                    insertions.push((i, format!("// {macro_name}")));
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

fn annotate_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            annotate_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                annotate_statement_list(branch_false);
            }
        }
        AstStatement::While(_, body) => annotate_statement_list(body),
        AstStatement::For(init, _, update, body) => {
            annotate_statement(init);
            annotate_statement(update);
            annotate_statement_list(body);
        }
        AstStatement::Switch(_, cases, default) => {
            for (_lit, case_body) in cases.iter_mut() {
                annotate_statement_list(case_body);
            }
            if let Some(default_body) = default {
                annotate_statement_list(default_body);
            }
        }
        AstStatement::Block(body) => annotate_statement_list(body),
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
fn annotate_this_or_sret_pointer(body: &mut [WrappedAstStatement], first_param: AstVariableId) {
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
                first.comment =
                    Some("first parameter likely sret (hidden return pointer)".to_string());
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

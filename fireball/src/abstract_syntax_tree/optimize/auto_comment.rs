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
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        body = std::mem::take(&mut function.body);
    }

    annotate_statement_list(&mut body);
    annotate_crypto_fingerprint(&mut body);
    annotate_decompression_fingerprint(&mut body);
    annotate_xor_decryption_loop(&mut body);
    annotate_integrity_check_loop(&mut body);
    annotate_loop_invariants(&mut body);

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

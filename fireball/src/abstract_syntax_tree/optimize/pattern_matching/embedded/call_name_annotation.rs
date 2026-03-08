//! Call-name annotation extracted from auto_comment.rs.
//!
//! Inserts descriptive comments before statement-level calls whose names
//! match known patterns (noreturn, SEH, allocation, etc.).

use crate::{
    abstract_syntax_tree::{
        Ast, AstCall, AstFunctionId, AstFunctionVersion, AstStatement, AstStatementOrigin,
        ProcessedOptimization, WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn annotate_call_names(
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

    annotate_call_names_in_list(&mut body);

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

fn annotate_call_names_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse first.
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                annotate_call_names_in_list(bt);
                if let Some(bf) = bf {
                    annotate_call_names_in_list(bf);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::Block(body) => {
                annotate_call_names_in_list(body);
            }
            AstStatement::For(_, _, _, body) => {
                annotate_call_names_in_list(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    annotate_call_names_in_list(case_body);
                }
                if let Some(default_body) = default {
                    annotate_call_names_in_list(default_body);
                }
            }
            _ => {}
        }
    }

    // Collect insertions at this level.
    let mut insertions: Vec<(usize, String)> = Vec::new();
    for (i, stmt) in stmts.iter().enumerate() {
        if let AstStatement::Call(call) = &stmt.statement {
            for comment in call_name_comments(call) {
                insertions.push((i, comment.to_string()));
            }
        }
    }

    // Insert from back to front.
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

/// Return all applicable comment strings for a given call.
fn call_name_comments(call: &AstCall) -> Vec<&'static str> {
    let name = match call {
        AstCall::Unknown(name, _) => name.as_str(),
        _ => return Vec::new(),
    };
    let lower = name.to_ascii_lowercase();
    let mut comments = Vec::new();

    // noreturn
    if lower.contains("exit")
        || lower.contains("abort")
        || lower.contains("panic")
        || lower.contains("terminate")
    {
        comments.push("// does not return");
    }

    // SEH
    if name.contains("SEH")
        || name.contains("_except_handler")
        || lower.contains("_seh_")
        || lower.contains("setunhandledexceptionfilter")
    {
        comments.push("// SEH setup");
    }

    // Guard
    if lower.contains("_guard_") || lower.contains("__cxa_guard") {
        comments.push("// static local initialization guard");
    }

    // Anti-debug
    if lower.contains("isdebuggerpresent")
        || lower.contains("ptrace")
        || lower.contains("ntqueryinformationprocess")
        || lower.contains("checkremotedebuggerpresent")
    {
        comments.push("// anti-debug / anti-analysis check");
    }

    // Instrumentation
    if lower.contains("__sanitizer")
        || lower.contains("__asan")
        || lower.contains("__msan")
        || lower.contains("__tsan")
        || lower.contains("__llvm_profile")
        || lower.contains("__gcov")
    {
        comments.push("// sanitizer/coverage instrumentation");
    }

    // Retpoline
    if lower.contains("retpoline") || lower.contains("__x86_indirect_thunk") {
        comments.push("// retpoline indirect call thunk");
    }

    // Logging
    if lower.contains("printf")
        || lower.contains("syslog")
        || lower.contains("nslog")
        || lower.contains("outputdebugstring")
    {
        comments.push("// logging / debug output");
    }

    // Allocation
    if lower.contains("malloc") || lower.contains("calloc") {
        comments.push("// heap allocation");
    }
    if lower.contains("realloc") {
        comments.push("// heap reallocation");
    }
    if lower.contains("free") && !lower.contains("freeze") {
        comments.push("// heap deallocation");
    }

    // Timing
    if lower.contains("clock_gettime")
        || lower.contains("gettimeofday")
        || lower.contains("queryperformancecounter")
    {
        comments.push("// timing / performance measurement");
    }

    // Alloca
    if lower.contains("alloca") || lower.contains("__chkstk") {
        comments.push("// dynamic stack allocation (alloca/VLA)");
    }

    // Objective-C
    if lower.contains("objc_msgsend") {
        comments.push("// Objective-C runtime dispatch");
    }

    // Safe/split stack
    if lower.contains("__safestack") || lower.contains("__splitstack") {
        comments.push("// safe/split stack instrumentation");
    }

    // C++ exceptions
    if lower.contains("__cxa_throw")
        || lower.contains("__cxa_begin_catch")
        || lower.contains("__cxa_end_catch")
        || lower.contains("__cxa_allocate_exception")
    {
        comments.push("// C++ exception handling runtime");
    }

    // TLS
    if lower.contains("__tls_") || lower.contains("emutls") || lower.contains("__emutls") {
        comments.push("// thread-local storage access");
    }

    // Rust panic
    if lower.contains("rust_begin_unwind") || lower.contains("rust_panic") {
        comments.push("// Rust panic / unwind runtime");
    }

    // setjmp/longjmp
    if lower.contains("setjmp") || lower.contains("longjmp") {
        comments.push("// non-local jump (setjmp/longjmp)");
    }

    // Atomic
    if lower.contains("__atomic") || lower.contains("__sync_") {
        comments.push("// atomic operation");
    }

    // String ops
    if lower.contains("memcpy")
        || lower.contains("memset")
        || lower.contains("memmove")
        || lower.contains("strlen")
        || lower.contains("strcmp")
        || lower.contains("strncpy")
        || lower.contains("strcpy")
    {
        comments.push("// string operation");
    }

    // Math
    if lower.contains("sqrt")
        || lower.contains("sin")
        || lower.contains("cos")
        || lower.contains("pow")
        || lower.contains("exp")
        || lower.contains("log")
        || lower.contains("fabs")
        || lower.contains("ceil")
        || lower.contains("floor")
    {
        comments.push("// math library call");
    }

    // Stack canary
    if lower.contains("stack_chk") {
        comments.push("// stack canary check");
    }

    comments
}

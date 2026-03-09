//! Logging call suppression.
//!
//! Removes calls to:
//!   - NSLog
//!   - syslog
//!   - __android_log_print
//!   - __android_log_write

use crate::{
    abstract_syntax_tree::{
        Ast, AstCall, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

const SUPPRESSED_SYMBOLS: &[&str] = &[
    "NSLog",
    "syslog",
    "__android_log_print",
    "__android_log_write",
];

pub(crate) fn suppress_logging(
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

    suppress_calls_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::PatternMatch);
    }

    Ok(())
}

fn suppress_calls_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    // Recurse into nested structures
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                suppress_calls_in_list(bt);
                if let Some(bf) = bf {
                    suppress_calls_in_list(bf);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::Block(body) => {
                suppress_calls_in_list(body);
            }
            AstStatement::For(_, _, _, body) => {
                suppress_calls_in_list(body);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    suppress_calls_in_list(case_body);
                }
                if let Some(default_body) = default {
                    suppress_calls_in_list(default_body);
                }
            }
            _ => {}
        }
    }

    stmts.retain(|stmt| {
        if let AstStatement::Call(call) = &stmt.statement {
            !call_matches_any(call)
        } else {
            true
        }
    });
}

fn call_matches_any(call: &AstCall) -> bool {
    let name = match call {
        AstCall::Function { name, .. } => name.to_string(),
        AstCall::Unknown(name, _) => name.clone(),
        _ => return false,
    };
    SUPPRESSED_SYMBOLS.iter().any(|sym| name.contains(sym))
}

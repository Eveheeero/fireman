//! Security-mitigation scaffold suppression.
//!
//! Removes calls to mitigation helpers:
//!   - __stack_chk_fail
//!   - __cfi_check
//!   - __cfi_slowpath

use crate::{
    abstract_syntax_tree::{
        Ast, AstCall, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

const SUPPRESSED_SYMBOLS: &[&str] = &["__stack_chk_fail", "__cfi_check", "__cfi_slowpath"];

pub(crate) fn suppress_security_scaffolds(
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
            .push(ProcessedOptimization::PatternMatching);
    }

    Ok(())
}

fn suppress_calls_in_list(stmts: &mut Vec<WrappedAstStatement>) {
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
        AstCall::Unknown(name, _) => name.clone(),
        _ => return false,
    };

    SUPPRESSED_SYMBOLS.iter().any(|sym| name.contains(sym))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abstract_syntax_tree::{
        AstFunctionId, AstStatement,
        optimize::pattern_matching::embedded::test_utils::test_utils::*,
    };

    #[test]
    fn parity_security_scaffold_suppression() {
        let fid = AstFunctionId { address: 0x9000 };
        let (_ids, vm) = make_var_map(fid, &[]);

        let body = vec![
            wrap_statement(AstStatement::Call(AstCall::Unknown(
                "__stack_chk_fail".to_string(),
                vec![],
            ))),
            wrap_statement(AstStatement::Call(AstCall::Unknown(
                "real_work".to_string(),
                vec![],
            ))),
            wrap_statement(AstStatement::Call(AstCall::Unknown(
                "__cfi_check".to_string(),
                vec![],
            ))),
            wrap_statement(AstStatement::Call(AstCall::Unknown(
                "__cfi_slowpath".to_string(),
                vec![],
            ))),
        ];

        let (fb, embed) = run_parity(body, vm, |c| c.security_scaffold_suppression(true));
        assert_eq!(fb, embed, "security_scaffold_suppression parity failed");
        assert!(embed.contains("real_work"));
        assert!(!embed.contains("__stack_chk_fail"));
        assert!(!embed.contains("__cfi_check"));
        assert!(!embed.contains("__cfi_slowpath"));
    }
}

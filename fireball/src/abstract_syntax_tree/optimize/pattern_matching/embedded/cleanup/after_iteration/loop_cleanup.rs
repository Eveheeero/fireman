//! Redundant loop-control cleanup: while(c) { ... continue; } -> while(c) { ... }.

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(crate) fn cleanup_loops(
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

    cleanup_in_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::ControlFlowCleanup);
    }

    Ok(())
}

fn cleanup_in_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                cleanup_in_list(bt);
                if let Some(bf) = bf {
                    cleanup_in_list(bf);
                }
            }
            AstStatement::While(_, body) | AstStatement::DoWhile(_, body) => {
                cleanup_in_list(body);
                try_remove_last_continue(body);
            }
            AstStatement::For(_, _, _, body) => {
                cleanup_in_list(body);
                try_remove_last_continue(body);
            }
            AstStatement::Block(body) => cleanup_in_list(body),
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases {
                    cleanup_in_list(case_body);
                }
                if let Some(body) = default {
                    cleanup_in_list(body);
                }
            }
            _ => {}
        }
    }
}

fn try_remove_last_continue(body: &mut Vec<WrappedAstStatement>) {
    if body.is_empty() {
        return;
    }
    let last_idx = body.len() - 1;
    if matches!(body[last_idx].statement, AstStatement::Continue) {
        body.remove(last_idx);
    }
}

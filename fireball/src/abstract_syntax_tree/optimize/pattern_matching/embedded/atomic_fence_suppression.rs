//! Atomic fence instruction suppression.
//!
//! Removes assembly lines containing memory fence instructions that add
//! noise to decompiled output:
//!   - mfence
//!   - lfence
//!   - sfence

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
    },
    prelude::DecompileError,
};

const FENCE_SYMBOLS: &[&str] = &["mfence", "lfence", "sfence"];

pub(crate) fn suppress_atomic_fences(
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

    body.retain(|stmt| {
        if let AstStatement::Assembly(asm_text) = &stmt.statement {
            !FENCE_SYMBOLS.iter().any(|sym| asm_text.contains(sym))
        } else {
            true
        }
    });

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

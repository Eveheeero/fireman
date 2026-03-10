//! Stack allocation intrinsic suppression.
//!
//! Removes assembly lines containing compiler-generated alloca helpers:
//!   - __alloca
//!   - _alloca

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
    },
    prelude::DecompileError,
};

const ALLOCA_SYMBOLS: &[&str] = &["__alloca", "_alloca"];

pub(crate) fn suppress_alloca(
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
            !ALLOCA_SYMBOLS.iter().any(|sym| asm_text.contains(sym))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abstract_syntax_tree::optimize::pattern_matching::embedded::test_utils::test_utils::assert_before_ir_suppression;

    #[test]
    fn direct_alloca_suppression_removes_alloca_helpers() {
        assert_before_ir_suppression(
            "suppression/before-ir-analyzation/alloca-suppression.fb",
            "call __alloca",
            "mov eax, ebx",
            suppress_alloca,
        );
    }
}

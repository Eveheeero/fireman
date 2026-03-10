//! PLT resolver stub suppression.
//!
//! Removes assembly lines containing dynamic linker resolution stubs:
//!   - _dl_runtime_resolve
//!   - _dl_fixup

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
    },
    prelude::DecompileError,
};

const PLT_RESOLVER_SYMBOLS: &[&str] = &["_dl_runtime_resolve", "_dl_fixup"];

pub(crate) fn suppress_plt_resolver(
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
            !PLT_RESOLVER_SYMBOLS
                .iter()
                .any(|sym| asm_text.contains(sym))
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
    fn direct_plt_resolver_suppression_removes_runtime_resolvers() {
        assert_before_ir_suppression(
            "suppression/before-ir-analyzation/plt-resolver-suppression.fb",
            "jmp _dl_runtime_resolve",
            "mov eax, ebx",
            suppress_plt_resolver,
        );
    }
}

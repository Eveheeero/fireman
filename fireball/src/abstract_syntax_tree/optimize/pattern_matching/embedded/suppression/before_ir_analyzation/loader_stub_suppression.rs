//! MSVC/CRT loader stub suppression.
//!
//! Removes assembly lines containing CRT initialization and security cookie
//! routines that clutter decompiled output:
//!   - __security_init_cookie
//!   - _initterm
//!   - __scrt_common_main
//!   - _CRT_INIT

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
    },
    prelude::DecompileError,
};

const LOADER_STUB_SYMBOLS: &[&str] = &[
    "__security_init_cookie",
    "_initterm",
    "__scrt_common_main",
    "_CRT_INIT",
];

pub(crate) fn suppress_loader_stubs(
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
            !LOADER_STUB_SYMBOLS.iter().any(|sym| asm_text.contains(sym))
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
    fn direct_loader_stub_suppression_removes_loader_bootstrap() {
        assert_before_ir_suppression(
            "suppression/before-ir-analyzation/loader-stub-suppression.fb",
            "call __security_init_cookie",
            "mov eax, ebx",
            suppress_loader_stubs,
        );
    }
}

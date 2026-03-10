//! Bare-metal startup routine suppression.
//!
//! Removes assembly lines containing C runtime startup/teardown stubs:
//!   - __libc_start_main
//!   - __do_global_ctors
//!   - __do_global_dtors

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
    },
    prelude::DecompileError,
};

const BARE_METAL_STARTUP_SYMBOLS: &[&str] = &[
    "__libc_start_main",
    "__do_global_ctors",
    "__do_global_dtors",
];

pub(crate) fn suppress_bare_metal_startup(
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
            !BARE_METAL_STARTUP_SYMBOLS
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
    fn direct_bare_metal_startup_suppression_removes_startup_stubs() {
        assert_before_ir_suppression(
            "suppression/before-ir-analyzation/bare-metal-startup-suppression.fb",
            "call __libc_start_main",
            "mov eax, ebx",
            suppress_bare_metal_startup,
        );
    }
}

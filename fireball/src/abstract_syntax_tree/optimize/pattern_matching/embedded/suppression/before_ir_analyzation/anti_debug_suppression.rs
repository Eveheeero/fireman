//! Anti-debug and anti-VM instruction suppression.
//!
//! Removes assembly lines containing anti-debugging calls:
//!   - IsDebuggerPresent
//!   - NtQueryInformationProcess
//!   - CheckRemoteDebuggerPresent
//!   - int 0x2d

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
    },
    prelude::DecompileError,
};

const ANTI_DEBUG_SYMBOLS: &[&str] = &[
    "IsDebuggerPresent",
    "NtQueryInformationProcess",
    "CheckRemoteDebuggerPresent",
    "int 0x2d",
];

pub(crate) fn suppress_anti_debug(
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
            !ANTI_DEBUG_SYMBOLS.iter().any(|sym| asm_text.contains(sym))
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
    fn direct_anti_debug_suppression_removes_debug_probes() {
        assert_before_ir_suppression(
            "suppression/before-ir-analyzation/anti-debug-suppression.fb",
            "call IsDebuggerPresent",
            "mov eax, ebx",
            suppress_anti_debug,
        );
    }
}

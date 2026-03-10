//! Anti-debug call suppression (AST level).
//!
//! Removes calls to:
//!   - IsDebuggerPresent
//!   - CheckRemoteDebuggerPresent
//!   - NtQueryInformationProcess

use crate::{
    abstract_syntax_tree::{
        Ast, AstCall, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};
use hashbrown::HashMap;

const SUPPRESSED_SYMBOLS: &[&str] = &[
    "IsDebuggerPresent",
    "CheckRemoteDebuggerPresent",
    "NtQueryInformationProcess",
];

pub(crate) fn suppress_anti_debug_ast(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let function_versions = ast.function_versions.clone();
    let functions = ast.functions.clone();
    let mut body;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        body = std::mem::take(&mut function.body);
    }

    suppress_calls_in_list(&mut body, &function_versions, &functions);

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

fn suppress_calls_in_list(
    stmts: &mut Vec<WrappedAstStatement>,
    function_versions: &HashMap<AstFunctionId, AstFunctionVersion>,
    functions: &crate::abstract_syntax_tree::ArcAstFunctionMap,
) {
    // Recurse into nested structures
    for stmt in stmts.iter_mut() {
        match &mut stmt.statement {
            AstStatement::If(_, bt, bf) => {
                suppress_calls_in_list(bt, function_versions, functions);
                if let Some(bf) = bf {
                    suppress_calls_in_list(bf, function_versions, functions);
                }
            }
            AstStatement::While(_, body)
            | AstStatement::DoWhile(_, body)
            | AstStatement::Block(body) => {
                suppress_calls_in_list(body, function_versions, functions);
            }
            AstStatement::For(_, _, _, body) => {
                suppress_calls_in_list(body, function_versions, functions);
            }
            AstStatement::Switch(_, cases, default) => {
                for (_, case_body) in cases.iter_mut() {
                    suppress_calls_in_list(case_body, function_versions, functions);
                }
                if let Some(default_body) = default {
                    suppress_calls_in_list(default_body, function_versions, functions);
                }
            }
            _ => {}
        }
    }

    stmts.retain(|stmt| {
        if let AstStatement::Call(call) = &stmt.statement {
            !call_matches_any(call, function_versions, functions)
        } else {
            true
        }
    });
}

fn call_matches_any(
    call: &AstCall,
    function_versions: &HashMap<AstFunctionId, AstFunctionVersion>,
    functions: &crate::abstract_syntax_tree::ArcAstFunctionMap,
) -> bool {
    let name = match call {
        AstCall::Function { target, .. } => {
            let Some(version) = function_versions.get(target) else {
                return false;
            };
            let functions = functions.read().unwrap();
            let Some(function_versions) = functions.get(target) else {
                return false;
            };
            let Some(function) = function_versions.get(version) else {
                return false;
            };
            let Some(name) = &function.name else {
                return false;
            };
            name.clone()
        }
        AstCall::Unknown(name, _) => name.clone(),
        _ => return false,
    };
    SUPPRESSED_SYMBOLS.iter().any(|sym| name.contains(sym))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        abstract_syntax_tree::{
            AstFunction, AstParameter, AstStatement, AstValueType,
            optimize::pattern_matching::embedded::test_utils::test_utils::*,
        },
        ir::analyze::IrFunction,
        utils::version_map::VersionMap,
    };
    use std::sync::{Arc, RwLock};

    fn build_function_map(
        target: AstFunctionId,
        version: AstFunctionVersion,
        name: &str,
    ) -> crate::abstract_syntax_tree::ArcAstFunctionMap {
        let mut functions = HashMap::new();
        functions.insert(
            target,
            VersionMap::new(
                version,
                AstFunction {
                    name: Some(name.to_string()),
                    id: target,
                    ir: Arc::new(IrFunction::new(Vec::new().into(), Vec::new(), Vec::new())),
                    return_type: AstValueType::Int,
                    parameters: Vec::<AstParameter>::new(),
                    variables: Arc::new(RwLock::new(HashMap::new())),
                    body: Vec::new(),
                    processed_optimizations: Vec::new(),
                },
            ),
        );
        Arc::new(RwLock::new(functions))
    }

    #[test]
    fn parity_anti_debug_ast_suppression() {
        let fid = AstFunctionId { address: 0x9000 };
        let (_ids, vm) = make_var_map(fid, &[]);

        let body = vec![
            wrap_statement(AstStatement::Call(AstCall::Unknown(
                "IsDebuggerPresent".to_string(),
                vec![],
            ))),
            wrap_statement(AstStatement::Call(AstCall::Unknown(
                "real_work".to_string(),
                vec![],
            ))),
        ];

        let (fb, embed) = run_parity(
            "suppression/after-iteration/anti-debug-ast-suppression.fb",
            body,
            vm,
            |c| c.anti_debug_ast_suppression(true),
        );
        assert_eq!(fb, embed, "anti_debug_ast_suppression parity failed");
        assert!(embed.contains("real_work"));
        assert!(!embed.contains("IsDebuggerPresent"));
    }

    #[test]
    fn matches_direct_function_calls_by_resolved_name() {
        let target = AstFunctionId { address: 0x401000 };
        let version = AstFunctionVersion(1);
        let function_versions = HashMap::from([(target, version)]);
        let functions = build_function_map(target, version, "IsDebuggerPresent");
        let call = AstCall::Function {
            target,
            args: Vec::new(),
        };

        assert!(call_matches_any(&call, &function_versions, &functions));
    }
}

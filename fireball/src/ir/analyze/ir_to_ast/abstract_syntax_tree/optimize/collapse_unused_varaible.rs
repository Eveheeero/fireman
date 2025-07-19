use crate::{
    ir::analyze::ir_to_ast::abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstVariableId,
    },
    prelude::DecompileError,
};
use hashbrown::HashSet;

pub(super) fn collapse_unused_variables(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let body;
    let variables;
    let ir_function;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();

        body = std::mem::take(&mut function.body);
        variables = function.variables.clone();
        ir_function = function.ir.clone();
    }

    let mut used_vars: HashSet<AstVariableId> = HashSet::new();
    let mut removable_vars: HashSet<AstVariableId> = HashSet::new();
    for ast_statement in body.iter().rev() {}

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
    }
    Ok(())
}

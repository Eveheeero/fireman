use crate::{
    abstract_syntax_tree::{Ast, AstFunctionId, AstFunctionVersion, ProcessedOptimization},
    ir::data::IrDataAccessType,
    prelude::DecompileError,
};
use hashbrown::HashSet;

pub(super) fn analyze_parameters(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let _variables;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();

        _variables = function.variables.clone();
    }

    Ok(())
}

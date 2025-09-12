use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, GetRelatedVariables, ProcessedOptimization,
    },
    ir::data::IrDataAccessType,
    prelude::DecompileError,
};
use hashbrown::HashSet;

pub(super) fn analyze_parameters(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let variables;
    let body;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();

        body = std::mem::take(&mut function.body);
        variables = function.variables.clone();
    }
    enum ParameterLocation {}
    let parameter_locations: HashSet<ParameterLocation> = HashSet::new();

    let first_arg_undetectable_statement_index =
        super::utils::get_first_arg_undetectable_statement_index(body.iter());
    let len = body.len();
    for (i, stmt) in body.iter().enumerate() {
        let stmt = &stmt.statement;

        /* analyze registers before undetectable statements */
        if i < first_arg_undetectable_statement_index.unwrap_or(usize::MAX) {
            // check used registers
            // super::utils::var_id_to_access_location();

            /* TODO what if there is register used after undetectable statement? */
        }

        /* analyze stack related accesses */
        'a: {
            let _related_vars = stmt.get_related_variables();
            // check stack reading
            // check stack assigning
        }
    }

    // TODO result 레지스터 및 스택 순서 기반으로 정렬 및 적용

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function
            .processed_optimizations
            .push(ProcessedOptimization::ParameterAnalyzation);
        function.body = body;
    }
    Ok(())
}

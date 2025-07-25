use hashbrown::HashSet;

use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, AstVariableId, WrappedAstStatement,
    },
    ir::data::IrData,
    prelude::DecompileError,
    utils::Aos,
};

/// check variables are overwritten without reading in ir level
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

    let mut used_locations: HashSet<Aos<IrData>> = HashSet::new();
    let mut new_body: Vec<WrappedAstStatement> = Vec::new();
    for stmt in body.into_iter().rev() {
        match stmt.statement {
            AstStatement::Declaration(ast_variable, wrapped) => todo!(),
            AstStatement::Assignment(wrapped, wrapped1) => todo!(),
            AstStatement::If(wrapped, wrapped_ast_statements, wrapped_ast_statements1) => todo!(),
            AstStatement::While(wrapped, wrapped_ast_statements) => todo!(),
            AstStatement::For(
                wrapped_ast_statement,
                wrapped,
                wrapped_ast_statement1,
                wrapped_ast_statements,
            ) => todo!(),
            AstStatement::Call(ast_jump_target, wrappeds) => todo!(),
            AstStatement::Label(_) => todo!(),
            AstStatement::Block(wrapped_ast_statements) => todo!(),
            AstStatement::Assembly(_) => todo!(),
            AstStatement::Comment(_) => todo!(),
            AstStatement::Ir(ir_statement) => todo!(),
            AstStatement::Empty => todo!(),

            /* assignment */
            // variables.get(variable_id).unwrap().data_access_ir.unwrap() check if data access is single and access_type is write

            /* statement containable */

            /* etc */

            /* next statements undetectable */
            AstStatement::Return(_)
            | AstStatement::Undefined
            | AstStatement::Goto(_)
            | AstStatement::Exception(_) => {
                used_locations.clear();
                new_body.push(stmt);
                continue;
            }
        }
    }
    new_body.reverse();

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = new_body;
    }
    Ok(())
}

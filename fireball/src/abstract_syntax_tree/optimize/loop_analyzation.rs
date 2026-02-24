use crate::{
    abstract_syntax_tree::{
        Ast, AstFunctionId, AstFunctionVersion, AstStatement, ProcessedOptimization,
        WrappedAstStatement,
    },
    prelude::DecompileError,
};

pub(super) fn analyze_loops(
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

    normalize_statement_list(&mut body);

    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();
        function.body = body;
        function
            .processed_optimizations
            .push(ProcessedOptimization::LoopAnalyzation);
    }

    Ok(())
}

fn normalize_statement_list(stmts: &mut Vec<WrappedAstStatement>) {
    for stmt in stmts.iter_mut() {
        normalize_statement(stmt);
    }
}

fn normalize_statement(stmt: &mut WrappedAstStatement) {
    match &mut stmt.statement {
        AstStatement::If(_, branch_true, branch_false) => {
            normalize_statement_list(branch_true);
            if let Some(branch_false) = branch_false {
                normalize_statement_list(branch_false);
            }
        }
        AstStatement::While(_, body) => {
            normalize_statement_list(body);
        }
        AstStatement::For(init, cond, update, body) => {
            normalize_statement(init);
            normalize_statement(update);
            normalize_statement_list(body);
            if is_noop_statement(init.as_ref()) && is_noop_statement(update.as_ref()) {
                stmt.statement = AstStatement::While(cond.clone(), std::mem::take(body));
            }
        }
        AstStatement::Block(body) => {
            normalize_statement_list(body);
        }
        AstStatement::Declaration(_, _)
        | AstStatement::Assignment(_, _)
        | AstStatement::Return(_)
        | AstStatement::Call(_)
        | AstStatement::Label(_)
        | AstStatement::Goto(_)
        | AstStatement::Assembly(_)
        | AstStatement::Undefined
        | AstStatement::Exception(_)
        | AstStatement::Comment(_)
        | AstStatement::Ir(_)
        | AstStatement::Empty => {}
    }
}

fn is_noop_statement(stmt: &WrappedAstStatement) -> bool {
    matches!(
        &stmt.statement,
        AstStatement::Empty | AstStatement::Comment(_)
    )
}

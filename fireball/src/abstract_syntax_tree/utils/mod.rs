use crate::{
    abstract_syntax_tree::{ArcAstVariableMap, AstStatement, AstVariableId, WrappedAstStatement},
    ir::data::IrData,
    utils::Aos,
};

/// ### Note
/// the inner recursive call uses reversed iteration because it is likely to be at the back
pub fn get_first_arg_undetectable_statement_index<'a>(
    stmts: impl Iterator<Item = &'a WrappedAstStatement>,
) -> Option<usize> {
    for (i, stmt) in stmts.enumerate() {
        match &stmt.statement {
            AstStatement::Call(_, _)
            | AstStatement::Assembly(_)
            | AstStatement::Ir(_)
            | AstStatement::Return(_)
            | AstStatement::Undefined
            | AstStatement::Goto(_)
            | AstStatement::Exception(_) => {
                return Some(i);
            }

            AstStatement::Declaration(_, _)
            | AstStatement::Assignment(_, _)
            | AstStatement::Label(_)
            | AstStatement::Comment(_)
            | AstStatement::Empty => continue,

            AstStatement::If(_cond, branch_true, branch_false) => {
                if get_first_arg_undetectable_statement_index(branch_true.iter().rev()).is_some() {
                    return Some(i);
                }
                if let Some(branch_false) = branch_false {
                    if get_first_arg_undetectable_statement_index(branch_false.iter().rev())
                        .is_some()
                    {
                        return Some(i);
                    }
                }
            }
            AstStatement::While(_cond, stmts) => {
                if get_first_arg_undetectable_statement_index(stmts.iter().rev()).is_some() {
                    return Some(i);
                }
            }
            AstStatement::For(_init, _cond, _update, stmts) => {
                if get_first_arg_undetectable_statement_index(stmts.iter().rev()).is_some() {
                    return Some(i);
                }
            }
            AstStatement::Block(stmts) => {
                if get_first_arg_undetectable_statement_index(stmts.iter().rev()).is_some() {
                    return Some(i);
                }
            }
        }
    }
    None
}

pub fn var_id_to_access_location(
    variables: &ArcAstVariableMap,
    var_id: AstVariableId,
) -> Aos<IrData> {
    let variables = variables.read().unwrap();
    let data_accesses = variables
        .get(&var_id)
        .and_then(|var| var.data_access_ir.as_ref())
        .expect("manually manipulated variable maps?");

    #[cfg(debug_assertions)]
    {
        let mut t_location = None;
        data_accesses.values().flat_map(|x| x.iter()).for_each(|x| {
            let location = x.location();
            if let Some(loc) = &t_location {
                debug_assert_eq!(
                    loc, location,
                    "variables all data access should have same location, but found different for variable {:?}",
                    variables.get(&var_id).unwrap()
                );
            } else {
                t_location = Some(location.clone());
            }
        });
    }

    let location = data_accesses
        .values()
        .flat_map(|x| x.iter())
        .next()
        .expect("all variable should have at least data access")
        .location();
    location.clone()
}

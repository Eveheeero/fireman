use crate::{
    abstract_syntax_tree::{
        ArcAstVariableMap, Ast, AstExpression, AstFunctionId, AstFunctionVersion, AstStatement,
        AstVariableId, WrappedAstStatement,
    },
    ir::data::IrData,
    prelude::{DecompileError, *},
    utils::Aos,
};
use hashbrown::HashSet;

/// check variables are overwritten without reading in ir level
pub(super) fn collapse_unused_variables(
    ast: &mut Ast,
    function_id: AstFunctionId,
    function_version: AstFunctionVersion,
) -> Result<(), DecompileError> {
    let body;
    let variables;
    {
        let mut functions = ast.functions.write().unwrap();
        let function = functions
            .get_mut(&function_id)
            .and_then(|x| x.get_mut(&function_version))
            .unwrap();

        body = std::mem::take(&mut function.body);
        variables = function.variables.clone();
    }

    let mut overwritten_locations: HashSet<Aos<IrData>> = HashSet::new();
    let mut new_body: Vec<WrappedAstStatement> = Vec::new();
    for mut stmt in body.into_iter().rev() {
        match &mut stmt.statement {
            /* removable */
            AstStatement::Declaration(lhs, _rhs) => {
                let data_access_count: usize = lhs
                    .data_access_ir
                    .as_ref()
                    .unwrap()
                    .values()
                    .map(|x| x.len())
                    .sum();
                let var_id = &lhs.id;
                let location = super::utils::var_id_to_access_location(&variables, *var_id);
                let overwritten = overwritten_locations.contains(&location);
                if data_access_count == 1 && overwritten {
                    trace!(?lhs,?stmt.comment, "Removing declaration of unused variable");
                    continue;
                }
                overwritten_locations.insert(location.clone());
                new_body.push(stmt);
                continue;
            }
            AstStatement::Assignment(lhs, _rhs) => {
                let AstExpression::Variable(_, var_id) = lhs.item else {
                    new_body.push(stmt);
                    continue;
                };
                let location = super::utils::var_id_to_access_location(&variables, var_id);
                let variables = variables.read().unwrap();
                let lhs = variables.get(&var_id).unwrap();
                let data_access_count: usize = lhs
                    .data_access_ir
                    .as_ref()
                    .unwrap()
                    .values()
                    .map(|x| x.len())
                    .sum();
                let overwritten = overwritten_locations.contains(&location);
                if data_access_count == 1 && overwritten {
                    trace!(?lhs,?stmt.comment, "Removing assignment of unused variable");
                    continue;
                }
                overwritten_locations.insert(location.clone());
                new_body.push(stmt);
                continue;
            }

            /* statement containable */
            AstStatement::If(_cond, branch_true, branch_false) => {
                let Some(branch_false) = branch_false else {
                    collapse(&variables, &mut overwritten_locations, branch_true);
                    if branch_true.is_empty() {
                        continue;
                    }
                    new_body.push(stmt);
                    continue;
                };

                let mut b1_overwritten_locations = overwritten_locations.clone();
                collapse(&variables, &mut b1_overwritten_locations, branch_true);

                let mut b2_overwritten_locations = overwritten_locations;
                collapse(&variables, &mut b2_overwritten_locations, branch_false);

                overwritten_locations = b1_overwritten_locations
                    .intersection(&b2_overwritten_locations)
                    .cloned()
                    .collect();

                if branch_true.is_empty() && branch_false.is_empty() {
                    continue;
                }
                new_body.push(stmt);

                continue;
            }
            AstStatement::While(_cond, _stmts) => todo!("same with `for`"),
            AstStatement::For(_init, _cond, _update, _stmts) => todo!(
                "if for pattern used, there might be user-defined or optimization variable exists. how should we handle this?"
            ),
            AstStatement::Block(stmts) => {
                collapse(&variables, &mut overwritten_locations, stmts);
                new_body.push(stmt);
                continue;
            }

            /* etc */
            AstStatement::Label(_) | AstStatement::Comment(_) | AstStatement::Empty => {
                new_body.push(stmt);
                continue;
            }

            /* next statements undetectable */
            AstStatement::Call(_, _)
            | AstStatement::Goto(_)
            | AstStatement::Assembly(_)
            | AstStatement::Ir(_)
            | AstStatement::Return(_)
            | AstStatement::Undefined
            | AstStatement::Exception(_) => {
                overwritten_locations.clear();
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

/// stmts containable stmt handling is different
fn collapse(
    variables: &ArcAstVariableMap,
    overwritten_locations: &mut HashSet<Aos<IrData>>,
    stmts: &mut Vec<WrappedAstStatement>,
) {
    let mut i = stmts.len();
    while i > 0 {
        i -= 1;
        let mut drop_needed = false;
        let stmt = &mut stmts[i];

        match &mut stmt.statement {
            /* removable */
            AstStatement::Declaration(lhs, _rhs) => {
                let data_access_count: usize = lhs
                    .data_access_ir
                    .as_ref()
                    .unwrap()
                    .values()
                    .map(|x| x.len())
                    .sum();
                let var_id = &lhs.id;
                let location = super::utils::var_id_to_access_location(&variables, *var_id);
                let overwritten = overwritten_locations.contains(&location);
                if data_access_count == 1 && overwritten {
                    trace!(?lhs,?stmt.comment, "Removing declaration of unused variable");
                    drop_needed = true;
                } else {
                    overwritten_locations.insert(location.clone());
                }
            }
            AstStatement::Assignment(lhs, _rhs) => {
                if let AstExpression::Variable(_, var_id) = lhs.item {
                    let location = super::utils::var_id_to_access_location(&variables, var_id);
                    let variables = variables.read().unwrap();
                    let lhs = variables.get(&var_id).unwrap();
                    let data_access_count: usize = lhs
                        .data_access_ir
                        .as_ref()
                        .unwrap()
                        .values()
                        .map(|x| x.len())
                        .sum();
                    let overwritten = overwritten_locations.contains(&location);
                    if data_access_count == 1 && overwritten {
                        trace!(?lhs,?stmt.comment, "Removing assignment of unused variable");
                        drop_needed = true;
                    } else {
                        overwritten_locations.insert(location.clone());
                    }
                }
            }

            /* stmts containable */
            AstStatement::If(_cond, branch_true, branch_false) => {
                if let Some(branch_false) = branch_false {
                    let mut b1_overwritten_locations = overwritten_locations.clone();
                    collapse(&variables, &mut b1_overwritten_locations, branch_true);

                    let mut b2_overwritten_locations = [].into();
                    std::mem::swap(&mut b2_overwritten_locations, overwritten_locations);
                    collapse(&variables, &mut b2_overwritten_locations, branch_false);

                    std::mem::swap(
                        overwritten_locations,
                        &mut b1_overwritten_locations
                            .intersection(&b2_overwritten_locations)
                            .cloned()
                            .collect(),
                    );

                    if branch_true.is_empty() && branch_false.is_empty() {
                        drop_needed = true;
                    }
                } else {
                    collapse(&variables, overwritten_locations, branch_true);
                }
            }
            AstStatement::While(_cond, _stmts) => todo!("same with `for`"),
            AstStatement::For(_init, _cond, _update, _stmts) => todo!(
                "if for pattern used, there might be user-defined or optimization variable exists. how should we handle this?"
            ),
            AstStatement::Block(stmts) => {
                collapse(variables, overwritten_locations, stmts);
                if stmts.is_empty() {
                    drop_needed = true;
                }
            }

            /* etc */
            AstStatement::Label(_) | AstStatement::Comment(_) | AstStatement::Empty => {}

            /* next statements undetectable */
            AstStatement::Call(_, _)
            | AstStatement::Goto(_)
            | AstStatement::Assembly(_)
            | AstStatement::Ir(_)
            | AstStatement::Return(_)
            | AstStatement::Undefined
            | AstStatement::Exception(_) => {
                overwritten_locations.clear();
                // newly overwritten won't clear, it will use in out of recursive calls
            }
        }

        if drop_needed {
            stmts.remove(i);
        }
    }
}

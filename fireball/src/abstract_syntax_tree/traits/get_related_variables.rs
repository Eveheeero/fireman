use super::*;

impl GetRelatedVariables for AstStatement {
    fn get_related_variables(&self) -> Vec<(AstVariableAccessType, AstVariableId)> {
        match self {
            AstStatement::Declaration(var, init) => {
                let mut ret = Vec::new();
                ret.push((AstVariableAccessType::Write, var.id.clone()));
                if let Some(init) = init {
                    for var_id in init.get_related_variables() {
                        ret.push((AstVariableAccessType::Read, var_id));
                    }
                }
                ret
            }
            AstStatement::Assignment(lhs, rhs) => {
                let mut ret = Vec::new();
                for var_id in lhs.get_related_variables() {
                    ret.push((AstVariableAccessType::Write, var_id));
                }
                for var_id in rhs.get_related_variables() {
                    ret.push((AstVariableAccessType::Read, var_id));
                }
                ret
            }
            AstStatement::If(cond, branch_true, branch_false) => {
                let mut ret = Vec::new();
                for var_id in cond.get_related_variables() {
                    ret.push((AstVariableAccessType::Read, var_id));
                }
                for stmt in branch_true {
                    ret.extend(stmt.get_related_variables());
                }
                if let Some(branch_false) = branch_false {
                    for stmt in branch_false {
                        ret.extend(stmt.get_related_variables());
                    }
                }
                ret
            }
            AstStatement::While(cond, stmts) => {
                let mut ret = Vec::new();
                for var_id in cond.get_related_variables() {
                    ret.push((AstVariableAccessType::Read, var_id));
                }
                for stmt in stmts {
                    ret.extend(stmt.get_related_variables());
                }
                ret
            }
            AstStatement::For(init, cond, update, stmts) => {
                let mut ret = init.get_related_variables();
                for var_id in cond.get_related_variables() {
                    ret.push((AstVariableAccessType::Read, var_id));
                }
                ret.extend(update.get_related_variables());
                for stmt in stmts {
                    ret.extend(stmt.get_related_variables());
                }
                ret
            }
            AstStatement::Return(val) => {
                if let Some(val) = val {
                    let mut ret = Vec::new();
                    for var_id in val.get_related_variables() {
                        ret.push((AstVariableAccessType::Read, var_id));
                    }
                    ret
                } else {
                    Vec::new()
                }
            }
            AstStatement::Call(target, args) => {
                let mut ret = target.get_related_variables();
                for var_id in args.iter().flat_map(|x| x.get_related_variables()) {
                    ret.push((AstVariableAccessType::Read, var_id));
                }
                ret
            }
            AstStatement::Goto(target) => target.get_related_variables(),
            AstStatement::Block(stmts) => stmts
                .iter()
                .flat_map(|stmt| stmt.get_related_variables())
                .collect(),

            AstStatement::Label(_)
            | AstStatement::Assembly(_)
            | AstStatement::Undefined
            | AstStatement::Exception(_)
            | AstStatement::Comment(_)
            | AstStatement::Ir(_)
            | AstStatement::Empty => Vec::new(),
        }
    }
}

impl GetRelatedVariables for AstJumpTarget {
    fn get_related_variables(&self) -> Vec<(AstVariableAccessType, AstVariableId)> {
        match self {
            AstJumpTarget::Variable { var_id, .. } => {
                [(AstVariableAccessType::Read, var_id.clone())].into()
            }
            AstJumpTarget::Function { .. }
            | AstJumpTarget::Instruction { .. }
            | AstJumpTarget::Unknown(_) => Vec::new(),
        }
    }
}

use crate::ir::{
    data::{AccessSize, DataAccess, DataAccessType},
    statements::{IrStatement, IrStatementSpecial},
    Ir,
};

pub fn analyze_data_access(ir: &Ir) -> Vec<DataAccess> {
    if ir.statements.is_none() {
        return Vec::new();
    }
    let mut result = Vec::new();
    for statement in ir.statements.as_ref().unwrap().iter() {
        analyze_data_access_raw(&mut result, statement);
    }
    result
}

pub fn analyze_data_access_raw(v: &mut Vec<DataAccess>, statement: &IrStatement) {
    match statement {
        IrStatement::Assignment { from, to, size } => {
            v.push(DataAccess::new(
                from.clone(),
                DataAccessType::Read,
                size.clone(),
            ));
            v.push(DataAccess::new(
                to.clone(),
                DataAccessType::Write,
                size.clone(),
            ));
            match size {
                AccessSize::ResultOfBit(aos) | AccessSize::ResultOfByte(aos) => {
                    v.push(DataAccess::new(
                        aos.clone(),
                        DataAccessType::Read,
                        AccessSize::Unlimited,
                    ));
                }
                AccessSize::RelativeWith(_)
                | AccessSize::ArchitectureSize
                | AccessSize::Unlimited => {}
            }
        }
        IrStatement::Jump { target } | IrStatement::Call { target } => {
            v.push(DataAccess::new(
                target.clone(),
                DataAccessType::Read,
                AccessSize::ArchitectureSize,
            ));
        }
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            v.push(DataAccess::new(
                condition.clone(),
                DataAccessType::Read,
                AccessSize::Unlimited,
            ));
            for statement in true_branch.iter().chain(false_branch.iter()) {
                analyze_data_access_raw(v, statement);
            }
        }
        IrStatement::Special(IrStatementSpecial::ArchitectureByteSizeCondition {
            condition: _,
            true_branch,
            false_branch,
        }) => {
            for statement in true_branch.iter().chain(false_branch.iter()) {
                analyze_data_access_raw(v, statement);
            }
        }

        IrStatement::Undefined
        | IrStatement::Exception(_)
        | IrStatement::Halt
        | IrStatement::Special(IrStatementSpecial::Assertion { .. })
        | IrStatement::Special(IrStatementSpecial::TypeSpecified { .. })
        | IrStatement::Special(IrStatementSpecial::CalcFlagsAutomatically { .. }) => {}
    }
}

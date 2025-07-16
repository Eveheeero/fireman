use crate::{
    ir::{
        Ir,
        data::{IrAccessSize, IrDataAccess, IrDataAccessType},
        statements::{IrStatement, IrStatementSpecial},
        utils::{IrStatementDescriptor, IrStatementDescriptorMap},
    },
    prelude::*,
};

pub fn analyze_data_access(
    out: &mut IrStatementDescriptorMap<Vec<IrDataAccess>>,
    ir_index: u32,
    ir: &Ir,
) {
    if ir.statements.is_none() {
        return;
    }
    for (statement_index, statement) in ir.statements.as_ref().unwrap().iter().enumerate() {
        let statement_index = statement_index as u8;
        let mut now = Vec::new();
        let mut insert = |x| {
            trace!("Inserting data access {}", x);
            now.push(x);
        };
        trace!("Analyzing data access for statement {}", statement);
        analyze_data_access_raw(&mut insert, statement);
        now.shrink_to_fit();
        out.insert(IrStatementDescriptor::new(ir_index, statement_index), now);
    }
}

pub fn analyze_data_access_raw(insert: &mut impl FnMut(IrDataAccess), statement: &IrStatement) {
    match statement {
        IrStatement::Assignment { from, to, size } => {
            insert(IrDataAccess::new(
                from.clone(),
                IrDataAccessType::Read,
                size.clone(),
            ));
            insert(IrDataAccess::new(
                to.clone(),
                IrDataAccessType::Write,
                size.clone(),
            ));
            match size {
                IrAccessSize::ResultOfBit(aos) | IrAccessSize::ResultOfByte(aos) => {
                    insert(IrDataAccess::new(
                        aos.clone(),
                        IrDataAccessType::Read,
                        IrAccessSize::Unlimited,
                    ));
                }
                IrAccessSize::RelativeWith(_)
                | IrAccessSize::ArchitectureSize
                | IrAccessSize::Unlimited => {}
            }
        }
        IrStatement::Jump { target } | IrStatement::JumpByCall { target } => {
            insert(IrDataAccess::new(
                target.clone(),
                IrDataAccessType::Read,
                IrAccessSize::ArchitectureSize,
            ));
        }
        IrStatement::Condition {
            condition,
            true_branch,
            false_branch,
        } => {
            insert(IrDataAccess::new(
                condition.clone(),
                IrDataAccessType::Read,
                IrAccessSize::Unlimited,
            ));
            for statement in true_branch.iter().chain(false_branch.iter()) {
                analyze_data_access_raw(insert, statement);
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

use crate::{
    ir::{
        Ir,
        data::{AccessSize, DataAccess, DataAccessType},
        statements::{IrStatement, IrStatementSpecial},
        utils::{IrStatementDescriptor, IrStatementDescriptorMap},
    },
    prelude::*,
};

pub fn analyze_data_access(
    out: &mut IrStatementDescriptorMap<Vec<DataAccess>>,
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

pub fn analyze_data_access_raw(insert: &mut impl FnMut(DataAccess), statement: &IrStatement) {
    match statement {
        IrStatement::Assignment { from, to, size } => {
            insert(DataAccess::new(
                from.clone(),
                DataAccessType::Read,
                size.clone(),
            ));
            insert(DataAccess::new(
                to.clone(),
                DataAccessType::Write,
                size.clone(),
            ));
            match size {
                AccessSize::ResultOfBit(aos) | AccessSize::ResultOfByte(aos) => {
                    insert(DataAccess::new(
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
        IrStatement::Jump { target } | IrStatement::JumpByCall { target } => {
            insert(DataAccess::new(
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
            insert(DataAccess::new(
                condition.clone(),
                DataAccessType::Read,
                AccessSize::Unlimited,
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
        IrStatement::Atomic { statement, .. } => {
            analyze_data_access_raw(insert, statement);
        }
        IrStatement::AtomicLoad { address, .. } => {
            insert(DataAccess::new(
                address.clone(),
                DataAccessType::Read,
                AccessSize::ArchitectureSize,
            ));
        }
        IrStatement::AtomicStore { address, value, .. } => {
            insert(DataAccess::new(
                address.clone(),
                DataAccessType::Read,
                AccessSize::ArchitectureSize,
            ));
            insert(DataAccess::new(
                value.clone(),
                DataAccessType::Read,
                AccessSize::Unlimited,
            ));
        }
        IrStatement::AtomicRmw { address, value, .. } => {
            insert(DataAccess::new(
                address.clone(),
                DataAccessType::Read,
                AccessSize::ArchitectureSize,
            ));
            insert(DataAccess::new(
                value.clone(),
                DataAccessType::Read,
                AccessSize::Unlimited,
            ));
        }
        IrStatement::AtomicCompareExchange {
            address,
            expected,
            desired,
            ..
        } => {
            insert(DataAccess::new(
                address.clone(),
                DataAccessType::Read,
                AccessSize::ArchitectureSize,
            ));
            insert(DataAccess::new(
                expected.clone(),
                DataAccessType::Read,
                AccessSize::Unlimited,
            ));
            insert(DataAccess::new(
                desired.clone(),
                DataAccessType::Read,
                AccessSize::Unlimited,
            ));
        }
        IrStatement::Fence { .. } => {
            // Fence has no data access
        }
    }
}

use crate::{
    ir::{
        Ir,
        data::{AccessSize, IrData, IrDataContainable},
        statements::{IrStatement, IrStatementSpecial},
        utils::{IrStatementDescriptor, IrStatementDescriptorMap},
    },
    prelude::*,
    utils::Aos,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KnownDataType {
    pub location: Aos<IrData>,
    pub data_type: DataType,
    pub data_size: AccessSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Unknown,
    Bool,
    Int,
    Float,
    StringPointer,
    Char,
    Address,
}

pub fn analyze_datatype(
    out: &mut IrStatementDescriptorMap<Vec<KnownDataType>>,
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
            trace!("Inserting data type {}", x);
            now.push(x);
        };
        trace!("Analyzing data access for statement {}", statement);
        analyze_datatype_raw(&mut insert, statement);
        now.shrink_to_fit();
        out.insert(IrStatementDescriptor::new(ir_index, statement_index), now);
    }
}

/// ### TODO
/// 인스트럭션을 통한 데이터 타입 추가 유추 필요
pub fn analyze_datatype_raw(insert: &mut impl FnMut(KnownDataType), statement: &IrStatement) {
    match statement {
        IrStatement::Assignment { from, to, size } => {
            insert(KnownDataType {
                location: from.clone(),
                data_type: DataType::Unknown,
                data_size: size.clone(),
            });
            insert(KnownDataType {
                location: to.clone(),
                data_type: DataType::Unknown,
                data_size: size.clone(),
            });
        }
        IrStatement::Jump { target } => {
            insert(KnownDataType {
                location: target.clone(),
                data_type: DataType::Address,
                data_size: AccessSize::ArchitectureSize,
            });
        }
        IrStatement::JumpByCall { target } => {
            insert(KnownDataType {
                location: target.clone(),
                data_type: DataType::Address,
                data_size: AccessSize::ArchitectureSize,
            });
        }
        IrStatement::Condition {
            condition: _,
            true_branch,
            false_branch,
        } => {
            for statement in true_branch.iter().chain(false_branch.iter()) {
                analyze_datatype_raw(insert, statement);
            }
        }
        IrStatement::Special(IrStatementSpecial::TypeSpecified {
            location,
            size,
            data_type,
        }) => {
            insert(KnownDataType {
                location: location.clone(),
                data_type: *data_type,
                data_size: size.clone(),
            });
        }

        IrStatement::Undefined
        | IrStatement::Exception(_)
        | IrStatement::Halt
        | IrStatement::Special(IrStatementSpecial::Assertion { .. })
        | IrStatement::Special(IrStatementSpecial::CalcFlagsAutomatically { .. }) => {}
        IrStatement::Atomic { statement, .. } => {
            analyze_datatype_raw(insert, statement);
        }
    }
}

impl IrDataContainable for KnownDataType {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        self.location.get_related_ir_data(v);
        v.push(&self.location);
        self.data_size.get_related_ir_data(v);
    }
}

impl std::fmt::Display for KnownDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.location, self.data_type, self.data_size)
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Unknown => write!(f, "u"),
            DataType::Bool => write!(f, "b"),
            DataType::Int => write!(f, "i"),
            DataType::Float => write!(f, "f"),
            DataType::StringPointer => write!(f, "*c"),
            DataType::Char => write!(f, "c"),
            DataType::Address => write!(f, "*"),
        }
    }
}

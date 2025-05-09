use crate::{
    ir::{
        data::{AccessSize, IrData, IrDataContainable},
        statements::{IrStatement, IrStatementSpecial},
        Ir,
    },
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
    Int,
    Float,
    StringPointer,
    Char,
    Address,
}

pub fn analyze_datatype(ir: &Ir) -> Vec<KnownDataType> {
    if ir.statements.is_none() {
        return Vec::new();
    }
    let mut result = Vec::new();
    for statement in ir.statements.as_ref().unwrap().iter() {
        analyze_datatype_raw(&mut result, statement);
    }
    result
}

/// ### TODO
/// 인스트럭션을 통한 데이터 타입 추가 유추 필요
pub fn analyze_datatype_raw(v: &mut Vec<KnownDataType>, statement: &IrStatement) {
    match statement {
        IrStatement::Assignment { from, to, size } => {
            v.push(KnownDataType {
                location: from.clone(),
                data_type: DataType::Unknown,
                data_size: size.clone(),
            });
            v.push(KnownDataType {
                location: to.clone(),
                data_type: DataType::Unknown,
                data_size: size.clone(),
            });
        }
        IrStatement::Jump { target } => {
            v.push(KnownDataType {
                location: target.clone(),
                data_type: DataType::Address,
                data_size: AccessSize::ArchitectureSize,
            });
        }
        IrStatement::Call { target } => {
            v.push(KnownDataType {
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
                analyze_datatype_raw(v, statement);
            }
        }
        IrStatement::Special(IrStatementSpecial::TypeSpecified {
            location,
            size,
            data_type,
        }) => {
            v.push(KnownDataType {
                location: location.clone(),
                data_type: *data_type,
                data_size: size.clone(),
            });
        }
        IrStatement::Special(IrStatementSpecial::ArchitectureByteSizeCondition {
            condition: _,
            true_branch,
            false_branch,
        }) => {
            for statement in true_branch.iter().chain(false_branch.iter()) {
                analyze_datatype_raw(v, statement);
            }
        }

        IrStatement::Undefined
        | IrStatement::Exception(_)
        | IrStatement::Halt
        | IrStatement::Special(IrStatementSpecial::Assertion { .. })
        | IrStatement::Special(IrStatementSpecial::CalcFlagsAutomatically { .. }) => {}
    }
}

impl IrDataContainable for KnownDataType {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        self.location.get_related_ir_data(v);
        v.push(&self.location);
        self.data_size.get_related_ir_data(v);
    }
}

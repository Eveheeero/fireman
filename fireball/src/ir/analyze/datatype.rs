use crate::{
    core::Address,
    ir::{
        data::{AccessSize, IrData, IrDataContainable},
        statements::IrStatement,
        Ir,
    },
    utils::Aos,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KnownDataType {
    pub shown_in: Address,
    pub location: Aos<IrData>,
    pub data_type: DataType,
    /// None if size depends on architecture
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
    if ir.statements.is_right() {
        return Vec::new();
    }
    let mut result = Vec::new();
    let address = &ir.address;
    for statement in ir.statements.as_ref().unwrap_left().iter() {
        analyze_datatype_raw(&mut result, address, statement);
    }
    result
}

/// ### TODO
/// 인스트럭션을 통한 데이터 타입 추가 유추 필요
pub fn analyze_datatype_raw(
    v: &mut Vec<KnownDataType>,
    address: &Address,
    statement: &IrStatement,
) {
    match statement {
        crate::ir::statements::IrStatement::Assignment { from, to, size } => {
            v.push(KnownDataType {
                shown_in: address.clone(),
                location: from.clone(),
                data_type: DataType::Unknown,
                data_size: size.clone(),
            });
            v.push(KnownDataType {
                shown_in: address.clone(),
                location: to.clone(),
                data_type: DataType::Unknown,
                data_size: size.clone(),
            });
        }
        crate::ir::statements::IrStatement::Jump { target } => {
            v.push(KnownDataType {
                shown_in: address.clone(),
                location: target.clone(),
                data_type: DataType::Address,
                data_size: AccessSize::ArchitectureSize,
            });
        }
        crate::ir::statements::IrStatement::Call { target } => {
            v.push(KnownDataType {
                shown_in: address.clone(),
                location: target.clone(),
                data_type: DataType::Address,
                data_size: AccessSize::ArchitectureSize,
            });
        }
        crate::ir::statements::IrStatement::Condition {
            condition: _,
            true_branch,
            false_branch,
        } => {
            for statement in true_branch.iter().chain(false_branch.iter()) {
                analyze_datatype_raw(v, address, statement);
            }
        }
        crate::ir::statements::IrStatement::Special(
            crate::ir::statements::IrStatementSpecial::TypeSpecified {
                location,
                size,
                data_type,
            },
        ) => {
            v.push(KnownDataType {
                shown_in: address.clone(),
                location: location.clone(),
                data_type: *data_type,
                data_size: size.clone(),
            });
        }
        _ => {}
    }
}

impl IrDataContainable for KnownDataType {
    fn get_related_ir_data(&self, v: &mut Vec<Aos<IrData>>) {
        self.location.get_related_ir_data(v);
        v.push(self.location.clone());
        self.data_size.get_related_ir_data(v);
    }
}

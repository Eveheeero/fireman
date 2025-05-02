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
    analyze_datatype_raw(
        &ir.address,
        ir.statements
            .as_ref()
            .expect_left("분석에 실패한 IR데이터가 입력되었습니다."),
    )
}

/// ### TODO
/// 인스트럭션을 통한 데이터 타입 추가 유추 필요
pub fn analyze_datatype_raw(address: &Address, statements: &[IrStatement]) -> Vec<KnownDataType> {
    let mut known_datatypes: Vec<KnownDataType> = Vec::new();
    for statement in statements.iter() {
        match statement {
            crate::ir::statements::IrStatement::Assignment { from, to, size } => {
                known_datatypes.push(KnownDataType {
                    shown_in: address.clone(),
                    location: from.clone(),
                    data_type: DataType::Unknown,
                    data_size: size.clone(),
                });
                known_datatypes.push(KnownDataType {
                    shown_in: address.clone(),
                    location: to.clone(),
                    data_type: DataType::Unknown,
                    data_size: size.clone(),
                });
            }
            crate::ir::statements::IrStatement::Jump { target } => {
                known_datatypes.push(KnownDataType {
                    shown_in: address.clone(),
                    location: target.clone(),
                    data_type: DataType::Address,
                    data_size: AccessSize::ArchitectureSize,
                });
            }
            crate::ir::statements::IrStatement::Call { target } => {
                known_datatypes.push(KnownDataType {
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
                let true_branch = analyze_datatype_raw(address, true_branch);
                let false_branch = analyze_datatype_raw(address, false_branch);
                known_datatypes.extend(true_branch);
                known_datatypes.extend(false_branch);
            }
            crate::ir::statements::IrStatement::Special(
                crate::ir::statements::IrStatementSpecial::TypeSpecified {
                    location,
                    size,
                    data_type,
                },
            ) => {
                known_datatypes.push(KnownDataType {
                    shown_in: address.clone(),
                    location: location.clone(),
                    data_type: *data_type,
                    data_size: size.clone(),
                });
            }
            _ => continue,
        }
    }
    known_datatypes
}

impl IrDataContainable for KnownDataType {
    fn get_related_ir_data(&self, v: &mut Vec<Aos<IrData>>) {
        self.location.get_related_ir_data(v);
        v.push(self.location.clone());
        self.data_size.get_related_ir_data(v);
    }
}

use crate::{
    core::Address,
    ir::{data::IRData, statements::IRStatement, Ir},
};
use std::num::NonZeroU16;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KnownDataType {
    pub shown_in: Address,
    pub location: IRData,
    pub data_type: DataType,
    /// None if size depends on architecture
    pub data_size: Option<NonZeroU16>,
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
    analyze_datatype_raw(&ir.address, &ir.statements)
}

/// ### TODO
/// 인스트럭션을 통한 데이터 타입 추가 유추 필요
pub fn analyze_datatype_raw(address: &Address, statements: &[IRStatement]) -> Vec<KnownDataType> {
    let mut known_datatypes: Vec<KnownDataType> = Vec::new();
    for statement in statements.iter() {
        match statement {
            crate::ir::statements::IRStatement::Assignment { from, to, size } => {
                known_datatypes.push(KnownDataType {
                    shown_in: address.clone(),
                    location: from.clone(),
                    data_type: DataType::Unknown,
                    data_size: Some(*size),
                });
                known_datatypes.push(KnownDataType {
                    shown_in: address.clone(),
                    location: to.clone(),
                    data_type: DataType::Unknown,
                    data_size: Some(*size),
                });
            }
            crate::ir::statements::IRStatement::Jump(irstatement_jump) => match irstatement_jump {
                crate::ir::statements::IRStatementJump::Conditional { ok, fail } => {
                    known_datatypes.push(KnownDataType {
                        shown_in: address.clone(),
                        location: ok.clone(),
                        data_type: DataType::Address,
                        data_size: None,
                    });
                    known_datatypes.push(KnownDataType {
                        shown_in: address.clone(),
                        location: fail.clone(),
                        data_type: DataType::Address,
                        data_size: None,
                    });
                }
                crate::ir::statements::IRStatementJump::Unconditional(irdata) => {
                    known_datatypes.push(KnownDataType {
                        shown_in: address.clone(),
                        location: irdata.clone(),
                        data_type: DataType::Address,
                        data_size: None,
                    });
                }
            },
            crate::ir::statements::IRStatement::Call { target } => {
                known_datatypes.push(KnownDataType {
                    shown_in: address.clone(),
                    location: target.clone(),
                    data_type: DataType::Address,
                    data_size: None,
                });
            }
            crate::ir::statements::IRStatement::Touch { data, size, .. } => {
                known_datatypes.push(KnownDataType {
                    shown_in: address.clone(),
                    location: data.clone(),
                    data_type: DataType::Unknown,
                    data_size: Some(*size),
                });
            }
            crate::ir::statements::IRStatement::Condition {
                condition,
                size,
                true_branch,
                false_branch,
            } => {
                known_datatypes.push(KnownDataType {
                    shown_in: address.clone(),
                    location: condition.clone(),
                    data_type: DataType::Unknown,
                    data_size: Some(*size),
                });
                let true_branch = analyze_datatype_raw(address, true_branch);
                let false_branch = analyze_datatype_raw(address, false_branch);
                known_datatypes.extend(true_branch);
                known_datatypes.extend(false_branch);
            }
            _ => continue,
        }
    }
    known_datatypes
}

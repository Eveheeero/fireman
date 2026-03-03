use crate::{
    ir::{
        Ir,
        data::{IrAccessSize, IrData, IrDataContainable},
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
    pub data_size: IrAccessSize,
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
            trace!("- {}", x);
            now.push(x);
        };
        trace!("{}", statement);
        analyze_datatype_raw(&mut insert, statement);
        now.shrink_to_fit();
        out.insert(IrStatementDescriptor::new(ir_index, statement_index), now);
    }
}

pub fn analyze_datatype_raw(insert: &mut impl FnMut(KnownDataType), statement: &IrStatement) {
    match statement {
        IrStatement::Assignment { from, to, size } => {
            let inferred = infer_type_from_data(from);
            insert(KnownDataType {
                location: from.clone(),
                data_type: inferred,
                data_size: size.clone(),
            });
            insert(KnownDataType {
                location: to.clone(),
                data_type: inferred,
                data_size: size.clone(),
            });
            infer_operand_types(insert, from);
        }
        IrStatement::Jump { target } => {
            insert(KnownDataType {
                location: target.clone(),
                data_type: DataType::Address,
                data_size: IrAccessSize::ArchitectureSize,
            });
        }
        IrStatement::JumpByCall { target } => {
            insert(KnownDataType {
                location: target.clone(),
                data_type: DataType::Address,
                data_size: IrAccessSize::ArchitectureSize,
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
    }
}

use crate::ir::{
    data::IrDataOperation,
    operator::{IrBinaryOperator, IrUnaryOperator},
};

fn infer_type_from_data(data: &Aos<IrData>) -> DataType {
    match data.as_ref() {
        IrData::Operation(IrDataOperation::Binary { operator, .. }) => match operator {
            IrBinaryOperator::Equal(_)
            | IrBinaryOperator::SignedLess(_)
            | IrBinaryOperator::SignedLessOrEqual(_)
            | IrBinaryOperator::UnsignedLess(_)
            | IrBinaryOperator::UnsignedLessOrEqual(_) => DataType::Bool,
            IrBinaryOperator::SignedDiv
            | IrBinaryOperator::SignedRem
            | IrBinaryOperator::Add
            | IrBinaryOperator::Sub
            | IrBinaryOperator::Mul => DataType::Int,
            _ => DataType::Unknown,
        },
        IrData::Operation(IrDataOperation::Unary { operator, .. }) => match operator {
            IrUnaryOperator::Negation => DataType::Int,
            IrUnaryOperator::Not => DataType::Bool,
            _ => DataType::Unknown,
        },
        IrData::Dereference(_) => DataType::Unknown,
        _ => DataType::Unknown,
    }
}

fn infer_operand_types(insert: &mut impl FnMut(KnownDataType), data: &Aos<IrData>) {
    match data.as_ref() {
        IrData::Operation(IrDataOperation::Binary {
            operator,
            arg1,
            arg2,
        }) => {
            let operand_type = match operator {
                IrBinaryOperator::SignedDiv
                | IrBinaryOperator::SignedRem
                | IrBinaryOperator::Sar => DataType::Int,
                IrBinaryOperator::Equal(_)
                | IrBinaryOperator::SignedLess(_)
                | IrBinaryOperator::SignedLessOrEqual(_) => DataType::Int,
                _ => DataType::Unknown,
            };
            if operand_type != DataType::Unknown {
                insert(KnownDataType {
                    location: arg1.clone(),
                    data_type: operand_type,
                    data_size: IrAccessSize::RelativeWith(arg1.clone()),
                });
                insert(KnownDataType {
                    location: arg2.clone(),
                    data_type: operand_type,
                    data_size: IrAccessSize::RelativeWith(arg2.clone()),
                });
            }
            if let IrBinaryOperator::Add { .. } = operator {
                if matches!(arg1.as_ref(), IrData::Dereference(_)) {
                    insert(KnownDataType {
                        location: arg1.clone(),
                        data_type: DataType::Address,
                        data_size: IrAccessSize::RelativeWith(arg1.clone()),
                    });
                }
                if matches!(arg2.as_ref(), IrData::Dereference(_)) {
                    insert(KnownDataType {
                        location: arg2.clone(),
                        data_type: DataType::Address,
                        data_size: IrAccessSize::RelativeWith(arg2.clone()),
                    });
                }
            }
        }
        IrData::Operation(IrDataOperation::Unary { operator, arg }) => {
            let operand_type = match operator {
                IrUnaryOperator::SignExtend => DataType::Int,
                _ => DataType::Unknown,
            };
            if operand_type != DataType::Unknown {
                insert(KnownDataType {
                    location: arg.clone(),
                    data_type: operand_type,
                    data_size: IrAccessSize::RelativeWith(arg.clone()),
                });
            }
        }
        IrData::Dereference(inner) => {
            insert(KnownDataType {
                location: inner.clone(),
                data_type: DataType::Address,
                data_size: IrAccessSize::RelativeWith(inner.clone()),
            });
        }
        _ => {}
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

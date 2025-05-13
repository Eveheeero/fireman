//! IR의 각 명령이 담겨져 있는 모듈

use crate::{
    ir::data::{AccessSize, IrData, IrDataContainable},
    utils::Aos,
};

/// IR의 각 명령에 대한 Enum
///
/// ### Note
/// snowman's expressions.h, StatementBase based classes, or snowman's ir::statement.h classes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrStatement {
    /// 정의되지 않은 명령
    Undefined,
    /// 오류 발생
    Exception(&'static str),
    /// 변수 할당
    Assignment {
        from: Aos<IrData>,
        to: Aos<IrData>,
        size: AccessSize,
    },
    /// 명령 라인 변경
    Jump {
        target: Aos<IrData>,
    },
    /// 함수 호출
    JumpByCall {
        target: Aos<IrData>,
    },
    /// 함수 호출 후 반환
    Halt,
    /// 조건문
    Condition {
        condition: Aos<IrData>,
        true_branch: Box<[IrStatement]>,
        false_branch: Box<[IrStatement]>,
    },
    Special(IrStatementSpecial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrStatementSpecial {
    TypeSpecified {
        location: Aos<IrData>,
        size: AccessSize,
        data_type: crate::ir::analyze::DataType,
    },
    ArchitectureByteSizeCondition {
        condition: NumCondition,
        true_branch: Box<[IrStatement]>,
        false_branch: Box<[IrStatement]>,
    },
    CalcFlagsAutomatically {
        operation: Aos<IrData>,
        size: AccessSize,
        of: bool,
        sf: bool,
        zf: bool,
        af: bool,
        cf: bool,
        pf: bool,
    },
    Assertion {
        condition: Aos<IrData>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum NumCondition {
    Higher(u16),
    HigherOrEqual(u16),
    Lower(u16),
    LowerOrEqual(u16),
    Equal(u16),
    NotEqual(u16),
    Between(u16, u16),

    NotBetween(u16, u16),
}

impl IrDataContainable for IrStatement {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        match self {
            IrStatement::Assignment { from, to, size } => {
                from.get_related_ir_data(v);
                v.push(from);
                to.get_related_ir_data(v);
                v.push(to);
                size.get_related_ir_data(v);
            }
            IrStatement::Jump { target } | IrStatement::JumpByCall { target } => {
                target.get_related_ir_data(v);
                v.push(target);
            }
            IrStatement::Condition {
                condition,
                true_branch,
                false_branch,
            } => {
                condition.get_related_ir_data(v);
                v.push(condition);
                true_branch.iter().for_each(|b| b.get_related_ir_data(v));
                false_branch.iter().for_each(|b| b.get_related_ir_data(v));
            }
            IrStatement::Special(ir_statement_special) => {
                ir_statement_special.get_related_ir_data(v)
            }
            _ => {}
        }
    }
}

impl IrDataContainable for IrStatementSpecial {
    fn get_related_ir_data<'d>(&'d self, v: &mut Vec<&'d Aos<IrData>>) {
        match self {
            IrStatementSpecial::TypeSpecified {
                location,
                size,
                data_type: _,
            } => {
                location.get_related_ir_data(v);
                v.push(location);
                size.get_related_ir_data(v);
            }
            IrStatementSpecial::ArchitectureByteSizeCondition {
                condition: _,
                true_branch,
                false_branch,
            } => {
                true_branch.iter().for_each(|b| b.get_related_ir_data(v));
                false_branch.iter().for_each(|b| b.get_related_ir_data(v));
            }
            IrStatementSpecial::CalcFlagsAutomatically {
                operation, size, ..
            } => {
                operation.get_related_ir_data(v);
                v.push(operation);
                size.get_related_ir_data(v);
            }
            IrStatementSpecial::Assertion { condition } => {
                condition.get_related_ir_data(v);
                v.push(condition);
            }
        }
    }
}

impl std::fmt::Display for IrStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrStatement::Assignment { from, to, size } => {
                write!(f, "{} = ({}){}", to, size, from)
            }
            IrStatement::Jump { target } => write!(f, "jmp {}", target),
            IrStatement::JumpByCall { target } => write!(f, "call {}", target),
            IrStatement::Condition {
                condition,
                true_branch,
                false_branch,
            } => {
                write!(f, "if {}", condition)?;
                write!(f, "{{")?;
                for statement in true_branch {
                    write!(f, "\n    {}", statement)?;
                }
                write!(f, "\n}}")?;
                write!(f, "else {{")?;
                for statement in false_branch {
                    write!(f, "\n    {}", statement)?;
                }
                write!(f, "\n}}")
            }
            IrStatement::Special(ir_statement_special) => write!(f, "{}", ir_statement_special),
            IrStatement::Undefined => write!(f, "undefined"),
            IrStatement::Exception(e) => write!(f, "exception {}", e),
            IrStatement::Halt => write!(f, "halt"),
        }
    }
}

impl std::fmt::Display for IrStatementSpecial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrStatementSpecial::TypeSpecified {
                location,
                size,
                data_type,
            } => {
                write!(f, "type {} = {}", location, size)?;
                write!(f, "{}", data_type)
            }
            IrStatementSpecial::ArchitectureByteSizeCondition {
                condition,
                true_branch,
                false_branch,
            } => {
                write!(f, "if {}", condition)?;
                write!(f, "{{")?;
                for statement in true_branch {
                    write!(f, "\n    {}", statement)?;
                }
                write!(f, "\n}}")?;
                write!(f, "else {{")?;
                for statement in false_branch {
                    write!(f, "\n    {}", statement)?;
                }
                write!(f, "\n}}")
            }
            IrStatementSpecial::CalcFlagsAutomatically {
                operation,
                size: _,
                of,
                sf,
                zf,
                af,
                cf,
                pf,
            } => {
                let mut flags = 0;
                if *of {
                    flags |= 1 << 0;
                }
                if *sf {
                    flags |= 1 << 1;
                }
                if *zf {
                    flags |= 1 << 2;
                }
                if *af {
                    flags |= 1 << 3;
                }
                if *cf {
                    flags |= 1 << 4;
                }
                if *pf {
                    flags |= 1 << 5;
                }
                write!(f, "calc_flags {:#b} ({})", flags, operation)
            }
            IrStatementSpecial::Assertion { condition } => write!(f, "assert ({})", condition),
        }
    }
}
impl std::fmt::Display for NumCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumCondition::Higher(value) => write!(f, "{} > {}", self, value),
            NumCondition::HigherOrEqual(value) => write!(f, "{} >= {}", self, value),
            NumCondition::Lower(value) => write!(f, "{} < {}", self, value),
            NumCondition::LowerOrEqual(value) => write!(f, "{} <= {}", self, value),
            NumCondition::Equal(value) => write!(f, "{} == {}", self, value),
            NumCondition::NotEqual(value) => write!(f, "{} != {}", self, value),
            NumCondition::Between(value1, value2) => {
                write!(f, "{} in [{}..{}]", self, value1, value2)
            }
            NumCondition::NotBetween(value1, value2) => {
                write!(f, "{} not in [{}..{}]", self, value1, value2)
            }
        }
    }
}

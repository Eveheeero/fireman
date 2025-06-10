//! Module containing IR statements

use crate::{
    ir::data::{AccessSize, IrData, IrDataContainable},
    utils::Aos,
};

/// Memory ordering for atomic operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryOrdering {
    /// No synchronization (normal memory access)
    Relaxed,
    /// Acquire semantics (for loads)
    Acquire,
    /// Release semantics (for stores)
    Release,
    /// Both acquire and release semantics
    AcqRel,
    /// Sequential consistency
    SeqCst,
}

/// Enum representing each IR statement
///
/// ### Note
/// snowman's expressions.h, StatementBase based classes, or snowman's ir::statement.h classes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrStatement {
    /// Undefined statement
    Undefined,
    /// Exception occurred
    Exception(&'static str),
    /// Variable assignment
    Assignment {
        from: Aos<IrData>,
        to: Aos<IrData>,
        size: AccessSize,
    },
    /// Jump instruction
    Jump {
        target: Aos<IrData>,
    },
    /// Function call
    JumpByCall {
        target: Aos<IrData>,
    },
    /// Return after function call
    Halt,
    /// Conditional statement
    Condition {
        condition: Aos<IrData>,
        true_branch: Box<[IrStatement]>,
        false_branch: Box<[IrStatement]>,
    },
    Special(IrStatementSpecial),
    /// Atomic operation wrapper
    Atomic {
        statement: Box<IrStatement>,
        ordering: MemoryOrdering,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrStatementSpecial {
    TypeSpecified {
        location: Aos<IrData>,
        size: AccessSize,
        data_type: crate::ir::analyze::DataType,
    },
    CalcFlagsAutomatically {
        operation: Aos<IrData>,
        size: AccessSize,
        flags: Vec<Aos<IrData>>,
    },
    Assertion {
        condition: Aos<IrData>,
    },
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
            IrStatement::Atomic { statement, .. } => statement.get_related_ir_data(v),
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
            IrStatement::Atomic {
                statement,
                ordering,
            } => {
                write!(f, "atomic[{:?}] {{", ordering)?;
                write!(f, "\n    {}", statement)?;
                write!(f, "\n}}")
            }
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
            IrStatementSpecial::CalcFlagsAutomatically {
                operation,
                size: _,
                flags,
            } => {
                let flags = flags
                    .iter()
                    .map(|flag| format!("{}", flag))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "calc_flags [{}] ({})", flags, operation)
            }
            IrStatementSpecial::Assertion { condition } => write!(f, "assert ({})", condition),
        }
    }
}

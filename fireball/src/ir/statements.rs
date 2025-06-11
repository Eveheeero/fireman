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
    /// Atomic load operation
    AtomicLoad {
        result: Aos<IrData>,
        address: Aos<IrData>,
        size: usize,
        ordering: MemoryOrdering,
    },
    /// Atomic store operation
    AtomicStore {
        address: Aos<IrData>,
        value: Aos<IrData>,
        size: usize,
        ordering: MemoryOrdering,
    },
    /// Atomic read-modify-write operation
    AtomicRmw {
        result: Aos<IrData>,
        operation: crate::ir::operator::Operator,
        address: Aos<IrData>,
        value: Aos<IrData>,
        size: usize,
        ordering: MemoryOrdering,
    },
    /// Atomic compare and exchange
    AtomicCompareExchange {
        result: Aos<IrData>,
        address: Aos<IrData>,
        expected: Aos<IrData>,
        desired: Aos<IrData>,
        size: usize,
        success_ordering: MemoryOrdering,
        failure_ordering: MemoryOrdering,
    },
    /// Memory fence/barrier
    Fence {
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
            IrStatement::AtomicLoad { address, .. } => {
                v.push(address);
            }
            IrStatement::AtomicStore { address, value, .. } => {
                v.push(address);
                v.push(value);
            }
            IrStatement::AtomicRmw { address, value, .. } => {
                v.push(address);
                v.push(value);
            }
            IrStatement::AtomicCompareExchange {
                address,
                expected,
                desired,
                ..
            } => {
                v.push(address);
                v.push(expected);
                v.push(desired);
            }
            IrStatement::Fence { .. } => {}
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
            IrStatement::AtomicLoad {
                result,
                address,
                size,
                ordering,
            } => write!(
                f,
                "{} = atomic_load[{:?}]({}, size={})",
                result, ordering, address, size
            ),
            IrStatement::AtomicStore {
                address,
                value,
                size,
                ordering,
            } => write!(
                f,
                "atomic_store[{:?}]({} = {}, size={})",
                ordering, address, value, size
            ),
            IrStatement::AtomicRmw {
                result,
                operation,
                address,
                value,
                size,
                ordering,
            } => write!(
                f,
                "{} = atomic_rmw[{:?}]({:?}, {}, {}, size={})",
                result, ordering, operation, address, value, size
            ),
            IrStatement::AtomicCompareExchange {
                result,
                address,
                expected,
                desired,
                size,
                success_ordering,
                failure_ordering,
            } => write!(
                f,
                "{} = atomic_cmpxchg[{:?}/{:?}]({}, {}, {}, size={})",
                result, success_ordering, failure_ordering, address, expected, desired, size
            ),
            IrStatement::Fence { ordering } => write!(f, "fence[{:?}]", ordering),
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

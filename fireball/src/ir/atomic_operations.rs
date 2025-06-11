//! Atomic operations and memory ordering support for IR
//!
//! This module provides cross-architecture support for atomic operations,
//! memory barriers, and synchronization primitives.

use crate::arch::BaseOperation;
use crate::ir::data::IrData;
use crate::ir::operator::Operator;
use crate::ir::statements::{IrStatement, MemoryOrdering};
use crate::utils::Aos;

/// Atomic operation types supported in the IR
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomicOperation {
    /// Atomic load from memory
    Load {
        address: Aos<IrData>,
        size: usize,
        ordering: MemoryOrdering,
    },
    /// Atomic store to memory
    Store {
        address: Aos<IrData>,
        value: Aos<IrData>,
        size: usize,
        ordering: MemoryOrdering,
    },
    /// Atomic read-modify-write operation
    Rmw {
        operation: AtomicRmwOp,
        address: Aos<IrData>,
        value: Aos<IrData>,
        size: usize,
        ordering: MemoryOrdering,
    },
    /// Compare and exchange
    CompareExchange {
        address: Aos<IrData>,
        expected: Aos<IrData>,
        desired: Aos<IrData>,
        size: usize,
        success_ordering: MemoryOrdering,
        failure_ordering: MemoryOrdering,
    },
    /// Memory fence/barrier
    Fence { ordering: MemoryOrdering },
}

/// Atomic read-modify-write operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomicRmwOp {
    Add,  // Atomic addition
    Sub,  // Atomic subtraction
    And,  // Atomic bitwise AND
    Or,   // Atomic bitwise OR
    Xor,  // Atomic bitwise XOR
    Xchg, // Atomic exchange
    Max,  // Atomic maximum (signed)
    Min,  // Atomic minimum (signed)
    Umax, // Atomic maximum (unsigned)
    Umin, // Atomic minimum (unsigned)
}

impl AtomicOperation {
    /// Convert an atomic operation to IR statements
    pub fn to_ir_statements(&self, result: Option<Aos<IrData>>) -> Vec<IrStatement> {
        match self {
            AtomicOperation::Load {
                address,
                size,
                ordering,
            } => {
                vec![IrStatement::AtomicLoad {
                    result: result.unwrap_or_else(|| {
                        Aos::new(IrData::Intrinsic(crate::ir::data::IrIntrinsic::Undefined))
                    }),
                    address: address.clone(),
                    size: *size,
                    ordering: *ordering,
                }]
            }
            AtomicOperation::Store {
                address,
                value,
                size,
                ordering,
            } => {
                vec![IrStatement::AtomicStore {
                    address: address.clone(),
                    value: value.clone(),
                    size: *size,
                    ordering: *ordering,
                }]
            }
            AtomicOperation::Rmw {
                operation,
                address,
                value,
                size,
                ordering,
            } => {
                let result = result.unwrap_or_else(|| {
                    Aos::new(IrData::Intrinsic(crate::ir::data::IrIntrinsic::Undefined))
                });
                vec![IrStatement::AtomicRmw {
                    result: result.clone(),
                    operation: Self::rmw_op_to_operator(*operation),
                    address: address.clone(),
                    value: value.clone(),
                    size: *size,
                    ordering: *ordering,
                }]
            }
            AtomicOperation::CompareExchange {
                address,
                expected,
                desired,
                size,
                success_ordering,
                failure_ordering,
            } => {
                let result = result.unwrap_or_else(|| {
                    Aos::new(IrData::Intrinsic(crate::ir::data::IrIntrinsic::Undefined))
                });
                vec![IrStatement::AtomicCompareExchange {
                    result: result.clone(),
                    address: address.clone(),
                    expected: expected.clone(),
                    desired: desired.clone(),
                    size: *size,
                    success_ordering: *success_ordering,
                    failure_ordering: *failure_ordering,
                }]
            }
            AtomicOperation::Fence { ordering } => {
                vec![IrStatement::Fence {
                    ordering: *ordering,
                }]
            }
        }
    }

    /// Convert atomic RMW operation to operator
    fn rmw_op_to_operator(op: AtomicRmwOp) -> Operator {
        match op {
            AtomicRmwOp::Add => Operator::Add,
            AtomicRmwOp::Sub => Operator::Sub,
            AtomicRmwOp::And => Operator::And,
            AtomicRmwOp::Or => Operator::Or,
            AtomicRmwOp::Xor => Operator::Xor,
            AtomicRmwOp::Xchg => Operator::Mov, // Exchange is like move
            AtomicRmwOp::Max => Operator::Max,
            AtomicRmwOp::Min => Operator::Min,
            AtomicRmwOp::Umax => Operator::Umax,
            AtomicRmwOp::Umin => Operator::Umin,
        }
    }
}

/// Builder for creating atomic operations
pub struct AtomicBuilder {
    base_op: BaseOperation,
    operands: Vec<Aos<IrData>>,
    size: usize,
    ordering: MemoryOrdering,
}

impl AtomicBuilder {
    /// Create a new atomic operation builder
    pub fn new(base_op: BaseOperation) -> Self {
        Self {
            base_op,
            operands: Vec::new(),
            size: 0,
            ordering: MemoryOrdering::SeqCst, // Default to sequential consistency
        }
    }

    /// Add an operand
    pub fn operand(mut self, data: Aos<IrData>) -> Self {
        self.operands.push(data);
        self
    }

    /// Set the operation size
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    /// Set the memory ordering
    pub fn ordering(mut self, ordering: MemoryOrdering) -> Self {
        self.ordering = ordering;
        self
    }

    /// Build the atomic operation
    pub fn build(self) -> Result<AtomicOperation, &'static str> {
        match self.base_op {
            BaseOperation::Load => {
                if self.operands.len() != 1 {
                    return Err("Atomic load requires exactly one operand (address)");
                }
                Ok(AtomicOperation::Load {
                    address: self.operands[0].clone(),
                    size: self.size,
                    ordering: self.ordering,
                })
            }
            BaseOperation::Store => {
                if self.operands.len() != 2 {
                    return Err("Atomic store requires exactly two operands (address, value)");
                }
                Ok(AtomicOperation::Store {
                    address: self.operands[0].clone(),
                    value: self.operands[1].clone(),
                    size: self.size,
                    ordering: self.ordering,
                })
            }
            BaseOperation::Add => self.build_rmw(AtomicRmwOp::Add),
            BaseOperation::Subtract => self.build_rmw(AtomicRmwOp::Sub),
            BaseOperation::And => self.build_rmw(AtomicRmwOp::And),
            BaseOperation::Or => self.build_rmw(AtomicRmwOp::Or),
            BaseOperation::Xor => self.build_rmw(AtomicRmwOp::Xor),
            BaseOperation::Exchange => self.build_rmw(AtomicRmwOp::Xchg),
            BaseOperation::CompareExchange => {
                if self.operands.len() != 3 {
                    return Err(
                        "Compare-exchange requires three operands (address, expected, desired)",
                    );
                }
                Ok(AtomicOperation::CompareExchange {
                    address: self.operands[0].clone(),
                    expected: self.operands[1].clone(),
                    desired: self.operands[2].clone(),
                    size: self.size,
                    success_ordering: self.ordering,
                    failure_ordering: self.ordering, // Same ordering for simplicity
                })
            }
            _ => Err("Unsupported atomic operation"),
        }
    }

    fn build_rmw(self, op: AtomicRmwOp) -> Result<AtomicOperation, &'static str> {
        if self.operands.len() != 2 {
            return Err("Atomic RMW requires exactly two operands (address, value)");
        }
        Ok(AtomicOperation::Rmw {
            operation: op,
            address: self.operands[0].clone(),
            value: self.operands[1].clone(),
            size: self.size,
            ordering: self.ordering,
        })
    }
}

// Note: The actual implementation of IrStatementAtomic would be done
// by extending the IrStatement enum in statements.rs with these atomic variants
// The trait is removed as we've already added the variants directly to IrStatement

// Note: The actual implementation of IrStatementAtomic would be done
// by extending the IrStatement enum in statements.rs with these atomic variants

/// Helper functions for working with atomic operations
pub mod helpers {
    use super::*;

    /// Check if a base operation can be made atomic
    pub fn can_be_atomic(op: BaseOperation) -> bool {
        matches!(
            op,
            BaseOperation::Load
                | BaseOperation::Store
                | BaseOperation::Add
                | BaseOperation::Subtract
                | BaseOperation::And
                | BaseOperation::Or
                | BaseOperation::Xor
                | BaseOperation::Exchange
                | BaseOperation::CompareExchange
        )
    }

    /// Get the appropriate memory ordering for an architecture
    pub fn default_ordering_for_arch(arch: crate::arch::ArchType) -> MemoryOrdering {
        match arch {
            // x86/x86_64 has strong memory model
            crate::arch::ArchType::X86 | crate::arch::ArchType::X86_64 => MemoryOrdering::SeqCst,
            // ARM has weaker memory model
            crate::arch::ArchType::Arm32 | crate::arch::ArchType::Arm64 => MemoryOrdering::AcqRel,
            _ => MemoryOrdering::SeqCst,
        }
    }

    /// Convert x86 LOCK prefix to memory ordering
    pub fn lock_prefix_to_ordering() -> MemoryOrdering {
        // x86 LOCK prefix provides sequential consistency
        MemoryOrdering::SeqCst
    }

    /// Convert ARM load-acquire/store-release to memory ordering
    pub fn arm_ldar_stlr_to_ordering() -> MemoryOrdering {
        MemoryOrdering::AcqRel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_builder() {
        // Test atomic add
        let op = AtomicBuilder::new(BaseOperation::Add)
            .operand(Aos::new(IrData::Constant(0x1000)))
            .operand(Aos::new(IrData::Constant(1)))
            .size(8)
            .ordering(MemoryOrdering::SeqCst)
            .build()
            .unwrap();

        match op {
            AtomicOperation::Rmw {
                operation,
                size,
                ordering,
                ..
            } => {
                assert_eq!(operation, AtomicRmwOp::Add);
                assert_eq!(size, 8);
                assert_eq!(ordering, MemoryOrdering::SeqCst);
            }
            _ => panic!("Expected RMW operation"),
        }
    }

    #[test]
    fn test_can_be_atomic() {
        assert!(helpers::can_be_atomic(BaseOperation::Add));
        assert!(helpers::can_be_atomic(BaseOperation::CompareExchange));
        assert!(!helpers::can_be_atomic(BaseOperation::Jump));
        assert!(!helpers::can_be_atomic(BaseOperation::Call));
    }

    #[test]
    fn test_default_ordering() {
        assert_eq!(
            helpers::default_ordering_for_arch(crate::arch::ArchType::X86_64),
            MemoryOrdering::SeqCst
        );
        assert_eq!(
            helpers::default_ordering_for_arch(crate::arch::ArchType::Arm64),
            MemoryOrdering::AcqRel
        );
    }
}

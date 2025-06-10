//! Simulation framework for executing IR statements
//!
//! This module provides a simulation engine that can execute IR statements
//! to understand program behavior, perform symbolic execution, and verify
//! decompilation correctness.

use crate::ir::statements::*;

pub mod cpu_state;
pub mod executor;
pub mod memory;
pub mod symbolic;

pub use cpu_state::CpuState;
pub use executor::Executor;
pub use memory::Memory;
pub use symbolic::{SymbolicEngine, SymbolicValue};

/// Result type for simulation operations
pub type SimulationResult<T> = Result<T, SimulationError>;

/// Errors that can occur during simulation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationError {
    /// Memory access violation
    MemoryAccessViolation { address: u64, size: usize },
    /// Invalid instruction or operation
    InvalidOperation(String),
    /// Unsupported feature
    UnsupportedFeature(String),
    /// Division by zero
    DivisionByZero,
    /// Stack overflow
    StackOverflow,
    /// Unknown register
    UnknownRegister(String),
    /// Type mismatch
    TypeMismatch { expected: String, found: String },
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimulationError::MemoryAccessViolation { address, size } => {
                write!(
                    f,
                    "Memory access violation at 0x{:016x} (size: {})",
                    address, size
                )
            }
            SimulationError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            SimulationError::UnsupportedFeature(msg) => write!(f, "Unsupported feature: {}", msg),
            SimulationError::DivisionByZero => write!(f, "Division by zero"),
            SimulationError::StackOverflow => write!(f, "Stack overflow"),
            SimulationError::UnknownRegister(name) => write!(f, "Unknown register: {}", name),
            SimulationError::TypeMismatch { expected, found } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)
            }
        }
    }
}

impl std::error::Error for SimulationError {}

/// Simulation context containing CPU state and memory
pub struct SimulationContext {
    /// CPU state (registers, flags)
    pub cpu_state: CpuState,
    /// Memory state
    pub memory: Memory,
    /// Symbolic execution engine (if enabled)
    pub symbolic_engine: Option<SymbolicEngine>,
}

impl Default for SimulationContext {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationContext {
    /// Create a new simulation context
    pub fn new() -> Self {
        Self {
            cpu_state: CpuState::new(),
            memory: Memory::new(),
            symbolic_engine: None,
        }
    }

    /// Create a new simulation context with symbolic execution enabled
    pub fn with_symbolic() -> Self {
        Self {
            cpu_state: CpuState::new(),
            memory: Memory::new(),
            symbolic_engine: Some(SymbolicEngine::new()),
        }
    }

    /// Execute a single IR statement
    pub fn execute_statement(&mut self, statement: &IrStatement) -> SimulationResult<()> {
        let mut executor = Executor::new(self);
        executor.execute_statement(statement)
    }

    /// Execute a sequence of IR statements
    pub fn execute_statements(&mut self, statements: &[IrStatement]) -> SimulationResult<()> {
        for statement in statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_context_creation() {
        let ctx = SimulationContext::new();
        assert!(ctx.symbolic_engine.is_none());

        let ctx_with_symbolic = SimulationContext::with_symbolic();
        assert!(ctx_with_symbolic.symbolic_engine.is_some());
    }
}

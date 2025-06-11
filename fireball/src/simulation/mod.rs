//! Simulation framework for executing IR statements
//!
//! DEPRECATED: This module will be replaced with Unicorn Engine integration.
//! Unicorn provides a much more complete and battle-tested emulation framework.
//!
//! TODO: Replace with unicorn-engine crate:
//! - https://github.com/unicorn-engine/unicorn
//! - Better architecture support (x86, ARM, etc.)
//! - More accurate CPU emulation
//! - Better performance
//!
//! This module provides a basic simulation engine that can execute IR statements
//! to understand program behavior, perform symbolic execution, and verify
//! decompilation correctness.

use crate::ir::statements::*;

pub mod cpu_state;
pub mod executor;
pub mod fpu_state;
pub mod memory;
pub mod symbolic;
pub mod unicorn;

pub use cpu_state::CpuState;
pub use executor::Executor;
pub use fpu_state::FpuState;
pub use memory::Memory;
pub use symbolic::{SymbolicEngine, SymbolicValue};
pub use unicorn::{UnicornSimulator, create_arm_memory_layout, create_x86_64_memory_layout};

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
    /// FPU stack overflow
    FpuStackOverflow,
    /// FPU stack underflow
    FpuStackUnderflow,
    /// Invalid FPU register index
    InvalidFpuRegister(u8),
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
            SimulationError::FpuStackOverflow => write!(f, "FPU stack overflow"),
            SimulationError::FpuStackUnderflow => write!(f, "FPU stack underflow"),
            SimulationError::InvalidFpuRegister(index) => {
                write!(f, "Invalid FPU register index: {}", index)
            }
        }
    }
}

impl std::error::Error for SimulationError {}

/// Simulation backend type
pub enum SimulationBackend {
    /// Legacy custom simulation
    Legacy,
    /// Unicorn Engine-based simulation
    Unicorn,
}

/// Simulation context containing CPU state and memory
pub struct SimulationContext {
    /// CPU state (registers, flags) - legacy mode only
    pub cpu_state: CpuState,
    /// Memory state - legacy mode only
    pub memory: Memory,
    /// Symbolic execution engine (if enabled) - legacy mode only
    pub symbolic_engine: Option<SymbolicEngine>,
    /// Unicorn simulator (if using Unicorn backend)
    pub unicorn_simulator: Option<UnicornSimulator<'static>>,
    /// Current backend being used
    pub backend: SimulationBackend,
}

impl SimulationContext {
    /// Create a new simulation context (legacy mode)
    pub fn new() -> Self {
        Self {
            cpu_state: CpuState::new(),
            memory: Memory::new(),
            symbolic_engine: None,
            unicorn_simulator: None,
            backend: SimulationBackend::Legacy,
        }
    }

    /// Create a new simulation context with symbolic execution enabled (legacy mode)
    pub fn with_symbolic() -> Self {
        Self {
            cpu_state: CpuState::new(),
            memory: Memory::new(),
            symbolic_engine: Some(SymbolicEngine::new()),
            unicorn_simulator: None,
            backend: SimulationBackend::Legacy,
        }
    }

    /// Create a new simulation context with Unicorn backend
    pub fn with_unicorn(
        architecture: crate::arch::architecture::ArchitectureInfo,
    ) -> SimulationResult<Self> {
        let unicorn_sim = UnicornSimulator::new(architecture)?;

        Ok(Self {
            cpu_state: CpuState::new(), // Keep for compatibility
            memory: Memory::new(),      // Keep for compatibility
            symbolic_engine: None,
            unicorn_simulator: Some(unicorn_sim),
            backend: SimulationBackend::Unicorn,
        })
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

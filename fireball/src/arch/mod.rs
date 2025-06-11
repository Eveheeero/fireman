//! Module containing implementations for multiple architectures
//!
//! TODO: Unified Architecture Implementation (Sprint 6)
//! - Phase 1: Create x86_unified and arm_unified modules
//! - Phase 2: Migrate x86/x86_64 to use x86_unified (90% code sharing)
//! - Phase 3: Migrate arm32/arm64 to use arm_unified (60% code sharing)
//! - Phase 4: Remove old separate modules after migration
//! - Expected: 60-80% code reduction, better maintainability
//!
//! See: docs/implementation/unified-architecture-implementation-plan.md

pub mod architecture;
pub mod arm32;
pub mod arm64;
pub mod calling_convention;
pub mod register_mapping;
pub mod unified_instruction;
pub mod x86_64;

// Re-export key types
pub use architecture::{
    ArchType, ArchitectureContext, ArchitectureDetector, ArchitectureInfo, Endianness,
    OperatingSystem,
};
pub use calling_convention::{
    CallingConventionInfo, CallingConventionProvider, ParamLocation, ParamType,
    get_calling_convention_provider,
};
pub use register_mapping::{CallingConventionRegisters, RegisterMapper, get_register_mapper};
pub use unified_instruction::{
    ArchMode, ArmInstructionSet, BaseOperation, InstructionCategory, InstructionModifiers,
    ProcessorMode, UnifiedArchContext, UnifiedInstruction, UnifiedInstructionAnalyzer,
};

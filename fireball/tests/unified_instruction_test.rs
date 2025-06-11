//! Tests for unified instruction handling across architectures
//!
//! This test suite verifies that the unified instruction interface
//! correctly handles instructions from different architectures and
//! maintains deterministic output.

use fireball::arch::{
    architecture::{ArchType, ArchitectureInfo, Endianness},
    unified_instruction::{
        ArchFeatures, ArchMode, ArmInstructionSet, BaseOperation, ConditionCode,
        InstructionCategory, InstructionModifiers, ProcessorMode, UnifiedArchContext,
        UnifiedInstructionAnalyzer,
    },
};
use fireball::ir::statements::MemoryOrdering;

/// Helper to create a test architecture context
fn create_test_context(arch_type: ArchType) -> UnifiedArchContext {
    let arch_info = ArchitectureInfo {
        arch_type,
        pointer_size: arch_type.default_pointer_size(),
        endianness: arch_type.default_endianness(),
        register_count: match arch_type {
            ArchType::X86 => 8,
            ArchType::X86_64 => 16,
            ArchType::Arm32 => 16,
            ArchType::Arm64 => 31,
            _ => 0,
        },
        instruction_alignment: arch_type.instruction_alignment(),
    };

    let mode = match arch_type {
        ArchType::X86 => ArchMode::X86(ProcessorMode::Protected32),
        ArchType::X86_64 => ArchMode::X86(ProcessorMode::Long64),
        ArchType::Arm32 => ArchMode::Arm(ArmInstructionSet::Arm32),
        ArchType::Arm64 => ArchMode::Arm(ArmInstructionSet::AArch64),
        _ => ArchMode::X86(ProcessorMode::Long64),
    };

    UnifiedArchContext {
        arch_type,
        arch_info,
        mode,
        features: ArchFeatures::default(),
    }
}

#[test]
fn test_unified_analyzer_creation() {
    // Just test that we can create the analyzer
    let _analyzer = UnifiedInstructionAnalyzer::new();
}

#[test]
fn test_base_operations_enum() {
    // Test that base operations are properly defined
    let operations = vec![
        BaseOperation::Add,
        BaseOperation::Subtract,
        BaseOperation::And,
        BaseOperation::Or,
        BaseOperation::Xor,
        BaseOperation::Jump,
        BaseOperation::CompareExchange,
    ];

    // Ensure all operations are distinct
    for (i, op1) in operations.iter().enumerate() {
        for (j, op2) in operations.iter().enumerate() {
            if i != j {
                assert_ne!(op1, op2, "Duplicate base operations");
            }
        }
    }
}

#[test]
fn test_instruction_modifiers_default() {
    // Test that instruction modifiers are properly initialized
    let modifiers = InstructionModifiers::default();
    assert!(!modifiers.lock_prefix);
    assert!(modifiers.rep_prefix.is_none());
    assert!(modifiers.condition.is_none());
    assert!(!modifiers.update_flags);
    assert!(modifiers.size_override.is_none());
}

#[test]
fn test_memory_ordering_for_architectures() {
    use fireball::ir::atomic_operations::helpers;

    // x86/x86_64 has strong memory model
    assert_eq!(
        helpers::default_ordering_for_arch(ArchType::X86),
        MemoryOrdering::SeqCst
    );
    assert_eq!(
        helpers::default_ordering_for_arch(ArchType::X86_64),
        MemoryOrdering::SeqCst
    );

    // ARM has weaker memory model
    assert_eq!(
        helpers::default_ordering_for_arch(ArchType::Arm32),
        MemoryOrdering::AcqRel
    );
    assert_eq!(
        helpers::default_ordering_for_arch(ArchType::Arm64),
        MemoryOrdering::AcqRel
    );
}

#[test]
fn test_x86_32bit_mode_handling() {
    let ctx_32 = create_test_context(ArchType::X86);
    let ctx_64 = create_test_context(ArchType::X86_64);

    // In 32-bit mode, architecture size should be 32 bits
    // In 64-bit mode, architecture size should be 64 bits
    match ctx_32.mode {
        ArchMode::X86(ProcessorMode::Protected32) => {
            assert_eq!(ctx_32.arch_info.pointer_size, 32);
        }
        _ => panic!("Wrong mode for x86"),
    }

    match ctx_64.mode {
        ArchMode::X86(ProcessorMode::Long64) => {
            assert_eq!(ctx_64.arch_info.pointer_size, 64);
        }
        _ => panic!("Wrong mode for x86_64"),
    }
}

#[test]
fn test_architecture_detection() {
    use fireball::arch::architecture::ArchitectureDetector;

    // Test PE header detection (x86_64)
    let pe_header = vec![
        0x4D, 0x5A, // MZ signature
             // ... minimal PE header
    ];

    let arch_info = ArchitectureDetector::detect_from_bytes(&pe_header);
    // Without a full PE header, it should default to unknown
    assert_eq!(arch_info.arch_type, ArchType::Unknown);

    // Test ELF header detection
    let elf_header = vec![
        0x7F, 0x45, 0x4C, 0x46, // ELF magic
        0x02, // 64-bit
        0x01, // Little endian
        0x01, // Version
        0x00, // System V ABI
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding
        0x02, 0x00, // Executable
        0x3E, 0x00, // x86_64
    ];

    let arch_info = ArchitectureDetector::detect_from_bytes(&elf_header);
    assert_eq!(arch_info.arch_type, ArchType::X86_64);
    assert_eq!(arch_info.pointer_size, 64);
    assert_eq!(arch_info.endianness, Endianness::Little);
}

#[test]
fn test_condition_code_mapping() {
    // Test that condition codes are properly defined
    let conditions = vec![
        ConditionCode::Equal,
        ConditionCode::NotEqual,
        ConditionCode::SignedLess,
        ConditionCode::SignedGreater,
        ConditionCode::UnsignedLowerOrSame,
        ConditionCode::UnsignedHigher,
    ];

    // Ensure all condition codes are distinct
    for (i, cond1) in conditions.iter().enumerate() {
        for (j, cond2) in conditions.iter().enumerate() {
            if i != j {
                assert_ne!(cond1, cond2, "Duplicate condition codes");
            }
        }
    }
}

#[test]
fn test_instruction_categories() {
    // Test that instruction categories are properly defined
    let categories = vec![
        InstructionCategory::Arithmetic,
        InstructionCategory::Logic,
        InstructionCategory::DataTransfer,
        InstructionCategory::ControlFlow,
        InstructionCategory::Compare,
        InstructionCategory::Stack,
        InstructionCategory::Simd,
        InstructionCategory::System,
        InstructionCategory::Atomic,
        InstructionCategory::Unknown,
    ];

    // Ensure all categories are distinct
    for (i, cat1) in categories.iter().enumerate() {
        for (j, cat2) in categories.iter().enumerate() {
            if i != j {
                assert_ne!(cat1, cat2, "Duplicate instruction categories");
            }
        }
    }
}

#[test]
fn test_atomic_operation_support() {
    use fireball::ir::atomic_operations::helpers;

    // Test that appropriate operations can be made atomic
    assert!(helpers::can_be_atomic(BaseOperation::Add));
    assert!(helpers::can_be_atomic(BaseOperation::CompareExchange));
    assert!(helpers::can_be_atomic(BaseOperation::Load));
    assert!(helpers::can_be_atomic(BaseOperation::Store));
    assert!(!helpers::can_be_atomic(BaseOperation::Jump));
    assert!(!helpers::can_be_atomic(BaseOperation::Call));
}

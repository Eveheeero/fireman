# Unified Architecture Implementation Plan

## Executive Summary

Unify 32-bit and 64-bit implementations for both x86 and ARM architectures to reduce code duplication by 60-80% while
maintaining architecture-specific features.

## Current State Analysis

### Code Duplication Metrics

```
x86/x86_64:
- Shared instructions: ~90% (ADD, MOV, JMP, etc.)
- Shared concepts: ~95% (conditions, flags, addressing)
- Unique to x64: ~10% (REX prefix, R8-R15, RIP-relative)

ARM32/ARM64:
- Shared instructions: ~60% (basic arithmetic, logic)
- Shared concepts: ~80% (conditions, load/store patterns)
- Unique to ARM64: ~40% (X registers, different encoding)
```

## Implementation Phases

### Phase 1: Foundation (Week 1-2)

#### 1.1 Create Unified Module Structure

```
TODO: Create unified architecture modules
Priority: HIGH
Subtasks:
- [ ] Create fireball/src/arch/x86_unified/
  - [ ] mod.rs - Main module with ProcessorMode enum
  - [ ] common.rs - Shared instructions, conditions, flags
  - [ ] register.rs - Unified register handling
  - [ ] instruction_analyze.rs - Common analysis logic
  - [ ] x86_specific.rs - 32-bit only features
  - [ ] x64_specific.rs - 64-bit only features
  
- [ ] Create fireball/src/arch/arm_unified/
  - [ ] mod.rs - Main module with InstructionSet enum
  - [ ] common.rs - Shared conditions, operations
  - [ ] register.rs - Unified register handling
  - [ ] instruction_analyze.rs - Common analysis logic
  - [ ] arm32_specific.rs - ARM32/Thumb specific
  - [ ] arm64_specific.rs - AArch64 specific
```

#### 1.2 Design Common Interfaces

```rust
// Unified instruction trait
trait UnifiedInstruction {
    fn mnemonic(&self) -> &str;
    fn category(&self) -> InstructionCategory;
    fn operand_count(&self) -> usize;
    fn to_ir(&self, ctx: &ArchContext) -> Result<Vec<IrStatement>, DecompileError>;
}

// Architecture context
struct ArchContext {
    mode: ProcessorMode,        // or InstructionSet for ARM
    pointer_size: u8,
    default_operand_size: u8,
    features: ArchFeatures,
}
```

### Phase 2: X86 Unification (Week 3-4)

#### 2.1 Common X86 Implementation

```
TODO: Implement X86 unified analyzer
Priority: HIGH
Subtasks:
- [ ] Merge common instruction definitions
  - [ ] Arithmetic: ADD, SUB, MUL, DIV (90% shared)
  - [ ] Logic: AND, OR, XOR, NOT (100% shared)
  - [ ] Control: JMP, CALL, RET, Jcc (95% shared)
  - [ ] Memory: MOV, LEA, PUSH, POP (85% shared)
  
- [ ] Unified register handling
  - [ ] Map 32-bit registers to 64-bit equivalents
  - [ ] Handle zero-extension in 64-bit mode
  - [ ] REX prefix detection and handling
  
- [ ] Addressing mode unification
  - [ ] Base + Index*Scale + Displacement (shared)
  - [ ] RIP-relative (64-bit only)
  - [ ] Segment overrides (mostly 32-bit)
```

#### 2.2 Mode-Specific Handling

```rust
impl X86UnifiedAnalyzer {
    fn analyze_instruction(&self, inst: &Instruction) -> Vec<IrStatement> {
        let base_ir = self.analyze_common(inst);
        
        match self.mode {
            ProcessorMode::Protected32 => {
                // No zero-extension needed
                // Segment registers matter more
                self.adjust_for_32bit(base_ir)
            }
            ProcessorMode::Long64 => {
                // Zero-extend 32-bit ops to 64-bit
                // Handle REX prefixes
                // RIP-relative addressing
                self.adjust_for_64bit(base_ir)
            }
        }
    }
}
```

### Phase 3: ARM Unification (Week 5-6)

#### 3.1 Common ARM Implementation

```
TODO: Implement ARM unified analyzer
Priority: HIGH
Subtasks:
- [ ] Merge common concepts
  - [ ] Condition codes (EQ, NE, LT, GT - 100% shared)
  - [ ] Data processing (ADD, SUB, AND, ORR - 80% shared)
  - [ ] Load/Store patterns (LDR, STR - 70% shared)
  
- [ ] Unified register handling
  - [ ] R0-R15 vs X0-X30/W0-W30
  - [ ] Special registers (SP, LR, PC)
  - [ ] NEON/FP registers
  
- [ ] Instruction set handling
  - [ ] ARM32: ARM, Thumb, Thumb2
  - [ ] ARM64: AArch64 only
```

#### 3.2 Encoding Differences

```rust
impl ARMUnifiedAnalyzer {
    fn decode_instruction(&self, bytes: &[u8]) -> Result<ARMInstruction, Error> {
        match self.instruction_set {
            InstructionSet::ARM => {
                // 32-bit fixed encoding
                self.decode_arm32(bytes)
            }
            InstructionSet::Thumb => {
                // 16-bit or 32-bit variable
                self.decode_thumb(bytes)
            }
            InstructionSet::AArch64 => {
                // 32-bit fixed encoding (different from ARM32)
                self.decode_arm64(bytes)
            }
        }
    }
}
```

### Phase 4: Integration (Week 7)

#### 4.1 Update Existing Code

```
TODO: Migrate to unified modules
Priority: MEDIUM
Subtasks:
- [ ] Update fireball/src/arch/mod.rs
  - [ ] Add pub mod x86_unified
  - [ ] Add pub mod arm_unified
  - [ ] Create compatibility aliases
  
- [ ] Update instruction analysis
  - [ ] Route x86/x64 through unified analyzer
  - [ ] Route arm32/arm64 through unified analyzer
  - [ ] Maintain backward compatibility
  
- [ ] Update tests
  - [ ] Add unified module tests
  - [ ] Ensure existing tests pass
  - [ ] Add cross-architecture tests
```

#### 4.2 Compatibility Layer

```rust
// Temporary compatibility during migration
pub mod x86 {
    pub use super::x86_unified::{
        X86UnifiedAnalyzer as X86Analyzer,
        X86Register,
        // ... other exports
    };
}

pub mod x86_64 {
    pub use super::x86_unified::{
        X86UnifiedAnalyzer as X64Analyzer,
        X86Register,
        // ... other exports
    };
}
```

### Phase 5: Testing & Validation (Week 8)

#### 5.1 Comprehensive Testing

```
TODO: Test unified implementations
Priority: HIGH
Subtasks:
- [ ] Unit tests
  - [ ] Test each instruction in both modes
  - [ ] Verify register mappings
  - [ ] Check mode-specific behavior
  
- [ ] Integration tests
  - [ ] Full binary decompilation
  - [ ] Compare output with old implementation
  - [ ] Performance benchmarks
  
- [ ] Cross-architecture tests
  - [ ] Same algorithm in x86 vs x64
  - [ ] Same algorithm in ARM32 vs ARM64
  - [ ] Verify consistent IR generation
```

#### 5.2 Determinism Verification

```rust
#[test]
fn test_unified_determinism() {
    let binary = include_bytes!("test.exe");
    
    // Test 100 times
    let outputs: Vec<_> = (0..100)
        .map(|_| decompile_with_unified(binary))
        .collect();
    
    // All outputs must be identical
    assert!(outputs.windows(2).all(|w| w[0] == w[1]));
}
```

### Phase 6: Cleanup (Week 9)

#### 6.1 Remove Old Modules

```
TODO: Remove deprecated modules
Priority: LOW
Subtasks:
- [ ] Delete old architecture modules
  - [ ] Remove src/arch/x86/
  - [ ] Remove src/arch/x86_64/
  - [ ] Remove src/arch/arm32/
  - [ ] Remove src/arch/arm64/
  
- [ ] Update all imports
- [ ] Remove compatibility layer
- [ ] Update documentation
```

#### 6.2 Final Optimization

```
TODO: Optimize unified implementation
Priority: LOW
Subtasks:
- [ ] Profile hot paths
- [ ] Inline critical functions
- [ ] Remove redundant checks
- [ ] Optimize common patterns
```

## Success Metrics

### Code Reduction

- Target: 60-80% reduction in architecture code
- Measurement: Lines of code before/after

### Performance

- Target: No regression (±5%)
- Measurement: Benchmark suite

### Correctness

- Target: 100% test pass rate
- Measurement: Existing test suite

### Maintainability

- Target: Single fix location for bugs
- Measurement: Bug fix commits touch fewer files

## Risk Mitigation

### Risk 1: Breaking Changes

- **Mitigation**: Compatibility layer during migration
- **Testing**: Run old and new in parallel

### Risk 2: Performance Regression

- **Mitigation**: Benchmark throughout development
- **Testing**: Performance test suite

### Risk 3: Missing Edge Cases

- **Mitigation**: Extensive test coverage
- **Testing**: Fuzzing with random binaries

## Implementation Checklist

### Week 1-2: Foundation

- [ ] Create directory structure
- [ ] Design common interfaces
- [ ] Write initial documentation

### Week 3-4: X86 Unification

- [ ] Implement common x86 logic
- [ ] Handle mode-specific features
- [ ] Test x86/x64 compatibility

### Week 5-6: ARM Unification

- [ ] Implement common ARM logic
- [ ] Handle instruction set differences
- [ ] Test ARM32/ARM64 compatibility

### Week 7: Integration

- [ ] Update existing code
- [ ] Create compatibility layer
- [ ] Fix breaking changes

### Week 8: Testing

- [ ] Run comprehensive tests
- [ ] Verify determinism
- [ ] Benchmark performance

### Week 9: Cleanup

- [ ] Remove old code
- [ ] Optimize implementation
- [ ] Update all documentation

## Final Cleanup Task

```
TODO: Remove redundancies and unused code
Priority: LOWEST
When: After all other tasks complete
Subtasks:
- [ ] Audit all modules for dead code
- [ ] Remove unused functions/types
- [ ] Consolidate duplicate logic
- [ ] Remove outdated documentation
- [ ] Clean up test fixtures
- [ ] Remove compatibility shims
- [ ] Archive old implementation notes
```

## Notes

1. **Incremental Migration**: We can migrate one architecture at a time
2. **Feature Flags**: Use Cargo features to enable/disable architectures
3. **Benchmarking**: Compare performance at each stage
4. **Documentation**: Update as we go, not at the end

## Dependencies

- No external dependencies needed
- Uses existing iceball definitions
- Builds on current IR infrastructure

## Timeline

Total: 9 weeks

- Planning: 1 week (done)
- Implementation: 6 weeks
- Testing: 1 week
- Cleanup: 1 week

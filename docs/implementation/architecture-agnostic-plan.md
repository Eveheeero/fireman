# Architecture-Agnostic Implementation Plan

## Overview

Comprehensive plan for implementing unified architecture support (x86, x86_64, ARM32, ARM64) at the AST level with
proper visibility controls and documentation updates.

## Core Principles

1. **AST-Level Optimization**: All optimizations happen at AST level, NOT IR level
2. **Hexadecimal Default**: Default numeric output in hex, user-configurable
3. **Unicorn Integration**: Replace custom simulation with Unicorn Engine
4. **Strict Visibility**: Minimize public API surface

## Phase 1: Code Cleanup & Visibility Fixes (Week 1)

### Day 1-2: Visibility Restrictions

- [ ] **Core Module Visibility**
    - [ ] Change `Instruction::new()` to `pub(crate)`
    - [ ] Review all `pub` methods in core modules
    - [ ] Document why each public API exists

- [ ] **Binary Module Visibility**
    - [ ] Fix PE fire analysis methods (`pub(crate)` → `pub(super)`)
    - [ ] Review visibility in:
        - `/binary/pe/fire/*.rs`
        - `/binary/elf/fire/*.rs`
        - `/binary/macho/fire/*.rs`

- [ ] **Sections API**
    - [ ] Keep `Default` impl but document it's for empty sections
    - [ ] Change `add_section()` to `pub(crate)`
    - [ ] Add `#[doc(hidden)]` to internal methods

### Day 3: Remove Invalid Defaults

- [ ] **Descriptor Defaults**
    - [ ] Remove `Default` impl from `IrStatementDescriptorMap`
    - [ ] Find all usages and fix them
    - [ ] Add documentation about why no default

- [ ] **Other Invalid Defaults**
    - [ ] Audit all `Default` implementations
    - [ ] Remove those that don't make semantic sense
    - [ ] Document remaining ones

### Day 4-5: Documentation Updates

- [ ] **Update CLAUDE.md**
    - [x] Clarify AST-level optimization (NOT IR-level)
    - [x] Add architecture-agnostic design section
    - [x] Add Enhanced C AST guidelines
    - [ ] Add visibility guidelines section
    - [ ] Add Unicorn integration notes

- [ ] **Update TODOS.md**
    - [x] Add Sprint 5: Architecture-Agnostic AST
    - [x] Update simulation section for Unicorn
    - [ ] Add detailed micro-steps for each task
    - [ ] Update timeline estimates

## Phase 2: Architecture Detection & Support (Week 2)

### Day 6-7: Binary Format Detection

```rust
// Micro-steps:
1.Read magic bytes (4 bytes minimum)
2.Identify format: PE/ELF/Mach-O
3.Parse headers for architecture info
4.Return ArchitectureInfo struct
```

- [ ] **PE Architecture Detection**
    - [ ] Read DOS header → PE offset
    - [ ] Read PE machine type (0x014C=x86, 0x8664=x64, etc.)
    - [ ] Read optional header magic (32/64 bit)
    - [ ] Create test suite with sample binaries

- [ ] **ELF Architecture Detection**
    - [ ] Read EI_CLASS (32/64 bit)
    - [ ] Read e_machine field
    - [ ] Handle endianness (EI_DATA)
    - [ ] Test with various ELF files

- [ ] **Mach-O Architecture Detection**
    - [ ] Read magic (feedface/feedfacf)
    - [ ] Read cputype field
    - [ ] Handle fat binaries
    - [ ] Test with universal binaries

### Day 8-9: Unified Instruction Interface

- [ ] **Create Common Traits**
  ```rust
  trait ArchInstruction {
      fn mnemonic(&self) -> &str;
      fn operand_count(&self) -> usize;
      fn is_branch(&self) -> bool;
      fn is_call(&self) -> bool;
      fn to_low_ir(&self, ctx: &ArchContext) -> Vec<LowIr>;
  }
  ```

- [ ] **Implement for Each Architecture**
    - [ ] X86Instruction impl ArchInstruction
    - [ ] X64Instruction impl ArchInstruction
    - [ ] Arm32Instruction impl ArchInstruction
    - [ ] Arm64Instruction impl ArchInstruction

### Day 10: Architecture Context

- [ ] **Create ArchContext struct**
  ```rust
  struct ArchContext {
      arch_type: ArchType,
      pointer_size: u8,
      endianness: Endianness,
      calling_convention: CallingConvention,
      register_map: RegisterMap,
  }
  ```

- [ ] **Calling Convention Mappings**
    - [ ] x86: cdecl, stdcall, fastcall
    - [ ] x64: System V, Microsoft
    - [ ] ARM32: AAPCS
    - [ ] ARM64: AAPCS64

## Phase 3: Enhanced AST Generator (Week 3)

### Day 11-12: AST Structure Extensions

- [ ] **Extend Existing AST Types**
    - [ ] Add architecture info to CAst
    - [ ] Create EnhancedPrintConfig
    - [ ] Implement EnhancedPrintWithConfig trait
    - [ ] DO NOT bypass existing structure

- [ ] **Numeric Format Support**
  ```rust
  enum NumericFormat {
      Hexadecimal,  // Default
      Decimal,
      Binary,
      Auto,         // Smart detection
  }
  ```

### Day 13-14: Pattern to AST Conversion

- [ ] **Control Flow Patterns**
    - [ ] ForLoop → For statement with proper init/cond/inc
    - [ ] WhileLoop → While statement
    - [ ] DoWhileLoop → Do-while statement
    - [ ] IfElse → If statement with optional else

- [ ] **Expression Patterns**
    - [ ] ArrayAccess → ArrayAccess expression
    - [ ] FieldAccess → MemberAccess expression
    - [ ] FunctionCall → Call expression
    - [ ] BinaryOp → BinaryOp expression

### Day 15: Architecture-Aware Types

- [ ] **Type Sizing**
  ```rust
  fn get_pointer_type(arch: &ArchInfo) -> CType {
      match arch.pointer_size {
          32 => CType::UInt32,
          64 => CType::UInt64,
          _ => CType::UInt,
      }
  }
  ```

- [ ] **Fixed-Width Types**
    - [ ] Map to stdint.h types
    - [ ] Handle architecture differences
    - [ ] Generate proper includes

## Phase 4: Unicorn Engine Integration (Week 4)

### Day 16-17: Remove Custom Simulation

- [ ] **Mark Current Simulation as Deprecated**
    - [ ] Add deprecation warnings
    - [ ] Document migration path
    - [ ] Keep for reference temporarily

- [ ] **Add Unicorn Dependency**
  ```toml
  [dependencies]
  unicorn-engine = "2.0"
  ```

### Day 18-19: Unicorn Wrapper

- [ ] **Create Emulation Interface**
  ```rust
  pub struct UnicornEmulator {
      engine: Unicorn<()>,
      arch: Architecture,
      hooks: Vec<HookHandle>,
  }
  ```

- [ ] **Architecture Mapping**
    - [ ] ArchType::X86 → Arch::X86
    - [ ] ArchType::X64 → Arch::X86 + Mode::MODE_64
    - [ ] ArchType::ARM32 → Arch::ARM
    - [ ] ArchType::ARM64 → Arch::ARM64

### Day 20: Hook Implementation

- [ ] **Memory Hooks**
    - [ ] Track memory reads/writes
    - [ ] Detect access patterns
    - [ ] Build memory map

- [ ] **Code Hooks**
    - [ ] Instruction tracing
    - [ ] Branch tracking
    - [ ] Call graph building

## Phase 5: Testing & Validation (Week 5)

### Day 21-22: Cross-Architecture Tests

- [ ] **Test Binary Collection**
    - [ ] x86: Simple programs (hello world, loops)
    - [ ] x64: Modern applications
    - [ ] ARM32: Embedded binaries
    - [ ] ARM64: Mobile/server apps

- [ ] **Determinism Tests**
  ```rust
  #[test]
  fn test_deterministic_across_architectures() {
      for arch in [X86, X64, ARM32, ARM64] {
          let output1 = decompile(binary, arch);
          let output2 = decompile(binary, arch);
          assert_eq!(output1, output2);
      }
  }
  ```

### Day 23-24: Numeric Format Tests

- [ ] **Format Switching**
    - [ ] Test hex output (default)
    - [ ] Test decimal output
    - [ ] Test binary output
    - [ ] Test auto-detection

- [ ] **Architecture-Specific Formatting**
    - [ ] 32-bit addresses: 8 hex digits
    - [ ] 64-bit addresses: 16 hex digits
    - [ ] Consistent across all architectures

### Day 25: Integration Tests

- [ ] **Full Pipeline Tests**
    - [ ] Binary → AST → Enhanced C
    - [ ] Verify compilable output
    - [ ] Check type correctness
    - [ ] Validate numeric formats

## Phase 6: CLI/GUI Updates (Week 6)

### Day 26-27: CLI Enhancements

- [ ] **New Flags**
  ```bash
  fireman --arch=auto|x86|x64|arm32|arm64
  fireman --numeric-format=hex|dec|bin|auto
  fireman --detect-arch binary.exe
  ```

- [ ] **Output Options**
    - [ ] JSON architecture info
    - [ ] Configurable verbosity
    - [ ] Progress indicators

### Day 28-29: GUI Updates

- [ ] **Architecture Display**
    - [ ] Show detected architecture
    - [ ] Allow manual override
    - [ ] Display arch-specific info

- [ ] **Numeric Format Toggle**
    - [ ] Add format selector
    - [ ] Hotkeys (H=hex, D=dec, B=bin)
    - [ ] Live preview update

### Day 30: Documentation

- [ ] **User Guide**
    - [ ] Architecture support matrix
    - [ ] Numeric format examples
    - [ ] CLI usage examples

- [ ] **Developer Guide**
    - [ ] Architecture extension guide
    - [ ] AST manipulation examples
    - [ ] Unicorn integration patterns

## Success Metrics

1. **Architecture Support**
    - [ ] All 4 architectures detected correctly
    - [ ] Instruction decoding works for each
    - [ ] Proper type sizing per architecture

2. **AST Quality**
    - [ ] No string concatenation in generation
    - [ ] All patterns convert to AST nodes
    - [ ] Enhanced features configurable

3. **Determinism**
    - [ ] 1000 runs = identical output
    - [ ] Cross-architecture consistency
    - [ ] Format changes don't affect structure

4. **Performance**
    - [ ] < 100ms overhead for arch detection
    - [ ] Unicorn emulation < 2x native speed
    - [ ] AST generation < 50ms per function

## Risk Mitigation

1. **Unicorn Compatibility**
    - Risk: Version conflicts
    - Mitigation: Pin to stable version, extensive testing

2. **AST Complexity**
    - Risk: Too many architecture-specific cases
    - Mitigation: Common trait abstractions

3. **Binary Format Edge Cases**
    - Risk: Malformed headers
    - Mitigation: Fallback to manual selection

## Timeline Summary

- **Week 1**: Code cleanup, visibility fixes, documentation
- **Week 2**: Architecture detection and support
- **Week 3**: Enhanced AST generator implementation
- **Week 4**: Unicorn Engine integration
- **Week 5**: Testing and validation
- **Week 6**: CLI/GUI updates and final documentation

Total: 6 weeks to complete architecture-agnostic support with all features.

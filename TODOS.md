# Fireman - Features & Plans

Working In Progress (2025.01 Updated)

## 🎯 Project Status Overview

**Current Focus**: Binary-to-Enhanced-C Decompiler with Absolute Determinism

**Enhanced C**: A tailored C-like language for decompiler output that minimally leverages modern C++ features for
improved readability while preserving low-level details.

### ✅ Completed Components
- [x] Basic IR Environment Foundation
- [x] X64 Instruction Parsing (Most common instructions)
- [x] Control Flow Analysis with Complex Loop Detection
- [x] Data Flow Analysis (Reaching Definitions, Liveness)
- [x] Basic Simulation Framework (CPU State, Memory)
- [x] Deterministic Infrastructure (Address-based naming, BTreeMap usage)
- [x] Multi-Level IR Implementation (Low → Medium → High)
- [x] Add confidence scoring system
- [x] Add high-level construct generation
- [x] Implement IR → C code generator (Simple version)
- [x] Advanced Type Recovery System (Jan 2025)
  - [x] Confidence-based type inference
  - [x] Library function signature matching
  - [x] Array and struct detection
  - [x] Type constraint propagation
- [x] Variable Naming Heuristics (Jan 2025)
  - [x] Type-based naming
  - [x] Usage pattern detection
  - [x] Reserved keyword avoidance
- [x] Struct/Class Reconstruction (Jan 2025)
  - [x] Field layout detection
  - [x] Access pattern analysis
  - [x] Field type inference
- [x] Enhanced C Code Generation (Jan 2025)
  - [x] Auto type inference for complex types
  - [x] Fixed-width integer types (uint32_t, int64_t)
  - [x] nullptr instead of NULL
  - [x] Range-based for loops where applicable
  - [x] Inline variable declarations
  - [x] Confidence-based feature usage

### 🚧 Current Sprint Focus

#### 🏗️ Sprint 5: Architecture-Agnostic AST Generation [NEW - IN PROGRESS]

**Goal**: Implement unified architecture support at the AST level with configurable output

1. **AST-Level Architecture Support** [Critical]
    - [x] Create unified Memory struct for all architectures ✅
    - [x] Add Architecture enum (X86, X86_64, ARM32, ARM64) ✅
    - [ ] Implement architecture-aware type sizing in AST
    - [ ] Add architecture detection from binary headers
    - [ ] Create architecture-specific register mappings

2. **Enhanced AST Generator** [High Priority]
    - [x] Design EnhancedAstConfig with numeric format options ✅
    - [x] Implement hexadecimal as default output format ✅
    - [ ] Add user-configurable numeric display (Hex/Dec/Bin/Auto)
    - [ ] Create architecture-aware literal formatting
    - [ ] Support proper type sizing based on architecture (32 vs 64-bit)

3. **Unified Instruction Handling** [Medium Priority]
    - [ ] Create common instruction interface for all architectures
    - [ ] Map architecture-specific instructions to common IR
    - [ ] Handle architecture-specific calling conventions
    - [ ] Support architecture-specific optimizations at AST level

4. **Testing & Validation** [High Priority]
    - [ ] Create cross-architecture test suite
    - [ ] Verify deterministic output across architectures
    - [ ] Test numeric format switching
    - [ ] Validate AST structure preservation

#### 🔧 Sprint 6: Unified Architecture Implementation [NEW - PLANNED]

**Goal**: Unify 32-bit and 64-bit implementations for x86 and ARM to reduce code duplication by 60-80%

1. **Foundation & Design** [Week 1-2]
    - [ ] Create x86_unified module structure
        - [ ] common.rs - Shared x86/x64 logic (90% of instructions)
        - [ ] x86_specific.rs - 32-bit only features
        - [ ] x64_specific.rs - 64-bit only features (REX, R8-R15)
    - [ ] Create arm_unified module structure
        - [ ] common.rs - Shared ARM32/64 logic (60% of instructions)
        - [ ] arm32_specific.rs - ARM32/Thumb specific
        - [ ] arm64_specific.rs - AArch64 specific
    - [ ] Design unified instruction trait interface

2. **X86 Unification** [Week 3-4]
    - [ ] Merge common instruction definitions
        - [ ] Arithmetic (ADD, SUB, MUL, DIV) - 90% shared
        - [ ] Logic (AND, OR, XOR) - 100% shared
        - [ ] Control flow (JMP, CALL, Jcc) - 95% shared
    - [ ] Unified register handling with mode detection
    - [ ] Zero-extension handling for 64-bit mode
    - [ ] REX prefix processing

3. **ARM Unification** [Week 5-6]
    - [ ] Merge common ARM concepts
        - [ ] Condition codes (100% shared)
        - [ ] Data processing (80% shared)
        - [ ] Load/Store patterns (70% shared)
    - [ ] Handle instruction set differences (ARM/Thumb/AArch64)
    - [ ] Register mapping (R0-R15 vs X0-X30)

4. **Integration & Migration** [Week 7]
    - [ ] Update arch/mod.rs with unified modules
    - [ ] Create compatibility layer for smooth migration
    - [ ] Update all instruction analysis paths
    - [ ] Maintain backward compatibility during transition

5. **Testing & Validation** [Week 8]
    - [ ] Comprehensive instruction coverage tests
    - [ ] Cross-mode verification (32 vs 64-bit)
    - [ ] Performance benchmarks (no regression)
    - [ ] Determinism verification (1000 runs)

6. **Cleanup & Optimization** [Week 9]
    - [ ] Remove old architecture modules
    - [ ] Delete compatibility layer
    - [ ] Profile and optimize hot paths
    - [ ] Update all documentation

**Success Metrics**:

- 60-80% code reduction in architecture modules
- Zero performance regression (±5%)
- 100% test compatibility
- Single location for bug fixes

**Key Design Decisions**:

- Work WITH existing AST structure, not against it
- All optimizations happen at AST level, NOT IR level
- Hexadecimal output by default, user-configurable
- Architecture detection automatic from binary

#### ✅ Sprint 1: C Code Generation Quality (COMPLETED)

**Goal**: Produce compilable, readable C code from binaries

1. **Fix C Code Generation Issues** [COMPLETED]

- [x] Handle return statements from Low IR patterns ✅ DONE
- [x] Fix variable name mapping (currently showing as `result_0`, `a_1`, etc.) ✅ DONE
- [x] Generate proper parameter names (not `param_0`, `param_1`) ✅ DONE
- [x] Handle terminator instructions in pattern conversion ✅ DONE
- [x] Add variable type declarations (int result = ... instead of result = ...) ✅ DONE
- [x] Fix conditional branch code generation ✅ DONE
- [x] Fix condition expression generation (shows as 'temp') ✅ DONE

2. **Variable Naming & Type Recovery** [COMPLETED] ✅

- [x] Implement smart variable naming heuristics
  - [x] Use purpose field from LocalId
  - [x] Detect common patterns (loop counters → i,j,k)
  - [x] Function parameter semantic analysis
- [x] Basic type inference from usage patterns
- [x] Advanced type recovery system with confidence scoring
- [x] Library function signature matching
- [x] Array and struct detection patterns

3. **Code Quality Improvements** [Medium Priority]
  - [ ] Expression simplification (remove redundant parentheses)
  - [ ] Dead code elimination
  - [ ] Constant folding
  - [ ] Common subexpression elimination

#### 🎯 Sprint 2: Pattern Recognition Enhancement (PARTIAL)

**Goal**: Improve pattern detection accuracy and coverage

1. **Array Access Patterns** [FRAMEWORK COMPLETED]

- [x] Detect base[index] patterns - Framework implemented
- [x] Handle pointer arithmetic variations - Basic support added
- [ ] Multi-dimensional array support - Future work

**Note**: Array access pattern detection framework is implemented but needs refinement for complex cases

2. **Pattern Database Integration** [Medium Priority]
  - [ ] Design pattern storage format
  - [ ] Implement pattern matching engine
  - [ ] Add common stdlib patterns
  - [ ] Create pattern learning mechanism

3. **Advanced Patterns** [Low Priority]
  - [ ] String manipulation patterns
  - [ ] Memory allocation patterns
  - [ ] Error handling patterns

#### 🔒 Sprint 3: Determinism & Testing

**Goal**: Ensure 100% deterministic output

1. **Determinism Test Suite** [Critical]
  - [ ] Cross-platform determinism tests
  - [ ] Parallel execution tests (1-32 threads)
  - [ ] Memory pressure tests
  - [ ] Repeated execution verification (1000+ runs)

2. **Test Coverage** [High Priority]
  - [ ] Unit tests for all IR transformations
  - [ ] Integration tests for full pipeline
  - [ ] Regression test suite
  - [ ] Performance benchmarks

### 🎨 Enhanced C Generation Pipeline

```
Binary → Disassembly → Low IR → Medium IR → High IR → Enhanced C
                           ↓         ↓          ↓         ↓
                    [Deterministic] [Patterns] [Types] [Modern]
```

**Current Status**:

- ✅ Binary → Low IR: Working
- ✅ Low IR → Medium IR: Working
- ✅ Medium IR → High IR: Working
- ✅ High IR → Enhanced C: Implemented with modern features
- ✅ Enhanced C Features:
  - Auto type inference (C++11) for complex types
  - Fixed-width types (uint32_t, int64_t)
  - nullptr instead of NULL
  - Range-based for loops
  - Inline comments for uncertainty
  - Anonymous structs for memory layouts

### 🔧 Systematic Implementation Approach

**📚 Implementation Guides Created:**

- [`docs/implementation/c-generation-fixes.md`](docs/implementation/c-generation-fixes.md) - Complete fix guide
- [`docs/implementation/immediate-fixes-spec.md`](docs/implementation/immediate-fixes-spec.md) - Technical specification
- [`docs/implementation/execution-plan.md`](docs/implementation/execution-plan.md) - Week-by-week plan

#### Phase 1: Fix Core C Generation (Current Week)

```rust
// Critical Issues (2 bugs blocking everything):
1. Missing return statements - Pattern::LowIR lacks terminator info
2. Bad variable names - "param_0", "result_0" instead of meaningful names

// Root Cause:
Pattern::LowIR { instructions, confidence } // Missing terminator!
convert_low_ir_instructions() // Only processes instructions, not terminators

// Solution (2-day fix):
Day 1: Add terminator to Pattern, handle in High IR
Day 2: Fix variable naming using LocalId::purpose
Day 3: Test and verify all outputs compile
```

#### Phase 2: Smart Variable Naming

```rust
// Naming Strategy:
1. Parameters: Use function signature hints
2. Locals: Extract from LocalId purpose field
3. Temporaries: Context-based naming (loop_counter, condition, etc.)
4. Globals: Preserve original symbols when available

// Implementation:
- Create NameResolver trait
- Implement semantic analysis for common patterns
- Add name propagation through IR transformations
```

#### Phase 3: Pattern Enhancement

```rust
// Pattern Priority:
1. Array access: ptr + offset patterns
2. Struct access: base + field offset
3. Function pointers: indirect call patterns
4. String operations: strlen, strcpy patterns

// Database Design:
- YAML/JSON pattern definitions
- Confidence scoring per pattern
- Version control for pattern evolution
```

---

## Detailed Tasks
### 🔴 P0: Critical Foundation Tasks

#### x64 Instruction Coverage Enhancement
- [x] Basic SSE/AVX instructions (MOVAPS, ADDPS, MULPS, etc.)
- [x] Common instructions (MOVSX/MOVZX, NEG/NOT, CMOVcc)
- [x] String operations (MOVS*, STOS*, SCAS*)
- [x] Flag operations (SAHF, STC, SETCC)
- [x] Conditional moves (CMOVcc family)
- [x] **Basic x64 Instructions** (sahf, xchg, cmpxchg) ✅ VERIFIED
  - [x] SAHF - Store AH into Flags (implemented in s.rs)
  - [x] XCHG - Exchange operands (implemented in x.rs)
  - [x] CMPXCHG - Compare and exchange (implemented in c.rs)
- [ ] **LOCK Prefix Support** [High Priority]
  - [ ] Add prefix detection in instruction parsing
  - [ ] Create atomic IR operations
  - [ ] Model memory barriers/fences
  - [ ] Support for atomic variants (LOCK CMPXCHG, LOCK XADD, etc.)
- [ ] **Remaining x64 Instructions**
  - [ ] FPU instructions (FLD, FST, FADD, etc.)
  - [ ] Advanced SSE4/AVX2/AVX-512 instructions
  - [ ] System instructions (CPUID, RDTSC, etc.)
  - [ ] Rare/undocumented instructions
- [ ] **Instruction Prefix Handling**
  - [ ] REX prefix proper decoding
  - [ ] VEX prefix support
  - [ ] EVEX prefix support
  - [ ] Segment override prefixes

#### ARM Architecture Support (ARM64 & ARM32)
- [ ] Research ARM64 instruction set
- [ ] Design ARM-specific IR mappings
- [ ] Implement basic ARM64 decoder
- [ ] Add Thumb mode support

#### x86 (32-bits) Instruction Support

- [ ] x86 Instruction Support

### 🟠 P1: Extended Architecture Support

#### IR Analysis Infrastructure
- [x] **Loop Analysis** (Completed)
  - [x] Loop pattern recognition (for, while, do-while)
  - [x] Nested loop handling
  - [x] Loop invariant detection
  - [x] Iterator variable identification
  - [ ] Range-based loop support (C++11/C++20)
- [ ] **Advanced Analysis Passes**
  - [ ] Pointer alias analysis
  - [ ] Type propagation system
  - [ ] Value range analysis
  - [ ] Memory access pattern detection
  - [ ] Function signature recovery

#### Enhanced C Code Generation Pipeline

- [x] **Enhanced C Language Design** [COMPLETED]
  - [x] Minimal modern C++ features for readability
  - [x] Auto type inference for complex types
  - [x] Fixed-width integer types
  - [x] nullptr and range-based for loops
  - [x] Confidence-based feature usage
- [ ] **AST Generation**
  - [ ] IR → Enhanced C AST converter
  - [ ] Control flow reconstruction
  - [ ] Expression tree building
  - [ ] Type annotation system
- [ ] **Optimization Passes**
  - [ ] Expression simplification
  - [ ] Dead code elimination
  - [ ] Redundant cast removal
  - [ ] Common subexpression elimination
  - [ ] Constant folding
- [ ] **Code Quality**
  - [x] Variable naming heuristics
  - [ ] Idiomatic pattern generation
  - [x] Comment generation for uncertainty
  - [ ] Readable formatting rules
  - [ ] Macro reconstruction

#### Simulation & Verification Framework

- [x] **Core Simulation** (Basic implementation - TO BE REPLACED)
  - [x] CPU state emulation for x64
  - [x] Memory management simulation
  - [x] Basic symbolic execution
- [ ] **Unicorn Engine Integration** [PLANNED]
    - [ ] Replace custom simulation with Unicorn Engine
    - [ ] Support all architectures (x86, x86_64, ARM32, ARM64)
    - [ ] Leverage Unicorn's accurate CPU emulation
    - [ ] Hook-based instrumentation for analysis
    - [ ] Snapshot/restore for path exploration
- [ ] **Enhanced Analysis on Unicorn**
    - [ ] Taint analysis via Unicorn hooks
    - [ ] Dynamic type recovery during execution
    - [ ] Constraint collection for symbolic execution
    - [ ] Integration with Z3/CVC5 for path conditions
  - [ ] Concrete execution validation

### 🟡 P2: User Interface & Tools

#### GUI Enhancements
- [ ] Add IR modification capabilities
- [ ] Implement instruction editing
- [ ] Create memory/register simulation view
- [ ] Add breakpoint support for simulation
- [ ] Implement undo/redo for modifications
- [ ] **Architecture Selection UI** [NEW]
    - [ ] Add architecture dropdown/selector
    - [ ] Display detected architecture info
    - [ ] Allow manual architecture override
    - [ ] Show architecture-specific features

#### TUI Decompiler
- [ ] Design terminal UI framework
- [ ] Implement assembly view
- [ ] Add IR visualization
- [ ] Create navigation shortcuts
- [ ] Support mouse interaction
- [ ] **Numeric Format Toggle** [NEW]
    - [ ] Add hotkey for hex/dec/bin switching
    - [ ] Remember user preference
    - [ ] Show current format in status bar

#### CLI Decompiler
- [ ] Design command-line interface
- [ ] Add batch processing support
- [ ] Implement output format options
- [ ] Create scripting interface
- [ ] Add progress reporting
- [ ] **Architecture & Format Flags** [NEW]
    - [ ] --arch flag for manual architecture
    - [ ] --numeric-format flag (hex/dec/bin/auto)
    - [ ] --detect-arch for architecture info
    - [ ] --fixed-width-types toggle

### 🟢 P3: Advanced Analysis Features

#### IR Pattern Matching
- [ ] Design pattern description language
- [ ] Implement pattern matcher
- [ ] Create MSVC runtime patterns
- [ ] Add glibc function patterns
- [ ] Build pattern learning system

#### Optimizer Framework
- [ ] Design optimization pass system
- [ ] Implement peephole optimizer
- [ ] Add constant propagation
- [ ] Create alias analysis
- [ ] Build inlining heuristics

#### Deobfuscation
- [ ] Research obfuscation techniques
- [ ] Implement control flow unflattening
- [ ] Add opaque predicate detection
- [ ] Create VM-based obfuscation handler
- [ ] Build pattern-based deobfuscator

## 🏛️ Architecture-Specific Considerations [NEW SECTION]

### x86/x86_64 Unified Handling

- **Shared Components**:
    - Common instruction decoding logic
    - Unified memory addressing modes
    - Shared flag handling
- **Differences to Handle**:
    - Register sizes (32 vs 64-bit)
    - Default operand sizes
    - REX prefix handling (x86_64 only)
    - Calling conventions (cdecl/stdcall vs System V/Win64)

### ARM32/ARM64 Unified Handling

- **Shared Components**:
    - Condition code system
    - Load/store architecture principles
    - Similar instruction patterns
- **Differences to Handle**:
    - Register count (R0-R15 vs X0-X30)
    - Instruction encoding (32-bit vs fixed 32-bit)
    - Thumb mode (ARM32 only)
    - SIMD differences (NEON vs SVE)

### Cross-Architecture Patterns

- **Common Patterns**:
  ```rust
  // Function prologue detection
  match arch {
      X86 | X86_64 => detect_x86_prologue(),
      ARM32 | ARM64 => detect_arm_prologue(),
  }
  
  // Calling convention mapping
  let arg_regs = match (arch, platform) {
      (X86_64, Linux) => &["rdi", "rsi", "rdx", "rcx", "r8", "r9"],
      (X86_64, Windows) => &["rcx", "rdx", "r8", "r9"],
      (ARM64, _) => &["x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7"],
      _ => &[],
  };
  ```

### AST Generation Strategy

- **Architecture-Aware Types**:
    - size_t → uint32_t or uint64_t based on arch
    - intptr_t → proper sized integer
    - Pointer sizes in struct layouts
- **Numeric Display**:
    - Addresses: Always hex with proper width (8 or 16 digits)
    - Small constants: Decimal by default
    - Bit patterns: Hex or binary based on context
    - User override: Global format preference

## 📊 Technical Debt & Infrastructure

### 🔴 Code Quality & Reliability
- [ ] **Error Handling**
  - [ ] Comprehensive error types for each module
  - [ ] Error recovery strategies
  - [ ] Graceful degradation paths
- [ ] **Performance & Profiling**
  - [ ] Create performance benchmark suite
  - [ ] Memory usage profiling
  - [ ] Optimization hotspot identification
  - [ ] Zero-copy architecture validation
- [ ] **Build & CI/CD**
  - [ ] GitHub Actions workflow
  - [ ] Cross-platform build matrix
  - [ ] Automated release pipeline
  - [ ] Dependency security scanning

### 🟠 Documentation & Knowledge Base
- [x] Architecture documentation (partial)
- [x] IR specification document
- [ ] **Developer Documentation**
  - [ ] API reference generation
  - [ ] Plugin development guide
  - [ ] Contributing guidelines
  - [ ] Code style guide

### 🟡 Testing Infrastructure
- [ ] **Unit Testing** (Target: 80% coverage)
  - [ ] IR generation tests
  - [ ] Pattern matching tests
  - [ ] C generation tests
- [ ] **Integration Testing**
  - [ ] End-to-end decompilation tests
  - [ ] Cross-architecture tests
  - [ ] Large binary test suite
- [ ] **Specialized Testing**
  - [ ] Determinism verification suite
  - [ ] Differential testing against other decompilers
  - [ ] Fuzzing harness for robustness
  - [ ] Performance regression tests
  - [ ] Memory leak detection

## 📝 Quick Task Reference

### Immediate Action Items (COMPLETED ✅)

1. **C Code Generation Fixes** [COMPLETED]
   ```rust
   // Previous output:
   int sub_1000(int param_0, int param_1) {
       result_0 = (a_1 + b_2);  // Missing return, bad names
   }

   // Current output:
   int sub_1000(int a, int b) {
       int result = (a + b);
       return result;
   }
   ```

2. **Variable Naming System** [COMPLETED] ✅

- Map LocalId purpose → variable name ✅
- Parameter naming from signatures ✅
- Loop counter detection ✅
- Type-based naming (ptr, flag, arr, etc.) ✅
- Usage-based naming patterns ✅

3. **Pattern Test Suite** [1 day]
  - Create golden test files
  - Verify pattern detection accuracy
  - Benchmark performance

### Sprint Planning

**Sprint 1 (COMPLETED)**: C Code Quality ✅

- Week 1: Fix critical C generation bugs ✅
- Variable naming from LocalId purpose ✅
- Type declarations and return statements ✅

**Sprint 2 (COMPLETED)**: x86_64 Instruction Coverage ✅

- Week 2: Fix remaining x86_64 instructions (sahf, xchg, lock cmpxchg) ✅
  - SAHF: Already implemented in s.rs ✅
  - XCHG: Already implemented in x.rs ✅
  - CMPXCHG: Already implemented in c.rs ✅
  - LOCK prefix: Identified as separate feature needed ⚠️
- Week 2: Add comprehensive instruction tests ✅
- Week 2: Document instruction semantics ✅

**Sprint 3**: Pattern Recognition & Type Recovery [COMPLETED] ✅

- Week 3: Pattern database design ⚠️ (Partial - framework exists)
- Week 3: Type recovery system ✅
- Week 3: Struct/class reconstruction ✅

**Sprint 4**: Determinism & Polish

- Week 4: Determinism test suite
- Week 4: Performance optimization
- Week 4: Documentation & release prep

### Success Metrics

1. **Code Generation**
  - [ ] 100% of tests produce compilable C code
  - [ ] Variable names match semantic purpose
  - [ ] No syntax errors in output

2. **Pattern Detection**
  - [ ] 95%+ accuracy on common patterns
  - [ ] <100ms pattern matching time
  - [ ] Zero false positives

3. **Determinism**
  - [ ] 1000 runs = identical output
  - [ ] Cross-platform consistency
  - [ ] Thread-count independent

## 🗺️ Project Roadmap

- ✅ Multi-level IR architecture
- ✅ Basic C code generation
- 🚧 C code quality improvements
- 🚧 Pattern recognition system

- [ ] Full x86_64 instruction coverage
- [ ] Advanced pattern matching
- [x] Type recovery system ✅
- [x] Struct/class reconstruction ✅
- [ ] ARM architecture support
- [ ] Deobfuscation capabilities

## 🧹 Final Cleanup Phase [LOWEST PRIORITY]

**Goal**: Remove all redundancies and unused code after all other tasks are complete

### When to Execute

- **Prerequisite**: ALL other tasks must be 100% complete
- **Timing**: Final phase before v1.0 release
- **Duration**: 1-2 weeks

### Cleanup Tasks

1. **Code Redundancy Removal**
    - [ ] Audit all modules for dead code using `cargo-udeps`
    - [ ] Remove unused functions, types, and imports
    - [ ] Consolidate duplicate logic across modules
    - [ ] Remove commented-out code blocks
    - [ ] Clean up `#[allow(dead_code)]` annotations

2. **Module Consolidation**
    - [ ] Merge small modules with <100 lines
    - [ ] Combine related utility functions
    - [ ] Flatten unnecessary module hierarchies
    - [ ] Remove empty or single-item modules

3. **Documentation Cleanup**
    - [ ] Remove outdated documentation
    - [ ] Delete obsolete design docs
    - [ ] Archive old implementation notes
    - [ ] Update README with current state
    - [ ] Remove TODO comments that are done

4. **Test Cleanup**
    - [ ] Remove duplicate test cases
    - [ ] Delete obsolete test fixtures
    - [ ] Consolidate similar test files
    - [ ] Remove unused test utilities
    - [ ] Clean up test data files

5. **Build & Dependencies**
    - [ ] Remove unused dependencies
    - [ ] Clean up feature flags
    - [ ] Remove build script hacks
    - [ ] Delete temporary compatibility code
    - [ ] Optimize compile times

6. **Final Verification**
    - [ ] Run full test suite
    - [ ] Verify no functionality lost
    - [ ] Check binary size reduction
    - [ ] Measure compile time improvement
    - [ ] Document what was removed

### Cleanup Checklist

```bash
# Tools to use
cargo udeps          # Find unused dependencies
cargo bloat          # Analyze binary size
cargo machete        # Find unused code
cargo audit          # Security check
cargo clippy         # Final lint pass
```

### Expected Outcomes

- 20-30% reduction in codebase size
- Faster compile times
- Cleaner module structure
- Easier maintenance
- Better documentation

### Important Notes

- **DO NOT** remove code that might be useful later
- **DO NOT** optimize prematurely
- **DO NOT** break public APIs
- **ALWAYS** run full tests after each removal
- **DOCUMENT** why something was removed

This cleanup phase ensures the codebase remains maintainable and efficient after rapid development phases.

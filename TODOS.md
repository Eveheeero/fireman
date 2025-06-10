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
- [x] **Core Simulation** (Completed)
  - [x] CPU state emulation for x64
  - [x] Memory management simulation
  - [x] Basic symbolic execution
- [ ] **Enhanced Simulation**
  - [ ] Taint analysis integration
  - [ ] Path exploration strategies
  - [ ] Constraint solver integration (Z3/CVC5)
  - [ ] Concrete execution validation
  - [ ] Simulation-guided decompilation

### 🟡 P2: User Interface & Tools

#### GUI Enhancements
- [ ] Add IR modification capabilities
- [ ] Implement instruction editing
- [ ] Create memory/register simulation view
- [ ] Add breakpoint support for simulation
- [ ] Implement undo/redo for modifications

#### TUI Decompiler
- [ ] Design terminal UI framework
- [ ] Implement assembly view
- [ ] Add IR visualization
- [ ] Create navigation shortcuts
- [ ] Support mouse interaction

#### CLI Decompiler
- [ ] Design command-line interface
- [ ] Add batch processing support
- [ ] Implement output format options
- [ ] Create scripting interface
- [ ] Add progress reporting

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

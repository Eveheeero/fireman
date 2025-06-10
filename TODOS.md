# Fireman - Features & Plans

Working In Progress (2025.01 Updated)

## 🎯 Project Status Overview

**Current Focus**: Implementing Binary-to-Enhanced-C Decompiler with Absolute Determinism

### ✅ Completed Components
- [x] Basic IR Environment Foundation
- [x] X64 Instruction Parsing (Most common instructions)
- [x] Control Flow Analysis with Complex Loop Detection
- [x] Data Flow Analysis (Reaching Definitions, Liveness)
- [x] Basic Simulation Framework (CPU State, Memory)
- [x] Deterministic Infrastructure (Address-based naming, BTreeMap usage)

### 🚧 In Progress
- [ ] Multi-Level IR Implementation (Low → Medium → High)
- [ ] Enhanced C Code Generation with Optimizations
- [ ] Complete Determinism Testing Suite

### 📋 Planned Features
- [ ] ARM Architecture Support
- [ ] GUI/TUI/CLI Interfaces
- [ ] IR Pattern Matching for Library Functions
- [ ] Advanced Deobfuscation

---

## 🔴 P0: CRITICAL - Deterministic Binary-to-Enhanced-C Pipeline

### Deterministic Foundation (In Progress)
- [x] Create deterministic naming system (address-based)
- [x] Replace all HashMap with BTreeMap
- [x] Implement deterministic ID generation
- [x] Add SHA-256 based determinism testing
- [ ] Complete determinism test suite
  - [ ] Cross-platform determinism tests
  - [ ] Parallel execution determinism tests
  - [ ] Memory pressure determinism tests

### Multi-Level IR Implementation
- [ ] **Low IR (Direct Translation)**
  - [ ] Define complete Low IR statement types
  - [ ] Implement x64 → Low IR lifter
  - [ ] Add flag preservation logic
  - [ ] Create SSA construction with deterministic phi placement
  - [ ] Test: Verify 1:1 mapping with assembly

- [ ] **Medium IR (Pattern Recognition)**
  - [ ] Design pattern matching framework
  - [ ] Implement basic patterns (loops, function calls, switches)
  - [ ] Add confidence scoring system
  - [ ] Create pattern database integration
  - [ ] Test: Pattern detection accuracy

- [ ] **High IR (Source-Like)**
  - [ ] Implement type recovery system
  - [ ] Add high-level construct generation
  - [ ] Create variable naming heuristics
  - [ ] Build struct/class reconstruction
  - [ ] Test: Readability metrics

### Enhanced C Generation
- [ ] Implement IR → C AST converter
- [ ] Add expression simplification
- [ ] Create idiomatic C pattern generation
- [ ] Implement comment generation for uncertainty
- [ ] Test: Compilability of generated code

---

## Detailed Tasks
### 🔴 P0: Critical Foundation Tasks

#### x64 Instruction Coverage Enhancement
- [x] Basic SSE/AVX instructions (MOVAPS, ADDPS, MULPS, etc.)
- [x] Common instructions (MOVSX/MOVZX, NEG/NOT, CMOVcc)
- [x] String operations (MOVS*, STOS*, SCAS*)
- [x] Flag operations (SAHF, STC, SETCC)
- [x] Conditional moves (CMOVcc family)
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

#### ARM Architecture Support
- [ ] Research ARM64 instruction set
- [ ] Design ARM-specific IR mappings
- [ ] Implement basic ARM64 decoder
- [ ] Add Thumb mode support

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
- [ ] **AST Generation**
  - [ ] IR → C AST converter implementation
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
  - [ ] Variable naming heuristics
  - [ ] Idiomatic C pattern generation
  - [ ] Comment generation for uncertainty
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
- [ ] **User Documentation**
  - [ ] Usage tutorials
  - [ ] Example walkthroughs
  - [ ] Video tutorials
  - [ ] FAQ compilation

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

## 🚀 Future Roadmap

### Phase 1: Production-Ready Core (Q1 2025)
- Complete deterministic Binary-to-Enhanced-C pipeline
- Achieve 95% x64 instruction coverage
- Release v0.1.0 with CLI interface

### Phase 2: Multi-Architecture (Q2 2025)
- ARM64 support
- ELF file format support
- GUI application beta

### Phase 3: Advanced Features (Q3-Q4 2025)
- ML-powered pattern recognition
- Advanced deobfuscation
- Plugin system
- Cloud-based analysis

## 📝 Quick Task Reference

### Immediate Next Steps
1. Run and fix determinism tests
2. Implement Low IR statement types
3. Create x64 → Low IR lifter
4. Build Medium IR pattern matcher
5. Implement High IR generator

### Weekly Goals
- Week 1: Complete Low IR implementation
- Week 2: Medium IR pattern framework
- Week 3: High IR and C generation
- Week 4: Testing and optimization

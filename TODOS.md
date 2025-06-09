# Fireman Decompiler - TODO List

## Priority Levels
- 🔴 **P0**: Critical - Blocking other work
- 🟠 **P1**: High - Core functionality
- 🟡 **P2**: Medium - Important features
- 🟢 **P3**: Low - Nice to have

## Immediate Tasks (From README.md)

### 🔴 P0: Critical Foundation

#### Complete x64 Instruction Coverage
- [ ] Implement remaining SSE/AVX instructions
- [ ] Add support for rare/undocumented instructions
- [ ] Handle instruction prefixes properly (REX, VEX, EVEX)

#### ARM Architecture Support
- [ ] Research ARM64 instruction set
- [ ] Design ARM-specific IR mappings
- [ ] Implement basic ARM64 decoder
- [ ] Add Thumb mode support

### 🟠 P1: Core Decompilation

#### Complex Loop Analysis
- [ ] Implement loop pattern recognition (for, while, do-while)
- [ ] Handle nested loops correctly
- [ ] Detect loop invariants
- [ ] Identify iterator variables
- [ ] Support range-based loops (C++11)

#### C Code Generation Optimization
- [ ] Implement expression simplification
- [ ] Add dead code elimination
- [ ] Optimize redundant casts
- [ ] Improve variable naming heuristics
- [ ] Generate idiomatic C patterns

#### Simulation Routine
- [ ] Design simulation framework
- [ ] Implement x64 CPU state emulation
- [ ] Add memory management simulation
- [ ] Create symbolic execution engine
- [ ] Build constraint solver integration

### 🟡 P2: User Interface

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

### 🟢 P3: Advanced Features

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

## Technical Debt

### 🔴 Code Quality
- [ ] Add comprehensive error handling
- [ ] Improve logging infrastructure
- [ ] Create performance benchmarks
- [ ] Add fuzzing harness
- [ ] Implement CI/CD pipeline

### 🟠 Documentation
- [ ] Write architecture documentation
- [ ] Create plugin development guide
- [ ] Add code examples
- [ ] Document IR specification
- [ ] Create video tutorials

### 🟡 Testing
- [ ] Achieve 80% test coverage
- [ ] Add integration tests
- [ ] Create regression test suite
- [ ] Implement differential testing
- [ ] Add performance tests

## Feature Roadmap

### Q1 2025: Foundation
- [x] Basic IR generation
- [x] Simple C output
- [ ] Complete x64 support
- [ ] Robust error handling
- [ ] Plugin system design

### Q2 2025: Core Features
- [ ] Advanced control flow
- [ ] Type recovery system
- [ ] Variable analysis
- [ ] Function detection
- [ ] Import reconstruction

### Q3 2025: Performance
- [ ] Parallel analysis
- [ ] Incremental updates
- [ ] Memory optimization
- [ ] Cache system
- [ ] Streaming mode

### Q4 2025: Polish
- [ ] Production GUI
- [ ] Plugin ecosystem
- [ ] Cloud features
- [ ] AI integration
- [ ] Commercial support

## Research Tasks

### Algorithm Research
- [ ] Study latest CFG reconstruction papers
- [ ] Investigate neural decompilation
- [ ] Research type inference algorithms
- [ ] Explore symbolic execution optimizations
- [ ] Analyze compiler fingerprinting

### Tool Analysis
- [ ] Reverse engineer Hex-Rays microcode
- [ ] Study Ghidra's P-code design
- [ ] Analyze Binary Ninja's IL layers
- [ ] Investigate radare2's ESIL
- [ ] Learn from angr's VEX usage

### Performance Studies
- [ ] Profile current bottlenecks
- [ ] Benchmark against competition
- [ ] Identify optimization opportunities
- [ ] Study memory usage patterns
- [ ] Analyze cache behavior

## Implementation Notes

### Dependencies to Evaluate
- **Intel XED**: Better x86 decoding than Capstone
- **MLIR**: Possible IR framework (but might be overkill)
- **Z3**: SMT solver for type inference
- **Tree-sitter**: For C output formatting
- **WASM**: For browser-based version

### Design Decisions Needed
1. **IR Format**: Binary vs text representation
2. **Plugin API**: FFI vs WASM vs native
3. **Storage**: Database vs custom format
4. **Parallelism**: Rayon vs custom thread pool
5. **GUI Framework**: Keep Tauri vs native

### Performance Goals
- Startup time: <100ms
- First function: <500ms
- Full analysis: <30s for 10MB
- Memory usage: <10x binary size
- Incremental update: <100ms

## Weekly Sprint Plan

### Week 1-2
- Complete x64 instruction parsing
- Fix critical bugs in IR generation
- Set up comprehensive testing

### Week 3-4
- Implement loop detection
- Add expression simplification
- Create benchmarking suite

### Week 5-6
- Design plugin system
- Implement first optimization pass
- Start ARM architecture research

### Week 7-8
- Build TUI prototype
- Add IR modification support
- Create pattern matching framework

## Remember

> "Perfect is the enemy of good. Ship early, ship often, and listen to users."

Focus on what matters: making reverse engineering faster and easier for everyone.
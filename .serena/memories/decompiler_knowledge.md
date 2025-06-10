# Decompiler Domain Knowledge

## Core Pipeline

Binary → PE Parser → Disassembler → Custom IR Generation → Analysis → C Code

## Key Concepts

### IR Design for Decompilation

- Not like LLVM (compilation IR) - needs to capture low-level semantics
- Preserve implicit information (CPU flags, side effects)
- Support gradual refinement from low to high level
- 3-address code format for simplicity

### Industry Techniques

1. **Hex-Rays**: Microcode approach (~40 ops), aggressive optimization
2. **Ghidra**: P-code with ~400 operations for precision
3. **Binary Ninja**: LLIL→MLIL→HLIL progressive lifting
4. **IDA Pro**: FLIRT signatures for library detection

### Analysis Algorithms

- **Control Flow**: Cifuentes' structural analysis, T1-T2 transformations
- **Data Flow**: SSA form, reaching definitions, liveness analysis
- **Type Recovery**: Constraint-based inference, VSA (Value Set Analysis)
- **Pattern Matching**: Fuzzy matching for compiler idioms

### Performance Strategies

- Incremental analysis (only reanalyze changes)
- Parallel processing for functions
- Caching at multiple levels
- Streaming architecture for large binaries

### Common Challenges

- Indirect jumps (switch statements, function pointers)
- Obfuscated code (control flow flattening, VM protection)
- Optimized code (inlined functions, loop transformations)
- Missing type information (stripped binaries)

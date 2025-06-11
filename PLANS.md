# Fireman Decompiler - Project Development Plans

## Executive Summary

Fireman aims to be the decompiler that reverse engineers actually want to use. We're building on what works (Hex-Rays' production quality, Ghidra's openness, Binary Ninja's modern architecture) while fixing what doesn't (speed, accuracy, usability). Our secret weapon? Rust's performance + safety, Unicorn Engine's battle-tested emulation, and a pragmatic approach to innovation.

## The Problems We're Actually Solving

### What's Wrong with Current Decompilers?
1. **Hex-Rays**: Expensive, closed-source, slow on large binaries
2. **Ghidra**: Java overhead, complex P-code, steep learning curve
3. **Binary Ninja**: Better but still proprietary, limited free version
4. **RetDec**: Overly complex, poor output quality
5. **Radare2**: Powerful but fragmented, inconsistent

### What Reverse Engineers Really Need
- **Fast**: Decompile a 10MB binary in under 30 seconds
- **Accurate**: Output that actually looks like the original source
- **Interactive**: Change something, see results immediately
- **Extensible**: Easy to add custom analysis without recompiling
- **Free**: Open source with no artificial limitations

## Our Approach: Practical Innovation

### Core Philosophy
1. **Steal Shamelessly**: Take the best ideas from everywhere
2. **Simplify Ruthlessly**: If it's too complex, it's wrong
3. **Optimize Intelligently**: Fast where it matters, correct everywhere
4. **Fail Gracefully**: Better partial output than crashes

### Technical Strategy

#### 1. Smart IR Design (Not Another LLVM)
Our IR is designed specifically for decompilation:
- **3-Address Code**: Simple, easy to analyze and transform
- **High-Level Operations**: One IR op can represent complex x86 patterns
- **Metadata Rich**: Carry source information through all passes
- **SSA When Useful**: Not religious about it - use where it helps

Example: Instead of modeling every x86 flag individually:
```
// Bad: Too low level
t1 = add r1, r2
cf = carry_flag(t1)
zf = zero_flag(t1)

// Good: Practical abstraction
t1 = add r1, r2 with_flags [carry, zero]
```

#### 2. Incremental Everything
- **Incremental Parsing**: Only reparse changed functions
- **Incremental Analysis**: Cache results, update on demand
- **Incremental UI**: Smooth updates as analysis progresses

#### 3. Pattern Recognition That Works
Instead of complex academic approaches, we use:
- **Fuzzy Matching**: "Close enough" is often good enough
- **Statistical Learning**: Learn patterns from real binaries
- **User Feedback Loop**: Let users correct and teach

#### 4. Dynamic Analysis with Unicorn Engine
Instead of building custom simulation from scratch:
- **[Unicorn Engine Integration](https://docs.rs/unicorn-engine/latest/unicorn_engine/)**: Official Rust bindings (v2.1.3)
- **Multi-Architecture Support**: x86, x86_64, ARM, ARM64, MIPS, RISC-V in one framework
- **Hook-Based Analysis**: Memory access tracking, instruction tracing, syscall interception
- **Safe Execution**: Sandboxed environment for analyzing potentially malicious code
- **Performance**: Optimized CPU emulation for large binary analysis

## Development Roadmap (Practical Edition)

### Phase 1: Make It Work (Q1 2025)

#### Week 1-4: IR That Doesn't Suck ✅

- Design IR for real x86/x64 (not textbook RISC) ✅
- Handle the weird stuff: partial register updates, flags, segments ✅
- Make it debuggable: good pretty-printing, verification ✅

#### Week 5-8: Complete x64 Support (In Progress)
- Use Intel XED for decoding (better than Capstone for x86)
- Handle ALL instructions, even the weird ones (Partially complete)
- Test on real malware, packers, obfuscated code

#### Week 9-12: Analysis That Matters (In Progress)

- **Data Flow**: But only what helps readability ✅
- **Type Recovery**: Focus on structs and arrays first ✅
- **Variable Names**: Use debug info when available, ML-assisted otherwise ✅

#### Sprint 6: Unified Instruction Handling & Testing ✅

**Status: COMPLETED**

- Created common instruction interface for all architectures ✅
- Mapped architecture-specific instructions to common IR ✅
- Handled architecture-specific calling conventions ✅
- Supported architecture-specific optimizations at AST level ✅
- Implemented x86-64 as superset handler for x86-32 ✅
- Added atomic operation support (LOCK prefix) ✅
- Created comprehensive test suite ✅
- Verified deterministic output across architectures ✅

### Phase 2: Make It Good (Q2 2025)

#### Month 1: Control Flow Recovery
- **Loops**: Detect common patterns (for, while, do-while)
- **Switches**: Both jump tables and if-else chains
- **Exceptions**: Try-catch detection for Enhanced C and SEH

Real innovation: Use compiler fingerprinting to guide recovery

**Unicorn Engine Example for Type Recovery**:
```rust
use unicorn_engine::{Unicorn, RegisterX86};
use unicorn_engine::unicorn_const::{Arch, Mode, Permission};

// Hook memory access to understand struct layouts
let mut emu = Unicorn::new(Arch::X86, Mode::MODE_64)?;
emu.mem_map(0x400000, 0x1000, Permission::ALL)?;

// Hook to track memory accesses for type inference
let hook = emu.add_mem_hook(HookType::MEM_READ | HookType::MEM_WRITE,
    0x400000, 0x401000, |_uc, mem_type, address, size, value| {
        // Track access patterns to infer struct fields
        println!("Memory access: {:x} size {} type {:?}", address, size, mem_type);
    })?;

// Execute and learn from dynamic behavior
emu.emu_start(0x400000, 0x400100, 10 * SECOND_SCALE, 1000)?;
```

#### Month 2: Type System That Works
- **Start Simple**: int, pointer, struct, array
- **Constraint Solving**: But with timeouts and fallbacks
- **Import Signatures**: Steal type info from headers
- **Dynamic Hints**: Use runtime behavior when available

#### Unicorn Engine Integration (Concurrent with Phase 2)
- **Replace Custom Simulation**: Migrate from `fireball/src/simulation/` to `unicorn_engine` crate
- **Hook Implementation**: Memory access, instruction execution, syscall interception
- **Multi-Architecture Support**: Unified interface for x86/ARM dynamic analysis
- **Type Recovery Enhancement**: Use dynamic analysis to improve static type inference
- **Malware Analysis**: Safe execution environment for reverse engineering hostile code

#### Month 3: Output Quality
- **Readable Names**: v1, v2 → counter, buffer, result
- **Idiomatic Code**: Recognize and restore common patterns
- **Comment Generation**: Explain non-obvious transformations

### Phase 3: Make It Fast (Q3 2025)

#### Parallel Analysis Pipeline
```
Binary → [Parallel Disasm] → [Queue] → [Parallel Analysis] → [Merge] → Output
           ↓                             ↓
        [Cache]                      [Incremental Update]
```

#### Performance Targets
- 1MB binary: <2 seconds (like opening a text file)
- 10MB binary: <20 seconds (like compiling medium project)
- 100MB binary: <3 minutes (still faster than IDA)

#### Memory Efficiency
- Streaming architecture: Don't load entire binary
- Compressed IR: 10x smaller than raw instructions
- Lazy evaluation: Analyze on demand
- Unicorn Engine optimization: JIT compilation for repeated execution paths

### Phase 4: Make It Awesome (Q4 2025)

#### Killer Features
1. **Time Travel Debugging**: Step through decompilation process
2. **Collaborative RE**: Google Docs for reverse engineering
3. **AI Assistant**: Not replacing RE, but handling tedious parts
4. **Cross-Binary Diffing**: What changed between versions?

#### Migration Timeline: Custom Simulation → Unicorn Engine

**Current State**: `fireball/src/simulation/` module is marked as DEPRECATED
**Target State**: Replace with `unicorn_engine` crate integration

**Migration Steps**:
1. **Add Dependency**: Include `unicorn_engine = "2.1.3"` in Cargo.toml
2. **Create Wrapper**: Build safe Rust wrapper around Unicorn for our use cases
3. **Hook Implementation**: Memory access tracking, instruction tracing, syscall hooks
4. **API Compatibility**: Maintain existing `SimulationContext` interface where possible
5. **Remove Legacy**: Delete custom simulation module after verification
6. **Testing**: Ensure deterministic behavior across architectures

## Secret Sauce: Industry Techniques

### From Hex-Rays
- Microcode approach but simpler (~50 ops vs their ~40)
- Aggressive dead code elimination
- Pattern-based optimizations

### From Binary Ninja
- LLIL → MLIL → HLIL pipeline, but in 2 stages not 3
- Confidence scores for uncertain transformations
- Plugin architecture from day one

### From Ghidra
- Sleigh's idea but better syntax
- Overlapping instruction handling
- Architecture description files

### From Unicorn Engine
- **Cross-platform CPU emulation**: Support for 8 architectures in one framework
- **Hook-based analysis**: Intercept and analyze execution at instruction/memory/exception level
- **Production quality**: Used by major security tools and malware analysis platforms
- **Rust integration**: Official bindings with safe wrappers around C engine

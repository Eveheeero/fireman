# Fireman Decompiler - Project Development Plans

## Executive Summary

Fireman aims to be the decompiler that reverse engineers actually want to use. We're building on what works (Hex-Rays' production quality, Ghidra's openness, Binary Ninja's modern architecture) while fixing what doesn't (speed, accuracy, usability). Our secret weapon? Rust's performance + safety, and a pragmatic approach to innovation.

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

#### Month 2: Type System That Works
- **Start Simple**: int, pointer, struct, array
- **Constraint Solving**: But with timeouts and fallbacks
- **Import Signatures**: Steal type info from headers
- **Dynamic Hints**: Use runtime behavior when available

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

### Phase 4: Make It Awesome (Q4 2025)

#### Killer Features
1. **Time Travel Debugging**: Step through decompilation process
2. **Collaborative RE**: Google Docs for reverse engineering
3. **AI Assistant**: Not replacing RE, but handling tedious parts
4. **Cross-Binary Diffing**: What changed between versions?

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

### Our Innovation
1. **Compiler Fingerprinting**: Detect exact compiler version
2. **Learning from Symbols**: When available, learn patterns
3. **Differential Analysis**: Compare similar functions
4. **Confidence Propagation**: Track uncertainty through analysis

## Practical Metrics

### What Success Looks Like
- **Adoption**: 10k+ users in first year
- **Quality**: 80% of functions "look right" without editing
- **Performance**: Faster than IDA on 90% of binaries
- **Community**: 50+ contributors, 200+ plugins

### What We're NOT Doing
- ❌ Formal verification (too slow)
- ❌ 100% accuracy (impossible)
- ❌ Every architecture (x86/ARM first)
- ❌ Academic purity (practical > perfect)

## Risk Management

### Technical Risks
1. **Performance**: Continuous profiling, benchmarking
2. **Complexity**: Regular refactoring, code reviews
3. **Compatibility**: Test on real-world binaries daily

### Mitigation Strategies
- **Fail Fast**: Prototype risky features quickly
- **User Testing**: Get feedback early and often
- **Incremental Delivery**: Ship working features ASAP

## The Competition

### How We'll Win
1. **vs Hex-Rays**: Free, open source, faster
2. **vs Ghidra**: Native performance, better UX, simpler
3. **vs Binary Ninja**: Fully free, equally powerful
4. **vs Others**: Actually maintained, modern codebase

## Call to Action

We're building the decompiler we wish existed. Join us if you:
- Want readable decompilation output
- Care about performance and UX
- Believe in open source
- Have real-world RE experience to share

Together, we'll make reverse engineering accessible to everyone.

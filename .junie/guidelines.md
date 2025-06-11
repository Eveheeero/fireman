# Agentic Instructions

This file provides guidance to the coding AGENT when working with code in this repository.

## Project Overview

Fireman is a high-performance, deterministic decompiler framework written in Rust. It transforms binary executables into
readable, Enhanced C code through a sophisticated multi-stage pipeline: Binary → Disassembly → Multi-Level IR →
Analysis → Enhanced C Output.

**Enhanced C**: A tailored C-like language that minimally leverages modern C++ features to improve readability while
preserving decompiler characteristics and low-level details.

**Core Design Principles**:

1. **Absolute Determinism**: Same binary input ALWAYS produces identical output
2. **Zero-Copy Performance**: Memory-mapped files, arena allocation, streaming analysis
3. **Accuracy First**: Never guess - mark uncertainty explicitly
4. **Human Readability**: Output optimized for human understanding

### Workspace Structure

```
fireman/                 # Workspace root
├── fireman/            # CLI executable
├── fireball/           # Core decompiler library (main logic)
├── firebat/            # Tauri-based GUI application
├── iceball/            # Disassembly library (work in progress)
├── dryice/             # IR pattern matching framework (reserved for future use)
└── fireman_macro/      # Procedural macros
```

The project uses Cargo workspace features with centralized dependency management. All common dependencies are defined in
the root `Cargo.toml` and inherited by subcrates using `.workspace = true`.

Note: The `iceball/architecture_doc_extractor` subcrate is excluded from the workspace as it has special dependency
requirements.

## Development Commands

### Testing
```bash
# Run all tests in workspace
cargo test --workspace

# Run tests for specific package
cargo test -p fireball

# Run tests with output visible
cargo test --workspace -- --nocapture
```

### Code Quality
```bash
# Format code (required before commits)
cargo fmt --all

# Lint code
cargo clippy --workspace --tests --all-features

# Check compilation without building
cargo check --workspace --tests --all-features
```

### GUI Development
```bash
# Run GUI in development mode
cd firebat
npm install  # first time only
npm run tauri dev

# Build GUI for production
npm run tauri build
```

## Architecture Overview

The decompilation process follows a sophisticated multi-stage pipeline:
```
Binary File → PE Parser → Disassembler → IR Generation → Analysis → AST Generation → Enhanced C Output
                                              ↓                           ↓
                                        GUI Visualization          AST Optimizations
```

**Key Point**: Optimizations happen at the AST level, not during IR processing. The IR levels are for analysis only.

### Multi-Level IR Design

1. **Low IR**: Direct instruction translation, preserves all semantics
2. **Medium IR**: Pattern recognition, confidence tracking (NO optimizations here)
3. **High IR**: Near-source representation with recovered types and structures

**IMPORTANT**: All optimizations happen at the AST level, NOT at the IR level. The IR stages are purely for analysis and
representation.

### Important Types

- `Fire`: Main decompiler interface that orchestrates the analysis
- `Block`: Basic block containing sequential instructions
- `IR::Statement`: Intermediate representation of operations
- `CAbstractSyntaxTree`: Represents generated C code

## Current Implementation Status

✅ Implemented:
- x86_64 instruction parsing
- Basic block detection
- Control flow graph construction
- IR generation for common instructions
- Basic data flow analysis
- Enhanced C code generation from IR
    - Auto type inference for complex types
    - Fixed-width integer types (uint32_t, int64_t)
    - nullptr instead of NULL
    - Range-based for loops
    - Confidence-based feature usage
- Advanced type recovery system
- Variable naming heuristics
- Struct/class reconstruction
- GUI for visualizing assembly and IR

🚧 In Progress:
- Complete x86_64 instruction coverage
- Advanced IR optimizations
- Symbol resolution

📋 Planned:
- ARM architecture support
- Unify 32-bit and 64-bit implementations for ARM to reduce code duplication by 60-80%
- ELF file format support
- Advanced decompilation patterns
- Code simulation capabilities
- Clean old redunduncies

### Architecture Support Strategy

1. **Unified Architecture Interface**:

- `ArchitectureInfo` structure contains arch type, pointer size, endianness, register count
- Architecture detection from binary headers (PE, ELF, Mach-O)
- Architecture-aware type sizing and instruction alignment

2. **AST-Level Optimization** (NOT IR-level):

- Enhanced C generation works directly with the C Abstract Syntax Tree
- Architecture-specific details handled during AST construction
- Preserves AST structure for further analysis and optimization

3. **Numeric Display Configuration**:

- Default: Hexadecimal output for addresses and large values
- User-configurable: Decimal, Binary, or Auto-detection modes
- Architecture-aware formatting (32-bit vs 64-bit addresses)

### Implementation Guidelines

```rust
// Architecture-aware type selection
match arch.pointer_size {
32 => CType::UInt32,  // or "unsigned int" based on config
64 => CType::UInt64,  // or "unsigned long" based on config
_ => CType::UInt,
}

// Unified instruction handling
match arch.arch_type {
ArchType::X86 | ArchType::X86_64 => x86_handler(),
ArchType::Arm32 | ArchType::Arm64 => arm_handler(),
_ => generic_handler(),
}
```

## Critical: Determinism Requirements

**ABSOLUTE RULE**: The decompiler MUST produce byte-for-byte identical output for identical input, regardless of:

- Machine architecture, available memory, CPU cores
- Previous runs, system load, time of day
- Thread scheduling, hash function seeds

### Mandatory Practices

1. **Data Structures**:
   ```rust
   // ❌ NEVER use
   HashMap, HashSet, FxHashMap, AHashMap, IndexMap (without sorting)

   // ✅ ALWAYS use
   BTreeMap, BTreeSet, Vec (for ordered data)
   ```

2. **Address Format**: Always use 16-digit hex: `{:016x}`
3. **Deterministic Naming**: `purpose.address.counter` format
4. **Processing Order**: Always sort by address before processing
5. **No Floating Point**: Use fixed-point arithmetic instead

## Enhanced C AST Design

**CRITICAL**: Work WITH the existing AST structure, not against it:

1. **Extend, Don't Replace**:

- Use the existing `CAst`, `Statement`, `Expression` types
- Add enhanced features through configuration and helper functions
- Implement `PrintWithConfig` traits for custom rendering

2. **Architecture-Aware AST Generation**:
   ```rust
   pub struct EnhancedAstConfig {
       pub use_auto: bool,
       pub use_nullptr: bool,
       pub use_fixed_width_types: bool,
       pub numeric_format: NumericFormat,  // Hex by default
       pub architecture: Option<ArchitectureInfo>,
   }
   ```

3. **Proper AST Integration**:

- Convert Medium IR patterns to AST statements
- Maintain AST structure for analysis passes
- Support confidence-based feature selection

## Code Style Guidelines

- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Use `cargo fmt --all` before committing
- Document public APIs with `///` doc comments
- Use custom error types defined in `utils/error.rs`
- Keep modules focused and well-organized
- **CRITICAL**: Validate determinism in all code changes

### Visibility Guidelines

1. **Minimize Public API Surface**:
   ```rust
   // ❌ AVOID: Overly permissive
   pub fn internal_helper() { }

   // ✅ CORRECT: Appropriate visibility
   pub(crate) fn internal_helper() { }  // Crate-visible
   pub(super) fn module_helper() { }    // Module-visible
   ```

2. **Default Implementations**:

- Only implement `Default` when empty/zero state makes semantic sense
- Document why `Default` exists if not obvious
- Consider factory methods instead of `Default`
- **In decompiler context**: Avoid `Default` for analysis passes and transformations
    - Analysis structures should be explicitly constructed with context
    - Use `new()` or `with_context()` factory methods instead
    - Empty/default state often doesn't make sense for decompilation stages

3. **Constructor Visibility**:

- `new()` methods should generally be `pub(crate)` unless part of public API
- Document why public constructors exist
- Consider builder pattern for complex initialization

## Working with the Codebase

### Testing Requirements

**Every change MUST include**:

1. **Determinism tests**: Verify identical output across 1000+ runs
2. **Cross-platform tests**: Ensure same output on Linux/Windows/Mac
3. **Parallel execution tests**: Same output with 1-32 threads
4. **Memory pressure tests**: Behavior under constrained resources

### When implementing new instructions:
1. Add instruction parsing in `arch/x86_64/instruction_analyze/`
2. Implement IR generation with deterministic temporary naming
3. Add comprehensive tests including edge cases
4. Verify determinism with repetition tests
5. Update the instruction coverage

### When modifying IR:

1. Update `ir/statements.rs` maintaining `Ord` implementations
2. Implement analysis passes in `ir/analyze/`
3. Ensure all iterations use sorted collections
4. Update Enhanced C generation in `ir/analyze/ir_to_c/`
5. Add confidence tracking for uncertain transformations

## Simulation & Emulation Strategy

**IMPORTANT**: Custom simulation code will be replaced with Unicorn Engine:

1. **Why Unicorn Engine**:

- Battle-tested emulation framework
- Supports all target architectures (x86, ARM, MIPS, etc.)
- Better performance and accuracy
- Active maintenance and community

2. **Migration Plan**:
   ```rust
   // Current (to be deprecated)
   use crate::simulation::{CpuState, Memory, Executor};

   // Future (Unicorn-based)
   use unicorn_engine::{Unicorn, RegisterX86, RegisterARM};
   ```

3. **Hook-Based Analysis**:

- Memory access tracking for type recovery
- Instruction tracing for coverage
- Syscall interception for API detection
- Branch tracking for control flow validation

## Working with Enhanced C AST

### Critical Implementation Notes

1. **AST Extension Pattern**:
   ```rust
   // ❌ WRONG: Bypassing AST to generate strings directly
   fn generate_enhanced_c(patterns: &[Pattern]) -> String {
       let mut output = String::new();
       // Direct string generation
   }

   // ✅ CORRECT: Extend existing AST structure
   fn patterns_to_ast(patterns: &[Pattern]) -> CAst {
       let mut ast = CAst::new();
       // Convert patterns to AST nodes
       // Let existing infrastructure handle rendering
   }
   ```

2. **Architecture-Aware Code Generation**:

- Always check architecture before type sizing
- Use BTreeMap for deterministic iteration
- Format addresses based on pointer size (8 or 16 hex digits)
- Default to hexadecimal for numeric output

3. **Numeric Format Handling**:
   ```rust
   match config.numeric_format {
       NumericFormat::Hexadecimal => format!("0x{:x}", value),  // Default
       NumericFormat::Decimal => value.to_string(),
       NumericFormat::Binary => format!("0b{:b}", value),
       NumericFormat::Auto => {
           // Heuristic: hex for addresses, decimal for small values
           if value > 0x1000 { format!("0x{:x}", value) }
           else { value.to_string() }
       }
   }
   ```

4. **Pattern to AST Conversion Priority**:

- ForLoop, WhileLoop, DoWhileLoop → Control flow statements
- IfElse, SwitchCase → Conditional statements
- FunctionCall → Call expressions
- ArrayAccess, FieldAccess → Memory access expressions
- Always preserve confidence scores for uncertain transformations

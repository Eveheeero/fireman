## Project Overview

Fireman is a high-performance, deterministic decompiler framework written in Rust. It transforms binary executables into
readable, enhanced C code through a sophisticated multi-stage pipeline: Binary → Disassembly → Multi-Level IR →
Analysis → Enhanced C Output.

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
cargo clippy --workspace --tests

# Check compilation without building
cargo check --workspace --tests
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
Binary File → PE Parser → Disassembler → IR Generation → Analysis → C Generation
                                              ↓
                                        GUI Visualization
```

### Multi-Level IR Design

1. **Low IR**: Direct instruction translation, preserves all semantics
2. **Medium IR**: Pattern recognition, basic optimizations, confidence tracking
3. **High IR**: Near-source representation with recovered types and structures

### Key Components in fireball/

- **pe/**: PE file parsing, section handling, entry point detection
- **arch/x86_64/**: x86_64 instruction parsing and analysis
- **ir/**: Intermediate representation, control flow graphs, data flow analysis
- **core/**: Core data structures (Fire, Block, Instruction, Section)

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
- C code generation from IR
- GUI for visualizing assembly and IR

🚧 In Progress:
- Complete x86_64 instruction coverage
- Advanced IR optimizations
- Symbol resolution

📋 Planned:
- ARM architecture support
- x86_32 support
- ELF file format support
- Advanced decompilation patterns
- Code simulation capabilities

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

## Code Style Guidelines

- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Use `cargo fmt --all` before committing
- Document public APIs with `///` doc comments
- Use custom error types defined in `utils/error.rs`
- Keep modules focused and well-organized
- **CRITICAL**: Validate determinism in all code changes

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

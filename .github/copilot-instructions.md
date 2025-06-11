# Fireman Project Guidelines

## Project Overview

Fireman is a deterministic decompiler framework in Rust that transforms binaries into Enhanced C code through: Binary →
Disassembly → Multi-Level IR → Analysis → Enhanced C.

**Core Principles**:

1. **Absolute Determinism**: Identical input = identical output
2. **Zero-Copy Performance**: Memory-mapped files, arena allocation
3. **Accuracy First**: Mark uncertainty explicitly
4. **Human Readability**: Output optimized for human understanding

## Structure

```
fireman/                 # Workspace root
├── fireman/            # CLI executable
├── fireball/           # Core decompiler library
├── firebat/            # GUI application
├── iceball/            # Disassembly library
├── dryice/             # IR pattern matching
└── fireman_macro/      # Procedural macros
```

## Architecture

- **Multi-Level IR**: Low IR (direct translation) → Medium IR (pattern recognition) → High IR (near-source)
- **Key Point**: Optimizations happen at AST level, not during IR processing
- **Important Types**: `Fire` (main interface), `Block` (basic block), `IR::Statement` (operations),
  `CAbstractSyntaxTree` (C code)

## Critical: Determinism Requirements

**ABSOLUTE RULE**: Byte-for-byte identical output for identical input.

**Mandatory Practices**:

1. **Data Structures**: Use `BTreeMap`, `BTreeSet`, `Vec` (ordered) - NEVER use `HashMap`, `HashSet`, etc.
2. **Address Format**: Always use 16-digit hex: `{:016x}`
3. **Deterministic Naming**: `purpose.address.counter` format
4. **Processing Order**: Sort by address before processing
5. **No Floating Point**: Use fixed-point arithmetic

## Code Style

- Follow Rust naming conventions
- Use `cargo fmt --all` before committing
- Document public APIs with `///`
- Use appropriate visibility (`pub(crate)`, `pub(super)`)
- Only implement `Default` when empty state makes sense
- **CRITICAL**: Validate determinism in all changes

## Testing Requirements

Every change MUST include:

1. **Determinism tests**: Verify identical output across 1000+ runs
2. **Cross-platform tests**: Same output on Linux/Windows/Mac
3. **Parallel execution tests**: Same output with 1-32 threads

## Implementation Guidelines

- When implementing instructions: Add parsing, IR generation with deterministic naming, comprehensive tests
- When modifying IR: Update statements maintaining `Ord`, use sorted collections, add confidence tracking
- AST Extension: Extend existing structure, don't bypass it for string generation
- Architecture-aware: Check architecture before type sizing, use BTreeMap for iteration

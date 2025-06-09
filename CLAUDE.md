# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Fireman is a decompiler framework written in Rust, inspired by Snowman. It analyzes and decompiles binary executables (currently focusing on PE files) by transforming machine code → IR (Intermediate Representation) → C-like code.

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

The project uses Cargo workspace features with centralized dependency management. All common dependencies are defined in the root `Cargo.toml` and inherited by subcrates using `.workspace = true`.

## Development Commands

### Building
```bash
# Build entire workspace
cargo build

# Build with optimizations
cargo build --release

# Build specific component
cargo build -p fireball

# Note: When using the Agent tool, avoid running build commands
# as they consume significant resources. Use `cargo check` instead.
```

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

The decompilation process follows this flow:
```
Binary File → PE Parser → Disassembler → IR Generation → Analysis → C Generation
                                              ↓
                                        GUI Visualization
```

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
- ELF file format support
- Advanced decompilation patterns
- Code simulation capabilities

## Code Style Guidelines

- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Use `cargo fmt` before committing
- Document public APIs with `///` doc comments
- Use custom error types defined in `utils/error.rs`
- Keep modules focused and well-organized

## Working with the Codebase

When implementing new instructions:
1. Add instruction parsing in `arch/x86_64/instruction_analyze/`
2. Implement IR generation in the corresponding module
3. Add tests for the new instruction
4. Update the instruction coverage if needed

When modifying IR:
1. Update `ir/statements.rs` for new statement types
2. Implement analysis passes in `ir/analyze/`
3. Update C generation in `ir/analyze/ir_to_c/`

When working on GUI:
1. Tauri backend code is in `firebat/src-tauri/`
2. React frontend is in `firebat/src/`
3. Use TypeScript with strict mode
4. Test with `npm run tauri dev`

## Project Documentation

- **PLANS.md**: Development roadmap and technical strategy
- **TODOS.md**: Detailed task list with priorities
- **STRUCTURES.md**: Architecture diagrams and component relationships
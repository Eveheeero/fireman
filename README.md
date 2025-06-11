# Fireman

![Logo](firebat/src-tauri/icons/icon.png)

A decompiler framework written in Rust, inspired by Snowman. Fireman analyzes and decompiles binary executables by
transforming machine code → IR (Intermediate Representation) → C-like code.

## Installation

To get started with Fireman, clone the repository with its submodules:

```bash
git clone https://github.com/your-username/fireman.git
cd fireman
git submodule init
git submodule update
```

## Building

Build the entire workspace:

```bash
cargo build -r
```

## Features & Roadmap

See [TODOS.md](TODOS.md) for the detailed features list and development roadmap.

## Development

### Pre-commit Hooks

This project uses pre-commit hooks to ensure code quality. The hooks run automatically before each commit to check for
formatting, linting, and compilation issues.

To set up pre-commit hooks:

1. Install the pre-commit tool:
   ```bash
   # Using pip
   pip install pre-commit

   # Or using homebrew on macOS
   brew install pre-commit
   ```

2. Install the git hooks:
   ```bash
   pre-commit install
   ```

3. (Optional) Run the hooks manually:
   ```bash
   pre-commit run --all-files
   ```

The pre-commit hooks will:

- Format your code with `cargo fmt`
- Check your code with `cargo clippy`
- Verify compilation with `cargo check`
- Fix common issues like trailing whitespace and end-of-file newlines
- Check for common determinism issues (e.g., use of HashMap in critical paths)

> **Note**: Determinism is a critical requirement for this project. The pre-commit hook checks for common determinism
> issues, but for a more comprehensive check, run the determinism tests with
`cargo test --package fireball --test determinism -- --nocapture`.

## Code Style

This project follows the [official Rust Style Guide](https://doc.rust-lang.org/stable/style-guide/) with
decompiler-specific adaptations. For comprehensive guidelines, see [RUST_STYLE_GUIDE.md](RUST_STYLE_GUIDE.md).

### Quick Reference

#### Formatting & Organization
- **4 spaces** for indentation (no tabs), **100 characters** max line width
- Use `cargo fmt --all` for automatic formatting (enforced by pre-commit hooks)
- **Block indent** over visual indent for better diffs
- **Deterministic collections**: Use `BTreeMap`/`BTreeSet` instead of `HashMap`/`HashSet` for reproducible analysis results

#### Naming Conventions
- `snake_case` for variables, functions, and modules
- `PascalCase` for types, structs, enums, and traits
- `SCREAMING_SNAKE_CASE` for constants and statics
- Architecture-specific prefixes: `X86Instruction`, `ArmRegister`, `RiscVDecoder`
- IR level indicators: `HighIR`, `MediumIR`, `LowIR`

#### Decompiler-Specific Patterns
```rust
// ✅ Memory layout documentation
/// Memory layout: [REX][Opcode][ModR/M][SIB][Disp][Imm]
#[repr(C)]
pub struct X86Instruction { /* ... */ }

// ✅ Rich error context
#[error("Invalid instruction at {address:#x}: {reason}")]
InvalidInstruction { address: u64, reason: String },

// ✅ Architecture-agnostic traits
pub trait InstructionTrait {
    fn address(&self) -> u64;
    fn bytes(&self) -> &[u8];
    fn mnemonic(&self) -> &str;
}
```

### Documentation Standards

All public items must have comprehensive documentation following this template:

```rust
/// Brief one-line description.
///
/// Detailed explanation of functionality, algorithms used,
/// and important implementation details.
///
/// # Arguments
/// # Returns
/// # Errors
/// # Examples
/// # Note (performance, safety, determinism considerations)
```

### Error Handling

- Use `Result<T, E>` for fallible operations with domain-specific error types
- Rich error context with addresses, instruction details, and analysis state
- Use `?` operator for error propagation
- Avoid `unwrap()` and `expect()` in production code

### Testing Standards

- **Unit tests** for all public functions with descriptive names
- **Property-based testing** for instruction decoding and analysis invariants
- **Snapshot testing** for IR transformations and code generation
- **Determinism tests** to ensure reproducible analysis results

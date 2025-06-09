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

## Code style

### Comment Template (optional, to avoid typing Note, NOTE, NOTES, notes, ....)

- \#\#\# Arguments
- \#\#\# Returns
- \#\#\# Note
- \#\#\# Todo

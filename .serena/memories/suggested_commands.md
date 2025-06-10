# Suggested Commands for Fireman Development

## Building

```bash
# Build entire workspace
cargo build

# Build with optimizations
cargo build --release

# Build specific component
cargo build -p fireball
cargo build -p firebat

# Note: When using the Agent tool, avoid running build commands
# as they consume significant resources. Use `cargo check` instead.
```

## Testing

```bash
# Run all tests in workspace
cargo test --workspace

# Run tests for specific package
cargo test -p fireball

# Run specific test
cargo test test_name

# Run tests with output
cargo test --workspace -- --nocapture
```

## Running

```bash
# Run CLI
cargo run --bin fireman -- [args]

# Run GUI (from firebat directory)
cd firebat
npm install  # first time only
npm run tauri dev

# Build GUI for production
npm run tauri build
```

## Development Tools

```bash
# Format code
cargo fmt --all

# Check code without building
cargo check --workspace --tests

# Lint code
cargo clippy --workspace --tests

# Generate documentation
cargo doc --open

# Update dependencies
cargo update
```

## Git Commands

```bash
git status
git add .
git commit -m "message"
git push
git pull
```

## System Utilities (Linux)

```bash
ls -la         # List files
find . -name "*.rs"  # Find Rust files
grep -r "pattern" .  # Search in files
rg "pattern"   # Faster search (ripgrep)
```

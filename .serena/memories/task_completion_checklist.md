# Task Completion Checklist

When completing a development task in Fireman, ensure:

## Before Committing

1. **Code Quality**
    - Run `cargo fmt --all` to format code
    - Run `cargo clippy --workspace --tests` to check for common issues
    - Run `cargo test` to ensure all tests pass
    - Run `cargo check --workspace --tests` for quick compilation check

2. **Testing**
    - Add unit tests for new functionality
    - Update existing tests if behavior changes
    - Ensure tests cover edge cases
    - Run specific package tests: `cargo test -p <package>`

3. **Documentation**
    - Update doc comments for public APIs
    - Update README.md if adding major features
    - Add inline comments for complex logic

4. **GUI Changes (if applicable)**
    - Test in development mode: `npm run tauri dev`
    - Check browser console for errors
    - Verify UI responsiveness
    - Test on different window sizes

## Verification Commands

```bash
# Full check sequence
cargo fmt --all
cargo clippy --workspace --tests
cargo test
cargo check --workspace --tests
cargo doc --no-deps

# For GUI changes
cd firebat
npm run tauri dev
```

## Before Push

- Review changes with `git diff`
- Ensure commit message is descriptive
- Check that no debug code remains
- Verify no sensitive information is included

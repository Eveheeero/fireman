# Code Style and Conventions

## Rust Code Style

- Follow standard Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Use `cargo fmt` for consistent formatting
- Prefer descriptive names over abbreviations
- Use type annotations when clarity is needed

## Documentation

- Document public APIs with `///` doc comments
- Include examples in doc comments where helpful
- Keep internal documentation concise but clear

## Error Handling

- Use `Result<T, E>` for fallible operations
- Create custom error types in `utils/error.rs`
- Provide meaningful error messages

## Module Organization

- Each major component has its own module
- Use `mod.rs` for module exports
- Keep related functionality together
- Separate concerns (parsing, analysis, generation)

## Testing

- Unit tests in same file using `#[cfg(test)]`
- Integration tests in `tests/` directory
- Test edge cases and error conditions

## Performance Considerations

- Profile before optimizing
- Document performance-critical sections
- Consider memory usage for large binaries

## GUI Development (Tauri/React)

- TypeScript with strict mode
- React functional components
- Consistent component naming (PascalCase)
- Keep components focused and reusable

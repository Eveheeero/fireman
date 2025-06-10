# Workspace Configuration

The Fireman project uses Cargo workspace features for better dependency management.

## Workspace Structure

- Root `Cargo.toml` contains all shared dependencies
- Subcrates inherit common metadata (version, edition, authors, etc.)
- Dependencies use `.workspace = true` syntax

## Benefits

1. **Centralized version management**: Update dependency versions in one place
2. **Consistency**: All crates use the same dependency versions
3. **Reduced duplication**: No need to specify versions in each crate
4. **Better maintainability**: Easier to track and update dependencies

## Adding New Dependencies

1. Add to root `Cargo.toml` under `[workspace.dependencies]`
2. Reference in subcrate with `dependency.workspace = true`
3. Can still override features in subcrates if needed

## Profile Configuration

- `dev`: Optimized for fast compilation
- `release`: Maximum optimization with LTO
- `release-with-debug`: Release build with debug symbols
- `test`: Optimized for test execution speed

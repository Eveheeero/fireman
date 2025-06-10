# Visibility Audit Checklist

## Overview

This document tracks the visibility audit progress for the Fireman codebase to ensure minimal public API surface.

## Audit Status

### Core Module (`fireball/src/core/`)

- [x] **instruction.rs**
    - [x] `Instruction::new()` → `pub(crate)` ✓
    - [ ] Review `inner()` method visibility

- [x] **sections.rs**
    - [x] Remove `#[derive(Default)]` → Keep with documentation ✓
    - [x] `add_section()` → `pub(crate)` ✓
    - [ ] Review other public methods

- [ ] **address.rs**
    - [ ] Review all public methods
    - [ ] Consider `pub(crate)` for constructors
    - [ ] Document ordering implications

- [ ] **block.rs**
    - [ ] Audit public methods
    - [ ] Check if `Block::new()` should be `pub(crate)`

- [ ] **fire.rs & fire_raw.rs**
    - [ ] Review main API surface
    - [ ] Ensure only decompiler entry points are public

### Binary Modules (`fireball/src/binary/`)

- [ ] **pe/fire/*.rs**
    - [x] `_analyze_from_virtual_address` → Should be `pub(super)` ✗
    - [ ] `_analyze_from_file_offset` → Should be `pub(super)`
    - [ ] `_analyze_from_entry` → Should be `pub(super)`
    - [ ] `_analyze_all` → Should be `pub(super)`
    - [ ] `_analyze_block` → Should be `pub(super)`

- [ ] **elf/fire/*.rs**
    - [ ] Apply same visibility restrictions as PE
    - [ ] Ensure consistency across formats

- [ ] **macho/fire/*.rs**
    - [ ] Apply same visibility restrictions as PE
    - [ ] Ensure consistency across formats

### IR Modules (`fireball/src/ir/`)

- [x] **utils.rs**
    - [x] Remove `Default` for `IrStatementDescriptorMap` ✓
    - [ ] Review other utility types

- [ ] **low_ir.rs**
    - [ ] Check visibility of constructors
    - [ ] Review helper methods

- [ ] **medium_ir/mod.rs**
    - [ ] Audit pattern types
    - [ ] Check builder methods

- [ ] **high_ir/mod.rs**
    - [ ] Review AST generation methods
    - [ ] Ensure internal helpers are not public

### Architecture Modules (`fireball/src/arch/`)

- [ ] **architecture.rs**
    - [ ] Review detection methods
    - [ ] Check helper functions

- [ ] **x86_64/mod.rs**
    - [ ] Instruction analysis functions
    - [ ] Register definitions

- [ ] **arm32/mod.rs & arm64/mod.rs**
    - [ ] Similar audit as x86_64
    - [ ] Ensure consistency

## Visibility Rules

### 1. Public (`pub`) - Use Sparingly

Only for:

- Main API entry points (Fire::new, decompile methods)
- Types that are part of the public API
- Traits that users must implement

### 2. Crate-Visible (`pub(crate)`)

For:

- Internal constructors used across modules
- Utility functions needed by multiple modules
- Types shared between binary formats

### 3. Super-Visible (`pub(super)`)

For:

- Module-internal helpers
- Implementation details of a module
- Methods only used within parent module

### 4. Private (no modifier)

Default for:

- All implementation details
- Helper functions
- Internal state management

## Common Patterns to Fix

### Pattern 1: Overly Public Constructors

```rust
// ❌ WRONG
pub fn new() -> Self { }

// ✅ CORRECT (unless part of public API)
pub(crate) fn new() -> Self { }
```

### Pattern 2: Unnecessary Default Implementations

```rust
// ❌ WRONG - Empty map doesn't make semantic sense
impl Default for DescriptorMap { }

// ✅ CORRECT - Document why default exists
/// Creates an empty sections collection for fallback scenarios
impl Default for Sections { }
```

### Pattern 3: Internal Analysis Methods

```rust
// ❌ WRONG
pub fn _internal_analyze() { }

// ✅ CORRECT
pub(super) fn _internal_analyze() { }
```

## Action Items

1. **Immediate** (Day 1-2):
    - [ ] Fix all `_analyze_*` methods in binary modules
    - [ ] Remove invalid Default implementations
    - [ ] Update constructor visibility

2. **Short-term** (Week 1):
    - [ ] Complete core module audit
    - [ ] Document all public APIs
    - [ ] Add #[doc(hidden)] where appropriate

3. **Long-term** (Month 1):
    - [ ] Create public API stability guarantees
    - [ ] Version public API changes properly
    - [ ] Add visibility lint rules

## Testing Considerations

- [ ] Ensure tests can still access needed internals
- [ ] Use `#[cfg(test)] pub` where necessary
- [ ] Consider test-only helper modules

## Documentation Requirements

For every `pub` item:

1. Document why it's public
2. Provide usage examples
3. Note stability guarantees
4. Link to related items

## Automated Checks

Consider adding:

```toml
# .cargo/config.toml
[lints.rust]
unreachable_pub = "warn"
missing_docs = "warn"
```

## Review Process

1. Every PR must justify new `pub` items
2. Quarterly visibility audit
3. Track public API changes in CHANGELOG
4. Use `cargo doc --document-private-items` to verify

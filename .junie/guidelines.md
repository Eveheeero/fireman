# Fireman AI Agent Guide

## Default

- Avoid `cargo test`, use `cargo check`

## Structure Path

- core structures - `fireball/src/core/mod.rs`
- `IrBlock`, `Ir` - `fireball/src/ir/mod.rs`
- `IrStatement` - `fireball/src/ir/statements.rs`
- `IrData` - `fireball/src/ir/data.rs`
- asm to `IrStatement` conversation - `fireball/src/arch/...`
- `Ast...` - `fireball/src/abstract_syntax_tree/objects/...`
- AST optimization logics - `fireball/src/abstract_syntax_tree/optimize/...`

If you want to find another struct or trait, search by regex

# Unified Architecture Quick Reference Card

## 🎯 Goal

Merge x86+x86_64 and ARM32+ARM64 into unified modules with 60-80% code sharing.

## 📁 New Structure

```
x86_unified/
  ├── mod.rs         → ProcessorMode enum
  ├── common.rs      → 90% of instructions
  ├── register.rs    → All registers
  └── *_specific.rs  → Mode-specific code

arm_unified/
  ├── mod.rs         → InstructionSet enum
  ├── common.rs      → 60% of instructions
  ├── register.rs    → All registers
  └── *_specific.rs  → Architecture-specific code
```

## 🔧 Key Types

```rust
enum ProcessorMode {
    Protected32,  // x86
    Long64,       // x86_64
}

enum InstructionSet {
    ARM,      // ARM32
    Thumb,    // ARM32 16-bit
    AArch64,  // ARM64
}
```

## 📝 Implementation Checklist

### For X86 Unification:

- [ ] Move shared instructions to common.rs
- [ ] Handle zero-extension in 64-bit mode
- [ ] REX prefix detection
- [ ] Register mapping (EAX→RAX)
- [ ] Test both modes

### For ARM Unification:

- [ ] Move shared conditions to common.rs
- [ ] Handle instruction encoding differences
- [ ] Register mapping (R0→X0)
- [ ] Thumb mode support
- [ ] Test all instruction sets

## 🚀 Quick Start

```bash
# Enable unified architecture feature
cargo build --features unified-arch

# Run unified tests
cargo test -p fireball unified

# Compare old vs new
cargo bench compare_architectures
```

## ⚠️ Common Issues

1. **Register Size**: Check mode before accessing 64-bit registers
2. **Zero Extension**: x64 auto-extends 32→64, x86 doesn't
3. **Encoding**: ARM32/64 have completely different encodings
4. **Defaults**: x64 default operand size is still 32-bit!

## 📊 Progress Tracking

- Week 1-2: Foundation [🟡 In Progress]
- Week 3-4: X86 [⏳ Planned]
- Week 5-6: ARM [⏳ Planned]
- Week 7: Integration [⏳ Planned]
- Week 8: Testing [⏳ Planned]
- Week 9: Cleanup [⏳ Planned]

## 🔗 Links

- [Full Plan](unified-architecture-implementation-plan.md)
- [Design Docs](../design/unified-architecture-implementation.md)
- [TODOS.md](../../../TODOS.md#sprint-6-unified-architecture-implementation)

## 💡 Tips

- Start with most common instructions
- Test after each instruction group
- Keep compatibility layer until done
- Document mode-specific behavior

---
*Last Updated: 2025-01-11 | Sprint 6 | Unified Architecture*

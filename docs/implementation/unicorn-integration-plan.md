# Unicorn Engine Integration Plan

## Overview

This document outlines the plan to replace Fireman's custom simulation module with Unicorn Engine (v2.1.3), a battle-tested CPU emulation framework.

## Motivation

- **Accuracy**: Unicorn provides cycle-accurate CPU emulation
- **Performance**: Optimized JIT-based execution
- **Architecture Support**: x86, x86_64, ARM, ARM64, MIPS, RISC-V, and more
- **Maintenance**: Active development and extensive testing
- **Features**: Built-in hooks, memory protection, snapshot/restore

## Current Status

### Completed
- ✅ Added unicorn-engine v2.1.3 to workspace dependencies
- ✅ Created initial emulation module structure (`emulation/mod.rs`)
- ✅ Documented FPU instruction implementation issues

### In Progress
- 🚧 Creating support modules (context, hooks, memory, state)
- 🚧 Fixing FPU instruction stack semantics

### Pending
- ⏳ Complete Unicorn integration
- ⏳ Migrate pattern recognition to use Unicorn
- ⏳ Remove legacy simulation module

## Architecture

```
fireball/src/
├── simulation/          # Legacy (to be deprecated)
│   ├── cpu_state.rs
│   ├── memory.rs
│   └── executor.rs
│
└── emulation/          # New Unicorn-based
    ├── mod.rs          # Main emulator interface ✅
    ├── context.rs      # CPU context save/restore 🚧
    ├── hooks.rs        # Hook management system 🚧
    ├── memory.rs       # Memory region tracking 🚧
    └── state.rs        # Emulator state management 🚧
```

## Implementation Phases

### Phase 1: Foundation (Current)
1. Create emulation module structure
2. Implement register mapping (IR ↔ Unicorn)
3. Add memory management wrapper
4. Create execution control interface

### Phase 2: Integration
1. Add hook-based analysis capabilities
2. Implement snapshot/restore functionality
3. Create migration helpers from old simulation
4. Add comprehensive tests

### Phase 3: Migration
1. Add feature flag for gradual migration
2. Update dependent code to use new interface
3. Performance benchmarking
4. Deprecate and remove old simulation module

## FPU Implementation Issues

The current FPU instruction implementation in `f.rs` has several critical issues:

1. **No Stack Semantics**: FLD/FSTP don't push/pop the FPU stack
2. **No TOP Tracking**: ST0 is relative to the TOP pointer, not fixed
3. **Missing Status Updates**: Condition codes (C0-C3) not set
4. **No Exception Handling**: Stack overflow/underflow not detected
5. **Limited Instructions**: Missing FINIT, FLDCW, transcendentals

These will be addressed after Unicorn integration provides proper FPU emulation.

## Key Benefits

### Dynamic Analysis
- Hook memory accesses for type recovery
- Track register values for data flow
- Monitor API calls for behavior analysis
- Validate control flow transfers

### Accuracy
- Proper flag calculations
- Accurate FPU stack emulation
- Correct exception handling
- Architecture-specific quirks

### Performance
- JIT compilation for hot paths
- Efficient memory management
- Minimal overhead for hooks
- Parallel execution support

## Testing Strategy

1. **Unit Tests**: Each emulation component
2. **Integration Tests**: Full decompilation with emulation
3. **Compatibility Tests**: Ensure same results as old simulation
4. **Performance Tests**: Benchmark vs custom implementation
5. **Architecture Tests**: Verify x86, ARM support

## Migration Guide

For code currently using the simulation module:

```rust
// Old
use crate::simulation::{SimulationContext, CpuState};
let mut ctx = SimulationContext::new();
ctx.execute_statement(&stmt)?;

// New
use crate::emulation::{Emulator, EmulationContext};
let mut emu = Emulator::new(arch_info)?;
emu.execute_ir_statement(&stmt)?;
```

## Timeline

- Week 1: Complete foundation modules
- Week 2: Integration and testing
- Week 3: Migration and benchmarking
- Week 4: Cleanup and documentation

## Success Criteria

1. All existing tests pass with Unicorn backend
2. Performance within 10% of custom simulation
3. Support for x86_64 and ARM architectures
4. Proper FPU emulation with stack semantics
5. Clean migration path with feature flags

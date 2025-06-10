# x86_64 Instruction Implementation Status

## Overview

This document tracks the implementation status of x86_64 instructions in Fireman's decompiler.

## Recently Verified Instructions

### ✅ SAHF (Store AH into Flags)

- **Status**: Implemented
- **Location**: `fireball/src/arch/x86_64/instruction_analyze/s.rs:134-164`
- **Description**: Loads SF, ZF, AF, PF, and CF from AH register into EFLAGS
- **Implementation Details**:
    - Extracts individual bits from AH register
    - Sets corresponding flags based on bit positions:
        - Bit 0 → CF (Carry Flag)
        - Bit 2 → PF (Parity Flag)
        - Bit 4 → AF (Auxiliary Flag)
        - Bit 6 → ZF (Zero Flag)
        - Bit 7 → SF (Sign Flag)

### ✅ XCHG (Exchange Register/Memory with Register)

- **Status**: Implemented
- **Location**: `fireball/src/arch/x86_64/instruction_analyze/x.rs:5-10`
- **Description**: Exchanges values between two operands
- **Implementation Details**:
    - Uses temporary register (tmp64) for swap
    - Generates 3 IR statements:
        1. `tmp = operand1`
        2. `operand1 = operand2`
        3. `operand2 = tmp`

### ✅ CMPXCHG (Compare and Exchange)

- **Status**: Implemented (without LOCK prefix support)
- **Location**: `fireball/src/arch/x86_64/instruction_analyze/c.rs:147-161`
- **Description**: Compares AL/AX/EAX/RAX with first operand and exchanges if equal
- **Implementation Details**:
    - Compares RAX with memory/register operand
    - If equal: stores second operand to first operand
    - If not equal: loads first operand to RAX
    - Updates all arithmetic flags based on comparison

## Missing Features

### ❌ LOCK Prefix Support

- **Status**: Not Implemented
- **Impact**: Atomic operations are not properly modeled
- **Required Changes**:
    1. Add prefix detection in instruction parsing
    2. Create atomic IR operations
    3. Model memory barriers/fences
    4. Ensure operations are marked as non-interruptible

### Implementation Recommendation for LOCK Prefix

```rust
// Suggested IR extensions for atomic operations
pub enum IrStatement {
    // ... existing variants ...

    /// Atomic memory operation with lock semantics
    AtomicOperation {
        op: AtomicOp,
        operands: Vec<IrData>,
        ordering: MemoryOrdering,
    },

    /// Memory fence/barrier
    MemoryFence {
        ordering: MemoryOrdering,
    },
}

pub enum AtomicOp {
    CompareExchange,
    Exchange,
    Add,
    Sub,
    And,
    Or,
    Xor,
}

pub enum MemoryOrdering {
    Relaxed,
    Acquire,
    Release,
    AcquireRelease,
    SequentiallyConsistent,
}
```

## Other x86_64 Instructions Needing Implementation

### High Priority

- [ ] FPU Instructions (FLD, FST, FADD, FSUB, FMUL, FDIV, etc.)
- [ ] Advanced SSE4/AVX2/AVX-512 instructions
- [ ] System instructions (CPUID, RDTSC, WRMSR, RDMSR)

### Medium Priority

- [ ] Rare/undocumented instructions
- [ ] Instruction prefix handling (REX, VEX, EVEX)
- [ ] Segment override prefixes

## Testing Considerations

Due to private fields in `core::Instruction`, unit testing individual instruction implementations requires:

1. Integration tests using actual binaries
2. Testing through the full decompilation pipeline
3. Creating test utilities that can construct instructions through the proper channels

## Summary

The three instructions mentioned in TODOS.md are all implemented:

- ✅ SAHF - Fully implemented
- ✅ XCHG - Fully implemented
- ✅ CMPXCHG - Implemented but lacks LOCK prefix support

The main missing feature is proper handling of the LOCK prefix for atomic operations, which affects not just CMPXCHG but
also other instructions like XADD, ADD, SUB, etc. when used with the LOCK prefix.

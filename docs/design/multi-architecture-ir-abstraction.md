# Multi-Architecture IR Abstraction Design

## Overview

This document describes the design and implementation of the multi-architecture IR abstraction layer in the Fireman
decompiler. The goal is to provide a unified interface for handling different CPU architectures while maintaining the
decompiler's strict determinism requirements.

## Architecture Detection

### Binary Format Recognition

The architecture detection system automatically identifies the target architecture from binary files by:

1. **Magic Byte Detection**: First identifies the binary format (PE, ELF, Mach-O)
2. **Header Parsing**: Extracts architecture information from format-specific headers
3. **Endianness Detection**: Determines byte ordering from binary headers

### Supported Architectures

- **x86**: 32-bit Intel/AMD processors
- **x86_64**: 64-bit Intel/AMD processors (AMD64/Intel 64)
- **ARM32**: 32-bit ARM processors
- **ARM64**: 64-bit ARM processors (AArch64)

## Core Components

### 1. ArchitectureInfo Structure

```rust
pub struct ArchitectureInfo {
    pub arch_type: ArchType,        // Architecture enum
    pub pointer_size: u8,            // 32 or 64 bits
    pub endianness: Endianness,      // Little or Big
    pub register_count: usize,       // Number of general-purpose registers
    pub instruction_alignment: u8,   // Alignment requirement (1 for x86, 4 for ARM)
}
```

### 2. ArchitectureDetector

The `ArchitectureDetector` provides static methods to detect architecture from:

- Raw binary data
- PE files (Windows)
- ELF files (Linux/Unix)
- Mach-O files (macOS/iOS)

### 3. ArchitectureContext

The `ArchitectureContext` encapsulates architecture-specific operations:

- Instruction to IR conversion
- Register mapping
- Target information for Low IR

## IR Abstraction Layers

### Low IR Integration

The architecture abstraction integrates with the existing Low IR system:

```rust
pub struct TargetInfo {
    pub arch: String,      // Architecture name
    pub bits: u32,         // Pointer width in bits
    pub endian: Endianness // Byte ordering
}
```

### Instruction Analysis Pipeline

1. **Binary Parsing**: Extract raw bytes based on architecture alignment
2. **Instruction Decoding**: Architecture-specific decoders
3. **IR Generation**: Convert to architecture-neutral IR statements
4. **Optimization**: Apply architecture-aware optimizations

## Architecture-Specific Modules

Each architecture has its own module structure:

```
arch/
├── x86/
│   ├── mod.rs
│   ├── register.rs
│   ├── instruction_analyze.rs
│   └── lifter.rs
├── x86_64/
│   ├── mod.rs
│   ├── register.rs
│   ├── instruction_analyze/
│   └── lifter.rs
├── arm32/
│   └── ... (similar structure)
└── arm64/
    └── ... (similar structure)
```

### Register Definitions

Each architecture defines its register set:

- **x86**: 8 general-purpose registers (EAX, EBX, etc.)
- **x86_64**: 16 general-purpose registers (RAX-R15)
- **ARM32**: 16 registers (R0-R15)
- **ARM64**: 31 general-purpose registers (X0-X30)

### Instruction Decoders

Architecture-specific instruction decoders handle:

- Variable-length instructions (x86/x86_64)
- Fixed-length instructions (ARM32/ARM64)
- Endianness conversion
- Addressing mode parsing

## Implementation Status

### Completed

- ✅ Architecture detection from binary formats
- ✅ ArchitectureInfo and ArchitectureContext structures
- ✅ Integration with existing x86_64 analyzer
- ✅ Comprehensive test coverage for detection

### In Progress

- 🚧 x86 (32-bit) instruction decoder
- 🚧 ARM32 instruction decoder
- 🚧 ARM64 instruction decoder

### Future Work

- RISC-V support
- PowerPC support
- MIPS support
- Architecture-specific optimization passes

## Determinism Guarantees

The multi-architecture system maintains determinism by:

1. **Ordered Processing**: Always process architectures in a defined order
2. **Stable Naming**: Use deterministic naming for architecture-specific elements
3. **Fixed Representations**: Use fixed-size integers for all architecture data
4. **No Floating Point**: Avoid floating-point in architecture detection

## Usage Example

```rust
// Automatic architecture detection
let arch_info = ArchitectureDetector::detect_from_bytes( & binary_data);

// Create architecture context
let arch_context = ArchitectureContext::new(arch_info, sections);

// Convert instruction to IR
if let Some(ir_statements) = arch_context.instruction_to_ir( & instruction) {
// Process IR statements
}

// Get target info for Low IR
let target_info = arch_context.get_target_info();
```

## Testing Strategy

1. **Unit Tests**: Test each architecture detector individually
2. **Integration Tests**: Test full pipeline with real binaries
3. **Cross-Architecture Tests**: Ensure consistent IR generation
4. **Determinism Tests**: Verify identical output across runs

## Performance Considerations

- Architecture detection is performed once per binary
- Instruction decoding uses architecture-specific optimized paths
- Register mapping uses static lookup tables
- Alignment checking is compile-time when possible

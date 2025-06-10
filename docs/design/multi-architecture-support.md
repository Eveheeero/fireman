# Multi-Architecture Support Design

## Overview

Fireman is designed to support multiple architectures and binary formats, enabling decompilation of executables from
various platforms.

## Supported Architectures

### Current

- **x86_64** - Full support with most instructions implemented

### In Development

- **ARM64 (AArch64)** - Infrastructure created, instruction implementation pending
- **x86 (32-bit)** - Infrastructure created, shares many instructions with x86_64

### Planned

- **ARM32** - Including Thumb mode
- **MIPS** - Both 32-bit and 64-bit variants
- **PowerPC** - Focus on game console binaries
- **RISC-V** - Growing embedded market

## Supported Binary Formats

### Current

- **PE (Portable Executable)** - Windows executables (.exe, .dll)

### In Development

- **ELF (Executable and Linkable Format)** - Linux/Unix executables
- **Mach-O** - macOS/iOS executables

### Planned

- **COFF** - Object files
- **Universal Binary** - macOS fat binaries
- **Android DEX** - Dalvik executables

## Architecture Abstraction Design

```rust
// Unified architecture interface
pub trait Architecture {
    /// Architecture identifier
    fn name(&self) -> &'static str;

    /// Pointer size in bits
    fn pointer_size(&self) -> u8;

    /// Default calling convention
    fn default_calling_convention(&self) -> CallConv;

    /// Convert native instruction to IR
    fn instruction_to_ir(&self, inst: &Instruction) -> Result<Vec<IrStatement>, Error>;

    /// Get register information
    fn register_info(&self) -> &RegisterInfo;

    /// Get instruction set information
    fn instruction_set(&self) -> &InstructionSet;
}

// Binary format interface
pub trait BinaryFormat {
    /// Format identifier
    fn name(&self) -> &'static str;

    /// Parse from bytes
    fn parse(data: Vec<u8>) -> Result<Self, Error>;

    /// Get entry point
    fn entry_point(&self) -> Option<u64>;

    /// Get sections
    fn sections(&self) -> Result<Sections, Error>;

    /// Get imports
    fn imports(&self) -> Vec<Import>;

    /// Get exports
    fn exports(&self) -> Vec<Export>;

    /// Detect architecture
    fn architecture(&self) -> Architecture;
}
```

## IR Design for Multi-Architecture

The IR must be generic enough to represent concepts from all architectures:

### Register Abstraction

- Generic register names with architecture prefixes
- Size-aware register operations
- Architecture-specific register mappings

### Memory Model

- Configurable address sizes (32-bit, 64-bit)
- Endianness support
- Segmented memory support (x86 real mode)

### Instruction Abstraction

- Common operations (arithmetic, logic, control flow)
- Architecture-specific intrinsics
- SIMD/Vector operation support

## Implementation Strategy

### Phase 1: Core Infrastructure

1. ✅ Create architecture module structure
2. ✅ Define architecture traits
3. ✅ Create ARM64 module skeleton
4. ✅ Create x86 module skeleton
5. ✅ Create ELF format skeleton
6. ✅ Create Mach-O format skeleton

### Phase 2: Binary Format Parsers

1. [ ] Implement ELF parser
    - [ ] ELF header parsing
    - [ ] Program header parsing
    - [ ] Section header parsing
    - [ ] Symbol table parsing
    - [ ] Relocation handling

2. [ ] Implement Mach-O parser
    - [ ] Mach header parsing
    - [ ] Load command parsing
    - [ ] Segment/section parsing
    - [ ] Symbol table parsing
    - [ ] Dynamic loader info

### Phase 3: Architecture Implementation

1. [ ] ARM64 instruction decoder
    - [ ] Data processing instructions
    - [ ] Load/store instructions
    - [ ] Branch instructions
    - [ ] SIMD instructions

2. [ ] x86 instruction decoder
    - [ ] Reuse x86_64 implementations
    - [ ] Handle 32-bit specific instructions
    - [ ] Segment register support

### Phase 4: Integration

1. [ ] Unified binary loader
2. [ ] Architecture auto-detection
3. [ ] Cross-architecture IR validation
4. [ ] Multi-architecture test suite

## Testing Strategy

### Binary Format Tests

- Test files for each format (PE, ELF, Mach-O)
- Malformed file handling
- Various architecture variants

### Architecture Tests

- Instruction coverage tests
- Edge case handling
- Cross-architecture IR consistency

### Integration Tests

- Full decompilation pipeline
- Multi-architecture binaries
- Cross-compilation validation

## Challenges and Solutions

### Challenge 1: Instruction Set Complexity

**Problem**: Each architecture has hundreds of instructions with complex semantics.
**Solution**:

- Incremental implementation focusing on common instructions
- Shared implementations for similar operations
- Community contributions for rare instructions

### Challenge 2: Binary Format Variations

**Problem**: Binary formats have many versions and extensions.
**Solution**:

- Start with common/modern variants
- Graceful degradation for unsupported features
- Clear error messages for unsupported formats

### Challenge 3: IR Abstraction

**Problem**: Creating IR that works well for all architectures.
**Solution**:

- Architecture-specific IR extensions
- Multiple IR levels (low/medium/high)
- Clear documentation of IR semantics

### Challenge 4: Testing Coverage

**Problem**: Need test binaries for all architecture/format combinations.
**Solution**:

- Automated test binary generation
- Community-contributed test suite
- Focus on real-world binaries

## Future Considerations

### WebAssembly Support

- Growing target for decompilation
- Simpler instruction set
- Good test case for IR design

### GPU Shader Support

- SPIR-V decompilation
- HLSL/GLSL bytecode
- Compute shader analysis

### Embedded Formats

- Arduino/AVR binaries
- PIC microcontroller code
- Custom firmware formats

## Conclusion

Multi-architecture support is essential for Fireman to be a comprehensive decompilation solution. The modular design
allows incremental implementation while maintaining code quality and determinism requirements.

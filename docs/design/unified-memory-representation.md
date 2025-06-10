# Unified Memory Representation Design

## Current Memory Struct (CORRECT DESIGN)

```rust
pub struct Memory {
    pub base: Option<Register>,      // Base register
    pub index: Option<Register>,     // Index register for scaled addressing
    pub scale: u8,                   // Scale factor (1, 2, 4, 8)
    pub displacement: i64,           // Displacement/offset
    pub size: Option<u8>,           // Memory access size in bytes
}
```

## Why This Design is Correct

### 1. Covers All Architecture Addressing Modes

#### x86/x86_64 Addressing

- **Direct**: `[0x1234]` → base=None, displacement=0x1234
- **Register Indirect**: `[RAX]` → base=RAX, displacement=0
- **Register + Displacement**: `[RBP-8]` → base=RBP, displacement=-8
- **Scaled Index**: `[RAX + RBX*4]` → base=RAX, index=RBX, scale=4
- **Full SIB**: `[RAX + RBX*4 + 0x10]` → all fields used

#### ARM32/ARM64 Addressing

- **Register**: `[R0]` or `[X0]` → base=R0/X0
- **Register + Offset**: `[R0, #8]` → base=R0, displacement=8
- **Register + Register**: `[R0, R1]` → base=R0, index=R1, scale=1
- **Register + Scaled Register**: `[R0, R1, LSL #2]` → base=R0, index=R1, scale=4

### 2. Benefits of Unified Representation

1. **Single Type for All Architectures**
    - No need for architecture-specific memory types
    - Simplifies IR generation and analysis
    - Easier pattern matching across architectures

2. **Deterministic Representation**
    - Fixed field order ensures consistent output
    - No ambiguity in representation
    - Easy to normalize for comparison

3. **Future-Proof**
    - Can handle new architectures (RISC-V, MIPS)
    - Flexible enough for complex addressing modes
    - Size field supports variable-width accesses

### 3. Example Mappings

```rust
// x86: mov eax, [ebx + ecx*4 + 0x10]
Memory {
base: Some(Register::X86(X86Register::EBX)),
index: Some(Register::X86(X86Register::ECX)),
scale: 4,
displacement: 0x10,
size: Some(4), // 32-bit access
}

// ARM64: ldr x0, [x1, x2, lsl #3]
Memory {
base: Some(Register::Arm64(Arm64Register::X1)),
index: Some(Register::Arm64(Arm64Register::X2)),
scale: 8, // LSL #3 = multiply by 8
displacement: 0,
size: Some(8), // 64-bit access
}

// x64: mov rax, [rip + 0x1234]
Memory {
base: Some(Register::X64(X64Register::RIP)),
index: None,
scale: 1,
displacement: 0x1234,
size: Some(8), // 64-bit access
}
```

### 4. Why Not Architecture-Specific?

Alternative (BAD) design:

```rust
// ❌ DON'T DO THIS
pub enum Memory {
    X86(X86Memory),
    X64(X64Memory),
    Arm32(Arm32Memory),
    Arm64(Arm64Memory),
}
```

Problems:

- Duplicate logic for each architecture
- Harder to find common patterns
- More complex IR transformations
- Breaks architecture-agnostic principle

### 5. Implementation Guidelines

1. **Normalization**
   ```rust
   impl Memory {
       /// Normalize representation for comparison
       pub fn normalize(&self) -> Self {
           let mut mem = self.clone();
           // If scale is 1 and no index, move to displacement
           if mem.scale == 1 && mem.index.is_some() && mem.base.is_none() {
               // Convert [index*1] to [displacement]
           }
           mem
       }
   }
   ```

2. **Architecture-Specific Display**
   ```rust
   impl Memory {
       pub fn display_for_arch(&self, arch: Architecture) -> String {
           match arch {
               Architecture::X64 | Architecture::X86 => {
                   // Intel syntax: [base + index*scale + disp]
               }
               Architecture::Arm32 | Architecture::Arm64 => {
                   // ARM syntax: [base, index, LSL #n]
               }
           }
       }
   }
   ```

3. **Validation**
   ```rust
   impl Memory {
       pub fn is_valid_for_arch(&self, arch: Architecture) -> bool {
           match arch {
               Architecture::X86 => {
                   // x86: scale must be 1, 2, 4, 8
                   // Some registers can't be index (ESP)
               }
               Architecture::Arm64 => {
                   // ARM64: scale represented as shift
                   // Different constraints
               }
               _ => true,
           }
       }
   }
   ```

## Conclusion

The current `Memory` struct is the **correct design** for Fireman's architecture-agnostic approach. It provides a
unified representation that can handle all addressing modes across x86, x86_64, ARM32, and ARM64, while maintaining
simplicity and determinism.

**No changes needed** - this design aligns perfectly with the project's goals.

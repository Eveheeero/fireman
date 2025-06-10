# Unified Architecture Implementation Design

## Overview

Design for unifying 32-bit and 64-bit implementations for both x86 and ARM architectures to maximize code reuse and
minimize duplication.

## Current Structure (Separate)

```
arch/
├── x86/
│   ├── mod.rs
│   ├── instruction_analyze.rs
│   └── register.rs
├── x86_64/
│   ├── mod.rs
│   ├── instruction_analyze.rs
│   └── register.rs
├── arm32/
│   ├── mod.rs
│   ├── instruction_analyze.rs
│   └── register.rs
└── arm64/
    ├── mod.rs
    ├── instruction_analyze.rs
    └── register.rs
```

## Proposed Unified Structure

```
arch/
├── x86_unified/
│   ├── mod.rs
│   ├── common.rs         # Shared x86/x86_64 logic
│   ├── x86_specific.rs   # 32-bit only
│   ├── x64_specific.rs   # 64-bit only
│   ├── instruction_analyze.rs
│   └── register.rs
├── arm_unified/
│   ├── mod.rs
│   ├── common.rs         # Shared ARM32/ARM64 logic
│   ├── arm32_specific.rs # 32-bit only
│   ├── arm64_specific.rs # 64-bit only
│   ├── instruction_analyze.rs
│   └── register.rs
└── architecture.rs
```

## Implementation Strategy

### 1. X86 Unified Implementation

```rust
// arch/x86_unified/mod.rs
pub mod common;
pub mod x86_specific;
pub mod x64_specific;

use crate::arch::architecture::ArchType;

pub struct X86Analyzer {
    arch_type: ArchType, // X86 or X86_64
    mode: ProcessorMode,
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessorMode {
    Real,       // 16-bit (legacy)
    Protected,  // 32-bit
    Long,       // 64-bit
}

impl X86Analyzer {
    pub fn new(arch_type: ArchType) -> Self {
        let mode = match arch_type {
            ArchType::X86 => ProcessorMode::Protected,
            ArchType::X86_64 => ProcessorMode::Long,
            _ => panic!("Invalid architecture for X86 analyzer"),
        };
        Self { arch_type, mode }
    }
}
```

```rust
// arch/x86_unified/common.rs
/// Common x86/x86_64 instruction definitions
#[derive(Debug, Clone, Copy)]
pub enum X86BaseInstruction {
    // Arithmetic (shared between 32/64-bit)
    Add, Sub, Mul, Div, Inc, Dec,
    
    // Logic (shared)
    And, Or, Xor, Not,
    
    // Control flow (shared)
    Jmp, Call, Ret,
    Jcc(Condition),
    
    // Memory (shared base)
    Mov, Push, Pop, Lea,
    
    // These exist in both but behave differently
    // based on mode (32 vs 64-bit)
}

/// Common condition codes
#[derive(Debug, Clone, Copy)]
pub enum Condition {
    Equal, NotEqual,
    Less, LessEqual,
    Greater, GreaterEqual,
    Below, BelowEqual,
    Above, AboveEqual,
    Sign, NotSign,
    Overflow, NotOverflow,
    Carry, NotCarry,
    Zero, NotZero,
}
```

```rust
// arch/x86_unified/register.rs
#[derive(Debug, Clone, Copy)]
pub enum X86Register {
    // 8-bit registers (shared)
    AL, CL, DL, BL, AH, CH, DH, BH,
    
    // 16-bit registers (shared)
    AX, CX, DX, BX, SP, BP, SI, DI,
    
    // 32-bit registers (shared)
    EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI,
    
    // 64-bit registers (x64 only)
    #[cfg(feature = "x64")]
    RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI,
    #[cfg(feature = "x64")]
    R8, R9, R10, R11, R12, R13, R14, R15,
    
    // Extended 8-bit registers (x64 only)
    #[cfg(feature = "x64")]
    R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B,
    
    // Segment registers (shared)
    CS, DS, ES, FS, GS, SS,
    
    // Special registers
    RIP, // x64 only but can be represented as EIP in x86
    RFLAGS, // EFLAGS in x86
}

impl X86Register {
    /// Get the 64-bit version of a register (if in 64-bit mode)
    pub fn to_64bit(&self) -> Option<Self> {
        match self {
            Self::EAX => Some(Self::RAX),
            Self::ECX => Some(Self::RCX),
            Self::EDX => Some(Self::RDX),
            Self::EBX => Some(Self::RBX),
            Self::ESP => Some(Self::RSP),
            Self::EBP => Some(Self::RBP),
            Self::ESI => Some(Self::RSI),
            Self::EDI => Some(Self::RDI),
            _ => None,
        }
    }
    
    /// Get register size in bytes
    pub fn size(&self) -> u8 {
        match self {
            Self::AL | Self::CL | Self::DL | Self::BL |
            Self::AH | Self::CH | Self::DH | Self::BH => 1,
            
            Self::AX | Self::CX | Self::DX | Self::BX |
            Self::SP | Self::BP | Self::SI | Self::DI => 2,
            
            Self::EAX | Self::ECX | Self::EDX | Self::EBX |
            Self::ESP | Self::EBP | Self::ESI | Self::EDI => 4,
            
            #[cfg(feature = "x64")]
            Self::RAX | Self::RCX | Self::RDX | Self::RBX |
            Self::RSP | Self::RBP | Self::RSI | Self::RDI |
            Self::R8 | Self::R9 | Self::R10 | Self::R11 |
            Self::R12 | Self::R13 | Self::R14 | Self::R15 => 8,
            
            _ => 4, // Default
        }
    }
}
```

### 2. ARM Unified Implementation

```rust
// arch/arm_unified/mod.rs
pub struct ARMAnalyzer {
    arch_type: ArchType, // ARM32 or ARM64
    instruction_set: InstructionSet,
}

#[derive(Debug, Clone, Copy)]
pub enum InstructionSet {
    ARM,      // 32-bit ARM instructions
    Thumb,    // 16-bit Thumb instructions (ARM32 only)
    Thumb2,   // Variable-length Thumb2 (ARM32 only)
    AArch64,  // 64-bit ARM instructions
}

impl ARMAnalyzer {
    pub fn new(arch_type: ArchType) -> Self {
        let instruction_set = match arch_type {
            ArchType::Arm32 => InstructionSet::ARM,
            ArchType::Arm64 => InstructionSet::AArch64,
            _ => panic!("Invalid architecture for ARM analyzer"),
        };
        Self { arch_type, instruction_set }
    }
}
```

```rust
// arch/arm_unified/common.rs
/// Common ARM concepts
#[derive(Debug, Clone, Copy)]
pub enum ARMCondition {
    EQ,  // Equal
    NE,  // Not equal
    CS,  // Carry set (HS - unsigned higher or same)
    CC,  // Carry clear (LO - unsigned lower)
    MI,  // Minus/negative
    PL,  // Plus/positive or zero
    VS,  // Overflow set
    VC,  // Overflow clear
    HI,  // Unsigned higher
    LS,  // Unsigned lower or same
    GE,  // Signed greater or equal
    LT,  // Signed less than
    GT,  // Signed greater than
    LE,  // Signed less or equal
    AL,  // Always
    NV,  // Never (obsolete)
}

/// Common instruction categories
#[derive(Debug, Clone)]
pub enum ARMInstructionCategory {
    DataProcessing {
        op: DataOp,
        cond: ARMCondition,
        set_flags: bool,
    },
    LoadStore {
        is_load: bool,
        width: AccessWidth,
        addressing: AddressingMode,
    },
    Branch {
        is_link: bool,
        cond: ARMCondition,
    },
    Multiply {
        accumulate: bool,
        long: bool,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum DataOp {
    // Arithmetic
    Add, Sub, Rsb, Adc, Sbc, Rsc,
    // Logical
    And, Orr, Eor, Bic,
    // Move
    Mov, Mvn,
    // Compare
    Cmp, Cmn, Tst, Teq,
    // Shift (ARM32 specific but concept exists in ARM64)
    Lsl, Lsr, Asr, Ror,
}
```

```rust
// arch/arm_unified/register.rs
#[derive(Debug, Clone, Copy)]
pub enum ARMRegister {
    // ARM32 general purpose (R0-R15)
    #[cfg(feature = "arm32")]
    R0, R1, R2, R3, R4, R5, R6, R7,
    R8, R9, R10, R11, R12, R13, R14, R15,
    
    // ARM64 general purpose (X0-X30, W0-W30)
    #[cfg(feature = "arm64")]
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30,
    
    // 32-bit views of X registers
    #[cfg(feature = "arm64")]
    W0, W1, W2, W3, W4, W5, W6, W7,
    W8, W9, W10, W11, W12, W13, W14, W15,
    W16, W17, W18, W19, W20, W21, W22, W23,
    W24, W25, W26, W27, W28, W29, W30,
    
    // Special registers
    SP,  // Stack pointer (R13 in ARM32, separate in ARM64)
    LR,  // Link register (R14 in ARM32, X30 in ARM64)
    PC,  // Program counter (R15 in ARM32, not directly accessible in ARM64)
    
    // Status registers
    CPSR,  // ARM32
    SPSR,  // ARM32
    NZCV,  // ARM64 condition flags
    
    // Zero register (ARM64 only)
    #[cfg(feature = "arm64")]
    XZR, WZR,
}

impl ARMRegister {
    /// Get the architectural name for the register
    pub fn name(&self, arch: ArchType) -> &'static str {
        match (self, arch) {
            (Self::SP, ArchType::Arm32) => "r13",
            (Self::LR, ArchType::Arm32) => "r14",
            (Self::PC, ArchType::Arm32) => "r15",
            (Self::SP, ArchType::Arm64) => "sp",
            (Self::LR, ArchType::Arm64) => "x30",
            _ => // ... handle other cases
        }
    }
}
```

### 3. Unified Instruction Analysis

```rust
// arch/x86_unified/instruction_analyze.rs
use crate::ir::statements::IrStatement;

pub struct X86InstructionAnalyzer {
    mode: ProcessorMode,
}

impl X86InstructionAnalyzer {
    pub fn analyze(&self, inst: &iceball::Instruction) -> Vec<IrStatement> {
        // Common analysis logic
        let base_ir = self.analyze_common(inst);
        
        // Mode-specific adjustments
        match self.mode {
            ProcessorMode::Protected => self.adjust_for_32bit(base_ir),
            ProcessorMode::Long => self.adjust_for_64bit(base_ir),
            _ => base_ir,
        }
    }
    
    fn analyze_common(&self, inst: &iceball::Instruction) -> Vec<IrStatement> {
        // 90% of instructions are the same between x86/x64
        match inst.mnemonic() {
            "add" | "sub" | "and" | "or" | "xor" => {
                self.analyze_binary_op(inst)
            }
            "mov" => self.analyze_mov(inst),
            "jmp" | "call" => self.analyze_control_flow(inst),
            _ => self.analyze_specific(inst),
        }
    }
    
    fn adjust_for_64bit(&self, mut ir: Vec<IrStatement>) -> Vec<IrStatement> {
        // Adjust for 64-bit specifics:
        // - Zero-extend 32-bit operations
        // - Handle RIP-relative addressing
        // - Process REX prefixes
        ir
    }
}
```

### 4. Benefits of Unification

1. **Code Reuse**: ~80% of x86/x64 and ~60% of ARM32/64 logic can be shared
2. **Easier Maintenance**: Fix bugs once, not four times
3. **Consistent Behavior**: Same patterns generate same IR
4. **Smaller Binary**: Less duplicated code
5. **Easier Testing**: Test common logic once

### 5. Migration Strategy

```rust
// Phase 1: Create unified structure with compatibility layer
pub mod x86_unified;
pub mod x86 {
    pub use super::x86_unified::*;
}
pub mod x86_64 {
    pub use super::x86_unified::*;
}

// Phase 2: Update all references
// Phase 3: Remove old modules
```

### 6. Conditional Compilation

```toml
# Cargo.toml
[features]
default = ["x86", "x64", "arm32", "arm64"]
x86 = []
x64 = ["x86"]  # x64 implies x86 support
arm32 = []
arm64 = []

# Allow minimal builds
minimal-x86 = ["x86", "x64"]
minimal-arm = ["arm32", "arm64"]
```

### 7. Example: Unified MOV Instruction

```rust
fn analyze_mov(&self, inst: &Instruction) -> Vec<IrStatement> {
    let (dst, src) = extract_operands(inst);
    
    // Determine operation size
    let size = match self.mode {
        ProcessorMode::Long => {
            // Check REX.W prefix for 64-bit operation
            if has_rex_w(inst) { 8 } else { 4 }
        }
        ProcessorMode::Protected => {
            // Check operand size prefix
            if has_size_prefix(inst) { 2 } else { 4 }
        }
        ProcessorMode::Real => 2,
    };
    
    // Generate appropriate IR
    vec![IrStatement::Assignment {
        to: dst.to_ir(size),
        from: src.to_ir(size),
        size: AccessSize::Bytes(size),
    }]
}
```

## Conclusion

Unifying 32/64-bit implementations is not only possible but highly recommended. It will:

- Reduce code duplication by 60-80%
- Improve maintainability
- Ensure consistent behavior
- Make adding new architectures easier

The key is identifying common patterns and parameterizing the differences.

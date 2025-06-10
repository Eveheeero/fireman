# ARM32 Instruction Set Documentation

This document provides a comprehensive overview of the ARM32 (ARMv7) instruction set implementation in the Fireman
decompiler framework.

## Overview

The ARM32 architecture uses a 32-bit instruction set with both ARM and Thumb instruction encodings. This implementation
focuses on the core ARM instruction set with support for common Thumb-2 extensions.

## Instruction Categories

### 1. Data Processing Instructions

These instructions perform arithmetic and logical operations on registers.

| Mnemonic | Description                 | Example          | Operation         |
|----------|-----------------------------|------------------|-------------------|
| **ADD**  | Add                         | `ADD R0, R1, R2` | R0 = R1 + R2      |
| **SUB**  | Subtract                    | `SUB R0, R1, R2` | R0 = R1 - R2      |
| **RSB**  | Reverse Subtract            | `RSB R0, R1, R2` | R0 = R2 - R1      |
| **ADC**  | Add with Carry              | `ADC R0, R1, R2` | R0 = R1 + R2 + C  |
| **SBC**  | Subtract with Carry         | `SBC R0, R1, R2` | R0 = R1 - R2 - !C |
| **RSC**  | Reverse Subtract with Carry | `RSC R0, R1, R2` | R0 = R2 - R1 - !C |
| **AND**  | Bitwise AND                 | `AND R0, R1, R2` | R0 = R1 & R2      |
| **ORR**  | Bitwise OR                  | `ORR R0, R1, R2` | R0 = R1 \| R2     |
| **EOR**  | Bitwise XOR                 | `EOR R0, R1, R2` | R0 = R1 ^ R2      |
| **BIC**  | Bit Clear                   | `BIC R0, R1, R2` | R0 = R1 & ~R2     |
| **MOV**  | Move                        | `MOV R0, R1`     | R0 = R1           |
| **MVN**  | Move NOT                    | `MVN R0, R1`     | R0 = ~R1          |

### 2. Comparison Instructions

These instructions update condition flags without storing results.

| Mnemonic | Description      | Example      | Operation                     |
|----------|------------------|--------------|-------------------------------|
| **CMP**  | Compare          | `CMP R0, R1` | Update flags based on R0 - R1 |
| **CMN**  | Compare Negative | `CMN R0, R1` | Update flags based on R0 + R1 |
| **TST**  | Test             | `TST R0, R1` | Update flags based on R0 & R1 |
| **TEQ**  | Test Equivalence | `TEQ R0, R1` | Update flags based on R0 ^ R1 |

### 3. Multiply Instructions

| Mnemonic  | Description                       | Example                | Operation                 |
|-----------|-----------------------------------|------------------------|---------------------------|
| **MUL**   | Multiply                          | `MUL R0, R1, R2`       | R0 = R1 × R2              |
| **MLA**   | Multiply Accumulate               | `MLA R0, R1, R2, R3`   | R0 = (R1 × R2) + R3       |
| **UMULL** | Unsigned Multiply Long            | `UMULL R0, R1, R2, R3` | R1:R0 = R2 × R3           |
| **UMLAL** | Unsigned Multiply Accumulate Long | `UMLAL R0, R1, R2, R3` | R1:R0 += R2 × R3          |
| **SMULL** | Signed Multiply Long              | `SMULL R0, R1, R2, R3` | R1:R0 = R2 × R3 (signed)  |
| **SMLAL** | Signed Multiply Accumulate Long   | `SMLAL R0, R1, R2, R3` | R1:R0 += R2 × R3 (signed) |

### 4. Division Instructions (ARMv7+)

| Mnemonic | Description     | Example           | Operation               |
|----------|-----------------|-------------------|-------------------------|
| **SDIV** | Signed Divide   | `SDIV R0, R1, R2` | R0 = R1 ÷ R2 (signed)   |
| **UDIV** | Unsigned Divide | `UDIV R0, R1, R2` | R0 = R1 ÷ R2 (unsigned) |

### 5. Saturating Arithmetic

These instructions saturate results instead of wrapping on overflow.

| Mnemonic  | Description                    | Example            | Operation                  |
|-----------|--------------------------------|--------------------|----------------------------|
| **QADD**  | Saturating Add                 | `QADD R0, R1, R2`  | R0 = sat(R1 + R2)          |
| **QSUB**  | Saturating Subtract            | `QSUB R0, R1, R2`  | R0 = sat(R1 - R2)          |
| **QDADD** | Saturating Double and Add      | `QDADD R0, R1, R2` | R0 = sat(R1 + sat(2 × R2)) |
| **QDSUB** | Saturating Double and Subtract | `QDSUB R0, R1, R2` | R0 = sat(R1 - sat(2 × R2)) |

### 6. Load/Store Instructions

#### Single Register Load/Store

| Mnemonic  | Description                   | Example             | Operation                          |
|-----------|-------------------------------|---------------------|------------------------------------|
| **LDR**   | Load Register                 | `LDR R0, [R1]`      | R0 = Memory[R1]                    |
| **LDRB**  | Load Register Byte            | `LDRB R0, [R1]`     | R0 = Memory[R1] (byte)             |
| **LDRH**  | Load Register Halfword        | `LDRH R0, [R1]`     | R0 = Memory[R1] (halfword)         |
| **LDRSB** | Load Register Signed Byte     | `LDRSB R0, [R1]`    | R0 = SignExt(Memory[R1])           |
| **LDRSH** | Load Register Signed Halfword | `LDRSH R0, [R1]`    | R0 = SignExt(Memory[R1])           |
| **LDRD**  | Load Register Dual            | `LDRD R0, R1, [R2]` | R0 = Memory[R2], R1 = Memory[R2+4] |
| **STR**   | Store Register                | `STR R0, [R1]`      | Memory[R1] = R0                    |
| **STRB**  | Store Register Byte           | `STRB R0, [R1]`     | Memory[R1] = R0 (byte)             |
| **STRH**  | Store Register Halfword       | `STRH R0, [R1]`     | Memory[R1] = R0 (halfword)         |
| **STRD**  | Store Register Dual           | `STRD R0, R1, [R2]` | Memory[R2] = R0, Memory[R2+4] = R1 |

#### Multiple Register Load/Store

| Mnemonic | Description    | Example            | Operation                 |
|----------|----------------|--------------------|---------------------------|
| **LDM**  | Load Multiple  | `LDM R0!, {R1-R4}` | Load R1-R4 from [R0]      |
| **STM**  | Store Multiple | `STM R0!, {R1-R4}` | Store R1-R4 to [R0]       |
| **PUSH** | Push registers | `PUSH {R0-R3, LR}` | Store registers on stack  |
| **POP**  | Pop registers  | `POP {R0-R3, PC}`  | Load registers from stack |

#### Exclusive Access (for synchronization)

| Mnemonic   | Description                | Example                   | Operation                              |
|------------|----------------------------|---------------------------|----------------------------------------|
| **LDREX**  | Load Exclusive             | `LDREX R0, [R1]`          | R0 = Memory[R1], set exclusive monitor |
| **STREX**  | Store Exclusive            | `STREX R0, R1, [R2]`      | Conditionally store R1 to [R2]         |
| **LDREXB** | Load Exclusive Byte        | `LDREXB R0, [R1]`         | Load byte with exclusive access        |
| **STREXB** | Store Exclusive Byte       | `STREXB R0, R1, [R2]`     | Store byte with exclusive access       |
| **LDREXH** | Load Exclusive Halfword    | `LDREXH R0, [R1]`         | Load halfword with exclusive access    |
| **STREXH** | Store Exclusive Halfword   | `STREXH R0, R1, [R2]`     | Store halfword with exclusive access   |
| **LDREXD** | Load Exclusive Doubleword  | `LDREXD R0, R1, [R2]`     | Load doubleword with exclusive access  |
| **STREXD** | Store Exclusive Doubleword | `STREXD R0, R1, R2, [R3]` | Store doubleword with exclusive access |

### 7. Branch Instructions

| Mnemonic | Description                   | Example       | Operation                         |
|----------|-------------------------------|---------------|-----------------------------------|
| **B**    | Branch                        | `B label`     | PC = label                        |
| **BL**   | Branch with Link              | `BL function` | LR = PC + 4, PC = function        |
| **BX**   | Branch and Exchange           | `BX R0`       | PC = R0, switch ARM/Thumb         |
| **BLX**  | Branch with Link and Exchange | `BLX R0`      | LR = PC + 4, PC = R0, switch mode |

### 8. Conditional Execution

#### IT Block (Thumb-2)

| Mnemonic | Description                    | Example          | Operation                        |
|----------|--------------------------------|------------------|----------------------------------|
| **IT**   | If-Then                        | `IT EQ`          | Next instruction executes if Z=1 |
| **CBZ**  | Compare and Branch if Zero     | `CBZ R0, label`  | Branch if R0 == 0                |
| **CBNZ** | Compare and Branch if Non-Zero | `CBNZ R0, label` | Branch if R0 != 0                |

### 9. Shift and Rotate Instructions

| Mnemonic | Description              | Example          | Operation                    |
|----------|--------------------------|------------------|------------------------------|
| **LSL**  | Logical Shift Left       | `LSL R0, R1, #2` | R0 = R1 << 2                 |
| **LSR**  | Logical Shift Right      | `LSR R0, R1, #2` | R0 = R1 >> 2 (unsigned)      |
| **ASR**  | Arithmetic Shift Right   | `ASR R0, R1, #2` | R0 = R1 >> 2 (signed)        |
| **ROR**  | Rotate Right             | `ROR R0, R1, #2` | R0 = rotate_right(R1, 2)     |
| **RRX**  | Rotate Right with Extend | `RRX R0, R1`     | R0 = rotate_right_extend(R1) |

### 10. Bit Field Instructions

| Mnemonic | Description                | Example               | Operation                            |
|----------|----------------------------|-----------------------|--------------------------------------|
| **BFI**  | Bit Field Insert           | `BFI R0, R1, #8, #4`  | Insert R1[3:0] into R0[11:8]         |
| **BFC**  | Bit Field Clear            | `BFC R0, #8, #4`      | Clear R0[11:8]                       |
| **SBFX** | Signed Bit Field Extract   | `SBFX R0, R1, #8, #4` | Extract R1[11:8] with sign extension |
| **UBFX** | Unsigned Bit Field Extract | `UBFX R0, R1, #8, #4` | Extract R1[11:8] zero extended       |

### 11. SIMD Instructions (NEON)

The implementation includes various NEON SIMD instructions for parallel data processing:

- **VADD**: Vector Add
- **VSUB**: Vector Subtract
- **VMUL**: Vector Multiply
- **VAND**: Vector AND
- **VORR**: Vector OR
- **VEOR**: Vector XOR
- **VMOV**: Vector Move
- **VLD1-VLD4**: Vector Load
- **VST1-VST4**: Vector Store

### 12. Coprocessor Instructions

| Mnemonic | Description                 | Example                     | Operation              |
|----------|-----------------------------|-----------------------------|------------------------|
| **MCR**  | Move to Coprocessor         | `MCR p15, 0, R0, c1, c0, 0` | Coprocessor ← Register |
| **MRC**  | Move from Coprocessor       | `MRC p15, 0, R0, c1, c0, 0` | Register ← Coprocessor |
| **CDP**  | Coprocessor Data Processing | `CDP p15, 1, c0, c1, c2, 3` | Coprocessor operation  |
| **LDC**  | Load Coprocessor            | `LDC p15, c0, [R1]`         | Coprocessor ← Memory   |
| **STC**  | Store Coprocessor           | `STC p15, c0, [R1]`         | Memory ← Coprocessor   |

### 13. System Instructions

| Mnemonic    | Description               | Example          | Operation             |
|-------------|---------------------------|------------------|-----------------------|
| **SWI/SVC** | Software Interrupt        | `SVC #0`         | Enter supervisor mode |
| **MRS**     | Move from Status Register | `MRS R0, CPSR`   | R0 = CPSR             |
| **MSR**     | Move to Status Register   | `MSR CPSR_f, R0` | CPSR flags = R0       |
| **NOP**     | No Operation              | `NOP`            | Do nothing            |
| **WFI**     | Wait For Interrupt        | `WFI`            | Enter low-power state |
| **WFE**     | Wait For Event            | `WFE`            | Wait for event        |
| **SEV**     | Send Event                | `SEV`            | Send event signal     |

## Condition Codes

Most ARM instructions can be conditionally executed based on condition flags:

| Suffix    | Condition                 | Flags           |
|-----------|---------------------------|-----------------|
| **EQ**    | Equal                     | Z = 1           |
| **NE**    | Not Equal                 | Z = 0           |
| **CS/HS** | Carry Set/Higher or Same  | C = 1           |
| **CC/LO** | Carry Clear/Lower         | C = 0           |
| **MI**    | Minus/Negative            | N = 1           |
| **PL**    | Plus/Positive             | N = 0           |
| **VS**    | Overflow Set              | V = 1           |
| **VC**    | Overflow Clear            | V = 0           |
| **HI**    | Higher (unsigned)         | C = 1 and Z = 0 |
| **LS**    | Lower or Same (unsigned)  | C = 0 or Z = 1  |
| **GE**    | Greater or Equal (signed) | N = V           |
| **LT**    | Less Than (signed)        | N ≠ V           |
| **GT**    | Greater Than (signed)     | Z = 0 and N = V |
| **LE**    | Less or Equal (signed)    | Z = 1 or N ≠ V  |
| **AL**    | Always                    | Any             |

## Addressing Modes

ARM32 supports flexible addressing modes:

1. **Immediate**: `#value`
2. **Register**: `Rn`
3. **Register with offset**: `[Rn, #offset]`
4. **Register with register offset**: `[Rn, Rm]`
5. **Register with scaled register offset**: `[Rn, Rm, LSL #2]`
6. **Pre-indexed**: `[Rn, #offset]!`
7. **Post-indexed**: `[Rn], #offset`

## Implementation Status

✅ **Fully Implemented:**

- Basic data processing instructions
- Load/store instructions
- Branch instructions
- Multiply instructions
- Shift operations

🚧 **Partially Implemented:**

- SIMD/NEON instructions
- Coprocessor operations
- System control instructions

📋 **Planned:**

- Complete NEON instruction set
- Thumb-2 specific encodings
- Advanced system instructions

## Usage in Fireman

The ARM32 instruction decoder is integrated into the Fireman decompiler through the `iceball` disassembly library.
Instructions are parsed and converted to the intermediate representation (IR) for further analysis and decompilation.

### Example Usage

```rust
use iceball::{Statement, Arm32Statement};

// Parse an ARM32 instruction
let statement = Statement::Arm32(Arm32Statement::Add);

// Convert to IR for decompilation
let ir = create_ir_statement( & instruction);
```

## References

- ARM Architecture Reference Manual ARMv7-A and ARMv7-R edition
- ARM Instruction Set Quick Reference Card
- Thumb-2 Supplement Reference Manual

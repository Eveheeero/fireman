# x86 (32-bit) Instruction Set Documentation

This document provides a comprehensive overview of the x86 (32-bit) instruction set implementation in the Fireman
decompiler framework.

## Overview

The x86 architecture, also known as IA-32 or i386, is Intel's 32-bit instruction set architecture. This implementation
covers the core x86 instructions including legacy instructions not available in x86-64 mode.

## Key Differences from x86-64

- **Register Size**: Primary registers are 32-bit (EAX, EBX, etc.) vs 64-bit (RAX, RBX)
- **Address Space**: 32-bit addressing (4GB limit) vs 64-bit
- **Legacy Instructions**: Includes BCD arithmetic (AAA, AAS, DAA, DAS) not available in 64-bit mode
- **Segmentation**: More prominent use of segment registers
- **Stack Operations**: PUSHAD/POPAD for saving all general registers

## Instruction Categories

### 1. Data Transfer Instructions

| Mnemonic   | Description            | Example            | Operation                            |
|------------|------------------------|--------------------|--------------------------------------|
| **MOV**    | Move data              | `MOV EAX, EBX`     | EAX = EBX                            |
| **MOVZX**  | Move with zero extend  | `MOVZX EAX, BL`    | EAX = zero_extend(BL)                |
| **MOVSX**  | Move with sign extend  | `MOVSX EAX, BL`    | EAX = sign_extend(BL)                |
| **LEA**    | Load effective address | `LEA EAX, [EBX+4]` | EAX = EBX + 4                        |
| **XCHG**   | Exchange               | `XCHG EAX, EBX`    | EAX ↔ EBX                            |
| **PUSH**   | Push to stack          | `PUSH EAX`         | [ESP-4] = EAX, ESP -= 4              |
| **POP**    | Pop from stack         | `POP EAX`          | EAX = [ESP], ESP += 4                |
| **PUSHAD** | Push all registers     | `PUSHAD`           | Push EAX,ECX,EDX,EBX,ESP,EBP,ESI,EDI |
| **POPAD**  | Pop all registers      | `POPAD`            | Pop EDI,ESI,EBP,ESP,EBX,EDX,ECX,EAX  |
| **PUSHFD** | Push EFLAGS            | `PUSHFD`           | Push EFLAGS register                 |
| **POPFD**  | Pop EFLAGS             | `POPFD`            | Pop EFLAGS register                  |

### 2. Arithmetic Instructions

#### Basic Arithmetic

| Mnemonic | Description               | Example        | Operation                    |
|----------|---------------------------|----------------|------------------------------|
| **ADD**  | Add                       | `ADD EAX, EBX` | EAX = EAX + EBX              |
| **ADC**  | Add with carry            | `ADC EAX, EBX` | EAX = EAX + EBX + CF         |
| **SUB**  | Subtract                  | `SUB EAX, EBX` | EAX = EAX - EBX              |
| **SBB**  | Subtract with borrow      | `SBB EAX, EBX` | EAX = EAX - EBX - CF         |
| **INC**  | Increment                 | `INC EAX`      | EAX = EAX + 1                |
| **DEC**  | Decrement                 | `DEC EAX`      | EAX = EAX - 1                |
| **NEG**  | Two's complement negation | `NEG EAX`      | EAX = -EAX                   |
| **CMP**  | Compare                   | `CMP EAX, EBX` | Set flags based on EAX - EBX |

#### Multiplication and Division

| Mnemonic | Description       | Example         | Operation                            |
|----------|-------------------|-----------------|--------------------------------------|
| **MUL**  | Unsigned multiply | `MUL EBX`       | EDX:EAX = EAX × EBX                  |
| **IMUL** | Signed multiply   | `IMUL EAX, EBX` | EAX = EAX × EBX                      |
| **DIV**  | Unsigned divide   | `DIV EBX`       | EAX = EDX:EAX ÷ EBX, EDX = remainder |
| **IDIV** | Signed divide     | `IDIV EBX`      | EAX = EDX:EAX ÷ EBX, EDX = remainder |

#### BCD Arithmetic (x86-specific, not in x64)

| Mnemonic | Description                       | Example | Operation                         |
|----------|-----------------------------------|---------|-----------------------------------|
| **DAA**  | Decimal adjust after addition     | `DAA`   | Adjust AL after BCD addition      |
| **DAS**  | Decimal adjust after subtraction  | `DAS`   | Adjust AL after BCD subtraction   |
| **AAA**  | ASCII adjust after addition       | `AAA`   | Adjust AX after ASCII addition    |
| **AAS**  | ASCII adjust after subtraction    | `AAS`   | Adjust AX after ASCII subtraction |
| **AAM**  | ASCII adjust after multiplication | `AAM`   | Adjust AX after multiplication    |
| **AAD**  | ASCII adjust before division      | `AAD`   | Adjust AX before division         |

### 3. Logical Instructions

| Mnemonic | Description     | Example         | Operation                    |
|----------|-----------------|-----------------|------------------------------|
| **AND**  | Bitwise AND     | `AND EAX, EBX`  | EAX = EAX & EBX              |
| **OR**   | Bitwise OR      | `OR EAX, EBX`   | EAX = EAX \| EBX             |
| **XOR**  | Bitwise XOR     | `XOR EAX, EBX`  | EAX = EAX ^ EBX              |
| **NOT**  | Bitwise NOT     | `NOT EAX`       | EAX = ~EAX                   |
| **TEST** | Logical compare | `TEST EAX, EBX` | Set flags based on EAX & EBX |

### 4. Shift and Rotate Instructions

| Mnemonic    | Description                | Example       | Operation                      |
|-------------|----------------------------|---------------|--------------------------------|
| **SHL/SAL** | Shift left                 | `SHL EAX, CL` | EAX = EAX << CL                |
| **SHR**     | Logical shift right        | `SHR EAX, CL` | EAX = EAX >> CL (zero fill)    |
| **SAR**     | Arithmetic shift right     | `SAR EAX, CL` | EAX = EAX >> CL (sign fill)    |
| **ROL**     | Rotate left                | `ROL EAX, CL` | Rotate EAX left by CL bits     |
| **ROR**     | Rotate right               | `ROR EAX, CL` | Rotate EAX right by CL bits    |
| **RCL**     | Rotate left through carry  | `RCL EAX, CL` | Rotate EAX:CF left by CL bits  |
| **RCR**     | Rotate right through carry | `RCR EAX, CL` | Rotate EAX:CF right by CL bits |

### 5. Control Transfer Instructions

#### Unconditional Jumps

| Mnemonic  | Description    | Example     | Operation              |
|-----------|----------------|-------------|------------------------|
| **JMP**   | Jump           | `JMP label` | EIP = label            |
| **CALL**  | Call procedure | `CALL func` | Push EIP+4, EIP = func |
| **RET**   | Return         | `RET`       | Pop EIP                |
| **RET n** | Return and pop | `RET 8`     | Pop EIP, ESP += 8      |

#### Conditional Jumps

| Mnemonic    | Condition                  | Flags              |
|-------------|----------------------------|--------------------|
| **JE/JZ**   | Jump if equal/zero         | ZF = 1             |
| **JNE/JNZ** | Jump if not equal/not zero | ZF = 0             |
| **JA/JNBE** | Jump if above (unsigned)   | CF = 0 and ZF = 0  |
| **JAE/JNB** | Jump if above or equal     | CF = 0             |
| **JB/JNAE** | Jump if below (unsigned)   | CF = 1             |
| **JBE/JNA** | Jump if below or equal     | CF = 1 or ZF = 1   |
| **JG/JNLE** | Jump if greater (signed)   | ZF = 0 and SF = OF |
| **JGE/JNL** | Jump if greater or equal   | SF = OF            |
| **JL/JNGE** | Jump if less (signed)      | SF ≠ OF            |
| **JLE/JNG** | Jump if less or equal      | ZF = 1 or SF ≠ OF  |
| **JS**      | Jump if sign               | SF = 1             |
| **JNS**     | Jump if not sign           | SF = 0             |
| **JO**      | Jump if overflow           | OF = 1             |
| **JNO**     | Jump if not overflow       | OF = 0             |
| **JP/JPE**  | Jump if parity even        | PF = 1             |
| **JNP/JPO** | Jump if parity odd         | PF = 0             |

### 6. String Instructions

| Mnemonic        | Description            | Example       | Operation                       |
|-----------------|------------------------|---------------|---------------------------------|
| **MOVS**        | Move string            | `MOVSB`       | [EDI] = [ESI], inc/dec ESI,EDI  |
| **CMPS**        | Compare string         | `CMPSB`       | Compare [ESI] with [EDI]        |
| **SCAS**        | Scan string            | `SCASB`       | Compare AL with [EDI]           |
| **LODS**        | Load string            | `LODSB`       | AL = [ESI], inc/dec ESI         |
| **STOS**        | Store string           | `STOSB`       | [EDI] = AL, inc/dec EDI         |
| **REP**         | Repeat prefix          | `REP MOVSB`   | Repeat while ECX ≠ 0            |
| **REPE/REPZ**   | Repeat while equal     | `REPE CMPSB`  | Repeat while ZF = 1 and ECX ≠ 0 |
| **REPNE/REPNZ** | Repeat while not equal | `REPNE SCASB` | Repeat while ZF = 0 and ECX ≠ 0 |

### 7. Flag Control Instructions

| Mnemonic | Description           | Operation  |
|----------|-----------------------|------------|
| **STC**  | Set carry flag        | CF = 1     |
| **CLC**  | Clear carry flag      | CF = 0     |
| **CMC**  | Complement carry flag | CF = ~CF   |
| **STD**  | Set direction flag    | DF = 1     |
| **CLD**  | Clear direction flag  | DF = 0     |
| **STI**  | Set interrupt flag    | IF = 1     |
| **CLI**  | Clear interrupt flag  | IF = 0     |
| **SAHF** | Store AH into flags   | FLAGS = AH |
| **LAHF** | Load flags into AH    | AH = FLAGS |

### 8. Bit Manipulation Instructions

| Mnemonic | Description             | Example        | Operation                            |
|----------|-------------------------|----------------|--------------------------------------|
| **BT**   | Bit test                | `BT EAX, 5`    | CF = bit 5 of EAX                    |
| **BTS**  | Bit test and set        | `BTS EAX, 5`   | CF = bit 5 of EAX, then set bit 5    |
| **BTR**  | Bit test and reset      | `BTR EAX, 5`   | CF = bit 5 of EAX, then clear bit 5  |
| **BTC**  | Bit test and complement | `BTC EAX, 5`   | CF = bit 5 of EAX, then toggle bit 5 |
| **BSF**  | Bit scan forward        | `BSF EAX, EBX` | EAX = index of first set bit in EBX  |
| **BSR**  | Bit scan reverse        | `BSR EAX, EBX` | EAX = index of last set bit in EBX   |

### 9. System Instructions

| Mnemonic  | Description             | Example    | Operation                 |
|-----------|-------------------------|------------|---------------------------|
| **INT**   | Software interrupt      | `INT 0x80` | Call interrupt handler    |
| **INTO**  | Interrupt on overflow   | `INTO`     | INT 4 if OF = 1           |
| **IRET**  | Return from interrupt   | `IRET`     | Pop EIP, CS, EFLAGS       |
| **HLT**   | Halt                    | `HLT`      | Halt until interrupt      |
| **NOP**   | No operation            | `NOP`      | Do nothing                |
| **CPUID** | CPU identification      | `CPUID`    | Get CPU info based on EAX |
| **RDTSC** | Read time stamp counter | `RDTSC`    | EDX:EAX = TSC             |

### 10. FPU Instructions (x87)

#### Data Transfer

| Mnemonic | Description               | Example                |
|----------|---------------------------|------------------------|
| **FLD**  | Load floating point       | `FLD DWORD PTR [EBX]`  |
| **FST**  | Store floating point      | `FST DWORD PTR [EBX]`  |
| **FSTP** | Store and pop             | `FSTP QWORD PTR [EBX]` |
| **FXCH** | Exchange ST(0) with ST(i) | `FXCH ST(1)`           |

#### Arithmetic

| Mnemonic  | Description             | Example             |
|-----------|-------------------------|---------------------|
| **FADD**  | Floating point add      | `FADD ST(1), ST(0)` |
| **FSUB**  | Floating point subtract | `FSUB ST(1), ST(0)` |
| **FMUL**  | Floating point multiply | `FMUL ST(1), ST(0)` |
| **FDIV**  | Floating point divide   | `FDIV ST(1), ST(0)` |
| **FSQRT** | Square root             | `FSQRT`             |
| **FABS**  | Absolute value          | `FABS`              |
| **FCHS**  | Change sign             | `FCHS`              |

### 11. MMX Instructions

| Mnemonic   | Description            | Example           |
|------------|------------------------|-------------------|
| **MOVD**   | Move doubleword        | `MOVD MM0, EAX`   |
| **MOVQ**   | Move quadword          | `MOVQ MM0, MM1`   |
| **PADDB**  | Packed add bytes       | `PADDB MM0, MM1`  |
| **PADDW**  | Packed add words       | `PADDW MM0, MM1`  |
| **PADDD**  | Packed add doublewords | `PADDD MM0, MM1`  |
| **PSUBB**  | Packed subtract bytes  | `PSUBB MM0, MM1`  |
| **PMULLW** | Packed multiply low    | `PMULLW MM0, MM1` |
| **EMMS**   | Empty MMX state        | `EMMS`            |

### 12. SSE/SSE2 Instructions

#### Data Movement

| Mnemonic   | Description                  | Example              |
|------------|------------------------------|----------------------|
| **MOVAPS** | Move aligned packed single   | `MOVAPS XMM0, XMM1`  |
| **MOVUPS** | Move unaligned packed single | `MOVUPS XMM0, [EBX]` |
| **MOVSS**  | Move scalar single           | `MOVSS XMM0, [EBX]`  |
| **MOVAPD** | Move aligned packed double   | `MOVAPD XMM0, XMM1`  |
| **MOVSD**  | Move scalar double           | `MOVSD XMM0, [EBX]`  |

#### Arithmetic

| Mnemonic   | Description               | Example             |
|------------|---------------------------|---------------------|
| **ADDPS**  | Add packed single         | `ADDPS XMM0, XMM1`  |
| **SUBPS**  | Subtract packed single    | `SUBPS XMM0, XMM1`  |
| **MULPS**  | Multiply packed single    | `MULPS XMM0, XMM1`  |
| **DIVPS**  | Divide packed single      | `DIVPS XMM0, XMM1`  |
| **ADDSS**  | Add scalar single         | `ADDSS XMM0, XMM1`  |
| **SQRTPS** | Square root packed single | `SQRTPS XMM0, XMM1` |

## Addressing Modes

x86 supports complex addressing modes:

1. **Immediate**: `MOV EAX, 0x1234`
2. **Register**: `MOV EAX, EBX`
3. **Direct**: `MOV EAX, [0x401000]`
4. **Register Indirect**: `MOV EAX, [EBX]`
5. **Base + Displacement**: `MOV EAX, [EBX + 4]`
6. **Base + Index**: `MOV EAX, [EBX + ESI]`
7. **Base + Index × Scale**: `MOV EAX, [EBX + ESI*4]`
8. **Base + Index × Scale + Displacement**: `MOV EAX, [EBX + ESI*4 + 8]`

## Segment Registers

x86 uses segment registers for memory segmentation:

- **CS**: Code Segment
- **DS**: Data Segment
- **ES**: Extra Segment
- **FS**: Additional Segment
- **GS**: Additional Segment
- **SS**: Stack Segment

## Condition Codes (EFLAGS)

| Flag   | Name           | Description                     |
|--------|----------------|---------------------------------|
| **CF** | Carry Flag     | Set if arithmetic carry/borrow  |
| **PF** | Parity Flag    | Set if low byte has even parity |
| **AF** | Auxiliary Flag | Set if carry from bit 3         |
| **ZF** | Zero Flag      | Set if result is zero           |
| **SF** | Sign Flag      | Set if result is negative       |
| **OF** | Overflow Flag  | Set if signed overflow          |
| **DF** | Direction Flag | String operation direction      |
| **IF** | Interrupt Flag | Enable external interrupts      |

## Implementation Status

✅ **Fully Implemented:**

- Basic data transfer (MOV, PUSH, POP, LEA, XCHG)
- Arithmetic instructions (ADD, SUB, MUL, DIV, etc.)
- Logical operations (AND, OR, XOR, NOT)
- Shift and rotate instructions
- Basic control flow (JMP, Jcc, CALL, RET)
- Comparison instructions

🚧 **Partially Implemented:**

- BCD arithmetic instructions
- String operations
- FPU instructions
- MMX instructions
- SSE/SSE2 instructions

📋 **Planned:**

- Complete BCD arithmetic support
- Full string instruction support
- Complete FPU instruction set
- Advanced system instructions
- Segment override prefixes

## Usage in Fireman

The x86 instruction decoder is integrated into the Fireman decompiler through the `iceball` disassembly library.
Instructions are parsed and converted to the intermediate representation (IR) for analysis.

### Example Usage

```rust
use iceball::{Statement, X86Statement};

// Parse an x86 instruction
let statement = Statement::X86(X86Statement::Mov);

// Convert to IR for decompilation
let ir = create_ir_statement( & instruction);
```

## Key Implementation Notes

1. **32-bit Focus**: All operations assume 32-bit mode
2. **Legacy Support**: Includes instructions removed in x64 (BCD, BOUND, INTO)
3. **Stack Pointer**: ESP is used instead of RSP
4. **Instruction Pointer**: EIP is used instead of RIP
5. **Deterministic**: All operations maintain deterministic output

## References

- Intel® 64 and IA-32 Architectures Software Developer's Manual
- x86 Instruction Set Reference
- Intel x86 Assembly Language Reference Manual

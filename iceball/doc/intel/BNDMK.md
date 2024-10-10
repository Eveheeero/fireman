# BNDMK

Make Bounds

InstructionMode Feature SupportFlagF3 0F 1B /rRMNE/VMPXMake lower and upper bounds from m32 and store them in bnd.BNDMK bnd, m32F3 0F 1B /rRMV/NEMPXMake lower and upper bounds from m64 and store them in bnd.BNDMK bnd, m64Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RMModRM:reg (w)ModRM:r/m (r)N/AMakes bounds from the second operand and stores the lower and upper bounds in the bound register bnd.
The second operand must be a memory operand.
The content of the base register from the memory operand is stored in the lower bound bnd.LB.
The 1's complement of the effective address of m32/m64 is stored in the upper bound b.UB.
Computation of m32/m64 has identical behavior to LEA.This instruction does not cause any memory access, and does not read or write any flags.
If the instruction did not specify base register, the lower bound will be zero.
The reg-reg form of this instruction retains legacy behavior (NOP).
The instruction causes an invalid-opcode exception (#UD) if executed in 64-bit mode with RIP-relative addressing.

## Flags affected

- None.

## Exceptions

- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m and REX encodes BND4-B
  > ND15 when Intel MPX is enabled.
  > If RIP-relative addressing is used.
  - #SS(0) - If the memory address referencing the SS segment is in a non-canonical form.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 67H prefix is not used and CS.D=0.
  > If 67H prefix is used and CS.D=1.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If ModRM.r/m encodes BND4-BND7 when Intel MPX is enabled.
  > If 16-bit addressing is used.

## Operation

```C
BND.LB := SRCMEM.base;IF 64-bit mode ThenBND.UB := NOT(LEA.64_bits(SRCMEM)); ELSEBND.UB := Zero_Extend.64_bits(NOT(LEA.32_bits(SRCMEM)));FI;Intel C/C++ Compiler Intrinsic EquivalentBNDMKvoid * _bnd_set_ptr_bounds(const void * q, size_t size); 
```

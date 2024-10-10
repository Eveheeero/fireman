# CMPXCHG8B/CMPXCHG16B

Compare and Exchange Bytes

Compares the 64-bit value in EDX:EAX (or 128-bit value in RDX:RAX if operand size is 128 bits) with the operand (destination operand).
If the values are equal, the 64-bit value in ECX:EBX (or 128-bit value in RCX:RBX) is stored in the destination operand.
Otherwise, the value in the destination operand is loaded into EDX:EAX (or RDX:RAX).
The destination operand is an 8-byte memory location (or 16-byte memory location if operand size is 128 bits).
For the EDX:EAX and ECX:EBX register pairs, EDX and ECX contain the high-order 32 bits and EAX and EBX contain the low-order 32 bits of a 64-bit value.
For the RDX:RAX and RCX:RBX register pairs, RDX and RCX contain the high-order 64 bits and RAX and RBX contain the low-order 64bits of a 128-bit value.
This instruction can be used with a LOCK prefix to allow the instruction to be executed atomically.
To simplify the interface to the processor's bus, the destination operand receives a write cycle without regard to the result of the comparison.
The destination operand is written back if the comparison fails; otherwise, the source operand is written into the destination.
(The processor never produces a locked read without also producing a locked write.)In 64-bit mode, default operation size is 64 bits.
Use of the REX.W prefix promotes operation to 128 bits.
Note that CMPXCHG16B requires that the destination (memory) operand be 16-byte aligned.
See the summary chart at the beginning of this section for encoding data and limits.
For information on the CPUID flag that indicates CMPX-CHG16B, see page 3-242.

## Flags affected

- The ZF flag is set if the destination operand and EDX:EAX are equal; otherwise it is cleared. The CF, PF, AF, SF, and OF flags are unaffected.

## Exceptions

- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  > If memory operand for CMPXCHG16B is
  >  not aligned on a 16-byte boundary.
  > If CPUID.01H:ECX.CMPXCHG16B[bit 13] = 0.
  - #UD - If the destination operand is not a memory location.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  - #UD - If the destination operand is not a memory location.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
- Protected Mode Exceptions
  - #UD - If the destination is not a memory operand.
  - #GP(0) - If the destination is located in a non-writable segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
- Real-Address Mode Exceptions
  - #UD - If the destination operand is not a memory location.
  - #GP - If a memory operand effective address is ou

## Operation

```C
IF (64-Bit Mode and OperandSize = 64)THENTEMP128 := DEST= TEMP128)IF (RDX:RAX THENZF := 1;DEST := RCX:RBX;ELSEZF := 0;RDX:RAX := TEMP128;DEST := TEMP128;FI;FIELSETEMP64 := DEST;= TEMP64)IF (EDX:EAX THENZF := 1;DEST := ECX:EBX;ELSEZF := 0;EDX:EAX := TEMP64;DEST := TEMP64;FI;FI;FI;
```

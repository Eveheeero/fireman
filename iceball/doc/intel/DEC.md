# DEC

Decrement by 1

Subtracts 1 from the destination operand, while preserving the state of the CF flag.
The destination operand can be a register or a memory location.
This instruction allows a loop counter to be updated without disturbing the CF flag.
(To perform a decrement operation that updates the CF flag, use a SUB instruction with an immediate operand of 1.)This instruction can be used with a LOCK prefix to allow the instruction to be executed atomically.In 64-bit mode, DEC r16 and DEC r32 are not encodable (because opcodes 48H through 4FH are REX prefixes).
Otherwise, the instruction's 64-bit mode default operation size is 32 bits.
Use of the REX.R prefix permits access to additional registers (R8-R15).
Use of the REX.W prefix promotes operation to 64 bits.
See the summary chart at the beginning of this section for encoding data and limits.

## Flags affected

- The CF flag is not affected. The OF, SF, ZF, AF, and PF flags are set according to the result.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
- Real-Address Mode Exceptions
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used but th
  > e destination is not a memory operand.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used but th
  > e destination is not a memory operand.
- Protected Mode Exceptions
  - #GP(0) - If the destination operand is
  > located in a non-writable segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used but the destination is not a memory operand.

## Operation

```C
DEST := DEST - 1;
```

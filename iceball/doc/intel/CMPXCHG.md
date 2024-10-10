# CMPXCHG

Compare and Exchange

Compares the value in the AL, AX, EAX, or RAX register with the first operand (destination operand).
If the two values are equal, the second operand (source operand) is loaded into the destination operand.
Otherwise, the destination operand is loaded into the AL, AX, EAX or RAX register.
RAX register is available only in 64-bit mode.This instruction can be used with a LOCK prefix to allow the instruction to be executed atomically.
To simplify the interface to the processor's bus, the destination operand receives a write cycle without regard to the result of the comparison.
The destination operand is written back if the comparison fails; otherwise, the source operand is written into the destination.
(The processor never produces a locked read without also producing a locked write.)In 64-bit mode, the instruction's default operation size is 32 bits.
Use of the REX.R prefix permits access to addi-tional registers (R8-R15).
Use of the REX.W prefix promotes operation to 64 bits.
See the summary chart at the beginning of this section for encoding data and limits.IA-32 Architecture CompatibilityThis instruction is not supported on Intel processors earlier than the Intel486 processors.

## Flags affected

- The ZF flag is set if the values in the destination operand and register AL, AX, or EAX are equal; otherwise it is cleared. The CF, PF, AF, SF, and OF flags are set according to the results of the comparison operation.

## Exceptions

- Protected Mode Exceptions
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
  - #UD - If the LOCK prefix is used but th
  > e destination is not a memory operand.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used but th
  > e destination is not a memory operand.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
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
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.

## Operation

```C
=(* Accumulator  AL, AX, EAX, or RAX depending on whether a byte, word, doubleword, or quadword comparison is being performed *)TEMP := DEST= TEMPIF accumulator THENZF := 1;DEST := SRC;ELSEZF := 0;accumulator := TEMP;
```

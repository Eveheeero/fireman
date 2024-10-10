# SHLD

Double Precision Shift Left

The SHLD instruction is used for multi-precision shifts of 64 bits or more.The instruction shifts the first operand (destination operand) to the left the number of bits specified by the third operand (count operand).
The second operand (source operand) provides bits to shift in from the right (starting with bit 0 of the destination operand).
The destination operand can be a register or a memory location; the source operand is a register.
The count operand is an unsigned integer that can be stored in an immediate byte or in the CL register.
If the count operand is CL, the shift count is the logical AND of CL and a count mask.
In non-64-bit modes and default 64-bit mode; only bits 0 through 4 of the count are used.
This masks the count to a value between 0 and 31.
If a count is greater than the operand size, the result is undefined.If the count is 1 or greater, the CF flag is filled with the last bit shifted out of the destination operand.
For a 1-bit shift, the OF flag is set if a sign change occurred; otherwise, it is cleared.
If the count operand is 0, flags are not affected.In 64-bit mode, the instruction's default operation size is 32 bits.
Using a REX prefix in the form of REX.R permits access to additional registers (R8-R15).
Using a REX prefix in the form of REX.W promotes operation to 64 bits (upgrading the count mask to 6 bits).
See the summary chart at the beginning of this section for encoding data and limits.

## Flags affected

- If the count is 1 or greater, the CF flag is filled with the last bit shifted out of the destination operand and the SF, ZF, and PF flags are set according to the value of the result. For a 1-bit shift, the OF flag is set if a sign change occurred; otherwise, it is cleared. For shifts greater than 1 bit, the OF flag is undefined. If a shift occurs, the AF flag is undefined. If the count operand is 0, the flags are not affected. If the count is greater than the operand size, the flags are undefined.

## Exceptions

- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used.
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
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.

## Operation

```C
IF (In 64-Bit Mode and REX.W = 1) THEN COUNT := COUNT MOD 64;ELSE COUNT := COUNT MOD 32;FISIZE := OperandSize; 0IF COUNT =THEN IF COUNT > SIZETHEN (* Bad parameters *)DEST is undefined;CF, OF, SF, ZF, AF, PF are undefined;ELSE (* Perform the shift *)CF := BIT[DEST, SIZE - COUNT];(* Last bit shifted out on exit *)FOR i := SIZE - 1 DOWN TO COUNTDOBit(DEST, i) := Bit(DEST, i - COUNT);OD;FOR i := COUNT - 1 DOWN TO 0DOBIT[DEST, i] := BIT[SRC, i - COUNT + SIZE];OD;FI;FI;
```

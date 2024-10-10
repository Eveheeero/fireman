# DIV

Unsigned Divide

Divides unsigned the value in the AX, DX:AX, EDX:EAX, or RDX:RAX registers (dividend) by the source operand (divisor) and stores the result in the AX (AH:AL), DX:AX, EDX:EAX, or RDX:RAX registers.
The source operand can be a general-purpose register or a memory location.
The action of this instruction depends on the operand size (dividend/divisor).
Division using 64-bit operand is available only in 64-bit mode.Non-integral results are truncated (chopped) towards 0.
The remainder is always less than the divisor in magni-tude.
Overflow is indicated with the #DE (divide error) exception rather than with the CF flag.In 64-bit mode, the instruction's default operation size is 32 bits.
Use of the REX.R prefix permits access to addi-tional registers (R8-R15).
Use of the REX.W prefix promotes operation to 64 bits.
In 64-bit mode when REX.W is applied, the instruction divides the unsigned value in RDX:RAX by the source operand and stores the quotient in RAX, the remainder in RDX.
See the summary chart at the beginning of this section for encoding data and limits.
See Table 3-15.Table 3-15.
 DIV Action Maximum Operand SizeDividendDivisorQuotientRemainderQuotientWord/byteAXr/m8ALAH255Doubleword/wordDX:AXr/m16AXDX65,535Quadword/doublewordEDX:EAXr/m32EAXEDX2 - 132Doublequadword/RDX:RAXr/m64RAXRDX2 - 1

## Exceptions

- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #DE - If the source operand (divisor) is 0
  > If the quotient is too large for the designated register.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  - #DE - If the source operand (divisor) is 0.
  > If the quotient is too large for the designated register.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #DE - If the source operand (divisor) is 0
  > If the quotient is too large for the designated register.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #DE - If the source operand (divisor) is 0.
  > If the quotient is too large for the designated register.
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register contains a NULL segment selector.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #UD - If the LOCK prefix is used.

## Operation

```C
 =IF SRC 0THEN #DE; FI; (* Divide Error *) IF OperandSize = 8 (* Word/Byte Operation *)THENtemp := AX / SRC;IF temp > FFHTHEN #DE; (* Divide error *) ELSEAL := temp;AH := AX MOD SRC;FI;= 16 (* Doubleword/word operation *)ELSE IF OperandSize THENtemp := DX:AX / SRC;IF temp > FFFFHTHEN #DE; (* Divide error *) ELSEAX := temp;DX := DX:AX MOD SRC;FI;FI;ELSE IF Operandsize = 32 (* Quadword/doubleword operation *)THENtemp := EDX:EAX / SRC;IF temp > FFFFFFFFHTHEN #DE; (* Divide error *) ELSEEAX := temp;EDX := EDX:EAX MOD SRC;FI;FI;ELSE IF 64-Bit Mode and Operandsize = 64 (* Doublequadword/quadword operation *)THENtemp := RDX:RAX / SRC;IF temp > FFFFFFFFFFFFFFFFHTHEN #DE; (* Divide error *) ELSERAX := temp;RDX := RDX:RAX MOD SRC;FI;FI;FI;
```

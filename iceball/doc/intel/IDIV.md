# IDIV

Signed Divide

Divides the (signed) value in the AX, DX:AX, or EDX:EAX (dividend) by the source operand (divisor) and stores the result in the AX (AH:AL), DX:AX, or EDX:EAX registers.
The source operand can be a general-purpose register or a memory location.
The action of this instruction depends on the operand size (dividend/divisor).Non-integral results are truncated (chopped) towards 0.
The remainder is always less than the divisor in magni-tude.
Overflow is indicated with the #DE (divide error) exception rather than with the CF flag.In 64-bit mode, the instruction's default operation size is 32 bits.
Use of the REX.R prefix permits access to addi-tional registers (R8-R15).
Use of the REX.W prefix promotes operation to 64 bits.
In 64-bit mode when REX.W is applied, the instruction divides the signed value in RDX:RAX by the source operand.
RAX contains a 64-bit quotient; RDX contains a 64-bit remainder.
See the summary chart at the beginning of this section for encoding data and limits.
See Table 3-51.Table 3-51.
 IDIV ResultsOperand SizeDividendDivisorQuotientRemainderQuotient RangeWord/byteAXr/m8ALAH-128 to +127Doubleword/wordDX:AXr/m16AXDX-32,768 to +32,767Quadword/doublewordEDX:EAXr/m32EAXEDX-2 to 2 - 13131 to 2 - 1Doublequadword/ quadwordRDX:RAXr/m64RAXRDX-2

## Flags affected

- The CF, OF, SF, ZF, AF, and PF flags are undefined.

## Exceptions

- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #DE - If the source operand (divisor) is 0
  > If the quotient is too large for the designated register.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Protected Mode Exceptions
  - #DE - If the source operand (divisor) is 0.
  > The signed result (quotient) is too large for the destination.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #DE - If the source operand (divisor) is 0.
  > The signed result (quotient) is too large for the destination.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #DE - If the source operand (divisor) is 0.
  > The signed result (quotient) is too large for the destination.
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
= IF SRC 0THEN #DE; (* Divide error *) FI;=IF OperandSize  8 (* Word/byte operation *)THENtemp := AX / SRC; (* Signed division *)IF (temp > 7FH) or (temp < 80H) (* If a positive result is greater than 7FH or a negative result is less than 80H *)THEN #DE; (* Divide error *) ELSEAL := temp;AH := AX SignedModulus SRC;FI;= 16 (* Doubleword/word operation *)ELSE IF OperandSize THENtemp := DX:AX / SRC; (* Signed division *)IF (temp > 7FFFH) or (temp < 8000H) (* If a positive result is greater than 7FFFH or a negative result is less than 8000H *)THEN#DE; (* Divide error *) ELSEAX := temp;DX := DX:AX SignedModulus SRC;FI;FI;ELSE IF OperandSize = 32 (* Quadword/doubleword operation *)temp := EDX:EAX / SRC; (* Signed division *)IF (temp > 7FFFFFFFH) or (temp < 80000000H) (* If a positive result is greater than 7FFFFFFFH or a negative result is less than 80000000H *)THEN #DE; (* Divide error *) ELSEEAX := temp;EDX := EDXE:AX SignedModulus SRC;FI;FI;ELSE IF OperandSize = 64 (* Doublequadword/quadword operation *)temp := RDX:RAX / SRC; (* Signed division *)IF (temp > 7FFFFFFFFFFFFFFFH) or (temp < 8000000000000000H) (* If a positive result is greater than 7FFFFFFFFFFFFFFFH or a negative result is less than 8000000000000000H *)THEN #DE; (* Divide error *) ELSERAX := temp;RDX := RDE:RAX SignedModulus SRC;FI;
```

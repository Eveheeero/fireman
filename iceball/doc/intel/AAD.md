# AAD

ASCII Adjust AX Before Division

Adjusts two unpacked BCD digits (the least-significant digit in the AL register and the most-significant digit in the AH register) so that a division operation performed on the result will yield a correct unpacked BCD value.
The AAD instruction is only useful when it precedes a DIV instruction that divides (binary division) the adjusted value in the AX register by an unpacked BCD value.The AAD instruction sets the value in the AL register to (AL + (10 * AH)), and then clears the AH register to 00H.
The value in the AX register is then equal to the binary equivalent of the original unpacked two-digit (base 10) number in registers AH andAL.The generalized version of this instruction allows adjustment of two unpacked digits of any number base (see the "Operation" section below), by setting the imm8 byte to the selected number base (for example, 08H for octal, 0AH for decimal, or 0CH for base 12 numbers).
The AAD mnemonic is interpreted by all assemblers to mean adjust ASCII (base 10) values.
To adjust values in another number base, the instruction must be hand coded in machine code (D5 imm8).This instruction executes as described in compatibility mode and legacy mode.
It is not valid in 64-bit mode.

## Flags affected

- The SF, ZF, and PF flags are set according to the resulting binary value in the AL register; the OF, AF, and CF flags are undefined.

## Exceptions

- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  > Same exceptions as protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as protected mode.

## Operation

```C
IF 64-Bit ModeTHEN#UD;ELSEtempAL := AL;tempAH := AH;AL := (tempAL + (tempAH  imm8)) AND FFH; (* imm8 is set to 0AH for the AAD mnemonic.*)AH := 0;FI;The immediate value (imm8) is taken from the second byte of the instruction.
```

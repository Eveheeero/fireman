# AAA

ASCII Adjust After Addition

Adjusts the sum of two unpacked BCD values to create an unpacked BCD result.
The AL register is the implied source and destination operand for this instruction.
The AAA instruction is only useful when it follows an ADD instruction that adds (binary addition) two unpacked BCD values and stores a byte result in the AL register.
The AAA instruction then adjusts the contents of the AL register to contain the correct 1-digit unpacked BCD result.
If the addition produces a decimal carry, the AH register increments by 1, and the CF and AF flags are set.
If there was no decimal carry, the CF and AF flags are cleared and the AH register is unchanged.
In either case, bits 4 through 7 of the AL register are set to 0.This instruction executes as described in compatibility mode and legacy mode.
It is not valid in 64-bit mode.

## Flags affected

- The AF and CF flags are set to 1 if the adjustment results in a decimal carry; otherwise they are set to 0. The OF, SF, ZF, and PF flags are undefined.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as protected mode.
- Real-Address Mode Exceptions
  > Same exceptions as protected mode.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.

## Operation

```C
IF 64-Bit ModeTHEN#UD;ELSE= 1)IF ((AL AND 0FH) > 9) or (AF THENAX := AX + 106H;AF := 1;CF := 1;ELSEAF := 0;CF := 0;FI;AL := AL AND 0FH;FI;
```

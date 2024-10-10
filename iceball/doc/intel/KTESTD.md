# KTESTW/KTESTB/KTESTQ/KTESTD

Packed Bit Test Masks and Set Flags

Performs a bitwise comparison of the bits of the first source operand and corresponding bits in the second source operand.
If the AND operation produces all zeros, the ZF is set else the ZF is clear.
If the bitwise AND operation of the inverted first source operand with the second source operand produces all zeros the CF is set else the CF is clear.
Only the EFLAGS register is updated.Note: In VEX-encoded versions, VEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
KTESTW TEMP[15:0] := SRC2[15:0] AND SRC1[15:0]IF (TEMP[15:0] = = 0)THEN ZF :=1;ELSE ZF := 0;FI;TEMP[15:0] := SRC2[15:0] AND NOT SRC1[15:0]IF (TEMP[15:0] = = 0)THEN CF :=1;ELSE CF := 0;FI;AF := OF := PF := SF := 0;KTESTB TEMP[7:0] := SRC2[7:0] AND SRC1[7:0]IF (TEMP[7:0] = = 0)THEN ZF :=1;ELSE ZF := 0;FI;TEMP[7:0] := SRC2[7:0] AND NOT SRC1[7:0]IF (TEMP[7:0] = = 0)THEN CF :=1;ELSE CF := 0;FI;AF := OF := PF := SF :=KTESTQ TEMP[63:0] := SRC2[63:0] AND SRC1[63:0]IF (TEMP[63:0] = = 0)THEN ZF :=1;ELSE ZF := 0;FI;TEMP[63:0] := SRC2[63:0] AND NOT SRC1[63:0]IF (TEMP[63:0] = = 0)THEN CF :=1;ELSE CF := 0;FI;AF := OF := PF := SF := 0;KTESTD TEMP[31:0] := SRC2[31:0] AND SRC1[31:0]IF (TEMP[31:0] = = 0)THEN ZF :=1;ELSE ZF := 0;FI;TEMP[31:0] := SRC2[31:0] AND NOT SRC1[31:0]IF (TEMP[31:0] = = 0)THEN CF :=1;ELSE CF := 0;FI;AF := OF := PF := SF := 0;Intel C/C++ Compiler Intrinsic Equivalent
```

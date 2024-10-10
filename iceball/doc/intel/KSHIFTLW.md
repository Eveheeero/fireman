# KSHIFTLW/KSHIFTLB/KSHIFTLQ/KSHIFTLD

Shift Left Mask Registers

InstructionMode Feature FlagSupportVEX.L0.66.0F3A.W1 32 /r RRIV/VAVX512FShift left 16 bits in k2 by immediate and write result in k1.KSHIFTLW k1, k2, imm8VEX.L0.66.0F3A.W0 32 /r RRIV/VAVX512DQShift left 8 bits in k2 by immediate and write result in k1.KSHIFTLB k1, k2, imm8VEX.L0.66.0F3A.W1 33 /r RRIV/VAVX512BWShift left 64 bits in k2 by immediate and write result in k1.KSHIFTLQ k1, k2, imm8VEX.L0.66.0F3A.W0 33 /r RRIV/VAVX512BWShift left 32 bits in k2 by immediate and write result in k1.KSHIFTLD k1, k2, imm8Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RRIModRM:reg (w)ModRM:r/m (r, ModRM:[7:6] must be 11b)imm8Shifts 8/16/32/64 bits in the second operand (source operand) left by the count specified in immediate byte and place the least significant 8/16/32/64 bits of the result in the destination operand.
The higher bits of the destina-tion are zero-extended.
The destination is set to zero if the count value is greater than 7 (for byte shift), 15 (for word shift), 31 (for doubleword shift) or 63 (for quadword shift).

## Flags affected

- None.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
KSHIFTLWCOUNT := imm8[7:0]DEST[MAX_KL-1:0] := 0IF COUNT <=15THEN DEST[15:0] := SRC1[15:0] << COUNT;FI;KSHIFTLB  :=imm8[7:0]COUNTDEST[MAX_KL-1:0] := 0IF COUNT <=7THEN DEST[7:0] := SRC1[7:0] << COUNT;FI;KSHIFTLQ := imm8[7:0]COUNTDEST[MAX_KL-1:0] := 0IF COUNT <=63  THEN DEST[63:0]:=KSHIFTLDCOUNT := imm8[7:0]DEST[MAX_KL-1:0] := 0IF COUNT <=31  THEN DEST[31:0]:=SRC1[31:0] << COUNT;FI;Intel C/C++ Compiler Intrinsic EquivalentCompiler auto generates KSHIFTLW when needed.
```

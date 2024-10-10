# KSHIFTRW/KSHIFTRB/KSHIFTRQ/KSHIFTRD

Shift Right Mask Registers

InstructionMode Feature FlagSupportVEX.L0.66.0F3A.W1 30 /r RRIV/VAVX512FShift right 16 bits in k2 by immediate and write result in k1.KSHIFTRW k1, k2, imm8VEX.L0.66.0F3A.W0 30 /r RRIV/VAVX512DQShift right 8 bits in k2 by immediate and write result in k1.KSHIFTRB k1, k2, imm8VEX.L0.66.0F3A.W1 31 /r RRIV/VAVX512BWShift right 64 bits in k2 by immediate and write result in k1.KSHIFTRQ k1, k2, imm8VEX.L0.66.0F3A.W0 31 /r RRIV/VAVX512BWShift right 32 bits in k2 by immediate and write result in k1.KSHIFTRD k1, k2, imm8Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RRIModRM:reg (w)ModRM:r/m (r, ModRM:[7:6] must be 11b)imm8Shifts 8/16/32/64 bits in the second operand (source operand) right by the count specified in immediate and place the least significant 8/16/32/64 bits of the result in the destination operand.
The higher bits of the destination are zero-extended.
The destination is set to zero if the count value is greater than 7 (for byte shift), 15 (for word shift), 31 (for doubleword shift) or 63 (for quadword shift).

## Flags affected

- None.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
KSHIFTRWCOUNT := imm8[7:0]DEST[MAX_KL-1:0] := 0IF COUNT <=15THEN DEST[15:0] := SRC1[15:0] >> COUNT;FI;KSHIFTRB := imm8[7:0]COUNTDEST[MAX_KL-1:0] := 0IF COUNT <=7  THEN DEST[7:0]:=SRC1[7:0] >> COUNT;FI;KSHIFTRQCOUNT := imm8[7:0]DEST[MAX_KL-1:0] := 0IF COUNT <=63THEN DEST[63:0]  :=KSHIFTRD  COUNT:=imm8[7:0]DEST[MAX_KL-1:0] := 0IF COUNT <=31 THEN DEST[31:0] :=SRC1[31:0] >> COUNT;FI;Intel C/C++ Compiler Intrinsic EquivalentCompiler auto generates KSHIFTRW when needed.
```

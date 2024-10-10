# KADDW/KADDB/KADDQ/KADDD

ADD Two Masks

InstructionMode Feature FlagSupportVEX.L1.0F.W0 4A /r RVRV/VAVX512DQAdd 16 bits masks in k2 and k3 and place result in k1.KADDW k1, k2, k3VEX.L1.66.0F.W0 4A /r RVRV/VAVX512DQAdd 8 bits masks in k2 and k3 and place result in k1.KADDB k1, k2, k3VEX.L1.0F.W1 4A /r RVRV/VAVX512BWAdd 64 bits masks in k2 and k3 and place result in k1.
KADDQ k1, k2, k3VEX.L1.66.0F.W1 4A /r RVRV/VAVX512BWAdd 32 bits masks in k2 and k3 and place result in k1.KADDD k1, k2, k3Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RVRModRM:reg (w)VEX.1vvv (r)ModRM:r/m (r, ModRM:[7:6] must be 11b)Adds the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.

## Flags affected

- None.

## Exceptions

- Other Exceptions

## Operation

```C
KADDWDEST[15:0] := SRC1[15:0] + SRC2[15:0]DEST[MAX_KL-1:16] := 0KADDBDEST[7:0] := SRC1[7:0] + SRC2[7:0]DEST[MAX_KL-1:8] := 0KADDQDEST[63:0] := SRC1[63:0] + SRC2[63:0]DEST[MAX_KL-1:64] := 0KADDDDEST[31:0] := SRC1[31:0] + SRC2[31:0]DEST[MAX_KL-1:32] := 0Intel C/C++ Compiler Intrinsic EquivalentKADDW __mmask16 _kadd_mask16 (__mmask16 a, __mmask16 b);KADDB __mmask8 _kadd_mask8 (__mmask8 a, __mmask8 b);KADDQ __mmask64 _kadd_mask64 (__mmask64 a, __mmask64 b);KADDD __mmask32 _kadd_mask32 (__mmask32 a, __mmask32 b);
```

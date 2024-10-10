# KORW/KORB/KORQ/KORD

Bitwise Logical OR Masks

InstructionMode Feature FlagSupportVEX.L1.0F.W0 45 /r RVRV/VAVX512FBitwise OR 16 bits masks k2 and k3 and place result in k1.KORW k1, k2, k3VEX.L1.66.0F.W0 45 /r RVRV/VAVX512DQBitwise OR 8 bits masks k2 and k3 and place result in k1.KORB k1, k2, k3VEX.L1.0F.W1 45 /r RVRV/VAVX512BWBitwise OR 64 bits masks k2 and k3 and place result in k1.KORQ k1, k2, k3VEX.L1.66.0F.W1 45 /r RVRV/VAVX512BWBitwise OR 32 bits masks k2 and k3 and place result in k1.KORD k1, k2, k3Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RVRModRM:reg (w)VEX.1vvv (r)ModRM:r/m (r, ModRM:[7:6] must be 11b)Performs a bitwise OR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
KORWDEST[15:0] := SRC1[15:0] BITWISE OR SRC2[15:0]DEST[MAX_KL-1:16] := 0KORBDEST[7:0] := SRC1[7:0] BITWISE OR SRC2[7:0]DEST[MAX_KL-1:8] := 0KORQDEST[63:0] := SRC1[63:0] BITWISE OR SRC2[63:0]DEST[MAX_KL-1:64] := 0KORDDEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]DEST[MAX_KL-1:32] := 0Intel C/C++ Compiler Intrinsic EquivalentKORW __mmask16 _mm512_kor(__mmask16 a, __mmask16 b);
```

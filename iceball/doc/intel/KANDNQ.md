# KANDNW/KANDNB/KANDNQ/KANDND

Bitwise Logical AND NOT Masks

InstructionMode Feature FlagSupportVEX.L1.0F.W0 42 /r RVRV/VAVX512FBitwise AND NOT 16 bits masks k2 and k3 and place result in k1.KANDNW k1, k2, k3VEX.L1.66.0F.W0 42 /r RVRV/VAVX512DQBitwise AND NOT 8 bits masks k1 and k2 and place result in k1.KANDNB k1, k2, k3VEX.L1.0F.W1 42 /r RVRV/VAVX512BWBitwise AND NOT 64 bits masks k2 and k3 and place result in k1.KANDNQ k1, k2, k3VEX.L1.66.0F.W1 42 /r RVRV/VAVX512BWBitwise AND NOT 32 bits masks k2 and k3 and place result in k1.KANDND k1, k2, k3Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RVRModRM:reg (w)VEX.1vvv (r)ModRM:r/m (r, ModRM:[7:6] must be 11b)Performs a bitwise AND NOT between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
KANDNWDEST[15:0] := (BITWISE NOT SRC1[15:0]) BITWISE AND SRC2[15:0]DEST[MAX_KL-1:16] := 0KANDNBDEST[7:0] := (BITWISE NOT SRC1[7:0]) BITWISE AND SRC2[7:0]DEST[MAX_KL-1:8] := 0KANDNQDEST[63:0] := (BITWISE NOT SRC1[63:0]) BITWISE AND SRC2[63:0]DEST[MAX_KL-1:64] := 0KANDNDDEST[31:0] := (BITWISE NOT SRC1[31:0]) BITWISE AND SRC2[31:0]DEST[MAX_KL-1:32] := 0Intel C/C++ Compiler Intrinsic EquivalentKANDNW __mmask16 _mm512_kandn(__mmask16 a, __mmask16 b);
```

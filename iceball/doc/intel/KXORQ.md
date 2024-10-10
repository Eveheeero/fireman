# KXORW/KXORB/KXORQ/KXORD

Bitwise Logical XOR Masks

InstructionMode Feature FlagSupportVEX.L1.0F.W0 47 /r RVRV/VAVX512FBitwise XOR 16-bit masks k2 and k3 and place result in k1.KXORW k1, k2, k3VEX.L1.66.0F.W0 47 /rRVRV/VAVX512DQBitwise XOR 8-bit masks k2 and k3 and place result in k1.KXORB k1, k2, k3VEX.L1.0F.W1 47 /r RVRV/VAVX512BWBitwise XOR 64-bit masks k2 and k3 and place result in k1.KXORQ k1, k2, k3VEX.L1.66.0F.W1 47 /r RVRV/VAVX512BWBitwise XOR 32-bit masks k2 and k3 and place result in k1.KXORD k1, k2, k3Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RVRModRM:reg (w)VEX.1vvv (r)ModRM:r/m (r, ModRM:[7:6] must be 11b)Performs a bitwise XOR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).

## Flags affected

- None.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
KXORWDEST[15:0] := SRC1[15:0] BITWISE XOR SRC2[15:0]DEST[MAX_KL-1:16] := 0KXORBDEST[7:0] := SRC1[7:0] BITWISE XOR SRC2[7:0]DEST[MAX_KL-1:8] := 0KXORQDEST[63:0] := SRC1[63:0] BITWISE XOR SRC2[63:0]DEST[MAX_KL-1:64] := 0KXORDDEST[31:0] := SRC1[31:0] BITWISE XOR SRC2[31:0]DEST[MAX_KL-1:32] := 0Intel C/C++ Compiler Intrinsic EquivalentKXORW __mmask16 _mm512_kxor(__mmask16 a, __mmask16 b);
```

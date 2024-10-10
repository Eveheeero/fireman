# KXNORW/KXNORB/KXNORQ/KXNORD

Bitwise Logical XNOR Masks

InstructionMode Feature FlagSupportVEX.L1.0F.W0 46 /r RVRV/VAVX512FBitwise XNOR 16-bit masks k2 and k3 and place result in k1.KXNORW k1, k2, k3VEX.L1.66.0F.W0 46 /r RVRV/VAVX512DQBitwise XNOR 8-bit masks k2 and k3 and place result in k1.KXNORB k1, k2, k3VEX.L1.0F.W1 46 /r RVRV/VAVX512BWBitwise XNOR 64-bit masks k2 and k3 and place result in k1.KXNORQ k1, k2, k3VEX.L1.66.0F.W1 46 /r RVRV/VAVX512BWBitwise XNOR 32-bit masks k2 and k3 and place result in k1.KXNORD k1, k2, k3Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RVRModRM:reg (w)VEX.1vvv (r)ModRM:r/m (r, ModRM:[7:6] must be 11b)Performs a bitwise XNOR between the vector mask k2 and the vector mask k3, and writes the result into vector mask k1 (three-operand form).

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
KXNORWDEST[15:0] := NOT (SRC1[15:0] BITWISE XOR SRC2[15:0])DEST[MAX_KL-1:16] := 0KXNORBDEST[7:0] := NOT (SRC1[7:0] BITWISE XOR SRC2[7:0])DEST[MAX_KL-1:8] := 0KXNORQDEST[63:0] := NOT (SRC1[63:0] BITWISE XOR SRC2[63:0])DEST[MAX_KL-1:64] := 0KXNORDDEST[31:0] := NOT (SRC1[31:0] BITWISE XOR SRC2[31:0])DEST[MAX_KL-1:32] := 0Intel C/C++ Compiler Intrinsic EquivalentKXNORW __mmask16 _mm512_kxnor(__mmask16 a, __mmask16 b);
```

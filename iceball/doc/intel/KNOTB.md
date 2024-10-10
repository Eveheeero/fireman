# KNOTW/KNOTB/KNOTQ/KNOTD

NOT Mask Register

InstructionMode Feature FlagSupportVEX.L0.0F.W0 44 /r RRV/VAVX512FBitwise NOT of 16 bits mask k2.KNOTW k1, k2VEX.L0.66.0F.W0 44 /r RRV/VAVX512DQBitwise NOT of 8 bits mask k2.KNOTB k1, k2VEX.L0.0F.W1 44 /r RRV/VAVX512BWBitwise NOT of 64 bits mask k2.KNOTQ k1, k2VEX.L0.66.0F.W1 44 /r RRV/VAVX512BWBitwise NOT of 32 bits mask k2.KNOTD k1, k2Instruction Operand EncodingOp/EnOperand 1Operand 2RRModRM:reg (w)ModRM:r/m (r, ModRM:[7:6] must be 11b)Performs a bitwise NOT of vector mask k2 and writes the result into vector mask k1.

## Flags affected

- None.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
KNOTWDEST[15:0] := BITWISE NOT SRC[15:0]DEST[MAX_KL-1:16] := 0KNOTBDEST[7:0] := BITWISE NOT SRC[7:0]DEST[MAX_KL-1:8] := 0KNOTQDEST[63:0] := BITWISE NOT SRC[63:0]DEST[MAX_KL-1:64] := 0KNOTDDEST[31:0] := BITWISE NOT SRC[31:0]DEST[MAX_KL-1:32] := 0Intel C/C++ Compiler Intrinsic EquivalentKNOTW __mmask16 _mm512_knot(__mmask16 a);
```

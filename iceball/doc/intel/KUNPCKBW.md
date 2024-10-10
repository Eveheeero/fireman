# KUNPCKBW/KUNPCKWD/KUNPCKDQ

Unpack for Mask Registers

InstructionMode Feature FlagSupportVEX.L1.66.0F.W0 4B /rRVRV/VAVX512FUnpack 8-bit masks in k2 and k3 and write word result in k1.KUNPCKBW k1, k2, k3VEX.L1.0F.W0 4B /r RVRV/VAVX512BWUnpack 16-bit masks in k2 and k3 and write doubleword result KUNPCKWD k1, k2, k3in k1.VEX.L1.0F.W1 4B /r RVRV/VAVX512BWUnpack 32-bit masks in k2 and k3 and write quadword result in KUNPCKDQ k1, k2, k3k1.Instruction Operand EncodingOp/EnOperand 1Operand 2Operand 3RVRModRM:reg (w)VEX.1vvv (r)ModRM:r/m (r, ModRM:[7:6] must be 11b)Unpacks the lower 8/16/32 bits of the second and third operands (source operands) into the low part of the first operand (destination operand), starting from the low bytes.
The result is zero-extended in the destination.

## Flags affected

- None.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
KUNPCKBWDEST[7:0] := SRC2[7:0]DEST[15:8] := SRC1[7:0]DEST[MAX_KL-1:16] := 0KUNPCKWD  :=SRC2[15:0]DEST[15:0] DEST[31:16] :=SRC1[15:0]DEST[MAX_KL-1:32] := 0KUNPCKDQ SRC2[31:0]DEST[31:0] :=  DEST[63:32]:=SRC1[31:0]DEST[MAX_KL-1:64] := 0Intel C/C++ Compiler Intrinsic EquivalentKUNPCKBW __mmask16 _mm512_kunpackb(__mmask16 a, __mmask16 b);KUNPCKDQ __mmask64 _mm512_kunpackd(__mmask64 a, __mmask64 b);KUNPCKWD __mmask32 _mm512_kunpackw(__mmask32 a, __mmask32 b);
```

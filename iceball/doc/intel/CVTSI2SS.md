# CVTSI2SS

Convert Doubleword Integer to Scalar Single Precision Floating-Point Value

Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the "convert-from" source operand to a single precision floating-point value in the destination operand (first operand).
The "convert-from" source operand can be a general-purpose register or a memory location.
The destination operand is an XMM register.
The result is stored in the low doubleword of the destination operand, and the upper three doublewords are left unchanged.
When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.128-bit Legacy SSE version: In 64-bit mode, Use of the REX.W prefix promotes the instruction to use 64-bit input value.
The "convert-from" source operand (the second operand) is a general-purpose register or memory location.
Bits (MAXVL-1:32) of the corresponding destination register remain unchanged.VEX.128 and EVEX encoded versions: The "convert-from" source operand (the third operand) can be a general-purpose register or a memory location.
The first source and destination operands are XMM registers.
Bits (127:32) of the XMM register destination are copied from corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX encoded version: the converted result in written to the low doubleword element of the destination under the writemask.Software should ensure VCVTSI2SS is encoded with VE

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Precision.

## Operation

```C
VCVTSI2SS (EVEX Encoded Version)IF (SRC2 *is register*) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF 64-Bit Mode And OperandSize = 64THENDEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:0]);ELSEDEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0]);FI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0VCVTSI2SS (VEX.128 Encoded Version)IF 64-Bit Mode And OperandSize = 64THENDEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:0]);ELSEDEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0]);FI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0CVTSI2SS (128-bit Legacy SSE Version)IF 64-Bit Mode And OperandSize = 64THENDEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:0]);ELSEDEST[31:0] :=Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0]);FI;DEST[MAXVL-1:32] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVCVTSI2SS __m128 _mm_cvti32_ss(__m128 s, int a);VCVTSI2SS __m128 _mm_cvt_roundi32_ss(__m128 s, int a, int r);VCVTSI2SS __m128 _mm_cvti64_ss(__m128 s, __int64 a);VCVTSI2SS __m128 _mm_cvt_roundi64_ss(__m128 s, __int64 a, int r);CVTSI2SS __m128 _mm_cvtsi64_ss(__m128 s, __int64 a);CVTSI2SS __m128 _mm_cvtsi32_ss(__m128 a, int b);
```

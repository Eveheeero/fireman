# CVTSI2SD

Convert Doubleword Integer to Scalar Double Precision Floating-Point Value

Converts a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the "convert-from" source operand to a double precision floating-point value in the destination operand.
The result is stored in the low quadword of the destination operand, and the high quadword left unchanged.
When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.The second source operand can be a general-purpose register or a 32/64-bit memory location.
The first source and destination operands are XMM registers.
128-bit Legacy SSE version: Use of the REX.W prefix promotes the instruction to 64-bit operands.
The "convert-from" source operand (the second operand) is a general-purpose register or memory location.
The destination is an XMM register Bits (MAXVL-1:64) of the corresponding destination register remain unchanged.VEX.128 and EVEX encoded versions: The "convert-from" source operand (the third operand) can be a general-purpose register or a memory location.
The first source and destination operands are XMM registers.
Bits (127:64) of the XMM register destination are copied from the corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX.W0 version: attempt to encode this instruction with EVEX embedded rounding is ignored.VEX.W1 and EVEX.W1 versions: promotes the instruction to use 64-bit input value in 64-bit mode.

## Exceptions

- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 3 Class Exception Conditions," if W1; else see Table2-22, "Type 
  > 5 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-48, "Type E3NF Cla
- SIMD Floating-Point Exceptions
  > Precision.

## Operation

```C
VCVTSI2SD (EVEX Encoded Version)IF (SRC2 *is register*) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF 64-Bit Mode And OperandSize = 64THENDEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC2[63:0]);ELSEDEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC2[31:0]);FI;DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0VCVTSI2SD (VEX.128 Encoded Version)IF 64-Bit Mode And OperandSize = 64THENDEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC2[63:0]);ELSEDEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC2[31:0]);FI;DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0CVTSI2SD IF 64-Bit Mode And OperandSize = 64THENDEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[63:0]);ELSEDEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0]);FI;DEST[MAXVL-1:64] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVCVTSI2SD __m128d _mm_cvti32_sd(__m128d s, int a);VCVTSI2SD __m128d _mm_cvti64_sd(__m128d s, __int64 a);VCVTSI2SD __m128d _mm_cvt_roundi64_sd(__m128d s, __int64 a, int r);CVTSI2SD __m128d _mm_cvtsi64_sd(__m128d s, __int64 a);CVTSI2SD __m128d_mm_cvtsi32_sd(__m128d a, int b)
```

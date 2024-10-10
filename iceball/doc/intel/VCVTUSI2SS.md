# VCVTUSI2SS

Convert Unsigned Integer to Scalar Single Precision Floating-Point Value

Converts a unsigned doubleword integer (or unsigned quadword integer if operand size is 64 bits) in the source operand (second operand) to a single precision floating-point value in the destination operand (first operand).
The source operand can be a general-purpose register or a memory location.
The destination operand is an XMM register.
The result is stored in the low doubleword of the destination operand.
When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.The second source operand can be a general-purpose register or a 32/64-bit memory location.
The first source and destination operands are XMM registers.
Bits (127:32) of the XMM register destination are copied from corre-sponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX.W1 version: promotes the instruction to use 64-bit input value in 64-bit mode.

## Exceptions

- SIMD Floating-Point Exceptions
  > Precision.

## Operation

```C
VCVTUSI2SS (EVEX Encoded Version)IF (SRC2 *is register*) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF 64-Bit Mode And OperandSize = 64THENDEST[31:0] := Convert_UInteger_To_Single_Precision_Floating_Point(SRC[63:0]);ELSEDEST[31:0] := Convert_UInteger_To_Single_Precision_Floating_Point(SRC[31:0]);FI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTUSI2SS __m128 _mm_cvtu32_ss( __m128 s, unsigned a);VCVTUSI2SS __m128 _mm_cvt_roundu32_ss( __m128 s, unsigned a, int r);VCVTUSI2SS __m128 _mm_cvtu64_ss( __m128 s, unsigned __int64 a);
```

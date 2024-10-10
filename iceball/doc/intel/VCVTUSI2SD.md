# VCVTUSI2SD

Convert Unsigned Integer to Scalar Double Precision Floating-Point Value

Converts an unsigned doubleword integer (or unsigned quadword integer if operand size is 64 bits) in the second source operand to a double precision floating-point value in the destination operand.
The result is stored in the low quadword of the destination operand.
When conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.The second source operand can be a general-purpose register or a 32/64-bit memory location.
The first source and destination operands are XMM registers.
Bits (127:64) of the XMM register destination are copied from corre-sponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.EVEX.W1 version: promotes the instruction to use 64-bit input value in 64-bit mode.EVEX.W0 version: attempt to encode this instruction with EVEX embedded rounding is ignored.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Precision.

## Operation

```C
VCVTUSI2SD (EVEX Encoded Version)IF (SRC2 *is register*) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF 64-Bit Mode And OperandSize = 64THENDEST[63:0] := Convert_UInteger_To_Double_Precision_Floating_Point(SRC2[63:0]);ELSEDEST[63:0] := Convert_UInteger_To_Double_Precision_Floating_Point(SRC2[31:0]);FI;Intel C/C++ Compiler Intrinsic EquivalentVCVTUSI2SD __m128d _mm_cvtu32_sd( __m128d s, unsigned a); VCVTUSI2SD __m128d _mm_cvtu64_sd( __m128d s, unsigned __int64 a);VCVTUSI2SD __m128d _mm_cvt_roundu64_sd( __m128d s, unsigned __int64 a, int r);
```

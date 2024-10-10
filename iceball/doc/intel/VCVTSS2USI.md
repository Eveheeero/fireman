# VCVTSS2USI

Convert Scalar Single Precision Floating-Point Value to Unsigned Doubleword Integer

Converts a single precision floating-point value in the source operand (the second operand) to an unsigned double-word integer (or unsigned quadword integer if operand size is 64 bits) in the destination operand (the first operand).
The source operand can be an XMM register or a memory location.
The destination operand is a general-purpose register.
When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination w - 1 is format, the floating-point invalid exception is raised, and if this exception is masked, the integer value 2returned, where w represents the number of bits in the destination format.VEX.W1 and EVEX.W1 versions: promotes the instruction to produce 64-bit data in 64-bit mode.Note: EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTSS2USI (EVEX Encoded Version)IF (SRC *is register*) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;IF 64-bit Mode and OperandSize = 64THENDEST[63:0] := Convert_Single_Precision_Floating_Point_To_UInteger(SRC[31:0]);ELSEDEST[31:0] := Convert_Single_Precision_Floating_Point_To_UInteger(SRC[31:0]);FI;Intel C/C++ Compiler Intrinsic EquivalentVCVTSS2USI unsigned _mm_cvtss_u32( __m128 a);VCVTSS2USI unsigned _mm_cvt_roundss_u32( __m128 a, int r);
```

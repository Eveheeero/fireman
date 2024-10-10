# VCVTTSS2USI

Convert With Truncation Scalar Single Precision Floating-Point Value to Unsigned Integer

Converts with truncation a single precision floating-point value in the source operand (the second operand) to an unsigned doubleword integer (or unsigned quadword integer if operand size is 64 bits) in the destination operand (the first operand).
The source operand can be an XMM register or a memory location.
The destination operand is a general-purpose register.
When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.When a conversion is inexact, a truncated (round toward zero) value is returned.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, w - 1 is returned, where w represents the number of bits in the destination format.the integer value 2EVEX.W1 version: promotes the instruction to produce 64-bit data in 64-bit mode.Note: EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTTSS2USI (EVEX Encoded Version)IF 64-bit Mode and OperandSize = 64THENDEST[63:0] := Convert_Single_Precision_Floating_Point_To_UInteger_Truncate(SRC[31:0]);ELSEDEST[31:0] := Convert_Single_Precision_Floating_Point_To_UInteger_Truncate(SRC[31:0]);FI;Intel C/C++ Compiler Intrinsic EquivalentVCVTTSS2USI unsigned int _mm_cvttss_u32( __m128 a);VCVTTSS2USI unsigned int _mm_cvtt_roundss_u32( __m128 a, int sae);VCVTTSS2USI unsigned __int64 _mm_cvttss_u64( __m128 a);VCVTTSS2USI unsigned __int64 _mm_cvtt_roundss_u64( __m128 a, int sae);
```

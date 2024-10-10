# VCVTTSD2USI

Convert With Truncation Scalar Double Precision Floating-Point Value to Unsigned Integer

Converts with truncation a double precision floating-point value in the source operand (the second operand) to an unsigned doubleword integer (or unsigned quadword integer if operand size is 64 bits) in the destination operand (the first operand).
The source operand can be an XMM register or a 64-bit memory location.
The destination operand is a general-purpose register.
When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.When a conversion is inexact, a truncated (round toward zero) value is returned.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, w - 1 is returned, where w represents the number of bits in the destination format.the integer value 2EVEX.W1 version: promotes the instruction to produce 64-bit data in 64-bit mode.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTTSD2USI (EVEX Encoded Version)IF 64-Bit Mode and OperandSize = 64THENDEST[63:0] := Convert_Double_Precision_Floating_Point_To_UInteger_Truncate(SRC[63:0]);ELSEDEST[31:0] := Convert_Double_Precision_Floating_Point_To_UInteger_Truncate(SRC[63:0]);FIIntel C/C++ Compiler Intrinsic EquivalentVCVTTSD2USI unsigned int _mm_cvttsd_u32(__m128d);VCVTTSD2USI unsigned int _mm_cvtt_roundsd_u32(__m128d, int sae);VCVTTSD2USI unsigned __int64 _mm_cvttsd_u64(__m128d);VCVTTSD2USI unsigned __int64 _mm_cvtt_roundsd_u64(__m128d, int sae);
```

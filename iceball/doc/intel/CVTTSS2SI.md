# CVTTSS2SI

Convert With Truncation Scalar Single Precision Floating-Point Value to Integer

Converts a single precision floating-point value in the source operand (the second operand) to a signed doubleword integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand).
The source operand can be an XMM register or a 32-bit memory location.
The destination operand is a general purpose register.
When the source operand is an XMM register, the single precision floating-point value is contained in the low doubleword of the register.
When a conversion is inexact, a truncated (round toward zero) result is returned.
If a converted result is larger than the maximum signed doubleword integer, the floating-point invalid exception is raised.
If this exception is masked, the indefinite integer value (80000000H or 80000000_00000000H if operand size is 64 bits) is returned.Legacy SSE instructions: In 64-bit mode, Use of the REX.W prefix promotes the instruction to 64-bit operation.
See the summary chart at the beginning of this section for encoding data and limits.VEX.W1 and EVEX.W1 versions: promotes the instruction to produce 64-bit data in 64-bit mode.Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b, otherwise instructions will #UD.Software should ensure VCVTTSS2SI is encoded with VEX.L=0.
Encoding VCVTTSS2SI with VEX.L=1 may 

## Exceptions

- Other Exceptions
  > See Table2-20, "Type 3 Class Exce
  > ption Conditions," additionally:
- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
(V)CVTTSS2SI (All Versions)IF 64-Bit Mode and OperandSize = 64THENDEST[63:0] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[31:0]);ELSEDEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[31:0]);FI;Intel C/C++ Compiler Intrinsic EquivalentVCVTTSS2SI int _mm_cvttss_i32( __m128 a);VCVTTSS2SI int _mm_cvtt_roundss_i32( __m128 a, int sae);VCVTTSS2SI __int64 _mm_cvttss_i64( __m128 a);VCVTTSS2SI __int64 _mm_cvtt_roundss_i64( __m128 a, int sae);CVTTSS2SI int _mm_cvttss_si32( __m128 a);CVTTSS2SI __int64 _mm_cvttss_si64( __m128 a);
```

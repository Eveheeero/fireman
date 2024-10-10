# CVTTSD2SI

Convert With Truncation Scalar Double Precision Floating-Point Value to Signed Integer

Converts a double precision floating-point value in the source operand (the second operand) to a signed double-word integer (or signed quadword integer if operand size is 64 bits) in the destination operand (the first operand).
The source operand can be an XMM register or a 64-bit memory location.
The destination operand is a general purpose register.
When the source operand is an XMM register, the double precision floating-point value is contained in the low quadword of the register.
When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.
If a converted result exceeds the range limits of signed doubleword integer (in non-64-bit modes or 64-bit mode with REX.W/VEX.W/EVEX.W=0), the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value (80000000H) is returned.If a converted result exceeds the range limits of signed quadword integer (in 64-bit mode and REX.W/VEX.W/EVEX.W = 1), the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value (80000000_00000000H) is returned.Legacy SSE instructions: In 64-bit mode, Use of the REX.W prefix promotes the instruction to 64-bit operation.
See the summary chart at the beginning of this section for encoding data and limits.Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b, otherwise instructions will #UD.Software should ensure VCVTTSD2SI is encoded with VEX.L=0.
Encoding VCVTTSD2SI with VEX.L=1 may encounter unpredictable behavior across different processor generations.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 
  > 3 Class Exception Conditions," additionally:

## Operation

```C
(V)CVTTSD2SI (All Versions)IF 64-Bit Mode and OperandSize = 64THENDEST[63:0] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0]);ELSEDEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0]);FI;Intel C/C++ Compiler Intrinsic EquivalentVCVTTSD2SI int _mm_cvttsd_i32( __m128d a);VCVTTSD2SI int _mm_cvtt_roundsd_i32( __m128d a, int sae);VCVTTSD2SI __int64 _mm_cvttsd_i64( __m128d a);VCVTTSD2SI __int64 _mm_cvtt_roundsd_i64( __m128d a, int sae);CVTTSD2SI int _mm_cvttsd_si32( __m128d a);CVTTSD2SI __int64 _mm_cvttsd_si64( __m128d a);
```

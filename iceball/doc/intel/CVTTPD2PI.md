# CVTTPD2PI

Convert With Truncation Packed Double Precision Floating-Point Values to Packed Dword Integers

Converts two packed double precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand).
The source operand can be an XMM register or a 128-bit memory location.
The destination operand is an MMX technology register.
When a conversion is inexact, a truncated (round toward zero) result is returned.
If a converted result is larger than the maximum signed doubleword integer, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value (80000000H) is returned.This instruction causes a transition from x87 FPU to MMX technology operation (that is, the x87 FPU top-of-stack pointer is set to 0 and the x87 FPU tag word is set to all 0s [valid]).
If this instruction is executed while an x87 FPU floating-point exception is pending, the exception is handled before the CVTTPD2PI instruction is executed.In 64-bit mode, use of the REX.R prefix permits this instruction to access additional registers (XMM8-XMM15).

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
- Other Mode Exceptions
  > See Table23-4, "Exception Conditions for Legacy SIMD/M
  > MX Instructions with FP Exception and 16-Byte Align-
  > ®

## Operation

```C
DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer32_Truncate(SRC[63:0]);DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer32_Truncate(SRC[127:64]);Intel C/C++ Compiler Intrinsic EquivalentCVTTPD1PI __m64 _mm_cvttpd_pi32(__m128d a)
```

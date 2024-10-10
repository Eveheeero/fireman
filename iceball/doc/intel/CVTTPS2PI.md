# CVTTPS2PI

Convert With Truncation Packed Single Precision Floating-Point Values to Packed Dword Integers

Converts two packed single precision floating-point values in the source operand (second operand) to two packed signed doubleword integers in the destination operand (first operand).
The source operand can be an XMM register or a 64-bit memory location.
The destination operand is an MMX technology register.
When the source operand is an XMM register, the two single precision floating-point values are contained in the low quadword of the register.When a conversion is inexact, a truncated (round toward zero) result is returned.
If a converted result is larger than the maximum signed doubleword integer, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value (80000000H) is returned.This instruction causes a transition from x87 FPU to MMX technology operation (that is, the x87 FPU top-of-stack pointer is set to 0 and the x87 FPU tag word is set to all 0s [valid]).
If this instruction is executed while an x87 FPU floating-point exception is pending, the exception is handled before the CVTTPS2PI instruction is executed.In 64-bit mode, use of the REX.R prefix permits this instruction to access additional registers (XMM8-XMM15).

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
- Other Exceptions
  > See Table23-5, "Exception Conditions for Legacy SIMD/M
  > MX Instructions with XMM and FP Exception," in the 
  > ®

## Operation

```C
DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[31:0]);DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer_Truncate(SRC[63:32]);Intel C/C++ Compiler Intrinsic EquivalentCVTTPS2PI __m64 _mm_cvttps_pi32(__m128 a)
```

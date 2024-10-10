# CVTPI2PS

Convert Packed Dword Integers to Packed Single Precision Floating-Point Values

Converts two packed signed doubleword integers in the source operand (second operand) to two packed single precision floating-point values in the destination operand (first operand).
The source operand can be an MMX technology register or a 64-bit memory location.
The destination operand is an XMM register.
The results are stored in the low quadword of the destination operand, and the high quadword remains unchanged.
When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register.
This instruction causes a transition from x87 FPU to MMX technology operation (that is, the x87 FPU top-of-stack pointer is set to 0 and the x87 FPU tag word is set to all 0s [valid]).
If this instruction is executed while an x87 FPU floating-point exception is pending, the exception is handled before the CVTPI2PS instruction is executed.In 64-bit mode, use of the REX.R prefix permits this instruction to access additional registers (XMM8-XMM15).

## Exceptions

- SIMD Floating-Point Exceptions
  > Precision.
- Other Exceptions
  > ®
  > See Table23-5, "Exception Conditions for Legacy SIMD/MMX
  >  Instructions with XMM and FP Exception" in the Intel
  >  

## Operation

```C
DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0]);DEST[63:32] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:32]);(* High quadword of destination unchanged *)Intel C/C++ Compiler Intrinsic EquivalentCVTPI2PS __m128 _mm_cvtpi32_ps(__m128 a, __m64 b)
```

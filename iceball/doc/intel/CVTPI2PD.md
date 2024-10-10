# CVTPI2PD

Convert Packed Dword Integers to Packed Double Precision Floating-Point Values

Converts two packed signed doubleword integers in the source operand (second operand) to two packed double precision floating-point values in the destination operand (first operand).
The source operand can be an MMX technology register or a 64-bit memory location.
The destination operand is an XMM register.
In addition, depending on the operand configuration: - For operands xmm, mm: the instruction causes a transition from x87 FPU to MMX technology operation (that is, the x87 FPU top-of-stack pointer is set to 0 and the x87 FPU tag word is set to all 0s [valid]).
If this instruction is executed while an x87 FPU floating-point exception is pending, the exception is handled before the CVTPI2PD instruction is executed.
- For operands xmm, m64: the instruction does not cause a transition to MMX technology and does not take x87 FPU exceptions.In 64-bit mode, use of the REX.R prefix permits this instruction to access additional registers (XMM8-XMM15).

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > See Table23-6, "Exception Conditions for Legacy SIMD/MMX
  >  Instructions with XMM and without FP Exception" in 
  > ®

## Operation

```C
DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0]);DEST[127:64] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[63:32]);Intel C/C++ Compiler Intrinsic EquivalentCVTPI2PD __m128d _mm_cvtpi32_pd(__m64 a)
```

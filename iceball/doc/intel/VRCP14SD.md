# VRCP14SD

Compute Approximate Reciprocal of Scalar Float64 Value

This instruction performs a SIMD computation of the approximate reciprocal of the low double precision floating-point value in the second source operand (the third operand) stores the result in the low quadword element of the destination operand (the first operand) according to the writemask k1.
Bits (127:64) of the XMM register destina-tion are copied from corresponding bits in the first source operand (the second operand).
The maximum relative -14.
The source operand can be an XMM register or a 64-bit memory loca-error for this approximation is less than 2tion.
The destination operand is an XMM register.The VRCP14SD instruction is not affected by the rounding control bits in the MXCSR register.
When a source value is a 0.0, an  with the sign of the source value is returned.
A denormal source value will be treated as zero only in case of DAZ bit set in MXCSR.
Otherwise it is treated correctly (i.e., not as a 0.0).
Underflow results are flushed to zero only in case of FTZ bit set in MXCSR.
Otherwise it will be treated correctly (i.e., correct underflow result is written) with the sign of the operand.
When a source value is a SNaN or QNaN, the SNaN is converted to a QNaN or the source QNaN is returned.
See Table 5-16 for special-case input values.MXCSR exception flags are not affected by this instruction and floating-point exceptions are not reported.A numerically exact implementation of VRCP14xx can be found at:https://software.intel.com/en-us/articles/reference-implementations-for-IA-approximation-instructions-vrcp14-vrsqrt14-vrcp28-vrsqrt28-vexp2.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VRCP14SD (EVEX version)IF k1[0] OR *no writemask*THEN DEST[63:0] := APPROXIMATE(1.0/SRC2[63:0]);ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingDEST[63:0] := 0FI;FI;Intel C/C++ Compiler Intrinsic EquivalentVRCP14SD __m128d _mm_rcp14_sd( __m128d a, __m128d b);VRCP14SD __m128d _mm_mask_rcp14_sd(__m128d s, __mmask8 k, __m128d a, __m128d b);VRCP14SD __m128d _mm_maskz_rcp14_sd( __mmask8 k, __m128d a, __m128d b);
```

# VRSQRT14PD

Compute Approximate Reciprocals of Square Roots of Packed Float64 Values

This instruction performs a SIMD computation of the approximate reciprocals of the square roots of the eight packed double precision floating-point values in the source operand (the second operand) and stores the packed double precision floating-point results in the destination operand (the first operand) according to the writemask.
-14.
The maximum relative error for this approximation is less than 2EVEX.512 encoded version: The source operand can be a ZMM register, a 512-bit memory location, or a 512-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM register, conditionally updated using writemask k1.
EVEX.256 encoded version: The source operand is a YMM register, a 256-bit memory location, or a 256-bit vector broadcasted from a 64-bit memory location.
The destination operand is a YMM register, conditionally updated using writemask k1.
EVEX.128 encoded version: The source operand is a XMM register, a 128-bit memory location, or a 128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a XMM register, conditionally updated using writemask k1.
The VRSQRT14PD instruction is not affected by the rounding control bits in the MXCSR register.
When a source value is a 0.0, an  with the sign of the source value is returned.
When the source operand is an + then +ZERO value is returned.
A denormal source value is treated as zero only if DAZ bit is set in MXCSR.
Otherwise it is treated correctly and performs the approximation with the specified masked response.
When a source value is a negative value (other than 0.0) a floating-point QNaN_indefinite is returned.
When a source value is an SNaN or QNaN, the SNaN is converted to a QNaN or the source QNaN is returned.MXCSR exception flags are not affected by this instruction and floating-point exceptions are not reported.Note: EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.A numerically exact implementation of VRSQRT14xx can be found at https://software.intel.com/en-us/arti-

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VRSQRT14PD (EVEX encoded versions) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b = 1) AND (SRC *is memory*)THEN DEST[i+63:i] := APPROXIMATE(1.0/ SQRT(SRC[63:0]));ELSE DEST[i+63:i] := APPROXIMATE(1.0/ SQRT(SRC[i+63:i]));FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FI;FI;ENDFOR;DEST[MAXVL-1:VL] := 0Table 5-24. VRSQRT14PD Special CasesInput valueResult valueCommentsAny denormalNormalCannot generate overflowX = 2-2nn2X < 0QNaN_IndefiniteIncluding -INFX = -0-INFX = +0+INFX = +INF+0Intel C/C++ Compiler Intrinsic EquivalentVRSQRT14PD __m512d _mm512_rsqrt14_pd( __m512d a);VRSQRT14PD __m512d _mm512_mask_rsqrt14_pd(__m512d s, __mmask8 k, __m512d a);VRSQRT14PD __m512d _mm512_maskz_rsqrt14_pd( __mmask8 k, __m512d a);VRSQRT14PD __m256d _mm256_rsqrt14_pd( __m256d a);VRSQRT14PD __m256d _mm512_mask_rsqrt14_pd(__m256d s, __mmask8 k, __m256d a);VRSQRT14PD __m256d _mm512_maskz_rsqrt14_pd( __mmask8 k, __m256d a);VRSQRT14PD __m128d _mm_rsqrt14_pd( __m128d a);VRSQRT14PD __m128d _mm_mask_rsqrt14_pd(__m128d s, __mmask8 k, __m128d a);VRSQRT14PD __m128d _mm_maskz_rsqrt14_pd( __mmask8 k, __m128d a);
```

# VRSQRT14SS

Compute Approximate Reciprocal of Square Root of Scalar Float32 Value

Computes of the approximate reciprocal of the square root of the scalar single-precision floating-point value in the low doubleword element of the source operand (the second operand) and stores the result in the low doubleword element of the destination operand (the first operand) according to the writemask.
The maximum relative error for -14.
The source operand can be an XMM register or a 32-bit memory location.
The this approximation is less than 2destination operand is an XMM register.
Bits (127:32) of the XMM register destination are copied from corresponding bits in the first source operand.
Bits (MAXVL-1:128) of the destination register are zeroed.The VRSQRT14SS instruction is not affected by the rounding control bits in the MXCSR register.
When a source value is a 0.0, an  with the sign of the source value is returned.
When the source operand is an , zero with the sign of the source value is returned.
A denormal source value is treated as zero only if DAZ bit is set in MXCSR.
Otherwise it is treated correctly and performs the approximation with the specified masked response.
When a source value is a negative value (other than 0.0) a floating-point indefinite is returned.
When a source value is an SNaN or QNaN, the SNaN is converted to a QNaN or the source QNaN is returned.MXCSR exception flags are not affected by this instruction and floating-point exceptions are not reported.A numerically exact implementation of VRSQRT14xx can be found at https://software.intel.com/en-us/arti-cles/reference-implementations-for-IA-approximation-instructions-vrcp14-vrsqrt14-vrcp28-vrsqrt28-vexp2.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VRSQRT14SS (EVEX version)IF k1[0] or *no writemask*THENDEST[31:0] := APPROXIMATE(1.0/ SQRT(SRC2[31:0]))ELSE IF *merging-masking*; merging-maskingTHEN *DEST[31:0] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[31:0] := 0FI;FI;DEST[127:32] := SRC1[127:32]DEST[MAXVL-1:128] Table 5-27. VRSQRT14SS Special CasesInput valueResult valueCommentsAny denormalNormalCannot generate overflow-2nnX = 22X < 0QNaN_IndefiniteIncluding -INFX = -0-INFX = +0+INFX = +INF+0Intel C/C++ Compiler Intrinsic EquivalentVRSQRT14SS __m128 _mm_rsqrt14_ss( __m128 a, __m128 b);VRSQRT14SS __m128 _mm_mask_rsqrt14_ss(__m128 s, __mmask8 k, __m128 a, __m128 b);VRSQRT14SS __m128 _mm_maskz_rsqrt14_ss( __mmask8 k, __m128 a, __m128 b);
```

# MINPD

Minimum of Packed Double Precision Floating-Point Values

Performs a SIMD compare of the packed double precision floating-point values in the first source operand and the second source operand and returns the minimum value for each pair of values to the destination operand.
If the values being compared are both 0.0s (of either sign), the value in the second operand (source operand) is returned.
If a value in the second operand is an SNaN, then SNaN is forwarded unchanged to the destination (that is, a QNaN version of the SNaN is not returned).
If only one value is a NaN (SNaN or QNaN) for this instruction, the second operand (source operand), either a NaN or a valid floating-point value, is written to the result.
If instead of this behavior, it is required that the NaN source operand (from either the first or second operand) be returned, the action of MINPD can be emulated using a sequence of instructions, such as, a comparison followed by AND, ANDN, and OR.
EVEX encoded versions: The first source operand (the second operand) is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: The first source operand is a XMM register.
The second source operand can be a XMM register or a 128-bit memory location.
The destination op128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Invalid (including QNaN Source Operand), Denormal.

## Operation

```C
MIN(SRC1, SRC2){IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;ELSE IF (SRC1 < SRC2) THEN DEST := SRC1;ELSE DEST := SRC2; FI; }VMINPD (EVEX Encoded Version)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THENDEST[i+63:i] := MIN(SRC1[i+63:i], SRC2[63:0])ELSE DEST[i+63:i] := MIN(SRC1[i+63:i], SRC2[i+63:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE  DEST[i+63:i] := 0 ; zeroing-maskingFIFI;ENDFORDEST[MAXVL-1:VL] := 0VMINPD (VEX.256 Encoded Version)DEST[63:0] := MIN(SRC1[63:0], SRC2[63:0])DEST[127:64] := MIN(SRC1[127:64], SRC2[127:64])DEST[191:128] := MIN(SRC1[191:128], SRC2[191:128])DEST[255:192] := MIN(SRC1[255:192], SRC2[255:192])VMINPD (VEX.128 Encoded Version)DEST[63:0] := MIN(SRC1[63:0], SRC2[63:0])DEST[127:64] := MIN(SRC1[127:64], SRC2[127:64])DEST[MAXVL-1:128] := 0MINPD (128-bit Legacy SSE Version)DEST[63:0] := MIN(SRC1[63:0], SRC2[63:0])Intel C/C++ Compiler Intrinsic EquivalentVMINPD __m512d _mm512_min_pd( __m512d a, __m512d b);VMINPD __m512d _mm512_mask_min_pd(__m512d s, __mmask8 k, __m512d a, __m512d b);VMINPD __m512d _mm512_maskz_min_pd( __mmask8 k, __m512d a, __m512d b);VMINPD __m512d _mm512_min_round_pd( __m512d a, __m512d b, int);VMINPD __m512d _mm512_mask_min_round_pd(__m512d s, __mmask8 k, __m512d a, __m512d b, int);VMINPD __m512d _mm512_maskz_min_round_pd( __mmask8 k, __m512d a, __m512d b, int);VMINPD __m256d _mm256_mask_min_pd(__m256d s, __mmask8 k, __m256d a, __m256d b);VMINPD __m256d _mm256_maskz_min_pd( __mmask8 k, __m256d a, __m256d b);VMINPD __m128d _mm_mask_min_pd(__m128d s, __mmask8 k, __m128d a, __m128d b);VMINPD __m128d _mm_maskz_min_pd( __mmask8 k, __m128d a, __m128d b);VMINPD __m256d _mm256_min_pd (__m256d a, __m256d b);MINPD __m128d _mm_min_pd (__m128d a, __m128d b);
```

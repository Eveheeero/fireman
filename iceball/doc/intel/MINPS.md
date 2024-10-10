# MINPS

Minimum of Packed Single Precision Floating-Point Values

Performs a SIMD compare of the packed single precision floating-point values in the first source operand and the second source operand and returns the minimum value for each pair of values to the destination operand.
If the values being compared are both 0.0s (of either sign), the value in the second operand (source operand) is returned.
If a value in the second operand is an SNaN, then SNaN is forwarded unchanged to the destination (that is, a QNaN version of the SNaN is not returned).
If only one value is a NaN (SNaN or QNaN) for this instruction, the second operand (source operand), either a NaN or a valid floating-point value, is written to the result.
If instead of this behavior, it is required that the NaN source operand (from either the first or second operand) be returned, the action of MINPS can be emulated using a sequence of instructions, such as, a comparison followed by AND, ANDN, and OR.
EVEX encoded versions: The first source operand (the second operand) is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: The first source operand is a XMM register.
The second source operand can be a XMM 128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Invalid (including QNaN Source Operand), Denormal.

## Operation

```C
MIN(SRC1, SRC2){IF ((SRC1 = 0.0) and (SRC2 = 0.0)) THEN DEST := SRC2;ELSE IF (SRC1 = NaN) THEN DEST := SRC2; FI;ELSE IF (SRC2 = NaN) THEN DEST := SRC2; FI;ELSE IF (SRC1 < SRC2) THEN DEST := SRC1;ELSE DEST := SRC2; FI; }VMINPS (EVEX Encoded Version)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THENDEST[i+31:i] := MIN(SRC1[i+31:i], SRC2[31:0])ELSE DEST[i+31:i] := MIN(SRC1[i+31:i], SRC2[i+31:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE  DEST[i+31:i] := 0 ; zeroing-maskingFIFI;ENDFORDEST[MAXVL-1:VL] := 0VMINPS (VEX.256 Encoded Version)DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])DEST[63:32] := MIN(SRC1[63:32], SRC2[63:32])DEST[95:64] := MIN(SRC1[95:64], SRC2[95:64])DEST[127:96] := MIN(SRC1[127:96], SRC2[127:96])DEST[159:128] := MIN(SRC1[159:128], SRC2[159:128])DEST[191:160] := MIN(SRC1[191:160], SRC2[191:160])DEST[223:192] := MIN(SRC1[223:192], SRC2[223:192])DEST[255:224] := MIN(SRC1[255:224], SRC2[255:224])VMINPS (VEX.128 Encoded Version)DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])DEST[63:32] := MIN(SRC1[63:32], SRC2[63:32])DEST[95:64] := MIN(SRC1[95:64], SRC2[95:64])MINPS (128-bit Legacy SSE Version)DEST[31:0] := MIN(SRC1[31:0], SRC2[31:0])DEST[63:32] := MIN(SRC1[63:32], SRC2[63:32])DEST[95:64] := MIN(SRC1[95:64], SRC2[95:64])DEST[127:96] := MIN(SRC1[127:96], SRC2[127:96])DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVMINPS __m512 _mm512_min_ps( __m512 a, __m512 b);VMINPS __m512 _mm512_mask_min_ps(__m512 s, __mmask16 k, __m512 a, __m512 b);VMINPS __m512 _mm512_maskz_min_ps( __mmask16 k, __m512 a, __m512 b);VMINPS __m512 _mm512_min_round_ps( __m512 a, __m512 b, int);VMINPS __m512 _mm512_mask_min_round_ps(__m512 s, __mmask16 k, __m512 a, __m512 b, int);VMINPS __m512 _mm512_maskz_min_round_ps( __mmask16 k, __m512 a, __m512 b, int);VMINPS __m256 _mm256_mask_min_ps(__m256 s, __mmask8 k, __m256 a, __m256 b);VMINPS __m256 _mm256_maskz_min_ps( __mmask8 k, __m256 a, __m25 b);VMINPS __m128 _mm_mask_min_ps(__m128 s, __mmask8 k, __m128 a, __m128 b);VMINPS __m128 _mm_maskz_min_ps( __mmask8 k, __m128 a, __m128 b);VMINPS __m256 _mm256_min_ps (__m256 a, __m256 b);MINPS __m128 _mm_min_ps (__m128 a, __m128 b);
```

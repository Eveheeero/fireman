# SUBPD

Subtract Packed Double Precision Floating-Point Values

Performs a SIMD subtract of the two, four or eight packed double precision floating-point values of the second Source operand from the first Source operand, and stores the packed double precision floating-point results in the destination operand.VEX.128 and EVEX.128 encoded versions: The second source operand is an XMM register or an 128-bit memory location.
The first source operand and destination operands are XMM registers.
Bits (MAXVL-1:128) of the corre-sponding destination register are zeroed.VEX.256 and EVEX.256 encoded versions: The second source operand is an YMM register or an 256-bit memory location.
The first source operand and destination operands are YMM registers.
Bits (MAXVL-1:256) of the corre-sponding destination register are zeroed.EVEX.512 encoded version: The second source operand is a ZMM register, a 512-bit memory location or a 512-bit vector broadcasted from a 64-bit memory location.
The first source operand and destination operands are ZMM registers.
The destination operand is conditionally updated according to the writemask.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM regist

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Overflow, Underflow, Invalid, Precision, Denormal.

## Operation

```C
VSUBPD (EVEX Encoded Versions When SRC2 Operand is a Vector Register)(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL = 512) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := SRC1[i+63:i] - SRC2[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingDEST[63:0] := 0FI;FI;ENDFORDEST[MAXVL-1:VL] := 0VSUBPD (EVEX Encoded Versions When SRC2 Operand is a Memory Source)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask* THENIF (EVEX.b = 1)THEN DEST[i+63:i] := SRC1[i+63:i] - SRC2[63:0];ELSE EST[i+63:i] := SRC1[i+63:i] - SRC2[i+63:i];FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[63:0] remains unchanged*ELSE ; zeroing-maskingDEST[63:0] := 0FI;FI;ENDFORDEST[MAXVL-1:VL] := 0VSUBPD (VEX.256 Encoded Version)DEST[63:0] := SRC1[63:0] - SRC2[63:0]DEST[127:64] := SRC1[127:64] - SRC2[127:64]DEST[191:128] := SRC1[191:128] - SRC2[191:128]VSUBPD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0] - SRC2[63:0]DEST[127:64] := SRC1[127:64] - SRC2[127:64]DEST[MAXVL-1:128] := 0SUBPD (128-bit Legacy SSE Version)DEST[63:0] := DEST[63:0] - SRC[63:0]DEST[127:64] := DEST[127:64] - SRC[127:64]DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVSUBPD __m512d _mm512_sub_pd (__m512d a, __m512d b);VSUBPD __m512d _mm512_mask_sub_pd (__m512d s, __mmask8 k, __m512d a, __m512d b);VSUBPD __m512d _mm512_maskz_sub_pd (__mmask8 k, __m512d a, __m512d b);VSUBPD __m512d _mm512_sub_round_pd (__m512d a, __m512d b, int);VSUBPD __m512d _mm512_mask_sub_round_pd (__m512d s, __mmask8 k, __m512d a, __m512d b, int);VSUBPD __m512d _mm512_maskz_sub_round_pd (__mmask8 k, __m512d a, __m512d b, int);VSUBPD __m256d _mm256_sub_pd (__m256d a, __m256d b);VSUBPD __m256d _mm256_mask_sub_pd (__m256d s, __mmask8 k, __m256d a, __m256d b);VSUBPD __m256d _mm256_maskz_sub_pd (__mmask8 k, __m256d a, __m256d b);SUBPD __m128d _mm_sub_pd (__m128d a, __m128d b);VSUBPD __m128d _mm_mask_sub_pd (__m128d s, __mmask8 k, __m128d a, __m128d b);VSUBPD __m128d _mm_maskz_sub_pd (__mmask8 k, __m128d a, __m128d b);
```

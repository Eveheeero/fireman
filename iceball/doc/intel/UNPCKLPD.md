# UNPCKLPD

Unpack and Interleave Low Packed Double Precision Floating-Point Values

Performs an interleaved unpack of the low double precision floating-point values from the first source operand and the second source operand.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.
When unpacking from a memory operand, an implementation may fetch only the appropriate 64 bits; however, alignment to 16-byte boundary and normal segment checking will still be enforced.VEX.128 encoded version: The first source operand is a XMM register.
The second source operand can be a XMM register or a 128-bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM register or a 256-bit memory location.
The destination operand is a YMM register.EVEX.512 encoded version: The first source operand is a ZMM register.
The second source operand is a ZMM register, a 512-bit memory location, or a 512-bit vector broadcasted from a 64-bit memory location.
The destina-tion operand is a ZMM register, conditionally updated using writemask k1.
EVEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register, a 256-bit memory location, or a 256-bit vector broadcasted from a 64-bit memory location.
The destina-tion operand is a YMM register, conditionally updated using writemask k1.
EVEX.128 encoded version: The first source operand is an XMM register.
The second source operand is a XMM register, a 128-bit memory location, or a 128-bit vector 

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VUNPCKLPD (EVEX Encoded Versions When SRC2 is a Register)(KL, VL) = (2, 128), (4, 256), (8, 512)IF VL >= 128TMP_DEST[63:0] := SRC1[63:0]TMP_DEST[127:64] := SRC2[63:0]FI;IF VL >= 256TMP_DEST[191:128] := SRC1[191:128]TMP_DEST[255:192] := SRC2[191:128]FI;IF VL >= 512TMP_DEST[319:256] := SRC1[319:256]TMP_DEST[383:320] := SRC2[319:256]TMP_DEST[447:384] := SRC1[447:384]TMP_DEST[511:448] := SRC2[447:384]FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := TMP_DEST[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+63:i] := 0FIFI;VUNPCKLPD (EVEX Encoded Version When SRC2 is Memory)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF (EVEX.b = 1)THEN TMP_SRC2[i+63:i] := SRC2[63:0]ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]FI;ENDFOR;IF VL >= 128TMP_DEST[63:0] := SRC1[63:0]TMP_DEST[127:64] := TMP_SRC2[63:0]FI;IF VL >= 256TMP_DEST[191:128] := SRC1[191:128]TMP_DEST[255:192] := TMP_SRC2[191:128]FI;IF VL >= 512TMP_DEST[319:256] := SRC1[319:256]TMP_DEST[383:320] := TMP_SRC2[319:256]TMP_DEST[447:384] := SRC1[447:384]TMP_DEST[511:448] := TMP_SRC2[447:384]FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := TMP_DEST[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VUNPCKLPD (VEX.256 Encoded Version)DEST[63:0] := SRC1[63:0]DEST[127:64] := SRC2[63:0]DEST[191:128] := SRC1[191:128]DEST[255:192] := SRC2[191:128]DEST[MAXVL-1:256] := 0VUNPCKLPD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0]DEST[127:64] := SRC2[63:0]DEST[MAXVL-1:128] := 0UNPCKLPD (128-bit Legacy SSE Version)DEST[63:0] := SRC1[63:0]Intel C/C++ Compiler Intrinsic EquivalentVUNPCKLPD __m512d _mm512_unpacklo_pd( __m512d a, __m512d b);VUNPCKLPD __m512d _mm512_mask_unpacklo_pd(__m512d s, __mmask8 k, __m512d a, __m512d b);VUNPCKLPD __m512d _mm512_maskz_unpacklo_pd(__mmask8 k, __m512d a, __m512d b);VUNPCKLPD __m256d _mm256_unpacklo_pd(__m256d a, __m256d b)VUNPCKLPD __m256d _mm256_mask_unpacklo_pd(__m256d s, __mmask8 k, __m256d a, __m256d b);VUNPCKLPD __m256d _mm256_maskz_unpacklo_pd(__mmask8 k, __m256d a, __m256d b);UNPCKLPD __m128d _mm_unpacklo_pd(__m128d a, __m128d b)VUNPCKLPD __m128d _mm_mask_unpacklo_pd(__m128d s, __mmask8 k, __m128d a, __m128d b);VUNPCKLPD __m128d _mm_maskz_unpacklo_pd(__mmask8 k, __m128d a, __m128d b);
```

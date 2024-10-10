# ANDPD

Bitwise Logical AND of Packed Double Precision Floating-Point Values

Performs a bitwise logical AND of the two, four or eight packed double precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operand.EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: The first source operand is an XMM register.
The second source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM regist

## Exceptions

- Other Exceptions

## Operation

```C
VANDPD (EVEX Encoded Versions) (KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THENIF (EVEX.b == 1) AND (SRC2 *is memory*)THENDEST[i+63:i] := SRC1[i+63:i] BITWISE AND SRC2[63:0]ELSE DEST[i+63:i] := SRC1[i+63:i] BITWISE AND SRC2[i+63:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] = 0FI;FI;ENDFORDEST[MAXVL-1:VL] := 0VANDPD (VEX.256 Encoded Version)DEST[63:0] := SRC1[63:0] BITWISE AND SRC2[63:0]DEST[127:64] := SRC1[127:64] BITWISE AND SRC2[127:64]DEST[191:128] := SRC1[191:128] BITWISE AND SRC2[191:128]DEST[255:192] := SRC1[255:192] BITWISE AND SRC2[255:192]DEST[MAXVL-1:256] := 0VANDPD (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0] BITWISE AND SRC2[63:0]DEST[127:64] := SRC1[127:64] BITWISE AND SRC2[127:64]DEST[MAXVL-1:128] := 0ANDPD (128-bit Legacy SSE Version)DEST[63:0] := DEST[63:0] BITWISE AND SRC[63:0]DEST[127:64] := DEST[127:64] BITWISE AND SRC[127:64]DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVANDPD __m512d _mm512_and_pd (__m512d a, __m512d b);VANDPD __m512d _mm512_mask_and_pd (__m512d s, __mmask8 k, __m512d a, __m512d b);VANDPD __m512d _mm512_maskz_and_pd (__mmask8 k, __m512d a, __m512d b);VANDPD __m256d _mm256_mask_and_pd (__m256d s, __mmask8 k, __m256d a, __m256d b);VANDPD __m256d _mm256_maskz_and_pd (__mmask8 k, __m256d a, __m256d b);VANDPD __m128d _mm_mask_and_pd (__m128d s, __mmask8 k, __m128d a, __m128d b);VANDPD __m128d _mm_maskz_and_pd (__mmask8 k, __m128d a, __m128d b);VANDPD __m256d _mm256_and_pd (__m256d a, __m256d b);ANDPD __m128d _mm_and_pd (__m128d a, __m128d b);
```

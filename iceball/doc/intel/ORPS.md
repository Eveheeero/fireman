# ORPS

Bitwise Logical OR of Packed Single Precision Floating-Point Values

Performs a bitwise logical OR of the four, eight or sixteen packed single precision floating-point values from the first source operand and the second source operand, and stores the result in the destination operandEVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with write-mask k1.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: The first source operand is an XMM register.
The second source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation is not distinct from the first source XMM regist

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
VORPS (EVEX Encoded Versions)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b == 1) AND (SRC2 *is memory*)THENDEST[i+31:i] := SRC1[i+31:i] BITWISE OR SRC2[31:0]ELSE DEST[i+31:i] := SRC1[i+31:i] BITWISE OR SRC2[i+31:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VORPS (VEX.256 Encoded Version)DEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]DEST[63:32] := SRC1[63:32] BITWISE OR SRC2[63:32]DEST[95:64] := SRC1[95:64] BITWISE OR SRC2[95:64]DEST[127:96] := SRC1[127:96] BITWISE OR SRC2[127:96]DEST[159:128] := SRC1[159:128] BITWISE OR SRC2[159:128]DEST[191:160] := SRC1[191:160] BITWISE OR SRC2[191:160]DEST[223:192] := SRC1[223:192] BITWISE OR SRC2[223:192]DEST[255:224] := SRC1[255:224] BITWISE OR SRC2[255:224].DEST[MAXVL-1:256] := 0VORPS (VEX.128 Encoded Version)DEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]DEST[63:32] := SRC1[63:32] BITWISE OR SRC2[63:32]DEST[95:64] := SRC1[95:64] BITWISE OR SRC2[95:64]DEST[127:96] := SRC1[127:96] BITWISE OR SRC2[127:96]DEST[MAXVL-1:128] := 0ORPS (128-bit Legacy SSE Version)DEST[31:0] := SRC1[31:0] BITWISE OR SRC2[31:0]DEST[63:32] := SRC1[63:32] BITWISE OR SRC2[63:32]DEST[95:64] := SRC1[95:64] BITWISE OR SRC2[95:64]Intel C/C++ Compiler Intrinsic EquivalentVORPS __m512 _mm512_or_ps ( __m512 a, __m512 b);VORPS __m512 _mm512_mask_or_ps ( __m512 s, __mmask16 k, __m512 a, __m512 b);VORPS __m512 _mm512_maskz_or_ps (__mmask16 k, __m512 a, __m512 b);VORPS __m256 _mm256_mask_or_ps (__m256 s, ___mmask8 k, __m256 a, __m256 b);VORPS __m256 _mm256_maskz_or_ps (__mmask8 k, __m256 a, __m256 b);VORPS __m128 _mm_mask_or_ps ( __m128 s, __mmask8 k, __m128 a, __m128 b);VORPS __m128 _mm_maskz_or_ps (__mmask8 k, __m128 a, __m128 b);VORPS __m256 _mm256_or_ps (__m256 a, __m256 b);ORPS __m128 _mm_or_ps (__m128 a, __m128 b);
```

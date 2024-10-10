# VPBLENDMD/VPBLENDMQ

Blend Int32/Int64 Vectors Using an OpMask Control

Performs an element-by-element blending of dword/qword elements between the first source operand (the second operand) and the elements of the second source operand (the third operand) using an opmask register as select control.
The blended result is written into the destination.
The destination and first source operands are ZMM registers.
The second source operand can be a ZMM register, a 512-bit memory location or a 512-bit vector broadcasted from a 32-bit memory location.The opmask register is not used as a writemask for this instruction.
Instead, the mask is used as an element selector: every element of the destination is conditionally selected between first source or second source using the value of the related mask bit (0 for the first source operand, 1 for the second source operand).

## Exceptions

- SIMD Floating-Point Exceptions
  > None

## Operation

```C
VPBLENDMD (EVEX encoded versions)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no controlmask*THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THENDEST[i+31:i] := SRC2[31:0]ELSE DEST[i+31:i] := SRC2[i+31:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN DEST[i+31:i] := SRC1[i+31:i]ELSE ; zeroing-maskingDEST[i+31:i] := 0FI;FI;ENDFORDEST[MAXVL-1:VL] := 0;VPBLENDMD (EVEX encoded versions)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no controlmask*THENIF (EVEX.b = 1) AND (SRC2 *is memory*)THENDEST[i+31:i] := SRC2[31:0]ELSE DEST[i+31:i] := SRC2[i+31:i]FI;ELSE IF *merging-masking*; merging-maskingTHEN DEST[i+31:i] := SRC1[i+31:i]ELSE ; zeroing-maskingDEST[i+31:i] := 0FI;FI;Intel C/C++ Compiler Intrinsic EquivalentVPBLENDMD __m512i _mm512_mask_blend_epi32(__mmask16 k, __m512i a, __m512i b);VPBLENDMD __m256i _mm256_mask_blend_epi32(__mmask8 m, __m256i a, __m256i b);VPBLENDMD __m128i _mm_mask_blend_epi32(__mmask8 m, __m128i a, __m128i b);VPBLENDMQ __m512i _mm512_mask_blend_epi64(__mmask8 k, __m512i a, __m512i b);VPBLENDMQ __m256i _mm256_mask_blend_epi64(__mmask8 m, __m256i a, __m256i b);VPBLENDMQ __m128i _mm_mask_blend_epi64(__mmask8 m, __m128i a, __m128i b);
```

# VPERMD/VPERMW

Permute Packed Doubleword/Word Elements

Copies doublewords (or words) from the second source operand (the third operand) to the destination operand (the first operand) according to the indices in the first source operand (the second operand).
Note that this instruc-tion permits a doubleword (word) in the source operand to be copied to more than one location in the destination operand.VEX.256 encoded VPERMD: The first and second operands are YMM registers, the third operand can be a YMM register or memory location.
Bits (MAXVL-1:256) of the corresponding destination register are zeroed.
EVEX encoded VPERMD: The first and second operands are ZMM/YMM registers, the third operand can be a ZMM/YMM register, a 512/256-bit memory location or a 512/256-bit vector broadcasted from a 32-bit memory location.
The elements in the destination are updated using the writemask k1.VPERMW: first and second operands are ZMM/YMM/XMM registers, the third operand can be a ZMM/YMM/XMM register, or a 512/256/128-bit memory location.
The destination is updated using the writemask k1.

## Exceptions

- SIMD Floating-Point Exceptions
  > None
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."
  > EVEX-encoded 
  > VPERMD
  > , see Table2-50, "Type E4NF Class Exception Conditions."
  > EVEX-encoded 
  > VPERMW
  > , see Exceptions Type E4NF.nb in Table2-50, "Type E4NF Class Exception Conditions."
  > Additionally:

## Operation

```C
VPERMD (EVEX encoded versions)(KL, VL) = (8, 256), (16, 512)IF VL = 256 THEN n := 2; FI;IF VL = 512 THEN n := 3; FI;FOR j := 0 TO KL-1i := j * 32id := 32*SRC1[i+n:i]IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN DEST[i+31:i] := SRC2[31:0];ELSE DEST[i+31:i] := SRC2[id+31:id];FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VPERMD (VEX.256 encoded version)DEST[31:0] := (SRC2[255:0] >> (SRC1[2:0] * 32))[31:0];DEST[63:32] := (SRC2[255:0] >> (SRC1[34:32] * 32))[31:0];DEST[95:64] := (SRC2[255:0] >> (SRC1[66:64] * 32))[31:0];DEST[127:96] := (SRC2[255:0] >> (SRC1[98:96] * 32))[31:0];DEST[159:128] := (SRC2[255:0] >> (SRC1[130:128] * 32))[31:0];DEST[191:160] := (SRC2[255:0] >> (SRC1[162:160] * 32))[31:0];DEST[223:192] := (SRC2[255:0] >> (SRC1[194:192] * 32))[31:0];DEST[255:224] := (SRC2[255:0] >> (SRC1[226:224] * 32))[31:0];DEST[MAXVL-1:256] := 0VPERMW (EVEX encoded versions)(KL, VL) = (8, 128), (16, 256), (32, 512)IF VL = 128 THEN n := 2; FI;IF VL = 256 THEN n := 3; FI;IF VL = 512 THEN n := 4; FI;FOR j := 0 TO KL-1i := j * 16id := 16*SRC1[i+n:i]IF k1[j] OR *no writemask*THEN DEST[i+15:i] := SRC2[id+15:id]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+15:i] := 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentVPERMD __m512i _mm512_permutexvar_epi32( __m512i idx, __m512i a);VPERMD __m512i _mm512_mask_permutexvar_epi32(__m512i s, __mmask16 k, __m512i idx, __m512i a);VPERMD __m512i _mm512_maskz_permutexvar_epi32( __mmask16 k, __m512i idx, __m512i a);VPERMD __m256i _mm256_permutexvar_epi32( __m256i idx, __m256i a);VPERMD __m256i _mm256_mask_permutexvar_epi32(__m256i s, __mmask8 k, __m256i idx, __m256i a);VPERMD __m256i _mm256_maskz_permutexvar_epi32( __mmask8 k, __m256i idx, __m256i a);VPERMW __m512i _mm512_permutexvar_epi16( __m512i idx, __m512i a);VPERMW __m512i _mm512_mask_permutexvar_epi16(__m512i s, __mmask32 k, __m512i idx, __m512i a);VPERMW __m512i _mm512_maskz_permutexvar_epi16( __mmask32 k, __m512i idx, __m512i a);VPERMW __m256i _mm256_permutexvar_epi16( __m256i idx, __m256i a);VPERMW __m256i _mm256_mask_permutexvar_epi16(__m256i s, __mmask16 k, __m256i idx, __m256i a);VPERMW __m256i _mm256_maskz_permutexvar_epi16( __mmask16 k, __m256i idx, __m256i a);VPERMW __m128i _mm_permutexvar_epi16( __m128i idx, __m128i a);VPERMW __m128i _mm_mask_permutexvar_epi16(__m128i s, __mmask8 k, __m128i idx, __m128i a);VPERMW __m128i _mm_maskz_permutexvar_epi16( __mmask8 k, __m128i idx, __m128i a);
```

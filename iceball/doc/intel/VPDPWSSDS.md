# VPDPWSSDS

Multiply and Add Signed Word Integers With Saturation

Multiplies the individual signed words of the first source operand by the corresponding signed words of the second source operand, producing intermediate signed, doubleword results.
The adjacent doubleword results are then summed and accumulated in the destination operand.
If the intermediate sum overflows a 32b signed number, the result is saturated to either 0x7FFF_FFFF for po

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2
  > -21, "Type 4 Class Exception Conditions."

## Operation

```C
VPDPWSSDS dest, src1, src2 (VEX encoded versions)VL=(128, 256)KL=VL/32ORIGDEST := DESTFOR i := 0 TO KL-1:p1dword := SIGN_EXTEND(SRC1.word[2*i+0]) * SIGN_EXTEND(SRC2.word[2*i+0])p2dword := SIGN_EXTEND(SRC1.word[2*i+1]) * SIGN_EXTEND(SRC2.word[2*i+1])DEST.dword[i] := SIGNED_DWORD_SATURATE(ORIGDEST.dword[i] + p1dword + p2dword)DEST[MAX_VL-1:VL] := 0VPDPWSSDS dest, src1, src2 (EVEX encoded versions)(KL,VL)=(4,128), (8,256), (16,512)ORIGDEST := DESTFOR i := 0 TO KL-1:IF k1[i] or *no writemask*:IF SRC2 is memory and EVEX.b == 1:t := SRC2.dword[0]ELSE:t := SRC2.dword[i]p1dword := SIGN_EXTEND(SRC1.word[2*i]) * SIGN_EXTEND(t.word[0])p2dword := SIGN_EXTEND(SRC1.word[2*i+1]) * SIGN_EXTEND(t.word[1])DEST.dword[i] := SIGNED_DWORD_SATURATE(ORIGDEST.dword[i] + p1dword + p2dword)ELSE IF *zeroing*:DEST.dword[i] := 0ELSE: // Merge masking, dest element unchangedDEST.dword[i] := ORIGDEST.dword[i]DEST[MAX_VL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPDPWSSDS __m128i _mm_dpwssds_avx_epi32(__m128i, __m128i, __m128i);VPDPWSSDS __m128i _mm_dpwssds_epi32(__m128i, __m128i, __m128i);VPDPWSSDS __m128i _mm_mask_dpwssd_epi32(__m128i, __mmask8, __m128i, __m128i);VPDPWSSDS __m128i _mm_maskz_dpwssd_epi32(__mmask8, __m128i, __m128i, __m128i);VPDPWSSDS __m256i _mm256_dpwssds_avx_epi32(__m256i, __m256i, __m256i);VPDPWSSDS __m256i _mm256_dpwssd_epi32(__m256i, __m256i, __m256i);VPDPWSSDS __m256i _mm256_mask_dpwssd_epi32(__m256i, __mmask8, __m256i, __m256i);VPDPWSSDS __m256i _mm256_maskz_dpwssd_epi32(__mmask8, __m256i, __m256i, __m256i);VPDPWSSDS __m512i _mm512_dpwssd_epi32(__m512i, __m512i, __m512i);VPDPWSSDS __m512i _mm512_mask_dpwssd_epi32(__m512i, __mmask16, __m512i, __m512i);VPDPWSSDS __m512i _mm512_maskz_dpwssd_epi32(__mmask16, __m512i, __m512i, __m512i);
```

# VPDPBUSDS

Multiply and Add Unsigned and Signed Bytes With Saturation

Multiplies the individual unsigned bytes of the first source operand by the corresponding signed bytes of the second source operand, producing intermediate signed word results.
The word results are then summed and accumulated in the destination dword element size operand.
If the intermediate sum overflows a 32b signed number the result is saturated to either 0x7FFF_FFFF for positive numbers of 0x8000_0000 for negative numbers.This instruction supports memory fault suppression.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2
  > -21, "Type 4 Class Exception Conditions."

## Operation

```C
VPDPBUSDS dest, src1, src2 (VEX encoded versions)VL=(128, 256)KL=VL/32// Extending to 16b // src1extend := ZERO_EXTEND// src2extend := SIGN_EXTENDp1word := src1extend(SRC1.byte[4*i+0]) * src2extend(SRC2.byte[4*i+0])p2word := src1extend(SRC1.byte[4*i+1]) * src2extend(SRC2.byte[4*i+1])p3word := src1extend(SRC1.byte[4*i+2]) * src2extend(SRC2.byte[4*i+2])p4word := src1extend(SRC1.byte[4*i+3]) * src2extend(SRC2.byte[4*i+3])DEST.dword[i] := SIGNED_DWORD_SATURATE(ORIGDEST.dword[i] + p1word + p2word + p3word + p4word)DEST[MAX_VL-1:VL] := 0VPDPBUSDS dest, src1, src2 (EVEX encoded versions)(KL,VL)=(4,128), (8,256), (16,512)ORIGDEST := DESTFOR i := 0 TO KL-1:IF k1[i] or *no writemask*:// Byte elements of SRC1 are zero-extended to 16b and// byte elements of SRC2 are sign extended to 16b before multiplication.IF SRC2 is memory and EVEX.b == 1:t := SRC2.dword[0]ELSE:t := SRC2.dword[i]p1word := ZERO_EXTEND(SRC1.byte[4*i]) * SIGN_EXTEND(t.byte[0])p2word := ZERO_EXTEND(SRC1.byte[4*i+1]) * SIGN_EXTEND(t.byte[1])p3word := ZERO_EXTEND(SRC1.byte[4*i+2]) * SIGN_EXTEND(t.byte[2])p4word := ZERO_EXTEND(SRC1.byte[4*i+3]) *SIGN_EXTEND(t.byte[3])DEST.dword[i] := SIGNED_DWORD_SATURATE(ORIGDEST.dword[i] + p1word + p2word + p3word + p4word)ELSE IF *zeroing*:DEST.dword[i] := 0ELSE: // Merge masking, dest element unchangedDEST.dword[i] := ORIGDEST.dword[i]DEST[MAX_VL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPDPBUSDS __m128i _mm_dpbusds_avx_epi32(__m128i, __m128i, __m128i);VPDPBUSDS __m128i _mm_dpbusds_epi32(__m128i, __m128i, __m128i);VPDPBUSDS __m128i _mm_mask_dpbusds_epi32(__m128i, __mmask8, __m128i, __m128i);VPDPBUSDS __m128i _mm_maskz_dpbusds_epi32(__mmask8, __m128i, __m128i, __m128i);VPDPBUSDS __m256i _mm256_dpbusds_avx_epi32(__m256i, __m256i, __m256i);VPDPBUSDS __m256i _mm256_dpbusds_epi32(__m256i, __m256i, __m256i);VPDPBUSDS __m256i _mm256_mask_dpbusds_epi32(__m256i, __mmask8, __m256i, __m256i);VPDPBUSDS __m256i _mm256_maskz_dpbusds_epi32(__mmask8, __m256i, __m256i, __m256i);VPDPBUSDS __m512i _mm512_dpbusds_epi32(__m512i, __m512i, __m512i);VPDPBUSDS __m512i _mm512_mask_dpbusds_epi32(__m512i, __mmask16, __m512i, __m512i);VPDPBUSDS __m512i _mm512_maskz_dpbusds_epi32(__mmask16, __m512i, __m512i, __m512i);
```

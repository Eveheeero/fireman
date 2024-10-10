# VPMULTISHIFTQB

Select Packed Unaligned Bytes From Quadword Sources

This instruction selects eight unaligned bytes from each input qword element of the second source operand (the third operand) and writes eight assembled bytes for each qword element in the destination operand (the first operand).
Each byte result is selected using a byte-granular shift control within the corresponding qword element of the first source operand (the second operand).
Each byte result in the destination operand is updated under the writemask k1.Only the low 6 bits of each control byte are used to select an 8-bit slot to extract the output byte from the qword data in the second source operand.
The starting bit of the 8-bit slot can be unaligned relative to any byte boundary and is extracted from the input qword source at the location specified in the low 6-bit of the control byte.
If the 8-bit slot would exceed the qword boundary, the out-of-bound portion of the 8-bit slot is wrapped back to start from bit 0 of the input qword element.The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM reg-ister, a 512/256/128-bit memory location or a 512/256/12

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPMULTISHIFTQB DEST, SRC1, SRC2 (EVEX encoded version)(KL, VL) = (2, 128),(4, 256), (8, 512)FOR i := 0 TO KL-1IF EVEX.b=1 AND src2 is memory THENtcur := src2.qword[0]; //broadcastingELSEtcur := src2.qword[i];FI;FOR j := 0 to 7 ctrl := src1.qword[i].byte[j] & 63;FOR k := 0 to 7 res.bit[k] := tcur.bit[ (ctrl+k) mod 64 ];ENDFORIF k1[i*8+j] or no writemask THENDEST.qword[i].byte[j] := res;ELSE IF zeroing-masking THENDEST.qword[i].byte[j] := 0;ENDFORENDFORDEST.qword[MAX_VL-1:VL] := 0;Intel C/C++ Compiler Intrinsic EquivalentVPMULTISHIFTQB __m512i _mm512_multishift_epi64_epi8( __m512i a, __m512i b);VPMULTISHIFTQB __m512i _mm512_mask_multishift_epi64_epi8(__m512i s, __mmask64 k, __m512i a, __m512i b);VPMULTISHIFTQB __m512i _mm512_maskz_multishift_epi64_epi8( __mmask64 k, __m512i a, __m512i b);VPMULTISHIFTQB __m256i _mm256_multishift_epi64_epi8( __m256i a, __m256i b);VPMULTISHIFTQB __m256i _mm256_mask_multishift_epi64_epi8(__m256i s, __mmask32 k, __m256i a, __m256i b);VPMULTISHIFTQB __m256i _mm256_maskz_multishift_epi64_epi8( __mmask32 k, __m256i a, __m256i b);VPMULTISHIFTQB __m128i _mm_multishift_epi64_epi8( __m128i a, __m128i b);VPMULTISHIFTQB __m128i _mm_mask_multishift_epi64_epi8(__m128i s, __mmask8 k, __m128i a, __m128i b);VPMULTISHIFTQB __m128i _mm_maskz_multishift_epi64_epi8( __mmask8 k, __m128i a, __m128i b);
```

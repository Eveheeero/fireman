# VPCMPB/VPCMPUB

Compare Packed Byte Values Into Mask

Performs a SIMD compare of the packed byte values in the second source operand and the first source operand and returns the results of the comparison to the mask destination operand.
The comparison predicate operand (imme-diate byte) specifies the type of comparison performed on each pair of packed values in the two source operands.
The result of each comparison is a single mask bit result of 1 (comparison true) or 0 (comparison false).VPCMPB performs a comparison between pairs of signed byte values.VPCMPUB performs a comparison between pairs of unsigned byte values.The first source operand (second operand) is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register or a 512/256/128-bit memory location.
The destination operand (first operand) is a mask register k1.
Up to 64/32/16 comparisonThe comparison predicate operand is an 8-bit immediate: bits 2:0 define the type of comparison to be performed.
Bits 3 through 7 of the immediate are reserved.
Compiler can implement the pseudo-op mnemonic listed in Table 5-11.Table 5-11.
Pseudo-Op and VPCMP* Implementation:Pseudo-OpPCMPM ImplementationVPCMPEQ* reg1, reg2, reg3VPCMP* reg1, reg2, reg3, 0VPCMPLT* reg1, reg2, reg3VPCMP*reg1, reg2, reg3, 1VPCMPLE* reg1, reg2, reg3VPCMP* reg1, reg2, reg3, 2VPCMPNEQ* reg1, reg2, reg3VPCMP* reg1, reg2, reg3, 4VPPCMPNLT* reg1, reg2, reg3VPCMP* reg1, reg2, reg3, 5VPCMPNLE* reg1, reg2, reg3VPCMP* reg1, reg2, reg3, 6

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None

## Operation

```C
CASE (COMPARISON PREDICATE) OF0: OP := EQ; 1: OP := LT; 2: OP := LE; 3: OP := FALSE;4: OP := NEQ;5: OP := NLT; 6: OP := NLE; 7: OP := TRUE;ESAC;VPCMPB (EVEX encoded versions) (KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j * 8IF k2[j] OR *no writemask*THEN CMP := SRC1[i+7:i] OP SRC2[i+7:i];IF CMP = TRUETHEN DEST[j] := 1;ELSE DEST[j] := 0; FI;ELSE DEST[j] = 0; zeroing-masking onlyFI;FI;VPCMPUB (EVEX encoded versions) (KL, VL) = (16, 128), (32, 256), (64, 512)FOR j := 0 TO KL-1i := j * 8IF k2[j] OR *no writemask*THEN CMP := SRC1[i+7:i] OP SRC2[i+7:i];IF CMP = TRUETHEN DEST[j] := 1;ELSE DEST[j] := 0; FI;ELSE DEST[j] = 0; zeroing-masking onlyFI;FI;ENDFORDEST[MAX_KL-1:KL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPCMPB __mmask64 _mm512_cmp_epi8_mask( __m512i a, __m512i b, int cmp);VPCMPB __mmask64 _mm512_mask_cmp_epi8_mask( __mmask64 m, __m512i a, __m512i b, int cmp);VPCMPB __mmask32 _mm256_cmp_epi8_mask( __m256i a, __m256i b, int cmp);VPCMPB __mmask32 _mm256_mask_cmp_epi8_mask( __mmask32 m, __m256i a, __m256i b, int cmp);VPCMPB __mmask16 _mm_cmp_epi8_mask( __m128i a, __m128i b, int cmp);VPCMPB __mmask16 _mm_mask_cmp_epi8_mask( __mmask16 m, __m128i a, __m128i b, int cmp);VPCMPB __mmask64 _mm512_cmp[eq|ge|gt|le|lt|neq]_epi8_mask( __m512i a, __m512i b);VPCMPB __mmask64 _mm512_mask_cmp[eq|ge|gt|le|lt|neq]_epi8_mask( __mmask64 m, __m512i a, __m512i b);VPCMPB __mmask32 _mm256_cmp[eq|ge|gt|le|lt|neq]_epi8_mask( __m256i a, __m256i b);VPCMPB __mmask32 _mm256_mask_cmp[eq|ge|gt|le|lt|neq]_epi8_mask( __mmask32 m, __m256i a, __m256i b);VPCMPB __mmask16 _mm_cmp[eq|ge|gt|le|lt|neq]_epi8_mask( __m128i a, __m128i b);VPCMPB __mmask16 _mm_mask_cmp[eq|ge|gt|le|lt|neq]_epi8_mask( __mmask16 m, __m128i a, __m128i b);VPCMPUB __mmask64 _mm512_cmp_epu8_mask( __m512i a, __m512i b, int cmp);VPCMPUB __mmask64 _mm512_mask_cmp_epu8_mask( __mmask64 m, __m512i a, __m512i b, int cmp);VPCMPUB __mmask32 _mm256_cmp_epu8_mask( __m256i a, __m256i b, int cmp);VPCMPUB __mmask32 _mm256_mask_cmp_epu8_mask( __mmask32 m, __m256i a, __m256i b, int cmp);VPCMPUB __mmask16 _mm_cmp_epu8_mask( __m128i a, __m128i b, int cmp);VPCMPUB __mmask16 _mm_mask_cmp_epu8_mask( __mmask16 m, __m128i a, __m128i b, int cmp);VPCMPUB __mmask64 _mm512_cmp[eq|ge|gt|le|lt|neq]_epu8_mask( __m512i a, __m512i b, int cmp);VPCMPUB __mmask64 _mm512_mask_cmp[eq|ge|gt|le|lt|neq]_epu8_mask( __mmask64 m, __m512i a, __m512i b, int cmp);VPCMPUB __mmask32 _mm256_cmp[eq|ge|gt|le|lt|neq]_epu8_mask( __m256i a, __m256i b, int cmp);VPCMPUB __mmask32 _mm256_mask_cmp[eq|ge|gt|le|lt|neq]_epu8_mask( __mmask32 m, __m256i a, __m256i b, int cmp);VPCMPUB __mmask16 _mm_cmp[eq|ge|gt|le|lt|neq]_epu8_mask( __m128i a, __m128i b, int cmp);VPCMPUB __mmask16 _mm_mask_cmp[eq|ge|gt|le|lt|neq]_epu8_mask( __mmask16 m, __m128i a, __m128i b, int cmp);
```

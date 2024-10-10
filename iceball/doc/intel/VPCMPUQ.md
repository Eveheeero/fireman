# VPCMPQ/VPCMPUQ

Compare Packed Integer Values Into Mask

Performs a SIMD compare of the packed integer values in the second source operand and the first source operand and returns the results of the comparison to the mask destination operand.
The comparison predicate operand (immediate byte) specifies the type of comparison performed on each pair of packed values in the two source oper-ands.
The result of each comparison is a single mask bit result of 1 (comparison true) or 0 (comparison false).VPCMPQ/VPCMPUQ performs a comparison between pairs of signed/unsigned quadword integer values.The first source operand (second operand) is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register or a 512/256/128-bit memory location or a 512-bit vector broadcasted from a 64-bit memory location.
The destination operand (first operand) is a mask register k1.
Up to 8/4/2 comparisons are performed with results written to the destination operand under the writemask k2.The comparison predicate operand is an 8-bit immediate: bits 2:0 define the type of comparison to be performed.
Bits 3 through 7 of the immediate are reserved.
Compiler 

## Exceptions

- SIMD Floating-Point Exceptions
  > None

## Operation

```C
CASE (COMPARISON PREDICATE) OF0: OP := EQ; 1: OP := LT; 2: OP := LE; 3: OP := FALSE;4: OP := NEQ;5: OP := NLT; 6: OP := NLE; 7: OP := TRUE;ESAC;VPCMPQ (EVEX encoded versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k2[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN CMP := SRC1[i+63:i] OP SRC2[63:0];ELSE CMP := SRC1[i+63:i] OP SRC2[i+63:i];FI;IF CMP = TRUETHEN DEST[j] := 1;ELSE DEST[j] := 0; FI;ELSE DEST[j] := 0 ; zeroing-masking onlyFI;ENDFORDEST[MAX_KL-1:KL] := 0VPCMPUQ (EVEX encoded versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k2[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN CMP := SRC1[i+63:i] OP SRC2[63:0];ELSE CMP := SRC1[i+63:i] OP SRC2[i+63:i];FI;IF CMP = TRUETHEN DEST[j] := 1;ELSE DEST[j] := 0; FI;ELSE DEST[j] := 0 ; zeroing-masking onlyFI;Intel C/C++ Compiler Intrinsic EquivalentVPCMPQ __mmask8 _mm512_cmp_epi64_mask( __m512i a, __m512i b, int imm);VPCMPQ __mmask8 _mm512_mask_cmp_epi64_mask(__mmask8 k, __m512i a, __m512i b, int imm);VPCMPQ __mmask8 _mm512_cmp[eq|ge|gt|le|lt|neq]_epi64_mask( __m512i a, __m512i b);VPCMPQ __mmask8 _mm512_mask_cmp[eq|ge|gt|le|lt|neq]_epi64_mask(__mmask8 k, __m512i a, __m512i b);VPCMPUQ __mmask8 _mm512_cmp_epu64_mask( __m512i a, __m512i b, int imm);VPCMPUQ __mmask8 _mm512_mask_cmp_epu64_mask(__mmask8 k, __m512i a, __m512i b, int imm);VPCMPUQ __mmask8 _mm512_cmp[eq|ge|gt|le|lt|neq]_epu64_mask( __m512i a, __m512i b);VPCMPUQ __mmask8 _mm512_mask_cmp[eq|ge|gt|le|lt|neq]_epu64_mask(__mmask8 k, __m512i a, __m512i b);VPCMPQ __mmask8 _mm256_cmp_epi64_mask( __m256i a, __m256i b, int imm);VPCMPQ __mmask8 _mm256_mask_cmp_epi64_mask(__mmask8 k, __m256i a, __m256i b, int imm);VPCMPQ __mmask8 _mm256_cmp[eq|ge|gt|le|lt|neq]_epi64_mask( __m256i a, __m256i b);VPCMPQ __mmask8 _mm256_mask_cmp[eq|ge|gt|le|lt|neq]_epi64_mask(__mmask8 k, __m256i a, __m256i b);VPCMPUQ __mmask8 _mm256_cmp_epu64_mask( __m256i a, __m256i b, int imm);VPCMPUQ __mmask8 _mm256_mask_cmp_epu64_mask(__mmask8 k, __m256i a, __m256i b, int imm);VPCMPUQ __mmask8 _mm256_cmp[eq|ge|gt|le|lt|neq]_epu64_mask( __m256i a, __m256i b);VPCMPUQ __mmask8 _mm256_mask_cmp[eq|ge|gt|le|lt|neq]_epu64_mask(__mmask8 k, __m256i a, __m256i b);VPCMPQ __mmask8 _mm_cmp_epi64_mask( __m128i a, __m128i b, int imm);VPCMPQ __mmask8 _mm_mask_cmp_epi64_mask(__mmask8 k, __m128i a, __m128i b, int imm);VPCMPQ __mmask8 _mm_cmp[eq|ge|gt|le|lt|neq]_epi64_mask( __m128i a, __m128i b);VPCMPQ __mmask8 _mm_mask_cmp[eq|ge|gt|le|lt|neq]_epi64_mask(__mmask8 k, __m128i a, __m128i b);VPCMPUQ __mmask8 _mm_cmp_epu64_mask( __m128i a, __m128i b, int imm);VPCMPUQ __mmask8 _mm_mask_cmp_epu64_mask(__mmask8 k, __m128i a, __m128i b, int imm);VPCMPUQ __mmask8 _mm_cmp[eq|ge|gt|le|lt|neq]_epu64_mask( __m128i a, __m128i b);VPCMPUQ __mmask8 _mm_mask_cmp[eq|ge|gt|le|lt|neq]_epu64_mask(__mmask8 k, __m128i a, __m128i b);
```

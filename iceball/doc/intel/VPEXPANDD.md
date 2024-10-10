# VPEXPANDD

Load Sparse Packed Doubleword Integer Values From Dense Memory/Register

Expand (load) up to 16 contiguous doubleword integer values of the input vector in the source operand (the second operand) to sparse elements in the destination operand (the first operand), selected by the writemask k1.
The destination operand is a ZMM register, the source operand can be a ZMM register or memory location.The input vector starts from the lowest element in the source operand.
The opmask register k1 selects the desti-nation elements (a partial vector or sparse elements if less than 8 elements) to be replaced by the ascending elements in the input vector.
Destination elements not selected by the writemask k1 are either unmodified or zeroed, depending on EVEX.z.Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.Note that the compressed displacement assumes a pre-scaling (N) corresponding to the size of one single element instead of the size of the full vector.

## Exceptions

- Other Exceptions
  > EVEX-encoded instruction, see Exceptions Type E4.nb 
  > in Table2-49, "Type E4 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPEXPANDD (EVEX encoded versions) (KL, VL) = (4, 128), (8, 256), (16, 512)k := 0FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := SRC[k+31:k];k := k + 32ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentVPEXPANDD __m512i _mm512_mask_expandloadu_epi32(__m512i s, __mmask16 k, void * a);VPEXPANDD __m512i _mm512_maskz_expandloadu_epi32( __mmask16 k, void * a);VPEXPANDD __m512i _mm512_mask_expand_epi32(__m512i s, __mmask16 k, __m512i a);VPEXPANDD __m512i _mm512_maskz_expand_epi32( __mmask16 k, __m512i a);VPEXPANDD __m256i _mm256_mask_expandloadu_epi32(__m256i s, __mmask8 k, void * a);VPEXPANDD __m256i _mm256_maskz_expandloadu_epi32( __mmask8 k, void * a);VPEXPANDD __m256i _mm256_mask_expand_epi32(__m256i s, __mmask8 k, __m256i a);VPEXPANDD __m256i _mm256_maskz_expand_epi32( __mmask8 k, __m256i a);VPEXPANDD __m128i _mm_mask_expandloadu_epi32(__m128i s, __mmask8 k, void * a);VPEXPANDD __m128i _mm_maskz_expandloadu_epi32( __mmask8 k, void * a);VPEXPANDD __m128i _mm_mask_expand_epi32(__m128i s, __mmask8 k, __m128i a);VPEXPANDD __m128i _mm_maskz_expand_epi32( __mmask8 k, __m128i a);
```

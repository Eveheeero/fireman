# VPEXPANDQ

Load Sparse Packed Quadword Integer Values From Dense Memory/Register

Expand (load) up to 8 quadword integer values from the source operand (the second operand) to sparse elements in the destination operand (the first operand), selected by the writemask k1.
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
VPEXPANDQ (EVEX encoded versions) (KL, VL) = (2, 128), (4, 256), (8, 512)k := 0FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := SRC[k+63:k];k := k + 64ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[i+63:i] := 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentVPEXPANDQ __m512i _mm512_mask_expandloadu_epi64(__m512i s, __mmask8 k, void * a);VPEXPANDQ __m512i _mm512_maskz_expandloadu_epi64( __mmask8 k, void * a);VPEXPANDQ __m512i _mm512_mask_expand_epi64(__m512i s, __mmask8 k, __m512i a);VPEXPANDQ __m512i _mm512_maskz_expand_epi64( __mmask8 k, __m512i a);VPEXPANDQ __m256i _mm256_mask_expandloadu_epi64(__m256i s, __mmask8 k, void * a);VPEXPANDQ __m256i _mm256_maskz_expandloadu_epi64( __mmask8 k, void * a);VPEXPANDQ __m256i _mm256_mask_expand_epi64(__m256i s, __mmask8 k, __m256i a);VPEXPANDQ __m256i _mm256_maskz_expand_epi64( __mmask8 k, __m256i a);VPEXPANDQ __m128i _mm_mask_expandloadu_epi64(__m128i s, __mmask8 k, void * a);VPEXPANDQ __m128i _mm_maskz_expandloadu_epi64( __mmask8 k, void * a);VPEXPANDQ __m128i _mm_mask_expand_epi64(__m128i s, __mmask8 k, __m128i a);VPEXPANDQ __m128i _mm_maskz_expand_epi64( __mmask8 k, __m128i a);
```

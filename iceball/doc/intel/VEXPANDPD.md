# VEXPANDPD

Load Sparse Packed Double Precision Floating-Point Values From Dense Memory

Expand (load) up to 8/4/2, contiguous, double precision floating-point values of the input vector in the source operand (the second operand) to sparse elements in the destination operand (the first operand) selected by the writemask k1.
The destination operand is a ZMM/YMM/XMM register, the source operand can be a ZMM/YMM/XMM register or a 512/256/128-bit memory location.The input vector starts from the lowest element in the source operand.
The writemask register k1 selects the desti-nation elements (a partial vector or sparse elements if less than 8 elements) to be replaced by the ascending elements in the input vector.
Destination elements not selected by the writemask k1 are either unmodified or zeroed, depending on EVEX.z.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.Note that the compressed displacement assumes a pre-scaling (N) corresponding to the size of one single element instead of the size of the full vector.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > See Exceptions Type E4.nb in Table2-49,
  >  "Type E4 Class Exception Conditions."

## Operation

```C
VEXPANDPD (EVEX Encoded Versions) (KL, VL) = (2, 128), (4, 256), (8, 512)k := 0FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := SRC[k+63:k];k := k + 64ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingTHEN DEST[i+63:i] := 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentVEXPANDPD __m512d _mm512_mask_expand_pd( __m512d s, __mmask8 k, __m512d a);VEXPANDPD __m512d _mm512_maskz_expand_pd( __mmask8 k, __m512d a);VEXPANDPD __m512d _mm512_mask_expandloadu_pd( __m512d s, __mmask8 k, void * a);VEXPANDPD __m512d _mm512_maskz_expandloadu_pd( __mmask8 k, void * a);VEXPANDPD __m256d _mm256_mask_expand_pd( __m256d s, __mmask8 k, __m256d a);VEXPANDPD __m256d _mm256_maskz_expand_pd( __mmask8 k, __m256d a);VEXPANDPD __m256d _mm256_mask_expandloadu_pd( __m256d s, __mmask8 k, void * a);VEXPANDPD __m256d _mm256_maskz_expandloadu_pd( __mmask8 k, void * a);VEXPANDPD __m128d _mm_mask_expand_pd( __m128d s, __mmask8 k, __m128d a);VEXPANDPD __m128d _mm_maskz_expand_pd( __mmask8 k, __m128d a);VEXPANDPD __m128d _mm_mask_expandloadu_pd( __m128d s, __mmask8 k, void * a);VEXPANDPD __m128d _mm_maskz_expandloadu_pd( __mmask8 k, void * a);
```

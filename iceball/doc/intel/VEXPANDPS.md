# VEXPANDPS

Load Sparse Packed Single Precision Floating-Point Values From Dense Memory

Expand (load) up to 16/8/4, contiguous, single precision floating-point values of the input vector in the source operand (the second operand) to sparse elements of the destination operand (the first operand) selected by the writemask k1.
The destination operand is a ZMM/YMM/XMM register, the source operand can be a ZMM/YMM/XMM register or a 512/256/128-bit memory location.The input vector starts from the lowest element in the source operand.
The writemask k1 selects the destination elements (a partial vector or sparse elements if less than 16 elements) to be replaced by the ascending elements in the input vector.
Destination elements not selected by the writemask k1 are either unmodified or zeroed, depending on EVEX.z.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.Note that the compressed displacement assumes a pre-scaling (N) corresponding to the size of one single element instead of the size of the full vector.

## Exceptions

- Other Exceptions
  > See Exceptions Type E4.nb in Table2-49,
  >  "Type E4 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VEXPANDPS (EVEX Encoded Versions) (KL, VL) = (4, 128), (8, 256), (16, 512)k := 0FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := SRC[k+31:k];k := k + 32ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;Intel C/C++ Compiler Intrinsic EquivalentVEXPANDPS __m512 _mm512_mask_expand_ps( __m512 s, __mmask16 k, __m512 a);VEXPANDPS __m512 _mm512_maskz_expand_ps( __mmask16 k, __m512 a);VEXPANDPS __m512 _mm512_mask_expandloadu_ps( __m512 s, __mmask16 k, void * a);VEXPANDPS __m512 _mm512_maskz_expandloadu_ps( __mmask16 k, void * a);VEXPANDPD __m256 _mm256_mask_expand_ps( __m256 s, __mmask8 k, __m256 a);VEXPANDPD __m256 _mm256_maskz_expand_ps( __mmask8 k, __m256 a);VEXPANDPD __m256 _mm256_mask_expandloadu_ps( __m256 s, __mmask8 k, void * a);VEXPANDPD __m256 _mm256_maskz_expandloadu_ps( __mmask8 k, void * a);VEXPANDPD __m128 _mm_mask_expand_ps( __m128 s, __mmask8 k, __m128 a);VEXPANDPD __m128 _mm_maskz_expand_ps( __mmask8 k, __m128 a);VEXPANDPD __m128 _mm_mask_expandloadu_ps( __m128 s, __mmask8 k, void * a);VEXPANDPD __m128 _mm_maskz_expandloadu_ps( __mmask8 k, void * a);
```

# VCOMPRESSPD

Store Sparse Packed Double Precision Floating-Point Values Into Dense Memory

Compress (store) up to 8 double precision floating-point values from the source operand (the second operand) as a contiguous vector to the destination operand (the first operand) The source operand is a ZMM/YMM/XMM register, the destination operand can be a ZMM/YMM/XMM register or a 512/256/128-bit memory location.The opmask register k1 selects the active elements (partial vector or possibly non-contiguous if less than 8 active elements) from the source operand to compress into a contiguous vector.
The contiguous vector is written to the destination starting from the low element of the destination operand.Memory destination version: Only the contiguous vector is written to the destination memory location.
EVEX.z must be zero.Register destination version: If the vector length of the contiguous vector is less than that of the input vector in the source operand, the upper bits of the destination register are unmodified if EVEX.z is not set, otherwise the upper bits are zeroed.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.Note that the compressed displacement assumes a pre-scaling (N) corresponding to the size of one single element instead of the size of the full vector.

## Exceptions

- Other Exceptions
  > EVEX-encoded instructions, see Exceptions Type E4.nb in Table2-49, "Type E4 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VCOMPRESSPD (EVEX Encoded Versions) Store Form(KL, VL) = (2, 128), (4, 256), (8, 512)SIZE := 64k := 0FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[k+SIZE-1:k] := SRC[i+63:i]k := k + SIZEVCOMPRESSPD (EVEX Encoded Versions) Reg-Reg Form(KL, VL) = (2, 128), (4, 256), (8, 512)SIZE := 64k := 0FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[k+SIZE-1:k] := SRC[i+63:i]k := k + SIZEFI;ENDFORIF *merging-masking* THEN *DEST[VL-1:k] remains unchanged*ELSE DEST[VL-1:k] := 0FIDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVCOMPRESSPD __m512d _mm512_mask_compress_pd( __m512d s, __mmask8 k, __m512d a);VCOMPRESSPD __m512d _mm512_maskz_compress_pd( __mmask8 k, __m512d a);VCOMPRESSPD void _mm512_mask_compressstoreu_pd( void * d, __mmask8 k, __m512d a);VCOMPRESSPD __m256d _mm256_mask_compress_pd( __m256d s, __mmask8 k, __m256d a);VCOMPRESSPD __m256d _mm256_maskz_compress_pd( __mmask8 k, __m256d a);VCOMPRESSPD void _mm256_mask_compressstoreu_pd( void * d, __mmask8 k, __m256d a);VCOMPRESSPD __m128d _mm_mask_compress_pd( __m128d s, __mmask8 k, __m128d a);VCOMPRESSPD __m128d _mm_maskz_compress_pd( __mmask8 k, __m128d a);VCOMPRESSPD void _mm_mask_compressstoreu_pd( void * d, __mmask8 k, __m128d a);
```

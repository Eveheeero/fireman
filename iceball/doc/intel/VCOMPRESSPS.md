# VCOMPRESSPS

Store Sparse Packed Single Precision Floating-Point Values Into Dense Memory

Compress (stores) up to 16 single precision floating-point values from the source operand (the second operand) to the destination operand (the first operand).
The source operand is a ZMM/YMM/XMM register, the destination operand can be a ZMM/YMM/XMM register or a 512/256/128-bit memory location.The opmask register k1 selects the active elements (a partial vector or possibly non-contiguous if less than 16 active elements) from the source operand to compress into a contiguous vector.
The contiguous vector is written to the destination starting from the low element of the destination operand.Memory destination version: Only the contiguous vector is written to the destination memory location.
EVEX.z must be zero.Register destination version: If the vector length of the contiguous vector is less than that of the input vector in the source operand, the upper bits of the destination register are unmodified if EVEX.z is not set, otherwise the upper bits are zeroed.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.Note that the compressed displacement assumes a pre-scaling (N) corresponding to the size of one single element instead of the size of the full vector.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > EVEX-encoded instructions, see Exceptions Type E4.nb.
  >  in Table2-49, "Type E4 Class Exception Conditions."

## Operation

```C
VCOMPRESSPS (EVEX Encoded Versions) Store Form(KL, VL) = (4, 128), (8, 256), (16, 512)SIZE := 32k := 0FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[k+SIZE-1:k] := SRC[i+31:i]k := k + SIZE VCOMPRESSPS (EVEX Encoded Versions) Reg-Reg Form(KL, VL) = (4, 128), (8, 256), (16, 512)SIZE := 32k := 0FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[k+SIZE-1:k] := SRC[i+31:i]k := k + SIZEFI;ENDFORIF *merging-masking* THEN *DEST[VL-1:k] remains unchanged*ELSE DEST[VL-1:k] := 0FIDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVCOMPRESSPS __m512 _mm512_mask_compress_ps( __m512 s, __mmask16 k, __m512 a);VCOMPRESSPS __m512 _mm512_maskz_compress_ps( __mmask16 k, __m512 a);VCOMPRESSPS void _mm512_mask_compressstoreu_ps( void * d, __mmask16 k, __m512 a);VCOMPRESSPS __m256 _mm256_mask_compress_ps( __m256 s, __mmask8 k, __m256 a);VCOMPRESSPS __m256 _mm256_maskz_compress_ps( __mmask8 k, __m256 a);VCOMPRESSPS void _mm256_mask_compressstoreu_ps( void * d, __mmask8 k, __m256 a);VCOMPRESSPS __m128 _mm_mask_compress_ps( __m128 s, __mmask8 k, __m128 a);VCOMPRESSPS __m128 _mm_maskz_compress_ps( __mmask8 k, __m128 a);VCOMPRESSPS void _mm_mask_compressstoreu_ps( void * d, __mmask8 k, __m128 a);
```

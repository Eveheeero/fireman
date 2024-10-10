# VPCOMPRESSD

Store Sparse Packed Doubleword Integer Values Into Dense Memory/Register

Compress (store) up to 16/8/4 doubleword integer values from the source operand (second operand) to the desti-nation operand (first operand).
The source operand is a ZMM/YMM/XMM register, the destination operand can be a ZMM/YMM/XMM register or a 512/256/128-bit memory location.The opmask register k1 selects the active elements (partial vector or possibly non-contiguous if less than 16 active elements) from the source operand to compress into a contiguous vector.
The contiguous vector is written to the destination starting from the low element of the destination operand.Memory destination version: Only the contiguous vector is written to the destination memory location.
EVEX.z must be zero.Register destination version: If the vector length of the contiguous vector is less than that of the input vector in the source operand, the upper bits of the destination register are unmodified if EVEX.z is not set, otherwise the upper bits are zeroed.Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.Note that the compressed displacement assumes a pre-scaling (N) corresponding to the size of one single element instead of the size of the full vector.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None

## Operation

```C
VPCOMPRESSD (EVEX encoded versions) store form(KL, VL) = (4, 128), (8, 256), (16, 512)SIZE := 32k := 0FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no controlmask*THEN DEST[k+SIZE-1:k] := SRC[i+31:i]k := k + SIZE VPCOMPRESSD (EVEX encoded versions) reg-reg form(KL, VL) = (4, 128), (8, 256), (16, 512)SIZE := 32k := 0FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no controlmask*THEN DEST[k+SIZE-1:k] := SRC[i+31:i]k := k + SIZEFI;ENDFORIF *merging-masking* THEN *DEST[VL-1:k] remains unchanged*ELSE DEST[VL-1:k] := 0FIDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVPCOMPRESSD __m512i _mm512_mask_compress_epi32(__m512i s, __mmask16 c, __m512i a);VPCOMPRESSD __m512i _mm512_maskz_compress_epi32( __mmask16 c, __m512i a);VPCOMPRESSD void _mm512_mask_compressstoreu_epi32(void * a, __mmask16 c, __m512i s);VPCOMPRESSD __m256i _mm256_mask_compress_epi32(__m256i s, __mmask8 c, __m256i a);VPCOMPRESSD __m256i _mm256_maskz_compress_epi32( __mmask8 c, __m256i a);VPCOMPRESSD void _mm256_mask_compressstoreu_epi32(void * a, __mmask8 c, __m256i s);VPCOMPRESSD __m128i _mm_mask_compress_epi32(__m128i s, __mmask8 c, __m128i a);VPCOMPRESSD __m128i _mm_maskz_compress_epi32( __mmask8 c, __m128i a);VPCOMPRESSD void _mm_mask_compressstoreu_epi32(void * a, __mmask8 c, __m128i s);
```

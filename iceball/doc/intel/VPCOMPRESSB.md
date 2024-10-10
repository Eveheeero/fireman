# VPCOMPRESSB/VCOMPRESSW

Store Sparse Packed Byte/Word Integer Values Into Dense Memory/Register

Compress (stores) up to 64 byte values or 32 word values from the source operand (second operand) to the desti-nation operand (first operand), based on the active elements determined by the writemask operand.
Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.Moves up to 512 bits of packed byte values from the source operand (second operand) to the destination operand (first operand).
This instruction is used to store partial contents of a vector register into a byte vector or single memory location using the active elements in operand writemask.Memory destination version: Only the contiguous vector is written to the destination memory location.
EVEX.z must be zero.Register destination version: If the vector length of the contiguous vector is less than that of the input vector in the source operand, the upper bits of the destination register are unmodified if EVEX.z is not set, otherwise the upper Note that the compressed displacement assumes a pre-scaling (N) corresponding to the size of one single element instead of the size of the full vector.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPCOMPRESSB store form(KL, VL) = (16, 128), (32, 256), (64, 512)k := 0FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:DEST.byte[k] := SRC.byte[j]k := k +1VPCOMPRESSB reg-reg form(KL, VL) = (16, 128), (32, 256), (64, 512)k := 0FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:DEST.byte[k] := SRC.byte[j]k := k + 1IF *merging-masking*:*DEST[VL-1:k*8] remains unchanged*ELSE DEST[VL-1:k*8] := 0DEST[MAX_VL-1:VL] := 0VPCOMPRESSW store form(KL, VL) = (8, 128), (16, 256), (32, 512)k := 0FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:DEST.word[k] := SRC.word[j]k := k + 1VPCOMPRESSW reg-reg form(KL, VL) = (8, 128), (16, 256), (32, 512)k := 0FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:DEST.word[k] := SRC.word[j]k := k + 1IF *merging-masking*:*DEST[VL-1:k*16] remains unchanged*Intel C/C++ Compiler Intrinsic EquivalentVPCOMPRESSB __m128i _mm_mask_compress_epi8(__m128i, __mmask16, __m128i);VPCOMPRESSB __m128i _mm_maskz_compress_epi8(__mmask16, __m128i);VPCOMPRESSB __m256i _mm256_mask_compress_epi8(__m256i, __mmask32, __m256i);VPCOMPRESSB __m256i _mm256_maskz_compress_epi8(__mmask32, __m256i);VPCOMPRESSB __m512i _mm512_mask_compress_epi8(__m512i, __mmask64, __m512i);VPCOMPRESSB __m512i _mm512_maskz_compress_epi8(__mmask64, __m512i);VPCOMPRESSB  void _mm_mask_compressstoreu_epi8(void*, __mmask16, __m128i);VPCOMPRESSB  void _mm256_mask_compressstoreu_epi8(void*, __mmask32, __m256i);VPCOMPRESSB  void _mm512_mask_compressstoreu_epi8(void*, __mmask64, __m512i);VPCOMPRESSW  __m128i  _mm_mask_compress_epi16(__m128i, __mmask8, __m128i);VPCOMPRESSW  __m128i  _mm_maskz_compress_epi16(__mmask8, __m128i); VPCOMPRESSW  __m256i  _mm256_mask_compress_epi16(__m256i, __mmask16, __m256i);VPCOMPRESSW  __m256i  _mm256_maskz_compress_epi16(__mmask16, __m256i);VPCOMPRESSW  __m512i  _mm512_mask_compress_epi16(__m512i, __mmask32, __m512i);VPCOMPRESSW  __m512i  _mm512_maskz_compress_epi16(__mmask32, __m512i);VPCOMPRESSW  void  _mm_mask_compressstoreu_epi16(void*, __mmask8, __m128i);VPCOMPRESSW  void  _mm256_mask_compressstoreu_epi16(void*, __mmask16, __m256i);VPCOMPRESSW  void  _mm512_mask_compressstoreu_epi16(void*, __mmask32, __m512i);
```

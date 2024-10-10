# VCVTNE2PS2BF16

Convert Two Packed Single Data to One Packed BF16 Data

Converts two SIMD registers of packed single data into a single register of packed BF16 data.This instruction does not support memory fault suppression.This instruction uses "Round to nearest (even)" rounding mode.
Output denormals are always flushed to zero and input denormals are always treated as zero.
MXCSR is not consulted nor updated.
No floating-point exceptions are generated.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VCVTNE2PS2BF16 dest, src1, src2VL = (128, 256, 512)KL = VL/16origdest := dest:=FOR i  0 to KL-1:IF k1[ i ] or *no writemask*:IF i < KL/2:IF src2 is memory and evex.b == 1::= t src2.fp32[0]ELSE::= t src2.fp32[ i ]ELSE::= t src1.fp32[ i-KL/2]// See VCVTNEPS2BF16 for definition of convert helper function := dest.word[i] convert_fp32_to_bfloat16(t)ELSE IF *zeroing*::= dest.word[ i ] 0ELSE:  // Merge masking, dest element unchanged:= dest.word[ i ] origdest.word[ i ]:= Intel C/C++ Compiler Intrinsic EquivalentVCVTNE2PS2BF16 __m128bh _mm_cvtne2ps_pbh (__m128, __m128);VCVTNE2PS2BF16 __m128bh _mm_mask_cvtne2ps_pbh (__m128bh, __mmask8, __m128, __m128);VCVTNE2PS2BF16 __m128bh _mm_maskz_cvtne2ps_pbh (__mmask8, __m128, __m128);VCVTNE2PS2BF16 __m256bh _mm256_cvtne2ps_pbh (__m256, __m256);VCVTNE2PS2BF16 __m256bh _mm256_mask_cvtne2ps_pbh (__m256bh, __mmask16, __m256, __m256);VCVTNE2PS2BF16 __m256bh _mm256_maskz_cvtne2ps_ pbh (__mmask16, __m256, __m256);VCVTNE2PS2BF16 __m512bh _mm512_cvtne2ps_pbh (__m512, __m512);VCVTNE2PS2BF16 __m512bh _mm512_mask_cvtne2ps_pbh (__m512bh, __mmask32, __m512, __m512);VCVTNE2PS2BF16 __m512bh _mm512_maskz_cvtne2ps_pbh (__mmask32, __m512, __m512);
```

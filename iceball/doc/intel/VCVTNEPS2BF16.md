# VCVTNEPS2BF16

Convert Packed Single Data to Packed BF16 Data

Converts one SIMD register of packed single data into a single register of packed BF16 data.This instruction uses "Round to nearest (even)" rounding mode.
Output denormals are always flushed to zero and input denormals are always treated as zero.
MXCSR is not consulted nor updated.
As the instruction operand encoding table shows, the EVEX.vvvv field is not used for encoding an operand.
EVEX.vvvv is reserved and must be 0b1111 otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
Define convert_fp32_to_bfloat16(x):IF x is zero or denormal::= x[31] // sign preserving zero (denormal go to zero)dest[15] :=dest[14:0]  0ELSE IF x is infinity::=dest[15:0]  x[31:16]ELSE IF x is NAN::=dest[15:0]  x[31:16] // truncate and set MSB of the mantissa to force QNAN:=dest[6]  1ELSE // normal number:=LSB  x[16]:=rounding_bias  0x00007FFF + LSB:=temp[31:0]  x[31:0] + rounding_bias // integer add:=dest[15:0] VCVTNEPS2BF16 dest, srcVL = (128, 256, 512)KL = VL/16:= destorigdest :=FOR i  0 to KL/2-1:IF k1[ i ] or *no writemask*:IF src is memory and evex.b == 1::= t src.fp32[0]ELSE::= t src.fp32[ i ]:= convert_fp32_to_bfloat16(t)dest.word[i] ELSE IF *zeroing*::= dest.word[ i ] 0ELSE:  // Merge masking, dest element unchanged:= dest.word[ i ] origdest.word[ i ]:= DEST[MAXVL-1:VL/2] 0Intel C/C++ Compiler Intrinsic EquivalentVCVTNEPS2BF16 __m128bh _mm_cvtneps_pbh (__m128);VCVTNEPS2BF16 __m128bh _mm_mask_cvtneps_pbh (__m128bh, __mmask8, __m128);VCVTNEPS2BF16 __m128bh _mm_maskz_cvtneps_pbh (__mmask8, __m128);VCVTNEPS2BF16 __m128bh _mm256_cvtneps_pbh (__m256);VCVTNEPS2BF16 __m128bh _mm256_mask_cvtneps_pbh (__m128bh, __mmask8, __m256);VCVTNEPS2BF16 __m128bh _mm256_maskz_cvtneps_pbh (__mmask8, __m256);VCVTNEPS2BF16 __m256bh _mm512_cvtneps_pbh (__m512);VCVTNEPS2BF16 __m256bh _mm512_mask_cvtneps_pbh (__m256bh, __mmask16, __m512);VCVTNEPS2BF16 __m256bh _mm512_maskz_cvtneps_pbh (__mmask16, __m512);
```

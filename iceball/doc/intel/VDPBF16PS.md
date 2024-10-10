# VDPBF16PS

Dot Product of BF16 Pairs Accumulated Into Packed Single Precision

This instruction performs a SIMD dot-product of two BF16 pairs and accumulates into a packed single precision register."Round to nearest even" rounding mode is used when doing each accumulation of the FMA.
Output denormals are always flushed to zero and input denormals are always treated as zero.
MXCSR is not consulted nor updated.
NaN propagation priorities are described in Table 5-1.
Table 5-1.
 NaN Propagation PrioritiesNaN PriorityDescriptionComments1src1 low is NaNLower part has priority over upper part, i.e., it overrides the upper part.2src2 low is NaN3src1 high is NaNUpper part may be overridden if lower has NaN.4src2 high is NaN5srcdest is NaNDest is propagated if no NaN is encountered by src2.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
Define make_fp32(x):// The x parameter is bfloat16. Pack it in to upper 16b of a dword. The bit pattern is a legal fp32 value. Return that bit pattern.:= 0dword :=dword[31:16] VDPBF16PS srcdest, src1, src2VL = (128, 256, 512)KL = VL/32:= srcdestorigdest := FOR i 0 to KL-1:IF k1[ i ] or *no writemask*:IF src2 is memory and evex.b == 1::=t  src2.dword[0]ELSE::=t  src2.dword[ i ]// FP32 FMA with daz in, ftz out and RNE rounding. MXCSR neither consulted nor updated.srcdest.fp32[ i ] += make_fp32(src1.bfloat16[2*i+1]) * make_fp32(t.bfloat[1])srcdest.fp32[ i ] += make_fp32(src1.bfloat16[2*i+0]) * make_fp32(t.bfloat[0])ELSE IF *zeroing*:srcdest.dword[ i ] := 0ELSE: // merge masking, dest element unchanged:=srcdest.dword[ i ]  origdest.dword[ i ]:= 0srcdest[MAXVL-1:VL] Intel C/C++ Compiler Intrinsic EquivalentVDPBF16PS __m128 _mm_dpbf16_ps(__m128, __m128bh, __m128bh);VDPBF16PS __m128 _mm_mask_dpbf16_ps( __m128, __mmask8, __m128bh, __m128bh);VDPBF16PS __m128 _mm_maskz_dpbf16_ps(__mmask8, __m128, __m128bh, __m128bh);VDPBF16PS __m256 _mm256_dpbf16_ps(__m256, __m256bh, __m256bh);VDPBF16PS __m256 _mm256_mask_dpbf16_ps(__m256, __mmask8, __m256bh, __m256bh);VDPBF16PS __m256 _mm256_maskz_dpbf16_ps(__mmask8, __m256, __m256bh, __m256bh);VDPBF16PS __m512 _mm512_dpbf16_ps(__m512, __m512bh, __m512bh);VDPBF16PS __m512 _mm512_mask_dpbf16_ps(__m512, __mmask16, __m512bh, __m512bh);VDPBF16PS __m512 _mm512_maskz_dpbf16_ps(__mmask16, __m512, __m512bh, __m512bh);
```

# VREDUCEPH

Perform Reduction Transformation on Packed FP16 Values

This instruction performs a reduction transformation of the packed binary encoded FP16 values in the source operand (the second operand) and store the reduced results in binary FP format to the destination operand (the first operand) under the writemask k1.The reduction transformation subtracts the integer part and the leading M fractional bits from the binary FP source value, where M is a unsigned integer specified by imm8[7:4].
Specifically, the reduction transformation can be expressed as:M íM * src)) * 2dest = src  í (ROUND(2Mwhere ROUND() treats src, 2, and their product as binary FP numbers with normalized significand and biased exponents.p * man2, where man2' is the normal-The magnitude of the reduced result can be expressed by considering src = 2ized significand and p' is the unbiased exponent. íM í1.Then if RC=RNE: 0  " |ReducedResult|  " 2 íM RNE: 0  " |ReducedResult| < 2.Then if RC This instruction might end up with a precision exception set.
However, in case of SPE set (i.e., Suppress Precision Exception, which is imm8[3]=1), no precision exception is reported.This instruction may generate tiny non-zero result.
If it does so, it does not report underflow exception, even if Table 5-20.
VREDUCEPH/VREDUCESH Special CasesInput valueRound ModeReturned Value îM î1|Src1| < 2RNESrc1 îM1RU, Src1 > 0Round(Src1 « 2)RU, Src1 ¶ 0Src1 îM|Src1| < 2RD, Src1 · 0Src1 îMRD, Src1 < 0Round(Src1 + 2)NOT RD+0.0Src1 = ±0 orDest = ±0 (Src1  »)RD«0.0Src1 = ±»Any+0.0Src1 = ±NANAnyQNaN (Src1)NOTES:1.
The Round(.) function uses rounding controls specified by (imm8[2]? MXCSR.RC: imm8[1:0]).

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
def reduce_fp16(src, imm8):nan := (src.exp = 0x1F) and (src.fraction != 0)if nan:return QNAN(src)m := imm8[7:4]rc := imm8[1:0]rc_source := imm8[2]spe := imm[3] // suppress precision exceptiontmp := 2^(-m) * ROUND(2^m * src, spe, rc_source, rc)tmp := src - tmp // using same RC, SPE controlsreturn tmpVREDUCEPH dest{k1}, src, imm8 VL = 128, 256 or 512KL := VL/16FOR i := 0 to KL-1:IF k1[i] or *no writemask*:IF SRC is memory and (EVEX.b = 1):tsrc := src.fp16[0]ELSE:tsrc := src.fp16[i]DEST.fp16[i] := reduce_fp16(tsrc, imm8)ELSE IF *zeroing*:DEST.fp16[i] := 0//else DEST.fp16[i]Intel C/C++ Compiler Intrinsic EquivalentVREDUCEPH __m128h _mm_mask_reduce_ph (__m128h src, __mmask8 k, __m128h a, int imm8);VREDUCEPH __m128h _mm_maskz_reduce_ph (__mmask8 k, __m128h a, int imm8);VREDUCEPH __m128h _mm_reduce_ph (__m128h a, int imm8);VREDUCEPH __m256h _mm256_mask_reduce_ph (__m256h src, __mmask16 k, __m256h a, int imm8);VREDUCEPH __m256h _mm256_maskz_reduce_ph (__mmask16 k, __m256h a, int imm8);VREDUCEPH __m256h _mm256_reduce_ph (__m256h a, int imm8);VREDUCEPH __m512h _mm512_mask_reduce_ph (__m512h src, __mmask32 k, __m512h a, int imm8);VREDUCEPH __m512h _mm512_maskz_reduce_ph (__mmask32 k, __m512h a, int imm8);VREDUCEPH __m512h _mm512_reduce_ph (__m512h a, int imm8);VREDUCEPH __m512h _mm512_mask_reduce_round_ph (__m512h src, __mmask32 k, __m512h a, int imm8, const int sae);VREDUCEPH __m512h _mm512_maskz_reduce_round_ph (__mmask32 k, __m512h a, int imm8, const int sae);VREDUCEPH __m512h _mm512_reduce_round_ph (__m512h a, int imm8, const int sae);
```

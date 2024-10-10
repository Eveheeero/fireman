# VGETMANTPS

Extract Float32 Vector of Normalized Mantissas From Float32 Vector

Convert single-precision floating values in the source operand (the second operand) to single-precision floating-point values with the mantissa normalization and sign control specified by the imm8 byte, see Figure5-15.
The converted results are written to the destination operand (the first operand) using writemask k1.
The normalized mantissa is specified by interv (imm8[1:0]) and the sign control (sc) is specified by bits 3:2 of the immediate byte.
The destination operand is a ZMM/YMM/XMM register updated under the writemask.
The source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 32-bit memory location.For each input single-precision floating-point value x, The conversion operation is:GetMant(x) = ±2|x.significand|kwhere:1 <= |x.significand| < 2Unbiased exponent k can be either 0 or -1, depending on the interval range defined by interv, the range of the significand and whether the exponent of the source is even or odd.
The sign of the final result is determined by sc and the source sign.
The encoded value of imm8[1:0] and sign control are shown in Figure5-15.Each converted single-precision floating-point result is encoded according to the sign control, the unbiased expo-nent k (adding bias) and a mantissa normalized to the range specified by interv.The GetMant() function follows Table 5-8 when dealing with floating-point special numbers.This instruction is writemasked, so only those elements with the corresponding bit set in vector mask register k1 are computed and stored into the destination.
Elements in zmm1 with the corresponding bit clear in k1 retain their previous values.

## Exceptions

- SIMD Floating-Point Exceptions
  > Denormal, Invalid.
- Other Exceptions
  > See Table2-46, "Type E2 Class Exception Conditions."

## Operation

```C
def getmant_fp32(src, sign_control, normalization_interval):bias := 127dst.sign := sign_control[0] ? 0 : src.signsigned_one := sign_control[0] ? +1.0 : -1.0dst.exp := src.expdst.fraction := src.fractionzero := (dst.exp = 0) and ((dst.fraction = 0) or (MXCSR.DAZ=1))denormal := (dst.exp = 0) and (dst.fraction != 0) and (MXCSR.DAZ=0)infinity := (dst.exp = 0xFF) and (dst.fraction = 0)nan := (dst.exp = 0xFF) and (dst.fraction != 0)src_signaling := src.fraction[22]snan := nan and (src_signaling = 0)positive := (src.sign = 0)negative := (src.sign = 1)if nan:if snan:MXCSR.IE := 1return qnan(src)if positive and (zero or infinity):return 1.0if negative:if zero:return signed_oneif infinity:if sign_control[1]:MXCSR.IE := 1return QNaN_Indefinitereturn signed_oneif sign_control[1]:MXCSR.IE := 1return QNaN_Indefiniteif denormal:jbit := 0dst.exp := biaswhile jbit = 0:jbit := dst.fraction[22]dst.fraction := dst.fraction << 1dst.exp : = dst.exp - 1MXCSR.DE := 1unbiased_exp := dst.exp - biasodd_exp  := unbiased_exp[0]signaling_bit := dst.fraction[22]if normalization_interval = 0b00:dst.exp := biaselse if normalization_interval = 0b01:dst.exp := odd_exp ? bias-1 : biaselse if normalization_interval = 0b10:dst.exp := bias-1return dstVGETMANTPS (EVEX encoded versions)VGETMANTPS dest{k1}, src, imm8VL = 128, 256, or 512KL := VL / 32sign_control := imm8[3:2] normalization_interval := imm8[1:0]FOR i := 0 to KL-1:IF k1[i] or *no writemask*:IF SRC is memory and (EVEX.b = 1):tsrc := src.float[0]ELSE:tsrc := src.float[i]DEST.float[i] := getmant_fp32(tsrc, sign_control, normalization_interval)ELSE IF *zeroing*:DEST.float[i] := 0//else DEST.float[i] remains unchangedDEST[MAX_VL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVGETMANTPS __m512 _mm512_getmant_ps( __m512 a, enum intv, enum sgn);VGETMANTPS __m512 _mm512_mask_getmant_ps(__m512 s, __mmask16 k, __m512 a, enum intv, enum sgn;VGETMANTPS __m512 _mm512_maskz_getmant_ps(__mmask16 k, __m512 a, enum intv, enum sgn);VGETMANTPS __m512 _mm512_getmant_round_ps( __m512 a, enum intv, enum sgn, int r);VGETMANTPS __m512 _mm512_mask_getmant_round_ps(__m512 s, __mmask16 k, __m512 a, enum intv, enum sgn, int r);VGETMANTPS __m512 _mm512_maskz_getmant_round_ps(__mmask16 k, __m512 a, enum intv, enum sgn, int r);VGETMANTPS __m256 _mm256_getmant_ps( __m256 a, enum intv, enum sgn);VGETMANTPS __m256 _mm256_mask_getmant_ps(__m256 s, __mmask8 k, __m256 a, enum intv, enum sgn);VGETMANTPS __m256 _mm256_maskz_getmant_ps( __mmask8 k, __m256 a, enum intv, enum sgn);VGETMANTPS __m128 _mm_getmant_ps( __m128 a, enum intv, enum sgn);VGETMANTPS __m128 _mm_mask_getmant_ps(__m128 s, __mmask8 k, __m128 a, enum intv, enum sgn);VGETMANTPS __m128 _mm_maskz_getmant_ps( __mmask8 k, __m128 a, enum intv, enum sgn);
```

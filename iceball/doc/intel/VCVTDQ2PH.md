# VCVTDQ2PH

Convert Packed Signed Doubleword Integers to Packed FP16 Values

This instruction converts four, eight, or sixteen packed signed doubleword integers in the source operand to four, eight, or sixteen packed FP16 values in the destination operand.EVEX encoded versions: The source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory loca-tion or a 512/256/128-bit vector broadcast from a 32-bit memory location.
The destination operand is a YMM/XMM register conditionally updated with writemask k1.EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.If the result of the convert operation is overflow and MXCSR.OM=0 then a SIMD exception will be raised with OE=1, PE=1.

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Precision.

## Operation

```C
VCVTDQ2PH DEST, SRC VL = 128, 256 or 512KL := VL / 32IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.dword[0]ELSEtsrc := SRC.dword[j]DEST.fp16[j] := Convert_integer32_to_fp16(tsrc)ELSE IF *zeroing*:DEST[MAXVL-1:VL/2] := 0 Intel C/C++ Compiler Intrinsic EquivalentVCVTDQ2PH __m256h _mm512_cvt_roundepi32_ph (__m512i a, int rounding);VCVTDQ2PH __m256h _mm512_mask_cvt_roundepi32_ph (__m256h src, __mmask16 k, __m512i a, int rounding);VCVTDQ2PH __m256h _mm512_maskz_cvt_roundepi32_ph (__mmask16 k, __m512i a, int rounding);VCVTDQ2PH __m128h _mm_cvtepi32_ph (__m128i a);VCVTDQ2PH __m128h _mm_mask_cvtepi32_ph (__m128h src, __mmask8 k, __m128i a);VCVTDQ2PH __m128h _mm_maskz_cvtepi32_ph (__mmask8 k, __m128i a);VCVTDQ2PH __m128h _mm256_cvtepi32_ph (__m256i a);VCVTDQ2PH __m128h _mm256_mask_cvtepi32_ph (__m128h src, __mmask8 k, __m256i a);VCVTDQ2PH __m128h _mm256_maskz_cvtepi32_ph (__mmask8 k, __m256i a);VCVTDQ2PH __m256h _mm512_cvtepi32_ph (__m512i a);VCVTDQ2PH __m256h _mm512_mask_cvtepi32_ph (__m256h src, __mmask16 k, __m512i a);VCVTDQ2PH __m256h _mm512_maskz_cvtepi32_ph (__mmask16 k, __m512i a);
```

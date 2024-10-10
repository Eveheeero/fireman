# VCVTUQQ2PH

Convert Packed Unsigned Quadword Integers to Packed FP16 Values

This instruction converts packed unsigned quadword integers in the source operand to packed FP16 values in the destination operand.
The destination elements are updated according to the writemask.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.If the result of the convert operation is overflow and MXCSR.OM=0 then a SIMD exception will be raised with OE=1, PE=1.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Overflow, Precision.

## Operation

```C
VCVTUQQ2PH dest, srcVL = 128, 256 or 512KL := VL / 64IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.qword[0]ELSEtsrc := SRC.qword[j]DEST.fp16[j] := Convert_unsigned_integer64_to_fp16(tsrc)ELSE IF *zeroing*:DEST.fp16[j] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTUQQ2PH __m128h _mm512_cvt_roundepu64_ph (__m512i a, int rounding);VCVTUQQ2PH __m128h _mm512_mask_cvt_roundepu64_ph (__m128h src, __mmask8 k, __m512i a, int rounding);VCVTUQQ2PH __m128h _mm512_maskz_cvt_roundepu64_ph (__mmask8 k, __m512i a, int rounding);VCVTUQQ2PH __m128h _mm_cvtepu64_ph (__m128i a);VCVTUQQ2PH __m128h _mm_mask_cvtepu64_ph (__m128h src, __mmask8 k, __m128i a);VCVTUQQ2PH __m128h _mm_maskz_cvtepu64_ph (__mmask8 k, __m128i a);VCVTUQQ2PH __m128h _mm256_cvtepu64_ph (__m256i a);VCVTUQQ2PH __m128h _mm256_mask_cvtepu64_ph (__m128h src, __mmask8 k, __m256i a);VCVTUQQ2PH __m128h _mm256_maskz_cvtepu64_ph (__mmask8 k, __m256i a);VCVTUQQ2PH __m128h _mm512_cvtepu64_ph (__m512i a);VCVTUQQ2PH __m128h _mm512_mask_cvtepu64_ph (__m128h src, __mmask8 k, __m512i a);VCVTUQQ2PH __m128h _mm512_maskz_cvtepu64_ph (__mmask8 k, __m512i a);
```

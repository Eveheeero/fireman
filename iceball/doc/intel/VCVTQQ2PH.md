# VCVTQQ2PH

Convert Packed Signed Quadword Integers to Packed FP16 Values

This instruction converts packed signed quadword integers in the source operand to packed FP16 values in the desti-nation operand.
The destination elements are updated according to the writemask.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.If the result of the convert operation is overflow and MXCSR.OM=0 then a SIMD exception will be raised with OE=1, PE=1.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > Overflow, Precision.

## Operation

```C
VCVTQQ2PH DEST, SRC VL = 128, 256 or 512KL := VL / 64IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.qword[0]ELSEtsrc := SRC.qword[j]DEST.fp16[j] := Convert_integer64_to_fp16(tsrc)ELSE IF *zeroing*:DEST.fp16[j] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTQQ2PH __m128h _mm512_cvt_roundepi64_ph (__m512i a, int rounding);VCVTQQ2PH __m128h _mm512_mask_cvt_roundepi64_ph (__m128h src, __mmask8 k, __m512i a, int rounding);VCVTQQ2PH __m128h _mm512_maskz_cvt_roundepi64_ph (__mmask8 k, __m512i a, int rounding);VCVTQQ2PH __m128h _mm_cvtepi64_ph (__m128i a);VCVTQQ2PH __m128h _mm_mask_cvtepi64_ph (__m128h src, __mmask8 k, __m128i a);VCVTQQ2PH __m128h _mm_maskz_cvtepi64_ph (__mmask8 k, __m128i a);VCVTQQ2PH __m128h _mm256_cvtepi64_ph (__m256i a);VCVTQQ2PH __m128h _mm256_mask_cvtepi64_ph (__m128h src, __mmask8 k, __m256i a);VCVTQQ2PH __m128h _mm256_maskz_cvtepi64_ph (__mmask8 k, __m256i a);VCVTQQ2PH __m128h _mm512_cvtepi64_ph (__m512i a);VCVTQQ2PH __m128h _mm512_mask_cvtepi64_ph (__m128h src, __mmask8 k, __m512i a);VCVTQQ2PH __m128h _mm512_maskz_cvtepi64_ph (__mmask8 k, __m512i a);
```

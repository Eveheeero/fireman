# VCVTUDQ2PH

Convert Packed Unsigned Doubleword Integers to Packed FP16 Values

This instruction converts packed unsigned doubleword integers in the source operand to packed FP16 values in the destination operand.
The destination elements are updated according to the writemask.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.If the result of the convert operation is overflow and MXCSR.OM=0 then a SIMD exception will be raised with OE=1, PE=1.

## Exceptions

- SIMD Floating-Point Exceptions
  > Overflow, Precision.
- Other Exceptions

## Operation

```C
VCVTUDQ2PH dest, srcVL = 128, 256 or 512KL := VL / 32IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.dword[0]ELSEtsrc := SRC.dword[j]DEST.fp16[j] := Convert_unsigned_integer32_to_fp16(tsrc)ELSE IF *zeroing*:DEST.fp16[j] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTUDQ2PH __m256h _mm512_cvt_roundepu32_ph (__m512i a, int rounding);VCVTUDQ2PH __m256h _mm512_mask_cvt_roundepu32_ph (__m256h src, __mmask16 k, __m512i a, int rounding);VCVTUDQ2PH __m256h _mm512_maskz_cvt_roundepu32_ph (__mmask16 k, __m512i a, int rounding);VCVTUDQ2PH __m128h _mm_cvtepu32_ph (__m128i a);VCVTUDQ2PH __m128h _mm_mask_cvtepu32_ph (__m128h src, __mmask8 k, __m128i a);VCVTUDQ2PH __m128h _mm_maskz_cvtepu32_ph (__mmask8 k, __m128i a);VCVTUDQ2PH __m128h _mm256_cvtepu32_ph (__m256i a);VCVTUDQ2PH __m128h _mm256_mask_cvtepu32_ph (__m128h src, __mmask8 k, __m256i a);VCVTUDQ2PH __m128h _mm256_maskz_cvtepu32_ph (__mmask8 k, __m256i a);VCVTUDQ2PH __m256h _mm512_cvtepu32_ph (__m512i a);VCVTUDQ2PH __m256h _mm512_mask_cvtepu32_ph (__m256h src, __mmask16 k, __m512i a);VCVTUDQ2PH __m256h _mm512_maskz_cvtepu32_ph (__mmask16 k, __m512i a);
```

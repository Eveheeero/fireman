# VCVTPD2PH

Convert Packed Double Precision FP Values to Packed FP16 Values

This instruction converts two, four, or eight packed double precision floating-point values in the source operand (second operand) to two, four, or eight packed FP16 values in the destination operand (first operand).
When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.EVEX encoded versions: The source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasts from a 64-bit memory location.
The destination operand is a XMM register conditionally updated with writemask k1.
The upper bits (MAXVL-1:128/64/32) of the corresponding destination are zeroed.EVEX.vvvv are reserved and must be 1111b otherwise instructions will #UD.This instruction uses MXCSR.DAZ for handling FP64 inputs

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal.

## Operation

```C
VCVTPD2PH DEST, SRC VL = 128, 256 or 512KL := VL / 64IF *SRC is a register* and (VL = 512) AND (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.double[0]ELSEtsrc := SRC.double[j]DEST.fp16[j] := Convert_fp64_to_fp16(tsrc)ELSE IF *zeroing*:DEST.fp16[j] := 0// else dest.fp16[j] remains unchangedDEST[MAXVL-1:VL/4] := 0 Intel C/C++ Compiler Intrinsic EquivalentVCVTPD2PH __m128h _mm512_cvt_roundpd_ph (__m512d a, int rounding);VCVTPD2PH __m128h _mm512_mask_cvt_roundpd_ph (__m128h src, __mmask8 k, __m512d a, int rounding);VCVTPD2PH __m128h _mm512_maskz_cvt_roundpd_ph (__mmask8 k, __m512d a, int rounding);VCVTPD2PH __m128h _mm_cvtpd_ph (__m128d a);VCVTPD2PH __m128h _mm_mask_cvtpd_ph (__m128h src, __mmask8 k, __m128d a);VCVTPD2PH __m128h _mm_maskz_cvtpd_ph (__mmask8 k, __m128d a);VCVTPD2PH __m128h _mm256_cvtpd_ph (__m256d a);VCVTPD2PH __m128h _mm256_mask_cvtpd_ph (__m128h src, __mmask8 k, __m256d a);VCVTPD2PH __m128h _mm256_maskz_cvtpd_ph (__mmask8 k, __m256d a);VCVTPD2PH __m128h _mm512_cvtpd_ph (__m512d a);VCVTPD2PH __m128h _mm512_mask_cvtpd_ph (__m128h src, __mmask8 k, __m512d a);VCVTPD2PH __m128h _mm512_maskz_cvtpd_ph (__mmask8 k, __m512d a);
```

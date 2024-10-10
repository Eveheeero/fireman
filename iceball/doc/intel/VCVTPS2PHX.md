# VCVTPS2PHX

Convert Packed Single Precision Floating-Point Values to Packed FP16 Values

This instruction converts packed single precision floating values in the source operand to FP16 values and stores to the destination operand.
The VCVTPS2PHX instruction supports broadcasting.
This instruction uses MXCSR.DAZ for handling FP32 inputs.
FP16 outputs can be normal or denormal numbers, and are not conditionally flushed based on MXCSR settings.

## Flags affected

- None.Intel C/C++ Compiler Intrinsic EquivalentVCVTPS2PHX __m256h _mm512_cvtx_roundps_ph (__m512 a, int rounding);VCVTPS2PHX __m256h _mm512_mask_cvtx_roundps_ph (__m256h src, __mmask16 k, __m512 a, int rounding);VCVTPS2PHX __m256h _mm512_maskz_cvtx_roundps_ph (__mmask16 k, __m512 a, int rounding);VCVTPS2PHX __m128h _mm_cvtxps_ph (__m128 a);VCVTPS2PHX __m128h _mm_mask_cvtxps_ph (__m128h src, __mmask8 k, __m128 a);VCVTPS2PHX __m128h _mm_maskz_cvtxps_ph (__mmask8 k, __m128 a);VCVTPS2PHX __m128h _mm256_cvtxps_ph (__m256 a);VCVTPS2PHX __m128h _mm256_mask_cvtxps_ph (__m128h src, __mmask8 k, __m256 a);VCVTPS2PHX __m128h _mm256_maskz_cvtxps_ph (__mmask8 k, __m256 a);VCVTPS2PHX __m256h _mm512_cvtxps_ph (__m512 a);VCVTPS2PHX __m256h _mm512_mask_cvtxps_ph (__m256h src, __mmask16 k, __m512 a);VCVTPS2PHX __m256h _mm512_maskz_cvtxps_ph (__mmask16 k, __m512 a);

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Underflow, Overflow, Precision, Denormal (if MXCSR.DAZ=0).
- Other Exceptions
  > EVEX-encoded instructions, see Table 2-46, "Type E2 Class Exception Conditions."
  > Additionally:

## Operation

```C
VCVTPS2PHX DEST, SRC (AVX512_FP16 Load Version With Broadcast Support)VL = 128, 256, or 512KL := VL / 32IF *SRC is a register* and (VL == 512) and (EVEX.b = 1):SET_RM(EVEX.RC)ELSE:SET_RM(MXCSR.RC)FOR j := 0 TO KL-1:IF k1[j] OR *no writemask*:IF *SRC is memory* and EVEX.b = 1:tsrc := SRC.fp32[0]ELSEtsrc := SRC.fp32[j]DEST.fp16[j] := Convert_fp32_to_fp16(tsrc)ELSE IF *zeroing*:DEST.fp16[j] := 0
```

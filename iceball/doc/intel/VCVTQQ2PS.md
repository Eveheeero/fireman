# VCVTQQ2PS

Convert Packed Quadword Integers to Packed Single Precision Floating-Point Values

Converts packed quadword integers in the source operand (second operand) to packed single precision floating-point values in the destination operand (first operand).
The source operand is a ZMM/YMM/XMM register or a 512/256/128-bit memory location.
The destination operation is a YMM/XMM/XMM (lower 64 bits) register conditionally updated with writemask k1.
EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > Precision.
- Other Exceptions
  > EVEX-encoded instructions, see Table2-46,
  >  "Type E2 Class Exception Conditions."

## Operation

```C
VCVTQQ2PS (EVEX Encoded Versions) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN DEST[k+31:k] :=Convert_QuadInteger_To_Single_Precision_Floating_Point(SRC[i+63:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[k+31:k] remains unchanged*ELSE ; zeroing-maskingDEST[k+31:k] := 0FIFI;VCVTQQ2PS (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b == 1) THENDEST[k+31:k] :=Convert_QuadInteger_To_Single_Precision_Floating_Point(SRC[63:0])ELSE DEST[k+31:k] :=Convert_QuadInteger_To_Single_Precision_Floating_Point(SRC[i+63:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[k+31:k] remains unchanged*ELSE ; zeroing-maskingDEST[k+31:k] := 0FIFI;ENDFORDEST[MAXVL-1:VL/2] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTQQ2PS __m256 _mm512_cvtepi64_ps( __m512i a);VCVTQQ2PS __m256 _mm512_mask_cvtepi64_ps( __m256 s, __mmask16 k, __m512i a);VCVTQQ2PS __m256 _mm512_maskz_cvtepi64_ps( __mmask16 k, __m512i a);VCVTQQ2PS __m256 _mm512_cvt_roundepi64_ps( __m512i a, int r);VCVTQQ2PS __m256 _mm512_mask_cvt_roundepi_ps( __m256 s, __mmask8 k, __m512i a, int r);VCVTQQ2PS __m256 _mm512_maskz_cvt_roundepi64_ps( __mmask8 k, __m512i a, int r);VCVTQQ2PS __m128 _mm256_cvtepi64_ps( __m256i a);VCVTQQ2PS __m128 _mm256_mask_cvtepi64_ps( __m128 s, __mmask8 k, __m256i a);VCVTQQ2PS __m128 _mm256_maskz_cvtepi64_ps( __mmask8 k, __m256i a);VCVTQQ2PS __m128 _mm_cvtepi64_ps( __m128i a);VCVTQQ2PS __m128 _mm_mask_cvtepi64_ps( __m128 s, __mmask8 k, __m128i a);VCVTQQ2PS __m128 _mm_maskz_cvtepi64_ps( __mmask8 k, __m128i a);
```

# VCVTUQQ2PS

Convert Packed Unsigned Quadword Integers to Packed Single Precision Floating-Point Values

Converts packed unsigned quadword integers in the source operand (second operand) to single precision floating-point values in the destination operand (first operand).
EVEX encoded versions: The source operand is a ZMM/YMM/XMM register or a 512/256/128-bit memory location.
The destination operand is a YMM/XMM/XMM (low 64 bits) register conditionally updated with writemask k1.
Note: EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > Precision.
- Other Exceptions
  > EVEX-encoded instructions, see Table2-46,
  >  "Type E2 Class Exception Conditions."

## Operation

```C
VCVTUQQ2PS (EVEX Encoded Version) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL = 512) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 32k := j * 64IF k1[j] OR *no writemask*THEN DEST[i+31:i] :=Convert_UQuadInteger_To_Single_Precision_Floating_Point(SRC[k+63:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;VCVTUQQ2PS (EVEX Encoded Version) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 32k := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+31:i] :=Convert_UQuadInteger_To_Single_Precision_Floating_Point(SRC[63:0])ELSE DEST[i+31:i] :=Convert_UQuadInteger_To_Single_Precision_Floating_Point(SRC[k+63:k])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL/2] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTUQQ2PS __m256 _mm512_cvtepu64_ps( __m512i a);VCVTUQQ2PS __m256 _mm512_mask_cvtepu64_ps( __m256 s, __mmask8 k, __m512i a);VCVTUQQ2PS __m256 _mm512_maskz_cvtepu64_ps( __mmask8 k, __m512i a);VCVTUQQ2PS __m256 _mm512_cvt_roundepu64_ps( __m512i a, int r);VCVTUQQ2PS __m256 _mm512_mask_cvt_roundepu64_ps( __m256 s, __mmask8 k, __m512i a, int r);VCVTUQQ2PS __m256 _mm512_maskz_cvt_roundepu64_ps( __mmask8 k, __m512i a, int r);VCVTUQQ2PS __m128 _mm256_cvtepu64_ps( __m256i a);VCVTUQQ2PS __m128 _mm256_mask_cvtepu64_ps( __m128 s, __mmask8 k, __m256i a);VCVTUQQ2PS __m128 _mm256_maskz_cvtepu64_ps( __mmask8 k, __m256i a);VCVTUQQ2PS __m128 _mm_cvtepu64_ps( __m128i a);VCVTUQQ2PS __m128 _mm_mask_cvtepu64_ps( __m128 s, __mmask8 k, __m128i a);VCVTUQQ2PS __m128 _mm_maskz_cvtepu64_ps( __mmask8 k, __m128i a);
```

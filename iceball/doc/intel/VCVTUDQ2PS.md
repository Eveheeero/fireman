# VCVTUDQ2PS

Convert Packed Unsigned Doubleword Integers to Packed Single Precision Floating-Point Values

Converts packed unsigned doubleword integers in the source operand (second operand) to single precision floating-point values in the destination operand (first operand).
The source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.
Note: EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- Other Exceptions
  > EVEX-encoded instructions, see Table2-46,
  >  "Type E2 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > Precision.

## Operation

```C
VCVTUDQ2PS (EVEX Encoded Version) When SRC Operand is a Register(KL, VL) = (4, 128), (8, 256), (16, 512)IF (VL = 512) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] :=Convert_UInteger_To_Single_Precision_Floating_Point(SRC[i+31:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;VCVTUDQ2PS (EVEX Encoded Version) When SRC Operand is a Memory Source(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+31:i] :=Convert_UInteger_To_Single_Precision_Floating_Point(SRC[31:0])ELSE DEST[i+31:i] :=Convert_UInteger_To_Single_Precision_Floating_Point(SRC[i+31:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTUDQ2PS __m512 _mm512_cvtepu32_ps( __m512i a);VCVTUDQ2PS __m512 _mm512_mask_cvtepu32_ps( __m512 s, __mmask16 k, __m512i a);VCVTUDQ2PS __m512 _mm512_maskz_cvtepu32_ps( __mmask16 k, __m512i a);VCVTUDQ2PS __m512 _mm512_cvt_roundepu32_ps( __m512i a, int r);VCVTUDQ2PS __m512 _mm512_mask_cvt_roundepu32_ps( __m512 s, __mmask16 k, __m512i a, int r);VCVTUDQ2PS __m512 _mm512_maskz_cvt_roundepu32_ps( __mmask16 k, __m512i a, int r);VCVTUDQ2PS __m256 _mm256_cvtepu32_ps( __m256i a);VCVTUDQ2PS __m256 _mm256_mask_cvtepu32_ps( __m256 s, __mmask8 k, __m256i a);VCVTUDQ2PS __m256 _mm256_maskz_cvtepu32_ps( __mmask8 k, __m256i a);VCVTUDQ2PS __m128 _mm_cvtepu32_ps( __m128i a);VCVTUDQ2PS __m128 _mm_mask_cvtepu32_ps( __m128 s, __mmask8 k, __m128i a);VCVTUDQ2PS __m128 _mm_maskz_cvtepu32_ps( __mmask8 k, __m128i a);
```

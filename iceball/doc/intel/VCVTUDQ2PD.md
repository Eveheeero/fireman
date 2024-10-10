# VCVTUDQ2PD

Convert Packed Unsigned Doubleword Integers to Packed Double Precision Floating-Point Values

Converts packed unsigned doubleword integers in the source operand (second operand) to packed double precision floating-point values in the destination operand (first operand).
The source operand is a YMM/XMM/XMM (low 64 bits) register, a 256/128/64-bit memory location or a 256/128/64-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.
Attempt to encode this instruction with EVEX embedded rounding is ignored.Note: EVEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > EVEX-encoded instructions, see Table2-51,
  >  "Type E5 Class Exception Conditions."

## Operation

```C
VCVTUDQ2PD (EVEX Encoded Versions) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN DEST[i+63:i] :=Convert_UInteger_To_Double_Precision_Floating_Point(SRC[k+31:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;VCVTUDQ2PD (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+63:i] :=Convert_UInteger_To_Double_Precision_Floating_Point(SRC[31:0])ELSE DEST[i+63:i] :=Convert_UInteger_To_Double_Precision_Floating_Point(SRC[k+31:k])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTUDQ2PD __m512d _mm512_cvtepu32_pd( __m256i a);VCVTUDQ2PD __m512d _mm512_mask_cvtepu32_pd( __m512d s, __mmask8 k, __m256i a);VCVTUDQ2PD __m512d _mm512_maskz_cvtepu32_pd( __mmask8 k, __m256i a);VCVTUDQ2PD __m256d _mm256_cvtepu32_pd( __m128i a);VCVTUDQ2PD __m256d _mm256_mask_cvtepu32_pd( __m256d s, __mmask8 k, __m128i a);VCVTUDQ2PD __m256d _mm256_maskz_cvtepu32_pd( __mmask8 k, __m128i a);VCVTUDQ2PD __m128d _mm_cvtepu32_pd( __m128i a);VCVTUDQ2PD __m128d _mm_mask_cvtepu32_pd( __m128d s, __mmask8 k, __m128i a);VCVTUDQ2PD __m128d _mm_maskz_cvtepu32_pd( __mmask8 k, __m128i a);
```

# CVTDQ2PD

Convert Packed Doubleword Integers to Packed Double Precision Floating-Point Values

Converts two, four or eight packed signed doubleword integers in the source operand (the second operand) to two, four or eight packed double precision floating-point values in the destination operand (the first operand).
EVEX encoded versions: The source operand can be a YMM/XMM/XMM (low 64 bits) register, a 256/128/64-bit memory location or a 256/128/64-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.
Attempt to encode this instruction with EVEX embedded rounding is ignored.VEX.256 encoded version: The source operand is an XMM register or 128- bit memory location.
The destination operand is a YMM register.
VEX.128 encoded version: The source operand is an XMM register or 64- bit memory location.
The destination operand is a XMM register.
The upper Bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The source operand is an XMM register or 64- bit memory location.
The destination operand is an XMM register.
The upper Bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.VEX.vvvv and EVEX.vvvv arX3X2X1X0SRCX3X2X1X0DESTFigure 3-11.
 CVTDQ2PD (VEX.256 encoded version)

## Exceptions

- Other Exceptions
  > VEX-encoded instructions, see Table2-22, "Type 5 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-51, "Type E5 Class Exception Conditions."

## Operation

```C
VCVTDQ2PD (EVEX Encoded Versions) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN DEST[i+63:i] :=Convert_Integer_To_Double_Precision_Floating_Point(SRC[k+31:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;VCVTDQ2PD (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+63:i] :=Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0])ELSE DEST[i+63:i] :=Convert_Integer_To_Double_Precision_Floating_Point(SRC[k+31:k])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VCVTDQ2PD (VEX.256 Encoded Version)DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0])DEST[127:64] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[63:32])DEST[191:128] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[95:64])DEST[255:192] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[127:96)DEST[MAXVL-1:256] := 0VCVTDQ2PD (VEX.128 Encoded Version)DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0])DEST[127:64] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[63:32])DEST[MAXVL-1:128] := 0CVTDQ2PD (128-bit Legacy SSE Version)DEST[63:0] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[31:0])DEST[127:64] := Convert_Integer_To_Double_Precision_Floating_Point(SRC[63:32])DEST[MAXVL-1:128] (unmodified)Intel C/C++ Compiler Intrinsic EquivalentVCVTDQ2PD __m512d _mm512_cvtepi32_pd( __m256i a);VCVTDQ2PD __m512d _mm512_mask_cvtepi32_pd( __m512d s, __mmask8 k, __m256i a);VCVTDQ2PD __m512d _mm512_maskz_cvtepi32_pd( __mmask8 k, __m256i a);VCVTDQ2PD __m256d _mm256_cvtepi32_pd (__m128i src);VCVTDQ2PD __m256d _mm256_mask_cvtepi32_pd( __m256d s, __mmask8 k, __m256i a);VCVTDQ2PD __m256d _mm256_maskz_cvtepi32_pd( __mmask8 k, __m256i a);VCVTDQ2PD __m128d _mm_mask_cvtepi32_pd( __m128d s, __mmask8 k, __m128i a);
```

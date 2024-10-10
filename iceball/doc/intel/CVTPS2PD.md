# CVTPS2PD

Convert Packed Single Precision Floating-Point Values to Packed Double Precision Floating-Point Values

Converts two, four or eight packed single precision floating-point values in the source operand (second operand) to two, four or eight packed double precision floating-point values in the destination operand (first operand).
EVEX encoded versions: The source operand is a YMM/XMM/XMM (low 64-bits) register, a 256/128/64-bit memory location or a 256/128/64-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.
VEX.256 encoded version: The source operand is an XMM register or 128- bit memory location.
The destination operand is a YMM register.
Bits (MAXVL-1:256) of the corresponding destination ZMM register are zeroed.VEX.128 encoded version: The source operand is an XMM register or 64- bit memory location.
The destination operand is a XMM register.
The upper Bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The source operand is an XMM register or 64- bit memory location.
The destination operand is an XMM register.
The upper Bits (MAXVL-1:128) of the corresponding ZMM register destination are X3X2X1X0SRCX3X2X1X0DESTFigure 3-14.
 CVTPS2PD (VEX.256 encoded version)

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Denormal.
- Other Exceptions
  > VEX-encoded instructions, see Table2-20, "Type 3 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-47, "Type E3 Class Exception Conditions."

## Operation

```C
VCVTPS2PD (EVEX Encoded Versions) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN DEST[i+63:i] :=Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[k+31:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VCVTPS2PD (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+63:i] :=Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[31:0])ELSE DEST[i+63:i] :=Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[k+31:k])IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VCVTPS2PD (VEX.256 Encoded Version)DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[31:0])DEST[127:64] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[63:32])DEST[191:128] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[95:64])DEST[255:192] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[127:96)DEST[MAXVL-1:256] := 0VCVTPS2PD (VEX.128 Encoded Version)DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[31:0])DEST[127:64] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[63:32])DEST[MAXVL-1:128] := 0CVTPS2PD (128-bit Legacy SSE Version)DEST[63:0] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[31:0])DEST[127:64] := Convert_Single_Precision_To_Double_Precision_Floating_Point(SRC[63:32])DEST[MAXVL-1:128] (unmodified)Intel C/C++ Compiler Intrinsic EquivalentVCVTPS2PD __m512d _mm512_cvtps_pd( __m256 a);VCVTPS2PD __m512d _mm512_mask_cvtps_pd( __m512d s, __mmask8 k, __m256 a);VCVTPS2PD __m512d _mm512_maskz_cvtps_pd( __mmask8 k, __m256 a);VCVTPS2PD __m512d _mm512_cvt_roundps_pd( __m256 a, int sae);VCVTPS2PD __m512d _mm512_mask_cvt_roundps_pd( __m512d s, __mmask8 k, __m256 a, int sae);VCVTPS2PD __m512d _mm512_maskz_cvt_roundps_pd( __mmask8 k, __m256 a, int sae);VCVTPS2PD __m256d _mm256_mask_cvtps_pd( __m256d s, __mmask8 k, __m128 a);VCVTPS2PD __m256d _mm256_maskz_cvtps_pd( __mmask8 k, __m128a);VCVTPS2PD __m128d _mm_mask_cvtps_pd( __m128d s, __mmask8 k, __m128 a);VCVTPS2PD __m128d _mm_maskz_cvtps_pd( __mmask8 k, __m128 a);VCVTPS2PD __m256d _mm256_cvtps_pd (__m128 a)CVTPS2PD __m128d _mm_cvtps_pd (__m128 a)
```

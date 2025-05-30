# CVTTPD2DQ

Convert with Truncation Packed Double Precision Floating-Point Values to Packed Doubleword Integers

Converts two, four or eight packed double precision floating-point values in the source operand (second operand) to two, four or eight packed signed doubleword integers in the destination operand (first operand).
When a conversion is inexact, a truncated (round toward zero) value is returned.
If a converted result is larger than the maximum signed doubleword integer, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value (80000000H) is returned.EVEX encoded versions: The source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a YMM/XMM/XMM (low 64 bits) register conditionally updated with writemask k1.
The upper bits (MAXVL-1:256) of the corresponding destination are zeroed.VEX.256 encoded version: The source operand is a YMM register or 256- bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: The source operand is an XMM register or 128- bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:64) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The source operand is an XMM register or 128- bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.X3X2SRCX1X0DEST0X3X2X1X0Figure 3-15.
 VCVTTPD2DQ (VEX.256 encoded version)

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
- Other Exceptions
  > VEX-encoded instructions, see Table2-19, "Type 2 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-46,
  >  "Type E2 Class Exception Conditions."
  > Additionally:

## Operation

```C
VCVTTPD2DQ (EVEX Encoded Versions) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 32k := j * 64IF k1[j] OR *no writemask*THEN DEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[k+63:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;VCVTTPD2DQ (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 32k := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0])ELSE DEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[k+63:k])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL/2] := 0VCVTTPD2DQ (VEX.256 Encoded Version)DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0])DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[127:64])DEST[95:64] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[191:128])DEST[127:96] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[255:192)DEST[MAXVL-1:128] := 0VCVTTPD2DQ (VEX.128 Encoded Version)DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0])DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[127:64])DEST[MAXVL-1:64] := 0CVTTPD2DQ (128-bit Legacy SSE Version)DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[63:0])DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer_Truncate(SRC[127:64])DEST[127:64] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTTPD2DQ __m256i _mm512_cvttpd_epi32( __m512d a);VCVTTPD2DQ __m256i _mm512_mask_cvttpd_epi32( __m256i s, __mmask8 k, __m512d a);VCVTTPD2DQ __m256i _mm512_maskz_cvttpd_epi32( __mmask8 k, __m512d a);VCVTTPD2DQ __m256i _mm512_cvtt_roundpd_epi32( __m512d a, int sae);VCVTTPD2DQ __m256i _mm512_mask_cvtt_roundpd_epi32( __m256i s, __mmask8 k, __m512d a, int sae);VCVTTPD2DQ __m256i _mm512_maskz_cvtt_roundpd_epi32( __mmask8 k, __m512d a, int sae);VCVTTPD2DQ __m128i _mm256_mask_cvttpd_epi32( __m128i s, __mmask8 k, __m256d a);VCVTTPD2DQ __m128i _mm256_maskz_cvttpd_epi32( __mmask8 k, __m256d a);VCVTTPD2DQ __m128i _mm_mask_cvttpd_epi32( __m128i s, __mmask8 k, __m128d a);VCVTTPD2DQ __m128i _mm_maskz_cvttpd_epi32( __mmask8 k, __m128d a);VCVTTPD2DQ __m128i _mm256_cvttpd_epi32 (__m256d src);CVTTPD2DQ __m128i _mm_cvttpd_epi32 (__m128d src);
```

# CVTPD2DQ

Convert Packed Double Precision Floating-Point Values to Packed Doubleword Integers

Converts packed double precision floating-point values in the source operand (second operand) to packed signed doubleword integers in the destination operand (first operand).
When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value w-1, where w represents the number of bits in the destination format) is returned.(2EVEX encoded versions: The source operand is a ZMM/YMM/XMM register, a 512-bit memory location, or a 512-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM/YMM/XMM register condi-tionally updated with writemask k1.
The upper bits (MAXVL-1:256/128/64) of the corresponding destination are zeroed.VEX.256 encoded version: The source operand is a YMM register or 256- bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: The source operand is an XMM register or 128- bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:64) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The source operand is an XMM register or 128- bit memory location.
The destination operand is an XMM register.
Bits[127:64] of the destination XMM register are zeroed.
However, the upper bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.
SRCX3X2X1X0DEST0X3X2X1X0Figure 3-12.
 VCVTPD2DQ (VEX.256 encoded version)

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
- Other Exceptions
  > See Table2-19, "Type 2 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-46,
  >  "Type E2 Class Exception Conditions."
  > Additionally:

## Operation

```C
VCVTPD2DQ (EVEX Encoded Versions) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL = 512) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 32k := j * 64IF k1[j] OR *no writemask*THEN DEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_Integer(SRC[k+63:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;VCVTPD2DQ (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 32k := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0])ELSE DEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_Integer(SRC[k+63:k])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL/2] := 0VCVTPD2DQ (VEX.256 Encoded Version)DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0])DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[127:64])DEST[95:64] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[191:128])DEST[127:96] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[255:192)DEST[MAXVL-1:128] := 0VCVTPD2DQ (VEX.128 Encoded Version)DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0])DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[127:64])DEST[MAXVL-1:64] := 0CVTPD2DQ (128-bit Legacy SSE Version)DEST[31:0] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[63:0])DEST[63:32] := Convert_Double_Precision_Floating_Point_To_Integer(SRC[127:64])DEST[127:64] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTPD2DQ __m256i _mm512_cvtpd_epi32( __m512d a);VCVTPD2DQ __m256i _mm512_mask_cvtpd_epi32( __m256i s, __mmask8 k, __m512d a);VCVTPD2DQ __m256i _mm512_maskz_cvtpd_epi32( __mmask8 k, __m512d a);VCVTPD2DQ __m256i _mm512_cvt_roundpd_epi32( __m512d a, int r);VCVTPD2DQ __m256i _mm512_mask_cvt_roundpd_epi32( __m256i s, __mmask8 k, __m512d a, int r);VCVTPD2DQ __m256i _mm512_maskz_cvt_roundpd_epi32( __mmask8 k, __m512d a, int r);VCVTPD2DQ __m128i _mm256_mask_cvtpd_epi32( __m128i s, __mmask8 k, __m256d a);VCVTPD2DQ __m128i _mm256_maskz_cvtpd_epi32( __mmask8 k, __m256d a);VCVTPD2DQ __m128i _mm_mask_cvtpd_epi32( __m128i s, __mmask8 k, __m128d a);VCVTPD2DQ __m128i _mm_maskz_cvtpd_epi32( __mmask8 k, __m128d a);VCVTPD2DQ __m128i _mm256_cvtpd_epi32 (__m256d src)CVTPD2DQ __m128i _mm_cvtpd_epi32 (__m128d src)
```

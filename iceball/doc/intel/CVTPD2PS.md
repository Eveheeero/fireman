# CVTPD2PS

Convert Packed Double Precision Floating-Point Values to Packed Single Precision Floating-Point Values

Converts two, four or eight packed double precision floating-point values in the source operand (second operand) to two, four or eight packed single precision floating-point values in the destination operand (first operand).
When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
EVEX encoded versions: The source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a YMM/XMM/XMM (low 64-bits) register conditionally updated with writemask k1.
The upper bits (MAXVL-1:256/128/64) of the corresponding destination are zeroed.VEX.256 encoded version: The source operand is a YMM register or 256- bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: The source operand is an XMM register or 128- bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:64) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The source operand is an XMM register or 128- bit memory location.
The destination operand is an XMM register.
Bits[127:64] of the destination XMM register are zeroed.
However, the upper Bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.VEX.vvvv and EVEX.vvvv arX3X2SRCX1X0DEST0X3X2X1X0Figure 3-13.
 VCVTPD2PS (VEX.256 encoded version)

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision, Underflow, Overflow, Denormal.
- Other Exceptions
  > VEX-encoded instructions, see Table2-19, "Type 2 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-46, "Type E2 Class Exception Conditions."

## Operation

```C
VCVTPD2PS (EVEX Encoded Version) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL = 512) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 32k := j * 64IF k1[j] OR *no writemask*THEN DEST[i+31:i] := Convert_Double_Precision_Floating_Point_To_Single_Precision_Floating_Point(SRC[k+63:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;VCVTPD2PS (EVEX Encoded Version) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 32k := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_Single_Precision_Floating_Point(SRC[63:0])ELSE DEST[i+31:i] := Convert_Double_Precision_Floating_Point_To_Single_Precision_Floating_Point(SRC[k+63:k])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL/2] := 0VCVTPD2PS (VEX.256 Encoded Version)DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[63:0])DEST[63:32] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[127:64])DEST[95:64] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[191:128])DEST[127:96] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[255:192)DEST[MAXVL-1:128] := 0VCVTPD2PS (VEX.128 Encoded Version)DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[63:0])DEST[63:32] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[127:64])DEST[MAXVL-1:64] := 0CVTPD2PS (128-bit Legacy SSE Version)DEST[31:0] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[63:0])DEST[63:32] := Convert_Double_Precision_To_Single_Precision_Floating_Point(SRC[127:64])DEST[127:64] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTPD2PS __m256 _mm512_cvtpd_ps( __m512d a);VCVTPD2PS __m256 _mm512_mask_cvtpd_ps( __m256 s, __mmask8 k, __m512d a);VCVTPD2PS __m256 _mm512_maskz_cvtpd_ps( __mmask8 k, __m512d a);VCVTPD2PS __m256 _mm512_cvt_roundpd_ps( __m512d a, int r);VCVTPD2PS __m256 _mm512_mask_cvt_roundpd_ps( __m256 s, __mmask8 k, __m512d a, int r);VCVTPD2PS __m256 _mm512_maskz_cvt_roundpd_ps( __mmask8 k, __m512d a, int r);VCVTPD2PS __m128 _mm256_mask_cvtpd_ps( __m128 s, __mmask8 k, __m256d a);VCVTPD2PS __m128 _mm256_maskz_cvtpd_ps( __mmask8 k, __m256d a);VCVTPD2PS __m128 _mm_mask_cvtpd_ps( __m128 s, __mmask8 k, __m128d a);VCVTPD2PS __m128 _mm_maskz_cvtpd_ps( __mmask8 k, __m128d a);VCVTPD2PS __m128 _mm256_cvtpd_ps (__m256d a)CVTPD2PS __m128 _mm_cvtpd_ps (__m128d a)
```

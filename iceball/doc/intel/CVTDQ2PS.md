# CVTDQ2PS

Convert Packed Doubleword Integers to Packed Single Precision Floating-Point Values

Converts four, eight or sixteen packed signed doubleword integers in the source operand to four, eight or sixteen packed single precision floating-point values in the destination operand.EVEX encoded versions: The source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory loca-tion or a 512/256/128-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.VEX.256 encoded version: The source operand is a YMM register or 256- bit memory location.
The destination operand is a YMM register.
Bits (MAXVL-1:256) of the corresponding register destination are zeroed.VEX.128 encoded version: The source operand is an XMM register or 128- bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:128) of the corresponding register destination are zeroed.128-bit Legacy SSE version: The source operand is an XMM register or 128- bit memory location.
The destination operand is an XMM register.
The upper Bits (MAXVL-1:128) of the corresponding register destination are unmodi-fied.VEX.vvvv and EVEX.vvvv ar

## Exceptions

- Other Exceptions
  > VEX-encoded instructions, see Table2-19, "Type 2 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-46,
  >  "Type E2 Class Exception Conditions."
  > Additionally:
- SIMD Floating-Point Exceptions
  > Precision.

## Operation

```C
VCVTDQ2PS (EVEX Encoded Versions) When SRC Operand is a Register(KL, VL) = (4, 128), (8, 256), (16, 512)IF (VL = 512) AND (EVEX.b = 1) THEN® 64 and IA-32 Architectures SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);  ; refer to Table 15-4 in the IntelSoftware Developer's Manual, Volume 1ELSE ®SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);  ; refer to Table 15-4 in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] :=Convert_Integer_To_Single_Precision_Floating_Point(SRC[i+31:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VCVTDQ2PS (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+31:i] :=Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0])ELSE DEST[i+31:i] :=Convert_Integer_To_Single_Precision_Floating_Point(SRC[i+31:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;VCVTDQ2PS (VEX.256 Encoded Version)DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0])DEST[63:32] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:32])DEST[95:64] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[95:64])DEST[127:96] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[127:96)DEST[159:128] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[159:128])DEST[191:160] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[191:160])DEST[223:192] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[223:192])DEST[255:224] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[255:224)DEST[MAXVL-1:256] := 0VCVTDQ2PS (VEX.128 Encoded Version)DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0])DEST[63:32] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:32])DEST[95:64] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[95:64])DEST[127:96] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[127z:96)DEST[MAXVL-1:128] := 0CVTDQ2PS (128-bit Legacy SSE Version)DEST[31:0] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[31:0])DEST[63:32] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[63:32])DEST[95:64] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[95:64])DEST[127:96] := Convert_Integer_To_Single_Precision_Floating_Point(SRC[127z:96)DEST[MAXVL-1:128] (unmodified)Intel C/C++ Compiler Intrinsic EquivalentVCVTDQ2PS __m512 _mm512_cvtepi32_ps( __m512i a);VCVTDQ2PS __m512 _mm512_mask_cvtepi32_ps( __m512 s, __mmask16 k, __m512i a);VCVTDQ2PS __m512 _mm512_maskz_cvtepi32_ps( __mmask16 k, __m512i a);VCVTDQ2PS __m512 _mm512_cvt_roundepi32_ps( __m512i a, int r);VCVTDQ2PS __m512 _mm512_mask_cvt_roundepi_ps( __m512 s, __mmask16 k, __m512i a, int r);VCVTDQ2PS __m512 _mm512_maskz_cvt_roundepi32_ps( __mmask16 k, __m512i a, int r);VCVTDQ2PS __m256 _mm256_mask_cvtepi32_ps( __m256 s, __mmask8 k, __m256i a);VCVTDQ2PS __m256 _mm256_maskz_cvtepi32_ps( __mmask8 k, __m256i a);VCVTDQ2PS __m128 _mm_mask_cvtepi32_ps( __m128 s, __mmask8 k, __m128i a);VCVTDQ2PS __m128 _mm_maskz_cvtepi32_ps( __mmask8 k, __m128i a);CVTDQ2PS __m256 _mm256_cvtepi32_ps (__m256i src)CVTDQ2PS __m128 _mm_cvtepi32_ps (__m128i src)
```

# CVTPS2DQ

Convert Packed Single Precision Floating-Point Values to Packed Signed Doubleword Integer Values

Converts four, eight or sixteen packed single precision floating-point values in the source operand to four, eight or sixteen signed doubleword integers in the destination operand.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value w-1, where w represents the number of bits in the destination format) is returned.(2EVEX encoded versions: The source operand is a ZMM register, a 512-bit memory location or a 512-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM register conditionally updated with writemask k1.
VEX.256 encoded version: The source operand is a YMM register or 256- bit memory location.
The destination operand is a YMM register.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 encoded version: The source operand is an XMM register or 128- bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.128-bit Legacy SSE version: The source operand is an XMM register or 128- bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.VEX.vvvv and EVEX.vvvv ar

## Exceptions

- Other Exceptions
  > VEX-encoded instructions, see Table2-19, "Type 2 Class Exception Conditions."
  > EVEX-encoded instructions, see Table2-46,
  >  "Type E2 Class Exception Conditions."
  > Additionally:
- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTPS2DQ (Encoded Versions) When SRC Operand is a Register(KL, VL) = (4, 128), (8, 256), (16, 512)IF (VL = 512) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] :=Convert_Single_Precision_Floating_Point_To_Integer(SRC[i+31:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VCVTPS2DQ (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO 15i := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+31:i] :=Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0])ELSE DEST[i+31:i] :=Convert_Single_Precision_Floating_Point_To_Integer(SRC[i+31:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;VCVTPS2DQ (VEX.256 Encoded Version)DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0])DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[63:32])DEST[95:64] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[95:64])DEST[127:96] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[127:96)DEST[159:128] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[159:128])DEST[191:160] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[191:160])DEST[223:192] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[223:192])DEST[255:224] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[255:224])VCVTPS2DQ (VEX.128 Encoded Version)DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0])DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[63:32])DEST[95:64] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[95:64])DEST[127:96] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[127:96])DEST[MAXVL-1:128] := 0CVTPS2DQ (128-bit Legacy SSE Version)DEST[31:0] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[31:0])DEST[63:32] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[63:32])DEST[95:64] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[95:64])DEST[127:96] := Convert_Single_Precision_Floating_Point_To_Integer(SRC[127:96])DEST[MAXVL-1:128] (unmodified)Intel C/C++ Compiler Intrinsic EquivalentVCVTPS2DQ __m512i _mm512_cvtps_epi32( __m512 a);VCVTPS2DQ __m512i _mm512_mask_cvtps_epi32( __m512i s, __mmask16 k, __m512 a);VCVTPS2DQ __m512i _mm512_maskz_cvtps_epi32( __mmask16 k, __m512 a);VCVTPS2DQ __m512i _mm512_cvt_roundps_epi32( __m512 a, int r);VCVTPS2DQ __m512i _mm512_mask_cvt_roundps_epi32( __m512i s, __mmask16 k, __m512 a, int r);VCVTPS2DQ __m512i _mm512_maskz_cvt_roundps_epi32( __mmask16 k, __m512 a, int r);VCVTPS2DQ __m256i _mm256_mask_cvtps_epi32( __m256i s, __mmask8 k, __m256 a);VCVTPS2DQ __m256i _mm256_maskz_cvtps_epi32( __mmask8 k, __m256 a);VCVTPS2DQ __m128i _mm_mask_cvtps_epi32( __m128i s, __mmask8 k, __m128 a);VCVTPS2DQ __m128i _mm_maskz_cvtps_epi32( __mmask8 k, __m128 a);VCVTPS2DQ __ m256i _mm256_cvtps_epi32 (__m256 a)CVTPS2DQ __m128i _mm_cvtps_epi32 (__m128 a)
```

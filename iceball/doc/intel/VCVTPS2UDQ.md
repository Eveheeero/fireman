# VCVTPS2UDQ

Convert Packed Single Precision Floating-Point Values to Packed Unsigned Doubleword Integer Values

Converts sixteen packed single precision floating-point values in the source operand to sixteen unsigned double-word integers in the destination operand.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination w - 1 is format, the floating-point invalid exception is raised, and if this exception is masked, the integer value 2returned, where w represents the number of bits in the destination format.The source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.


## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
- Other Exceptions
  > EVEX-encoded instructions, see Table2-46,
  >  "Type E2 Class Exception Conditions."

## Operation

```C
VCVTPS2UDQ (EVEX Encoded Versions) When SRC Operand is a Register(KL, VL) = (4, 128), (8, 256), (16, 512)IF (VL = 512) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] :=Convert_Single_Precision_Floating_Point_To_UInteger(SRC[i+31:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VCVTPS2UDQ (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no *THEN IF (EVEX.b = 1) THENDEST[i+31:i] :=Convert_Single_Precision_Floating_Point_To_UInteger(SRC[31:0])ELSE DEST[i+31:i] :=Convert_Single_Precision_Floating_Point_To_UInteger(SRC[i+31:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] :=Intel C/C++ Compiler Intrinsic EquivalentVCVTPS2UDQ __m512i _mm512_cvtps_epu32( __m512 a);VCVTPS2UDQ __m512i _mm512_mask_cvtps_epu32( __m512i s, __mmask16 k, __m512 a);VCVTPS2UDQ __m512i _mm512_maskz_cvtps_epu32( __mmask16 k, __m512 a);VCVTPS2UDQ __m512i _mm512_cvt_roundps_epu32( __m512 a, int r);VCVTPS2UDQ __m512i _mm512_mask_cvt_roundps_epu32( __m512i s, __mmask16 k, __m512 a, int r);VCVTPS2UDQ __m512i _mm512_maskz_cvt_roundps_epu32( __mmask16 k, __m512 a, int r);VCVTPS2UDQ __m256i _mm256_cvtps_epu32( __m256d a);VCVTPS2UDQ __m256i _mm256_mask_cvtps_epu32( __m256i s, __mmask8 k, __m256 a);VCVTPS2UDQ __m256i _mm256_maskz_cvtps_epu32( __mmask8 k, __m256 a);VCVTPS2UDQ __m128i _mm_cvtps_epu32( __m128 a);VCVTPS2UDQ __m128i _mm_mask_cvtps_epu32( __m128i s, __mmask8 k, __m128 a);VCVTPS2UDQ __m128i _mm_maskz_cvtps_epu32( __mmask8 k, __m128 a);
```

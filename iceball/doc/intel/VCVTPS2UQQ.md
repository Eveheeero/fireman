# VCVTPS2UQQ

Convert Packed Single Precision Floating-Point Values to Packed Unsigned Quadword Integer Values

Converts up to eight packed single precision floating-point values in the source operand to unsigned quadword integers in the destination operand.When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination w - 1 is format, the floating-point invalid exception is raised, and if this exception is masked, the integer value 2returned, where w represents the number of bits in the destination format.The source operand is a YMM/XMM/XMM (low 64- bits) register or a 256/128/64-bit memory location.
The destina-tion operation is a ZMM/YMM/XMM register conditionally updated with writemask k1.
EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > Invalid, Precision.
- Other Exceptions
  > EVEX-encoded instructions, see Table2-47,
  >  "Type E3 Class Exception Conditions."

## Operation

```C
VCVTPS2UQQ (EVEX Encoded Versions) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL == 512) AND (EVEX.b == 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN DEST[i+63:i] :=Convert_Single_Precision_To_UQuadInteger(SRC[k+31:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingFI;ENDFORDEST[MAXVL-1:VL] := 0VCVTPS2UQQ (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b == 1) THENDEST[i+63:i] :=Convert_Single_Precision_To_UQuadInteger(SRC[31:0])ELSE DEST[i+63:i] :=Convert_Single_Precision_To_UQuadInteger(SRC[k+31:k])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTPS2UQQ __m512i _mm512_cvtps_epu64( __m512 a);VCVTPS2UQQ __m512i _mm512_mask_cvtps_epu64( __m512i s, __mmask16 k, __m512 a);VCVTPS2UQQ __m512i _mm512_maskz_cvtps_epu64( __mmask16 k, __m512 a);VCVTPS2UQQ __m512i _mm512_cvt_roundps_epu64( __m512 a, int r);VCVTPS2UQQ __m512i _mm512_mask_cvt_roundps_epu64( __m512i s, __mmask16 k, __m512 a, int r);VCVTPS2UQQ __m512i _mm512_maskz_cvt_roundps_epu64( __mmask16 k, __m512 a, int r);VCVTPS2UQQ __m256i _mm256_cvtps_epu64( __m256 a);VCVTPS2UQQ __m256i _mm256_mask_cvtps_epu64( __m256i s, __mmask8 k, __m256 a);VCVTPS2UQQ __m256i _mm256_maskz_cvtps_epu64( __mmask8 k, __m256 a);VCVTPS2UQQ __m128i _mm_cvtps_epu64( __m128 a);VCVTPS2UQQ __m128i _mm_mask_cvtps_epu64( __m128i s, __mmask8 k, __m128 a);VCVTPS2UQQ __m128i _mm_maskz_cvtps_epu64( __mmask8 k, __m128 a);
```

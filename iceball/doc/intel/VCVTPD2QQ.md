# VCVTPD2QQ

Convert Packed Double Precision Floating-Point Values to Packed Quadword Integers

Converts packed double precision floating-point values in the source operand (second operand) to packed quad-word integers in the destination operand (first operand).
EVEX encoded versions: The source operand is a ZMM/YMM/XMM register or a 512/256/128-bit memory location.
The destination operation is a ZMM/YMM/XMM register conditionally updated with writemask k1.
When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, the indefinite integer value w-1, where w represents the number of bits in the destination format) is returned.(2EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.

## Exceptions

- Other Exceptions
  > EVEX-encoded instructions, see Table2-46, "Type E2 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTPD2QQ (EVEX Encoded Version) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL == 512) AND (EVEX.b == 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] :=Convert_Double_Precision_Floating_Point_To_QuadInteger(SRC[i+63:i])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingFI;ENDFORDEST[MAXVL-1:VL] := 0VCVTPD2QQ (EVEX Encoded Version) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b == 1) THENDEST[i+63:i] :=Convert_Double_Precision_Floating_Point_To_QuadInteger(SRC[63:0])ELSE DEST[i+63:i] := Convert_Double_Precision_Floating_Point_To_QuadInteger(SRC[i+63:i])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTPD2QQ __m512i _mm512_cvtpd_epi64( __m512d a);VCVTPD2QQ __m512i _mm512_mask_cvtpd_epi64( __m512i s, __mmask8 k, __m512d a);VCVTPD2QQ __m512i _mm512_maskz_cvtpd_epi64( __mmask8 k, __m512d a);VCVTPD2QQ __m512i _mm512_cvt_roundpd_epi64( __m512d a, int r);VCVTPD2QQ __m512i _mm512_mask_cvt_roundpd_epi64( __m512i s, __mmask8 k, __m512d a, int r);VCVTPD2QQ __m512i _mm512_maskz_cvt_roundpd_epi64( __mmask8 k, __m512d a, int r);VCVTPD2QQ __m256i _mm256_mask_cvtpd_epi64( __m256i s, __mmask8 k, __m256d a);VCVTPD2QQ __m256i _mm256_maskz_cvtpd_epi64( __mmask8 k, __m256d a);VCVTPD2QQ __m128i _mm_mask_cvtpd_epi64( __m128i s, __mmask8 k, __m128d a);VCVTPD2QQ __m128i _mm_maskz_cvtpd_epi64( __mmask8 k, __m128d a);VCVTPD2QQ __m256i _mm256_cvtpd_epi64 (__m256d src)VCVTPD2QQ __m128i _mm_cvtpd_epi64 (__m128d src)
```

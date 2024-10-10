# VCVTPD2UDQ

Convert Packed Double Precision Floating-Point Values to Packed Unsigned Doubleword Integers

Converts packed double precision floating-point values in the source operand (the second operand) to packed unsigned doubleword integers in the destination operand (the first operand).
When a conversion is inexact, the value returned is rounded according to the rounding control bits in the MXCSR register or the embedded rounding control bits.
If a converted result cannot be represented in the destination w - 1 is format, the floating-point invalid exception is raised, and if this exception is masked, the integer value 2returned, where w represents the number of bits in the destination format.The source operand is a ZMM/YMM/XMM register, a 512/256/128-bit memory location, or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destination operand is a ZMM/YMM/XMM register conditionally updated with writemask k1.
The upper bits (MAXVL-1:256) of the corresponding destination are zeroed.EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.

## Exceptions

- Other Exceptions
  > EVEX-encoded instructions, see Table2-46, "Type E2 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTPD2UDQ (EVEX Encoded Versions) When SRC2 Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)IF (VL = 512) AND (EVEX.b = 1) THENSET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(EVEX.RC);ELSE SET_ROUNDING_MODE_FOR_THIS_INSTRUCTION(MXCSR.RC);FI;FOR j := 0 TO KL-1i := j * 32k := j * 64IF k1[j] OR *no writemask*THEN DEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_UInteger(SRC[k+63:k])ELSE ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL/2] := 0VCVTPD2UDQ (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 32k := j * 64IF k1[j] OR *no writemask*THEN IF (EVEX.b = 1) THENDEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_UInteger(SRC[63:0])ELSE DEST[i+31:i] :=Convert_Double_Precision_Floating_Point_To_UInteger(SRC[k+63:k])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL/2] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTPD2UDQ __m256i _mm512_cvtpd_epu32( __m512d a);VCVTPD2UDQ __m256i _mm512_mask_cvtpd_epu32( __m256i s, __mmask8 k, __m512d a);VCVTPD2UDQ __m256i _mm512_maskz_cvtpd_epu32( __mmask8 k, __m512d a);VCVTPD2UDQ __m256i _mm512_cvt_roundpd_epu32( __m512d a, int r);VCVTPD2UDQ __m256i _mm512_mask_cvt_roundpd_epu32( __m256i s, __mmask8 k, __m512d a, int r);VCVTPD2UDQ __m256i _mm512_maskz_cvt_roundpd_epu32( __mmask8 k, __m512d a, int r);VCVTPD2UDQ __m128i _mm256_mask_cvtpd_epu32( __m128i s, __mmask8 k, __m256d a);VCVTPD2UDQ __m128i _mm256_maskz_cvtpd_epu32( __mmask8 k, __m256d a);VCVTPD2UDQ __m128i _mm_mask_cvtpd_epu32( __m128i s, __mmask8 k, __m128d a);VCVTPD2UDQ __m128i _mm_maskz_cvtpd_epu32( __mmask8 k, __m128d a);
```

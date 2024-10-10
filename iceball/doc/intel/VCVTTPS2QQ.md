# VCVTTPS2QQ

Convert With Truncation Packed Single Precision Floating-Point Values to Packed Signed Quadword Integer Values

Converts with truncation packed single precision floating-point values in the source operand to eight signed quad-word integers in the destination operand.When a conversion is inexact, a truncated (round toward zero) value is returned.
If a converted result cannot be represented in the destination format, the floating-point invalid exception is raised, and if this exception is masked, w-1, where w represents the number of bits in the destination format) is returned.the indefinite integer value (2EVEX encoded versions: The source operand is a YMM/XMM/XMM (low 64 bits) register or a 256/128/64-bit memory location.
The destination operation is a vector register conditionally updated with writemask k1.
Note: EVEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.

## Exceptions

- Other Exceptions
  > EVEX-encoded instructions, see Table2-47,
  >  "Type E3 Class Exception Conditions."
- SIMD Floating-Point Exceptions
  > Invalid, Precision.

## Operation

```C
VCVTTPS2QQ (EVEX Encoded Versions) When SRC Operand is a Register(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN DEST[i+63:i] :=Convert_Single_Precision_To_QuadInteger_Truncate(SRC[k+31:k])ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] :=VCVTTPS2QQ (EVEX Encoded Versions) When SRC Operand is a Memory Source(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j] OR *no writemask*THEN IF (EVEX.b == 1) THENDEST[i+63:i] :=Convert_Single_Precision_To_QuadInteger_Truncate(SRC[31:0])ELSE DEST[i+63:i] :=Convert_Single_Precision_To_QuadInteger_Truncate(SRC[k+31:k])FI;ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentVCVTTPS2QQ __m512i _mm512_cvttps_epi64( __m256 a);VCVTTPS2QQ __m512i _mm512_mask_cvttps_epi64( __m512i s, __mmask16 k, __m256 a);VCVTTPS2QQ __m512i _mm512_maskz_cvttps_epi64( __mmask16 k, __m256 a);VCVTTPS2QQ __m512i _mm512_cvtt_roundps_epi64( __m256 a, int sae);VCVTTPS2QQ __m512i _mm512_mask_cvtt_roundps_epi64( __m512i s, __mmask16 k, __m256 a, int sae);VCVTTPS2QQ __m512i _mm512_maskz_cvtt_roundps_epi64( __mmask16 k, __m256 a, int sae);VCVTTPS2QQ __m256i _mm256_mask_cvttps_epi64( __m256i s, __mmask8 k, __m128 a);VCVTTPS2QQ __m256i _mm256_maskz_cvttps_epi64( __mmask8 k, __m128 a);VCVTTPS2QQ __m128i _mm_mask_cvttps_epi64( __m128i s, __mmask8 k, __m128 a);VCVTTPS2QQ __m128i _mm_maskz_cvttps_epi64( __mmask8 k, __m128 a);
```

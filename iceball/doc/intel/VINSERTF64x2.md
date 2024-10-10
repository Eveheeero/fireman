# VINSERTF128/VINSERTF32x4/VINSERTF64x2/VINSERTF32x8/VINSERTF64x4

Insert Packed Floating-Point Values

VINSERTF128/VINSERTF32x4 and VINSERTF64x2 insert 128-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granularity offset multiplied by imm8[0] (256-bit) or imm8[1:0].
The remaining portions of the destination operand are copied from the corresponding fields of the first source operand (the second operand).
The second source operand can be either an XMM register or a 128-bit memory location.
The destination and first source operands are vector registers.VINSERTF32x4: The destination operand is a ZMM/YMM register and updated at 32-bit granularity according to the writemask.
The high 6/7 bits of the immediate are ignored.
VINSERTF64x2: The destination operand is a ZMM/YMM register and updated at 64-bit granularity according to the writemask.
The high 6/7 bits of the immediate are ignored.
VINSERTF32x8 and VINSERTF64x4 inserts 256-bits of packed floating-point values from the second source operand (the third operand) into the destination operand (the first operand) at a 256-bit granular offset multiplied by imm8[0].
The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand).
The second source operand can be either an YMM register or a 256-bit memory location.
The high 7 bits of the immediate are ignored.


## Exceptions

- Other Exceptions
  > VEX-encoded instruction, see Table2-23,
  >  "Type 6 Class Exception Conditions."
  > Additionally:
- SIMD Floating-Point Exceptions
  > None

## Operation

```C
VINSERTF32x4 (EVEX encoded versions) (KL, VL) = (8, 256), (16, 512)TEMP_DEST[VL-1:0] := SRC1[VL-1:0]IF VL = 256CASE (imm8[0]) OF0: TMP_DEST[127:0] := SRC2[127:0]1: TMP_DEST[255:128] := SRC2[127:0]ESAC.FI;IF VL = 512CASE (imm8[1:0]) OF00: TMP_DEST[127:0] := SRC2[127:0]01: TMP_DEST[255:128] := SRC2[127:0]10: TMP_DEST[383:256] := SRC2[127:0]11: TMP_DEST[511:384] := SRC2[127:0]ESAC.FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := TMP_DEST[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VINSERTF64x2 (EVEX encoded versions) (KL, VL) = (4, 256), (8, 512)TEMP_DEST[VL-1:0] := SRC1[VL-1:0]IF VL = 256CASE (imm8[0]) OF0: TMP_DEST[127:0] := SRC2[127:0]1: TMP_DEST[255:128] := SRC2[127:0]ESAC.FI;IF VL = 512CASE (imm8[1:0]) OF00: TMP_DEST[127:0] := SRC2[127:0]01: TMP_DEST[255:128] := SRC2[127:0]10: TMP_DEST[383:256] := SRC2[127:0]11: TMP_DEST[511:384] := SRC2[127:0]ESAC.FI;FOR j := 0 TO KL-1i := j * 64IF k1[j] OR *no writemask*IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VINSERTF32x8 (EVEX.U1.512 encoded version)TEMP_DEST[VL-1:0] := SRC1[VL-1:0]CASE (imm8[0]) OF0: TMP_DEST[255:0] := SRC2[255:0]1: TMP_DEST[511:256] := SRC2[255:0]ESAC.FOR j := 0 TO 15i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := TMP_DEST[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VINSERTF64x4 (EVEX.512 encoded version) VL = 512TEMP_DEST[VL-1:0] := SRC1[VL-1:0]CASE (imm8[0]) OF0: TMP_DEST[255:0] := SRC2[255:0]1: TMP_DEST[511:256] := SRC2[255:0]ESAC.FOR j := 0 TO 7i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := TMP_DEST[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;VINSERTF128 (VEX encoded version)TEMP[255:0] := SRC1[255:0]CASE (imm8[0]) OF0: TEMP[127:0] := SRC2[127:0]1: TEMP[255:128] := SRC2[127:0]ESACDEST := TEMPIntel C/C++ Compiler Intrinsic EquivalentVINSERTF32x4 __m512 _mm512_insertf32x4( __m512 a, __m128 b, int imm);VINSERTF32x4 __m512 _mm512_mask_insertf32x4(__m512 s, __mmask16 k, __m512 a, __m128 b, int imm);VINSERTF32x4 __m512 _mm512_maskz_insertf32x4( __mmask16 k, __m512 a, __m128 b, int imm);VINSERTF32x4 __m256 _mm256_insertf32x4( __m256 a, __m128 b, int imm);VINSERTF32x4 __m256 _mm256_mask_insertf32x4(__m256 s, __mmask8 k, __m256 a, __m128 b, int imm);VINSERTF32x4 __m256 _mm256_maskz_insertf32x4( __mmask8 k, __m256 a, __m128 b, int imm);VINSERTF32x8 __m512 _mm512_insertf32x8( __m512 a, __m256 b, int imm);VINSERTF32x8 __m512 _mm512_mask_insertf32x8(__m512 s, __mmask16 k, __m512 a, __m256 b, int imm);VINSERTF32x8 __m512 _mm512_maskz_insertf32x8( __mmask16 k, __m512 a, __m256 b, int imm);VINSERTF64x2 __m512d _mm512_insertf64x2( __m512d a, __m128d b, int imm);VINSERTF64x2 __m512d _mm512_mask_insertf64x2(__m512d s, __mmask8 k, __m512d a, __m128d b, int imm);VINSERTF64x2 __m512d _mm512_maskz_insertf64x2( __mmask8 k, __m512d a, __m128d b, int imm);VINSERTF64x2 __m256d _mm256_insertf64x2( __m256d a, __m128d b, int imm);VINSERTF64x2 __m256d _mm256_mask_insertf64x2(__m256d s, __mmask8 k, __m256d a, __m128d b, int imm);VINSERTF64x2 __m256d _mm256_maskz_insertf64x2( __mmask8 k, __m256d a, __m128d b, int imm);VINSERTF64x4 __m512d _mm512_insertf64x4( __m512d a, __m256d b, int imm);VINSERTF64x4 __m512d _mm512_mask_insertf64x4(__m512d s, __mmask8 k, __m512d a, __m256d b, int imm);VINSERTF64x4 __m512d _mm512_maskz_insertf64x4( __mmask8 k, __m512d a, __m256d b, int imm);VINSERTF128 __m256 _mm256_insertf128_ps (__m256 a, __m128 b, int offset);VINSERTF128 __m256d _mm256_insertf128_pd (__m256d a, __m128d b, int offset);VINSERTF128 __m256i _mm256_insertf128_si256 (__m256i a, __m128i b, int offset);
```

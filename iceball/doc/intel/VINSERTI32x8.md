# VINSERTI128/VINSERTI32x4/VINSERTI64x2/VINSERTI32x8/VINSERTI64x4

Insert Packed Integer Values

VINSERTI32x4 and VINSERTI64x2 inserts 128-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at an 128-bit granular offset multiplied by imm8[0] (256-bit) or imm8[1:0].
The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand).
The second source operand can be either an XMM register or a 128-bit memory location.
The high 6/7bits of the immediate are ignored.
The destination operand is a ZMM/YMM register and updated at 32 and 64-bit granularity according to the writemask.VINSERTI32x8 and VINSERTI64x4 inserts 256-bits of packed integer values from the second source operand (the third operand) into the destination operand (the first operand) at a 256-bit granular offset multiplied by imm8[0].
The remaining portions of the destination are copied from the corresponding fields of the first source operand (the second operand).
The second source operand can be either an YMM register or a 256-bit memory location.
The upper bits of the immediate are ignored.
The destination operand is a ZMM register and updated at 32 and 64-bit granularity according to the writemask.VINSERTI128 inserts 128-bits of packed integer data from the second source operand (the third operand) into the destination operand (the first operand) at a 128-bit granular offset multiplied by imm8[0].
The remaining portions second source operand can be either an XMM register or a 128-bit memory location.
The high 7 bits of the imme-diate are ignored.
VEX.L must be 1, otherwise attempt to execute this instruction with VEX.L=0 will cause #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > VEX-encoded instruction, see Table2-23,
  >  "Type 6 Class Exception Conditions."
  > Additionally:

## Operation

```C
VINSERTI32x4 (EVEX encoded versions) (KL, VL) = (8, 256), (16, 512)TEMP_DEST[VL-1:0] := SRC1[VL-1:0]IF VL = 256CASE (imm8[0]) OF0: TMP_DEST[127:0] := SRC2[127:0]1: TMP_DEST[255:128] := SRC2[127:0]ESAC.FI;IF VL = 512CASE (imm8[1:0]) OF00: TMP_DEST[127:0] := SRC2[127:0]01: TMP_DEST[255:128] := SRC2[127:0]10: TMP_DEST[383:256] := SRC2[127:0]11: TMP_DEST[511:384] := SRC2[127:0]ESAC.FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := TMP_DEST[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VINSERTI64x2 (EVEX encoded versions) (KL, VL) = (4, 256), (8, 512)TEMP_DEST[VL-1:0] := SRC1[VL-1:0]IF VL = 256CASE (imm8[0]) OF0: TMP_DEST[127:0] := SRC2[127:0]1: TMP_DEST[255:128] := SRC2[127:0]ESAC.FI;IF VL = 512CASE (imm8[1:0]) OF00: TMP_DEST[127:0] := SRC2[127:0]01: TMP_DEST[255:128] := SRC2[127:0]10: TMP_DEST[383:256] := SRC2[127:0]11: TMP_DEST[511:384] := SRC2[127:0]ESAC.FI;IF k1[j] OR *no writemask*THEN DEST[i+63:i] := TMP_DEST[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VINSERTI32x8 (EVEX.U1.512 encoded version)TEMP_DEST[VL-1:0] := SRC1[VL-1:0]CASE (imm8[0]) OF0: TMP_DEST[255:0] := SRC2[255:0]1: TMP_DEST[511:256] := SRC2[255:0]ESAC.FOR j := 0 TO 15i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := TMP_DEST[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VINSERTI64x4 (EVEX.512 encoded version) VL = 512TEMP_DEST[VL-1:0] := SRC1[VL-1:0]CASE (imm8[0]) OF0: TMP_DEST[255:0] := SRC2[255:0]1: TMP_DEST[511:256] := SRC2[255:0]ESAC.FOR j := 0 TO 7i := j * 64IF k1[j] OR *no writemask*THEN DEST[i+63:i] := TMP_DEST[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+63:i] := 0FIFI;VINSERTI128TEMP[255:0] := SRC1[255:0]CASE (imm8[0]) OF0: TEMP[127:0] := SRC2[127:0]1: TEMP[255:128] := SRC2[127:0]ESACDEST := TEMPIntel C/C++ Compiler Intrinsic EquivalentVINSERTI32x4 _mm512i _inserti32x4( __m512i a, __m128i b, int imm);VINSERTI32x4 _mm512i _mask_inserti32x4(__m512i s, __mmask16 k, __m512i a, __m128i b, int imm);VINSERTI32x4 _mm512i _maskz_inserti32x4( __mmask16 k, __m512i a, __m128i b, int imm);VINSERTI32x4 __m256i _mm256_inserti32x4( __m256i a, __m128i b, int imm);VINSERTI32x4 __m256i _mm256_mask_inserti32x4(__m256i s, __mmask8 k, __m256i a, __m128i b, int imm);VINSERTI32x4 __m256i _mm256_maskz_inserti32x4( __mmask8 k, __m256i a, __m128i b, int imm);VINSERTI32x8 __m512i _mm512_inserti32x8( __m512i a, __m256i b, int imm);VINSERTI32x8 __m512i _mm512_mask_inserti32x8(__m512i s, __mmask16 k, __m512i a, __m256i b, int imm);VINSERTI32x8 __m512i _mm512_maskz_inserti32x8( __mmask16 k, __m512i a, __m256i b, int imm);VINSERTI64x2 __m512i _mm512_inserti64x2( __m512i a, __m128i b, int imm);VINSERTI64x2 __m512i _mm512_mask_inserti64x2(__m512i s, __mmask8 k, __m512i a, __m128i b, int imm);VINSERTI64x2 __m512i _mm512_maskz_inserti64x2( __mmask8 k, __m512i a, __m128i b, int imm);VINSERTI64x2 __m256i _mm256_inserti64x2( __m256i a, __m128i b, int imm);VINSERTI64x2 __m256i _mm256_mask_inserti64x2(__m256i s, __mmask8 k, __m256i a, __m128i b, int imm);VINSERTI64x2 __m256i _mm256_maskz_inserti64x2( __mmask8 k, __m256i a, __m128i b, int imm);VINSERTI64x4 _mm512_inserti64x4( __m512i a, __m256i b, int imm);VINSERTI64x4 _mm512_mask_inserti64x4(__m512i s, __mmask8 k, __m512i a, __m256i b, int imm);VINSERTI64x4 _mm512_maskz_inserti64x4( __mmask m, __m512i a, __m256i b, int imm);VINSERTI128 __m256i _mm256_insertf128_si256 (__m256i a, __m128i b, int offset);
```

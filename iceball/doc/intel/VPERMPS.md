# VPERMPS

Permute Single Precision Floating-Point Elements

Copies doubleword elements of single-precision floating-point values from the second source operand (the third operand) to the destination operand (the first operand) according to the indices in the first source operand (the second operand).
Note that this instruction permits a doubleword in the source operand to be copied to more than one location in the destination operand.VEX.256 versions: The first and second operands are YMM registers, the third operand can be a YMM register or memory location.
Bits (MAXVL-1:256) of the corresponding destination register are zeroed.EVEX encoded version: The first and second operands are ZMM registers, the third operand can be a ZMM register, a 512-bit memory location or a 512-bit vector broadcasted from a 32-bit memory location.
The elements in the destination are updated using the writemask k1.If VPERMPS is encoded with VEX.L= 0, an attempt to execute the instruction encoded with VEX.L= 0 will cause an #UD exception.

## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."
  > Additionally:
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
VPERMPS (EVEX forms)(KL, VL) (8, 256),= (16, 512)FOR j := 0 TO KL-1i := j * 64IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN TMP_SRC2[i+31:i] := SRC2[31:0];ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i];FI;ENDFOR;IF VL = 256TMP_DEST[31:0] := (TMP_SRC2[255:0] >> (SRC1[2:0] * 32))[31:0];TMP_DEST[63:32] := (TMP_SRC2[255:0] >> (SRC1[34:32] * 32))[31:0];TMP_DEST[95:64] := (TMP_SRC2[255:0] >> (SRC1[66:64] * 32))[31:0];TMP_DEST[127:96] := (TMP_SRC2[255:0] >> (SRC1[98:96] * 32))[31:0];TMP_DEST[159:128] := (TMP_SRC2[255:0] >> (SRC1[130:128] * 32))[31:0];TMP_DEST[255:224] := (TMP_SRC2[255:0] >> (SRC1[226:224] * 32))[31:0];FI;IF VL = 512TMP_DEST[31:0] := (TMP_SRC2[511:0] >> (SRC1[3:0] * 32))[31:0];TMP_DEST[63:32] := (TMP_SRC2[511:0] >> (SRC1[35:32] * 32))[31:0];TMP_DEST[95:64] := (TMP_SRC2[511:0] >> (SRC1[67:64] * 32))[31:0];TMP_DEST[127:96] := (TMP_SRC2[511:0] >> (SRC1[99:96] * 32))[31:0];TMP_DEST[159:128] := (TMP_SRC2[511:0] >> (SRC1[131:128] * 32))[31:0];TMP_DEST[191:160] := (TMP_SRC2[511:0] >> (SRC1[163:160] * 32))[31:0];TMP_DEST[223:192] := (TMP_SRC2[511:0] >> (SRC1[195:192] * 32))[31:0];TMP_DEST[255:224] := (TMP_SRC2[511:0] >> (SRC1[227:224] * 32))[31:0];TMP_DEST[287:256] := (TMP_SRC2[511:0] >> (SRC1[259:256] * 32))[31:0];TMP_DEST[319:288] := (TMP_SRC2[511:0] >> (SRC1[291:288] * 32))[31:0];TMP_DEST[351:320] := (TMP_SRC2[511:0] >> (SRC1[323:320] * 32))[31:0];TMP_DEST[383:352] := (TMP_SRC2[511:0] >> (SRC1[355:352] * 32))[31:0];TMP_DEST[415:384] := (TMP_SRC2[511:0] >> (SRC1[387:384] * 32))[31:0];TMP_DEST[447:416] := (TMP_SRC2[511:0] >> (SRC1[419:416] * 32))[31:0];TMP_DEST[479:448] :=(TMP_SRC2[511:0] >> (SRC1[451:448] * 32))[31:0];TMP_DEST[511:480] := (TMP_SRC2[511:0] >> (SRC1[483:480] * 32))[31:0];FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := TMP_DEST[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE ; zeroing-maskingDEST[i+31:i] := 0;zeroing-maskingFI;FI;ENDFORDEST[MAXVL-1:VL] := 0VPERMPS (VEX.256 encoded version)DEST[31:0] := (SRC2[255:0] >> (SRC1[2:0] * 32))[31:0];DEST[63:32] := (SRC2[255:0] >> (SRC1[34:32] * 32))[31:0];DEST[95:64] := (SRC2[255:0] >> (SRC1[66:64] * 32))[31:0];DEST[127:96] := (SRC2[255:0] >> (SRC1[98:96] * 32))[31:0];DEST[159:128] := (SRC2[255:0] >> (SRC1[130:128] * 32))[31:0];DEST[191:160] := (SRC2[255:0] >> (SRC1[162:160] * 32))[31:0];DEST[223:192] := (SRC2[255:0] >> (SRC1[194:192] * 32))[31:0];DEST[255:224] := (SRC2[255:0] >> (SRC1[226:224] * 32))[31:0];DEST[MAXVL-1:256] := 0Intel C/C++ Compiler Intrinsic EquivalentVPERMPS __m512 _mm512_permutexvar_ps(__m512i i, __m512 a);VPERMPS __m512 _mm512_mask_permutexvar_ps(__m512 s, __mmask16 k, __m512i i, __m512 a);VPERMPS __m512 _mm512_maskz_permutexvar_ps( __mmask16 k, __m512i i, __m512 a);VPERMPS __m256 _mm256_permutexvar_ps(__m256 i, __m256 a);VPERMPS __m256 _mm256_mask_permutexvar_ps(__m256 s, __mmask8 k, __m256 i, __m256 a);
```

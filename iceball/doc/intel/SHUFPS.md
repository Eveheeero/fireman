# SHUFPS

Packed Interleave Shuffle of Quadruplets of Single Precision Floating-Point Values

Selects a single precision floating-point value of an input quadruplet using a two-bit control and move to a desig-nated element of the destination operand.
Each 64-bit element-pair of a 128-bit lane of the destination operand is interleaved between the corresponding lane of the first source operand and the second source operand at the gran-ularity 128 bits.
Each two bits in the imm8 byte, starting from bit 0, is the select control of the corresponding element of a 128-bit lane of the destination to received the shuffled result of an input quadruplet.
The two lower elements of a 128-bit lane in the destination receives shuffle results from the quadruple of the first source operand.
The next two elements of the destination receives shuffle results from the quadruple of the second source operand.
EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 32-bit memory location.
The destination operand is a ZMM/YMM/XMM register updated according to the writemask.
imm8[7:0] provides 4 select controls for each applicable 128-bit lane of the destination.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
Imm8[7:0] provides 4 select controls for the high and low 128-bit of the destination.VEX.128 encoded version: The first source operand is a XMM register.
The second source operand can be a XMM register or a 128-bit memory location.
The destination operand is a XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.
Imm8128-bit Legacy SSE version: The source can be an XMM register or an 128-bit memory location.
The destination is not distinct from the first source XMM register and the upper bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.
Imm8[7:0] provides 4 select controls for each element of the destination.X7X6X5X4X3X2X1X0SRC1Y7Y6Y5Y4Y3Y2Y1Y0SRC2DESTY7 ..
Y4Y7 ..
Y4X7 ..
X4X7 ..
X4Y3 ..Y0Y3 ..Y0X3 ..
X0X3 ..
X0Figure 4-26.
 256-bit VSHUFPS Operation of Selection from Input Quadruplet and Pair-wise Interleaved Result

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
Select4(SRC, control) {CASE (control[1:0]) OF0: TMP := SRC[31:0];1: TMP := SRC[63:32];2: TMP := SRC[95:64];3: TMP := SRC[127:96];ESAC;RETURN TMP}VPSHUFPS (EVEX Encoded Versions When SRC2 is a Vector Register)(KL, VL) = (4, 128), (8, 256), (16, 512)TMP_DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);TMP_DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);TMP_DEST[95:64] := Select4(SRC2[127:0], imm8[5:4]);TMP_DEST[127:96] := Select4(SRC2[127:0], imm8[7:6]);IF VL >= 256TMP_DEST[159:128] := Select4(SRC1[255:128], imm8[1:0]);TMP_DEST[191:160] := Select4(SRC1[255:128], imm8[3:2]);TMP_DEST[223:192] := Select4(SRC2[255:128], imm8[5:4]);TMP_DEST[255:224] := Select4(SRC2[255:128], imm8[7:6]);FI;IF VL >= 512TMP_DEST[287:256] := Select4(SRC1[383:256], imm8[1:0]);TMP_DEST[319:288] := Select4(SRC1[383:256], imm8[3:2]);TMP_DEST[351:320] := Select4(SRC2[383:256], imm8[5:4]);TMP_DEST[383:352] := Select4(SRC2[383:256], imm8[7:6]);TMP_DEST[415:384] := Select4(SRC1[511:384], imm8[1:0]);TMP_DEST[447:416] := Select4(SRC1[511:384], imm8[3:2]);TMP_DEST[479:448] := Select4(SRC2[511:384], imm8[5:4]);TMP_DEST[511:480] := Select4(SRC2[511:384], imm8[7:6]);FI;FOR j := 0 TO KL-1THEN DEST[i+31:i] := TMP_DEST[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+31:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VPSHUFPS (EVEX Encoded Versions When SRC2 is Memory)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF (EVEX.b = 1) THEN TMP_SRC2[i+31:i] := SRC2[31:0]ELSE TMP_SRC2[i+31:i] := SRC2[i+31:i]FI;ENDFOR;TMP_DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);TMP_DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);TMP_DEST[95:64] := Select4(TMP_SRC2[127:0], imm8[5:4]);TMP_DEST[127:96] := Select4(TMP_SRC2[127:0], imm8[7:6]);IF VL >= 256TMP_DEST[159:128] := Select4(SRC1[255:128], imm8[1:0]);TMP_DEST[191:160] := Select4(SRC1[255:128], imm8[3:2]);TMP_DEST[223:192] := Select4(TMP_SRC2[255:128], imm8[5:4]);TMP_DEST[255:224] := Select4(TMP_SRC2[255:128], imm8[7:6]);FI;IF VL >= 512TMP_DEST[287:256] := Select4(SRC1[383:256], imm8[1:0]);TMP_DEST[319:288] := Select4(SRC1[383:256], imm8[3:2]);TMP_DEST[351:320] := Select4(TMP_SRC2[383:256], imm8[5:4]);TMP_DEST[383:352] := Select4(TMP_SRC2[383:256], imm8[7:6]);TMP_DEST[415:384] := Select4(SRC1[511:384], imm8[1:0]);TMP_DEST[447:416] := Select4(SRC1[511:384], imm8[3:2]);TMP_DEST[479:448] := Select4(TMP_SRC2[511:384], imm8[5:4]);TMP_DEST[511:480] := Select4(TMP_SRC2[511:384], imm8[7:6]);FI;FOR j := 0 TO KL-1i := j * 32IF k1[j] OR *no writemask*THEN DEST[i+31:i] := TMP_DEST[i+31:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+31:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+31:i] := 0FIFI;VSHUFPS (VEX.256 Encoded Version)DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);DEST[95:64] := Select4(SRC2[127:0], imm8[5:4]);DEST[127:96] := Select4(SRC2[127:0], imm8[7:6]);DEST[159:128] := Select4(SRC1[255:128], imm8[1:0]);DEST[191:160] := Select4(SRC1[255:128], imm8[3:2]);DEST[223:192] := Select4(SRC2[255:128], imm8[5:4]);DEST[255:224] := Select4(SRC2[255:128], imm8[7:6]);DEST[MAXVL-1:256] := 0VSHUFPS (VEX.128 Encoded Version)DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);DEST[95:64] := Select4(SRC2[127:0], imm8[5:4]);DEST[127:96] := Select4(SRC2[127:0], imm8[7:6]);DEST[MAXVL-1:128] := 0SHUFPS (128-bit Legacy SSE Version)DEST[31:0] := Select4(SRC1[127:0], imm8[1:0]);DEST[63:32] := Select4(SRC1[127:0], imm8[3:2]);DEST[95:64] := Select4(SRC2[127:0], imm8[5:4]);DEST[127:96] := Select4(SRC2[127:0], imm8[7:6]);DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVSHUFPS __m512 _mm512_shuffle_ps(__m512 a, __m512 b, int imm);VSHUFPS __m512 _mm512_mask_shuffle_ps(__m512 s, __mmask16 k, __m512 a, __m512 b, int imm);VSHUFPS __m512 _mm512_maskz_shuffle_ps(__mmask16 k, __m512 a, __m512 b, int imm);VSHUFPS __m256 _mm256_shuffle_ps (__m256 a, __m256 b, const int select);VSHUFPS __m256 _mm256_mask_shuffle_ps(__m256 s, __mmask8 k, __m256 a, __m256 b, int imm);VSHUFPS __m256 _mm256_maskz_shuffle_ps(__mmask8 k, __m256 a, __m256 b, int imm);SHUFPS __m128 _mm_shuffle_ps (__m128 a, __m128 b, const int select);VSHUFPS __m128 _mm_mask_shuffle_ps(__m128 s, __mmask8 k, __m128 a, __m128 b, int imm);VSHUFPS __m128 _mm_maskz_shuffle_ps(__mmask8 k, __m128 a, __m128 b, int imm);
```

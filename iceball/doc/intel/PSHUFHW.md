# PSHUFHW

Shuffle Packed High Words

Copies words from the high quadword of a 128-bit lane of the source operand and inserts them in the high quad-word of the destination operand at word locations (of the respective lane) selected with the immediate operand.
This 256-bit operation is similar to the in-lane operation used by the 256-bit VPSHUFD instruction, which is illus-trated in Figure 4-16.
For 128-bit operation, only the low 128-bit lane is operative.
Each 2-bit field in the immediate operand selects the contents of one word location in the high quadword of the destination operand.
The binary encodings of the immediate operand fields select words (0, 1, 2 or 3, 4) from the high quadword of the source operand to be copied to the destination operand.
The low quadword of the source operand is copied to the low quadword of the destination operand, for each 128-bit lane.Note that this instruction permits a word in the high quadword of the source operand to be copied to more than one word location in the high quadword of the destination operand.In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).128-bit Legacy SSE version: The destination operand is an XMM register.
The source operand can be an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: The destination operand is an XMM register.
The source operand can be an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.vvvv is reserved and must be 1111b, VEX.L must be 0, otherwise the instruction will #UD.VEX.256 encoded version: The destination operand is an YEVEX encoded version: The destination operand is a ZMM/YMM/XMM registers.
The source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location.
The destination is updated according to the write-mask.Note: In VEX encoded versions, VEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."
  > EVEX-encoded instruction, see Exceptions Type E4NF.n
  > b in Table2-50, "Type E4NF Class Exception Conditions."

## Operation

```C
PSHUFHW (128-bit Legacy SSE Version)DEST[63:0] := SRC[63:0]DEST[79:64] := (SRC >> (imm[1:0] *16))[79:64]DEST[95:80] := (SRC >> (imm[3:2] * 16))[79:64]DEST[111:96] := (SRC >> (imm[5:4] * 16))[79:64]DEST[127:112] := (SRC >> (imm[7:6] * 16))[79:64]DEST[MAXVL-1:128] (Unmodified)VPSHUFHW (VEX.128 Encoded Version)DEST[63:0] := SRC1[63:0]DEST[79:64] := (SRC1 >> (imm[1:0] *16))[79:64]DEST[95:80] := (SRC1 >> (imm[3:2] * 16))[79:64]DEST[111:96] := (SRC1 >> (imm[5:4] * 16))[79:64]DEST[127:112] := (SRC1 >> (imm[7:6] * 16))[79:64]DEST[MAXVL-1:128] := 0VPSHUFHW (VEX.256 Encoded Version)DEST[63:0] := SRC1[63:0]DEST[79:64] := (SRC1 >> (imm[1:0] *16))[79:64]DEST[95:80] := (SRC1 >> (imm[3:2] * 16))[79:64]DEST[111:96] := (SRC1 >> (imm[5:4] * 16))[79:64]DEST[127:112] := (SRC1 >> (imm[7:6] * 16))[79:64]DEST[191:128] := SRC1[191:128]DEST[207192] := (SRC1 >> (imm[1:0] *16))[207:192]DEST[223:208] := (SRC1 >> (imm[3:2] * 16))[207:192]DEST[239:224] := (SRC1 >> (imm[5:4] * 16))[207:192]DEST[255:240] := (SRC1 >> (imm[7:6] * 16))[207:192]DEST[MAXVL-1:256] := 0VPSHUFHW (EVEX Encoded Versions)(KL, VL) = (8, 128), (16, 256), (32, 512)IF VL >= 128TMP_DEST[63:0] := SRC1[63:0]TMP_DEST[79:64] := (SRC1 >> (imm[1:0] *16))[79:64]TMP_DEST[95:80] := (SRC1 >> (imm[3:2] * 16))[79:64]TMP_DEST[111:96] := (SRC1 >> (imm[5:4] * 16))[79:64]TMP_DEST[127:112] := (SRC1 >> (imm[7:6] * 16))[79:64]FI;IF VL >= 256TMP_DEST[191:128] := SRC1[191:128]TMP_DEST[207:192] := (SRC1 >> (imm[1:0] *16))[207:192]TMP_DEST[223:208] := (SRC1 >> (imm[3:2] * 16))[207:192]TMP_DEST[239:224] := (SRC1 >> (imm[5:4] * 16))[207:192]TMP_DEST[255:240] := (SRC1 >> (imm[7:6] * 16))[207:192]FI;IF VL >= 512TMP_DEST[351:336] := (SRC1 >> (imm[3:2] * 16))[335:320]TMP_DEST[367:352] := (SRC1 >> (imm[5:4] * 16))[335:320]TMP_DEST[383:368] := (SRC1 >> (imm[7:6] * 16))[335:320]TMP_DEST[447:384] := SRC1[447:384]TMP_DEST[463:448] := (SRC1 >> (imm[1:0] *16))[463:448]TMP_DEST[479:464] := (SRC1 >> (imm[3:2] * 16))[463:448]TMP_DEST[495:480] := (SRC1 >> (imm[5:4] * 16))[463:448]TMP_DEST[511:496] := (SRC1 >> (imm[7:6] * 16))[463:448]FI;FOR j := 0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN DEST[i+15:i] := TMP_DEST[i+15:i];ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+15:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0 Intel C/C++ Compiler Intrinsic EquivalentVPSHUFHW __m512i _mm512_shufflehi_epi16(__m512i a, int n);VPSHUFHW __m512i _mm512_mask_shufflehi_epi16(__m512i s, __mmask16 k, __m512i a, int n );VPSHUFHW __m512i _mm512_maskz_shufflehi_epi16( __mmask16 k, __m512i a, int n );VPSHUFHW __m256i _mm256_mask_shufflehi_epi16(__m256i s, __mmask8 k, __m256i a, int n );VPSHUFHW __m256i _mm256_maskz_shufflehi_epi16( __mmask8 k, __m256i a, int n );VPSHUFHW __m128i _mm_mask_shufflehi_epi16(__m128i s, __mmask8 k, __m128i a, int n );VPSHUFHW __m128i _mm_maskz_shufflehi_epi16( __mmask8 k, __m128i a, int n );(V)PSHUFHW __m128i _mm_shufflehi_epi16(__m128i a, int n)VPSHUFHW __m256i _mm256_shufflehi_epi16(__m256i a, const int n)
```

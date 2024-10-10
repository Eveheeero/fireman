# PSHUFLW

Shuffle Packed Low Words

Copies words from the low quadword of a 128-bit lane of the source operand and inserts them in the low quadword of the destination operand at word locations (of the respective lane) selected with the immediate operand.
The 256-bit operation is similar to the in-lane operation used by the 256-bit VPSHUFD instruction, which is illustrated in Figure 4-16.
For 128-bit operation, only the low 128-bit lane is operative.
Each 2-bit field in the immediate operand selects the contents of one word location in the low quadword of the destination operand.
The binary encodings of the immediate operand fields select words (0, 1, 2 or 3) from the low quadword of the source operand to be copied to the destination operand.
The high quadword of the source operand is copied to the high quadword of the destination operand, for each 128-bit lane.Note that this instruction permits a word in the low quadword of the source operand to be copied to more than one word location in the low quadword of the destination operand.In 64-bit mode and not encoded with VEX/EVEX, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).128-bit Legacy SSE version: The destination operand is an XMM register.
The source operand can be an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: The destination operand is an XMM register.
The source operand can be an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.256 encoded version: The destination operand is an YMM register.
The source operand can be an YMM register or a 256-bit memory location.
EVEX encoded version: The destination operand is a ZMM/YMM/XMM registers.
The source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory locationNote: In VEX encoded versions, VEX.vvvv is reserved and must be 1111b otherwise instructions will #UD.

## Flags affected

- None.

## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."
  > EVEX-encoded instruction, see Exceptions Type E4NF.n
  > b in Table2-50, "Type E4NF Class Exception Conditions."
  > Additionally:
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
PSHUFLW (128-bit Legacy SSE Version)DEST[15:0] := (SRC >> (imm[1:0] *16))[15:0]DEST[31:16] := (SRC >> (imm[3:2] * 16))[15:0]DEST[47:32] := (SRC >> (imm[5:4] * 16))[15:0]DEST[63:48] := (SRC >> (imm[7:6] * 16))[15:0]DEST[127:64] := SRC[127:64]DEST[MAXVL-1:128] (Unmodified)VPSHUFLW (VEX.128 Encoded Version)DEST[15:0] := (SRC1 >> (imm[1:0] *16))[15:0]DEST[31:16] := (SRC1 >> (imm[3:2] * 16))[15:0]DEST[47:32] := (SRC1 >> (imm[5:4] * 16))[15:0]DEST[63:48] := (SRC1 >> (imm[7:6] * 16))[15:0]DEST[127:64] := SRC[127:64]DEST[MAXVL-1:128] := 0VPSHUFLW (VEX.256 Encoded Version)DEST[15:0] := (SRC1 >> (imm[1:0] *16))[15:0]DEST[31:16] := (SRC1 >> (imm[3:2] * 16))[15:0]DEST[47:32] := (SRC1 >> (imm[5:4] * 16))[15:0]DEST[63:48] := (SRC1 >> (imm[7:6] * 16))[15:0]DEST[127:64] := SRC1[127:64]DEST[143:128] := (SRC1 >> (imm[1:0] *16))[143:128]DEST[159:144] := (SRC1 >> (imm[3:2] * 16))[143:128]DEST[175:160] := (SRC1 >> (imm[5:4] * 16))[143:128]DEST[191:176] := (SRC1 >> (imm[7:6] * 16))[143:128]DEST[255:192] := SRC1[255:192]DEST[MAXVL-1:256] := 0VPSHUFLW (EVEX.U1.512 Encoded Version)(KL, VL) = (8, 128), (16, 256), (32, 512)IF VL >= 128TMP_DEST[15:0] := (SRC1 >> (imm[1:0] *16))[15:0]TMP_DEST[31:16] := (SRC1 >> (imm[3:2] * 16))[15:0]TMP_DEST[47:32] := (SRC1 >> (imm[5:4] * 16))[15:0]TMP_DEST[63:48] := (SRC1 >> (imm[7:6] * 16))[15:0]TMP_DEST[127:64] := SRC1[127:64]FI;IF VL >= 256TMP_DEST[143:128] := (SRC1 >> (imm[1:0] *16))[143:128]TMP_DEST[159:144] := (SRC1 >> (imm[3:2] * 16))[143:128]TMP_DEST[175:160] := (SRC1 >> (imm[5:4] * 16))[143:128]TMP_DEST[191:176] := (SRC1 >> (imm[7:6] * 16))[143:128]TMP_DEST[255:192] := SRC1[255:192]FI;IF VL >= 512TMP_DEST[271:256] := (SRC1 >> (imm[1:0] *16))[271:256]TMP_DEST[287:272] := (SRC1 >> (imm[3:2] * 16))[271:256]TMP_DEST[303:288] := (SRC1 >> (imm[5:4] * 16))[271:256]TMP_DEST[399:384] := (SRC1 >> (imm[1:0] *16))[399:384]TMP_DEST[415:400] := (SRC1 >> (imm[3:2] * 16))[399:384]TMP_DEST[431:416] := (SRC1 >> (imm[5:4] * 16))[399:384]TMP_DEST[447:432] := (SRC1 >> (imm[7:6] * 16))[399:384]TMP_DEST[511:448] := SRC1[511:448]FI;FOR j := 0 TO KL-1i := j * 16IF k1[j] OR *no writemask*THEN DEST[i+15:i] := TMP_DEST[i+15:i];ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+15:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+15:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0 Intel C/C++ Compiler Intrinsic EquivalentVPSHUFLW __m512i _mm512_shufflelo_epi16(__m512i a, int n);VPSHUFLW __m512i _mm512_mask_shufflelo_epi16(__m512i s, __mmask16 k, __m512i a, int n );VPSHUFLW __m512i _mm512_maskz_shufflelo_epi16( __mmask16 k, __m512i a, int n );VPSHUFLW __m256i _mm256_mask_shufflelo_epi16(__m256i s, __mmask8 k, __m256i a, int n );VPSHUFLW __m256i _mm256_maskz_shufflelo_epi16( __mmask8 k, __m256i a, int n );VPSHUFLW __m128i _mm_mask_shufflelo_epi16(__m128i s, __mmask8 k, __m128i a, int n );VPSHUFLW __m128i _mm_maskz_shufflelo_epi16( __mmask8 k, __m128i a, int n );(V)PSHUFLW:__m128i _mm_shufflelo_epi16(__m128i a, int n)VPSHUFLW:__m256i _mm256_shufflelo_epi16(__m256i a, const int n)
```

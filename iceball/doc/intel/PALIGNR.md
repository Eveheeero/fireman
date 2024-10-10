# PALIGNR

Packed Align Right

(V)PALIGNR concatenates the destination operand (the first operand) and the source operand (the second operand) into an intermediate composite, shifts the composite at byte granularity to the right by a constant imme-diate, and extracts the right-aligned result into the destination.
The first and the second operands can be an MMX, XMM or a YMM register.
The immediate value is considered unsigned.
Immediate shift counts larger than the 2L (i.e., 32 for 128-bit operands, or 16 for 64-bit operands) produce a zero result.
Both operands can be MMX regis-ters, XMM registers or YMM registers.
When the source In 64-bit mode and not encoded by VEX/EVEX prefix, use the REX prefix to access additional registers.128-bit Legacy SSE version: Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.EVEX.512 encoded version: The first source operand is a ZMM register and contains four 16-byte blocks.
The second source operand is a ZMM register or a 512-bit memory location containing four 16-byte block.
The destina-tion operand is a ZMM register and contain four 16-byte results.
The imm8[7:0] is the common shift countused for each of the four successive 16-byte block sources.
The low 16-byte block of the two source operands produce the low 16-byte result of the destination operand, the high 16-byte block of the two source operands produce the high 16-byte result of the destination operand and so on for the blocks in the middle.VEX.256 and EVEX.256 encoded versions: The first source operand is a YMM register and contains two 16-byte blocks.
The second source operand is a YMM register or a 256-bit memory location containing two 16-byte block.
The destination operand is a YMM register and contain two 16-byte results.
The imm8[7:0] is the common shift count used for the two lower 16-byte block sources and the two upper 16-byte block sources.
The low 16-byte block of the two source operands produce the low 16-byte result of the destination operand, the high 16-byte block of the two source operands produce the high 16-byte result of the destination operand.
The upper bits (MAXVL-1:256) of the corresponding ZMM register destination are zeroed.VEX.128 and EVEX.128 encoded versions: The first source operand is an XMM register.
The second source operand is an XMM register or 128-bit memory location.
The destination operand is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are zeroed.Concatenation is done with 128-bit data in the first and second source operand for both 128-bit and 256-bit instructions.
The high 128-bits of the intermediate composite 256-bit result came from the 128-bit data from the first source operand; the low 128-bits of the intermediate result came from the 128-bit data of the second source operand.Imm8[7:0]*8Imm8[7:0]*81270128SRC2255SRC1255128128SRC2255DEST1270DESTFigure 4-7.
 256-bit VPALIGN Instruction Operation

## Exceptions

- SIMD Floating-Point Exceptions
  > None. 
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."

## Operation

```C
PALIGNR (With 64-bit Operands)temp1[127:0] = CONCATENATE(DEST,SRC)>>(imm8*8) DEST[63:0] = temp1[63:0] PALIGNR (With 128-bit Operands)temp1[255:0] := ((DEST[127:0] << 128) OR SRC[127:0])>>(imm8*8);DEST[127:0] := temp1[127:0]DEST[MAXVL-1:128] (Unmodified)0VPALIGNR (VEX.128 Encoded Version)temp1[255:0] := ((SRC1[127:0] << 128) OR SRC2[127:0])>>(imm8*8);DEST[127:0] := temp1[127:0]DEST[MAXVL-1:128] := 0VPALIGNR (VEX.256 Encoded Version)temp1[255:0] := ((SRC1[127:0] << 128) OR SRC2[127:0])>>(imm8[7:0]*8);DEST[127:0] := temp1[127:0]temp1[255:0] := ((SRC1[255:128] << 128) OR SRC2[255:128])>>(imm8[7:0]*8);DEST[MAXVL-1:128] := temp1[127:0]VPALIGNR (EVEX Encoded Versions)(KL, VL) = (16, 128), (32, 256), (64, 512)FOR l := 0 TO VL-1 with increments of 128temp1[255:0] := ((SRC1[l+127:l] << 128) OR SRC2[l+127:l])>>(imm8[7:0]*8);TMP_DEST[l+127:l] := temp1[127:0]ENDFOR;FOR j := 0 TO KL-1i := j * 8IF k1[j] OR *no writemask*THEN DEST[i+7:i] := TMP_DEST[i+7:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+7:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+7:i] = 0FIFI;ENDFOR;DEST[MAXVL-1:VL] := 0Intel C/C++ Compiler Intrinsic EquivalentsPALIGNR __m64 _mm_alignr_pi8 (__m64 a, __m64 b, int n)(V)PALIGNR __m128i _mm_alignr_epi8 (__m128i a, __m128i b, int n)VPALIGNR __m256i _mm256_alignr_epi8 (__m256i a, __m256i b, const int n)VPALIGNR __m512i _mm512_alignr_epi8 (__m512i a, __m512i b, const int n)VPALIGNR __m512i _mm512_mask_alignr_epi8 (__m512i s, __mmask64 m, __m512i a, __m512i b, const int n)VPALIGNR __m512i _mm512_maskz_alignr_epi8 ( __mmask64 m, __m512i a, __m512i b, const int n)VPALIGNR __m256i _mm256_mask_alignr_epi8 (__m256i s, __mmask32 m, __m256i a, __m256i b, const int n)VPALIGNR __m256i _mm256_maskz_alignr_epi8 (__mmask32 m, __m256i a, __m256i b, const int n)VPALIGNR __m128i _mm_mask_alignr_epi8 (__m128i s, __mmask16 m, __m128i a, __m128i b, const int n)VPALIGNR __m128i _mm_maskz_alignr_epi8 (__mmask16 m, __m128i a, __m128i b, const int n)
```

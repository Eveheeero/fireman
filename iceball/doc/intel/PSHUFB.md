# PSHUFB

Packed Shuffle Bytes

PSHUFB performs in-place shuffles of bytes in the destination operand (the first operand) according to the shuffle control mask in the source operand (the second operand).
The instruction permutes the data in the destination operand, leaving the shuffle mask unaffected.
If the most significant bit (bit[7]) of each byte of the shuffle control mask is set, then constant zero is written in the result byte.
Each byte in the shuffle control mask forms an index to permute the corresponding byte in the destination operand.
The value of each index is the least significant 4 bits (128-bit operation) or 3 bits (64-bit operation) of the shuffle control byte.
When the source operand is a 128-bit memory operand, the operand must be aligned on a 16-byte boundary or a general-protection exception (#GP) will be generated.
In 64-bit mode and not encoded with VEX/EVEX, use the REX prefix to access XMM8-XMM15 registers.
Legacy SSE version 64-bit operand: Both operands can be MMX registers.128-bit Legacy SSE version: The first source operand and the destination operand are the same.
Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: The destination operand is the first operand, the first source operand is the second operand, the second source operand is the third operand.
Bits (MAXVL-1:128) of the destination YMM register are zeroed.
VEX.256 encoded version: Bits (255:128) of the destination YMM register stores the 16-byte shuffle result of the The value of each index is for the high 128-bit lane is the least significant 4 bits of the respective shuffle control byte.
The index value selects a source data element within each 128-bit lane.EVEX encoded version: The second source operand is an ZMM/YMM/XMM register or an 512/256/128-bit memory location.
The first source operand and destination operands are ZMM/YMM/XMM registers.
The destination is condi-tionally updated with writemask k1.EVEX and VEX encoded version: Four/two in-lane 128-bit shuffles.

## Exceptions

- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."

## Operation

```C
PSHUFB (With 64-bit Operands)TEMP := DESTfor i = 0 to 7 { if (SRC[(i * 8)+7] = 1 ) thenDEST[(i*8)+7...(i*8)+0] := 0;else index[2..0] := SRC[(i*8)+2 .. (i*8)+0];DEST[(i*8)+7...(i*8)+0] := TEMP[(index*8+7)..(index*8+0)];endif;}PSHUFB (with 128 bit operands)TEMP := DESTfor i = 0 to 15 { if (SRC[(i * 8)+7] = 1 ) thenDEST[(i*8)+7..(i*8)+0] := 0; else index[3..0] := SRC[(i*8)+3 .. (i*8)+0];DEST[(i*8)+7..(i*8)+0] := TEMP[(index*8+7)..(index*8+0)];endif}VPSHUFB (VEX.128 Encoded Version)for i = 0 to 15 {if (SRC2[(i * 8)+7] = 1) thenDEST[(i*8)+7..(i*8)+0] := 0;elseindex[3..0] := SRC2[(i*8)+3 .. (i*8)+0];DEST[(i*8)+7..(i*8)+0] := SRC1[(index*8+7)..(index*8+0)];endif}DEST[MAXVL-1:128] := 0VPSHUFB (VEX.256 Encoded Version)for i = 0 to 15 {if (SRC2[(i * 8)+7] == 1 ) thenDEST[(i*8)+7..(i*8)+0] := 0;elseindex[3..0] := SRC2[(i*8)+3 .. (i*8)+0];DEST[(i*8)+7..(i*8)+0] := SRC1[(index*8+7)..(index*8+0)];endifif (SRC2[128 + (i * 8)+7] == 1 ) thenDEST[128 + (i*8)+7..(i*8)+0] := 0;elseindex[3..0] := SRC2[128 + (i*8)+3 .. (i*8)+0];DEST[128 + (i*8)+7..(i*8)+0] := SR}VPSHUFB (EVEX Encoded Versions)(KL, VL) = (16, 128), (32, 256), (64, 512)jmask := (KL-1) & ~0xF // 0x00, 0x10, 0x30 depending on the VLFOR j = 0 TO KL-1// destIF kl[ i ] or no_maskingindex := src.byte[ j ];IF index & 0x80Dest.byte[ j ] := 0;ELSEindex := (index & 0xF) + (j & jmask);// 16-element in-lane lookupDest.byte[ j ] := src.byte[ index ];ELSE if zeroingDest.byte[ j ] := 0;DEST[MAXVL-1:VL] := 0;MM207H         07H              FFH               80H               01H           00H               00H            00HMM104H         01H              07H               03H               02H           02H               FFH            01HMM104H         04H              00H               00H               FFH           01H               01H            01HFigure 4-15.  PSHUFB with 64-Bit OperandsIntel C/C++ Compiler Intrinsic EquivalentVPSHUFB __m512i _mm512_shuffle_epi8(__m512i a, __m512i b);VPSHUFB __m512i _mm512_mask_shuffle_epi8(__m512i s, __mmask64 k, __m512i a, __m512i b);VPSHUFB __m512i _mm512_maskz_shuffle_epi8( __mmask64 k, __m512i a, __m512i b);VPSHUFB __m256i _mm256_mask_shuffle_epi8(__m256i s, __mmask32 k, __m256i a, __m256i b);VPSHUFB __m256i _mm256_maskz_shuffle_epi8( __mmask32 k, __m256i a, __m256i b);VPSHUFB __m128i _mm_mask_shuffle_epi8(__m128i s, __mmask16 k, __m128i a, __m128i b);VPSHUFB __m128i _mm_maskz_shuffle_epi8( __mmask16 k, __m128i a, __m128i b);PSHUFB: __m64 _mm_shuffle_pi8 (__m64 a, __m64 b)(V)PSHUFB: __m128i _mm_shuffle_epi8 (__m128i a, __m128i b)VPSHUFB:__m256i _mm256_shuffle_epi8(__m256i a, __m256i b)
```

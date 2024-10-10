# AESENCLAST

Perform Last Round of an AES Encryption Flow

This instruction performs the last round of an AES encryption flow using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
VEX and EVEX encoded versions of the instruction allows 3-operand (non-destructive) operation.
The legacy encoded versions of the instruction require that the first source operand and the destination operand are the same and must be an XMM register.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
AESENCLAST STATE := SRC1;RoundKey := SRC2;STATE := ShiftRows( STATE );STATE := SubBytes( STATE );DEST[127:0] := STATE XOR RoundKey;DEST[MAXVL-1:128] (Unmodified)VAESENCLAST (128b and 256b VEX Encoded Versions)(KL, VL) = (1,128), (2,256)FOR I=0 to KL-1::= SRC1.xmm[i]STATE := RoundKey SRC2.xmm[i]:= STATE ShiftRows( STATE ):= STATE SubBytes( STATE ):= DEST.xmm[i] STATE XOR RoundKey:= DEST[MAXVL-1:VL] 0VAESENCLAST (EVEX Encoded Version)(KL,VL) = (1,128), (2,256), (4,512)FOR i = 0 to KL-1::= STATE SRC1.xmm[i]:= RoundKey SRC2.xmm[i]:= STATE ShiftRows( STATE ):= STATE SubBytes( STATE ):= DEST.xmm[i] STATE XOR RoundKey:= DEST[MAXVL-1:VL] 0Intel C/C++ Compiler Intrinsic Equivalent(V)AESENCLAST __m128i _mm_aesenclast (__m128i, __m128i)VAESENCLAST __m256i _mm256_aesenclast_epi128(__m256i, __m256i);VAESENCLAST __m512i _mm512_aesenclast_epi128(__m512i, __m512i);
```

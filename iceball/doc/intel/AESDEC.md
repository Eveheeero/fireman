# AESDEC

Perform One Round of an AES Decryption Flow

This instruction performs a single round of the AES decryption flow using the Equivalent Inverse Cipher, using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destina-tion operand.
Use the AESDEC instruction for all but the last decryption round.
For the last decryption round, use the AESDE-CLAST instruction.VEX and EVEX encoded versions of the instruction allow 3-operand (non-destructive) operation.
The legacy encoded versions of the instruction require that the first source operand and the destination operand are the same and must be an XMM register.

## Exceptions

- Other Exceptions
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
AESDEC STATE := SRC1;RoundKey := SRC2;STATE := InvShiftRows( STATE );STATE := InvSubBytes( STATE );STATE := InvMixColumns( STATE );DEST[127:0] := STATE XOR RoundKey;DEST[MAXVL-1:128] (Unmodified)VAESDEC (128b and 256b VEX Encoded Versions)(KL,VL) = (1,128), (2,256)FOR i = 0 to KL-1::= SRC1.xmm[i]STATE := RoundKey SRC2.xmm[i]:= STATE InvShiftRows( STATE ):= STATE InvSubBytes( STATE ):= STATE InvMixColumns( STATE ):= DEST.xmm[i] STATE XOR RoundKey:= DEST[MAXVL-1:VL] 0VAESDEC (EVEX Encoded Version)(KL,VL) = (1,128), (2,256), (4,512)FOR i = 0 to KL-1::= STATE SRC1.xmm[i]:= RoundKey SRC2.xmm[i]:= STATE InvShiftRows( STATE ):= STATE InvSubBytes( STATE ):= STATE InvMixColumns( STATE ):= DEST.xmm[i] STATE XOR RoundKey:=DEST[MAXVL-1:VL] 0Intel C/C++ Compiler Intrinsic Equivalent(V)AESDEC __m128i _mm_aesdec (__m128i, __m128i)VAESDEC __m256i _mm256_aesdec_epi128(__m256i, __m256i);VAESDEC __m512i _mm512_aesdec_epi128(__m512i, __m512i);
```

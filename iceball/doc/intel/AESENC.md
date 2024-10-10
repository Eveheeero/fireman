# AESENC

Perform One Round of an AES Encryption Flow

This instruction performs a single round of an AES encryption flow using one/two/four (depending on vector length) 128-bit data (state) from the first source operand with one/two/four (depending on vector length) round key(s) from the second source operand, and stores the result in the destination operand.
Use the AESENC instruction for all but the last encryption rounds.
For the last encryption round, use the AESENC-CLAST instruction.VEX and EVEX encoded versions of the instruction allow 3-operand (non-destructive) operation.
The legacy encoded versions of the instruction require that the first source operand and the destination operand are the same and must be an XMM register.The EVEX encoded form of this instruction does not support memory fault suppression.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
AESENC STATE := SRC1;RoundKey := SRC2;STATE := ShiftRows( STATE );STATE := SubBytes( STATE );STATE := MixColumns( STATE );DEST[127:0] := STATE XOR RoundKey;VAESENC (128b and 256b VEX Encoded Versions)(KL,VL) = (1,128), (2,256):= 0 to KL-1:FOR I := STATE SRC1.xmm[i]:= RoundKey SRC2.xmm[i]:= STATE ShiftRows( STATE ):= STATE SubBytes( STATE ):= STATE MixColumns( STATE ):= DEST.xmm[i] STATE XOR RoundKey:= DEST[MAXVL-1:VL] 0VAESENC (EVEX Encoded Version)(KL,VL) = (1,128), (2,256), (4,512):= FOR i 0 to KL-1::= STATE SRC1.xmm[i] // xmm[i] is the i'th xmm word in the SIMD register:= RoundKey SRC2.xmm[i]:= STATE ShiftRows( STATE ):= STATE SubBytes( STATE ):= STATE MixColumns( STATE ):= DEST.xmm[i] STATE XOR RoundKey:= DEST[MAXVL-1:VL] 0Intel C/C++ Compiler Intrinsic Equivalent(V)AESENC __m128i _mm_aesenc (__m128i, __m128i)VAESENC __m256i _mm256_aesenc_epi128(__m256i, __m256i);VAESENC __m512i _mm512_aesenc_epi128(__m512i, __m512i);
```

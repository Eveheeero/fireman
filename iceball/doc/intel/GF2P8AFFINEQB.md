# GF2P8AFFINEQB

Galois Field Affine Transformation

8.
For this instruction, an affine The AFFINEB instruction computes an affine transformation in the Galois Field 2transformation is defined by A * x + b where "A" is an 8 by 8 bit matrix, and "x" and "b" are 8-bit vectors.
One SIMD register (operand 1) holds "x" as either 16, 32 or 64 8-bit vectors.
A second SIMD (operand 2) register or memory operand contains 2, 4, or 8 "A" values, which are operated upon by the correspondingly aligned 8 "x" values in the first register.
The "b" vector is constant for all calculations and contained in the immediate byte.The EVEX encoded form of this instruction does not support memory fault suppression.
The SSE encoded forms of the instruction require16B alignment on their memory operations.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Legacy-encoded and VEX-encoded: See Table2

## Operation

```C
define parity(x)::= 0 // single bitt := FOR i 0 to 7:t = t xor x.bit[i]return tdefine affine_byte(tsrc2qw, src1byte, imm)::= FOR i 0 to 7:* parity(x) = 1 if x has an odd number of 1s in it, and 0 otherwise.*:= retbyte.bit[i] VGF2P8AFFINEQB dest, src1, src2, imm8 (EVEX Encoded Version)(KL, VL) = (2, 128), (4, 256), (8, 512):= FOR j 0 TO KL-1:IF SRC2 is memory and EVEX.b==1::= tsrc2 SRC2.qword[0]ELSE::= tsrc2 SRC2.qword[j]:= 0 to 7:FOR b IF k1[j*8+b] OR *no writemask*::= DEST.qword[j].byte[b] affine_byte(tsrc2, SRC1.qword[j].byte[b], imm8)ELSE IF *zeroing*::= DEST.qword[j].byte[b] 0*ELSE DEST.qword[j].byte[b] remains unchanged*:= DEST[MAX_VL-1:VL] 0VGF2P8AFFINEQB dest, src1, src2, imm8 (128b and 256b VEX Encoded Versions)(KL, VL) = (2, 128), (4, 256):= 0 TO KL-1:FOR j := FOR b 0 to 7::= DEST.qword[j].byte[b] affine_byte(SRC2.qword[j], SRC1.qword[j].byte[b], imm8):= DEST[MAX_VL-1:VL] 0GF2P8AFFINEQB srcdest, src1, imm8 (128b SSE Encoded Version):= FOR j 0 TO 1::= FOR b 0 to 7::= SRCDEST.qword[j].byte[b] affine_byte(SRC1.qword[j], SRCDEST.qword[j].byte[b], imm8)Intel C/C++ Compiler Intrinsic Equivalent(V)GF2P8AFFINEQB __m128i _mm_gf2p8affine_epi64_epi8(__m128i, __m128i, int);(V)GF2P8AFFINEQB __m128i _mm_mask_gf2p8affine_epi64_epi8(__m128i, __mmask16, __m128i, __m128i, int);(V)GF2P8AFFINEQB __m128i _mm_maskz_gf2p8affine_epi64_epi8(__mmask16, __m128i, __m128i, int);VGF2P8AFFINEQB __m256i _mm256_gf2p8affine_epi64_epi8(__m256i, __m256i, int);VGF2P8AFFINEQB __m256i _mm256_mask_gf2p8affine_epi64_epi8(__m256i, __mmask32, __m256i, __m256i, int);VGF2P8AFFINEQB __m256i _mm256_maskz_gf2p8affine_epi64_epi8(__mmask32, __m256i, __m256i, int);VGF2P8AFFINEQB __m512i _mm512_gf2p8affine_epi64_epi8(__m512i, __m512i, int);VGF2P8AFFINEQB __m512i _mm512_mask_gf2p8affine_epi64_epi8(__m512i, __mmask64, __m512i, __m512i, int);VGF2P8AFFINEQB __m512i _mm512_maskz_gf2p8affine_epi64_epi8(__mmask64, __m512i, __m512i, int);
```

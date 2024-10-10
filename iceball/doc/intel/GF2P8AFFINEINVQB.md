# GF2P8AFFINEINVQB

Galois Field Affine Transformation Inverse

8.
For this instruction, an affine The AFFINEINVB instruction computes an affine transformation in the Galois Field 2transformation is defined by A * inv(x) + b where "A" is an 8 by 8 bit matrix, and "x" and "b" are 8-bit vectors.
The 843 + x + x + x + 1.inverse of the bytes in x is defined with respect to the reduction polynomial xOne SIMD register (operand 1) holds "x" as either 16, 32 or 64 8-bit vectors.
A second SIMD (operand 2) register or memory operand contains 2, 4, or 8 "A" values, which are operated upon by the correspondingly aligned 8 "x" values in the first register.
The "b" vector is constant for all calculations and contained in the immediate byte.The EVEX encoded form of this instruction does not support memory fault suppression.
The SSE encoded forms of the instruction require 16B alignment on their memory operations.The inverse of each byte is given by the following table.
The upper nibble is onTable 3-50.
 Inverse Byte Listings-0123456789ABCDEF0018DF6CB527BD1E84F29C0B0E1E5C7174B4AA4B992B605F583FFDCCFF40EEB223A6E5AF1554DA8C9C1A98153044A2C232C45926CF3396642F235206F77BB591941DFE37672D31F569A764AB135425E995ED5C5CA4C2487BF183E22F051EC61176165EAFD349A63643F44791DF3393213B779B7978510B5BA3CB670D06A1FA81828837E7F809673BE569B9E95D9F72B9A49DE6A326DD88A84722A149F88F9DC899AAFB7C2EC38FB8654826C8124ACEE7D262BCE01FEF11757871A58E763DBDBC8657CB282FA3DAD4E4FA9275341BFCACE6D7A7AE63C5DBE2EA948BC4D59DF8906BEB1DD6EBC6ECFAD84ED7E35D501EB3F5B233834684638CDD9C7DA0CD1A411C

## Exceptions

- Other Exceptions
  > Legacy-encoded and VEX-encoded: See Table2
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
define affine_inverse_byte(tsrc2qw, src1byte, imm)::= FOR i 0 to 7:* parity(x) = 1 if x has an odd number of 1s in it, and 0 otherwise.** inverse(x) is defined in the table above *:= parity(tsrc2qw.byte[7-i] AND inverse(src1byte)) XOR imm8.bit[i]retbyte.bit[i] return retbyteVGF2P8AFFINEINVQB dest, src1, src2, imm8 (EVEX Encoded Version)(KL, VL) = (2, 128), (4, 256), (8, 512):= 0 TO KL-1:FOR j IF SRC2 is memory and EVEX.b==1::= tsrc2 SRC2.qword[0]ELSE::= tsrc2 SRC2.qword[j]:= 0 to 7:FOR b IF k1[j*8+b] OR *no writemask*::= FOR i 0 to 7::= DEST.qword[j].byte[b] affine_inverse_byte(tsrc2, SRC1.qword[j].byte[b], imm8)ELSE IF *zeroing*::= DEST.qword[j].byte[b] 0*ELSE DEST.qword[j].byte[b] remains unchanged*:= VGF2P8AFFINEINVQB dest, src1, src2, imm8 (128b and 256b VEX Encoded Versions)(KL, VL) = (2, 128), (4, 256):= 0 TO KL-1:FOR j := FOR b 0 to 7::= DEST.qword[j].byte[b] affine_inverse_byte(SRC2.qword[j], SRC1.qword[j].byte[b], imm8):= DEST[MAX_VL-1:VL] 0GF2P8AFFINEINVQB srcdest, src1, imm8 (128b SSE Encoded Version):= FOR j 0 TO 1::= FOR b 0 to 7::= SRCDEST.qword[j].byte[b] affine_inverse_byte(SRC1.qword[j], SRCDEST.qword[j].byte[b], imm8)Intel C/C++ Compiler Intrinsic Equivalent(V)GF2P8AFFINEINVQB __m128i _mm_gf2p8affineinv_epi64_epi8(__m128i, __m128i, int);(V)GF2P8AFFINEINVQB __m128i _mm_mask_gf2p8affineinv_epi64_epi8(__m128i, __mmask16, __m128i, __m128i, int);(V)GF2P8AFFINEINVQB __m128i _mm_maskz_gf2p8affineinv_epi64_epi8(__mmask16, __m128i, __m128i, int);VGF2P8AFFINEINVQB __m256i _mm256_gf2p8affineinv_epi64_epi8(__m256i, __m256i, int);VGF2P8AFFINEINVQB __m256i _mm256_mask_gf2p8affineinv_epi64_epi8(__m256i, __mmask32, __m256i, __m256i, int);VGF2P8AFFINEINVQB __m256i _mm256_maskz_gf2p8affineinv_epi64_epi8(__mmask32, __m256i, __m256i, int);VGF2P8AFFINEINVQB __m512i _mm512_gf2p8affineinv_epi64_epi8(__m512i, __m512i, int);VGF2P8AFFINEINVQB __m512i _mm512_mask_gf2p8affineinv_epi64_epi8(__m512i, __mmask64, __m512i, __m512i, int);VGF2P8AFFINEINVQB __m512i _mm512_maskz_gf2p8affineinv_epi64_epi8(__mmask64, __m512i, __m512i, int);
```

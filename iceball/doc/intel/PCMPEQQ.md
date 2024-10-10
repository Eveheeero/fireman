# PCMPEQQ

Compare Packed Qword Data for Equal

Performs an SIMD compare for equality of the packed quadwords in the destination operand (first operand) and the source operand (second operand).
 If a pair of data elements is equal, the corresponding data element in the desti-nation is set to all 1s; otherwise, it is set to 0s.128-bit Legacy SSE version: The second source operand can be an XMM register or a 128-bit memory location.
The first source and destination operands are XMM registers.
Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: The second source operand can be an XMM register or a 128-bit memory location.
The first source and destination operands are XMM registers.
Bits (MAXVL-1:128) of the corresponding YMM register are zeroed.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand is a YMM register or a 256-bit memory location.
The destination operand is a YMM register.EVEX encoded VPCMPEQQ: The first source operand (second operand) is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 64-bit memory location.
The destinatio

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
PCMPEQQ (With 128-bit Operands)IF (DEST[63:0] = SRC[63:0]) THEN DEST[63:0] := FFFFFFFFFFFFFFFFH;ELSE DEST[63:0] := 0; FI;IF (DEST[127:64] = SRC[127:64]) THEN DEST[127:64] := FFFFFFFFFFFFFFFFH;ELSE DEST[127:64] := 0; FI;DEST[MAXVL-1:128] (Unmodified)COMPARE_QWORDS_EQUAL (SRC1, SRC2)IF SRC1[63:0] = SRC2[63:0]THEN DEST[63:0] := FFFFFFFFFFFFFFFFH;ELSE DEST[63:0] := 0; FI;IF SRC1[127:64] = SRC2[127:64]THEN DEST[127:64] := FFFFFFFFFFFFFFFFH;ELSE DEST[127:64] := 0; FI;VPCMPEQQ (VEX.128 Encoded Version)DEST[127:0] := COMPARE_QWORDS_EQUAL(SRC1,SRC2)DEST[MAXVL-1:128] := 0VPCMPEQQ (VEX.256 Encoded Version)DEST[127:0] := COMPARE_QWORDS_EQUAL(SRC1[127:0],SRC2[127:0])DEST[255:128] := COMPARE_QWORDS_EQUAL(SRC1[255:128],SRC2[255:128])DEST[MAXVL-1:256] := 0VPCMPEQQ (EVEX Encoded Versions)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF k2[j] OR *no writemask*THEN IF (EVEX.b = 1) AND (SRC2 *is memory*)THEN CMP := SRC1[i+63:i] = SRC2[63:0];ELSE CMP := SRC1[i+63:i] = SRC2[i+63:i];FI;IF CMP = TRUETHEN DEST[j] := 1;ELSE DEST[j] := 0; FI;ELSE DEST[j] := 0; zeroing-masking onlyFI;Intel C/C++ Compiler Intrinsic EquivalentVPCMPEQQ __mmask8 _mm512_cmpeq_epi64_mask( __m512i a, __m512i b);VPCMPEQQ __mmask8 _mm512_mask_cmpeq_epi64_mask(__mmask8 k, __m512i a, __m512i b);VPCMPEQQ __mmask8 _mm256_cmpeq_epi64_mask( __m256i a, __m256i b);VPCMPEQQ __mmask8 _mm256_mask_cmpeq_epi64_mask(__mmask8 k, __m256i a, __m256i b);VPCMPEQQ __mmask8 _mm_cmpeq_epi64_mask( __m128i a, __m128i b);VPCMPEQQ __mmask8 _mm_mask_cmpeq_epi64_mask(__mmask8 k, __m128i a, __m128i b);(V)PCMPEQQ __m128i _mm_cmpeq_epi64(__m128i a, __m128i b);VPCMPEQQ __m256i _mm256_cmpeq_epi64( __m256i a, __m256i b);
```

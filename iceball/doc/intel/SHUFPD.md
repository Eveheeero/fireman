# SHUFPD

Packed Interleave Shuffle of Pairs of Double Precision Floating-Point Values

Selects a double precision floating-point value of an input pair using a bit control and move to a designated element of the destination operand.
The low-to-high order of double precision element of the destination operand is inter-leaved between the first source operand and the second source operand at the granularity of input pair of 128 bits.
Each bit in the imm8 byte, starting from bit 0, is the select control of the corresponding element of the destination to received the shuffled result of an input pair.
EVEX encoded versions: The first source operand is a ZMM/YMM/XMM register.
The second source operand can be a ZMM/YMM/XMM register, a 512/256/128-bit memory location or a 512/256/128-bit vector broadcasted from a 64-bit memory location The destination operand is a ZMM/YMM/XMM register updated according to the writemask.
The select controls are the lower 8/4/2 bits of the imm8 byte.VEX.256 encoded version: The first source operand is a YMM register.
The second source operand can be a YMM register or a 256-bit memory location.
The destination operand is a YMM register.
The select controls are the bit 3:0 of the imm8 byte, imm8[7:4) are ignored.the corresponding ZMM register destination are zeroed.
The select controls are the bit 1:0 of the imm8 byte, imm8[7:2) are ignored.128-bit Legacy SSE version: The second source can be an XMM register or an 128-bit memory location.
The desti-nation operand and the first source operand is the same and is an XMM register.
The upper bits (MAXVL-1:128) of the corresponding ZMM register destination are unmodified.
The select controls are the bit 1:0 of the imm8 byte, imm8[7:2) are ignored.X3X2X1X0SRC1Y3Y2Y1Y0SRC2DESTY2 or Y3X2 or X3Y0 or Y1X0 or X1Figure 4-25.
 256-bit VSHUFPD Operation of Four Pairs of Double Precision Floating-Point Values

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions

## Operation

```C
VSHUFPD (EVEX Encoded Versions When SRC2 is a Vector Register)(KL, VL) = (2, 128), (4, 256), (8, 512)IF IMM0[0] = 0THEN TMP_DEST[63:0] := SRC1[63:0]ELSE TMP_DEST[63:0] := SRC1[127:64] FI;IF IMM0[1] = 0THEN TMP_DEST[127:64] := SRC2[63:0]ELSE TMP_DEST[127:64] := SRC2[127:64] FI;IF VL >= 256IF IMM0[2] = 0THEN TMP_DEST[191:128] := SRC1[191:128]ELSE TMP_DEST[191:128] := SRC1[255:192] FI;IF IMM0[3] = 0THEN TMP_DEST[255:192] := SRC2[191:128]ELSE TMP_DEST[255:192] := SRC2[255:192] FI;FI;IF VL >= 512IF IMM0[4] = 0THEN TMP_DEST[319:256] := SRC1[319:256]ELSE TMP_DEST[319:256] := SRC1[383:320] FI;IF IMM0[5] = 0THEN TMP_DEST[383:320] := SRC2[319:256]ELSE TMP_DEST[383:320] := SRC2[383:320] FI;IF IMM0[6] = 0THEN TMP_DEST[447:384] := SRC1[447:384]ELSE TMP_DEST[447:384] := SRC1[511:448] FI;IF IMM0[7] = 0THEN TMP_DEST[511:448] := SRC2[447:384]ELSE TMP_DEST[511:448] := SRC2[511:448] FI;FI;IF k1[j] OR *no writemask*THEN DEST[i+63:i] := TMP_DEST[i+63:i]ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VSHUFPD (EVEX Encoded Versions When SRC2 is Memory)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64IF (EVEX.b = 1) THEN TMP_SRC2[i+63:i] := SRC2[63:0]ELSE TMP_SRC2[i+63:i] := SRC2[i+63:i]FI;ENDFOR;IF IMM0[0] = 0THEN TMP_DEST[63:0] := SRC1[63:0]ELSE TMP_DEST[63:0] := SRC1[127:64] FI;IF IMM0[1] = 0THEN TMP_DEST[127:64] := TMP_SRC2[63:0]ELSE TMP_DEST[127:64] := TMP_SRC2[127:64] FI;IF VL >= 256IF IMM0[2] = 0THEN TMP_DEST[191:128] := SRC1[191:128]ELSE TMP_DEST[191:128] := SRC1[255:192] FI;IF IMM0[3] = 0THEN TMP_DEST[255:192] := TMP_SRC2[191:128]ELSE TMP_DEST[255:192] := TMP_SRC2[255:192] FI;FI;IF VL >= 512IF IMM0[4] = 0THEN TMP_DEST[319:256] := SRC1[319:256]ELSE TMP_DEST[319:256] := SRC1[383:320] FI;IF IMM0[5] = 0THEN TMP_DEST[383:320] := TMP_SRC2[319:256]ELSE TMP_DEST[383:320] := TMP_SRC2[383:320] FI;IF IMM0[6] = 0THEN TMP_DEST[447:384] := SRC1[447:384]ELSE TMP_DEST[447:384] := SRC1[511:448] FI;IF IMM0[7] = 0THEN TMP_DEST[511:448] := TMP_SRC2[447:384]ELSE TMP_DEST[511:448] := TMP_SRC2[511:448] FI;FI;FOR j := 0 TO KL-1i := j * 64ELSE IF *merging-masking*; merging-maskingTHEN *DEST[i+63:i] remains unchanged*ELSE *zeroing-masking*; zeroing-maskingDEST[i+63:i] := 0FIFI;ENDFORDEST[MAXVL-1:VL] := 0VSHUFPD (VEX.256 Encoded Version)IF IMM0[0] = 0THEN DEST[63:0] := SRC1[63:0]ELSE DEST[63:0] := SRC1[127:64] FI;IF IMM0[1] = 0THEN DEST[127:64] := SRC2[63:0]ELSE DEST[127:64] := SRC2[127:64] FI;IF IMM0[2] = 0THEN DEST[191:128] := SRC1[191:128]ELSE DEST[191:128] := SRC1[255:192] FI;IF IMM0[3] = 0THEN DEST[255:192] := SRC2[191:128]ELSE DEST[255:192] := SRC2[255:192] FI;DEST[MAXVL-1:256] (Unmodified)VSHUFPD (VEX.128 Encoded Version)IF IMM0[0] = 0THEN DEST[63:0] := SRC1[63:0]ELSE DEST[63:0] := SRC1[127:64] FI;IF IMM0[1] = 0THEN DEST[127:64] := SRC2[63:0]ELSE DEST[127:64] := SRC2[127:64] FI;DEST[MAXVL-1:128] := 0VSHUFPD (128-bit Legacy SSE Version)IF IMM0[0] = 0THEN DEST[63:0] := SRC1[63:0]ELSE DEST[63:0] := SRC1[127:64] FI;IF IMM0[1] = 0THEN DEST[127:64] := SRC2[63:0]ELSE DEST[127:64] := SRC2[127:64] FI;DEST[MAXVL-1:128] (Unmodified)Intel C/C++ Compiler Intrinsic EquivalentVSHUFPD __m512d _mm512_shuffle_pd(__m512d a, __m512d b, int imm);VSHUFPD __m512d _mm512_mask_shuffle_pd(__m512d s, __mmask8 k, __m512d a, __m512d b, int imm);VSHUFPD __m512d _mm512_maskz_shuffle_pd( __mmask8 k, __m512d a, __m512d b, int imm);VSHUFPD __m256d _mm256_shuffle_pd (__m256d a, __m256d b, const int select);VSHUFPD __m256d _mm256_mask_shuffle_pd(__m256d s, __mmask8 k, __m256d a, __m256d b, int imm);VSHUFPD __m256d _mm256_maskz_shuffle_pd( __mmask8 k, __m256d a, __m256d b, int imm);SHUFPD __m128d _mm_shuffle_pd (__m128d a, __m128d b, const int select);VSHUFPD __m128d _mm_mask_shuffle_pd(__m128d s, 
```

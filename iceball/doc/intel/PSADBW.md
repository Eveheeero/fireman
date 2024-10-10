# PSADBW

Compute Sum of Absolute Differences

Computes the absolute value of the difference of 8 unsigned byte integers from the source operand (second operand) and from the destination operand (first operand).
These 8 differences are then summed to produce an unsigned word integer result that is stored in the destination operand.
Figure4-14 shows the operation of the PSADBW instruction when using 64-bit operands.When operating on 64-bit operands, the word integer result is stored in the low word of the destination operand, and the remaining bytes in the destination operand are cleared to all 0s.When operating on 128-bit operands, two packed results are computed.
Here, the 8 low-order bytes of the source and destination operands are operated on to produce a word result that is stored in the low word of the destination operand, and the 8 high-order bytes are operated on to produce a word result that is stored in bits 64 through 79 of the destination operand.
The remaining bytes of the destination operand are cleared.For 256-bit version, the third group of 8 differences are summed to produce an unsigned word in bits[143:128] of the destination register and the fourth group of 8 differences are summed to produce an unsigned word in bits[207:192] of the destination register.
The remaining words of the destination are set to 0.
For 512-bit version, the fifth group result is stored in bits [271:256] of the destination.
The result from the sixth group is stored in bits [335:320].
The results for the seventh and eighth group are stored respectively in bits [399:384] and bits [463:447], respectively.
The remaining bits in the destination are set to 0.In 64-bit mode and not encoded by VEX/EVEX prefix, using a REX prefix in the form of REX.R permits this instruc-tion to access additional registers (XMM8-XMM15).Legacy SSE version: The source operand can be an MMX technology register or a 64-bit memory location.
The destination operand is an MMX technology register.128-bit Legacy SSE version: The first source operand and destination register are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding ZMM destination register remain unchanged.VEX.128 and EVEX.128 encoded versions: The first source operand and destination register are XMM registers.
The second source operand is an XMM register or a 128-bit memory location.
Bits (MAXVL-1:128) of the corresponding ZMM register are zeroed.VEX.256 and EVEX.256 encoded versions: The first source operand and destination register are YMM registers.
The second source operand is an YMM register or a 256-bit memory location.
Bits (MAXVL-1:256) of the corresponding ZMM register are zeroed.EVEX.512 encoded version: The first source operand and destination register are ZMM registers.
The second source operand is a ZMM register or a 512-bit memory location.SRCX7X6X5X4X3X2X1X0DESTY7Y6Y5Y4Y3Y2Y1Y0TEMPABS(X7:Y7)ABS(X6:Y6)ABS(X5:Y5)ABS(X4:Y4)ABS(X3:Y3)ABS(X2:Y2)ABS(X1:Y1)ABS(X0:Y0)DEST00H00H00H00H00H00HSUM(TEMP7...TEMP0)Figure 4-14.
 PSADBW Instructio

## Flags affected

- None.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-21, "Type 4 Class Exception Conditions."

## Operation

```C
VPSADBW (EVEX Encoded Versions)VL = 128, 256, 512TEMP0 := ABS(SRC1[7:0] - SRC2[7:0])(* Repeat operation for bytes 1 through 15 *)TEMP15 := ABS(SRC1[127:120] - SRC2[127:120])DEST[15:0] := SUM(TEMP0:TEMP7)DEST[63:16] := 000000000000HDEST[79:64] := SUM(TEMP8:TEMP15)DEST[127:80] := 00000000000HIF VL >= 256(* Repeat operation for bytes 16 through 31*)TEMP31 := ABS(SRC1[255:248] - SRC2[255:248])DEST[143:128] := SUM(TEMP16:TEMP23)DEST[191:144] := 000000000000HDEST[207:192] := SUM(TEMP24:TEMP31)DEST[223:208] := 00000000000HFI;IF VL >= 512(* Repeat operation for bytes 32 through 63*)TEMP63 := ABS(SRC1[511:504] - SRC2[511:504])DEST[271:256] := SUM(TEMP0:TEMP7)DEST[319:272] := 000000000000HDEST[335:320] := SUM(TEMP8:TEMP15)DEST[383:336] := 00000000000HDEST[399:384] := SUM(TEMP16:TEMP23)DEST[447:400] := 000000000000HDEST[463:448] := SUM(TEMP24:TEMP31)DEST[511:464] := 00000000000HFI;DEST[MAXVL-1:VL] := 0VPSADBW (VEX.256 Encoded Version)TEMP0 := ABS(SRC1[7:0] - SRC2[7:0])(* Repeat operation for bytes 2 through 30*)TEMP31 := ABS(SRC1[255:248] - SRC2[255:248])DEST[15:0] := SUM(TEMP0:TEMP7)DEST[63:16] := 000000000000HDEST[79:64] := SUM(TEMP8:TEMP15)DEST[127:80] := 00000000000HDEST[143:128] := SUM(TEMP16:TEMP23)DEST[191:144] := 000000000000HDEST[207:192] := SUM(TEMP24:TEMP31)VPSADBW (VEX.128 Encoded Version)TEMP0 := ABS(SRC1[7:0] - SRC2[7:0])(* Repeat operation for bytes 2 through 14 *)TEMP15 := ABS(SRC1[127:120] - SRC2[127:120])DEST[15:0] := SUM(TEMP0:TEMP7)DEST[63:16] := 000000000000HDEST[79:64] := SUM(TEMP8:TEMP15)DEST[127:80] := 00000000000HDEST[MAXVL-1:128] := 0PSADBW (128-bit Legacy SSE Version)TEMP0 := ABS(DEST[7:0] - SRC[7:0])(* Repeat operation for bytes 2 through 14 *)TEMP15 := ABS(DEST[127:120] - SRC[127:120])DEST[15:0] := SUM(TEMP0:TEMP7)DEST[63:16] := 000000000000HDEST[79:64] := SUM(TEMP8:TEMP15)DEST[127:80] := 00000000000DEST[MAXVL-1:128] (Unmodified)PSADBW (64-bit Operand)TEMP0 := ABS(DEST[7:0] - SRC[7:0])(* Repeat operation for bytes 2 through 6 *)TEMP7 := ABS(DEST[63:56] - SRC[63:56])DEST[15:0] := SUM(TEMP0:TEMP7)DEST[63:16] := 000000000000HIntel C/C++ Compiler Intrinsic EquivalentVPSADBW __m512i _mm512_sad_epu8( __m512i a, __m512i b)PSADBW __m64 _mm_sad_pu8(__m64 a,__m64 b)(V)PSADBW __m128i _mm_sad_epu8(__m128i a, __m128i b)VPSADBW __m256i _mm256_sad_epu8( __m256i a, __m256i b)
```

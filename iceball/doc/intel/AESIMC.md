# AESIMC

Perform the AES InvMixColumn Transformation

Perform the InvMixColumns transformation on the source operand and store the result in the destination operand.
The destination operand is an XMM register.
The source operand can be an XMM register or a 128-bit memory loca-tion.
Note: the AESIMC instruction should be applied to the expanded AES round keys (except for the first and last round key) in order to prepare them for decryption using the "Equivalent Inverse Cipher" (defined in FIPS 197).
128-bit Legacy SSE version: Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: Bits (MAXVL-1:128) of the destination YMM register are zeroed.Note: In VEX-encoded versions, VEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > See Table2-21, "Type 4 Class Exce
  > ption Conditions," additionally:
  - #UD - If VEX.vvvv

## Operation

```C
AESIMCDEST[127:0] := InvMixColumns( SRC );DEST[MAXVL-1:128] (Unmodified)VAESIMC DEST[127:0] := InvMixColumns( SRC );DEST[MAXVL-1:128] := 0;Intel C/C++ Compiler Intrinsic Equivalent(V)AESIMC __m128i _mm_aesimc (__m128i)
```

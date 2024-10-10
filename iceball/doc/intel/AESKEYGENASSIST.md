# AESKEYGENASSIST

AES Round Key Generation Assist

Assist in expanding the AES cipher key, by computing steps towards generating a round key for encryption, using 128-bit data specified in the source operand and an 8-bit round constant specified as an immediate, store the result in the destination operand.The destination operand is an XMM register.
The source operand can be an XMM register or a 128-bit memory loca-tion.128-bit Legacy SSE version: Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.VEX.128 encoded version: Bits (MAXVL-1:128) of the destination YMM register are zeroed.Note: In VEX-encoded versions, VEX.vvvv is reserved and must be 1111b, otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > See Table2-21, "Type 4 Class Exce
  > ption Conditions," additionally:
  - #UD - If VEX.vvvv

## Operation

```C
AESKEYGENASSISTX3[31:0] := SRC [127: 96];X2[31:0] := SRC [95: 64];X1[31:0] := SRC [63: 32];X0[31:0] := SRC [31: 0];RCON[31:0] := ZeroExtend(imm8[7:0]);DEST[31:0] := SubWord(X1);DEST[63:32 ] := RotWord( SubWord(X1) ) XOR RCON;DEST[95:64] := SubWord(X3);DEST[127:96] VAESKEYGENASSIST X3[31:0] := SRC [127: 96];X2[31:0] := SRC [95: 64];X1[31:0] := SRC [63: 32];X0[31:0] := SRC [31: 0];RCON[31:0] := ZeroExtend(imm8[7:0]);DEST[31:0] := SubWord(X1);DEST[63:32 ] := RotWord( SubWord(X1) ) XOR RCON;DEST[95:64] := SubWord(X3);DEST[127:96] := RotWord( SubWord(X3) ) XOR RCON;DEST[MAXVL-1:128] := 0;Intel C/C++ Compiler Intrinsic Equivalent(V)AESKEYGENASSIST__m128i _mm_aeskeygenassist (__m128i, const int)
```

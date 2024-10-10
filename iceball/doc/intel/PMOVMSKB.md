# PMOVMSKB

Move Byte Mask

Creates a mask made up of the most significant bit of each byte of the source operand (second operand) and stores the result in the low byte or word of the destination operand (first operand).The byte mask is 8 bits for 64-bit source operand, 16 bits for 128-bit source operand and 32 bits for 256-bit source operand.
The destination operand is a general-purpose register.
In 64-bit mode, the instruction can access additional registers (XMM8-XMM15, R8-R15) when used with a REX.R prefix.
The default operand size is 64-bit in 64-bit mode.Legacy SSE version: The source operand is an MMX technology register.128-bit Legacy SSE version: The source operand is an XMM register.VEX.128 encoded version: The source operand is an XMM register.VEX.256 encoded version: The source operand is a YMM register.Note: VEX.vvvv is reserved and must be 1111b.


## Flags affected

- None.

## Exceptions

- Other Exceptions
  > See Table2-24, "Type 7 Class Exception Conditions," additionally:
  - #UD - If VEX.vvvv
- Numeric Exceptions
  > None.

## Operation

```C
PMOVMSKB (With 64-bit Source Operand and r32)r32[0] := SRC[7];r32[1] := SRC[15];(* Repeat operation for bytes 2 through 6 *)(V)PMOVMSKB (With 128-bit Source Operand and r32)r32[0] := SRC[7];r32[1] := SRC[15];(* Repeat operation for bytes 2 through 14 *)r32[15] := SRC[127]; r32[31:16] := ZERO_FILL;VPMOVMSKB (With 256-bit Source Operand and r32)r32[0] := SRC[7];r32[1] := SRC[15];(* Repeat operation for bytes 3rd through 31*)r32[31] := SRC[255];PMOVMSKB (With 64-bit Source Operand and r64)r64[0] := SRC[7];r64[1] := SRC[15];(* Repeat operation for bytes 2 through 6 *)r64[7] := SRC[63]; r64[63:8] := ZERO_FILL;(V)PMOVMSKB (With 128-bit Source Operand and r64)r64[0] := SRC[7];r64[1] := SRC[15];(* Repeat operation for bytes 2 through 14 *)r64[15] := SRC[127]; r64[63:16] := ZERO_FILL;VPMOVMSKB (With 256-bit Source Operand and r64)r64[0] := SRC[7];r64[1] := SRC[15];(* Repeat operation for bytes 2 through 31*)r64[31] := SRC[255];r64[63:32] := ZERO_FILL;Intel C/C++ Compiler Intrinsic EquivalentPMOVMSKB int _mm_movemask_pi8(__m64 a)(V)PMOVMSKB int _mm_movemask_epi8 ( __m128i a)VPMOVMSKB int _mm256_movemask_epi8 ( __m256i a)
```

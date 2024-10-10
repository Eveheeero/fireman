# PINSRW

Insert Word

Three operand MMX and SSE instructions:Copies a word from the source operand and inserts it in the destination operand at the location specified with the count operand.
(The other words in the destination register are left untouched.) The source operand can be a general-purpose register or a 16-bit memory location.
(When the source operand is a general-purpose register, the low word of the register is copied.) The destination operand can be an MMX technology register or an XMM register.
The count operand is an 8-bit immediate.
When specifying a word location in an MMX technology register, the 2 least-significant bits of the count operand specify the location; for an XMM register, the 3 least-significant bits specify the location.Bits (MAXVL-1:128) of the corresponding YMM destination register remain unchanged.Four operand AVX and AVX-512 instructions: Combines a word from the first source operand with the second source operand, and inserts it in the destination operand at the location specified with the count operand.
The second source operand can be a general-purpose register or a 16-bit memory location.
(When the source operand is a general-purpose register, the low word of the register is copied.) The first source and destination operands are XMM registers.
The count operand is an 8-bit immediate.
When specifying a word location, the 3 least-significant bits specify the location.Bits (MAXVL-1:128) of the destination YMM register are ze

## Flags affected

- None.

## Exceptions

- Other Exceptions
  > EVEX-encoded instruction, see Table2-22, "Type 5 Class Exception Conditions."
  > EVEX-encoded instruction, see Table2-57, "Type E9NF Class Exception Conditions."
- Numeric Exceptions
  > None.

## Operation

```C
PINSRW dest, src, imm8 (MMX)SEL := imm8[1:0]     DEST.word[SEL] := src.word[0]PINSRW dest, src, imm8 (SSE)SEL := imm8[2:0]     DEST.word[SEL] := src.word[0]VPINSRW dest, src1, src2, imm8 (AVX/AVX512)SEL := imm8[2:0]DEST := src1     DEST.word[SEL] := src2.word[0]     DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentPINSRW __m64 _mm_insert_pi16 (__m64 a, int d, int n)PINSRW __m128i _mm_insert_epi16 ( __m128i a, int b, int imm)
```

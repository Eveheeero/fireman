# MOVHLPS

Move Packed Single Precision Floating-Point Values High to Low

This instruction cannot be used for memory to register moves.128-bit two-argument form:Moves two packed single precision floating-point values from the high quadword of the second XMM argument (second operand) to the low quadword of the first XMM register (first argument).
The quadword at bits 127:64 of the destination operand is left unchanged.
Bits (MAXVL-1:128) of the corresponding destination register remain unchanged.128-bit and EVEX three-argument form:Moves two packed single precision floating-point values from the high quadword of the third XMM argument (third operand) to the low quadword of the destination (first operand).
Copies the high quadword from the second XMM argument (second operand) to the high quadword of the destination (first operand).
Bits (MAXVL-1:128) of the corresponding destination register are zeroed.If VMOVHLPS is encoded with VEX.L or EVEX.L'L= 1, an attempt to execute the instruction encoded with VEX.L or EVEX.L'L= 1 will cause an #UD exception.

## Exceptions

- SIMD Floating-Point Exceptions
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-24, "T
  > ype 7 Class Exception Conditions," additionally:
  - #UD - If VEX.L = 1.

## Operation

```C
MOVHLPS (128-bit Two-Argument Form)DEST[63:0] := SRC[127:64]DEST[MAXVL-1:64] (Unmodified)VMOVHLPS (128-bit Three-Argument Form - VEX & EVEX)DEST[63:0] := SRC2[127:64]DEST[127:64] := SRC1[127:64]DEST[MAXVL-1:128] := 0Intel C/C++ Compiler Intrinsic EquivalentMOVHLPS __m128 _mm_movehl_ps(__m128 a, __m128 b)
```

# VMOVW

Move Word

This instruction either (a) copies one word element from an XMM register to a general-purpose register or memory location or (b) copies one word element from a general-purpose register or memory location to an XMM register.
When writing a general-purpose register, the lower 16-bits of the register will contain the word value.
The upper bits of the general-purpose register are written with zeros.

## Exceptions

- SIMD Floating-Point Exceptions
  > None

## Operation

```C
VMOVW dest, src (two operand load)DEST.word[0] := SRC.word[0]DEST[MAXVL:16] := 0 VMOVW dest, src (two operand store)DEST.word[0] := SRC.word[0]// upper bits of GPR DEST are zeroed Intel C/C++ Compiler Intrinsic EquivalentVMOVW short _mm_cvtsi128_si16 (__m128i a);VMOVW __m128i _mm_cvtsi16_si128 (short a);
```

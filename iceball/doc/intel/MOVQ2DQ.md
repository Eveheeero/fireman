# MOVQ2DQ

Move Quadword from MMX Technology to XMM Register

Moves the quadword from the source operand (second operand) to the low quadword of the destination operand (first operand).
The source operand is an MMX technology register and the destination operand is an XMM register.
This instruction causes a transition from x87 FPU to MMX technology operation (that is, the x87 FPU top-of-stack pointer is set to 0 and the x87 FPU tag word is set to all 0s [valid]).
If this instruction is executed while an x87 FPU floating-point exception is pending, the exception is handled before the MOVQ2DQ instruction is executed.In 64-bit mode, use of the REX.R prefix permits this instruction to access additional registers (XMM8-XMM15).

## Exceptions

- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - If CR0.TS[bit 3] = 1.
  - #UD - If CR0.EM[bit 2] = 1.
  > If CR4.OSFXSR[bit 9] = 0.
  > If CPUID.01H:EDX.SSE2[bit 26] = 0.
  > If the LOCK prefix is used.
  - #MF - If there is a pending x87 FPU exception.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
DEST[63:0] := SRC[63:0];DEST[127:64] := 00000000000000000H;Intel C/C++ Compiler Intrinsic EquivalentMOVQ2DQ__128i _mm_movpi64_epi64 ( __m64 a)
```

# MULX

Unsigned Multiply Without Affecting Flags

Performs an unsigned multiplication of the implicit source operand (EDX/RDX) and the specified source operand (the third operand) and stores the low half of the result in the second destination (second operand), the high half of the result in the first destination operand (first operand), without reading or writing the arithmetic flags.
This enables efficient programming where the software can interleave add with carry operations and multiplications.
If the first and second operand are identical, it will contain the high half of the multiplication result.This instruction is not supported in real mode and virtual-8086 mode.
The operand size is always 32 bits if not in 64-bit mode.
In 64-bit mode operand size 64 requires VEX.W1.
VEX.W1 is ignored in non-64-bit modes.
An attempt to execute this instruction with VEX.L not equal to 0 will cause #UD.

## Flags affected

- None.

## Operation

```C
// DEST1: ModRM:reg// DEST2: VEX.vvvvIF (OperandSize = 32)SRC1 := EDX;DEST2 := (SRC1*SRC2)[31:0];DEST1 := (SRC1*SRC2)[63:32];ELSE IF (OperandSize = 64)SRC1 := RDX;DEST2 := (SRC1*SRC2)[63:0];DEST1 := (SRC1*SRC2)[127:64];FIIntel C/C++ Compiler Intrinsic EquivalentAuto-generated from high-level language when possible.unsigned int mulx_u32(unsigned int a, unsigned int b, unsigned int * hi);unsigned __int64 mulx_u64(unsigned __int64 a, unsigned __int64 b, unsigned __int64 * hi);
```

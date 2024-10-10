# PSHUFW

Shuffle Packed Words

Copies words from the source operand (second operand) and inserts them in the destination operand (first operand) at word locations selected with the order operand (third operand).
This operation is similar to the opera-tion used by the PSHUFD instruction, which is illustrated in Figure4-16.
For the PSHUFW instruction, each 2-bit field in the order operand selects the contents of one word location in the destination operand.
The encodings of the order operand fields select words from the source operand to be copied to the destination operand.The source operand can be an MMX technology register or a 64-bit memory location.
The destination operand is an MMX technology register.
The order operand is an 8-bit immediate.
Note that this instruction permits a word in the source operand to be copied to more than one word location in the destination operand.In 64-bit mode, using a REX prefix in the form of REX.R permits this instruction to access additional registers (XMM8-XMM15).

## Flags affected

- None.

## Exceptions

- Numeric Exceptions
  > None.
- Other Exceptions
  > ®
  > See Table23-7, "Exception Conditions for SIMD/MMX 
  > Instructions with Memory Reference," in the Intel

## Operation

```C
DEST[15:0] := (SRC >> (ORDER[1:0] * 16))[15:0];DEST[31:16] := (SRC >> (ORDER[3:2] * 16))[15:0];DEST[47:32] := (SRC >> (ORDER[5:4] * 16))[15:0];DEST[63:48] := (SRC >> (ORDER[7:6] * 16))[15:0];Intel C/C++ Compiler Intrinsic EquivalentPSHUFW __m64 _mm_shuffle_pi16(__m64 a, int n)
```

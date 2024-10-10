# BSWAP

Byte Swap

Reverses the byte order of a 32-bit or 64-bit (destination) register.
This instruction is provided for converting little-endian values to big-endian format and vice versa.
To swap bytes in a word value (16-bit register), use the XCHG instruction.
When the BSWAP instruction references a 16-bit register, the result is undefined.In 64-bit mode, the instruction's default operation size is 32 bits.
Using a REX prefix in the form of REX.R permits access to additional registers (R8-R15).
Using a REX prefix in the form of REX.W promotes operation to 64 bits.
See the summary chart at the beginning of this section for encoding data and limits.IA-32 Architecture Legacy CompatibilityThe BSWAP instruction is not supported on IA-32 processors earlier than the Intel486 processor family.
For compatibility with this instruction, software should include functionally equivalent code for execution on Intel processors earlier than the Intel486 processor family.

## Flags affected

- None.

## Operation

```C
TEMP := DESTIF 64-bit mode AND OperandSize = 64THENDEST[7:0] := TEMP[63:56];DEST[15:8] := TEMP[55:48];DEST[23:16] := TEMP[47:40];DEST[31:24] := TEMP[39:32];DEST[39:32] := TEMP[31:24];DEST[47:40] := TEMP[23:16];DEST[55:48] := TEMP[15:8];DEST[63:56] := TEMP[7:0];ELSEDEST[7:0] := TEMP[31:24];DEST[15:8] := TEMP[23:16];DEST[23:16] := TEMP[15:8];DEST[31:24] := TEMP[7:0];FI;
```

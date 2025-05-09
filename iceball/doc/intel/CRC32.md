# CRC32

Accumulate CRC32 Value

Starting with an initial value in the first operand (destination operand), accumulates a CRC32 (polynomial 11EDC6F41H) value for the second operand (source operand) and stores the result in the destination operand.
The source operand can be a register or a memory location.
The destination operand must be an r32 or r64 register.
If the destination is an r64 register, then the 32-bit result is stored in the least significant double word and 00000000H is stored in the most significant double word of the r64 register.The initial value supplied in the destination operand is a double word integer stored in the r32 register or the least significant double word of the r64 register.
To incrementally accumulate a CRC32 value, software retains the result of the previous CRC32 operation in the destination operand, then executes the CRC32 instruction again with new input data in the source operand.
Data contained in the source operand is processed in reflected bit order.
This means that the most significant bit of the source operand is treated as the least significant bit of the quotient, and so on, for all the bits of the source operand.
Likewise, the result of the CRC operation is stored in the destination operand in reflected bit order.
This means that the most significant bit of the resulting CRC (bit 31) is stored in the least significant bit of the destination operand (bit 0), and so on, for all the bits of the CRC.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in Protected Mode.
- Real-Address Mode Exceptions
  - #GP(0) - If any part of the operand lies outside
  > of the effective address space from 0 to 0FFFFH.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #UD - If CPUID.01H:ECX.SSE4_2 [Bit 20] = 0.
  > If LOCK prefix is used.
- Virtual 8086 Mode Exceptions
  - #GP(0) - If any part of the operand lies outside
  > of the effective address space from 0 to 0FFFFH.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF (fault-code) - For a page fault.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If CPUID.01H:ECX.SSE4_2 [Bit 20] = 0.
  > If LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #GP(0) - If the memory address is in a non-canonical form.
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #PF (fault-code) - For a page fault.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
- SIMD Floating-Point Exceptions
  > None.
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  >  outside the CS, DS, ES, FS or GS segments.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF (fault-code) - For a page fault.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If CPUID.01H:ECX.SSE4_2 [Bit 20] = 0.
  > If LOCK prefix is used.

## Operation

```C
Notes:BIT_REFLECT64: DST[63-0] = SRC[0-63]BIT_REFLECT32: DST[31-0] = SRC[0-31]BIT_REFLECT16: DST[15-0] = SRC[0-15]CRC32 instruction for 64-bit source operand and 64-bit destination operand:TEMP1[63-0] := BIT_REFLECT64 (SRC[63-0])TEMP2[31-0] := BIT_REFLECT32 (DEST[31-0])TEMP3[95-0] := TEMP1[63-0] « 32TEMP4[95-0] := TEMP2[31-0] « 64TEMP5[95-0] := TEMP3[95-0] XOR TEMP4[95-0]TEMP6[31-0] := TEMP5[95-0] MOD2 11EDC6F41HDEST[31-0] := BIT_REFLECT (TEMP6[31-0])DEST[63-32] := 00000000HCRC32 instruction for 32-bit source operand and 32-bit destination operand:TEMP1[31-0] := BIT_REFLECT32 (SRC[31-0])TEMP2[31-0] := BIT_REFLECT32 (DEST[31-0])TEMP3[63-0] := TEMP1[31-0] « 32TEMP4[63-0] := TEMP2[31-0] « 32TEMP5[63-0] := TEMP3[63-0] XOR TEMP4[63-0]TEMP6[31-0] := TEMP5[63-0] MOD2 11EDC6F41HDEST[31-0] := BIT_REFLECT (TEMP6[31-0])CRC32 instruction for 16-bit source operand and 32-bit destination operand:TEMP1[15-0] := BIT_REFLECT16 (SRC[15-0])TEMP2[31-0] := BIT_REFLECT32 (DEST[31-0])TEMP3[47-0] := TEMP1[15-0] « 32TEMP4[47-0] := TEMP2[31-0] « 16TEMP5[47-0] := TEMP3[47-0] XOR TEMP4[47-0]TEMP6[31-0] := TEMP5[47-0] MOD2 11EDC6F41HDEST[31-0] := BIT_REFLECT (TEMP6[31-0])CRC32 instruction for 8-bit source operand and 64-bit destination operand:TEMP1[7-0] := BIT_REFLECT8(SRC[7-0])TEMP2[31-0] := BIT_REFLECT32 (DEST[31-0])TEMP3[39-0] := TEMP1[7-0] « 32TEMP4[39-0] := TEMP2[31-0] « 8TEMP5[39-0] := TEMP3[39-0] XOR TEMP4[39-0]TEMP6[31-0] := TEMP5[39-0] MOD2 11EDC6F41HDEST[31-0] := BIT_REFLECT (TEMP6[31-0])DEST[63-32] := 00000000HCRC32 instruction for 8-bit source operand and 32-bit destination operand:TEMP1[7-0] := BIT_REFLECT8(SRC[7-0])TEMP2[31-0] := BIT_REFLECT32 (DEST[31-0])TEMP3[39-0] := TEMP1[7-0] « 32TEMP4[39-0] := TEMP2[31-0] « 8TEMP5[39-0] := TEMP3[39-0] XOR TEMP4[39-0]TEMP6[31-0] := TEMP5[39-0] MOD2 11EDC6F41HDEST[31-0] := BIT_REFLECT (TEMP6[31-0])Intel C/C++ Compiler Intrinsic Equivalentunsigned int _mm_crc32_u8( unsigned int crc, unsigned char data )unsigned int _mm_crc32_u16( unsigned int crc, unsigned short data )unsigned int _mm_crc32_u32( unsigned int crc, unsigned int data )unsigned __int64 _mm_crc32_u64( unsigned __int64 crc, unsigned __int64 data )
```

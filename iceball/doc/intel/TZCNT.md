# TZCNT

Count the Number of Trailing Zero Bits

TZCNT counts the number of trailing least significant zero bits in source operand (second operand) and returns the result in destination operand (first operand).
TZCNT is an extension of the BSF instruction.
The key difference between TZCNT and BSF instruction is that TZCNT provides operand size as output when source operand is zero while in the case of BSF instruction, if source operand is zero, the content of destination operand are undefined.
On processors that do not support TZCNT, the instruction byte encoding is executed as BSF.

## Flags affected

- ZF is set to 1 in case of zero output (least significant bit of the source is set), and to 0 otherwise, CF is set to 1 if the input was zero and cleared otherwise. OF, SF, PF, and AF flags are undefined.Intel C/C++ Compiler Intrinsic EquivalentTZCNT unsigned __int32 _tzcnt_u32(unsigned __int32 src);

## Exceptions

- Virtual 8086 Mode Exceptions
  - #GP(0) - If any part of the operand lies outside
  > of the effective address space from 0 to 0FFFFH.
  - #SS(0) - For an illegal address in the SS segment.
  - #PF (fault-code) - For a page fault.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP(0) - If any part of the operand lies outside
  > of the effective address space from 0 to 0FFFFH.
  - #SS(0) - For an illegal address in the SS segment.
  - #UD - If LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #GP(0) - If the memory address is in a non-canonical form.
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #PF (fault-code) - For a page fault.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Protected Mode Exceptions
  - #GP(0) - For an illegal memory operand effective address in the CS, DS, ES, FS or GS segments.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a null segment 
  > selector.
  - #SS(0) - For an illegal address in the SS segment.
  - #PF (fault-code) - For a page fault.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in Protected Mode.

## Operation

```C
temp := 0DEST := 0DO WHILE ( (temp < OperandSize) and (SRC[ temp] = 0) )temp := temp +1DEST := DEST+ 1ODIF DEST = OperandSizeCF := 1ELSECF := 0FIIF DEST = 0ZF := 1ELSEZF := 0FI
```

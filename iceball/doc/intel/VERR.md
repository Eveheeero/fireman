# VERR/VERW

Verify a Segment for Reading or Writing

Verifies whether the code or data segment specified with the source operand is readable (VERR) or writable (VERW) from the current privilege level (CPL).
The source operand is a 16-bit register or a memory location that contains the segment selector for the segment to be verified.
If the segment is accessible and readable (VERR) or writable (VERW), the ZF flag is set; otherwise, the ZF flag is cleared.
Code segments are never verified as writable.
This check cannot be performed on system segments.
To set the ZF flag, the following conditions must be met: - The segment selector is not NULL.
- The selector must denote a descriptor within the bounds of the descriptor table (GDT or LDT).
- The selector must denote the descriptor of a code or data segment (not that of a system segment or gate).
- For the VERR instruction, the segment must be readable.
- For the VERW instruction, the segment must be a writable data segment.
- If the segment is not a conforming code segment, the segment's DPL must be greater than or equal to (have less or the same privilege as) both the CPL and the segment selector's RPL.The validation performed is the same as is performed when a segment selector is loaded into the DS, ES, FS, or GS register, and the indicated access (read or write) is performed.
The segment selector's value cannot result in a protection exception, enabling the software to anticipate possible segment access problems.This instruction's operation is the same in non-64-bit modes and 64-bit mode.
The operand size is fixed at 16 bits.

## Flags affected

- The ZF flag is set to 1 if the segment is accessible and readable (VERR) or writable (VERW); otherwise, it is set to 0.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  - #UD - The VERR and VERW instructions are not recognized in virtual-8086 mode.
  > If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Protected Mode Exceptions
  > The only exceptions generated for these instructions are 
  > those related to illegal addressing of the source operand.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #UD - The VERR and VERW instructions are not recognized in real-address mode.
  > If the LOCK prefix is used.

## Operation

```C
IF SRC(Offset) > (GDTR(Limit) or (LDTR(Limit))THEN ZF := 0; FI;Read segment descriptor;= IF SegmentDescriptor(DescriptorType) 0 (* System segment *)or (SegmentDescriptor(Type)  conforming code segment) and (CPL > DPL) or (RPL > DPL)THENZF := 0;ELSE= IF ((Instruction VERR) and (Segment readable))= or ((Instruction VERW) and (Segment writable))THEN ZF := 1;FI;FI;
```

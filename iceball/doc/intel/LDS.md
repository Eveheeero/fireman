# LDS/LES/LFS/LGS/LSS

Load Far Pointer

Loads a far pointer (segment selector and offset) from the second operand (source operand) into a segment register and the first operand (destination operand).
The source operand specifies a 48-bit or a 32-bit pointer in memory depending on the current setting of the operand-size attribute (32 bits or 16 bits, respectively).
The instruction opcode and the destination operand specify a segment register/general-purpose register pair.
The 16-bit segment selector from the source operand is loaded into the segment register specified with the opcode (DS, SS, ES, FS, or GS).
The 32-bit or 16-bit offset is loaded into the register specified with the destination operand.If one of these instructions is executed in protected mode, additional information from the segment descriptor pointed to by the segment selector in the source operand is loaded in the hidden part of the selected segment register.Also in protected mode, a NULL selector (values 0000 through 0003) can be loaded into DS, ES, FS, or GS registers without causing a protection exception.
(Any subsequent reference to a segment whose corresponding segment register is loaded with a NULL selector, causes a general-protection exception (#GP) and no memory reference to the segment occurs.)In 64-bit mode, the instruction's default operation size is 32 bits.
Using a REX prefix in the form of REX.W promotes operation to specify a source operand referencing an 80-bit pointer (16-bit selector, 64-bit offset) in memory.
Using a REX prefix in the form of REX.R permits access to additional registers (R8-R15).
See the summary chart at the beginning of this section for encoding data and limits.

## Flags affected

- None.

## Exceptions

- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #UD - If source operand is not a memory location.
  > If the LOCK prefix is used.
  - #GP(0) - If a NULL selector is loaded into the SS register.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #GP(selector) - If the SS register is being loaded and an
  > y of the following is true: the segment selector index 
  > is not within the descriptor table limits, the se
  > gment selector RPL is not equal to CPL, the 
  > segment is a non-writable data segment, or DPL is not equal to CPL.
  > If the DS, ES, FS, or GS register is being loaded with a non-NULL segment selector and any of 
  > the following is true: the segment selector index is not within descriptor table limits, the 
  > segment is neither a data nor a readable code 
  > segment, or the segment is a data or noncon-
  > forming-code segment and both RP
  > L and CPL are greater than DPL.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #SS(selector) - If the SS register is being loaded and the segment is marked not present.
  - #NP(selector) - If DS, ES, FS, or GS register is bein
  > g loaded with a non-NULL segment selector and the 
  > segment is marked not present.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
- Virtual-8086 Mode Exceptions
  - #UD - If source operand is not a memory location.
  > If the LOCK prefix is used.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
- 64-Bit Mode Exceptions
  - #GP(0) - If the memory address is in a non-canonical form.
  > If a NULL selector is attempted to be loaded
  >  into the SS register in compatibility mode.
  > If a NULL selector is attempted to be loaded 
  > into the SS register in CPL3 and 64-bit mode.
  > If a NULL selector is attempted to be loaded in
  > to the SS register in non-CPL3 and 64-bit mode 
  > where its RPL is not equal to CPL.
  - #GP(Selector) - If the FS, or GS register is being load
  > ed with a non-NULL segment selector and any of the 
  > following is true: the segment selector index is
  >  not within descriptor table limits, the memory 
  > address of the descriptor is non-canonical, the segment is neither a data nor a readable code 
  > segment, or the segment is a data or noncon
  > forming-code segment and both RPL and CPL are 
  > greater than DPL.
  > If the SS register is being loaded and any of the following is true: the segment selector index 
  > is not within the descriptor table limits, the me
  > mory address of the descriptor is non-canonical, 
  > the segment selector RPL is not equal to CPL, 
  > the segment is a nonwritable data segment, or 
  > DPL is not equal to CPL.
  - #SS(0) - If a memory operand effective address is non-canonical
  - #SS(Selector) - If the SS register is being loaded and the segment is marked not present.
  - #NP(selector) - If FS, or GS register is being loaded
  > with a non-NULL segment selector and the segment is 
  > marked not present.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.

## Operation

```C
64-BIT_MODEIF SS is loaded THEN =IF SegmentSelector  NULL and ( (RPL = 3) or  (RPL  3 and RPL CPL) )THEN #GP(selector); FI;ELSE IF Segment selector index is not within descriptor table limits or segment selector RPL CPLor access rights indicate nonwritable data segment  CPLor DPLTHEN #GP(selector); FI;ELSE IF Segment marked not presentTHEN #SS(selector); FI;FI;SS := SegmentSelector(SRC);SS := SegmentDescriptor([SRC]);ELSE IF attempt to load DS, or ES THEN #UD;ELSE IF FS, or GS is loaded with non-NULL segment selectorTHEN IF Segment selector index is not within descriptor table limitsor access rights indicate segment neither data nor readable code segmentor segment is data or nonconforming-code segment and ( RPL > DPL or CPL > DPL)THEN #GP(selector); FI;ELSE IF Segment marked not presentTHEN #NP(selector); FI;FI;SegmentRegister := SegmentSelector(SRC) ;SegmentRegister := SegmentDescriptor([SRC]);FI;ELSE IF FS, or GS is loaded with a NULL selector:THENSegmentRegister := NULLSelector;SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag; not accessible by software *)FI;DEST := Offset(SRC);PREOTECTED MODE OR COMPATIBILITY MODE;IF SS is loaded THEN = NULL IF SegementSelector THEN #GP(0);ELSE IF Segment selector index is not within descriptor table limits or segment selector RPL CPLor access rights indicate nonwritable data segment  CPLor DPLTHEN #GP(selector); FI;ELSE IF Segment marked not presentTHEN #SS(selector); FI;FI;SS := SegmentSelector(SRC);SS := SegmentDescriptor([SRC]);ELSE IF DS, ES, FS, or GS is loaded with non-NULL segment selectorTHEN IF Segment selector index is not within descriptor table limitsor access rights indicate segment neither data nor readable code segmentor segment is data or nonconforming-code segment and (RPL > DPL or CPL > DPL) THEN #NP(selector); FI;FI;SegmentRegister := SegmentSelector(SRC) AND RPL;SegmentRegister := SegmentDescriptor([SRC]);FI;ELSE IF DS, ES, FS, or GS is loaded with a NULL selector:THENSegmentRegister := NULLSelector;SegmentRegister(DescriptorValidBit) := 0; FI; (* Hidden flag; not accessible by software *)FI;DEST := Offset(SRC);Real-Address or Virtual-8086 ModeSegmentRegister := SegmentSelector(SRC); FI;DEST := Offset(SRC);
```

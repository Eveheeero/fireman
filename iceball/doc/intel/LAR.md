# LAR

Load Access Rights Byte

Loads the access rights from the segment descriptor specified by the second operand (source operand) into the first operand (destination operand) and sets the ZF flag in the flag register.
The source operand (which can be a register or a memory location) contains the segment selector for the segment descriptor being accessed.
If the source operand is a memory address, only 16 bits of data are accessed.
The destination operand is a general-purpose register.The processor performs access checks as part of the loading process.
Once loaded in the destination register, soft-ware can perform additional checks on the access rights information.
The access rights for a segment descriptor include fields located in the second doubleword (bytes 4-7) of the segment descriptor.
The following fields are loaded by the LAR instruction: - Bits 7:0 are returned as 0 - Bits 11:8 return the segment type.
- Bit 12 returns the S flag.
- Bits 14:13 return the DPL.
- Bit 15 returns the P flag.
- The following fields are returned only if the operand size is greater than 16 bits:-Bits 19:16 are undefined.-Bit 20 returns the software-available bit in the descriptor.-Bit 21 returns the L flag.-Bit 22 returns the D/B flag.-Bit 23 returns the G flag.-Bits 31:24 are returned as 0.This instruction performs the following checks before it loads the access rights in the destination register:  - Checks that the segment selector is not NULL.
- Checks that the segment selector points to a descriptor that is within the limits of the GDT or LDT being accessed - Checks that the descriptor type is valid for this instruction.
All code and data segment descriptors are valid for (can be accessed with) the LAR instruction.
The valid system segment and gate descriptor types are given in Table 3-53.
 - If the segment is not a conforming code segment, it checks that the specified segment descriptor is visible at the CPL (that is, if the CPL and the RPL of the segment selector are less than or equal to the DPL of the segment selector).If the segment descriptor cannot be accessed or is an invalidThe LAR instruction can only be executed in protected mode and IA-32e mode.Table 3-53.
 Segment and Gate TypesTypeProtected ModeIA-32e ModeNameValidNameValid0NoNoReservedReserved1YesNoAvailable 16-bit TSSReserved2YesYesLDTLDTYesNo3Busy 16-bit TSSReserved4YesNo16-bit call gateReserved5YesNo16-bit/32-bit task gateReservedNoNo616-bit interrupt gateReserved7NoNo16-bit trap gateReserved8NoNoReservedReserved9YesYesAvailable 32-bit TSSAvailable 64-bit TSSNoNoAReservedReservedBYesYesBusy 32-bit TSSBusy 64-bit TSSCYesYes32-bit call gate64-bit call gateNoNoDReservedReservedENoNo32-bit interrupt gate64-bit interrupt gateFNoNo32-bit trap gate64-bit trap gate

## Exceptions

- Virtual-8086 Mode Exceptions
  - #UD - The LAR instruction cannot be executed in virtual-8086 mode.
- Protected Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  - #SS(0) - If a memory operand effective a
  > ddress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and the me
  > mory operand effective address is unaligned while 
  > the current privilege level is 3. 
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If the memory operand effective address re
  > ferencing the SS segment is in a non-canonical 
  > form.
  - #GP(0) - If the memory operand effective address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and the me
  > mory operand effective address is unaligned while 
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #UD - The LAR instruction is not recognized in real-address mode.

## Operation

```C
IF Offset(SRC) > descriptor table limitTHEN ZF := 0; ELSESegmentDescriptor := descriptor referenced by SRC; conforming code segmentIF SegmentDescriptor(Type) and (CPL > DPL) or (RPL > DPL)or SegmentDescriptor(Type) is not valid for instructionTHENZF := 0;ELSEDEST := access rights from SegmentDescriptor as given in Description section;ZF := 1;FI;FI;
```

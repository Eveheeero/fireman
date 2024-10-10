# LSL

Load Segment Limit

Loads the unscrambled segment limit from the segment descriptor specified with the second operand (source operand) into the first operand (destination operand) and sets the ZF flag in the EFLAGS register.
The source operand (which can be a register or a memory location) contains the segment selector for the segment descriptor being accessed.
The destination operand is a general-purpose register.The processor performs access checks as part of the loading process.
Once loaded in the destination register, soft-ware can compare the segment limit with the offset of a pointer.
The segment limit is a 20-bit value contained in bytes 0 and 1 and in the first 4 bits of byte 6 of the segment descriptor.
If the descriptor has a byte granular segment limit (the granularity flag is set to 0), the destination operand is loaded with a byte granular value (byte limit).
If the descriptor has a page granular segment limit (the granularity flag is set to 1), the LSL instruction will translate the page granular limit (page limit) into a byte limit before loading it into the destination operand.
The translation is performed by shifting the 20-bit "raw" limit left 12 bits and filling the low-order 12 bits with 1s.When the operand size is 32 bits, the 32-bit byte limit is stored in the destination operand.
When the operand size is 16 bits, a valid 32-bit limit is computed; however, the upper 16 bits are truncated and only the low-order 16 bits are loaded into the destination operand.This instruction performs the following checks before it loads the segment limit into the destination register:  - Checks that the segment selector is not NULL.
- Checks that the segment selector points to a descriptor that is within the limits of the GDT or LDT being accessed - Checks that the descriptor type is valid for this instruction.
All code and data segment descriptors are valid for (can be accessed with) the LSL instruction.
The valid special segment and gate descriptor types are given in the following table.
 - If the segment is not a conforming code segment, the instruction checks that the specified segment descriptor is visible at the CPL (that is, if the CPL and the RPL of the segment selector are less than or equal to the DPL of the segment selector).If the segment descriptor cannot be accessed or is an invalidTable 3-56.
 Segment and Gate Descriptor TypesTypeProtected ModeIA-32e ModeNameValidNameValid0NoNoReservedReservedYesNo1Available 16-bit TSSReserved12YesYesLDTLDT3YesNoBusy 16-bit TSSReservedNoNo416-bit call gateReserved5NoNo16-bit/32-bit task gateReserved6NoNo16-bit interrupt gateReservedNoNo716-bit trap gateReserved8NoNoReservedReserved19YesYesAvailable 32-bit TSS64-bit TSSANoNoReservedReserved1YesYesBBusy 32-bit TSSBusy 64-bit TSSCNoNo32-bit call gate64-bit call gateDNoNoReservedReservedNoNoE32-bit interrupt gate64-bit interrupt gateFNoNo32-bit trap gate64-bit trap gateNOTES:1.
In this case, the descriptor comprises 16 bytes; bits 12:8 of the upper 4 bytes must be 0.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
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
- Real-Address Mode Exceptions
  - #UD - The LSL instruction cannot be executed in real-address mode.
- 64-Bit Mode Exceptions
  - #SS(0) - If the memory operand effective address re
  > ferencing the SS segment is in a non-canonical 
  > form.
  - #GP(0) - If the memory operand effective address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and the me
  > mory operand effective address is unaligned while 
- Virtual-8086 Mode Exceptions
  - #UD - The LSL instruction cannot be executed in virtual-8086 mode.

## Operation

```C
IF SRC(Offset) > descriptor table limitTHEN ZF := 0; FI;Read segment descriptor; IF SegmentDescriptor(Type) conforming code segmentand (CPL > DPL) OR (RPL > DPL)or Segment type is not valid for instructionTHENZF := 0;ELSEtemp := SegmentLimit([SRC]);IF (SegmentDescriptor(G) = 1)THEN temp := (temp << 12) OR 00000FFFH;= 32 ELSE IF OperandSize THEN DEST := temp; FI;=ELSE IF OperandSize  64 (* REX.W used *)THEN DEST := temp(* Zero-extended *); FI;= ELSE (* OperandSize 16 *)DEST := temp AND FFFFH;FI;FI;
```

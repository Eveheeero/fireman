# ENQCMD

Enqueue Command

The ENQCMD instruction allows software to write commands to enqueue registers, which are special device registers accessed using memory-mapped I/O (MMIO).Enqueue registers expect writes to have the following format:Figure 3-16.
 64-Byte Data Written to Enqueue RegistersBits19:0 convey the process address space identifier (PASID), a value which system software may assign to indi-vidual software threads.
Bit31 contains privilege identification (0= user; 1= supervisor).
Devices implementing enqueue registers may use these two values along with a device-specific command in the upper 60 bytes.The ENQCMD instruction begins by reading 64 bytes of command data from its source memory operand.
This is an ordinary load with cacheability and memory ordering implied normally by the memory type.
The source operand need not be aligned, and there is no guarantee that all 64 bytes are loaded atomically.
Bits 31:0 of the source operand must be zero.The instruction then formats those 64 bytes into command data with a format consistent with that given in Figure3-16: - 1Command[19:0] get IA32_PASID[19:0].
- Command[30:20] are zero.
- Command[31] is 0 (indicating user; this value is used regardless of CPL).
- Command[511:32] get bits511:32 of the source operand that was read from memory.The ENQCMD instruction uses an enqueue store (defined below) to write this command data to the destination operand.
The address of the destination operand is specified in a general-purpose register as an offset into the ES 2segment (the segment cannot be overridden).
The destination linear address must be 64-byte aligned.
The oper-ation of an enqueue store disregards the memory type of the destination memory address.

## Flags affected

- The ZF flag is set if the enqueue-store completion returns the retry status; otherwise it is cleared. All other flags are cleared.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #GP(0) - For an illegal memory operand effective address in the CS, DS, ES, FS or GS segments.
  > If destination linear address is not aligned to a 64-byte boundary.
  > If the PASID Valid field (bit 31) is 0 in IA32_PASID MSR.
  > If bits 31:0 of the source operand are not all zero.
  - #SS(0) - For an illegal address in the SS segment.
  - #PF(fault-code) - For a page fault.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in real-address mode. Additionally:
  - #PF(fault-code) - For a page fault.
- Real-Address Mode Exceptions
  - #GP - If any part of the operand lies outsid
  > e the effective address space from 0 to FFFFH.
  > If destination linear address is not aligned to a 64-byte boundary.
  > If the PASID Valid field (bit 31) is 0 in IA32_PASID MSR.
  > If bits 31:0 of the source operand are not all zero.
  - #UD - If CPUID.07H.0H:ECX.ENQCMD[bit 29] = 0.
  > If the LOCK prefix is used.
- SIMD Floating-Point Exceptions
  > None.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in non-canonical form.
  - #GP(0) - If the memory address is in non-canonical form.
  > If destination linear address is not aligned to a 64-byte boundary.
  > If the PASID Valid field (bit 31) is 0 in IA32_PASID MSR.
  > If bits 31:0 of the source operand are not all zero.
  - #PF(fault-code) - For a page fault.

## Operation

```C
IF IA32_PASID[31] = 0THEN #GP;ELSE:=COMMAND  (SRC & ~FFFFFFFFH) | (IA32_PASID & FFFFFH);:=DEST  COMMAND;FI;Intel C/C++ Compiler Intrinsic EquivalentENQCMD int_enqcmd(void *dst, const void *src)
```

# ENQCMDS

Enqueue Command Supervisor

The ENQCMDS instruction allows system software to write commands to enqueue registers, which are special device registers accessed using memory-mapped I/O (MMIO).Enqueue registers expect writes to have the format given in Figure3-16 and explained in the section on "ENQCMD-Enqueue Command."The ENQCMDS instruction begins by reading 64 bytes of command data from its source memory operand.
This is an ordinary load with cacheability and memory ordering implied normally by the memory type.
The source operand need not be aligned, and there is no guarantee that all 64 bytes are loaded atomically.
Bits 30:20 of the source operand must be zero.ENQCMDS formats its source data differently from ENQCMD.
Specifically, it formats them into command data as follows: - Command[19:0] get bits19:0 of the source operand that was read from memory.
These 20 bits communicate a process address-space identifier (PASID).
 - Command[30:20] are zero.
- Command[511:31] get bits511:31 of the source operand that was read from memory.
Bit31 communicates a privilege identification (0 = user; 1 = supervisor).The ENQCMDS instruction then uses an enqueue store (defined below) to write this command data to the desti-nation operand.
The address of the destination operand is specified in a general-purpose register as an offset into 1 The destination linear address must be 64-byte aligned.
The the ES segment (the segment cannot be overridden).operation of an enqueue store disregards the memory type of the destination memory address.An enqueue store is not ordered relative to older stores to WB or WC memory (including non-temporal stores) or to executions of the CLFLUSHOPT or CLWB (when applied to addresses other than that of the enqueue store).
Soft-ware can enforce such ordering by executing a fencing instruction such as SFENCE or MFENCE before the enqueue store.An enqueue store does not write the data into the cache hierarchy, nor does it fetch any data into the cache hier-archy.
An enqueue store's command data is never combined with that of any other store to the same address.Unlike other stores, an enqueue store returns a status, which the ENQCMDS instruction loads into the ZF flag in the RFLAGS register: - ZF= 0 (success) reports that the 64-byte command data was written atomically to a device's enqueue register and has been accepted by the device.
(It does not guarantee that the device has acted on the command; it may have queued it for later execution.) - ZF= 1 (retry) reports that the command data was not accepted.
This status is returned if the destination address is an enqueue register but the command was not accepted due to capacity or other temporal reasons.
1.In 64-bit mode, the width of the register operand is 64 bits (32 bits with a 67H prefix).
Outside 64-bit mode when CS.D = 1, the width is 32 bits (16 bits with a 67H prefix).
Outside 64-bit mode when CS.D=0, the widtThis status is also returned if the destination address was not an enqueue register (including the case of a memory address); in these cases, the store is dropped and is written neither to MMIO nor to memory.The ENQCMDS instruction may be executed only if CPL= 0.
Availability of the ENQCMDS instruction is indicated by the presence of the CPUID feature flag ENQCMD (CPUID.(EAX=07H, ECX=0H):ECX[bit 29]).

## Flags affected

- The ZF flag is set if the enqueue-store completion returns the retry status; otherwise it is cleared. All other flags are cleared.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - For an illegal memory operand effective address in the CS, DS, ES, FS or GS segments.
  > If destination linear address is not aligned to a 64-byte boundary.
  > If the current privilege level is not 0.
  > If bits 30:20 of the source operand are not all zero.
  - #SS(0) - For an illegal address in the SS segment.
  - #PF(fault-code) - For a page fault.
  - #UD - If CPUID.07H.0H:ECX.ENQCMD[bit 29] = 0.
  > If the LOCK prefix is used.
- SIMD Floating-Point Exceptions
  > None.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in non-canonical form.
  - #GP(0) - If the memory address is in non-canonical form.
  > If destination linear address is not aligned to a 64-byte boundary.
  > If the current privilege level is not 0.
- Real-Address Mode Exceptions
  - #GP - If any part of the operand lies outsid
  > e the effective address space from 0 to FFFFH.
  > If destination linear address is not aligned to a 64-byte boundary.
  > If bits 30:20 of the source operand are not all zero.
  - #UD - If CPUID.07H.0H:ECX.ENQCMD[bit 29] = 0.
  > If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - The ENQCMDS instruction is not recognized in virtual-8086 mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
:=DEST  SRC;Intel C/C++ Compiler Intrinsic EquivalentENQCMDS int_enqcmds(void *dst, const void *src)
```

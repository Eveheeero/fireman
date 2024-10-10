# CLFLUSHOPT

Flush Cache Line Optimized

Invalidates from every level of the cache hierarchy in the cache coherence domain the cache line that contains the linear address specified with the memory operand.
If that cache line contains modified data at any level of the cache hierarchy, that data is written back to memory.
The source operand is a byte memory location.The availability of CLFLUSHOPT is indicated by the presence of the CPUID feature flag CLFLUSHOPT (CPUID.(EAX=7,ECX=0):EBX[bit 23]).
The aligned cache line size affected is also indicated with the CPUID instruc-tion (bits 8 through 15 of the EBX register when the initial value in the EAX register is 1).The memory attribute of the page containing the affected line has no effect on the behavior of this instruction.
It should be noted that processors are free to speculatively fetch and cache data from system memory regions assigned a memory-type allowing for speculative reads (such as, the WB, WC, and WT memory types).
PREFETCHh instructions can be used to provide the processor with hints for this speculative behavior.
Because this speculative fetching can occur at any time and is not tied to instruction execution, the CLFLUSH instruction is not ordered with respect to PREFETCHh instructions or any of the speculative fetching mechanisms (that is, data can be specula-tively loaded into a cache line just before, during, or after the execution of a CLFLUSH instruction that references the cache line).Executions of the CLFLUSHOPT instruction are ordered with respect to fence instructions and to locked read-modify-write instructions; they are also ordered with respect to older writes to the cache line being invalidated.
They are not ordered with respect to other executions of CLFLUSHOPT, to executions of CLFLUSH and CLWB, or to younger writes to the cache line being invalidated.
Software can use the SFENCE instruction to order an execution of CLFLUSHOPT relative to one of those operations.The CLFLUSHOPT instruction can be used at all privilege levels and is subject to all permission checking and faults associated with a byte load (and in addition, a CLFLUSHOPT instruction is allowed to flush a linear address in an execute-only segment).
Like a load, the CLFLUSHOPT instruction sets the A bit but not the D bit in the page tables.In some implementations, the CLFLUSHOPT instruction may always cause transactional abort with Transactional Synchronization Extensions (TSX).
The CLFLUSHOPT instruction is not expected to be commonly used inside typical transactional regions.
However, programmers must not rely on CLFLUSHOPT instruction to force a transactional abort, since whether they cause transactional abort is implementation dependent.CLFLUSHOPT operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - For a page fault.
  - #UD - If CPUID.(EAX=7,ECX=0):EBX.CLFLUSHOPT[bit 23] = 0.
- Real-Address Mode Exceptions
  - #GP - If any part of the operand lies outside
  > the effective address space from 0 to FFFFH.
  - #UD - If CPUID.(EAX=7,ECX=0):EBX.CLFLUSHOPT[bit 23] = 0.
  > If the LOCK prefix is used.
  > If an instruction prefix F2H or F3H is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #GP(0) - For an illegal memory operand effective address in the CS, DS, ES, FS or GS segments.
  - #SS(0) - For an illegal address in the SS segment.
  - #PF(fault-code) - For a page fault.
  - #UD - If CPUID.(EAX=7,ECX=0):EBX.CLFLUSHOPT[bit 23] = 0.
  > If the LOCK prefix is used.
  > If an instruction prefix F2H or F3H is used.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in real address mode.
  - #PF(fault-code) - For a page fault.

## Operation

```C
Flush_Cache_Line_Optimized(SRC);Intel C/C++
```

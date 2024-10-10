# CLWB

Cache Line Write Back

Writes back to memory the cache line (if modified) that contains the linear address specified with the memory operand from any level of the cache hierarchy in the cache coherence domain.
The line may be retained in the cache hierarchy in non-modified state.
Retaining the line in the cache hierarchy is a performance optimization (treated as a hint by hardware) to reduce the possibility of cache miss on a subsequent access.
Hardware may choose to retain the line at any of the levels in the cache hierarchy, and in some cases, may invalidate the line from the cache hierarchy.
The source operand is a byte memory location.
The availability of CLWB instruction is indicated by the presence of the CPUID feature flag CLWB (bit 24 of the EBX register, see "CPUID - CPU Identification" in this chapter).
The aligned cache line size affected is also indicated with the CPUID instruction (bits 8 through 15 of the EBX register when the initial value in the EAX register is 1).The memory attribute of the page containing the affected line has no effect on the behavior of this instruction.
It should be noted that processors are free to speculatively fetch and cache data from system memory regions that are assigned a memory-type allowing for speculative reads (such as, the WB, WC, and WT memory types).
PREFETCHh instructions can be used to provide the processor with hints for this speculative behavior.
Because this speculative fetching can occur at any time and is not tied to instruction execution, the CLWB instruction is not ordered with respect to PREFETCHh instructions or any of the speculative fetching mechanisms (that is, data can be speculatively loaded into a cache line just before, during, or after the execution of a CLWB instruction that refer-ences the cache line).
Executions of the CLWB instruction are ordered with respect to fence instructions and to locked read-modify-write instructions; they are also ordered with respect to older writes to the cache line being written back.
They are not ordered with respect to other executions of CLWB, to executions of CLFLUSH and CLFLUSHOPT, or to younger writes to the cache line being written back.
Software can use the SFENCE instruction to order an execution of CLWB relative to one of those operations.For usages that require only writing back modified data from cache lines to memory (do not require the line to be invalidated), and expect to subsequently access the data, software is recommended to use CLWB (with appropriate fencing) instead of CLFLUSH or CLFLUSHOPT for improved performance.The CLWB instruction can be used at all privilege levels and is subject to all permission checking and faults associ-ated with a byte load.
Like a load, the CLWB instruction sets the accessed flag but not the dirty flag in the page tables.In some implementations, the CLWB instruction may always cause transactional abort with Transactional Synchro-nization Extensions (TSX).
CLWB instruction is not expected to be commonly used inside typical transactional regions.
However, programmers must not rely on CLWB instruction to force a transactional abort, since whether they cause transactional abort is implementation dependent.

## Flags affected

- None.C/C++ Compiler Intrinsic EquivalentCLWB void _mm_clwb(void const *p);

## Exceptions

- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.(EAX=07H, ECX=
  > 0H):EBX.CLWB[bit 24] = 0.
  - #GP - If any part of the operand lies outside
  > the effective address space from 0 to FFFFH.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.(EAX=07H, ECX=
  > 0H):EBX.CLWB[bit 24] = 0.
  - #GP(0) - For an illegal memory operand effective address in the CS, DS, ES, FS or GS segments.
  - #SS(0) - For an illegal address in the SS segment.
  - #PF(fault-code) - For a page fault.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in real address mode.
  - #PF(fault-code) - For a page fault.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.(EAX=07H, ECX=
  > 0H):EBX.CLWB[bit 24] = 0.
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

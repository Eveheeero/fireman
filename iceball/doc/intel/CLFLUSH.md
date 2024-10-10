# CLFLUSH

Flush Cache Line

Invalidates from every level of the cache hierarchy in the cache coherence domain the cache line that contains the linear address specified with the memory operand.
If that cache line contains modified data at any level of the cache hierarchy, that data is written back to memory.
The source operand is a byte memory location.The availability of CLFLUSH is indicated by the presence of the CPUID feature flag CLFSH (CPUID.01H:EDX[bit19]).
The aligned cache line size affected is also indicated with the CPUID instruction (bits 8 through 15 of the EBX register when the initial value in the EAX register is 1).The memory attribute of the page containing the affected line has no effect on the behavior of this instruction.
It should be noted that processors are free to speculatively fetch and cache data from system memory regions assigned a memory-type allowing for speculative reads (such as, the WB, WC, and WT memory types).
PREFETCHh instructions can be used to provide the processor with hints for this speculative behavior.
Because this speculative fetching can occur at any time and is not tied to instruction execution, the CLFLUSH instruction is not ordered with respect to PREFETCHh instructions or any of the speculative fetching mechanisms (that is, data can be specula-tively loaded into a cache line just before, during, or after the execution of a CLFLUSH instruction that references the cache line).Executions of the CLFLUSH instruction are ordered with respect to each other and with respect to writes, locked 1 They are not ordered with respect to executions of read-modify-write instructions, and fence instructions.CLFLUSHOPT and CLWB.
Software can use the SFENCE instruction to order an execution of CLFLUSH relative to one of those operations.The CLFLUSH instruction can be used at all privilege levels and is subject to all permission checking and faults asso-ciated with a byte load (and in addition, a CLFLUSH instruction is allowed to flush a linear address in an execute-only segment).
Like a load, the CLFLUSH instruction sets the A bit but not the D bit in the page tables.In some implementations, the CLFLUSH instruction may always cause transactional abort with Transactional Synchronization Extensions (TSX).
The CLFLUSH instruction is not expected to be commonly used inside typical transactional regions.
However, programmers must not rely on CLFLUSH instruction to force a transactional abort, since whether they cause transactional abort is implementation dependent.The CLFLUSH instruction was introduced with the SSE2 extensions; however, because it has its own CPUID feature flag, it can be implemented in IA-32 processors that do not include the SSE2 extensions.
Also, detecting the pres-ence of the SSE2 extensions with the CPUID instruction does not guarantee that the CLFLUSH instruction is imple-mented in the processor.CLFLUSH operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - For an illegal memory operand effective address in the CS, DS, ES, FS or GS segments.
  - #SS(0) - For an illegal address in the SS segment.
  - #PF(fault-code) - For a page fault.
  - #UD - If CPUID.01H:EDX.CLFSH[bit 19] = 0.
  > If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - For a page fault.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in real address mode.
  - #PF(fault-code) - For a page fault.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #GP - If any part of the operand lies outside
  > the effective address space from 0 to FFFFH.
  - #UD - If CPUID.01H:EDX.CLFSH[bit 19] = 0.
  > If the LOCK prefix is used.

## Operation

```C
Flush_Cache_Line(SRC);Intel C/C++ Compiler Intrinsic EquivalentsCLFLUSH void _mm_clflush(void const *p)
```

# CLDEMOTE

Cache Line Demote

Hints to hardware that the cache line that contains the linear address specified with the memory operand should be moved ("demoted") from the cache(s) closest to the processor core to a level more distant from the processor core.
This may accelerate subsequent accesses to the line by other cores in the same coherence domain, especially if the line was written by the core that demotes the line.
Moving the line in such a manner is a performance optimization, i.e., it is a hint which does not modify architectural state.
Hardware may choose which level in the cache hierarchy to retain the line (e.g., L3 in typical server designs).
The source operand is a byte memory location.
The availability of the CLDEMOTE instruction is indicated by the presence of the CPUID feature flag CLDEMOTE (bit 25 of the ECX register in sub-leaf 07H, see "CPUID-CPU Identification").
On processors which do not support the CLDEMOTE instruction (including legacy hardware) the instruction will be treated as a NOP.A CLDEMOTE instruction is ordered with respect to stores to the same cache line, but unordered with respect to other instructions including memory fences, CLDEMOTE, CLWB or CLFLUSHOPT instructions to a different cache line.
Since CLDEMOTE will retire in order with respect to stores to the same cache line, software should ensure that after issuing CLDEMOTE the line is not accessed again immediately by the same core to avoid cache data move-ment penalties.
The effective memory type of the page containing the affected line determines the effect; cacheable types are likely to generate a data movement operation, while uncacheable types may cause the instruction to be ignored.
Speculative fetching can occur at any time and is not tied to instruction execution.
The CLDEMOTE instruction is not ordered with respect to PREFETCHh instructions or any of the speculative fetching mechanisms.
That is, data can be speculatively loaded into a cache line just before, during, or after the execution of a CLDEMOTE instruction that references the cache line.Unlike CLFLUSH, CLFLUSHOPT, and CLWB instructions, CLDEMOTE is not guaranteed to write back modified data to memory.
The CLDEMOTE instruction may be ignored by hardware in certain cases and is not a guarantee.
The CLDEMOTE instruction can be used at all privilege levels.
In certain processor implementations the CLDEMOTE instruction may set the A bit but not the D bit in the page tables.
If the line is not found in the cache, the instruction will be treated as a NOP.
In some implementations, the CLDEMOTE instruction may always cause a transactional abort with Transactional Synchronization Extensions (TSX).
However, programmers must not rely on CLDEMOTE instruction to force a trans-

## Flags affected

- None.C/C++ Compiler Intrinsic EquivalentCLDEMOTE void _cldemote(const void*);

## Exceptions

- Virtual-8086 Mode Exceptions
  > Same exceptions as in real address mode.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
Cache_Line_Demote(m8);
```

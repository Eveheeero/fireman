# PREFETCHW

Prefetch Data Into Caches in Anticipation of a Write

Fetches the cache line of data from memory that contains the byte specified with the source operand to a location in the 1st or 2nd level cache and invalidates other cached instances of the line.The source operand is a byte memory location.
If the line selected is already present in the lowest level cache and is already in an exclusively owned state, no data movement occurs.
Prefetches from non-writeback memory are ignored.The PREFETCHW instruction is merely a hint and does not affect program behavior.
If executed, this instruction moves data closer to the processor and invalidates other cached copies in anticipation of the line being written to in the future.The characteristic of prefetch locality hints is implementation-dependent, and can be overloaded or ignored by a processor implementation.
The amount of data prefetched is also processor implementation-dependent.
It will, however, be a minimum of 32 bytes.
Additional details of the implementation-dependent locality hints are described in Section 7.4 of Intel® 64 and IA-32 Architectures Optimization Reference Manual.It should be noted that processors are free to speculatively fetch and cache data with exclusive ownership from system memory regions that permit such accesses (that is, the WB memory type).
A PREFETCHW instruction is considered a hint to this speculative behavior.
Because this speculative fetching can occur at any time and is not tied to instruction execution, a PREFETCHW instruction is not ordered with respect to the fence instructions (MFENCE, SFENCE, and LFENCE) or locked memory references.
A PREFETCHW instruction is also unordered with respect to CLFLUSH and CLFLUSHOPT instructions, other PREFETCHW instructions, or any other general instructionIt is ordered with respect to serializing instructions such as CPUID, WRMSR, OUT, and MOV CR.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Flags affected

- None.C/C++ Compiler Intrinsic Equivalentvoid _m_prefetchw( void * );

## Exceptions

- Compatibility Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.

## Operation

```C
FETCH_WITH_EXCLUSIVE_OWNERSHIP (m8);
```

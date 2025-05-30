# MOVNTI

Store Doubleword Using Non-Temporal Hint

Moves the doubleword integer in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to minimize cache pollution during the write to memory.
The source operand is a general-purpose register.
The destination operand is a 32-bit memory location.The non-temporal hint is implemented by using a write combining (WC) memory type protocol when writing the data to memory.
Using this protocol, the processor does not write the data into the cache hierarchy, nor does it fetch the corresponding cache line from memory into the cache hierarchy.
The memory type of the region being written to can override the non-temporal hint, if the memory address specified for the non-temporal store is in an uncacheable (UC) or write protected (WP) memory region.
For more information on non-temporal stores, see ® 64 and IA-32 Architectures Software "Caching of Temporal vs.
Non-Temporal Data" in Chapter 10 in the IntelDeveloper's Manual, Volume 1.Because the WC protocol uses a weakly-ordered memory consistency model, a fencing operation implemented with the SFENCE or MFENCE instruction should be used in conjunction with MOVNTI instructions if multiple processors might use different memory types to read/write the destination memory locations.In 64-bit mode, the instruction's default operation size is 32 bits.
Use of the REX.R prefix permits access to addi-tional registers (R8-R15).
Use of the REX.W prefix promotes operation to 64 bits.
See the summary chart at the beginning of this section for encoding data and limits.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - For an illegal memory operand effective address in the CS, DS, ES, FS or GS segments.
  - #SS(0) - For an illegal address in the SS segment.
  - #PF(fault-code) - For a page fault.
- SIMD Floating-Point Exceptions
  > None.
- Real-Address Mode Exceptions
  - #GP - If any part of the operand lies outside
  > the effective address space from 0 to FFFFH.
  - #UD - If CPUID.01H:EDX.SSE2[bit 26] = 0.
  > If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the memory address is in a non-canonical form.
  - #PF(fault-code) - For a page fault.
  - #UD - If CPUID.01H:EDX.SSE2[bit 26] = 0.
  > If the LOCK prefix is used.
  - #AC(0) - If alignment checking is enabled and an
- Virtual-8086 Mode Exceptions
  > Same exceptions as in real address mode.
  - #PF(fault-code) - For a page fault.

## Operation

```C
DEST := SRC;Intel C/C++ Compiler Intrinsic EquivalentMOVNTI void _mm_stream_si32 (int *p, int a)MOVNTI void _mm_stream_si64(__int64 *p, __int64 a)
```

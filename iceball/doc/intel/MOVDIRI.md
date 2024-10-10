# MOVDIRI

Move Doubleword as Direct Store

Moves the doubleword integer in the source operand (second operand) to the destination operand (first operand) using a direct-store operation.
The source operand is a general purpose register.
The destination operand is a 32-bit memory location.
In 64-bit mode, the instruction's default operation size is 32 bits.
Use of the REX.R prefix permits access to additional registers (R8-R15).
Use of the REX.W prefix promotes operation to 64 bits.
See summary chart at the beginning of this section for encoding data and limits.The direct-store is implemented by using write combining (WC) memory type protocol for writing data.
Using this protocol, the processor does not write the data into the cache hierarchy, nor does it fetch the corresponding cache line from memory into the cache hierarchy.
If the destination address is cached, the line is written-back (if modi-fied) and invalidated from the cache, before the direct-store.
Unlike stores with non-temporal hint that allow uncached (UC) and write-protected (WP) memory-type for the destination to override the non-temporal hint, direct-stores always follow WC memory type protocol irrespective of the destination address memory type (including UC and WP types).Unlike WC stores and stores with non-temporal hint, direct-stores are eligible for immediate eviction from the write-combining buffer, and thus not combined with younger stores (including direct-stores) to the same address.
Older WC and non-temporal stores held in the write-combing buffer may be combined with younger direct stores to the same address.
Direct stores are weakly ordered relative to other stores.
Software that desires stronger ordering should use a fencing instruction (MFENCE or SFENCE) before or after a direct store to enforce the ordering desired.Direct-stores issued by MOVDIRI to a destination aligned to a 4-byte boundary (8-byte boundary if used with REX.W prefix) guarantee 4-byte (8-byte with REX.W prefix) write-completion atomicity.
This means that the data arrives at the destination in a single undivided 4-byte (or 8-byte) write transaction.
If the destination is not aligned for the write size, the direct-stores issued by MOVDIRI are split and arrive at the destination in two parts.
Each part of such split direct-store will not merge with younger stores but can arrive at the destination in either order.
Avail-ability of the MOVDIRI instruction is indicated by the presence of the CPUID feature flag MOVDIRI (bit 27 of the ® 64 and IA-32 Architectures Software Devel-ECX register in leaf 07H, see "CPUID-CPU Identification" in the Inteloper's Manual, Volume 2A).

## Exceptions

- 64-Bit Mode Exceptions
  - #SS(0) - If memory address referencing the SS segment is in non-canonical form.
  - #GP(0) - If the memory address is in non-canonical form.
  - #PF (fault-code) - For a page fault.
  - #UD - If CPUID.07H.0H:ECX.MOVDIRI[bit 27] = 0.
  > If LOCK prefix or operand-size (66H) prefix is used.
  - #AC - If alignment checking is enabled and an unali
- Real-Address Mode Exceptions
  - #GP - If any part of the operand lies outside
  > the effective address space from 0 to FFFFH.
  - #UD - If CPUID.07H.0H:ECX.MOVDIRI[bit 27] = 0.
  > If LOCK prefix or operand-size (66H) prefix is used.
- Protected Mode Exceptions
  - #GP(0) - For an illegal memory operand effective address in the CS, DS, ES, FS or GS segments.
  - #SS(0) - For an illegal address in the SS segment.
  - #PF (fault-code) - For a page fault.
  - #UD - If CPUID.07H.0H:ECX.MOVDIRI[bit 27] = 0.
  > If LOCK prefix or operand-size (66H) prefix is used.
  - #AC - If alignment checking is enabled and an unali
  > gned memory reference made while in current 
  > privilege level 3.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in real address mode.
  - #PF (fault-code) - For a page fault.
  - #AC - If alignment checking is enabled and an unali
  > gned memory reference made while in current 
  > privilege level 3.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
DEST := SRC;Intel C/C++ Compiler Intrinsic EquivalentMOVDIRI void _directstoreu_u32(void *dst, uint32_t val)
```

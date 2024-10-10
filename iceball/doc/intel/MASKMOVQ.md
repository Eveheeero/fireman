# MASKMOVQ

Store Selected Bytes of Quadword

Stores selected bytes from the source operand (first operand) into a 64-bit memory location.
The mask operand (second operand) selects which bytes from the source operand are written to memory.
The source and mask oper-ands are MMX technology registers.
The memory location specified by the effective address in the DI/EDI/RDI register (the default segment register is DS, but this may be overridden with a segment-override prefix).
The memory location does not need to be aligned on a natural boundary.
(The size of the store address depends on the address-size attribute.)The most significant bit in each byte of the mask operand determines whether the corresponding byte in the source operand is written to the corresponding byte location in memory: 0 indicates no write and 1 indicates write.
The MASKMOVQ instruction generates a non-temporal hint to the processor to minimize cache pollution.
The non-temporal hint is implemented by using a write combining (WC) memory type protocol (see "Caching of Temporal ® 64 and IA-32 Architectures Software Developer's Manual, vs.
Non-Temporal Data" in Chapter 10, of the IntelVolume 1).
Because the WC protocol uses a weakly-ordered memory consistency model, a fencing operation imple-mented with the SFENCE or MFENCE instruction should be used in conjunction with MASKMOVQ instructions if multiple processors might use different memory types to read/write the destination memory locations.This instruction causes a transition from x87 FPU to MMX technology state (that is, the x87 FPU top-of-stack pointer is set to 0 and the x87 FPU tag word is set to all 0s [valid]).The behavior of the MASKMOVQ instruction with a mask of all 0s is as follows: - No data will be written to memory.
 - Transition from x87 FPU to MMX technology state will occur.
- Exceptions associated with addressing memory and page faults may still be signaled (implementation dependent).
- Signaling of breakpoints (code or data) is not guaranteed (implementation dependent).
- If the destination memory region is mapped as UC or WP, enforcement of associated semantics for these memory types is not guaranteed (that is, is reserved) and is implementation-specific.
The MASKMOVQ instruction can be used to improve performance for algorithms that need to merge data on a byte-by-byte basis.
It should not cause a read for ownership; doing so generates unnecessary bandwidth since data is to be written directly using the byte-mask without allocating old data prior to the store.
In 64-bit mode, the memory address is specified by DS:RDI.

## Exceptions

- Other Exceptions
  > ®
  > See Table23-8, "Exception Conditions for Legacy SIMD/M
  > MX Instructions without FP Exception," in the Intel

## Operation

```C
= IF (MASK[7] 1)THEN DEST[DI/EDI] := SRC[7:0] ELSE (* Memory location unchanged *); FI;= IF (MASK[15] 1) THEN DEST[DI/EDI +1] := SRC[15:8] ELSE (* Memory location unchanged *); FI;(* Repeat operation for 3rd through 6th bytes in source operand *)=IF (MASK[63]  1) THEN DEST[DI/EDI +Intel C/C++ Compiler Intrinsic Equivalentvoid _mm_maskmove_si64(__m64d, __m64n, char * p)
```

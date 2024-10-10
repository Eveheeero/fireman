# MOVNTPD

Store Packed Double Precision Floating-Point Values Using Non-Temporal Hint

Moves the packed double precision floating-point values in the source operand (second operand) to the destination operand (first operand) using a non-temporal hint to prevent caching of the data during the write to memory.
The source operand is an XMM register, YMM register or ZMM register, which is assumed to contain packed double preci-sion, floating-pointing data.
The destination operand is a 128-bit, 256-bit or 512-bit memory location.
The memory operand must be aligned on a 16-byte (128-bit version), 32-byte (VEX.256 encoded version) or 64-byte (EVEX.512 encoded version) boundary otherwise a general-protection exception (#GP) will be generated.
The non-temporal hint is implemented by using a write combining (WC) memory type protocol when writing the data to memory.
Using this protocol, the processor does not write the data into the cache hierarchy, nor does it fetch the corresponding cache line from memory into the cache hierarchy.
The memory type of the region being written to can override the non-temporal hint, if the memory address specified for the non-temporal store is in an uncacheable (UC) or write protected (WP) memory region.
For more information on non-temporal stores, see "Caching of Temporal vs.
Non-Temporal Data" in Chapter 10 in the IA-32 Intel Architecture Software Developer's Manual, Volume 1.Because the WC protocol uses a weakly-ordered memory consistency model, a fencing operation implemented with the SFENCE or MFENCE instruction should be used in conjunction with MOVNTPD instructions if multiple processors might use different memory types to read/write the destination memory locations.Note: VEX.vvvv and EVEX.vvvv are reserved and must be 1111b, VEX.L must be 0; otherwise instructions will #UD.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Exceptions Type1.SS
  > E2 in Table2-18, "Type 1 Class Exception Conditions."
  > EVEX-encoded instruction, see Table2-45, "Type E1NF Class Exception Conditions."
  > Additionally:

## Operation

```C
VMOVNTPD (EVEX Encoded Versions) VL = 128, 256, 512DEST[VL-1:0] := SRC[VL-1:0]MOVNTPD (Legacy and VEX Versions)DEST := SRCIntel C/C++ Compiler Intrinsic EquivalentVMOVNTPD void _mm512_stream_pd(double * p, __m512d a);VMOVNTPD void _mm256_stream_pd (double * p, __m256d a);MOVNTPD void _mm_stream_pd (double * p, __m128d a);
```

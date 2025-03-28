# MOVNTDQA

Load Double Quadword Non-Temporal Aligned Hint

MOVNTDQA loads a double quadword from the source operand (second operand) to the destination operand (first operand) using a non-temporal hint if the memory source is WC (write combining) memory type.
For WC memory type, the nontemporal hint may be implemented by loading a temporary internal buffer with the equivalent of an aligned cache line without filling this data to the cache.
Any memory-type aliased lines in the cache will be snooped and flushed.
Subsequent MOVNTDQA reads to unread portions of the WC cache line will receive data from the temporary internal buffer if data is available.
The temporary internal buffer may be flushed by the processor at any time for any reason, for example: -  A load operation other than a MOVNTDQA which references memory already resident in a temporary internal buffer.
-  A non-WC reference to memory already resident in a temporary internal buffer.
-  Interleaving of reads and writes to a single temporary internal buffer.
-  Repeated (V)MOVNTDQA loads of a particular 16-byte item in a streaming line.
-  Certain micro-architectural conditions including resource shortages, detection ofa mis-speculation condition, and various fault conditionsThe non-temporal hint is implemented by using a write combining (WC) memory type protocol when reading the data from memory.
Using this protocol, the processor does not read the data into the cache hierarchy, nor does it fetch the corresponding cache line from memory into the cache hierarchy.
The memory type of the region being read can override the non-temporal hint, if the memory address specified for the non-temporal read is not a WC memory region.
Information on non-temporal reads and writes can be found in "Caching of Temporal vs.
Non-Temporal Data" in Chapter 10 in the Intel® 64 and IA-32 Architecture Software Developer's Manual, Volume 3A.Because the WC protocol uses a weakly-ordered memory consistency model, a fencing operation implemented with a MFENCE instruction should be used in conjunction with MOVNTDQA instructions if multiple processors might use different memory types for the referenced memory locations or to synchronize reads of a processor with writes by other agents in the system.
A processor's implementation of the streaming load hint does not override the effective memory type, but the implementation of the hint is prtion may choose to ignore the hint and process the instruction as a normal MOVDQA for any memory type.
Alter-natively, another implementation may optimize cache reads generated by MOVNTDQA on WB memory type to reduce cache evictions.The 128-bit (V)MOVNTDQA addresses must be 16-byte aligned or the instruction will cause a #GP.The 256-bit VMOVNTDQA addresses must be 32-byte aligned or the instruction will cause a #GP.The 512-bit VMOVNTDQA addresses must be 64-byte aligned or the instruction will cause a #GP.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  > Non-EVEX-encoded instruction, see Table2-18, "Type 1 Class Exception Conditions."
  > EVEX-encoded instruction, see Table2-45, "Type E1NF Class Exception Conditions."
  > Additionally:

## Operation

```C
MOVNTDQA (128bit- Legacy SSE Form)DEST := SRCDEST[MAXVL-1:128] (Unmodified)VMOVNTDQA (VEX.128 and EVEX.128 Encoded Form)DEST := SRCDEST[MAXVL-1:128] := 0 VMOVNTDQA (VEX.256 and EVEX.256 Encoded Forms)DEST[255:0] := SRC[255:0]DEST[MAXVL-1:256] := 0VMOVNTDQA (EVEX.512 Encoded Form)DEST[511:0] := SRC[511:0]DEST[MAXVL-1:512] := 0Intel C/C++ Compiler Intrinsic EquivalentVMOVNTDQA __m512i _mm512_stream_load_si512(__m512i const* p);MOVNTDQA __m128i _mm_stream_load_si128 (const __m128i *p);VMOVNTDQA __m256i _mm256_stream_load_si256 (__m256i const* p);
```

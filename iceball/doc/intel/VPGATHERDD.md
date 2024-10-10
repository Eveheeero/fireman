# VPGATHERDD/VPGATHERDQ

Gather Packed Dword, Packed Qword With Signed Dword Indices

A set of 16 or 8 doubleword/quadword memory locations pointed to by base address BASE_ADDR and index vector VINDEX with scale SCALE are gathered.
The result is written into vector zmm1.
The elements are specified via the VSIB (i.e., the index register is a zmm, holding packed indices).
Elements will only be loaded if their corresponding mask bit is one.
If an element's mask bit is not set, the corresponding element of the destination register (zmm1) is left unchanged.
The entire mask register will be set to zero by this instruction unless it triggers an exception.This instruction can be suspended by an exception if at least one element is already gathered (i.e., if the exception is triggered by an element other than the rightmost one with its mask bit set).
When this happens, the destination register and the mask register (k1) are partially updated; those elements that have been gathered are placed into the destination register and have their mask bits set to zero.
If any traps or interrupts are pending from already gathered elements, they will be delivered in lieu of the exception; in this case, EFLAG.RF is set to one so an instruc-tion breakpoint is not re-triggered when the instruction is continued.If the data element size is less than the index element size, the higher part of the destination register and the mask register do not correspond to any elements being gathered.
This instruction sets those higher parts to zero.
It may update these unused elements to one or both of those registers even if the instruction triggers an exception, and even if the instruction triggers the exception before gathering any elements.Note that: - The values may be read from memory in any order.
Memory ordering with other instructions follows the Intel-64 memory-ordering model.
- Faults are delivered in a right-to-left manner.
That is, if a fault is triggered by an element and delivered, all elements closer to the LSB of the destination zmm will be completed (and non-faulting).
Individual elements closer to the MSB may or may not be completed.
If a given element triggers multiple faults, they are delivered in the conventional order.
- Elements may be gathered in any order, but faults must be delivered in a right-to-left order; thus, elements to the left of a faulting one may be gathered before the fault is delivered.
A given implementation of this instruction is repeatable - given the same input values and architectural state, the same set of elements to the left of the faulting one will be gathered.
- This instruction does not perform AC checks, and so will never deliver an AC fault.
- These instructions do not accept zeroing-masking since the 0 values in k1 are used to determine completion.Note that the presence of VSIB byte is enforced in this instruction.
Hence, the instruction will #UD fault if ModRM.rm is different than 100b.This instruction has the same disp8*N and alignment rules as for scalar instructions (Tuple 1).The instruction will #UD fault if the destination vector zmm1 is the same as index vector VINDEX.
The instruction will #UD fault if the k0 mask register is specified.The scaled index may require more bits to represent than the address bits used by the processor (e.g., in 32-bit mode, if the scale is greater than one).
In this case, the most significant bits beyond the number of address bits are ignored.

## Exceptions

- SIMD Floating-Point Exceptions
  > None.

## Operation

```C
BASE_ADDR stands for the memory operand base address (a GPR); may not existVINDEX stands for the memory operand vector of indices (a ZMM register)SCALE stands for the memory operand scalar (1, 2, 4 or 8)DISP is the optional 1 or 4 byte displacementVPGATHERDD (EVEX encoded version)(KL, VL) = (4, 128), (8, 256), (16, 512)FOR j := 0 TO KL-1i := j * 32IF k1[j]THEN DEST[i+31:i] := MEM[BASE_ADDR +SignExtend(VINDEX[i+31:i]) * SCALE + DISP]k1[j] := 0ELSE *DEST[i+31:i] := remains unchanged*  ; Only merging masking is allowedFI;ENDFORk1[MAX_KL-1:KL] := 0DEST[MAXVL-1:VL] := 0VPGATHERDQ (EVEX encoded version)(KL, VL) = (2, 128), (4, 256), (8, 512)FOR j := 0 TO KL-1i := j * 64k := j * 32IF k1[j]THEN DEST[i+63:i] := MEM[BASE_ADDR + SignExtend(VINDEX[k+31:k]) * SCALE + DISP]k1[j] := 0ELSE *DEST[i+63:i] := remains unchanged*  ; Only merging masking is allowedFI;ENDFORIntel C/C++ Compiler Intrinsic EquivalentVPGATHERDD __m512i _mm512_i32gather_epi32( __m512i vdx, void * base, int scale);VPGATHERDD __m512i _mm512_mask_i32gather_epi32(__m512i s, __mmask16 k, __m512i vdx, void * base, int scale);VPGATHERDD __m256i _mm256_mmask_i32gather_epi32(__m256i s, __mmask8 k, __m256i vdx, void * base, int scale);VPGATHERDD __m128i _mm_mmask_i32gather_epi32(__m128i s, __mmask8 k, __m128i vdx, void * base, int scale);VPGATHERDQ __m512i _mm512_i32logather_epi64( __m256i vdx, void * base, int scale);VPGATHERDQ __m512i _mm512_mask_i32logather_epi64(__m512i s, __mmask8 k, __m256i vdx, void * base, int scale);VPGATHERDQ __m256i _mm256_mmask_i32logather_epi64(__m256i s, __mmask8 k, __m128i vdx, void * base, int scale);VPGATHERDQ __m128i _mm_mmask_i32gather_epi64(__m128i s, __mmask8 k, __m128i vdx, void * base, int scale);
```

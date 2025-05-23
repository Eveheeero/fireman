# MFENCE

Memory Fence

Performs a serializing operation on all load-from-memory and store-to-memory instructions that were issued prior the MFENCE instruction.
This serializing operation guarantees that every load and store instruction that precedes the MFENCE instruction in program order becomes globally visible before any load or store instruction that follows 1 The MFENCE instruction is ordered with respect to all load and store instructions, other the MFENCE instruction.MFENCE instructions, any LFENCE and SFENCE instructions, and any serializing instructions (such as the CPUID instruction).
MFENCE does not serialize the instruction stream.Weakly ordered memory types can be used to achieve higher processor performance through such techniques as out-of-order issue, speculative reads, write-combining, and write-collapsing.
The degree to which a consumer of data recognizes or knows that the data is weakly ordered varies among applications and may be unknown to the producer of this data.
The MFENCE instruction provides a performance-efficient way of ensuring load and store ordering between routines that produce weakly-ordered results and routines that consume that data.Processors are free to fetch and cache data speculatively from regions of system memory that use the WB, WC, and WT memory types.
This speculative fetching can occur at any time and is not tied to instruction execution.
Thus, it is not ordered with respect to executions of the MFENCE instruction; data can be brought into the caches specula-tively just before, during, or after the execution of an MFENCE instruction.This instruction's operation is the same in non-64-bit modes and 64-bit mode.Specification of the instruction's opcode above indicates a ModR/M byte of F0.
For this instruction, the processor ignores the r/m field of the ModR/M byte.
Thus, MFENCE is encoded by any opcode of the form 0F AE Fx, where x is in the range 0-7.

## Operation

```C
Wait_On_Following_Loads_And_Stores_Until(preceding_loads_and_stores_globally_visible);Intel C/C++ Compiler Intrinsic Equivalentvoid _mm_mfence(void)Exceptions (All Modes of Operation)#UD If CPUID.01H:EDX.SSE2[bit 26] = 0.If the LOCK prefix is used.
```

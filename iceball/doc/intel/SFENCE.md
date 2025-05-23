# SFENCE

Store Fence

Orders processor execution relative to all memory stores prior to the SFENCE instruction.
The processor ensures that every store prior to SFENCE is globally visible before any store after SFENCE becomes globally visible.
The SFENCE instruction is ordered with respect to memory stores, other SFENCE instructions, MFENCE instructions, and any serializing instructions (such as the CPUID instruction).
It is not ordered with respect to memory loads or the LFENCE instruction.
Weakly ordered memory types can be used to achieve higher processor performance through such techniques as out-of-order issue, write-combining, and write-collapsing.
The degree to which a consumer of data recognizes or knows that the data is weakly ordered varies among applications and may be unknown to the producer of this data.
The SFENCE instruction provides a performance-efficient way of ensuring store ordering between routines that produce weakly-ordered results and routines that consume this data.This instruction's operation is the same in non-64-bit modes and 64-bit mode.Specification of the instruction's opcode above indicates a ModR/M byte of F8.
For this instruction, the processor ignores the r/m field of the ModR/M byte.
Thus, SFENCE is encoded by any opcode of the form 0F AE Fx, where x is in the range 8-F.

## Operation

```C
Wait_On_Following_Stores_Until(preceding_stores_globally_visible);Intel C/C++ Compiler Intrinsic Equivalentvoid _mm_sfence(void)Exceptions (All Operating Modes)
```

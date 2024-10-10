# SERIALIZE

Serialize Instruction Execution

Serializes instruction execution.
Before the next instruction is fetched and executed, the SERIALIZE instruction ensures that all modifications to flags, registers, and memory by previous instructions are completed, draining all buffered writes to memory.
This instruction is also a serializing instruction as defined in the section "Serializing ® 64 and IA-32 Architectures Software Developer's Manual, Volume 3A.Instructions" in Chapter 9 of the IntelSERIALIZE does not modify registers, arithmetic flags, or memory.


## Exceptions

- SIMD Floating-Point Exceptions
  > None.
- Other Exceptions
  - #UD - If the LOCK prefix is used.

## Operation

```C
Wait_On_Fetch_And_Execution_Of_Next_Instruction_Until(preceding_instructions_complete_and_preceding_stores_globally_visible);Intel C/C++ Compiler Intrinsic EquivalentSERIALIZE void _serialize(void);
```

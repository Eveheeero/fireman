# UD

Undefined Instruction

Generates an invalid opcode exception.
This instruction is provided for software testing to explicitly generate an invalid opcode exception.
The opcodes for this instruction are reserved for this purpose.Other than raising the invalid opcode exception, this instruction has no effect on processor state or memory.Even though it is the execution of the UD instruction that causes the invalid opcode exception, the instruction pointer saved by delivery of the exception references the UD instruction (and not the following instruction).This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Flags affected

- None.

## Operation

```C
#UD (* Generates invalid opcode exception *);
```

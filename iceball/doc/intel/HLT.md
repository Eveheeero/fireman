# HLT

Halt

Stops instruction execution and places the processor in a HALT state.
An enabled interrupt (including NMI and SMI), a debug exception, the BINIT# signal, the INIT# signal, or the RESET# signal will resume execution.
If an interrupt (including NMI) is used to resume execution after a HLT instruction, the saved instruction pointer (CS:EIP) points to the instruction following the HLT instruction.When a HLT instruction is executed on an Intel 64 or IA-32 processor supporting Intel Hyper-Threading Technology, only the logical processor that executes the instruction is halted.
The other logical processors in the physical processor remain active, unless they are each individually halted by executing a HLT instruction.The HLT instruction is a privileged instruction.
When the processor is running in protected or virtual-8086 mode, the privilege level of a program or procedure must be 0 to execute the HLT instruction.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  > None.

## Operation

```C
Enter Halt state;
```

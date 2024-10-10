# CLTS

Clear Task-Switched Flag in CR0

Clears the task-switched (TS) flag in the CR0 register.
This instruction is intended for use in operating-system procedures.
It is a privileged instruction that can only be executed at a CPL of 0.
It is allowed to be executed in real-address mode to allow initialization for protected mode.The processor sets the TS flag every time a task switch occurs.
The flag is used to synchronize the saving of FPU context in multitasking applications.
See the description of the TS flag in the section titled "Control Registers" in ® 64 and IA-32 Architectures Software Developer's Manual, Volume 3A, for more information Chapter 2 of the Intelabout this flag.
CLTS operation is the same in non-64-bit modes and 64-bit mode.See Chapter 26, "VMX Non-Root Operation," of the Intel® 64 and IA-32 Architectures Software Developer's Manual, Volume 3C, for more information about the behavior of this instruction in VMX non-root operation.

## Flags affected

- The TS flag in CR0 register is cleared.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #GP(0) - CLTS is not recognized in virtual-8086 mode.
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions

## Operation

```C
CR0.TS[bit 3] := 0;
```

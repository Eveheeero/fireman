# WAIT/FWAIT

Wait

Causes the processor to check for and handle pending, unmasked, floating-point exceptions before proceeding.
(FWAIT is an alternate mnemonic for WAIT.)This instruction is useful for synchronizing exceptions in critical sections of code.
Coding a WAIT instruction after a floating-point instruction ensures that any unmasked floating-point exceptions the instruction may raise are handled before the processor can modify the instruction's results.
See the section titled "Floating-Point Exception ® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for Synchronization" in Chapter 8 of the Intelmore information on using the WAIT/FWAIT instruction.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #NM - If CR0.MP[bit 1] = 1 and CR0.TS[bit 3] = 1.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.
- Floating-Point Exceptions
  > None. 
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
CheckForPendingUnmaskedFloatingPointExceptions;FPU Flags AffectedThe C0, C1, C2, and C3 flags are undefined.
```

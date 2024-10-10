# CLUI

Clear User Interrupt Flag

CLUI clears the user interrupt flag (UIF).
Its effect takes place immediately: a user interrupt cannot be delivered on the instruction boundary following CLUI.An execution of CLUI inside a transactional region causes a transactional abort; the abort loads EAX as it would have had it been caused due to an execution of CLI.

## Flags affected

- None.

## Exceptions

- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If executed inside an enclave.
- Virtual-8086 Mode Exceptions
  - #UD - The CLUI instruction is not
  > recognized in virtual-8086 mode.
- Protected Mode Exceptions
  - #UD - The CLUI instruction is not
  > recognized in protected mode.
- Real-Address Mode Exceptions
  - #UD - The CLUI instruction is not
  > recognized in real-address mode.
- Compatibility Mode Exceptions
  - #UD - The CLUI instruction is not recognized in compatibility mode.

## Operation

```C
UIF := 0;
```

# STUI

Set User Interrupt Flag

STUI sets the user interrupt flag (UIF).
Its effect takes place immediately; a user interrupt may be delivered on the instruction boundary following STUI.
(This is in contrast with STI, whose effect is delayed by one instruction).An execution of STUI inside a transactional region causes a transactional abort; the abort loads EAX as it would have had it been due to an execution of STI.

## Flags affected

- None.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #UD - The STUI instruction is not recognized in virtual-8086 mode.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If executed inside an enclave.
- Protected Mode Exceptions
  - #UD - The STUI instruction is not recognized in protected mode.
- Compatibility Mode Exceptions
  - #UD - The STUI instruction is not recognized in compatibility mode.
- Real-Address Mode Exceptions
  - #UD - The STUI instruction is not recognized in real-address mode.

## Operation

```C
UIF := 1;
```

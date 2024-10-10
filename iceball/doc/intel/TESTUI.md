# TESTUI

Determine User Interrupt Flag



## Flags affected

- The ZF, OF, AF, PF, SF flags are cleared and the CF flags to the value of the user interrupt flag.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #UD - The TESTUI instruction is not recognized in virtual-8086 mode.
- Protected Mode Exceptions
  - #UD - The TESTUI instruction is not recognized in protected mode.
- Real-Address Mode Exceptions
  - #UD - The TESTUI instruction is not recognized in real-address mode.
- Compatibility Mode Exceptions
  - #UD - The TESTUI instruction is not recognized in compatibility mode.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If executed inside an enclave.

## Operation

```C
CF := UIF;ZF := AF := OF := PF := SF := 0;
```

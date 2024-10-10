# CLC

Clear Carry Flag

Clears the CF flag in the EFLAGS register.
Operation is the same in all modes.

## Flags affected

- The CF flag is set to 0. The OF, ZF, SF, AF, and PF flags are unaffected.

## Operation

```C
CF := 0;
```

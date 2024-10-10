# CMC

Complement Carry Flag

Complements the CF flag in the EFLAGS register.
CMC operation is the same in non-64-bit modes and 64-bit mode.

## Flags affected

- The CF flag contains the complement of its original value. The OF, ZF, SF, AF, and PF flags are unaffected.

## Operation

```C
EFLAGS.CF[bit 0] := NOT EFLAGS.CF[bit 0];
```

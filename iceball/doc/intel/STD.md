# STD

Set Direction Flag

Sets the DF flag in the EFLAGS register.
When the DF flag is set to 1, string operations decrement the index regis-ters (ESI and/or EDI).
Operation is the same in all modes.

## Flags affected

- The DF flag is set. The CF, OF, ZF, SF, AF, and PF flags are unaffected.

## Operation

```C
DF := 1;
```

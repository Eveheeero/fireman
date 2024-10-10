# ENDBR64

Terminate an Indirect Branch in 64-bit Mode

Terminate an indirect branch in 64 bit mode.

## Flags affected

- None.

## Operation

```C
IF EndbranchEnabled(CPL) & IA32_EFER.LMA = 1 & CS.L = 1IF CPL = 3THENIA32_U_CET.TRACKER = IDLEIA32_U_CET.SUPPRESS = 0ELSEIA32_S_CET.TRACKER = IDLEIA32_S_CET.SUPPRESS = 0FI;FI;
```

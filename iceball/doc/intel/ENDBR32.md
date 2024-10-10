# ENDBR32

Terminate an Indirect Branch in 32-bit and Compatibility Mode

Terminate an indirect branch in 32 bit and compatibility mode.

## Flags affected

- None.

## Operation

```C
IF EndbranchEnabled(CPL) & (IA32_EFER.LMA = 0 | (IA32_EFER.LMA=1 & CS.L = 0)IF CPL = 3THENIA32_U_CET.TRACKER = IDLEIA32_U_CET.SUPPRESS = 0ELSEIA32_S_CET.TRACKER = IDLEIA32_S_CET.SUPPRESS = 0FI;FI;
```

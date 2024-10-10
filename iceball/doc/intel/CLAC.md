# CLAC

Clear AC Flag in EFLAGS Register

Clears the AC flag bit in EFLAGS register.
This disables any alignment checking of user-mode data accesses.
If the SMAP bit is set in the CR4 register, this disallows explicit supervisor-mode data accesses to user-mode pages.This instruction's operation is the same in non-64-bit modes and 64-bit mode.
Attempts to execute CLAC when CPL> 0 cause #UD.

## Flags affected

- AC cleared. Other flags are unaffected.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #UD - The CLAC instruction is not recognized in virtual-8086 mode.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.(EAX=07H, ECX=
  > 0H):EBX.SMAP[bit 20] = 0.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If the CPL > 0.
  > If CPUID.(EAX=07H, ECX=
  > 0H):EBX.SMAP[bit 20] = 0.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If the CPL > 0.
- Compatibility Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If the CPL > 0.
  > If CPUID.(EAX=07H, ECX=
  > 0H):EBX.SMAP[bit 20] = 0.

## Operation

```C
EFLAGS.AC := 0;
```

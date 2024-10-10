# STAC

Set AC Flag in EFLAGS Register

Sets the AC flag bit in EFLAGS register.
This may enable alignment checking of user-mode data accesses.
This allows explicit supervisor-mode data accesses to user-mode pages even if the SMAP bit is set in the CR4 register.This instruction's operation is the same in non-64-bit modes and 64-bit mode.
Attempts to execute STAC when CPL> 0 cause #UD.

## Flags affected

- AC set. Other flags are unaffected.

## Exceptions

- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If the CPL > 0.
- Compatibility Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If the CPL > 0.
  > If CPUID.(EAX=07H, ECX=
  > 0H):EBX.SMAP[bit 20] = 0.
- Virtual-8086 Mode Exceptions
  - #UD - The STAC instruction is not recognized in virtual-8086 mode.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If the CPL > 0.
  > If CPUID.(EAX=07H, ECX=
  > 0H):EBX.SMAP[bit 20] = 0.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CPUID.(EAX=07H, ECX=
  > 0H):EBX.SMAP[bit 20] = 0.

## Operation

```C
EFLAGS.AC := 1;
```

# LAHF

Load Status Flags Into AH Register

This instruction executes as described above in compatibility mode and legacy mode.
It is valid in 64-bit mode only if CPUID.80000001H:ECX.LAHF-SAHF[bit 0] = 1.


## Flags affected

- None. The state of the flags in the EFLAGS register is not affected.

## Exceptions

- 64-Bit Mode Exceptions
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF 64-Bit ModeTHENIF CPUID.80000001H:ECX.LAHF-SAHF[bit 0] = 1;THEN AH := RFLAGS(SF:ZF:0:AF:0:PF:1:CF);ELSE #UD; FI;ELSEAH := EFLAGS(SF:ZF:0:AF:0:PF:1:CF);FI;
```

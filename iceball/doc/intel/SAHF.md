# SAHF

Store AH Into Flags

Loads the SF, ZF, AF, PF, and CF flags of the EFLAGS register with values from the corresponding bits in the AH register (bits 7, 6, 4, 2, and 0, respectively).
Bits 1, 3, and 5 of register AH are ignored; the corresponding reserved bits (1, 3, and 5) in the EFLAGS register remain as shown in the "Operation" section below.This instruction executes as described above in compatibility mode and legacy mode.
It is valid in 64-bit mode only if CPUID.80000001H:ECX.LAHF-SAHF[bit 0] = 1.

## Flags affected

- The SF, ZF, AF, PF, and CF flags are loaded with values from the AH register. Bits 1, 3, and 5 of the EFLAGS register are unaffected, with the values remaining 1, 0, and 0, respectively.

## Exceptions

- Protected Mode Exceptions
  > None.
- Virtual-8086 Mode Exceptions
  > None.
- Real-Address Mode Exceptions
  > None.
- 64-Bit Mode Exceptions

## Operation

```C
IF IA-64 ModeTHENIF CPUID.80000001H.ECX[0] = 1;THENRFLAGS(SF:ZF:0:AF:0:PF:1:CF) := AH;ELSE#UD;FIELSEEFLAGS(SF:ZF:0:AF:0:PF:1:CF) := AH;FI;
```

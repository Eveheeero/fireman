# FXAM

Examine Floating-Point

Examines the contents of the ST(0) register and sets the condition code flags C0, C2, and C3 in the FPU status word to indicate the class of value or number in the register (see the table below).Table 3-42.
 FXAM Results.ClassC3C2C0000Unsupported001NaN010Normal finite number011Infinity100Zero101Empty110Denormal numberThe C1 flag is set to the sign of the value in ST(0), regardless of whether the register is empty or full.This instruction's operation is the same in non-64-bit modes and 64-bit mode.

## Exceptions

- Protected Mode Exceptions
  - #NM - CR0.EM[bit 2] or CR0.TS[bit 3] = 1.
  - #MF - If there is a pending x87 FPU exception.
  - #UD - If the LOCK prefix is used.
- Floating-Point Exceptions
  > None.
- Virtual-8086 Mode Exceptions
  > Same exceptions as in protected mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
C1 := sign bit of ST; (* 0 for positive, 1 for negative *)CASE (class of value or number in ST(0)) OFUnsupported:C3, C2, C0 := 000;NaN:C3, C2, C0 := 001;Normal:C3, C2, C0 := 010;Infinity:C3, C2, C0 := 011;Zero:C3, C2, C0 := 100;Empty:C3, C2, C0 := 101;Denormal:C3, C2, C0 := 110;ESAC;FPU Flags AffectedC1Sign of value in ST(0).C0, C2, C3See Table 3-42.
```

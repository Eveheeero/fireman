# SIDT

Store Interrupt Descriptor Table Register

Stores the content the interrupt descriptor table register (IDTR) in the destination operand.
The destination operand specifies a 6-byte memory location.
In non-64-bit modes, the 16-bit limit field of the register is stored in the low 2 bytes of the memory location and the 32-bit base address is stored in the high 4 bytes.In 64-bit mode, the operand size fixed at 8+2 bytes.
The instruction stores 8-byte base and 2-byte limit values.SIDT is only useful in operating-system software; however, it can be used in application programs without causing an exception to be generated if CR4.UMIP= 0.
See "LGDT/LIDT-Load Global/Interrupt Descriptor Table Register" ® 64 and IA-32 Architectures Software Developer's Manual, Volume 2A, for information on in Chapter 3, Intelloading the GDTR and IDTR.IA-32 Architecture CompatibilityThe 16-bit form of SIDT is compatible with the Intel 286 processor if the upper 8 bits are not referenced.
The Intel 286 processor fills these bits with 1s; processor generations later than the Intel 286 processor fill these bits with 0s.

## Exceptions

- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #UD - If the LOCK prefix is used.
  - #GP(0) - If the memory address is in a non-canonical form.
  > If CR4.UMIP= 1 and CPL> 0.
  - #PF(fault-code) - If a page fault occurs.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If CR4.UMIP= 1.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #GP(0) - If the destination is located in a non-writable segment.
  > If a memory operand effective address is outs
  > ide the CS, DS, ES, FS, or GS segment limit.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  > If CR4.UMIP= 1 and CPL> 0.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an una
  > ligned memory reference is made while CPL= 3.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
IF instruction is SIDTTHENIF OperandSize =16 or OperandSize = 32 (* Legacy or Compatibility Mode *)THEN DEST[0:15] := IDTR(Limit);DEST[16:47] := IDTR(Base); FI; (* Full 32-bit base address stored *)ELSE (* 64-bit Mode *)DEST[0:15] := IDTR(Limit);DEST[16:79] := IDTR(Base); (* Full 64-bit base address stored *)FI;FI;
```

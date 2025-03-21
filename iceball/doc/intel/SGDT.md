# SGDT

Store Global Descriptor Table Register

Stores the content of the global descriptor table register (GDTR) in the destination operand.
The destination operand specifies a memory location.
In legacy or compatibility mode, the destination operand is a 6-byte memory location.
If the operand-size attribute is 16 or 32 bits, the 16-bit limit field of the register is stored in the low 2 bytes of the memory location and the 32-bit base address is stored in the high 4 bytes.In 64-bit mode, the operand size is fixed at 8+2 bytes.
The instruction stores an 8-byte base and a 2-byte limit.SGDT is useful only by operating-system software.
However, it can be used in application programs without causing an exception to be generated if CR4.UMIP= 0.
See "LGDT/LIDT-Load Global/Interrupt Descriptor Table Register" ® 64 and IA-32 Architectures Software Developer's Manual, Volume 2A, for information on in Chapter 3, Intelloading the GDTR and IDTR.IA-32 Architecture CompatibilityThe 16-bit form of the SGDT is compatible with the Intel 286 processor if the upper 8 bits are not referenced.
The Intel 286 processor fills these bits with 1s; processor generations later than the Intel 286 processor fill these bits with 0s.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #UD - If the LOCK prefix is used.
  - #GP(0) - If the memory address is in a non-canonical form.
  > If CR4.UMIP= 1 and CPL> 0.
  - #PF(fault-code) - If a page fault occurs.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
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
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
- Virtual-8086 Mode Exceptions
  - #UD - If the LOCK prefix is used.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If CR4.UMIP= 1.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.

## Operation

```C
IF instruction is SGDTIF OperandSize =16 or OperandSize = 32 (* Legacy or Compatibility Mode *)THEN DEST[0:15] := GDTR(Limit);DEST[16:47] := GDTR(Base); (* Full 32-bit base address stored *)FI;ELSE (* 64-bit Mode *)DEST[0:15] := GDTR(Limit);DEST[16:79] := GDTR(Base); (* Full 64-bit base address stored *)FI; FI;
```

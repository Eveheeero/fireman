# INS/INSB/INSW/INSD

Input from Port to String

Copies the data from the I/O port specified with the source operand (second operand) to the destination operand (first operand).
The source operand is an I/O port address (from 0 to 65,535) that is read from the DX register.
The destination operand is a memory location, the address of which is read from either the ES:DI, ES:EDI or the RDI registers (depending on the address-size attribute of the instruction, 16, 32 or 64, respectively).
(The ES segment cannot be overridden with a segment override prefix.) The size of the I/O port being accessed (that is, the size of the source and destination operands) is determined by the opcode for an 8-bit I/O port or by the operand-size attri-bute of the instruction for a 16- or 32-bit I/O port.At the assembly-code level, two forms of this instruction are allowed: the "explicit-operands" form and the "no-operands" form.
The explicit-operands form (specified with the INS mnemonic) allows the source and destination operands to be specified explicitly.
Here, the source operand must be "DX," and the destination operand should be a symbol that indicates the size of the I/O port and the destination address.
This explicit-operands form is provided to allow documentation; however, note that the documentation provided by this form can be misleading.
That is, the destination operand symbol must specify the correct type (size) of the operand (byte, word, or doubleword), but it does not have to specify the correct location.
The location is always specified by the ES:(E)DI registers, which must be loaded correctly before the INS instruction is executed.The no-operands form provides "short forms" of the byte, word, and doubleword versions of the INS instructions.
Here also DX is assumed by the processor to be the source operand and ES:(E)DI is assumed to be the destination operand.
The size of the I/O port is specified with the choice of mnemonic: INSB (byte), INSW (word), or INSD (doubleword).After the byte, word, or doubleword is transfer from the I/O port to the memory location, the DI/EDI/RDI register is incremented or decremented automatically according to the setting of the DF flag in the EFLAGS register.
(If the DF flag is 0, the (E)DI register is incremented; if the DF flag is 1, the (E)DI register is decremented.) The (E)DI register is incremented or decremented by 1 for byte operThe INS, INSB, INSW, and INSD instructions can be preceded by the REP prefix for block input of ECX bytes, words, ®or doublewords.
See "REP/REPE/REPZ /REPNE/REPNZ-Repeat String Operation Prefix" in Chapter 4 of the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 2B, for a description of the REP prefix.These instructions are only useful for accessing I/O ports located in the processor's I/O address space.
See Chapter ®19, "Input/Output," in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for more information on accessing I/O ports in the I/O address space.In 64-bit mode, default address size is 64 bits, 32 bit address size is supported using the prefix 67H.
The address of the memory destination is specified by RDI or EDI.
16-bit address size is not supported in 64-bit mode.
The operand size is not promoted.These instructions may read from the I/O port without writing to the memory location if an exception or VM exit occurs due to the write (e.g.
#PF).
If this would be problematic, for example because the I/O port read has side-effects, software should ensure the write to the memory location does not cause an exception or VM exit.

## Flags affected

- None.

## Exceptions

- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If any of the I/O permission bits in
  > the TSS for the I/O port being accessed is 1.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled an
  > d an unaligned memory reference is made.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #GP(0) - If the CPL is greater than (has less privileg
  > e) the I/O privilege level (IOPL) and any of the 
  > corresponding I/O permission bits in TS
  > S for the I/O port being accessed is 1.
  > If the memory address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Real-Address Mode Exceptions
  - #GP - If a memory operand effective address is ou
  > tside the CS, DS, ES, FS, or GS segment limit.
  - #SS - If a memory operand effective address is outside the SS segment limit.
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #GP(0) - If the CPL is greater than (has less privileg
  > e) the I/O privilege level (IOPL) and any of the 
  > corresponding I/O permission bits in TS
  > S for the I/O port being accessed is 1.
  > If the destination is located in a non-writable segment.
  > If an illegal memory operand effective 
  > address in the ES segments is given.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3.
  - #UD - If the LOCK prefix is used.

## Operation

```C
= =IF ((PE  1) and ((CPL > IOPL) or (VM 1)))THEN (* Protected mode with CPL > IOPL or virtual-8086 mode *)= IF (Any I/O Permission Bit for I/O port being accessed 1)THEN (* I/O operation is not allowed *)#GP(0);ELSE (* I/O operation is allowed *) DEST := SRC; (* Read from I/O port *)FI;ELSE (Real Mode or Protected Mode with CPL IOPL *)DEST := SRC; (* Read from I/O port *)FI;Non-64-bit Mode:IF (Byte transfer)=THEN IF DF  0THEN (E)DI := (E)DI + 1; ELSE (E)DI := (E)DI - 1; FI;ELSE IF (Word transfer) = THEN IF DF0THEN (E)DI := (E)DI + 2; ELSE (E)DI := (E)DI - 2; FI;ELSE (* Doubleword transfer *)=THEN IF DF  0THEN (E)DI := (E)DI + 4; ELSE (E)DI := (E)DI - 4; FI;FI;FI;FI64-bit Mode:IF (Byte transfer)=THEN IF DF  0THEN (E|R)DI := (E|R)DI + 1; ELSE (E|R)DI := (E|R)DI - 1; FI;ELSE IF (Word transfer) = THEN IF DF0THEN (E)DI := (E)DI + 2; ELSE (E)DI := (E)DI - 2; FI;ELSE (* Doubleword transfer *)=THEN IF DF  0THEN (E|R)DI := (E|R)DI ELSE (E|R)DI := (E|R)DI - 4; FI;FI;FI;
```

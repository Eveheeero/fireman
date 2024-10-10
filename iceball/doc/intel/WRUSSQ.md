# WRUSSD/WRUSSQ

Write to User Shadow Stack

Writes bytes in register source to a user shadow stack page.
The WRUSS instruction can be executed only if CPL = 0, however the processor treats its shadow-stack accesses as user accesses.

## Flags affected

- None.C/C++ Compiler Intrinsic Equivalent

## Exceptions

- Virtual-8086 Mode Exceptions
  - #UD - The WRUSS instruction is not recognized in virtual-8086 mode.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  - #GP(0) - If a memory address is in a non-canonical form.
  > If linear address of destinat
  > ion is not 4 byte aligned.
  > If CPL is not 0.
- Real-Address Mode Exceptions
  - #UD - The WRUSS instruction is not recognized in real-address mode.
- Compatibility Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  - #GP(0) - If a memory address is in a non-canonical form.
  > If linear address of destinat
  > ion is not 4 byte aligned.
  > If CPL is not 0.
  - #SS(0) - If a memory address referencing the SS segment is in a non-canonical form.
  - #PF(fault-code) - If destination is not a user shadow stack.
  > Other terminal and non-terminal faults.
- Protected Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If CR4.CET = 0.
  - #GP(0) - If a memory operand effective address is
  > outside the CS, DS, ES, FS, or GS segment limit.
  > If destination is located in a non-writeable segment.
  > If the DS, ES, FS, or GS register is used to access memory and it contains a NULL segment 
  > selector.
  > If linear address of destinat
  > ion is not 4 byte aligned.
  > If CPL is not 0.
  - #SS(0) - If a memory operand effective ad
  > dress is outside the SS segment limit.
  - #PF(fault-code) - If destination is not a user shadow stack.
  > Other terminal and non-terminal faults.

## Operation

```C
IF CR4.CET = 0THEN #UD; FI;IF CPL > 0THEN #GP(0); FI;DEST_LA = Linear_Address(mem operand)IF (operand size is 64 bit)THEN(* Destination not 8B aligned *)IF DEST_LA[2:0]THEN GP(0); FI;Shadow_stack_store 8 bytes of SRC to DEST_LA as user-mode access;ELSE(* Destination not 4B aligned *)IF DEST_LA[1:0]THEN GP(0); FI;Shadow_stack_store 4 bytes of SRC[31:0] to DEST_LA as user-mode access;FI;
```

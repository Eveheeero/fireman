# POPF/POPFD/POPFQ

Pop Stack Into EFLAGS Register

Pops a doubleword (POPFD) from the top of the stack (if the current operand-size attribute is 32) and stores the value in the EFLAGS register, or pops a word from the top of the stack (if the operand-size attribute is 16) and stores it in the lower 16 bits of the EFLAGS register (that is, the FLAGS register).
These instructions reverse the operation of the PUSHF/PUSHFD/PUSHFQ instructions.
The POPF (pop flags) and POPFD (pop flags double) mnemonics reference the same opcode.
The POPF instruction is intended for use when the operand-size attribute is 16; the POPFD instruction is intended for use when the operand-size attribute is 32.
Some assemblers may force the operand size to 16 for POPF and to 32 for POPFD.
Others may treat the mnemonics as synonyms (POPF/POPFD) and use the setting of the operand-size attribute to determine the size of values to pop from the stack.The effect of POPF/POPFD on the EFLAGS register changes, depending on the mode of operation.
See Table 4-16 and the key below for details.When operating in protected, compatibility, or 64-bit mode at privilege level 0 (or in real-address mode, the equiv-1, VIP, VIF, and VM may be modi-alent to privilege level 0), all non-reserved flags in the EFLAGS register except RFfied.
VIP, VIF, and VM remain unaffected.When operating in protected, compatibility, or 64-bit mode with a privilege level greater than 0, but less than or equal to IOPL, all flags can be modified except the IOPL field and RF, IF, VIP, VIF, and VM; these remain unaffected.
The AC and ID flags can only be modified if the operand-size attribute is 32.
The interrupt flag (IF) is altered only when executing at a level at least as privileged as the IOPL.
If a POPF/POPFD instruction is executed with insuffi-cient privilege, an exception does not occur but privileged bits do not change.When operating in virtual-8086 mode (EFLAGS.VM = 1) without the virtual-8086 mode extensions (CR4.VME = 0), the POPF/POPFD instructions can be used only if IOPL = 3; otherwise, a general-protection exception (#GP) occurs.
If the virtual-8086 mode extensions are enabled (CR4.VME = 1), POPF (but not POPFD) can be executed in virtual-8086 mode with IOPL < 3.(The protected-mode virtual-interrupt feature - enabled by setting CR4.PVI - affects the CLI and STI instructions in the same manner as the virtual-8086 mode extensions.
POPF, however, is not affected by CR4.PVI.)In 64-bit mode, the mnemonic assigned is POPFQ (note that the 32-bit operand is not encodable).
POPFQ pops 64 bits from the stack.
Reserved bits of RFLAGS (including the upper 32 bits of RFLAGS) are not affected.® 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for more informa-See Chapter 3 of the Inteltion about the EFLAGS registers.

## Flags affected

- All flags may be affected; see the Operation section for details.

## Exceptions

- Real-Address Mode Exceptions
  - #SS - If the top of stack is not within the stack segment.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same as for protected mode exceptions.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If IOPL < 3 and VME is not enabled.
  > If IOPL < 3 and the 32-bit operand size is used.
  > If IOPL < 3, EFLAGS.VIP= 1, and bit 9 (IF)
  >  is set in the FLAGS value on the stack.
  > If IOPL < 3 and bit 8 (TF) is set in the FLAGS value on the stack.
  > If an attempt is made to execute the POPF/POPFD instruction with an operand-size override 
  > prefix.
  - #SS(0) - If the top of stack is not within the stack segment.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is
  > made while alignment checking is enabled.
  - #UD - If the LOCK prefix is used.
- 64-Bit Mode Exceptions
  - #SS(0) - If the stack address is in a non-canonical form.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
- Protected Mode Exceptions
  - #SS(0) - If the top of stack is not within the stack segment.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If an unaligned memory reference is made
  > while CPL = 3 and alignment checking is enabled.
  - #UD - If the LOCK prefix is used.

## Operation

```C
IF EFLAGS.VM = 0 (* Not in Virtual-8086 Mode *)= THEN IF CPL 0 OR CR0.PE = 0THENIF OperandSize = 32;THEN EFLAGS := Pop(); (* 32-bit pop *)(* All non-reserved flags except RF, VIP, VIF, and VM can be modified; VIP, VIF, VM, and all reserved bits are unaffected. RF is cleared. *)ELSE IF (Operandsize = 64)RFLAGS = Pop(); (* 64-bit pop *)(* All non-reserved flags except RF, ELSE (* OperandSize = 16 *)EFLAGS[15:0] := Pop(); (* 16-bit pop *)(* All non-reserved flags can be modified. *)FI;ELSE (* CPL > 0 *) 32IF OperandSize =THEN IF CPL > IOPLTHENEFLAGS := Pop(); (* 32-bit pop *)(* All non-reserved bits except IF, IOPL, VIP, VIF, VM, and RF can be modified; IF, IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)ELSEEFLAGS := Pop(); (* 32-bit pop *)(* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified; IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)FI;ELSE IF (Operandsize = 64)IF CPL > IOPLTHENRFLAGS := Pop(); (* 64-bit pop *)(* All non-reserved bits except IF, IOPL, VIP, VIF, VM, and RF can be modified; IF, IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)ELSERFLAGS := Pop(); (* 64-bit pop *)(* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified; IOPL, VIP, VIF, VM, and all reserved bits are unaffected; RF is cleared. *)FI; 16 *)ELSE (* OperandSize =EFLAGS[15:0] := Pop(); (* 16-bit pop *)(* All non-reserved bits except IOPL can be modified; IOPL and allreserved bits are unaffected. *)FI;FI;ELSE (* In virtual-8086 mode *)IF IOPL = 3THEN= 32 IF OperandSize THEN EFLAGS := Pop();(* All non-reserved bits except IOPL, VIP, VIF, VM, and RF can be modified; VIP, VIF, VM, IOPL, and all reserved bits are unaffected. RF is cleared. *)ELSE EFLAGS[15:0] := Pop(); FI;(* All non-reserved bits except IOPL can be modified; IOPL and all reserved bits are unaffected. *)FI;ELSE (* IOPL < 3 *)IF (Operandsize = 32) OR (CR4.VME = 0)THEN #GP(0); (* Trap to virtual-8086 monitor. *)ELSE (* Operandsize = 16 and CR4.VME = 1 *)tempFLAGS := Pop();IF (EFLAGS.VIP = 1 AND tempFLAGS[9] = 1) OR tempFLAGS[8] = 1                  EFLAGS.VIF := tempFLAGS[9];                  EFLAGS[15:0] := tempFLAGS;                  (* All non-reserved bits except IOPL and IF can be modified;IOPL, IF, and all reserved bits are unaffected. *)FI;FI;FI;FI;
```

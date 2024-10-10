# MOV

Move

Copies the second operand (source operand) to the first operand (destination operand).
The source operand can be an immediate value, general-purpose register, segment register, or memory location; the destination register can be a general-purpose register, segment register, or memory location.
Both operands must be the same size, which can be a byte, a word, a doubleword, or a quadword.The MOV instruction cannot be used to load the CS register.
Attempting to do so results in an invalid opcode excep-tion (#UD).
To load the CS register, use the far JMP, CALL, or RET instruction.If the destination operand is a segment register (DS, ES, FS, GS, or SS), the source operand must be a valid segment selector.
In protected mode, moving a segment selector into a segment register automatically causes the segment descriptor information associated with that segment selector to be loaded into the hidden (shadow) part of the segment register.
While loading this information, the segment selector and segment descriptor information is validated (see the "Operation" algorithm below).
The segment descriptor data is obtained from the GDT or LDT entry for the specified segment selector.
A NULL segment selector (values 0000-0003) can be loaded into the DS, ES, FS, and GS registers without causing a protection exception.
However, any subsequent attempt to reference a segment whose corresponding segment register is loaded with a NULL value causes a general protection exception (#GP) and no memory reference occurs.Loading the SS register with a MOV instruction suppresses or inhibits some debug exceptions and inhibits inter-rupts on the following instruction boundary.
(The inhibition ends after delivery of an exception or the execution of the next instruction.) This behavior allows a stack pointer to be loaded into the ESP register with the next instruc-tion (MOV ESP, stack-pointer valueMoves the contents of a control register (CR0, CR2, CR3, CR4, or CR8) to a general-purpose register or the contents of a general-purpose register to a control register.
The operand size for these instructions is always 32 bits in non-64-bit modes, regardless of the operand-size attribute.
On a 64-bit capable processor, an execution of MOV to CR outside of 64-bit mode zeros the upper 32 bits of the control register.
(See "Control Registers" in Chapter 2 ® 64 and IA-32 Architectures Software Developer's Manual, Volume 3A, for a detailed description of the of the Intelflags and fields in the control registers.) This instruction can be executed only when the current privilege level is 0.At the opcode level, the reg field within the ModR/M byte specifies which of the control registers is loaded or read.
The 2 bits in the mod field are ignored.
The r/m field specifies the general-purpose register loaded or read.
Some of the bits in CR0, CR3, and CR4 are reserved and must be written with zeros.
Attempting to set any reserved bits in CR0[31:0] is ignored.
Attempting to set any reserved bits in CR0[63:32] results in a general-protection excep-tion, #GP(0).
When PCIDs are not enabled, bits 2:0 and bits 11:5 of CR3 are not used and attempts to set them are ignored.
Attempting to set any reserved bits in CR3[63:MAXPHYADDR] results in #GP(0).
Attempting to set any reserved bits in CR4 results in #GP(0).
On Pentium 4, Intel Xeon and P6 family processors, CR0.ET remains set after any load of CR0; attempts to clear this bit have no impact.In certain cases, these instructions have the side effect of invalidating entries in the TLBs and the paging-structure ® 64 and caches.
See Section 4.10.4.1, "Operations that Invalidate TLBs and Paging-Structure Caches," in the IntelIA-32 Architectures Software Developer's Manual, Volume 3A, for details.The following side effects are implementation-specific for the Pentium 4, Intel Xeon, and P6 processor family: when modifying PE or PG in register CR0, or PSE or PAE in register CR4, all TLB entries are flushed, including global entries.
Software should not depend on this functionality in all Intel64 or IA-32 processors.In 64-bit mode, the instruction's default operation size is 64 bits.
The REX.R prefix must be used to access CR8.
of the REX.R prefix to specify a register other than CR8 causes an invalid-opcode exception.
See the summary chart at the beginning of this section for encoding data and limits.If CR4.PCIDE= 1, bit63 of the source operand to MOV to CR3 determines whether the instruction invalidates entries in the TLBs and the paging-structure caches (see Section 4.10.4.1, "Operations that Invalidate TLBs and ® Paging-Structure Caches," in the Intel64 and IA-32 Architectures Software Developer's Manual, Volume 3A).
The instruction does not modify bit63 of CR3, which is reserved and always 0.®See "Changes to Instruction Behavior in VMX Non-Root Operation" in Chapter 26 of the Intel 64 and IA-32 Archi-tectures Software Developer's Manual, Volume 3C, for more information about the behavior of this instruction in VMX non-root operation.Moves the contents of a debug register (DR0, DR1, DR2, DR3, DR4, DR5, DR6, or DR7) to a general-purpose register or vice versa.
The operand size for these instructions is always 32 bits in non-64-bit modes, regardless of ® 64 and IA-32 Architectures Soft-the operand-size attribute.
(See Section 18.2, "Debug Registers", of the Intelware Developer's Manual, Volume 3A, for a detailed description of the flags and fields in the debug registers.)The instructions must be executed at privilege level 0 or in real-address mode.When the debug extension (DE) flag in register CR4 is clear, these instructions operate on debug registers in a manner that is compatible with Intel386 and Intel486 processors.
In this mode, references to DR4 and DR5 refer to DR6 and DR7, respectively.
When the DE flag in CR4 is set, attempts to reference DR4 and DR5 result in an undefined opcode (#UD) exception.
(The CR4 register was added to the IA-32 Architecture beginning with the Pentium processor.)At the opcode level, the reg field within the ModR/M byte specifies which of the debug registers is loaded or read.
The two bits in the mod field are ignored.
The r/m field specifies the general-purpose register loaded or read.In 64-bit mode, the instruction's default operation size is 64 bits.
Use of the REX.B prefix permits access to addi-tional registers (R8-R15).
Use of the REX.W or 66H prefix is ignored.
Use of the REX.R prefix causes an invalid-opcode exception.
See the summary chart at the beginning of this section for encoding data and limits.


## Flags affected

- None.The OF, SF, ZF, AF, PF, and CF flags are undefined.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  - #UD - If CR4.DE[bit 3] = 1 (debug extensions) and a MOV instruction is executed involving DR4 or
  > DR5.
  > If the LOCK prefix is used.
  - #DB - If any debug register is accessed while the DR7.GD[bit 13] = 1.
- Virtual-8086 Mode Exceptions
  - #GP(0) - The debug registers cannot be loaded or read when in virtual-8086 mode.
- ) before an event can be delivered. See Section 6.8.3, "Masking Exceptions
  > ® 
  > 64 and IA-32 Architectures Software Developer's Manual, 
  > and Interrupts When Switching Stacks," in the Intel
  > Volume 3A. Intel recommends that software use the LSS in
  > struction to load the SS register and ESP together.
  > When executing MOV Reg, Sreg, the processor copies the co
  > ntent of Sreg to the 16 least significant bits of the 
  > general-purpose register. The upper bits of the destinatio
  > n register are zero for most IA-32 processors (Pentium 
  > Pro processors and later) and all Intel 64 processors, with
  >  the exception that bits 31:16 are undefined for Intel 
  > Quark X1000 processors, Pentium, and earlier processors.
  > In 64-bit mode, the instruction's default operation size is
  >  32 bits. Use of the REX.R prefix permits access to addi-
  > tional registers (R8-R15). Use of the REX.W prefix promotes operation to 64 bits. See the summary chart at the 
  > beginning of this section for encoding data and limits.
- Real-Address Mode Exceptions
  - #UD - If CR4.DE[bit 3] = 1 (debug extensions) and a MOV instruction is executed involving DR4 or
  > DR5. 
  > If the LOCK prefix is used.
  - #DB - If any debug register is accessed while the DR7.GD[bit 13] = 1.
- 64-Bit Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  > If an attempt is made to write a 1 to any of bits 63:32 in DR6.
  > If an attempt is made to write a 1 to any of bits 63:32 in DR7.
  - #UD - If CR4.DE[bit 3] = 1 (debug extensions) and a MOV instruction is executed involving DR4 or
  > DR5.
  > If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
DEST := SRC;Loading a segment register while in protected mode results in special checks and actions, as described in the following listing. These checks are performed on the segment selector anTHENIF segment selector is NULLTHEN #GP(0); FI;IF segment selector index is outside descriptor table limits OR segment selector's RPL  CPLOR segment is not a writable data segmentOR DPL  CPLTHEN #GP(selector); FI;IF segment not marked present THEN #SS(selector); ELSESS := segment selector;SS := segment descriptor; FI;FI;IF DS, ES, FS, or GS is loaded with non-NULL selectorTHENIF segment selector index is outside descriptor table limitsOR segment is not a data or readable code segmentOR ((segment is a data or nonconforming code segment) AND ((RPL > DPL) or (CPL > DPL)))THEN #GP(selector); FI;IF segment not marked presentTHEN #NP(selector);ELSESegmentRegister := segment selector;SegmentRegister := segment descriptor; FI;FI;IF DS, ES, FS, or GS is loaded with NULL selectorTHENSegmentRegister := segment selector;SegmentRegister := segment descriptor;FI;DEST := SRC;== IF ((DE  1) and (SRC or DEST DR4 or DR5))THEN#UD;ELSE DEST := SRC;FI;
```

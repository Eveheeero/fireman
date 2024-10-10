# STI

Set Interrupt Flag

In most cases, STI sets the interrupt flag (IF) in the EFLAGS register.
This allows the processor to respond to maskable hardware interrupts.If IF= 0, maskable hardware interrupts remain inhibited on the instruction boundary following an execution of STI.
(The delayed effect of this instruction is provided to allow interrupts to be enabled just before returning from a procedure or subroutine.
For instance, if an STI instruction is followed by an RET instruction, the RET instruction is allowed to execute before external interrupts are recognized.
No interrupts can be recognized if an execution of CLI immediately follow such an execution of STI.) The inhibition ends after delivery of another event (e.g., exception) or the execution of the next instruction.The IF flag and the STI and CLI instructions do not prohibit the generation of exceptions and nonmaskable inter-rupts (NMIs).
However, NMIs (and system-management interrupts) may be inhibited on the instruction boundary following an execution of STI that begins with IF= 0.Operation is different in two modes defined as follows: - PVI mode (protected-mode virtual interrupts): CR0.PE= 1, EFLAGS.VM= 0, CPL= 3, and CR4.PVI= 1; - VME mode (virtual-8086 mode extensions): CR0.PE= 1, EFLAGS.VM= 1, and CR4.VME= 1.If IOPL< 3, EFLAGS.VIP= 1, and either VME mode or PVI mode is active, STI sets the VIF flag in the EFLAGS register, leaving IF unaffected.Table 4-19 indicates the action of the STI instruction depending on the processor operating mode, IOPL, CPL, and EFLAGS.VIP.Table 4-19.
 Decision Table for STI Results ModeIOPLEFLAGS.VIPSTI Result1  1XIFReal-addressX=CPLXIF 1 = 2Protected, not PVICPLX#GP fault< 3XIF  1=3  Protected, PVI10VIF=0-21#GP fault3XIF  1=3Virtual-8086, not VME0-2X#GP fault3XIF  1=3  Virtual-8086, VME10VIF=0-21#GP faultNOTES:2.
For this table, "protected mode" applies whenever CR0.PE= 1 and EFLAGS.VM= 0; it includes compatibility mode and 64-bit mode.3.
PVI mode and virtual-8086 mode each imply CPL= 3.

## Flags affected

- Either the IF flag or the VIF flag is set to 1. Other flags are unaffected.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If CPL is greater than IOPL and PVI mode is not active.
  > If CPL is greater than IOPL and EFLAGS.VIP = 1.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If IOPL is less than 3 and VME mode is not active.
  > If IOPL is less than 3 and EFLAGS.VIP = 1.
  - #UD - If the LOCK prefix is used.

## Operation

```C
IF CR0.PE = 0  (* Executing in real-address mode *)THEN IF := 1; (* Set Interrupt Flag *)ELSEIF IOPL  CPL(* CPL = 3 if EFLAGS.VM = 1 *)THEN IF := 1; (* Set Interrupt Flag *)ELSEIF VME mode OR PVI modeTHENIF EFLAGS.VIP = 0THEN VIF := 1; (* Set Virtual Interrupt Flag *)ELSE #GP(0); FI;ELSE #GP(0); FI;FI;FI;
```

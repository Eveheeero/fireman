# SYSRET

Return From Fast System Call

SYSRET is a companion instruction to the SYSCALL instruction.
It returns from an OS system-call handler to user 1 With a 64-bit operand code at privilege level 3.
It does so by loading RIP from RCX and loading RFLAGS from R11.size, SYSRET remains in 64-bit mode; otherwise, it enters compatibility mode and only the low 32 bits of the regis-ters are loaded.SYSRET loads the CS and SS selectors with values derived from bits63:48 of the IA32_STAR MSR.
However, the CS and SS descriptor caches are not loaded from the descriptors (in GDT or LDT) referenced by those selectors.
Instead, the descriptor caches are loaded with fixed values.
See the Operation section for details.
It is the respon-sibility of OS software to ensure that the descriptors (in GDT or LDT) referenced by those selector values corre-spond to the fixed values loaded into the descriptor caches; the SYSRET instruction does not ensure this correspondence.The SYSRET instruction does not modify the stack pointer (ESP or RSP).
For that reason, it is necessary for software to switch to the user stack.
The OS may load the user stack pointer (if it was saved after SYSCALL) before executing SYSRET; alternatively, user code may load the stack pointer (if it was saved before SYSCALL) after receiving control from SYSRET.If the OS loads the stack pointer before executing SYSRET, it must ensure that the handler of any interrupt or exception delivered between restoring the stack pointer and successful execution of SYSRET is not invoked with the user stack.
It can do so using approaches such as the following: - External interrupts.
The OS can prevent an external interrupt from being delivered by clearing EFLAGS.IF before loading the user stack pointer.
- Nonmaskable interrupts (NMIs).
The OS can ensure that the NMI handler is invoked with the correct stack by using the interrupt stack table (IST) mechanism for gate 2 (NMI) in the IDT (see Section 6.14.5, "Interrupt ® 64 and IA-32 Architectures Software Developer's Manual, Volume 3A).Stack Table," in Intel - General-protection exceptions (#GP).
The SYSRET instruction generates #GP(0) if the value of RCX is not canonical.
The OS can address this possibility using one or more of the following approaches:-Confirming that the value of RCX is canonical before executing SYSRET.-Using paging to ensure that the SYSCALL instruction will never save a non-canonical value into RCX.-Using the IST mechanism for gate 13 (#GP) in the IDT.When shadow stacks are enabled at privilege level 3 the instruction loads SSP with value from IA32_PL3_SSP MSR.
Refer to Chapter 6, "Procedure Calls, Interrupts, and Exceptions" and Chapter 17, "Control-flow Enforcement ®Technology (CET)" in the Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, for additional CET details.

## Flags affected

- All.

## Exceptions

- Protected Mode Exceptions
  - #UD - The SYSRET instruction is not recognized in protected mode.
- Compatibility Mode Exceptions
  - #UD - The SYSRET instruction is not recognized in compatibility mode.
- Real-Address Mode Exceptions
  - #UD - The SYSRET instruction is not recognized in real-address mode.
- 64-Bit Mode Exceptions
  - #UD - If IA32_EFER.SCE = 0.
  > If the LOCK prefix is used.
  > 
  >  
  - #GP(0) - If CPL
- Virtual-8086 Mode Exceptions
  - #UD - The SYSRET instruction is not recognized in virtual-8086 mode.

## Operation

```C
 IF (CS.L 1 ) or (IA32_EFER.LMA  1) or (IA32_EFER.SCE  1)(* Not in 64-Bit Mode or SYSCALL/SYSRET not enabled in IA32_EFER *)THEN #UD; FI; IF (CPL 0) THEN #GP(0); FI;IF (operand size is 64-bit) THEN (* Return to 64-Bit Mode *)IF (RCX is not canonical) THEN #GP(0);RIP := RCX;ELSE (* Return to Compatibility Mode *)RIP := ECX;FI;RFLAGS := (R11 & 3C7FD7H) | 2;(* Clear RF, VM, reserved bits; set bit 1 *)IF (operand size is 64-bit) THEN CS.Selector := IA32_STAR[63:48]+16;ELSE CS.Selector := IA32_STAR[63:48];FI;CS.Selector := CS.Selector OR 3;(* RPL forced to 3 *)(* Set rest of CS to a fixed value *)CS.Base := 0;(* Flat segment *)CS.Limit := FFFFFH;(* With 4-KByte granularity, implies a 4-GByte limit *)CS.Type := 11;(* Execute/read code, accessed *)CS.S := 1;CS.DPL := 3;CS.P := 1;IF (operand size is 64-bit) THEN (* Return to 64-Bit Mode *)CS.L := 1;(* 64-bit code segment *)CS.D := 0;(* Required if CS.L = 1 *)ELSE (* Return to Compatibility Mode *)CS.L := 0;(* Compatibility mode *)CS.D := 1;(* 32-bit code segment *)FI;CS.G := 1;(* 4-KByte granularity *)CPL := 3;IF ShadowStackEnabled(CPL)SSP := IA32_PL3_SSP;FI;SS.Selector := (IA32_STAR[63:48]+8) OR 3;(* RPL forced to 3 *)(* Set rest of SS to a fixed value *)SS.Base := 0;(* Flat segment *)SS.Limit := FFFFFH;(* With 4-KByte granularity, implies a 4-GByte limit *)SS.Type := 3;(* Read/write data, accessed *)SS.S := 1;SS.DPL := 3;SS.P := 1;
```

# RDMSR

Read From Model Specific Register

Reads the contents of a 64-bit model specific register (MSR) specified in the ECX register into registers EDX:EAX.
(On processors that support the Intel64 architecture, the high-order 32 bits of RCX are ignored.) The EDX register is loaded with the high-order 32 bits of the MSR and the EAX register is loaded with the low-order 32 bits.
(On processors that support the Intel64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.) If fewer than 64 bits are implemented in the MSR being read, the values returned to EDX:EAX in unimplemented bit locations are undefined.This instruction must be executed at privilege level 0 or in real-address mode; otherwise, a general protection exception #GP(0) will be generated.
Specifying a reserved or unimplemented MSR address in ECX will also cause a general protection exception.The MSRs control functions for testability, execution tracing, performance-monitoring, and machine check errors.
® 64 and IA-32 Architectures Software Developer's Chapter 2, "Model-Specific Registers (MSRs)" of the IntelManual, Volume 4, lists all the MSRs that can be read with this instruction and their addresses.
Note that each processor family has its own set of MSRs.The CPUID instruction should be used to determine whether MSRs are supported (CPUID.01H:EDX[5] = 1) before using this instruction.IA-32 Architecture CompatibilityThe MSRs and the ability to read them with the RDMSR instruction were introduced into the IA-32 Architecture with the Pentium processor.
Execution of this instruction by an IA-32 processor earlier than the Pentium processor results in an invalid opcode exception #UD.® 64 and IA-32 Archi-See "Changes to Instruction Behavior in VMX Non-Root Operation" in Chapter 26 of the Inteltectures Software Developer's Manual, Volume 3C, for more information about the behavior of this instruction in VMX non-root operation.

## Flags affected

- None.

## Exceptions

- Virtual-8086 Mode Exceptions
  - #GP(0) - The RDMSR instruction is not recognized in virtual-8086 mode.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.
- Real-Address Mode Exceptions
  - #GP - If the value in ECX specifies a reserved or unimplemented MSR address.
  - #UD - If the LOCK prefix is used.
- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0.
  > If the value in ECX specifies a rese

## Operation

```C
EDX:EAX := MSR[ECX];
```

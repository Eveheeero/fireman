# RDPMC

Read Performance-Monitoring Counters

Reads the contents of the performance monitoring counter (PMC) specified in ECX register into registers EDX:EAX.
(On processors that support the Intel 64 architecture, the high-order 32 bits of RCX are ignored.) The EDX register is loaded with the high-order 32 bits of the PMC and the EAX register is loaded with the low-order 32 bits.
(On processors that support the Intel 64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.) If fewer than 64 bits are implemented in the PMC being read, unimplemented bits returned to EDX:EAX will have value zero.
 0) are The width of PMCs on processors supporting architectural performance monitoring (CPUID.0AH:EAX[7:0] reported by CPUID.0AH:EAX[23:16].
On processors that do not support architectural performance monitoring (CPUID.0AH:EAX[7:0]=0), the width of general-purpose performance PMCs is 40 bits, while the widths of special-purpose PMCs are implementation specific.Use of ECX to specify a PMC depends on whether the processor supports architectural performance monitoring: - If the processor does not support architectural performance monitoring (CPUID.0AH:EAX[7:0]=0), ECX[30:0] specifies the index of the PMC to be read.
Setting ECX[31] selects "fast" read mode if supported.
In this mode, RDPMC returns bits 31:0 of the PMC in EAX while clearing EDX to zero.
-  If the processor does support architectural performance monitoring (CPUID.0AH:EAX[7:0] 0), ECX[31:16] specifies type of PMC while ECX[15:0] specifies the index of the PMC to be read within that type.
The following PMC types are currently defined:-General-purpose counters use type 0.
The index x (to read IA32_PMCx) must be less than the value enumerated by CPUID.0AH.EAX[15:8] (thus ECX[15:8] must be zero).-Fixed-function counters use type 4000H.
The index x (to read IA32_FIXED_CTRx) can be used if either CPUID.0AH.EDX[4:0] > x or CPUID.0AH.ECX[x] = 1 (thus ECX[15:5] must be 0).-Performance metrics use type 2000H.
This type can be used only if IA32_PERF_CAPABILITIES.PERF_MET-RICS_AVAILABLE[bit 15]=1.
For this type, the index in ECX[15:0] is implementation specific.Specifying an unsupported PMC encoding will cause a general protection exception #GP(0).
For PMC details see ® 64 and IA-32 Architectures Software Developer's Manual, Chapter 20, "Performance Monitoring," in the IntelVolume 3B.When in protected or virtual 8086 mode, the Performance-monitoring Counters Enabled (PCE) flag in register CR4 restricts the use of the RDPMC instruction.
When the PCE flag is set, the RDPMC instruction can be executed at any privilege level; when the flag is clear, the instruction can only be executed at privilege level 0.
(When in real-address mode, the RDPMC instruction is always enabled.) The PMCs can also be read with the RDMSR instruction, when executing at privilege level 0.The RDPMC instruction is not a serializing instruction; that is, it does not imply that all the events caused by the preceding instructions have been completed or that events caused by subsequent instructions have not begun.
If an exact event count is desired, software must insert a serializing instruction (such as the CPUID instruction) before and/or after the RDPMC instruction.Performing back-to-back fast reads are not guaranteed to be monotonic.
To guarantee monotonicity on back-to-back reads, a serializing instruction must be placed between the two RDPMC instructions.The RDPMC instruction can execute in 16-bit addressing mode or virtual-8086 mode; however, the full contents of RDPMC instruction was introduced into the IA-32 Architecture in the Pentium Pro processor and the Pentium processor with MMX technology.
The earlier Pentium processors have PMCs, but they must be read with the RDMSR instruction.

## Flags affected

- None.

## Exceptions

- 64-Bit Mode Exceptions
  - #GP(0) - If the current privilege level is not 0 an
  > d the PCE flag in the CR4 register is clear.
- Protected Mode Exceptions
  - #GP(0) - If the current privilege level is not 0 an
  > d the PCE flag in the CR4 register is clear.
  > If an invalid performance counter index is specified.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #GP - If an invalid performance counter index is specified.
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If the PCE flag in the CR4 register is clear.
  > If an invalid performance counter index is specified.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
MSCB = Most Significant Counter Bit (* Model-specific *)IF (((CR4.PCE = 1) or (CPL = 0) or (CR0.PE = 0)) and (ECX indicates a supported counter))THENEAX := counter[31:0];EDX := ZeroExtend(counter[MSCB:32]); ELSE (* ECX is not valid or CR4.PCE is 0 and CPL is 1, 2, or 3 and CR0.PE is 1 *)#GP(0); FI;
```

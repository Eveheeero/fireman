# RDTSC

Read Time-Stamp Counter

Reads the current value of the processor's time-stamp counter (a 64-bit MSR) into the EDX:EAX registers.
The EDX register is loaded with the high-order 32 bits of the MSR and the EAX register is loaded with the low-order 32 bits.
(On processors that support the Intel64 architecture, the high-order 32 bits of each of RAX and RDX are cleared.)The processor monotonically increments the time-stamp counter MSR every clock cycle and resets it to 0 whenever ® 64 and IA-32 Architectures Software the processor is reset.
See "Time Stamp Counter" in Chapter 18 of the IntelDeveloper's Manual, Volume 3B, for specific details of the time stamp counter behavior.The time stamp disable (TSD) flag in register CR4 restricts the use of the RDTSC instruction as follows.
When the flag is clear, the RDTSC instruction can be executed at any privilege level; when the flag is set, the instruction can only be executed at privilege level 0.The time-stamp counter can also be read with the RDMSR instruction, when executing at privilege level 0.The RDTSC instruction is not a serializing instruction.
It does not necessarily wait until all previous instructions have been executed before reading the counter.
Similarly, subsequent instructions may begin execution before the read operation is performed.
The following items may guide software seeking to order executions of RDTSC: - If software requires RDTSC to be executed only after all previous instructions have executed and all previous 1 it can execute LFENCE immediately before RDTSC.loads are globally visible, - If software requires RDTSC to be executed only after all previous instructions have executed and all previous loads and stores are globally visible, it can execute the sequence MFENCE;LFENCE immediately before RDTSC.
- If software requires RDTSC to be executed prior to execution of any subsequent instruction (including any memory accesses), it can execute the sequence LFENCE immediately after RDTSC.This instruction was introduced by the Pentium processor.See "Changes to Instruction Behavior in VMX Non-Root Operation" in Chapter 26 of the Intel® 64 and IA-32 Archi-tectures Software Developer's Manual, Volume 3C, for more information about the behavior of this instruction in VMX non-root operation.

## Flags affected

- None.

## Exceptions

- Protected Mode Exceptions
  - #GP(0) - If the TSD flag in register CR4
  > is set and the CPL is greater than 0.
  - #UD - If the LOCK prefix is used.
- Real-Address Mode Exceptions
  - #UD - If the LOCK prefix is used.
- Virtual-8086 Mode Exceptions
  - #GP(0) - If the TSD flag in register CR4 is set.
  - #UD - If the LOCK prefix is used.
- Compatibility Mode Exceptions
  > Same exceptions as in protected mode.

## Operation

```C
 IF (CR4.TSD = 0) or (CPL= 0) or (CR0.PE = 0) THEN EDX:EAX := TimeStampCounter;ELSE (* CR4.TSD = 1 and (CPL = 1, 2, or 3) and CR0.PE = 1 *)#GP(0);FI;
```

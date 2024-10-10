# UIRET

User-Interrupt Return

UIRET returns from the handling of a user interrupt.
It can be executed regardless of CPL.Execution of UIRET inside a transactional region causes a transactional abort; the abort loads EAX as it would have had it been due to an execution of IRET.UIRET can be tracked by Architectural Last Branch Records (LBRs), Intel Processor Trace (Intel PT), and Perfor-mance Monitoring.
For both Intel PT and LBRs, UIRET is recorded in precisely the same manner as IRET.
Hence for LBRs, UIRETs fall into the OTHER_BRANCH category, which implies that IA32_LBR_CTL.OTHER_BRANCH[bit 22] must be set to record user-interrupt delivery, and that the IA32_LBR_x_INFO.BR_TYPE field will indicate OTHER_BRANCH for any recorded user interrupt.
For Intel PT, control flow tracing must be enabled by setting IA32_RTIT_CTL.BranchEn[bit 13].UIRET will also increment performance counters for which counting BR_INST_RETIRED.FAR_BRANCH is enabled.

## Flags affected

- See the Operation section.

## Exceptions

- Protected Mode Exceptions
- Real-Address Mode Exceptions
  - #UD - The UIRET instruction is not
  > recognized in real-address mode.
- Compatibility Mode Exceptions
  - #UD - The UIRET instruction is not re
  > cognized in compatibility mode.
- Virtual-8086 Mode Exceptions
  - #UD - The UIRET instruction is not recognized in virtual-8086 mode.
- 64-Bit Mode Exceptions
  - #GP(0) - If the return instruction pointer is non-canonical.
  - #SS(0) - If an attempt to pop a value off the stack causes a non-canonical address to be referenced.
  - #PF(fault-code) - If a page fault occurs.
  - #AC(0) - If alignment checking is enabled and an
  > unaligned memory reference is made while the 
  > current privilege level is 3. 
  - #CP - If return instruction pointer from stack and shadow stack do not match.
  - #UD - If the LOCK prefix is used.
  > If executed inside an enclave.

## Operation

```C
Pop tempRIP;Pop tempRFLAGS;// see below for how this is used to load RFLAGSPop tempRSP;IF tempRIP is not canonical in current paging modeTHEN #GP(0);FI;IF ShadowStackEnabled(CPL)THENPopShadowStack SSRIP; tempRIPIF SSRIP THEN #CP (FAR-RET/IRET);FI;FI;RIP := tempRIP;// update in RFLAGS only CF, PF, AF, ZF, SF, TF, DF, OF, NT, RF, AC, and IDRFLAGS := (RFLAGS & ~254DD5H) | (tempRFLAGS & 254DD5H);RSP := tempRSP;UIF := 1;Clear any cache-line monitoring established by MONITOR or UMONITOR;
```

# SENDUIPI

Send User Interprocessor Interrupt

The SENDUIPI instruction sends the user interprocessor interrupt (IPI) indicated by its register operand.
(The operand always has 64 bits; operand-size overrides such as the prefix 66 are ignored.)SENDUIPI uses a data structure called the user-interrupt target table (UITT).
This table is located at the linear address UITTADDR (in the IA32_UINTR_TT MSR); it comprises UITTSZ+1 16-byte entries, where UITTSZ = IA32_UINT_MISC[31:0].
SENDUIPI uses the UITT entry (UITTE) indexed by the instruction's register operand.
Each UITTE has the following format: - Bit 0: V, a valid bit.
- Bits 7:1 are reserved and must be 0.
- Bits 15:8: UV, the user-interrupt vector (in the range 0-63, so bits 15:14 must be 0).
- Bits 63:16 are reserved.
- Bits 127:64: UPIDADDR, the linear address of a user posted-interrupt descriptor (UPID).
(UPIDADDR is 64-byte aligned, so bits 69:64 of each UITTE must be 0.)Each UPID has the following format (fields and bits not referenced are reserved): - Bit0 (ON) indicates an outstanding notification.
If this bit is set, there is a notification outstanding for one or more user interrupts in PIR.
- Bit1 (SN) indicates that notifications should be suppressed.
If this bit is set, agents (including SENDUIPI) should not send notifications when posting user interrupts in this descriptor.
- Bits23:16 (NV) contain the notification vector.
This is used by agents sending user-interrupt notifications (including SENDUIPI).
- Bits63:32 (NDST) contain the notification destination.
This is the target physical APIC ID (in xAPIC mode, bits47:40 are the 8-bit APIC ID; in x2APIC mode, the entire field forms the 32-bit APIC ID).
- Bits127:64 (PIF) contain posted-interrupt requests.
There is one bit for each user-interrupt vector.
There is a user-interrupt request for a vector if the corresponding bit is 1.Although SENDUIPI may be executed at any privilege level, all of the instruction's memory accesses (to a UITTE and a UPID) are performed with supervisor privilege.SENDUIPI sends a user interrupt by posting a user interrupt with vector V in the UPID referenced by UPIDADDR and then sending, as an ordinary IPI, any notification interrupt specified in that UPID.

## Flags affected

- None.

## Exceptions

- Real-Address Mode Exceptions
  - #UD - The SENDUIPI instruction is no
  > t recognized in real-address mode.
- Virtual-8086 Mode Exceptions
  - #UD - The SENDUIPI instruction is not recognized in virtual-8086 mode.
- Compatibility Mode Exceptions
  - #UD - The SENDUIPI instruction is no
  > t recognized in compatibility mode.
- 64-Bit Mode Exceptions
  - #UD - If the LOCK prefix is used.
  > If executed inside an enclave.
  > If CR4.UINTR = 0.
  > If IA32_UINTR_TT[0] = 0.
  > If CPUID.07H.0H:EDX.UINTR[bit 5] = 0.
  - #PF - If a page fault occurs.
  - #GP - If the value of the register operand exceeds UITTSZ.
  > If the selected UITTE is not valid or sets any reserved bits.
  > If the selected UPID sets any reserved bits.
- Protected Mode Exceptions
  - #UD - The SENDUIPI instruction is not recognized in protected mode.

## Operation

```C
IF reg > UITTSZ;THEN #GP(0);FI;read tempUITTE from 16 bytes at UITTADDR+ (reg « 4);IF tempUITTE.V = 0 or tempUITTE sets any reserved bitread tempUPID from 16 bytes at tempUITTE.UPIDADDR;// under lockIF tempUPID sets any reserved bits or bits that must be zeroTHEN #GP(0); // release lockFI;tempUPID.PIR[tempUITTE.UV] := 1;IF tempUPID.SN = tempUPID.ON = 0THENtempUPID.ON := 1;sendNotify := 1;ELSE sendNotify := 0;FI;write tempUPID to 16 bytes at tempUITTE.UPIDADDR;// release lockIF sendNotify = 1THENIF local APIC is in x2APIC modeTHEN send ordinary IPI with vector tempUPID.NVto 32-bit physical APIC ID tempUPID.NDST;ELSE send ordinary IPI with vector tempUPID.NVto 8-bit physical APIC ID tempUPID.NDST[15:8];FI;FI;
```
